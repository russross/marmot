from pathlib import Path

import pytest

from timetable_chat.models import (
    AvoidSectionInRooms,
    CoordinationNote,
    CoordinationNoteKind,
    FacultySectionAddition,
    FacultySectionRemoval,
    FacultySubmission,
    SectionConstraintUpdate,
    SectionSetupMethod,
    WantADayOff,
    WantClassesEvenlySpreadAcrossDays,
)
from timetable_chat.preferences import PreferenceStore
from timetable_chat.semester import SemesterRepository


def test_complete_snippet_preserves_shared_section_assignment_mode(
    repository: SemesterRepository, runtime_directory: Path
) -> None:
    store = PreferenceStore(runtime_directory / "preferences", repository)
    submission = FacultySubmission(
        faculty_name="DS Hire",
        days_to_check="MT",
        preferences=[],
    )

    result = store.save(submission)

    assert "db.make_faculty_section('DS Hire', 'CS 4420-01'" in result.snippet
    assert "db.assign_faculty_to_existing_section('DS Hire', 'CS 4480R-01')" in result.snippet
    assert "db.assign_faculty_to_existing_section('DS Hire', 'CS 2420-02')" in result.snippet
    compile(result.snippet, result.path, "exec")


def test_invalid_update_leaves_previous_submission_untouched(
    repository: SemesterRepository, runtime_directory: Path
) -> None:
    store = PreferenceStore(runtime_directory / "preferences", repository)
    accepted = FacultySubmission(
        faculty_name="Bart Stander",
        days_to_check="MT",
        preferences=[WantADayOff(kind="want_a_day_off")],
    )
    saved = store.save(accepted)
    original = Path(saved.path).read_text(encoding="utf-8")
    rejected = FacultySubmission(
        faculty_name="Bart Stander",
        days_to_check="MT",
        preferences=[
            AvoidSectionInRooms(
                kind="avoid_section_in_rooms",
                section="CS 2100-01",
                room_tags=["pcs"],
            )
        ],
    )

    with pytest.raises(ValueError, match="does not intersect"):
        store.save(rejected)

    assert Path(saved.path).read_text(encoding="utf-8") == original


def test_even_spread_validation_ignores_online_workload_sections(
    repository: SemesterRepository, runtime_directory: Path
) -> None:
    store = PreferenceStore(runtime_directory / "preferences", repository)
    submission = FacultySubmission(
        faculty_name="Joe Francom",
        days_to_check="MT",
        preferences=[
            WantClassesEvenlySpreadAcrossDays(kind="want_classes_evenly_spread_across_days")
        ],
    )

    with pytest.raises(ValueError, match="more than three scheduleable sections"):
        store.save(submission)

    assert len(repository.faculty("Joe Francom").sections) == 4


def test_constraint_update_and_coordination_note_survive_complete_render(
    repository: SemesterRepository, runtime_directory: Path
) -> None:
    store = PreferenceStore(runtime_directory / "preferences", repository)
    submission = FacultySubmission(
        faculty_name="Bart Stander",
        days_to_check="MT",
        section_constraints=[
            SectionConstraintUpdate(
                section="CS 2100-01",
                room_tags=["flex", "stadium"],
                time_slot_tags=["3 credit bell schedule", "R1900+120"],
            )
        ],
        preferences=[],
        coordination_notes=[
            CoordinationNote(
                kind=CoordinationNoteKind.SHARED_EVENT_TIME,
                text="Coordinate this course with the Senior Project presentations.",
            )
        ],
    )

    result = store.save(submission)

    assert (
        "db.make_faculty_section('Bart Stander', 'CS 2100-01', "
        "'3 credit bell schedule', 'R1900+120', 'flex', 'stadium')"
    ) in result.snippet
    assert (
        "# Coordination note (shared event time): Coordinate this course with the "
        "Senior Project presentations."
    ) in result.snippet
    compile(result.snippet, result.path, "exec")


def test_department_approved_unavailability_is_immutable_faculty_context(
    repository: SemesterRepository, runtime_directory: Path
) -> None:
    joe = repository.faculty("Joe Francom")
    store = PreferenceStore(runtime_directory / "preferences", repository)
    result = store.save(
        FacultySubmission(
            faculty_name=joe.name,
            days_to_check="MT",
            preferences=[],
        )
    )

    assert joe.approved_unavailable_time_slots == [
        "MW1200+75",
        "MW1500+75",
        "TR1500+75",
    ]
    assert result.snippet.count("UnavailableTimeSlot(") == 3


def test_teaching_changes_are_applied_and_explained_in_the_snippet(
    repository: SemesterRepository, runtime_directory: Path
) -> None:
    store = PreferenceStore(runtime_directory / "preferences", repository)
    result = store.save(
        FacultySubmission(
            faculty_name="Bart Stander",
            days_to_check="MT",
            section_changes=[
                FacultySectionRemoval(
                    kind="remove_section",
                    section="CS 4995-01",
                    comment="The chair reassigned this supervision workload.",
                ),
                FacultySectionAddition(
                    kind="add_section",
                    method=SectionSetupMethod.MAKE,
                    section="CS 1410-01",
                    room_tags=["flex", "stadium"],
                    time_slot_tags=["3 credit bell schedule"],
                    comment="This replaces the reassigned supervision workload.",
                ),
            ],
            preferences=[],
        )
    )

    assert "omit CS 4995-01. The chair reassigned" in result.snippet
    assert "make_faculty_section('Bart Stander', 'CS 4995-01'" not in result.snippet
    assert "add CS 1410-01. This replaces" in result.snippet
    assert (
        "db.make_faculty_section('Bart Stander', 'CS 1410-01', "
        "'3 credit bell schedule', 'flex', 'stadium')"
    ) in result.snippet
    compile(result.snippet, result.path, "exec")
