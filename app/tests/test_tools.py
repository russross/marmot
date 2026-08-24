import json
from pathlib import Path

from timetable_chat.preferences import PreferenceStore
from timetable_chat.semester import SemesterRepository
from timetable_chat.tools import ToolService


def test_tool_preview_reports_validation_error_without_writing(
    repository: SemesterRepository, runtime_directory: Path
) -> None:
    store = PreferenceStore(runtime_directory / "preferences", repository)
    tools = ToolService(repository, store)

    response = json.loads(
        tools.execute(
            "preview_preferences",
            json.dumps(
                {
                    "faculty_name": "Lora Klein",
                    "days_to_check": "MT",
                    "preferences": [{"kind": "want_classes_evenly_spread_across_days"}],
                }
            ),
        )
    )

    assert response["ok"] is False
    assert "more than three scheduleable sections" in response["error"]
    assert not (runtime_directory / "preferences").exists()


def test_installed_snapshot_distinguishes_recent_history_and_new_faculty(
    repository: SemesterRepository,
) -> None:
    assert repository.semester.term == "Fall 2026"
    assert repository.semester.historical_terms == ["Spring 2026", "Fall 2025"]
    assert "reports Spring 2026 internally" in repository.semester.provenance.note
    bart = repository.faculty("bart stander")
    recent, older = bart.preference_history
    assert recent.term == "Spring 2026"
    assert recent.faculty_present is True
    assert recent.preferences is not None
    assert "WantClassesPackedIntoAsFewRoomsAsPossible" in recent.preferences
    assert older.term == "Fall 2025"
    assert older.faculty_present is True
    assert older.preferences is not None
    assert "AvoidClassClusterShorterThan(110, priority=12)" in older.preferences
    assert "ClusterTooShort" not in older.preferences

    new_faculty = repository.faculty("Design Faculty")
    assert all(not record.faculty_present for record in new_faculty.preference_history)
    assert all(record.preferences is None for record in new_faculty.preference_history)


def test_history_tool_keeps_history_separate_from_tentative_current_context(
    repository: SemesterRepository, runtime_directory: Path
) -> None:
    tools = ToolService(
        repository,
        PreferenceStore(runtime_directory / "preferences", repository),
    )

    context = json.loads(
        tools.execute("get_faculty_context", json.dumps({"faculty_name": "Bart Stander"}))
    )
    history = json.loads(
        tools.execute("get_previous_preferences", json.dumps({"faculty_name": "Bart Stander"}))
    )

    assert context["ok"] is True
    assert "preference_history" not in context["result"]["faculty"]
    assert history["ok"] is True
    assert [record["term"] for record in history["result"]["history"]] == [
        "Spring 2026",
        "Fall 2025",
    ]


def test_faculty_tools_require_a_reason_for_new_unavailable_times(
    repository: SemesterRepository, runtime_directory: Path
) -> None:
    tools = ToolService(
        repository,
        PreferenceStore(runtime_directory / "preferences", repository),
    )
    assert "unavailable_time_slot" in json.dumps(tools.definitions())
    response = json.loads(
        tools.execute(
            "preview_preferences",
            json.dumps(
                {
                    "faculty_name": "Bart Stander",
                    "days_to_check": "MT",
                    "preferences": [
                        {
                            "kind": "unavailable_time_slot",
                            "time_slot": "TR1500+75",
                            "comment": "I chair a university committee at this time.",
                        }
                    ],
                }
            ),
        )
    )
    assert response["ok"] is True
    assert "# Faculty exception: I chair a university committee" in response["result"]["snippet"]

    missing_reason = json.loads(
        tools.execute(
            "preview_preferences",
            json.dumps(
                {
                    "faculty_name": "Bart Stander",
                    "days_to_check": "MT",
                    "preferences": [
                        {
                            "kind": "unavailable_time_slot",
                            "time_slot": "TR1500+75",
                        }
                    ],
                }
            ),
        )
    )
    assert missing_reason["ok"] is False
    assert "comment" in missing_reason["error"]
