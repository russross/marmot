from __future__ import annotations

from enum import StrEnum
from typing import Annotated, Literal, Self

from pydantic import (
    AliasChoices,
    BaseModel,
    ConfigDict,
    Field,
    JsonValue,
    StringConstraints,
    model_validator,
)


class StrictModel(BaseModel):
    model_config = ConfigDict(extra="forbid", strict=True)


class AvailabilityInterval(StrictModel):
    days: Annotated[str, StringConstraints(pattern=r"^[MTWRFSU]+$")]
    start: Annotated[str, StringConstraints(pattern=r"^[0-2][0-9][0-5][0-9]$")]
    end: Annotated[str, StringConstraints(pattern=r"^[0-2][0-9][0-5][0-9]$")]


class SectionSetupMethod(StrEnum):
    MAKE = "make_faculty_section"
    ASSIGN = "assign_faculty_to_existing_section"


class SectionSetup(StrictModel):
    method: SectionSetupMethod
    name: str
    tags: list[str]


class FacultySection(StrictModel):
    name: str
    room_tags: list[str]
    time_slot_tags: list[str]


class PreferenceHistory(StrictModel):
    term: str
    faculty_present: bool
    preferences: str | None
    section_setup: list[SectionSetup] = Field(default_factory=list)


class Faculty(StrictModel):
    name: str
    department: str
    availability: list[AvailabilityInterval]
    sections: list[FacultySection]
    section_setup: list[SectionSetup]
    approved_unavailable_time_slots: list[str]
    current_preferences: str | None
    preference_history: Annotated[list[PreferenceHistory], Field(min_length=2, max_length=2)]


class Room(StrictModel):
    name: str
    capacity: int


class TimeSlot(StrictModel):
    name: str
    days: str
    start_minutes: int
    duration_minutes: int


class CourseSchedulingPolicy(StrEnum):
    SECTION_DEFINED = "section_defined"
    NEVER_SCHEDULED = "never_scheduled"
    EXTERNALLY_SCHEDULED = "externally_scheduled"


class Course(StrictModel):
    code: str
    department: str
    name: str
    minimum_credit_hours: Annotated[float, Field(ge=0, allow_inf_nan=False)]
    maximum_credit_hours: Annotated[float, Field(ge=0, allow_inf_nan=False)]
    scheduling_policy: CourseSchedulingPolicy
    contact_minutes_override: Annotated[int, Field(gt=0)] | None

    @model_validator(mode="after")
    def validate_credit_range(self) -> Self:
        if self.maximum_credit_hours < self.minimum_credit_hours:
            raise ValueError("maximum_credit_hours must be at least minimum_credit_hours")
        return self


class Conflict(StrictModel):
    name: str
    priority: int | None
    mode: Literal["boost", "reduce"]
    members: list[str]


class Program(StrictModel):
    name: str
    department: str
    conflicts: list[Conflict]


class HistoricalFacultySource(StrictModel):
    term: str
    faculty_source: str


class Provenance(StrictModel):
    database: str
    current_assignment_source: str = Field(
        validation_alias=AliasChoices("current_assignment_source", "current_faculty_source")
    )
    historical_faculty_sources: Annotated[
        list[HistoricalFacultySource], Field(min_length=2, max_length=2)
    ]
    note: str


class Semester(StrictModel):
    term: str
    historical_terms: Annotated[list[str], Field(min_length=2, max_length=2)]
    provenance: Provenance
    faculty: list[Faculty]
    rooms: list[Room]
    room_tags: dict[str, list[str]]
    time_slots: list[TimeSlot]
    time_slot_tags: dict[str, list[str]]
    courses: list[Course]
    programs: list[Program]


class CurrentAssignment(StrictModel):
    spreadsheet_row: int
    spreadsheet_faculty_name: str | None
    course_code: str
    section: str
    title: str
    notes: str | None
    section_number_inferred: bool
    constraints_inferred_from: str | None
    setup_method: SectionSetupMethod
    room_tags: list[str]
    time_slot_tags: list[str]
    issues: list[str]


class AssignmentSourceDetails(StrictModel):
    url: str
    revision: str
    last_modified: str | None
    etag: str | None


class FacultyContext(StrictModel):
    faculty: Faculty
    assignments: list[CurrentAssignment]
    assignment_source: AssignmentSourceDetails
    issues: list[str]


DurationMinutes = Annotated[int, Field(gt=50, lt=720)]


class WantADayOff(StrictModel):
    kind: Literal["want_a_day_off"]


class DoNotWantADayOff(StrictModel):
    kind: Literal["do_not_want_a_day_off"]


class WantClassesEvenlySpreadAcrossDays(StrictModel):
    kind: Literal["want_classes_evenly_spread_across_days"]


