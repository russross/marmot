Architecture and concept validation
===================================

Outcome
-------

assistant-ui is a suitable client for this app. Its Data Stream runtime is specifically
intended for custom HTTP backends and supplies conversation state, editing, regeneration,
stream cancellation, Markdown, code rendering, scrolling, and accessible composer
behavior. The generated UI components are source-owned shadcn-style components rather
than a sealed drop-in widget. We keep those generated components unchanged and limit the
integration surface to `frontend/app/assistant.tsx`.

The server translates OpenRouter's SSE stream into assistant-ui's Data Stream protocol.
Text deltas reach the browser as they arrive. Tool-call names and argument fragments open
visible activity items before server-side execution; results settle those items, and the
agent can continue through additional model rounds in the same response. Reasoning remains
hidden from the faculty interface but is preserved across tool rounds for models that need
it. Session logging records complete requests, tool calls, and assistant messages rather
than transient stream fragments.

Data structures
---------------

`data/spring-2027.json` is the deployable `Semester` snapshot:

-   `Faculty` holds the most recent historical baseline availability, immutable
    department-approved unavailable slots, and two ordered `PreferenceHistory` records.
    Current sections, setup calls, and preferences are empty in the installed Spring 2027
    record. Each history record distinguishes an absent/new faculty member from a present
    faculty member with no preference snippet and retains historical section setups for
    course-specific inference.
-   `FacultySection` holds the effective allowed room and time tags used for validation and
    explanation.
-   `SectionSetup` preserves whether the current input used
    `make_faculty_section(...)` or `assign_faculty_to_existing_section(...)` and preserves
    its ordered tag arguments for canonical output.
-   `Room`, `TimeSlot`, tag maps, `Course`, `Program`, and `Conflict` provide the installed
    constraint and curriculum vocabulary. Courses include their catalog credit range,
    scheduling policy, and any exceptional weekly contact-minute requirement.
-   `FacultySubmission` is the complete save boundary, not a patch over the workbook. Its
    `ProposedSection` records every course, setup method, scheduling mode, allowed room/time
    tags, and variable credit value. It also contains assignment-change reasons, ordered
    preferences, university-related hard unavailability, coordination notes, and decision
    summary items labeled as faculty input, inference, or source evidence. Workbook and
    saved-artifact revisions prevent stale writes.
-   `SpreadsheetAssignment` represents one decoded workbook row. `CurrentAssignment`
    preserves its source row, title, note, resolved isolated section name, inferred setup
    source, room/time tags, and explicit issues. `FacultyContext` combines those live rows
    with historical identity, availability, and preference history.

Data flow
---------

1.  `scripts/install_semester.py` gathers the current curriculum database and Fall 2026
    and Spring 2026 faculty sources into one snapshot. It never reads a Spring 2027
    `computingfaculty.py`. This is the only workflow that reads the wider Marmot tree.
2.  `SemesterRepository.load(path)` strictly validates that snapshot during server startup.
3.  `AssignmentWorkbookClient.fetch()` downloads the Spring 2027 XLSX for every
    assignment-bearing tool call. The parser bounds the download and XML-part sizes,
    rejects unsafe XML, locates the exact seven-column header, excludes fully struck-out
    assignment rows, and ignores unrelated or incomplete planning rows. Each remaining
    valid assignment retains its notes. Name resolution prefers exact historical identity,
    then an unambiguous surname match. Missing section numbers receive deterministic
    isolated-input numbers and an explicit warning. Course constraints are inferred from
    the same-season historical setup when unambiguous, then from the other historical term.
    A SHA-256 content revision identifies the exact workbook used for each context.
4.  `POST /api/chat` receives assistant-ui messages and passes them with the system prompt
    and server-owned tool schemas to `OpenRouterAgent.stream(...)`. Tool schemas use a
    provider-neutral JSON Schema subset: references are inlined, nullable fields are
    optional, and provider-specific strict-output and union keywords are omitted.
5.  `OpenRouterAgent` incrementally parses OpenRouter SSE chunks, executes complete model
    tool calls through `ToolService`, and yields typed text/tool/message events. The route
    maps those events to assistant-ui frames. Request, tool, response, and error records
    are appended through `SessionLog` at their complete semantic boundaries.
6.  `load_faculty_workspace` downloads and resolves the workbook once, then returns that
    coherent assignment revision with the saved typed draft and its revision, both history
    records, relevant course metadata, and discrepancies. Live discrepancies are reported
    without mutating the saved draft.
7.  `save_faculty_submission` re-downloads assignments and rejects stale workbook or saved
    revisions. It converts the flat model-facing preference shape into the canonical
    discriminated preference types, normalizes silent defaults, validates the complete
    submission, and renders the canonical Python. Contact-minute checks expand every
    allowed time tag and use exact section credits plus installed course exceptions.
8.  `PreferenceStore.save(...)` embeds the typed submission in comments, writes a temporary
    file, and atomically replaces `runtime/preferences/<faculty-slug>.py`. Identical saves
    are no-ops. Validation failures leave the prior artifact untouched.

Key interfaces
--------------

```text
SemesterRepository.load(path: Path) -> SemesterRepository
SemesterRepository.faculty(name: str) -> Faculty

AssignmentWorkbookClient.fetch() -> AssignmentWorkbook
resolve_faculty_contexts(
    repository: SemesterRepository,
    workbook: AssignmentWorkbook,
) -> tuple[dict[str, FacultyContext], list[CurrentAssignment]]

validate_submission(
    repository: SemesterRepository,
    submission: FacultySubmission,
    faculty: Faculty | None = None,
) -> Faculty

render_submission(faculty: Faculty, submission: FacultySubmission) -> str
PreferenceStore.save(
    submission: FacultySubmission,
    faculty: Faculty | None = None,
    expected_saved_revision: str | None = None,
) -> SaveResult

ToolService.definitions() -> list[tool schema]
ToolService.execute(name: str, arguments_json: str) -> Awaitable[str]
ToolFacultySubmission.to_submission() -> FacultySubmission
provider_tool_schema(schema: dict[str, JsonValue]) -> dict[str, JsonValue]

OpenRouterAgent.stream(
    system_prompt: str,
    ui_messages: Sequence[UiMessage],
    session_id: str,
) -> AsyncIterator[AgentEvent]
```

Save semantics
--------------

The model never writes Python directly. It submits a complete typed working draft. The
server preserves the resolved faculty identity, standard availability, and installed
department-approved unavailable slots. It infers fixed catalog credits, silently chooses
the minimum for unscheduled variable-credit sections when omitted, and requires an explicit
value for scheduled variable-credit sections. Shared sections obtain their credit from the
section creator.

The initial inferred draft and each actionable revision are saved automatically before the
assistant describes them. The artifact separates faculty statements, inferences, and source
notes for audit. Faculty edit only their own draft; cross-faculty requests remain coordination
comments for the merge. There is intentionally no login or authorization layer for this
firewall-only deployment.

Semester migration
------------------

Migration remains an explicit installation step because Marmot semester preparation is
manual. The new semester directory and `--term` argument are authoritative. Provenance
records the live assignment URL and two term-labeled historical source paths. After the
JSON file is created and configured through `MARMOT_DATA_FILE`, runtime code has no
dependency on those filesystem source paths. It depends only on the installed snapshot and
the configured assignment workbook URL.
