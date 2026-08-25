import json
from pathlib import Path

import pytest

from timetable_chat.assignments import AssignmentWorkbookClient
from timetable_chat.preferences import PreferenceStore
from timetable_chat.semester import SemesterRepository
from timetable_chat.tools import ToolService


@pytest.mark.asyncio
async def test_tool_preview_reports_validation_error_without_writing(
    spring_repository: SemesterRepository,
    assignment_client: AssignmentWorkbookClient,
    runtime_directory: Path,
) -> None:
    store = PreferenceStore(runtime_directory / "preferences", spring_repository)
    tools = ToolService(spring_repository, store, assignment_client)

    response = json.loads(
        await tools.execute(
            "preview_preferences",
            json.dumps(
                {
                    "faculty_name": "Lora Klein",
                    "assignment_revision": "fixture-revision-1",
                    "days_to_check": "MT",
                    "preferences": [{"kind": "want_classes_evenly_spread_across_days"}],
                }
            ),
        )
    )

    assert response["ok"] is False
    assert "more than three scheduleable sections" in response["error"]
    assert not (runtime_directory / "preferences").exists()

    stale = json.loads(
        await tools.execute(
            "save_preferences",
            json.dumps(
                {
                    "faculty_name": "Lora Klein",
                    "assignment_revision": "stale-revision",
                    "days_to_check": "MT",
                    "preferences": [],
                }
            ),
        )
    )
    assert stale["ok"] is False
    assert "assignments changed" in stale["error"]
    assert not (runtime_directory / "preferences").exists()


def test_installed_snapshot_distinguishes_recent_history_and_new_faculty(
    spring_repository: SemesterRepository,
) -> None:
    assert spring_repository.semester.term == "Spring 2027"
    assert spring_repository.semester.historical_terms == ["Fall 2026", "Spring 2026"]
    assert "No current-term faculty Python source" in spring_repository.semester.provenance.note
    bart = spring_repository.faculty("bart stander")
    assert bart.sections == []
    assert bart.section_setup == []
    assert bart.current_preferences is None
    recent, older = bart.preference_history
    assert recent.term == "Fall 2026"
    assert recent.faculty_present is True
    assert recent.preferences is not None
    assert recent.section_setup
    assert older.term == "Spring 2026"
    assert older.faculty_present is True
    assert older.preferences is not None
    assert any(section.name == "CS 3600-01" for section in older.section_setup)


@pytest.mark.asyncio
async def test_history_tool_keeps_history_separate_from_tentative_current_context(
    spring_repository: SemesterRepository,
    assignment_client: AssignmentWorkbookClient,
    runtime_directory: Path,
) -> None:
    tools = ToolService(
        spring_repository,
        PreferenceStore(runtime_directory / "preferences", spring_repository),
        assignment_client,
    )

    context = json.loads(
        await tools.execute("get_faculty_context", json.dumps({"faculty_name": "Bart Stander"}))
    )
    history = json.loads(
        await tools.execute(
            "get_previous_preferences", json.dumps({"faculty_name": "Bart Stander"})
        )
    )

    assert context["ok"] is True
    assert "preference_history" not in context["result"]["faculty"]
    assert history["ok"] is True
    assert [record["term"] for record in history["result"]["history"]] == [
        "Fall 2026",
        "Spring 2026",
    ]
    assignments = context["result"]["assignments"]
    graphics = next(row for row in assignments if row["course_code"] == "CS 3600")
    assert graphics["constraints_inferred_from"] == "Spring 2026"
    assert graphics["room_tags"] == ["pcs"]
    assert graphics["time_slot_tags"] == ["3 credit bell schedule"]


@pytest.mark.asyncio
async def test_faculty_tools_require_a_reason_for_new_unavailable_times(
    spring_repository: SemesterRepository,
    assignment_client: AssignmentWorkbookClient,
    runtime_directory: Path,
) -> None:
    tools = ToolService(
        spring_repository,
        PreferenceStore(runtime_directory / "preferences", spring_repository),
        assignment_client,
    )
    assert "unavailable_time_slot" in json.dumps(tools.definitions())
    response = json.loads(
        await tools.execute(
            "preview_preferences",
            json.dumps(
                {
                    "faculty_name": "Bart Stander",
                    "assignment_revision": "fixture-revision-1",
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
        await tools.execute(
            "preview_preferences",
            json.dumps(
                {
                    "faculty_name": "Bart Stander",
                    "assignment_revision": "fixture-revision-1",
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


@pytest.mark.asyncio
async def test_live_assignments_resolve_names_and_preserve_imprecision(
    spring_repository: SemesterRepository,
    assignment_client: AssignmentWorkbookClient,
    runtime_directory: Path,
) -> None:
    tools = ToolService(
        spring_repository,
        PreferenceStore(runtime_directory / "preferences", spring_repository),
        assignment_client,
    )

    faculty_list = json.loads(await tools.execute("list_faculty", "{}"))
    daley = json.loads(
        await tools.execute("get_faculty_context", '{"faculty_name":"Philip Daley"}')
    )
    kevin = json.loads(
        await tools.execute("get_faculty_context", '{"faculty_name":"Kevin Johnston"}')
    )
    kevin_history = json.loads(
        await tools.execute("get_previous_preferences", '{"faculty_name":"Kevin Johnston"}')
    )

    assert "Phil Daley" in faculty_list["result"]["faculty"]
    assert "Kevin Johnston" in faculty_list["result"]["faculty"]
    assert [row["section"] for row in daley["result"]["assignments"]] == [
        "IT 1100-01",
        "IT 1100-02",
    ]
    stander = json.loads(
        await tools.execute("get_faculty_context", '{"faculty_name":"Bart Stander"}')
    )
    assert [
        row["section"]
        for row in stander["result"]["assignments"]
        if row["course_code"] == "CS 3600"
    ] == ["CS 3600-01"]
    assert all(row["section_number_inferred"] for row in daley["result"]["assignments"])
    assert all(not record["faculty_present"] for record in kevin_history["result"]["history"])
    assert any("possible match: Kevin Johnson" in issue for issue in kevin["result"]["issues"])
    assert faculty_list["result"]["unassigned_courses"][0]["course_code"] == "CS 1030"