class WantBackToBackClassesInTheSameRoom(StrictModel):
    kind: Literal["want_back_to_back_classes_in_the_same_room"]


class WantClassesPackedIntoAsFewRoomsAsPossible(StrictModel):
    kind: Literal["want_classes_packed_into_as_few_rooms_as_possible"]


class AvoidGapBetweenClassClustersShorterThan(StrictModel):
    kind: Literal["avoid_gap_between_class_clusters_shorter_than"]
    minutes: DurationMinutes


class AvoidGapBetweenClassClustersLongerThan(StrictModel):
    kind: Literal["avoid_gap_between_class_clusters_longer_than"]
    minutes: DurationMinutes


class AvoidClassClusterShorterThan(StrictModel):
    kind: Literal["avoid_class_cluster_shorter_than"]
    minutes: DurationMinutes


class AvoidClassClusterLongerThan(StrictModel):
    kind: Literal["avoid_class_cluster_longer_than"]
    minutes: DurationMinutes


class AvoidSectionInRooms(StrictModel):
    kind: Literal["avoid_section_in_rooms"]
    section: str
    room_tags: Annotated[list[str], Field(min_length=1)]


class AvoidSectionInTimeSlots(StrictModel):
    kind: Literal["avoid_section_in_time_slots"]
    section: str
    time_slot_tags: Annotated[list[str], Field(min_length=1)]


class AvoidTimeSlot(StrictModel):
    kind: Literal["avoid_time_slot"]
    time_slot: str


class UseSameTimePattern(StrictModel):
    kind: Literal["use_same_time_pattern"]
    sections: Annotated[list[str], Field(min_length=2)]


Preference = Annotated[
    WantADayOff
    | DoNotWantADayOff
    | WantClassesEvenlySpreadAcrossDays
    | WantBackToBackClassesInTheSameRoom
    | WantClassesPackedIntoAsFewRoomsAsPossible
    | AvoidGapBetweenClassClustersShorterThan
    | AvoidGapBetweenClassClustersLongerThan
    | AvoidClassClusterShorterThan
    | AvoidClassClusterLongerThan
    | AvoidSectionInRooms
    | AvoidSectionInTimeSlots
    | AvoidTimeSlot
    | UseSameTimePattern,
    Field(discriminator="kind"),
]


class CoordinationNoteKind(StrEnum):
    SHARED_EVENT_TIME = "shared_event_time"
    TEAM_TEACHING = "team_teaching"
    ANTI_CONFLICT = "anti_conflict"
    OTHER = "other"


class CoordinationNote(StrictModel):
    kind: CoordinationNoteKind
    text: Annotated[str, StringConstraints(min_length=1, max_length=500)]


class DecisionOrigin(StrEnum):
    FACULTY = "faculty"
    INFERRED = "inferred"
    SOURCE = "source"


class DecisionSummaryItem(StrictModel):
    origin: DecisionOrigin
    text: Annotated[str, StringConstraints(strip_whitespace=True, min_length=1, max_length=1000)]


class SectionSchedulingMode(StrEnum):
    SCHEDULED = "scheduled"
    UNSCHEDULED = "unscheduled"


class ProposedSection(StrictModel):
    name: str
    method: SectionSetupMethod
    scheduling_mode: SectionSchedulingMode
    credit_hours: Annotated[float, Field(ge=0, allow_inf_nan=False)] | None = Field(
        description=("Required for variable-credit courses and omitted for fixed-credit courses.")
    )
    room_tags: list[str]
    time_slot_tags: list[str]
    comment: Annotated[str, StringConstraints(min_length=1, max_length=500)] | None = None


class AssignmentChangeKind(StrEnum):
    ADD = "add"
    REMOVE = "remove"


class AssignmentChangeReason(StrictModel):
    kind: AssignmentChangeKind
    section: str
    comment: Annotated[str, StringConstraints(min_length=1, max_length=500)]


class HardUnavailability(StrictModel):
    time_slot: str
    university_conflict: Annotated[str, StringConstraints(min_length=1, max_length=500)]


class FacultySubmission(StrictModel):
    faculty_name: str
    assignment_revision: str
    decision_summary: Annotated[
        list[DecisionSummaryItem],
        Field(
            min_length=1,
            max_length=30,
            description=(
                "Factual faculty statements, inferences, and source notes relevant to the draft."
            ),
        ),
    ]
    days_to_check: Annotated[str, StringConstraints(pattern=r"^[MTWRFSU]+$")]
    sections: list[ProposedSection]
    assignment_changes: list[AssignmentChangeReason] = Field(default_factory=list)
    hard_unavailability: list[HardUnavailability] = Field(default_factory=list)
    preferences: list[Preference]
    coordination_notes: list[CoordinationNote] = Field(default_factory=list)


