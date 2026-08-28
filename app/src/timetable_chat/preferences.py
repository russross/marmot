from __future__ import annotations

import re
import textwrap
from collections.abc import Iterable
from hashlib import sha256
from pathlib import Path

from timetable_chat.models import (
    AssignmentChangeKind,
    AvoidClassClusterLongerThan,
    AvoidClassClusterShorterThan,
    AvoidGapBetweenClassClustersLongerThan,
    AvoidGapBetweenClassClustersShorterThan,
    AvoidSectionInRooms,
    AvoidSectionInTimeSlots,
    AvoidTimeSlot,
    Course,
    CourseSchedulingPolicy,
    DecisionOrigin,
    DoNotWantADayOff,
    Faculty,
    FacultySubmission,
    Preference,
    ProposedSection,
    SavedSubmission,
    SaveResult,
    SaveStatus,
    SectionSchedulingMode,
    SectionSetupMethod,
    TimeSlot,
    UseSameTimePattern,
    WantADayOff,
    WantBackToBackClassesInTheSameRoom,
    WantClassesEvenlySpreadAcrossDays,
    WantClassesPackedIntoAsFewRoomsAsPossible,
)
from timetable_chat.semester import SemesterRepository

DAY_ORDER = "MTWRFSU"
EXPLICIT_TIME_SLOT = re.compile(
    r"^(?P<days>[MTWRFSU]+)(?P<hour>[0-2][0-9])(?P<minute>[0-5][0-9])\+(?P<duration>[1-9][0-9]*)$"
)


def python_string(value: str) -> str:
    return repr(value)


def python_comment(value: str) -> str:
    return " ".join(value.splitlines()).strip()


def duration_string(minutes: int) -> str:
    hours, remaining_minutes = divmod(minutes, 60)
    if hours == 0:
        return f"{remaining_minutes}m"
    if remaining_minutes == 0:
        return f"{hours}h"
    return f"{hours}h{remaining_minutes}m"


def render_preference(preference: Preference) -> str:
    match preference:
        case WantADayOff():
            return "WantADayOff()"
        case DoNotWantADayOff():
            return "DoNotWantADayOff()"
        case WantClassesEvenlySpreadAcrossDays():
            return "WantClassesEvenlySpreadAcrossDays()"
        case WantBackToBackClassesInTheSameRoom():
            return "WantBackToBackClassesInTheSameRoom()"
        case WantClassesPackedIntoAsFewRoomsAsPossible():
            return "WantClassesPackedIntoAsFewRoomsAsPossible()"
        case AvoidGapBetweenClassClustersShorterThan(minutes=minutes):
            duration = python_string(duration_string(minutes))
            return f"AvoidGapBetweenClassClustersShorterThan({duration})"
        case AvoidGapBetweenClassClustersLongerThan(minutes=minutes):
            return (
                f"AvoidGapBetweenClassClustersLongerThan({python_string(duration_string(minutes))})"
            )
        case AvoidClassClusterShorterThan(minutes=minutes):
            return f"AvoidClassClusterShorterThan({python_string(duration_string(minutes))})"
        case AvoidClassClusterLongerThan(minutes=minutes):
            return f"AvoidClassClusterLongerThan({python_string(duration_string(minutes))})"
        case AvoidSectionInRooms(section=section, room_tags=room_tags):
            return f"AvoidSectionInRooms({python_string(section)}, {room_tags!r})"
        case AvoidSectionInTimeSlots(section=section, time_slot_tags=time_slot_tags):
            return f"AvoidSectionInTimeSlots({python_string(section)}, {time_slot_tags!r})"
        case AvoidTimeSlot(time_slot=time_slot):
            return f"AvoidTimeSlot({python_string(time_slot)})"
        case UseSameTimePattern(sections=sections):
            return f"UseSameTimePattern({sections!r})"


