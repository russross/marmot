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
agent can continue through additional model rounds in the same response. Session logging
records complete requests, tool calls, and assistant messages rather than transient stream
fragments.

Data structures
---------------

`data/fall-2026.json` is the deployable `Semester` snapshot:

-   `Faculty` holds the chair's tentative sections, baseline availability, exact section
    setup calls, immutable department-approved unavailable slots, a possibly partial
    current-term example, and two ordered `PreferenceHistory` records. Each history record
    distinguishes an absent/new faculty member from a present faculty member with no
    preference snippet.
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
    coordination notes. Priorities and durations are range-constrained. Teaching changes
    and new hard-unavailability requests require explanatory comments.

Data flow
---------

1.  `scripts/install_semester.py` gathers a current database, current faculty source, and
    the two immediately preceding faculty sources into one snapshot. This is the only
    workflow that reads the wider Marmot tree. The Fall 2026 test install converts the
    legacy Fall 2025 preference call names; Spring 2026 and later sources are already in
    the current format.
2.  `SemesterRepository.load(path)` strictly validates that snapshot during server startup.
3.  `POST /api/chat` receives assistant-ui messages and passes them with the system prompt
    and server-owned tool schemas to `OpenRouterAgent.stream(...)`.
4.  `OpenRouterAgent` incrementally parses OpenRouter SSE chunks, executes complete model
    tool calls through `ToolService`, and yields typed text/tool/message events. The route
    maps those events to assistant-ui frames. Request, tool, response, and error records
    are appended through `SessionLog` at their complete semantic boundaries.
5.  `preview_preferences` and `save_preferences` parse the same `FacultySubmission`.
    `validate_submission(...)` checks solver preconditions and real allowed-set
    intersections. `render_submission(...)` produces the same Python API calls used by
    the current Marmot input.
6.  `PreferenceStore.save(...)` writes a temporary file and atomically replaces
    `runtime/preferences/<faculty-slug>.py`. Validation failures leave the prior file
    untouched.

Key interfaces
--------------

```text
SemesterRepository.load(path: Path) -> SemesterRepository
SemesterRepository.faculty(name: str) -> Faculty

validate_submission(
    repository: SemesterRepository,
    submission: FacultySubmission,
) -> Faculty

render_submission(faculty: Faculty, submission: FacultySubmission) -> str
PreferenceStore.save(submission: FacultySubmission) -> SaveResult

ToolService.definitions() -> list[tool schema]
ToolService.execute(name: str, arguments_json: str) -> str

OpenRouterAgent.stream(
    system_prompt: str,
    ui_messages: Sequence[UiMessage],
    session_id: str,
) -> AsyncIterator[AgentEvent]
```

Save semantics
--------------

The model never writes Python directly. It submits typed teaching changes, constraints,
preferences, and comments. The server preserves the installed faculty identity,
department, baseline availability, and department-approved unavailable slots. It applies
requested assignment additions/removals and room/time replacements immediately. Shared
section changes remain valid isolated Python while their requested tags are recorded in
comments for the department-wide integration pass. Hard time exclusions and teaching
changes are rendered with their rationale so exceptions remain visible.

The prompt requires a preview before saving and explicit faculty confirmation before the
write tool. There is intentionally no login or authorization layer for this firewall-only
deployment.

Semester migration
------------------

Migration remains an explicit installation step because Marmot semester preparation is
manual. The new semester directory and `--term` argument are authoritative. Provenance
records the current and two term-labeled historical source paths plus known metadata
mismatches for later debugging. After the JSON file is created and configured through
`MARMOT_DATA_FILE`, runtime code has no dependency on those source paths.
