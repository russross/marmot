from __future__ import annotations

import re
from collections.abc import Iterable
from pathlib import Path

from timetable_chat.models import (
    AvoidClassClusterLongerThan,
    AvoidClassClusterShorterThan,
    AvoidGapBetweenClassClustersLongerThan,
    AvoidGapBetweenClassClustersShorterThan,
    AvoidSectionInRooms,
    AvoidSectionInTimeSlots,
    AvoidTimeSlot,
    DoNotWantADayOff,
    Faculty,
    FacultySectionAddition,
    FacultySectionRemoval,
    FacultySubmission,
    Preference,
    SaveResult,
    SectionSetupMethod,
    UnavailableTimeSlot,
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


def with_priority(arguments: list[str], priority: int | None) -> str:
    if priority is not None:
        arguments.append(f"priority={priority}")
    return ", ".join(arguments)


def render_preference(preference: Preference) -> str:
    match preference:
        case WantADayOff(priority=priority):
            return f"WantADayOff({with_priority([], priority)})"
        case DoNotWantADayOff(priority=priority):
            return f"DoNotWantADayOff({with_priority([], priority)})"
        case WantClassesEvenlySpreadAcrossDays(priority=priority):
            return f"WantClassesEvenlySpreadAcrossDays({with_priority([], priority)})"
        case WantBackToBackClassesInTheSameRoom(priority=priority):
            return f"WantBackToBackClassesInTheSameRoom({with_priority([], priority)})"
        case WantClassesPackedIntoAsFewRoomsAsPossible(priority=priority):
            return f"WantClassesPackedIntoAsFewRoomsAsPossible({with_priority([], priority)})"
        case AvoidGapBetweenClassClustersShorterThan(minutes=minutes, priority=priority):
            arguments = [python_string(duration_string(minutes))]
            return f"AvoidGapBetweenClassClustersShorterThan({with_priority(arguments, priority)})"
        case AvoidGapBetweenClassClustersLongerThan(minutes=minutes, priority=priority):
            arguments = [python_string(duration_string(minutes))]
            return f"AvoidGapBetweenClassClustersLongerThan({with_priority(arguments, priority)})"
        case AvoidClassClusterShorterThan(minutes=minutes, priority=priority):
            arguments = [python_string(duration_string(minutes))]
            return f"AvoidClassClusterShorterThan({with_priority(arguments, priority)})"
        case AvoidClassClusterLongerThan(minutes=minutes, priority=priority):
            arguments = [python_string(duration_string(minutes))]
            return f"AvoidClassClusterLongerThan({with_priority(arguments, priority)})"
        case AvoidSectionInRooms(section=section, room_tags=room_tags, priority=priority):
            arguments = [python_string(section), repr(room_tags)]
            return f"AvoidSectionInRooms({with_priority(arguments, priority)})"
        case AvoidSectionInTimeSlots(
            section=section, time_slot_tags=time_slot_tags, priority=priority
        ):
            arguments = [python_string(section), repr(time_slot_tags)]
            return f"AvoidSectionInTimeSlots({with_priority(arguments, priority)})"
        case AvoidTimeSlot(time_slot=time_slot, priority=priority):
            arguments = [python_string(time_slot)]
            return f"AvoidTimeSlot({with_priority(arguments, priority)})"
        case UnavailableTimeSlot(time_slot=time_slot):
            return f"UnavailableTimeSlot({python_string(time_slot)})"
        case UseSameTimePattern(sections=sections, priority=priority):
            arguments = [repr(sections)]
            return f"UseSameTimePattern({with_priority(arguments, priority)})"


def render_submission(faculty: Faculty, submission: FacultySubmission) -> str:
    availability = ", ".join(
        f"TimeInterval({python_string(interval.days)}, {python_string(interval.start)}, "
        f"{python_string(interval.end)})"
        for interval in faculty.availability
    )
    lines = [
        f"db.make_faculty({python_string(faculty.name)}, {python_string(faculty.department)}, "
        f"[{availability}])"
    ]
    constraint_updates = {update.section: update for update in submission.section_constraints}
    removals = {
        change.section: change
        for change in submission.section_changes
        if isinstance(change, FacultySectionRemoval)
    }
    additions = [
        change
        for change in submission.section_changes
        if isinstance(change, FacultySectionAddition)
    ]
    for removal in removals.values():
        lines.append(
            f"# Teaching assignment change: omit {removal.section}. "
            f"{python_comment(removal.comment)}"
        )
    for section in faculty.section_setup:
        if section.name in removals:
            continue
        update = constraint_updates.get(section.name)
        tags = section.tags
        if update is not None:
            tags = [*update.time_slot_tags, *update.room_tags]
            if update.comment is not None:
                lines.append(
                    f"# Section constraint exception for {section.name}: "
                    f"{python_comment(update.comment)}"
                )
        arguments = [python_string(faculty.name), python_string(section.name)]
        if section.method is SectionSetupMethod.ASSIGN:
            if update is not None:
                lines.append(
                    f"# Requested shared-section tags for {section.name}: "
                    f"times={update.time_slot_tags!r}, rooms={update.room_tags!r}"
                )
        else:
            arguments.extend(python_string(tag) for tag in tags)
        lines.append(f"db.{section.method.value}({', '.join(arguments)})")

    for addition in additions:
        lines.append(
            f"# Teaching assignment change: add {addition.section}. "
            f"{python_comment(addition.comment)}"
        )
        arguments = [python_string(faculty.name), python_string(addition.section)]
        if addition.method is SectionSetupMethod.ASSIGN:
            if addition.time_slot_tags or addition.room_tags:
                lines.append(
                    f"# Requested shared-section tags for {addition.section}: "
                    f"times={addition.time_slot_tags!r}, rooms={addition.room_tags!r}"
                )
        else:
            arguments.extend(python_string(tag) for tag in addition.time_slot_tags)
            arguments.extend(python_string(tag) for tag in addition.room_tags)
        lines.append(f"db.{addition.method.value}({', '.join(arguments)})")

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
    for preference in submission.preferences:
        if isinstance(preference, UnavailableTimeSlot):
            lines.append(f"    # Faculty exception: {python_comment(preference.comment)}")
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
        if isinstance(preference, UnavailableTimeSlot):
            continue
        explicit = preference.priority
        if isinstance(preference, AvoidSectionInRooms) and explicit is None:
            result.append(current + 1)
            continue
        current = current + 1 if explicit is None else explicit
        result.append(current)
        if isinstance(preference, WantADayOff):
            current += 1
    return result


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
    time_slots = {slot.name for slot in repository.semester.time_slots}
    room_names = {room.name for room in repository.semester.rooms}
    valid_room_tags = set(repository.semester.room_tags) | room_names
    valid_time_tags = set(repository.semester.time_slot_tags) | time_slots
    valid_courses = {course.code for course in repository.semester.courses}
    room_tags_by_section = {section.name: section.room_tags for section in faculty.sections}
    time_tags_by_section = {section.name: section.time_slot_tags for section in faculty.sections}

    changed_sections: set[str] = set()
    created_time_slots: set[str] = set()
    for change in submission.section_changes:
        if change.section in changed_sections:
            raise ValueError(f"multiple teaching assignment changes for {change.section!r}")
        changed_sections.add(change.section)
        if isinstance(change, FacultySectionRemoval):
            if change.section not in room_tags_by_section:
                raise ValueError(f"{change.section!r} is not assigned to {faculty.name}")
            del room_tags_by_section[change.section]
            del time_tags_by_section[change.section]
            continue
        if change.section in room_tags_by_section:
            raise ValueError(f"{change.section!r} is already assigned to {faculty.name}")
        validate_section_course(change.section, valid_courses)
        validate_section_tags(
            change.room_tags,
            change.time_slot_tags,
            valid_room_tags,
            valid_time_tags,
        )
        room_tags_by_section[change.section] = change.room_tags
        time_tags_by_section[change.section] = change.time_slot_tags
        created_time_slots.update(
            tag for tag in change.time_slot_tags if valid_explicit_time_slot(tag)
        )

    constraint_updates = {update.section: update for update in submission.section_constraints}
    if len(constraint_updates) != len(submission.section_constraints):
        raise ValueError("each section may have only one constraint update")
    for update in submission.section_constraints:
        if update.section not in room_tags_by_section:
            raise ValueError(f"{update.section!r} is not in {faculty.name}'s proposed courses")
        if update.section in changed_sections:
            raise ValueError(
                f"put the room/time tags directly on the teaching change for {update.section!r}"
            )
        validate_section_tags(
            update.room_tags,
            update.time_slot_tags,
            valid_room_tags,
            valid_time_tags,
        )
        room_tags_by_section[update.section] = update.room_tags
        time_tags_by_section[update.section] = update.time_slot_tags
        created_time_slots.update(
            tag for tag in update.time_slot_tags if valid_explicit_time_slot(tag)
        )

    for section_name in room_tags_by_section:
        validate_section_course(section_name, valid_courses)

    scheduleable_section_count = sum(bool(tags) for tags in time_tags_by_section.values())
    concrete_time_slots = time_slots | created_time_slots

    priorities = effective_priorities(submission.preferences)
    if priorities and max(priorities) > 24:
        raise ValueError("implicit preference priorities exceed the supported range 10..24")

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
                if time_slot not in time_slots:
                    raise ValueError(f"unknown concrete time slot {time_slot!r}")
            case UnavailableTimeSlot(time_slot=time_slot):
                if time_slot in faculty.approved_unavailable_time_slots:
                    raise ValueError(f"{time_slot!r} is already an approved unavailable time")
                if time_slot not in concrete_time_slots:
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
                allowed_slots = {
                    slot
                    for tag in allowed_tags
                    for slot in repository.semester.time_slot_tags.get(
                        tag, [tag] if tag in concrete_time_slots else []
                    )
                }
                requested_slots = {
                    slot
                    for tag in tags
                    for slot in repository.semester.time_slot_tags.get(
                        tag, [tag] if tag in concrete_time_slots else []
                    )
                }
                if not allowed_slots.intersection(requested_slots):
                    raise ValueError(
                        f"time avoidance for {section_name} does not intersect its allowed slots"
                    )
            case UseSameTimePattern(sections=section_names):
                unknown = set(section_names).difference(room_tags_by_section)
                if unknown:
                    message = (
                        f"time-pattern sections are not assigned to {faculty.name}: "
                        f"{sorted(unknown)}"
                    )
                    raise ValueError(message)
            case _:
                continue
    return faculty


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

    def save(self, submission: FacultySubmission, faculty: Faculty | None = None) -> SaveResult:
        faculty = validate_submission(self.repository, submission, faculty)
        snippet = render_submission(faculty, submission)
        self.directory.mkdir(parents=True, exist_ok=True)
        destination = self.path_for(faculty.name)
        temporary = destination.with_suffix(".tmp")
        temporary.write_text(snippet, encoding="utf-8")
        temporary.replace(destination)
        return SaveResult(faculty_name=faculty.name, path=str(destination), snippet=snippet)