def render_submission(faculty: Faculty, submission: FacultySubmission) -> str:
    availability = ", ".join(
        f"TimeInterval({python_string(interval.days)}, {python_string(interval.start)}, "
        f"{python_string(interval.end)})"
        for interval in faculty.availability
    )
    serialized_submission = submission.model_dump_json()
    lines = ["# Marmot faculty submission v1"]
    lines.extend(
        f"# | {serialized_submission[offset : offset + 96]}"
        for offset in range(0, len(serialized_submission), 96)
    )
    lines.extend(["# End Marmot faculty submission", "#"])
    summary_labels = {
        DecisionOrigin.FACULTY: "Faculty-provided input",
        DecisionOrigin.INFERRED: "Inferences currently used",
        DecisionOrigin.SOURCE: "Source and reconciliation notes",
    }
    for origin, label in summary_labels.items():
        items = [item for item in submission.decision_summary if item.origin is origin]
        if not items:
            continue
        lines.append(f"# {label}")
        for item in items:
            lines.extend(
                textwrap.wrap(
                    python_comment(item.text),
                    width=100,
                    initial_indent="# - ",
                    subsequent_indent="#   ",
                )
            )
        lines.append("#")
    lines.extend(
        [
            "#",
            f"db.make_faculty({python_string(faculty.name)}, "
            f"{python_string(faculty.department)}, [{availability}])",
        ]
    )
    for change in submission.assignment_changes:
        verb = "add" if change.kind is AssignmentChangeKind.ADD else "omit"
        lines.append(
            f"# Teaching assignment change: {verb} {change.section}. "
            f"{python_comment(change.comment)}"
        )
    for section in submission.sections:
        if section.comment is not None:
            lines.append(f"# Section setup for {section.name}: {python_comment(section.comment)}")
        arguments = [python_string(faculty.name), python_string(section.name)]
        if section.method is SectionSetupMethod.ASSIGN:
            if section.time_slot_tags or section.room_tags:
                lines.append(
                    f"# Requested shared-section tags for {section.name}: "
                    f"times={section.time_slot_tags!r}, rooms={section.room_tags!r}"
                )
        else:
            arguments.extend(python_string(tag) for tag in section.time_slot_tags)
            arguments.extend(python_string(tag) for tag in section.room_tags)
        rendered_arguments = ", ".join(arguments)
        if section.credit_hours is not None and section.method is SectionSetupMethod.MAKE:
            rendered_arguments += f", credit_hours={section.credit_hours:g}"
        lines.append(f"db.{section.method.value}({rendered_arguments})")

    if submission.coordination_notes:
        lines.append("")
        for note in submission.coordination_notes:
            label = note.kind.value.replace("_", " ")
            lines.append(f"# Coordination note ({label}): {python_comment(note.text)}")

    lines.append(
        f"db.faculty_preferences({python_string(faculty.name)}, "
        f"{python_string(submission.days_to_check)},"
    )
    lines.extend(
        f"    UnavailableTimeSlot({python_string(time_slot)}),"
        for time_slot in faculty.approved_unavailable_time_slots
    )
    for unavailable in submission.hard_unavailability:
        lines.append(
            f"    # University conflict: {python_comment(unavailable.university_conflict)}"
        )
        lines.append(f"    UnavailableTimeSlot({python_string(unavailable.time_slot)}),")
    for preference in submission.preferences:
        lines.append(f"    {render_preference(preference)},")
    lines.append(")")
    return "\n".join(lines) + "\n"


def validate_day_order(days: str) -> None:
    indices = [DAY_ORDER.index(day) for day in days]
    if indices != sorted(set(indices)):
        raise ValueError("days_to_check must use unique days in MTWRFSU order")


def effective_priorities(preferences: Iterable[Preference]) -> list[int]:
    current = 9
    result: list[int] = []
    for preference in preferences:
        if isinstance(preference, AvoidSectionInRooms):
            result.append(current + 1)
            continue
        current += 1
        result.append(current)
        if isinstance(preference, WantADayOff):
            current += 1
    return result


def normalize_submission(
    repository: SemesterRepository,
    submission: FacultySubmission,
) -> FacultySubmission:
    courses = {course.code: course for course in repository.semester.courses}
    sections: list[ProposedSection] = []
    for section in submission.sections:
        course_code = section.name.rpartition("-")[0]
        course = courses.get(course_code)
        if (
            course is not None
            and section.method is SectionSetupMethod.MAKE
            and course.minimum_credit_hours != course.maximum_credit_hours
            and section.scheduling_mode is SectionSchedulingMode.UNSCHEDULED
            and section.credit_hours is None
        ):
            section = section.model_copy(update={"credit_hours": course.minimum_credit_hours})
        sections.append(section)
    return submission.model_copy(update={"sections": sections})


