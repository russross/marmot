from pathlib import Path

import pytest

from timetable_chat.models import (
    AssignmentChangeKind,
    AssignmentChangeReason,
    DecisionOrigin,
    DecisionSummaryItem,
    FacultySubmission,
    HardUnavailability,
    ProposedSection,
    SaveStatus,
    SectionSchedulingMode,
    SectionSetupMethod,
)
from timetable_chat.preferences import PreferenceStore
from timetable_chat.semester import SemesterRepository


def submission_with_section(
    *,
    faculty_name: str,
    section: ProposedSection,
    summary: str = "Use this section setup as the current working draft.",
) -> FacultySubmission:
    return FacultySubmission(
        faculty_name=faculty_name,
        assignment_revision="test-revision",
        decision_summary=[DecisionSummaryItem(origin=DecisionOrigin.INFERRED, text=summary)],
        days_to_check="MT",
        sections=[section],
        assignment_changes=[
            AssignmentChangeReason(
                kind=AssignmentChangeKind.ADD,
                section=section.name,
                comment="This is the current teaching assignment proposal.",
            )
        ],
        preferences=[],
    )


def test_variable_credit_defaults_only_for_unscheduled_sections(
    spring_repository: SemesterRepository,
    runtime_directory: Path,
) -> None:
    store = PreferenceStore(runtime_directory / "preferences", spring_repository)
    unscheduled = submission_with_section(
        faculty_name="Ren Quinn",
        section=ProposedSection(
            name="CS 4800R-01",
            method=SectionSetupMethod.MAKE,
            scheduling_mode=SectionSchedulingMode.UNSCHEDULED,
            credit_hours=None,
            room_tags=[],
            time_slot_tags=[],
        ),
    )

    saved = store.save(unscheduled)

    assert saved.submission.sections[0].credit_hours == 1
    assert "credit_hours=1" in saved.snippet
    assert saved.status is SaveStatus.CREATED

    scheduled = submission_with_section(
        faculty_name="Ren Quinn",
        section=ProposedSection(
            name="SE 4990-01",
            method=SectionSetupMethod.MAKE,
            scheduling_mode=SectionSchedulingMode.SCHEDULED,
            credit_hours=None,
            room_tags=["flex"],
            time_slot_tags=["3 credit bell schedule"],
        ),
    )
    with pytest.raises(ValueError, match="must specify credit_hours"):
        PreferenceStore(runtime_directory / "other", spring_repository).save(scheduled)


def test_fixed_credit_is_inferred_and_contact_minutes_are_validated(
    spring_repository: SemesterRepository,
    runtime_directory: Path,
) -> None:
    store = PreferenceStore(runtime_directory / "preferences", spring_repository)
    valid = submission_with_section(
        faculty_name="Ren Quinn",
        section=ProposedSection(
            name="CS 4480R-01",
            method=SectionSetupMethod.MAKE,
            scheduling_mode=SectionSchedulingMode.SCHEDULED,
            credit_hours=None,
            room_tags=["flex", "stadium"],
            time_slot_tags=["3 credit bell schedule"],
        ),
    )

    saved = store.save(valid)

    section_line = next(
        line for line in saved.snippet.splitlines() if "make_faculty_section" in line
    )
    assert "credit_hours" not in section_line

    invalid = valid.model_copy(
        update={
            "sections": [
                valid.sections[0].model_copy(
                    update={
                        "time_slot_tags": ["F1400+50"],
                        "comment": "Use the standard morning placement.",
                    }
                )
            ]
        }
    )
    with pytest.raises(ValueError, match="requires 150 contact minutes"):
        PreferenceStore(runtime_directory / "other", spring_repository).save(invalid)


