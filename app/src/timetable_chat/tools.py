import json
from enum import StrEnum
from typing import Literal

from pydantic import Field, JsonValue, TypeAdapter

from timetable_chat.assignments import (
    AssignmentWorkbookClient,
    empty_faculty_context,
    resolve_faculty_contexts,
)
from timetable_chat.models import (
    AssignmentChangeKind,
    DurationMinutes,
    FacultyContext,
    FacultySubmission,
    Preference,
    ProposedSection,
    StrictModel,
)
from timetable_chat.preferences import PreferenceStore
from timetable_chat.prompt import PREFERENCE_GUIDE, PRIORITY_BALANCING_GUIDE
from timetable_chat.semester import SemesterRepository


class FacultyNameArguments(StrictModel):
    faculty_name: str


class SchedulingReferenceArguments(StrictModel):
    topic: Literal["preferences", "rooms_and_times", "curriculum"]


class PreferenceKind(StrEnum):
    WANT_A_DAY_OFF = "want_a_day_off"
    DO_NOT_WANT_A_DAY_OFF = "do_not_want_a_day_off"
    WANT_CLASSES_EVENLY_SPREAD_ACROSS_DAYS = "want_classes_evenly_spread_across_days"
    WANT_BACK_TO_BACK_CLASSES_IN_THE_SAME_ROOM = "want_back_to_back_classes_in_the_same_room"
    WANT_CLASSES_PACKED_INTO_AS_FEW_ROOMS_AS_POSSIBLE = (
        "want_classes_packed_into_as_few_rooms_as_possible"
    )
    AVOID_GAP_BETWEEN_CLASS_CLUSTERS_SHORTER_THAN = "avoid_gap_between_class_clusters_shorter_than"
    AVOID_GAP_BETWEEN_CLASS_CLUSTERS_LONGER_THAN = "avoid_gap_between_class_clusters_longer_than"
    AVOID_CLASS_CLUSTER_SHORTER_THAN = "avoid_class_cluster_shorter_than"
    AVOID_CLASS_CLUSTER_LONGER_THAN = "avoid_class_cluster_longer_than"
    AVOID_SECTION_IN_ROOMS = "avoid_section_in_rooms"
    AVOID_SECTION_IN_TIME_SLOTS = "avoid_section_in_time_slots"
    AVOID_TIME_SLOT = "avoid_time_slot"
    USE_SAME_TIME_PATTERN = "use_same_time_pattern"


class ToolPreference(StrictModel):
    kind: PreferenceKind
    minutes: DurationMinutes | None = None
    section: str | None = None
    room_tags: list[str] | None = None
    time_slot_tags: list[str] | None = None
    time_slot: str | None = None
    sections: list[str] | None = None

    def to_preference(self) -> Preference:
        return PREFERENCE_ADAPTER.validate_json(
            self.model_dump_json(exclude_none=True), strict=True
        )


class ToolProposedSection(ProposedSection):
    credit_hours: float | None = Field(
        default=None,
        ge=0,
        allow_inf_nan=False,
        description="Required for variable-credit courses and omitted for fixed-credit courses.",
    )


class ToolFacultySubmission(FacultySubmission):
    sections: list[ToolProposedSection]
    preferences: list[ToolPreference]

    def to_submission(self) -> FacultySubmission:
        submission_data: dict[str, JsonValue] = self.model_dump(
            mode="json", exclude={"preferences"}
        )
        submission_data["preferences"] = [
            preference.to_preference().model_dump(mode="json") for preference in self.preferences
        ]
        return FacultySubmission.model_validate_json(json.dumps(submission_data), strict=True)


class SaveFacultySubmissionArguments(StrictModel):
    expected_saved_revision: str | None = Field(
        default=None,
        description="Omit when no saved working draft exists.",
    )
    submission: ToolFacultySubmission


FACULTY_NAME_ADAPTER = TypeAdapter(FacultyNameArguments)
REFERENCE_ADAPTER = TypeAdapter(SchedulingReferenceArguments)
PREFERENCE_ADAPTER = TypeAdapter(Preference)
SAVE_SUBMISSION_ADAPTER = TypeAdapter(SaveFacultySubmissionArguments)