def validate_submission(
    repository: SemesterRepository,
    submission: FacultySubmission,
    faculty: Faculty | None = None,
) -> Faculty:
    faculty = faculty or repository.faculty(submission.faculty_name)
    if faculty.name.casefold() != submission.faculty_name.strip().casefold():
        raise ValueError(
            f"submission faculty {submission.faculty_name!r} does not match {faculty.name!r}"
        )
    validate_day_order(submission.days_to_check)
    installed_slots = {slot.name: slot for slot in repository.semester.time_slots}
    room_names = {room.name for room in repository.semester.rooms}
    valid_room_tags = set(repository.semester.room_tags) | room_names
    valid_time_tags = set(repository.semester.time_slot_tags) | set(installed_slots)
    courses = {course.code: course for course in repository.semester.courses}

    sections = {section.name: section for section in submission.sections}
    if len(sections) != len(submission.sections):
        raise ValueError("each proposed section must appear exactly once")
    for section in submission.sections:
        course = course_for_section(section.name, courses)
        validate_section_tags(
            section.room_tags,
            section.time_slot_tags,
            valid_room_tags,
            valid_time_tags,
        )
        validate_section_credit_hours(section, course)
        if (
            course.scheduling_policy is CourseSchedulingPolicy.NEVER_SCHEDULED
            and section.scheduling_mode is not SectionSchedulingMode.UNSCHEDULED
        ):
            raise ValueError(f"{section.name} is an unscheduled research or internship course")
        if section.scheduling_mode is SectionSchedulingMode.UNSCHEDULED:
            if section.room_tags or section.time_slot_tags:
                raise ValueError(
                    f"unscheduled section {section.name} cannot have room or time tags"
                )
            continue
        if not section.time_slot_tags:
            raise ValueError(f"scheduled section {section.name} must have allowed times")
        uses_concrete_time = any(
            tag in installed_slots or valid_explicit_time_slot(tag)
            for tag in section.time_slot_tags
        )
        if uses_concrete_time and section.comment is None:
            raise ValueError(
                f"section {section.name} uses a concrete time and requires an explanation"
            )
        validate_contact_minutes(repository, section, course, installed_slots)

    current_sections = {section.name: section for section in faculty.sections}
    expected_changes = {
        (AssignmentChangeKind.ADD, name) for name in sections.keys() - current_sections.keys()
    } | {(AssignmentChangeKind.REMOVE, name) for name in current_sections.keys() - sections.keys()}
    supplied_changes = {(change.kind, change.section) for change in submission.assignment_changes}
    if len(supplied_changes) != len(submission.assignment_changes):
        raise ValueError("each teaching assignment change must appear exactly once")
    if supplied_changes != expected_changes:
        raise ValueError(
            "teaching assignment reasons must exactly match additions and removals: "
            f"expected {sorted((kind.value, section) for kind, section in expected_changes)}"
        )
    current_setup = {section.name: section for section in faculty.section_setup}
    for section_name in sections.keys() & current_sections.keys():
        proposed = sections[section_name]
        current = current_sections[section_name]
        setup = current_setup[section_name]
        if (
            proposed.room_tags != current.room_tags
            or proposed.time_slot_tags != current.time_slot_tags
            or proposed.method is not setup.method
        ) and proposed.comment is None:
            raise ValueError(f"changed section setup for {section_name} requires an explanation")

    scheduleable_section_count = sum(
        section.scheduling_mode is SectionSchedulingMode.SCHEDULED
        for section in submission.sections
    )
    concrete_time_slots = set(installed_slots) | {
        tag
        for section in submission.sections
        for tag in section.time_slot_tags
        if valid_explicit_time_slot(tag)
    }
    unavailable_slots = [item.time_slot for item in submission.hard_unavailability]
    if len(set(unavailable_slots)) != len(unavailable_slots):
        raise ValueError("each university-related hard-unavailability slot may appear once")
    for unavailable in submission.hard_unavailability:
        if unavailable.time_slot in faculty.approved_unavailable_time_slots:
            raise ValueError(f"{unavailable.time_slot!r} is already an approved unavailable time")
        if unavailable.time_slot not in concrete_time_slots:
            raise ValueError(f"unknown concrete time slot {unavailable.time_slot!r}")

    priorities = effective_priorities(submission.preferences)
    if priorities and max(priorities) > 24:
        raise ValueError("implicit preference priorities exceed the supported range 10..24")
    room_tags_by_section = {section.name: section.room_tags for section in submission.sections}
    time_tags_by_section = {section.name: section.time_slot_tags for section in submission.sections}
    for preference in submission.preferences:
        match preference:
            case WantADayOff() | DoNotWantADayOff():
                if len(submission.days_to_check) < 2:
                    raise ValueError("day-off preferences require at least two days_to_check")
                if scheduleable_section_count < 2:
                    raise ValueError(
                        "day-off preferences require at least two scheduleable sections"
                    )
            case WantClassesEvenlySpreadAcrossDays():
                if len(submission.days_to_check) < 2:
                    raise ValueError("even-spread requires at least two days_to_check")
                if scheduleable_section_count <= 3:
                    raise ValueError("even-spread requires more than three scheduleable sections")
            case AvoidTimeSlot(time_slot=time_slot):
                if time_slot not in installed_slots:
                    raise ValueError(f"unknown concrete time slot {time_slot!r}")
            case AvoidSectionInRooms(section=section_name, room_tags=tags):
                allowed_tags = room_tags_by_section.get(section_name)
                if allowed_tags is None:
                    raise ValueError(f"{section_name!r} is not in the proposed courses")
                allowed_rooms = {
                    room
                    for tag in allowed_tags
                    for room in repository.semester.room_tags.get(tag, [tag])
                }
                requested_rooms = {
                    room
                    for tag in tags
                    for room in repository.semester.room_tags.get(
                        tag, [tag] if tag in room_names else []
                    )
                }
                if not allowed_rooms.intersection(requested_rooms):
                    raise ValueError(
                        f"room avoidance for {section_name} does not intersect its allowed rooms"
                    )
            case AvoidSectionInTimeSlots(section=section_name, time_slot_tags=tags):
                allowed_tags = time_tags_by_section.get(section_name)
                if allowed_tags is None:
                    raise ValueError(f"{section_name!r} is not in the proposed courses")
                allowed_slots = expanded_time_slot_names(
                    repository, allowed_tags, concrete_time_slots
                )
                requested_slots = expanded_time_slot_names(repository, tags, concrete_time_slots)
                if not allowed_slots.intersection(requested_slots):
                    raise ValueError(
                        f"time avoidance for {section_name} does not intersect its allowed slots"
                    )
            case UseSameTimePattern(sections=section_names):
                unknown = set(section_names).difference(sections)
                if unknown:
                    raise ValueError(
                        f"time-pattern sections are not assigned to {faculty.name}: "
                        f"{sorted(unknown)}"
                    )
            case _:
                continue
    return faculty