def test_externally_scheduled_course_preserves_nonstandard_contact_minutes(
    spring_repository: SemesterRepository,
    runtime_directory: Path,
) -> None:
    store = PreferenceStore(runtime_directory / "preferences", spring_repository)
    submission = submission_with_section(
        faculty_name="Lora Klein",
        section=ProposedSection(
            name="SA 1400-01",
            method=SectionSetupMethod.MAKE,
            scheduling_mode=SectionSchedulingMode.SCHEDULED,
            credit_hours=None,
            room_tags=[],
            time_slot_tags=["TR0930+80"],
            comment="Success Academy sets this meeting time externally.",
        ),
    )

    saved = store.save(submission)

    assert "TR0930+80" in saved.snippet


def test_saved_artifact_round_trips_typed_state_and_provenance_safely(
    spring_repository: SemesterRepository,
    runtime_directory: Path,
) -> None:
    store = PreferenceStore(runtime_directory / "preferences", spring_repository)
    submission = submission_with_section(
        faculty_name="Bart Stander",
        summary="Prefer a compact schedule.\nraise RuntimeError('not code')",
        section=ProposedSection(
            name="CS 2100-01",
            method=SectionSetupMethod.MAKE,
            scheduling_mode=SectionSchedulingMode.SCHEDULED,
            credit_hours=None,
            room_tags=["flex", "stadium"],
            time_slot_tags=["3 credit bell schedule"],
        ),
    )

    saved = store.save(submission)
    loaded = store.read_submission("Bart Stander")

    assert loaded is not None
    assert loaded.saved_revision == saved.saved_revision
    assert loaded.submission == saved.submission
    assert "# Inferences currently used" in saved.snippet
    compile(saved.snippet, saved.path, "exec")


def test_stale_or_invalid_update_preserves_previous_artifact(
    spring_repository: SemesterRepository,
    runtime_directory: Path,
) -> None:
    store = PreferenceStore(runtime_directory / "preferences", spring_repository)
    submission = submission_with_section(
        faculty_name="Bart Stander",
        section=ProposedSection(
            name="CS 2100-01",
            method=SectionSetupMethod.MAKE,
            scheduling_mode=SectionSchedulingMode.SCHEDULED,
            credit_hours=None,
            room_tags=["flex"],
            time_slot_tags=["3 credit bell schedule"],
        ),
    )
    saved = store.save(submission)
    original = Path(saved.path).read_text(encoding="utf-8")

    unchanged = store.save(submission, expected_saved_revision=saved.saved_revision)
    assert unchanged.status is SaveStatus.UNCHANGED
    assert unchanged.saved_revision == saved.saved_revision

    with pytest.raises(ValueError, match="changed after they were loaded"):
        store.save(submission, expected_saved_revision="stale")
    assert Path(saved.path).read_text(encoding="utf-8") == original

    invalid = submission.model_copy(
        update={"sections": [submission.sections[0].model_copy(update={"time_slot_tags": []})]}
    )
    with pytest.raises(ValueError, match="must have allowed times"):
        store.save(invalid, expected_saved_revision=saved.saved_revision)
    assert Path(saved.path).read_text(encoding="utf-8") == original


def test_university_conflict_is_rendered_outside_ranked_preferences(
    spring_repository: SemesterRepository,
    runtime_directory: Path,
) -> None:
    store = PreferenceStore(runtime_directory / "preferences", spring_repository)
    submission = FacultySubmission(
        faculty_name="Bart Stander",
        assignment_revision="test-revision",
        decision_summary=[
            DecisionSummaryItem(
                origin=DecisionOrigin.FACULTY,
                text="A university committee meets Thursday afternoon.",
            )
        ],
        days_to_check="MT",
        sections=[],
        hard_unavailability=[
            HardUnavailability(
                time_slot="TR1500+75",
                university_conflict="Required university committee meeting.",
            )
        ],
        preferences=[],
    )

    saved = store.save(submission)

    assert "# University conflict: Required university committee meeting." in saved.snippet
    assert "UnavailableTimeSlot('TR1500+75')" in saved.snippet