def provider_tool_schema(schema: dict[str, JsonValue]) -> dict[str, JsonValue]:
    definitions_value = schema.get("$defs")
    definitions = definitions_value if isinstance(definitions_value, dict) else {}

    def simplify(value: JsonValue) -> JsonValue:
        if isinstance(value, list):
            return [simplify(item) for item in value]
        if not isinstance(value, dict):
            return value

        reference = value.get("$ref")
        if isinstance(reference, str):
            prefix = "#/$defs/"
            if not reference.startswith(prefix):
                raise ValueError(f"unsupported tool schema reference {reference!r}")
            target = definitions.get(reference.removeprefix(prefix))
            if not isinstance(target, dict):
                raise ValueError(f"unknown tool schema reference {reference!r}")
            return simplify(target)

        any_of = value.get("anyOf")
        if isinstance(any_of, list):
            non_null = [
                option
                for option in any_of
                if not (isinstance(option, dict) and option.get("type") == "null")
            ]
            if len(non_null) != 1:
                raise ValueError("tool schema contains a non-null union")
            return simplify(non_null[0])

        omitted = {"$defs", "default", "discriminator", "title"}
        return {key: simplify(child) for key, child in value.items() if key not in omitted}

    simplified = simplify(schema)
    if not isinstance(simplified, dict):
        raise ValueError("tool parameter schema must be an object")
    return simplified


