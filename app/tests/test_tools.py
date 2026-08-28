import json
from pathlib import Path
from typing import cast

import pytest
from pydantic import JsonValue

from timetable_chat.assignments import AssignmentWorkbookClient
from timetable_chat.models import FacultySubmission
from timetable_chat.preferences import PreferenceStore
from timetable_chat.semester import SemesterRepository
from timetable_chat.tools import ToolPreference, ToolService


def submission_from_workspace(workspace: dict[str, JsonValue]) -> dict[str, JsonValue]:
    assignments = cast(list[dict[str, JsonValue]], workspace["assignments"])
    courses = {
        cast(str, course["code"]): course
        for course in cast(list[dict[str, JsonValue]], workspace["courses"])
    }
    sections: list[dict[str, JsonValue]] = []
    for assignment in assignments:
        course = courses[cast(str, assignment["course_code"])]
        time_tags = cast(list[JsonValue], assignment["time_slot_tags"])
        minimum = cast(float, course["minimum_credit_hours"])
        maximum = cast(float, course["maximum_credit_hours"])
        variable_credit = minimum != maximum
        sections.append(
            {
                "name": assignment["section"],
                "method": assignment["setup_method"],
                "scheduling_mode": "scheduled" if time_tags else "unscheduled",
                "credit_hours": minimum if variable_credit and not time_tags else None,
                "room_tags": assignment["room_tags"],
                "time_slot_tags": time_tags,
                "comment": (
                    "This externally planned placement is retained from the assignment source."
                    if any("+" in cast(str, tag) for tag in time_tags)
                    else None
                ),
            }
        )
    assignment_source = cast(dict[str, JsonValue], workspace["assignment_source"])
    faculty = cast(dict[str, JsonValue], workspace["faculty"])
    return {
        "faculty_name": faculty["name"],
        "assignment_revision": assignment_source["revision"],
        "decision_summary": [
            {
                "origin": "inferred",
                "text": "Keep the tentative assignments as the initial working draft.",
            }
        ],
        "days_to_check": "MT",
        "sections": sections,
        "assignment_changes": [],
        "hard_unavailability": [],
        "preferences": [],
        "coordination_notes": [],
    }


@pytest.mark.asyncio
async def test_workspace_load_is_coherent_and_includes_course_credit_data(
    spring_repository: SemesterRepository,
    assignment_client: AssignmentWorkbookClient,
    runtime_directory: Path,
) -> None:
    tools = ToolService(
        spring_repository,
        PreferenceStore(runtime_directory / "preferences", spring_repository),
        assignment_client,
    )

    response = json.loads(
        await tools.execute("load_faculty_workspace", json.dumps({"faculty_name": "Bart Stander"}))
    )

    assert response["ok"] is True
    workspace = response["result"]
    assert workspace["assignment_source"]["revision"] == "fixture-revision-1"
    assert [record["term"] for record in workspace["history"]] == [
        "Fall 2026",
        "Spring 2026",
    ]
    graphics = next(
        assignment
        for assignment in workspace["assignments"]
        if assignment["course_code"] == "CS 3600"
    )
    assert graphics["constraints_inferred_from"] == "Spring 2026"
    assert graphics["room_tags"] == ["pcs"]
    course = next(course for course in workspace["courses"] if course["code"] == "CS 2100")
    assert course["minimum_credit_hours"] == 3.0
    assert course["maximum_credit_hours"] == 3.0
    assert workspace["saved"] is None
    assert workspace["saved_revision"] is None


@pytest.mark.asyncio
async def test_save_tool_creates_typed_draft_and_workspace_loads_it(
    spring_repository: SemesterRepository,
    assignment_client: AssignmentWorkbookClient,
    runtime_directory: Path,
) -> None:
    store = PreferenceStore(runtime_directory / "preferences", spring_repository)
    tools = ToolService(spring_repository, store, assignment_client)
    loaded = json.loads(
        await tools.execute("load_faculty_workspace", json.dumps({"faculty_name": "Bart Stander"}))
    )["result"]
    submission = submission_from_workspace(loaded)

    saved = json.loads(
        await tools.execute(
            "save_faculty_submission",
            json.dumps(
                {
                    "expected_saved_revision": loaded["saved_revision"],
                    "submission": submission,
                }
            ),
        )
    )

    assert saved["ok"] is True
    assert saved["result"]["status"] == "created"
    assert saved["result"]["submission"] == submission
    reloaded = json.loads(
        await tools.execute("load_faculty_workspace", json.dumps({"faculty_name": "Bart Stander"}))
    )["result"]
    assert reloaded["saved"]["submission"] == submission
    assert reloaded["saved_revision"] == saved["result"]["saved_revision"]
    assert reloaded["discrepancies"] == []


@pytest.mark.asyncio
async def test_save_rejects_stale_workbook_and_saved_revisions_without_writing(
    spring_repository: SemesterRepository,
    assignment_client: AssignmentWorkbookClient,
    runtime_directory: Path,
) -> None:
    store = PreferenceStore(runtime_directory / "preferences", spring_repository)
    tools = ToolService(spring_repository, store, assignment_client)
    workspace = json.loads(
        await tools.execute("load_faculty_workspace", json.dumps({"faculty_name": "Lora Klein"}))
    )["result"]
    submission = submission_from_workspace(workspace)
    submission["assignment_revision"] = "stale"

    stale_workbook = json.loads(
        await tools.execute(
            "save_faculty_submission",
            json.dumps(
                {
                    "expected_saved_revision": None,
                    "submission": submission,
                }
            ),
        )
    )

    assert stale_workbook["ok"] is False
    assert "assignments changed" in stale_workbook["error"]
    assert not (runtime_directory / "preferences").exists()

    submission["assignment_revision"] = "fixture-revision-1"
    stale_saved = json.loads(
        await tools.execute(
            "save_faculty_submission",
            json.dumps(
                {
                    "expected_saved_revision": "stale",
                    "submission": submission,
                }
            ),
        )
    )
    assert stale_saved["ok"] is False
    assert "saved preferences changed" in stale_saved["error"]
    assert not (runtime_directory / "preferences").exists()


