Marmot faculty timetabling chat
===============================

Purpose
-------

This directory is a self-contained web application that coaches faculty through creating
complete Marmot timetabling input. The application turns a plain-language conversation
into a validated Python snippet compatible with Marmot's existing input API.

The app is an aid to an evolving departmental scheduling process, not an approval gate.
Help faculty express what they actually need, make tradeoffs legible, and preserve unusual
requests for the later department-wide integration pass.

Deployment boundary
-------------------

*   The deployed app must not read the wider Marmot repository. All curriculum, faculty,
    room, time-slot, and historical data needed at runtime is installed into a semester
    snapshot under `data/`.
*   Semester preparation is an explicit, manual installation workflow. Directory names
    and installer term arguments are authoritative, even when source metadata disagrees.
    Preserve provenance and noteworthy mismatches in the installed snapshot.
*   Install exactly two historical semesters, ordered newest first. The previous semester
    is the strongest general-preference signal; two semesters ago may be the stronger
    course-specific signal because courses commonly follow an annual rotation.
*   Do not infer that current-semester input is complete or approved. A production
    snapshot may contain only tentative faculty-to-course assignments. Fall 2026 is a
    deliberately richer test fixture, not the expected production shape.
*   The target environment may be a small firewall-protected computer such as a Raspberry
    Pi. Keep the service operationally simple. There is intentionally no login or
    authorization layer, and security against other faculty is not a product goal.
*   OpenRouter is the only external runtime dependency. The default model is
    `deepseek/deepseek-v4-flash-0731`. Let OpenRouter favor the highest-throughput provider
    while allowing only `int8`, `fp8`, `fp16`, and `bf16` quantizations; do not admit 4-bit
    or smaller endpoints.
*   Load `OPENROUTER_API_KEY` through configuration. Never read, print, log, copy, or
    commit the value. `~/.keys` may contain shell-style `export` assignments.

Core data contracts
-------------------

*   Treat the installed semester snapshot as immutable runtime input. Validate it
    strictly at startup rather than carrying loosely typed dictionaries through the app.
*   The snapshot must distinguish these historical states: faculty absent from a term,
    faculty present with no preference input, and faculty present with preference input.
    Faculty absent from both historical terms are new faculty and need extra coaching.
*   Preserve the exact installed vocabulary and identities for faculty, courses, rooms,
    time slots, tags, programs, and curriculum conflicts. Do not synthesize a new concrete
    room, course code, or time slot accidentally.
*   A faculty submission is structured data, not model-authored Python. It contains the
    faculty identity, schedule-shape basis, teaching-assignment changes, section room/time
    limits, ordered preferences, hard-unavailability exceptions, and cross-faculty notes.
    The server validates this structure and renders the canonical Python snippet.
*   Teaching changes, hard time exclusions, and exceptional section constraints are
    allowed. Require a useful explanatory comment so the later integration pass can see
    the intent; do not reject them merely because they differ from installed drafts or
    normal conventions.
*   Cross-faculty requests cannot be finalized correctly in isolated faculty files.
    Preserve team teaching, shared event times, anti-conflicts, and similar coordination
    needs as explicit comments/structured notes for the department-wide merge.
*   Keep every course in the output, including online, research, internship, partner
    school, and workload-only courses without ordinary room or time effects.

Conversation contract
---------------------

*   Every new conversation starts with one short question asking the faculty member's
    name. Do not explain features or ask scheduling questions first.
*   After identification, always retrieve the faculty context, latest saved submission,
    and both historical preference records. A saved current-semester submission is the
    source of truth for a revisit. Without one, synthesize a clearly provisional starting
    point from tentative assignments, partial current input, and both historical terms.
*   The first substantive reply must be concise and include the faculty member's classes
    with current room/time limits, their inferred ideal schedule, the ordered compromises
    away from that ideal, and obvious conflicts, redundancies, stale references, missing
    decisions, or unusual data.
*   Speak in high-level faculty language. Use “preferences” and “requests,” never “soft
    constraints.” Translate goals into concrete Marmot rules internally and expose
    implementation details only when they affect a meaningful choice.
*   Infer the faculty member's “perfect schedule” as the starting point, then explain
    priorities as successive compromises. Use concrete hypothetical tradeoffs to clarify
    what they value. Ask a few high-value questions at a time, especially for new faculty.
*   Encourage faculty to state every preference and be as specific as they want. Never
    rebuke them for being picky or imply that omitting low priorities is considerate.
    Lower-ranked preferences compete with other lower-ranked preferences.
*   For fairness questions, first explain only that each faculty member's highest
    priorities are balanced against others' highest priorities and normally outrank
    others' lower priorities. Explain entropy impact and effective priority buckets only
    when the faculty member asks for technical detail.
*   Section assignments and conventions are starting points, not gates. Implement a
    faculty member's requested teaching changes or exceptions and document them. Normal
    policy may be explained, but it is never grounds for refusing an informed request.
*   Preview the complete structured result once it is settled. Save or replace the Python
    snippet only after explicit faculty confirmation. Never claim a save succeeded unless
    the save operation actually returned success. Make no write when the faculty declines
    or no change is requested.