class ToolService:
    def __init__(
        self,
        repository: SemesterRepository,
        preference_store: PreferenceStore,
        assignment_client: AssignmentWorkbookClient,
    ) -> None:
        self.repository = repository
        self.preference_store = preference_store
        self.assignment_client = assignment_client

    async def close(self) -> None:
        await self.assignment_client.close()

    def definitions(self) -> list[dict[str, JsonValue]]:
        return [
            self._definition(
                "list_faculty",
                "Download the live assignment spreadsheet and list its faculty names and "
                "currently unassigned courses.",
                {"type": "object", "properties": {}, "additionalProperties": False},
            ),
            self._definition(
                "load_faculty_workspace",
                "Load one faculty member's current assignments, complete saved working draft, "
                "saved revision, both historical records, relevant catalog courses, and source "
                "discrepancies from one coherent workbook revision.",
                FACULTY_NAME_ADAPTER.json_schema(),
            ),
            self._definition(
                "get_scheduling_reference",
                "Get detailed installed reference data for preferences, rooms/times, or curricula.",
                REFERENCE_ADAPTER.json_schema(),
            ),
            self._definition(
                "save_faculty_submission",
                "Validate and atomically create or replace the speaker's complete working draft. "
                "Call whenever actionable input changes the draft, including the initial inferred "
                "proposal. Pass the saved revision returned by load_faculty_workspace to prevent "
                "stale conversation branches from overwriting newer work.",
                SAVE_SUBMISSION_ADAPTER.json_schema(),
            ),
        ]

    @staticmethod
    def _definition(
        name: str, description: str, parameters: dict[str, JsonValue]
    ) -> dict[str, JsonValue]:
        return {
            "type": "function",
            "function": {
                "name": name,
                "description": description,
                "parameters": provider_tool_schema(parameters),
            },
        }

    async def execute(self, name: str, arguments_json: str) -> str:
        try:
            raw_arguments = json.loads(arguments_json)
            result = await self._execute_validated(name, raw_arguments)
            return json.dumps({"ok": True, "result": result}, ensure_ascii=False)
        except (ValueError, OSError) as error:
            return json.dumps({"ok": False, "error": str(error)}, ensure_ascii=False)

    async def _execute_validated(self, name: str, raw_arguments: JsonValue) -> JsonValue:
        match name:
            case "list_faculty":
                workbook = await self.assignment_client.fetch()
                contexts, unassigned = resolve_faculty_contexts(self.repository, workbook)
                return {
                    "term": self.repository.semester.term,
                    "faculty": sorted(
                        (context.faculty.name for context in contexts.values()),
                        key=str.casefold,
                    ),
                    "unassigned_courses": [
                        assignment.model_dump(mode="json") for assignment in unassigned
                    ],
                    "assignment_source": workbook.source.model_dump(mode="json"),
                }
            case "load_faculty_workspace":
                arguments = FACULTY_NAME_ADAPTER.validate_json(
                    json.dumps(raw_arguments), strict=True
                )
                context = await self._faculty_context(arguments.faculty_name)
                return self._workspace(context)
            case "get_scheduling_reference":
                arguments = REFERENCE_ADAPTER.validate_json(json.dumps(raw_arguments), strict=True)
                return self._reference(arguments.topic)
            case "save_faculty_submission":
                arguments = SAVE_SUBMISSION_ADAPTER.validate_json(
                    json.dumps(raw_arguments), strict=True
                )
                submission = arguments.submission.to_submission()
                context = await self._faculty_context(submission.faculty_name)
                self._validate_assignment_revision(submission, context)
                return self.preference_store.save(
                    submission,
                    context.faculty,
                    arguments.expected_saved_revision,
                ).model_dump(mode="json")
            case _:
                raise ValueError(f"unknown tool {name!r}")

    def _workspace(self, context: FacultyContext) -> JsonValue:
        faculty = context.faculty
        saved = self.preference_store.read_submission(faculty.name)
        legacy_snippet = None
        if saved is None:
            legacy_snippet = self.preference_store.read(faculty.name)
        current_sections = {assignment.section: assignment for assignment in context.assignments}
        discrepancies: list[str] = []
        saved_section_names: set[str] = set()
        if saved is not None:
            saved_sections = {section.name: section for section in saved.submission.sections}
            saved_section_names = set(saved_sections)
            for section_name in sorted(current_sections.keys() - saved_sections.keys()):
                discrepancies.append(
                    f"The live assignment source now includes {section_name}, which is not in "
                    "the saved working draft."
                )
            for section_name in sorted(saved_sections.keys() - current_sections.keys()):
                discrepancies.append(
                    f"The saved working draft includes {section_name}, which is not in the "
                    "live assignment source."
                )
            for section_name in sorted(current_sections.keys() & saved_sections.keys()):
                current = current_sections[section_name]
                proposed = saved_sections[section_name]
                if (
                    current.room_tags != proposed.room_tags
                    or current.time_slot_tags != proposed.time_slot_tags
                    or current.setup_method is not proposed.method
                ):
                    discrepancies.append(
                        f"The live assignment source and saved working draft have different "
                        f"section setup for {section_name}."
                    )
            expected_changes = {
                (AssignmentChangeKind.ADD, section_name)
                for section_name in saved_sections.keys() - current_sections.keys()
            } | {
                (AssignmentChangeKind.REMOVE, section_name)
                for section_name in current_sections.keys() - saved_sections.keys()
            }
            saved_changes = {
                (change.kind, change.section) for change in saved.submission.assignment_changes
            }
            if saved_changes != expected_changes:
                discrepancies.append(
                    "The saved teaching-change explanations no longer match the live "
                    "assignment differences and need reconciliation before the next update."
                )
        relevant_course_codes = {assignment.course_code for assignment in context.assignments} | {
            section.rpartition("-")[0] for section in saved_section_names
        }
        courses = [
            course.model_dump(mode="json")
            for course in self.repository.semester.courses
            if course.code in relevant_course_codes
        ]
        return {
            "term": self.repository.semester.term,
            "faculty": faculty.model_dump(
                mode="json",
                exclude={"preference_history", "current_preferences"},
            ),
            "assignments": [
                assignment.model_dump(mode="json") for assignment in context.assignments
            ],
            "assignment_source": context.assignment_source.model_dump(mode="json"),
            "issues": context.issues,
            "history": [record.model_dump(mode="json") for record in faculty.preference_history],
            "saved": None if saved is None else saved.model_dump(mode="json"),
            "saved_revision": self.preference_store.revision(faculty.name),
            "legacy_saved_preferences": legacy_snippet,
            "discrepancies": discrepancies,
            "courses": courses,
        }

    async def _faculty_context(self, name: str) -> FacultyContext:
        workbook = await self.assignment_client.fetch()
        contexts, _ = resolve_faculty_contexts(self.repository, workbook)
        normalized_name = name.strip().casefold()
        context = contexts.get(normalized_name)
        if context is not None:
            return context
        for candidate in contexts.values():
            if any(
                assignment.spreadsheet_faculty_name is not None
                and assignment.spreadsheet_faculty_name.casefold() == normalized_name
                for assignment in candidate.assignments
            ):
                return candidate
        try:
            faculty = self.repository.faculty(name)
        except ValueError as error:
            available = ", ".join(
                sorted(
                    (candidate.faculty.name for candidate in contexts.values()),
                    key=str.casefold,
                )
            )
            raise ValueError(
                f"unknown current faculty member {name!r}; spreadsheet names: {available}"
            ) from error
        return empty_faculty_context(workbook.source, faculty)

    @staticmethod
    def _validate_assignment_revision(
        submission: FacultySubmission,
        context: FacultyContext,
    ) -> None:
        if submission.assignment_revision != context.assignment_source.revision:
            raise ValueError(
                "current assignments changed after they were retrieved; reload the faculty "
                "workspace and reconcile the complete submission before saving"
            )

    def _reference(self, topic: str) -> JsonValue:
        semester = self.repository.semester
        match topic:
            case "preferences":
                return {"guide": f"{PRIORITY_BALANCING_GUIDE}\n\n{PREFERENCE_GUIDE}"}
            case "rooms_and_times":
                return {
                    "rooms": [room.model_dump(mode="json") for room in semester.rooms],
                    "room_tags": semester.room_tags,
                    "time_slots": [
                        time_slot.model_dump(mode="json") for time_slot in semester.time_slots
                    ],
                    "time_slot_tags": semester.time_slot_tags,
                }
            case "curriculum":
                return {
                    "courses": [course.model_dump(mode="json") for course in semester.courses],
                    "programs": [program.model_dump(mode="json") for program in semester.programs],
                }
            case _:
                raise ValueError(f"unknown reference topic {topic!r}")
