import json
from typing import Literal

from pydantic import JsonValue, TypeAdapter

from timetable_chat.assignments import (
    AssignmentWorkbookClient,
    empty_faculty_context,
    resolve_faculty_contexts,
)
from timetable_chat.models import FacultyContext, FacultySubmission, StrictModel
from timetable_chat.preferences import PreferenceStore, render_submission, validate_submission
from timetable_chat.prompt import PREFERENCE_GUIDE, PRIORITY_BALANCING_GUIDE
from timetable_chat.semester import SemesterRepository


class FacultyNameArguments(StrictModel):
    faculty_name: str


class SchedulingReferenceArguments(StrictModel):
    topic: Literal["preferences", "rooms_and_times", "curriculum"]


FACULTY_NAME_ADAPTER = TypeAdapter(FacultyNameArguments)
REFERENCE_ADAPTER = TypeAdapter(SchedulingReferenceArguments)
SUBMISSION_ADAPTER = TypeAdapter(FacultySubmission)


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
                "get_faculty_context",
                "Download the live assignment spreadsheet and get one faculty member's "
                "editable tentative teaching assignments, inferred room/time limitations, "
                "availability, and assignment warnings.",
                FACULTY_NAME_ADAPTER.json_schema(),
            ),
            self._definition(
                "get_previous_preferences",
                "Get preference history from the two immediately preceding semesters for "
                "one faculty member, including whether they were present in each term.",
                FACULTY_NAME_ADAPTER.json_schema(),
            ),
            self._definition(
                "get_saved_preferences",
                "Get the latest submission already saved by this app for one faculty "
                "member, if present.",
                FACULTY_NAME_ADAPTER.json_schema(),
            ),
            self._definition(
                "get_scheduling_reference",
                "Get detailed installed reference data for preferences, rooms/times, or curricula.",
                REFERENCE_ADAPTER.json_schema(),
            ),
            self._definition(
                "preview_preferences",
                "Validate complete proposed teaching changes, section constraints, "
                "preferences, and coordination notes, then render the final Python snippet "
                "without saving.",
                SUBMISSION_ADAPTER.json_schema(),
            ),
            self._definition(
                "save_preferences",
                "Validate and atomically replace the faculty member's complete saved "
                "Python snippet. Use only after explicit save confirmation.",
                SUBMISSION_ADAPTER.json_schema(),
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
                "parameters": parameters,
                "strict": True,
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
            case "get_faculty_context":
                arguments = FACULTY_NAME_ADAPTER.validate_python(raw_arguments, strict=True)
                context = await self._faculty_context(arguments.faculty_name)
                return {
                    "term": self.repository.semester.term,
                    **context.model_dump(mode="json", exclude={"faculty": {"preference_history"}}),
                }
            case "get_previous_preferences":
                arguments = FACULTY_NAME_ADAPTER.validate_python(raw_arguments, strict=True)
                faculty = (await self._faculty_context(arguments.faculty_name)).faculty
                return {
                    "faculty_name": faculty.name,
                    "history": [
                        record.model_dump(mode="json", exclude={"section_setup"})
                        for record in faculty.preference_history
                    ],
                }
            case "get_saved_preferences":
                arguments = FACULTY_NAME_ADAPTER.validate_python(raw_arguments, strict=True)
                faculty = (await self._faculty_context(arguments.faculty_name)).faculty
                return {
                    "faculty_name": faculty.name,
                    "preferences": self.preference_store.read(faculty.name),
                }
            case "get_scheduling_reference":
                arguments = REFERENCE_ADAPTER.validate_python(raw_arguments, strict=True)
                return self._reference(arguments.topic)
            case "preview_preferences":
                submission = SUBMISSION_ADAPTER.validate_python(raw_arguments, strict=True)
                context = await self._faculty_context(submission.faculty_name)
                self._validate_assignment_revision(submission, context)
                faculty = context.faculty
                faculty = validate_submission(self.repository, submission, faculty)
                return {
                    "faculty_name": faculty.name,
                    "snippet": render_submission(faculty, submission),
                }
            case "save_preferences":
                submission = SUBMISSION_ADAPTER.validate_python(raw_arguments, strict=True)
                context = await self._faculty_context(submission.faculty_name)
                self._validate_assignment_revision(submission, context)
                return self.preference_store.save(submission, context.faculty).model_dump(
                    mode="json"
                )
            case _:
                raise ValueError(f"unknown tool {name!r}")

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
                "current assignments changed after they were retrieved; get fresh faculty "
                "context and preview the complete submission again"
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