def course_for_section(section: str, courses: dict[str, Course]) -> Course:
    course_code, separator, section_number = section.rpartition("-")
    if not separator or not section_number:
        raise ValueError(f"section {section!r} must contain a course and section number")
    try:
        return courses[course_code]
    except KeyError as error:
        raise ValueError(
            f"section {section!r} uses unknown installed course {course_code!r}"
        ) from error


def validate_section_credit_hours(section: ProposedSection, course: Course) -> None:
    if section.method is SectionSetupMethod.ASSIGN:
        if section.credit_hours is not None:
            raise ValueError(
                f"shared section {section.name} gets credit hours from its section creator"
            )
        return
    is_variable = course.minimum_credit_hours != course.maximum_credit_hours
    if not is_variable:
        if section.credit_hours is not None:
            raise ValueError(
                f"fixed-credit section {section.name} must omit the redundant credit_hours value"
            )
        return
    if section.credit_hours is None:
        raise ValueError(f"variable-credit section {section.name} must specify credit_hours")
    if not course.minimum_credit_hours <= section.credit_hours <= course.maximum_credit_hours:
        raise ValueError(
            f"section {section.name} credit_hours {section.credit_hours:g} is outside the "
            f"catalog range {course.minimum_credit_hours:g}-{course.maximum_credit_hours:g}"
        )


def expanded_time_slot_names(
    repository: SemesterRepository,
    tags: list[str],
    concrete_time_slots: set[str],
) -> set[str]:
    return {
        slot
        for tag in tags
        for slot in repository.semester.time_slot_tags.get(
            tag, [tag] if tag in concrete_time_slots else []
        )
    }


def validate_contact_minutes(
    repository: SemesterRepository,
    section: ProposedSection,
    course: Course,
    installed_slots: dict[str, TimeSlot],
) -> None:
    if (
        section.method is SectionSetupMethod.ASSIGN
        or course.scheduling_policy is CourseSchedulingPolicy.EXTERNALLY_SCHEDULED
    ):
        return
    selected_credit_hours = section.credit_hours or course.minimum_credit_hours
    expected_minutes = course.contact_minutes_override or selected_credit_hours * 50
    concrete_names = expanded_time_slot_names(
        repository,
        section.time_slot_tags,
        set(installed_slots)
        | {tag for tag in section.time_slot_tags if valid_explicit_time_slot(tag)},
    )
    for time_slot_name in concrete_names:
        installed = installed_slots.get(time_slot_name)
        if installed is not None:
            weekly_minutes = len(installed.days) * installed.duration_minutes
        else:
            match = EXPLICIT_TIME_SLOT.fullmatch(time_slot_name)
            if match is None:
                raise ValueError(f"unknown concrete time slot {time_slot_name!r}")
            weekly_minutes = len(match.group("days")) * int(match.group("duration"))
        if weekly_minutes != expected_minutes:
            raise ValueError(
                f"{section.name} is {selected_credit_hours:g} credits and requires "
                f"{expected_minutes:g} contact minutes per week, but {time_slot_name} "
                f"provides {weekly_minutes}"
            )