@pytest.mark.asyncio
async def test_workspace_highlights_live_assignments_without_overriding_saved_draft(
    spring_repository: SemesterRepository,
    assignment_client: AssignmentWorkbookClient,
    runtime_directory: Path,
) -> None:
    store = PreferenceStore(runtime_directory / "preferences", spring_repository)
    tools = ToolService(spring_repository, store, assignment_client)
    workspace = json.loads(
        await tools.execute("load_faculty_workspace", json.dumps({"faculty_name": "Bart Stander"}))
    )["result"]
    submission = submission_from_workspace(workspace)
    removed_section = cast(list[dict[str, JsonValue]], submission["sections"]).pop()["name"]
    submission["assignment_changes"] = [
        {
            "kind": "remove",
            "section": removed_section,
            "comment": "The faculty member reports that this assignment moved elsewhere.",
        }
    ]
    saved = json.loads(
        await tools.execute(
            "save_faculty_submission",
            json.dumps(
                {
                    "expected_saved_revision": None,
                    "submission": submission,
                }
            ),
        )
    )
    assert saved["ok"] is True

    reloaded = json.loads(
        await tools.execute("load_faculty_workspace", json.dumps({"faculty_name": "Bart Stander"}))
    )["result"]

    saved_names = {section["name"] for section in reloaded["saved"]["submission"]["sections"]}
    assert removed_section not in saved_names
    assert any(removed_section in message for message in reloaded["discrepancies"])


@pytest.mark.parametrize(
    "wire_preference",
    [
        {"kind": "want_a_day_off"},
        {"kind": "do_not_want_a_day_off"},
        {"kind": "want_classes_evenly_spread_across_days"},
        {"kind": "want_back_to_back_classes_in_the_same_room"},
        {"kind": "want_classes_packed_into_as_few_rooms_as_possible"},
        {"kind": "avoid_gap_between_class_clusters_shorter_than", "minutes": 60},
        {"kind": "avoid_gap_between_class_clusters_longer_than", "minutes": 120},
        {"kind": "avoid_class_cluster_shorter_than", "minutes": 180},
        {"kind": "avoid_class_cluster_longer_than", "minutes": 240},
        {"kind": "avoid_section_in_rooms", "section": "CS 2100-01", "room_tags": ["pcs"]},
        {
            "kind": "avoid_section_in_time_slots",
            "section": "CS 2100-01",
            "time_slot_tags": ["late"],
        },
        {"kind": "avoid_time_slot", "time_slot": "TR0900+75"},
        {
            "kind": "use_same_time_pattern",
            "sections": ["CS 2100-01", "CS 2420-01"],
        },
    ],
)
def test_provider_neutral_preferences_convert_to_canonical_variants(
    wire_preference: dict[str, JsonValue],
) -> None:
    tool_preference = ToolPreference.model_validate_json(json.dumps(wire_preference), strict=True)

    assert tool_preference.to_preference().model_dump(mode="json") == wire_preference


@pytest.mark.parametrize(
    "wire_preference",
    [
        {"kind": "avoid_section_in_rooms", "section": "CS 2100-01"},
        {"kind": "want_a_day_off", "minutes": 60},
    ],
)
def test_provider_neutral_preferences_reject_incomplete_or_irrelevant_fields(
    wire_preference: dict[str, JsonValue],
) -> None:
    tool_preference = ToolPreference.model_validate_json(json.dumps(wire_preference), strict=True)

    with pytest.raises(ValueError):
        tool_preference.to_preference()


def test_tool_schema_exposes_provenance_and_university_only_hard_blocks(
    spring_repository: SemesterRepository,
    assignment_client: AssignmentWorkbookClient,
    runtime_directory: Path,
) -> None:
    tools = ToolService(
        spring_repository,
        PreferenceStore(runtime_directory / "preferences", spring_repository),
        assignment_client,
    )

    serialized_definitions = json.dumps(tools.definitions())

    def schema_keys(value: JsonValue) -> set[str]:
        if isinstance(value, list):
            return set().union(*(schema_keys(item) for item in value))
        if not isinstance(value, dict):
            return set()
        return set(value) | set().union(*(schema_keys(item) for item in value.values()))

    definitions = tools.definitions()
    emitted_schema_keys = schema_keys(definitions)

    assert "decision_summary" in serialized_definitions
    assert "university_conflict" in serialized_definitions
    assert "credit_hours" in serialized_definitions
    assert "expected_saved_revision" in serialized_definitions
    assert "preview_preferences" not in serialized_definitions
    assert '"priority"' not in serialized_definitions
    assert FacultySubmission.model_fields["decision_summary"].is_required()
    assert {"$defs", "$ref", "anyOf", "oneOf", "discriminator", "strict"}.isdisjoint(
        emitted_schema_keys
    )
