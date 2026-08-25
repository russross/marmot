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
    constraint and curriculum vocabulary.
-   `FacultySubmission` is the save boundary. It contains typed teaching-assignment
    additions/removals, optional replacements for a section's room/time tags, a
    discriminated union of faculty preference types, and structured cross-faculty
    coordination notes. It carries the workbook content revision used for stale-preview
    detection. Priorities and durations are range-constrained. Teaching changes and new
    hard-unavailability requests require explanatory comments.
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
    rejects unsafe XML, validates the exact seven-column schema, excludes fully struck-out
    assignment rows, and retains incomplete active rows and notes. Name resolution prefers
    exact historical identity, then an unambiguous surname match. Missing section numbers
    receive deterministic isolated-input numbers and an explicit warning. Course
    constraints are inferred from the same-season historical setup when unambiguous, then
    from the other historical term. A SHA-256 content revision identifies the exact
    workbook used for each context.
4.  `POST /api/chat` receives assistant-ui messages and passes them with the system prompt
    and server-owned tool schemas to `OpenRouterAgent.stream(...)`.
5.  `OpenRouterAgent` incrementally parses OpenRouter SSE chunks, executes complete model
    tool calls through `ToolService`, and yields typed text/tool/message events. The route
    maps those events to assistant-ui frames. Request, tool, response, and error records
    are appended through `SessionLog` at their complete semantic boundaries.
6.  `preview_preferences` and `save_preferences` re-download current assignments, parse
    the same `FacultySubmission`, and use the same resolved `Faculty` context. A stale
    assignment revision rejects preview or save, so a changed workbook cannot produce a
    stored snippet different from the confirmed preview.
    `validate_submission(...)` checks solver preconditions and real allowed-set
    intersections. `render_submission(...)` produces the same Python API calls used by
    Marmot input.
7.  `PreferenceStore.save(...)` writes a temporary file and atomically replaces
    `runtime/preferences/<faculty-slug>.py`. Validation failures leave the prior file
    untouched.

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
) -> SaveResult

ToolService.definitions() -> list[tool schema]
ToolService.execute(name: str, arguments_json: str) -> Awaitable[str]

OpenRouterAgent.stream(
    system_prompt: str,
    ui_messages: Sequence[UiMessage],
    session_id: str,
) -> AsyncIterator[AgentEvent]
```

Save semantics
--------------

The model never writes Python directly. It submits typed teaching changes, constraints,
preferences, and comments. The server preserves the resolved faculty identity,
department, historical baseline availability, and department-approved unavailable slots.
It applies requested assignment additions/removals and room/time replacements immediately.
Shared section changes remain valid isolated Python while their requested tags are recorded
in comments for the department-wide integration pass. Hard time exclusions and teaching
changes are rendered with their rationale so exceptions remain visible.

The prompt requires a preview before saving and explicit faculty confirmation before the
write tool. There is intentionally no login or authorization layer for this firewall-only
deployment.

Semester migration
------------------

Migration remains an explicit installation step because Marmot semester preparation is
manual. The new semester directory and `--term` argument are authoritative. Provenance
records the live assignment URL and two term-labeled historical source paths. After the
JSON file is created and configured through `MARMOT_DATA_FILE`, runtime code has no
dependency on those filesystem source paths. It depends only on the installed snapshot and
the configured assignment workbook URL.