def validate_section_tags(
    room_tags: list[str],
    time_slot_tags: list[str],
    valid_room_tags: set[str],
    valid_time_tags: set[str],
) -> None:
    unknown_room_tags = set(room_tags).difference(valid_room_tags)
    if unknown_room_tags:
        raise ValueError(f"unknown room tags: {sorted(unknown_room_tags)}")
    unknown_time_tags = [
        tag
        for tag in time_slot_tags
        if tag not in valid_time_tags and not valid_explicit_time_slot(tag)
    ]
    if unknown_time_tags:
        raise ValueError(f"unknown time-slot tags: {sorted(unknown_time_tags)}")


def validate_section_course(section: str, valid_courses: set[str]) -> None:
    course, separator, section_number = section.rpartition("-")
    if not separator or not section_number:
        raise ValueError(f"section {section!r} must contain a course and section number")
    if course not in valid_courses:
        raise ValueError(f"section {section!r} uses unknown installed course {course!r}")


def valid_explicit_time_slot(value: str) -> bool:
    match = EXPLICIT_TIME_SLOT.fullmatch(value)
    if match is None:
        return False
    return int(match.group("hour")) < 24


def faculty_slug(name: str) -> str:
    slug = re.sub(r"[^a-z0-9]+", "-", name.casefold()).strip("-")
    if not slug:
        raise ValueError("faculty name cannot produce an empty filename")
    return slug


class PreferenceStore:
    def __init__(self, directory: Path, repository: SemesterRepository) -> None:
        self.directory = directory
        self.repository = repository

    def path_for(self, faculty_name: str) -> Path:
        return self.directory / f"{faculty_slug(faculty_name)}.py"

    def read(self, faculty_name: str) -> str | None:
        path = self.path_for(faculty_name)
        if not path.exists():
            return None
        return path.read_text(encoding="utf-8")

    def read_submission(self, faculty_name: str) -> SavedSubmission | None:
        snippet = self.read(faculty_name)
        if snippet is None:
            return None
        lines = snippet.splitlines()
        try:
            start = lines.index("# Marmot faculty submission v1") + 1
            end = lines.index("# End Marmot faculty submission", start)
        except ValueError:
            return None
        payload_lines = lines[start:end]
        if not payload_lines or any(not line.startswith("# | ") for line in payload_lines):
            return None
        payload = "".join(line[4:] for line in payload_lines)
        submission = FacultySubmission.model_validate_json(payload, strict=True)
        return SavedSubmission(
            saved_revision=self._revision(snippet),
            submission=submission,
            snippet=snippet,
        )

    def revision(self, faculty_name: str) -> str | None:
        snippet = self.read(faculty_name)
        return None if snippet is None else self._revision(snippet)

    def save(
        self,
        submission: FacultySubmission,
        faculty: Faculty | None = None,
        expected_saved_revision: str | None = None,
    ) -> SaveResult:
        current_snippet = self.read(submission.faculty_name)
        current_revision = None if current_snippet is None else self._revision(current_snippet)
        if expected_saved_revision != current_revision:
            raise ValueError(
                "saved preferences changed after they were loaded; reload the faculty "
                "workspace before saving"
            )
        submission = normalize_submission(self.repository, submission)
        faculty = validate_submission(self.repository, submission, faculty)
        snippet = render_submission(faculty, submission)
        if snippet == current_snippet:
            return SaveResult(
                faculty_name=faculty.name,
                path=str(self.path_for(faculty.name)),
                status=SaveStatus.UNCHANGED,
                saved_revision=self._revision(snippet),
                submission=submission,
                snippet=snippet,
            )
        self.directory.mkdir(parents=True, exist_ok=True)
        destination = self.path_for(faculty.name)
        temporary = destination.with_suffix(".tmp")
        temporary.write_text(snippet, encoding="utf-8")
        temporary.replace(destination)
        return SaveResult(
            faculty_name=faculty.name,
            path=str(destination),
            status=(SaveStatus.CREATED if current_snippet is None else SaveStatus.UPDATED),
            saved_revision=self._revision(snippet),
            submission=submission,
            snippet=snippet,
        )

    @staticmethod
    def _revision(snippet: str) -> str:
        return sha256(snippet.encode("utf-8")).hexdigest()
