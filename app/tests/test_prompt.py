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
    assert "sequence of compromises" in prompt
    assert "Normal conventions are coaching defaults, never grounds for refusing" in prompt
    assert "Smith 107 is also a special case" in prompt
    assert "genuinely depends on Smith 107's IT equipment" in prompt
    assert "Never assume the live assignment spreadsheet is coherent" in prompt
    assert "collaboratively edited" in normalized_prompt
    assert "assignment_source.revision" in prompt
    assert "present a fresh preview" in prompt
    assert "use the older semester as potentially stronger evidence" in prompt
    assert "If both historical records say the faculty" in prompt
    assert "meaningful changes have accumulated since their last" in prompt
    assert "saving is a checkpoint" in prompt
    assert "Do not pester them after every message" in prompt
    assert "Fall 2026, then Spring 2026" in prompt
