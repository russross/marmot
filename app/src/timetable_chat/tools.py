import json
from typing import Literal

from pydantic import JsonValue, TypeAdapter

from timetable_chat.models import FacultySubmission, StrictModel
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
    def __init__(self, repository: SemesterRepository, preference_store: PreferenceStore) -> None:
        self.repository = repository
        self.preference_store = preference_store

    def definitions(self) -> list[dict[str, JsonValue]]:
        return [
            self._definition(
                "list_faculty",
                "List faculty names in the installed active semester.",
                {"type": "object", "properties": {}, "additionalProperties": False},
            ),
            self._definition(
                "get_faculty_context",
                "Get the editable tentative teaching assignment, room/time limitations, "
                "availability, and current-term example input for one faculty member.",
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

    def execute(self, name: str, arguments_json: str) -> str:
        try:
            raw_arguments = json.loads(arguments_json)
            result = self._execute_validated(name, raw_arguments)
            return json.dumps({"ok": True, "result": result}, ensure_ascii=False)
        except (ValueError, OSError) as error:
            return json.dumps({"ok": False, "error": str(error)}, ensure_ascii=False)

    def _execute_validated(self, name: str, raw_arguments: JsonValue) -> JsonValue:
        match name:
            case "list_faculty":
                return {
                    "term": self.repository.semester.term,
                    "faculty": self.repository.faculty_names(),
                }
            case "get_faculty_context":
                arguments = FACULTY_NAME_ADAPTER.validate_python(raw_arguments, strict=True)
                faculty = self.repository.faculty(arguments.faculty_name)
                return {
                    "term": self.repository.semester.term,
                    "faculty": faculty.model_dump(mode="json", exclude={"preference_history"}),
                }
            case "get_previous_preferences":
                arguments = FACULTY_NAME_ADAPTER.validate_python(raw_arguments, strict=True)
                faculty = self.repository.faculty(arguments.faculty_name)
                return {
                    "faculty_name": faculty.name,
                    "history": [
                        record.model_dump(mode="json") for record in faculty.preference_history
                    ],
                }
            case "get_saved_preferences":
                arguments = FACULTY_NAME_ADAPTER.validate_python(raw_arguments, strict=True)
                faculty = self.repository.faculty(arguments.faculty_name)
                return {
                    "faculty_name": faculty.name,
                    "preferences": self.preference_store.read(faculty.name),
                }
            case "get_scheduling_reference":
                arguments = REFERENCE_ADAPTER.validate_python(raw_arguments, strict=True)
                return self._reference(arguments.topic)
            case "preview_preferences":
                submission = SUBMISSION_ADAPTER.validate_python(raw_arguments, strict=True)
                faculty = validate_submission(self.repository, submission)
                return {
                    "faculty_name": faculty.name,
                    "snippet": render_submission(faculty, submission),
                }
            case "save_preferences":
                submission = SUBMISSION_ADAPTER.validate_python(raw_arguments, strict=True)
                return self.preference_store.save(submission).model_dump(mode="json")
            case _:
                raise ValueError(f"unknown tool {name!r}")

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
