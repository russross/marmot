from __future__ import annotations

from enum import StrEnum
from typing import Annotated, Literal

from pydantic import BaseModel, ConfigDict, Field, JsonValue, StringConstraints


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


class SectionConstraintUpdate(StrictModel):
    section: str
    room_tags: list[str]
    time_slot_tags: list[str]
    comment: Annotated[str, StringConstraints(min_length=1, max_length=500)] | None = None


class FacultySectionAddition(StrictModel):
    kind: Literal["add_section"]
    method: SectionSetupMethod
    section: str
    room_tags: list[str]
    time_slot_tags: list[str]
    comment: Annotated[str, StringConstraints(min_length=1, max_length=500)]


class FacultySectionRemoval(StrictModel):
    kind: Literal["remove_section"]
    section: str
    comment: Annotated[str, StringConstraints(min_length=1, max_length=500)]


SectionChange = Annotated[
    FacultySectionAddition | FacultySectionRemoval,
    Field(discriminator="kind"),
]


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


class Course(StrictModel):
    code: str
    department: str
    name: str


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
    current_faculty_source: str
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


Priority = Annotated[int, Field(ge=10, le=24)]
DurationMinutes = Annotated[int, Field(gt=50, lt=720)]


class WantADayOff(StrictModel):
    kind: Literal["want_a_day_off"]
    priority: Priority | None = None


class DoNotWantADayOff(StrictModel):
    kind: Literal["do_not_want_a_day_off"]
    priority: Priority | None = None


class WantClassesEvenlySpreadAcrossDays(StrictModel):
    kind: Literal["want_classes_evenly_spread_across_days"]
    priority: Priority | None = None


class WantBackToBackClassesInTheSameRoom(StrictModel):
    kind: Literal["want_back_to_back_classes_in_the_same_room"]
    priority: Priority | None = None


class WantClassesPackedIntoAsFewRoomsAsPossible(StrictModel):
    kind: Literal["want_classes_packed_into_as_few_rooms_as_possible"]
    priority: Priority | None = None


class AvoidGapBetweenClassClustersShorterThan(StrictModel):
    kind: Literal["avoid_gap_between_class_clusters_shorter_than"]
    minutes: DurationMinutes
    priority: Priority | None = None


class AvoidGapBetweenClassClustersLongerThan(StrictModel):
    kind: Literal["avoid_gap_between_class_clusters_longer_than"]
    minutes: DurationMinutes
    priority: Priority | None = None


class AvoidClassClusterShorterThan(StrictModel):
    kind: Literal["avoid_class_cluster_shorter_than"]
    minutes: DurationMinutes
    priority: Priority | None = None


class AvoidClassClusterLongerThan(StrictModel):
    kind: Literal["avoid_class_cluster_longer_than"]
    minutes: DurationMinutes
    priority: Priority | None = None


class AvoidSectionInRooms(StrictModel):
    kind: Literal["avoid_section_in_rooms"]
    section: str
    room_tags: Annotated[list[str], Field(min_length=1)]
    priority: Priority | None = None


class AvoidSectionInTimeSlots(StrictModel):
    kind: Literal["avoid_section_in_time_slots"]
    section: str
    time_slot_tags: Annotated[list[str], Field(min_length=1)]
    priority: Priority | None = None


class AvoidTimeSlot(StrictModel):
    kind: Literal["avoid_time_slot"]
    time_slot: str
    priority: Priority | None = None


class UnavailableTimeSlot(StrictModel):
    kind: Literal["unavailable_time_slot"]
    time_slot: str
    comment: Annotated[str, StringConstraints(min_length=1, max_length=500)]


class UseSameTimePattern(StrictModel):
    kind: Literal["use_same_time_pattern"]
    sections: Annotated[list[str], Field(min_length=2)]
    priority: Priority | None = None


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
    | UnavailableTimeSlot
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


class FacultySubmission(StrictModel):
    faculty_name: str
    days_to_check: Annotated[str, StringConstraints(pattern=r"^[MTWRFSU]+$")]
    section_changes: list[SectionChange] = Field(default_factory=list)
    section_constraints: list[SectionConstraintUpdate] = Field(default_factory=list)
    preferences: list[Preference]
    coordination_notes: list[CoordinationNote] = Field(default_factory=list)


class SaveResult(StrictModel):
    faculty_name: str
    path: str
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