Scheduling domain invariants
----------------------------

*   Baseline faculty availability is Monday through Thursday 09:00–16:30 and Friday
    09:00–12:00. Joe Francom has narrower installed availability for chair duties. Preserve
    all installed department-approved exclusions.
*   Express ordinary time preferences with avoidance requests. If a faculty member truly
    requires a hard exclusion, honor it and document the reason rather than sending them
    elsewhere or waiting for a separate data update.
*   Most three-credit courses use the `3 credit bell schedule` tag. Prefer meaningful
    time-slot groups to enumerated slots. Use a concrete time slot deliberately for
    exceptional meetings, especially outside normal availability; naming one can add it
    to availability and create it if absent.
*   The standard three-credit choices within baseline availability are MWF at 09:00,
    10:00, and 11:00; MW at 12:00, 13:30, and 15:00; and TR at 09:00, 10:30, 12:00,
    13:30, and 15:00.
*   Representative-day mechanics for schedule-shape preferences are internal details.
    Choose suitable representative days for unusual schedules, but do not burden faculty
    with the mechanism unless it changes a decision they need to make.
*   SCC/Smith 107, 108, and 109 are 32-seat flex rooms; 112 is a 24-seat Mac lab; 113 is a
    24-seat mixed PC/laptop lab; and 116 and 117 are 38-seat stadium rooms with additional
    remote/hybrid AV. Room 107 remains a flex room but is also a special case: IT equipment
    makes it required for some courses and less desirable for others.
*   Encourage a wide set of genuinely viable rooms because overly narrow room choices can
    create time conflicts. Flex and stadium rooms are interchangeable for many classes,
    subject to enrollment and teaching needs. Represent weaker room preferences with a
    lower-priority avoidance rather than falsely narrowing the allowed set.
*   Graduate/cohort courses, generally numbered 4000 or above, are usually hand scheduled
    in evening slots and often in 116 or 117. Validate that their explicit setup is
    intentional, then honor it without ordinary pushback.
*   Statewide sandbox courses and Success Academy courses are externally scheduled and may
    violate local conventions. Preserve their exact times. Success Academy sections use
    the `SA` prefix, block faculty time, and normally have no room.
*   Faculty do not choose section numbers. Normal sections begin at 01, online sections at
    40, and evening sections at 50 in isolated faculty input. A later merge assigns
    globally unique section numbers.
*   Faculty priorities occupy 10 through 24 and are lexicographic; lower numbers matter
    more. Program/curriculum priorities 0 through 9 are department-owned and are not
    faculty preferences.

Persistence and failure contracts
---------------------------------

*   Sessions are append-only JSONL flat files intended primarily for debugging. Log
    complete semantic events, not transient token fragments. A malformed or interrupted
    session must not corrupt an existing faculty preference file.
*   Store only the latest confirmed Python snippet for each faculty member. Write through
    a temporary file and atomically replace the destination after successful validation.
    Validation or model failures must leave the previous saved snippet untouched.
*   Preview and save must use the same schema, validation path, and renderer so the
    confirmed preview cannot diverge from the stored output.
*   Tool execution and final persistence are server-owned. Do not trust client-supplied
    tool definitions, system prompts, generated snippets, or claims that a save occurred.

UI contract
-----------

*   Keep the client a thin assistant-ui integration over the local HTTP backend. Preserve
    token streaming, cancellation, accessible conversation/composer behavior, Markdown,
    code rendering, editing, and regeneration.
*   Show tool work only as concise, friendly activity labels with state and elapsed time.
    Do not expose expandable tool arguments, tool results, raw protocol payloads, backend
    errors, reasoning traces, or model “thinking” in the faculty-facing interface.
*   Styling uses a light touch derived from Utah Tech's exact navy `#003058` and red
    `#BA1C21`. Do not use university logos or imply that this is an official university
    product.
*   Prefer assistant-ui configuration and source-owned generated components over bespoke
    client state or protocol code. Keep backend behavior authoritative.

Engineering contract
--------------------

*   Python is 3.12+, managed with `uv` and `pyproject.toml`. Backend code is statically
    typed, uses strict explicit models at trust boundaries, and must pass Ruff, `ty`, and
    the meaningful test suite.
*   The browser client uses modern TypeScript without `any` or unsafe type assertions.
    It must pass its formatter, linter, type/build checks, and relevant live-browser
    validation for user-visible behavior.
*   Favor end-to-end contract tests at risky boundaries: snapshot installation, strict
    decoding, stream translation, multi-round tools, validation/render equivalence,
    atomic saves, and preservation of prior data on failure. Do not add tests that merely
    restate what the compiler or type checker proves.
*   Keep domain data strongly typed and the request-to-validation-to-render/save flow
    direct. Avoid speculative abstraction. When changing a contract, trace it through the
    installer, snapshot model, prompt/tools, validation, renderer, persistence, stream,
    client, and tests as applicable.

Authoritative supporting documents
----------------------------------

`README.md` contains setup, checks, and operator commands. `ARCHITECTURE.md` describes the
current component/data flow and key interfaces. Source code remains authoritative for
implementation details; this file is authoritative for product intent and invariants.