class SaveStatus(StrEnum):
    CREATED = "created"
    UPDATED = "updated"
    UNCHANGED = "unchanged"


class SaveResult(StrictModel):
    faculty_name: str
    path: str
    status: SaveStatus
    saved_revision: str
    submission: FacultySubmission
    snippet: str


class SavedSubmission(StrictModel):
    saved_revision: str
    submission: FacultySubmission
    snippet: str


class TextPart(StrictModel):
    type: Literal["text"]
    text: str


class ToolCallPart(StrictModel):
    type: Literal["tool-call"]
    tool_call_id: str = Field(alias="toolCallId")
    tool_name: str = Field(alias="toolName")
    input: JsonValue


class ToolResultOutput(StrictModel):
    type: Literal["json", "error-json"]
    value: JsonValue


class ToolResultPart(StrictModel):
    type: Literal["tool-result"]
    tool_call_id: str = Field(alias="toolCallId")
    tool_name: str = Field(alias="toolName")
    output: ToolResultOutput


UiMessagePart = Annotated[
    TextPart | ToolCallPart | ToolResultPart,
    Field(discriminator="type"),
]


class UiMessage(StrictModel):
    role: Literal["user", "assistant", "system", "tool"]
    content: str | list[UiMessagePart]

    def text(self) -> str:
        if isinstance(self.content, str):
            return self.content
        return "\n".join(part.text for part in self.content if isinstance(part, TextPart))


class ChatRequest(StrictModel):
    messages: list[UiMessage]
    system: str | None = None
    tools: dict[str, JsonValue] = Field(default_factory=dict)
    thread_id: str | None = Field(default=None, alias="threadId")
    session_id: str | None = Field(default=None, alias="sessionId")
    assistant_message_id: str | None = Field(default=None, alias="unstable_assistantMessageId")
    parent_id: str | None = Field(default=None, alias="parentId")
    run_config: dict[str, JsonValue] = Field(default_factory=dict, alias="runConfig")


class OpenRouterToolFunction(StrictModel):
    name: str
    arguments: str


class OpenRouterToolCall(StrictModel):
    id: str
    type: Literal["function"]
    function: OpenRouterToolFunction
    index: int | None = None


class OpenRouterAssistantMessage(StrictModel):
    role: Literal["assistant"]
    content: str | None = None
    tool_calls: list[OpenRouterToolCall] | None = None
    reasoning: str | None = None
    reasoning_details: list[JsonValue] | None = None
    refusal: str | None = None
    annotations: list[JsonValue] | None = None
    images: list[JsonValue] | None = None


class OpenRouterChoice(StrictModel):
    index: int
    message: OpenRouterAssistantMessage
    finish_reason: str | None
    native_finish_reason: str | None = None
    logprobs: JsonValue = None


class OpenRouterUsage(StrictModel):
    prompt_tokens: int = 0
    completion_tokens: int = 0
    total_tokens: int = 0
    prompt_tokens_details: dict[str, JsonValue] | None = None
    completion_tokens_details: dict[str, JsonValue] | None = None
    cost: float | None = None
    is_byok: bool | None = None
    cost_details: dict[str, JsonValue] | None = None


class OpenRouterResponse(StrictModel):
    id: str
    choices: list[OpenRouterChoice]
    usage: OpenRouterUsage | None = None
    created: int | None = None
    model: str | None = None
    object: Literal["chat.completion"] | None = None
    system_fingerprint: str | None = None
    service_tier: str | None = None
    provider: str | None = None
    openrouter_metadata: dict[str, JsonValue] | None = None


class OpenRouterStreamToolFunction(StrictModel):
    name: str | None = None
    arguments: str | None = None


class OpenRouterStreamToolCall(StrictModel):
    index: int
    id: str | None = None
    type: Literal["function"] | None = None
    function: OpenRouterStreamToolFunction | None = None


class OpenRouterStreamDelta(StrictModel):
    role: Literal["assistant"] | None = None
    content: str | None = None
    tool_calls: list[OpenRouterStreamToolCall] | None = None
    reasoning: str | None = None
    reasoning_details: list[JsonValue] | None = None
    refusal: str | None = None


class OpenRouterStreamChoice(StrictModel):
    index: int
    delta: OpenRouterStreamDelta
    finish_reason: str | None = None
    native_finish_reason: str | None = None
    logprobs: JsonValue = None


class OpenRouterStreamChunk(StrictModel):
    id: str
    choices: list[OpenRouterStreamChoice]
    usage: OpenRouterUsage | None = None
    created: int | None = None
    model: str | None = None
    object: Literal["chat.completion.chunk"] | None = None
    system_fingerprint: str | None = None
    service_tier: str | None = None
    provider: str | None = None
