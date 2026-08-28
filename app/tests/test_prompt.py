from timetable_chat.prompt import build_system_prompt
from timetable_chat.semester import SemesterRepository


def test_priority_guidance_escalates_from_fairness_to_implementation(
    spring_repository: SemesterRepository,
) -> None:
    prompt = build_system_prompt(spring_repository)
    normalized_prompt = " ".join(prompt.split())

    high_level = prompt.index("Their highest priorities are balanced")
    technical = prompt.index("log2(total / remaining) bits of entropy")
    implementation = prompt.index("partitioned into contiguous buckets")

    assert high_level < technical < implementation
    assert "Do not rebuke, disapprove of," in prompt
    assert "Do not volunteer the entropy or" in prompt
    assert 'inferred description of their ideal or "perfect" schedule' in prompt
    assert "Preferences are stored from most important to least important" in normalized_prompt
    assert "give up the last, lowest-priority preference first" in normalized_prompt
    assert "Never describe the first preference in the list as the first compromise" in (
        normalized_prompt
    )
    assert "Normal conventions are coaching defaults, never grounds for refusing" in prompt
    assert "Smith 107 is also a special case" in prompt
    assert "genuinely depends on Smith 107's IT equipment" in prompt
    assert "Never assume the live assignment spreadsheet is coherent" in prompt
    assert "collaboratively edited" in normalized_prompt
    assert "assignment_source.revision" in prompt
    assert "saved_revision" in prompt
    assert "use the older semester as potentially stronger evidence" in prompt
    assert "If both historical records say the faculty" in prompt
    assert "The initial inferred proposal is saved automatically" in prompt
    assert "Every submission requires a decision_summary" in prompt
    assert "never load or save another faculty member's draft" in prompt
    assert "Never turn an inference into faculty-provided input" in normalized_prompt
    assert "Keep section feasibility separate from faculty preferences" in prompt
    assert 'should have the "3 credit bell schedule" tag' in prompt
    assert "include the full set of room tags or concrete rooms" in prompt
    assert "Do not turn preferred rooms or times into section constraints" in prompt
    assert "obtain a direct, specific justification" in prompt
    assert "Treat that as the goal from which to derive ordered preferences" in prompt
    assert "`preferences` is empty" in prompt
    assert '"Faculty requested this time," "faculty preference,"' in prompt
    assert "Do not save a newly constrained concrete time" in prompt
    assert "scheduled variable-credit section must specify" in normalized_prompt
    assert "silently uses the catalog minimum" in normalized_prompt
    assert "Graduate courses are numbered 5000 and above" in prompt
    assert "Multiple time tags are alternative placements" in normalized_prompt
    assert "reserved for university obligations" in normalized_prompt
    assert "Fall 2026, then Spring 2026" in prompt
