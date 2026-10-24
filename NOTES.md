Data input and solver contract
==============================

Status: design proposal and implementation audit, 2026-10-18. The current implementation is described separately below. Proposed tables and interfaces do not exist yet. This document plans the input migration; it does not authorize changes to optimization algorithms or scoring semantics.

The target is a durable SQLite database as the primary home of inputs, with scripts used for initial seeding and subsequent explicit imports. Administrators and faculty edit those same input records through validated transactions. Database views implement scheduling policy. Both solvers consume a compact, versioned output contract. Rebuilding derived data never deletes editable inputs or schedule history.


Established requirements
------------------------

*   Curriculum conflict priorities occupy 0–9, with smaller numbers stronger. Priority 0 survives every automatic filter and discount. Faculty stated ranks occupy 10–99; balanced effective priorities occupy 10–49.
*   Compute conflicts within each program, apply program-specific exceptions, then merge programs by minimum priority. Keep a small number of meaningful tiers. Broad, lower-tier elective requirements are explicitly ignored for scheduling, not assigned a weak tier.
*   Count online and other unscheduled offerings when discounting conflicts. Actual term offerings determine participation; catalog rotations are advisory. Preserve prerequisite/corequisite information even for courses outside the scheduling scope.
*   Anticonflicts are manual rules: one specific section must use the exact same time slot as any member of a group. Matching starts alone is insufficient. Preserve this semantics in both solvers.
*   Inputs, corrections, policy annotations, and overrides must round-trip through an editing UI without reconstructing Python. Preserve individual faculty requests through expansion and reprioritization. Solver cleanup concerns data loading and representation, not changes to search or SAT algorithms.


Current operation and audit baseline
------------------------------------

`data/build` creates Spring 2027. It deletes its working-directory `timetable.db`, loads `data/schema.sql`, creates the term and holidays, and invokes `courses.build_courses`, `computing.build_pre`, `computingfaculty.build_faculty`, and `computing.build_post`. It validates reciprocal shared-day-off preferences and commits. This disposable-build lifecycle must end before direct database editing starts.

The inspected `data/timetable.db` contains 334 courses, 82 sections, 61 distinct scheduled sections, six program records, and 31 conflict rules. Its canonical unordered conflict counts by priority are: 0:103, 1:117, 2:53, 3:8, 4:16, 5:77, 6:47, 7:11. These describe the existing database, not a freshly rebuilt or approved submission. `data/build` and the database identify the term as Spring 2027; the separate faculty app deliberately uses live assignment data and historical faculty input rather than treating this file as the final faculty submission.

### Catalog to term offerings

`data/scrape` fetches course descriptions for the configured prefixes, extracts credits, rotations, and referenced prerequisite/corequisite course codes, adds corrections, and emits `courses.py`. It flattens prerequisite Boolean expressions into edges. Concurrently permitted prerequisites are represented in both `prereqs` and `coreqs`. Senior-project prerequisite additions and ECE/MTRN corequisite corrections live inside the scraper. These corrections are policy/data inputs that must become visible records.

`data/queries.py` populates SQLite through a transaction. Section names encode their course; fixed-credit courses infer section credits, while variable-credit courses require a value. Room/time tags define allowed sets. Exact time-slot tags can bypass an assigned faculty member's base availability for that section only, but never explicit hard unavailability. All assigned faculty must be available. Soft room/time preferences intersect allowed options and never add options.

`sections_to_be_scheduled` selects sections with time tags and maps cross-listed aliases to their primary section. Sections without time tags remain in the database and count toward multiple-section discounts. Secondary cross-listed sections cannot own faculty, room tags, or time tags. The present model has one repeated weekly meeting pattern and at most one assigned room per scheduled section.

### Curriculum to conflict pairs

`undiscounted_conflict_pairs` expands course/course, course/section, and section/section cliques, maps cross-listings to primary sections, and groups by program and ordered section pair. Within a program, the strongest boost wins; the weakest reduction can weaken or cancel it, but a hard boost survives reductions. Reductions alone introduce no conflict. Programs then merge by minimum priority. Only scheduled endpoints survive this view.

`prereq_transitive_closure` unions directed prerequisite and corequisite edges, computes reachability, and removes directly listed corequisite pairs. `conflict_pairs` checks this relation in both directions. It then adds five priority levels for each additional section at either endpoint and drops results reaching 10. Same-course spread rules skip this discount. Same-faculty pairs are added as hard conflicts after curriculum processing. Both orientations are exported; the Rust loader selects one and constructs symmetric hard-conflict adjacency.

Current computing policy contains information beyond the degree requirements: priority-5 within-track rules and a priority-7 cross-track rule for Data Science; a special CS 4995 rule; explicit same-course section spreads; five anticonflicts; and section-count overrides of one for CS 4600 and SE 4600. Import these as explicit policy records, not inferred catalog requirements. IT and IT DevOps currently share a program record; the target should identify the published programs separately while retaining shared requirements.

### Faculty to solver criteria

`faculty_preferences` assigns omitted ranks in list order, resetting the next rank after an explicit rank. It spreads requests across scalar columns, interval rows, and room/time target rows. Some inputs collapse by minimum priority during insertion or view expansion, and original request identities are not retained consistently.

`src/input.rs::load_input` opens the database read-only, loads options, conflicts, anticonflicts, pattern matches, assignments, and preferences, computes faculty room-count targets, builds bundled criteria, and expands bundles again for entropy balancing. Balanced loading uses faculty-owned room/time views; stated-priority loading uses collapsed option penalties. `src/faculty_preferences.rs` groups owned criteria by faculty and stated rank, counts local schedules, assigns effective priorities, and merges reciprocal shared-day-off requests. Both SAT and search use the resulting `Input`; `src/sat_criteria.rs` performs further representation expansion for SAT.

Shared-day-off requests preserve two stated ranks, join both higher-ranked preference prefixes for entropy calculations, and become one scored criterion. Individual day-off requests remain separate. Faculty gap/cluster rules, room-switch counts, the first-short-cluster waiver, and threshold deduplication have existing scoring semantics; changing data representation must preserve them. The authoritative implementation references are `src/score.rs`, `src/faculty_preferences.rs`, and `src/sat_encoders.rs`, with behavior documentation in `src/AGENTS.md`.

The faculty app already has typed submissions, stale-write checks, and structured historical states. Its flow is installed semester JSON plus a live assignment workbook, then validated submissions embedded in generated Python files. `app/scripts/install_semester.py` extracts historical Python with AST parsing. `app/src/timetable_chat/preferences.py` validates, renders, and atomically saves snippets. Replace those persistence/import boundaries, retaining its useful submission model, assignment provenance, and distinction between absent faculty, faculty without input, and faculty with input. No new login system is part of this plan; administrative versus faculty ownership describes edit authority at the service boundary.

### Findings requiring explicit migration treatment

| Finding | Evidence and treatment |
| ------- | ---------------------- |
| Hard curriculum conflicts can disappear | The prerequisite `WHERE` clause applies even when priority is 0. An isolated in-memory fixture confirmed the loss. Correct this under the established hard-survival requirement; no such pair was found in the inspected term database. |
| Corequisite-only chains suppress endpoints | An isolated A-corequires-B-corequires-C fixture produces an A/C prerequisite exclusion. Recommend requiring a strict prerequisite edge somewhere on a suppressing path; keep this separate from a representation-only migration. |
| Prerequisite alternatives lose their logic | The scraper stores referenced codes, not AND/OR structure. Retain this as an explicitly named scheduling heuristic until a separate policy decision changes it; do not describe reachability as proof that concurrent enrollment is impossible. |
| Section discovery depends on surviving options | `load_sections` discovers sections from available time rows. A scheduled section with no options can disappear or cause a later missing-reference error. Export the section roster independently and report empty domains explicitly. |
| Input provenance is lossy | Unknown conflict courses can be printed and skipped; faculty requests can collapse; catalog corrections are embedded in code. New imports must distinguish unresolved references from intentionally excluded courses and retain request/correction identity. |

Other relevant limitations: cross-listing normalization must remove self-pairs and preserve all contributing course identities before discounts; empty anticonflict groups can disappear through joins; stated and balanced faculty loading currently take different paths; and the default database path in `src/main.rs` is working-directory-relative (`../data/timetable.db`). These are input-contract work, not solver algorithm work. Root and `data/` currently have no Makefile; add explicit build/check entry points during implementation.


Degree-sheet model and ingestion
--------------------------------

The published bachelor inventory is seven Computing programs, four Mathematics program/emphasis entries, three Design emphases, and four Engineering programs. BA/BS alternatives may share a scheduling projection when their differences are outside the owned courses. Integrated Studies, associates, certificates, and graduate programs are outside automatic program ingestion here. Existing graduate section offerings remain valid timetable inputs. Sources: [Computing](https://catalog.utahtech.edu/programs/computing/), [Mathematics](https://catalog.utahtech.edu/programs/mathematics/), [Design](https://catalog.utahtech.edu/programs/design/), and [Engineering](https://catalog.utahtech.edu/programs/engineering/).

Representative requirements establish why sets alone are insufficient:

*   [Software Engineering](https://catalog.utahtech.edu/programs/computing/software-engineering-bs/) has a database alternative, a choice of a complete track, and track courses that also appear in an elective pool. An alternative is local to its requirement, not a universal course-pair cancellation.
*   [Data Science](https://catalog.utahtech.edu/programs/computing/data-science-bs/) requires two tracks, and its capstone course is taken twice. Choosing two bundles requires cross-bundle conflicts, unlike choosing one. Repeat requirements are not multiple distinct course identities.
*   [Interaction Design](https://catalog.utahtech.edu/programs/design/design-interaction-design-bs/index.html) allows DES 1100, a bundle of DES 1101/1102/1103, or a skills exam. The bundle's internal requirements survive even though it competes with other alternatives.
*   [Mathematics](https://catalog.utahtech.edu/programs/mathematics/mathematics-ba-bs/index.html) has a MATH 2200/CS 2100 alternative, variable-credit electives, and MATH 4100 in both required and elective lists. Preserve and flag overlap rather than silently deciding double-counting policy. The sample graduation plan is advisory evidence, not an override of the requirement table.
*   [Computer Engineering](https://catalog.utahtech.edu/programs/engineering/computer-engineering-bs/index.html) has course/lab bundles, fractional credits, prefix/range elective selectors with exclusions, and subset credit limits. Credit counts cannot generally be converted to course counts. Broad pools can be stored but ignored for scheduling.

Store two related structures: course-set expressions describe which courses belong to a pool; requirement trees describe how students use courses or bundles. Neither structure stores generated conflict pairs as editable input.

### Proposed catalog and requirement tables

Table notation lists important columns and keys, not complete DDL. IDs are stable database identities, independent of display names and list position. Foreign keys, unique keys, and discriminant checks are required. Child rows are typed relations rather than opaque JSON blobs.

| Table | Shape and ownership |
| ----- | ------------------- |
| `source_documents` | `source_id PK, url, catalog_year, fetched_at, content_hash, source_text`; immutable imported evidence, including headings/footnote locators referenced by records. |
| `catalog_revisions`, `course_facts` | Revision ID, source ID; `(revision_id, course_id) PK`, code, title, owning department, credit range, rotations. Retain external referenced courses; ownership is a separate scheduling decision. |
| `catalog_corrections` and typed correction details | Stable correction ID, target fact/edge, action, value, reason, author, revision. Corrections overlay imported evidence and survive re-import. Include de facto prerequisite additions with an explicit policy origin. |
| `programs`, `program_versions` | Stable program ID, department, award/emphasis; version ID, catalog revision, source ID, root requirement, review state. A term explicitly selects applicable versions; old cohorts need not disappear when a new catalog is imported. |
| `course_sets`, `course_set_nodes`, `course_set_edges` | Named reusable expression roots; node kinds `members`, `selector`, `union`, `intersection`, `difference`; ordered children for difference. Acyclic expressions, revision-bound evaluation. |

| Table | Shape and ownership |
| ----- | ------------------- |
| `course_set_members`, `course_set_selectors` | Explicit course IDs; or typed prefix, numeric range, and suffix policy. No executable SQL predicates. Exclusions are set difference. New catalog matches produce a reviewable membership diff. |
| `requirements`, `requirement_children` | Version-owned nodes with label, source locator, kind; parent/child/order. Kinds: `course`, `all`, `choose_children`, `choose_courses`, `choose_credits`, `noncourse`. Shared sets can be referenced by multiple requirement occurrences. |
| Typed requirement details | Course ID and required repetitions; selected-child count; course-set ID and required course count; course-set ID and credit minimum/maximum; or noncourse evidence. Credit quantities use exact scaled integers or decimal serialization, not binary-float equality. |
| `requirement_limits` | Requirement, subset set, minimum/maximum credits or count, repetition cap, and documented double-counting scope. Preserve constraints even when irrelevant to the scheduling projection. |
| `requirement_scheduling` | Requirement ID, mode `include`/`ignore`, exposure tier, cross-child tier, reason, origin `manual`/`suggested`/`imported`. Ignoring a node prunes its scheduling subtree. Effective policy is separate from source requirements. |

Keep an actual requirement tree per version; reuse course sets, not mutable requirement nodes with multiple parents. This makes context and UI editing unambiguous. A requirement occurrence can be included in one program and ignored in another. A course appearing through two requirements retains both occurrences until pair contributions are merged.

An LLM extracts typed draft requirements with source locators and explicit unresolved items. It never invents priorities from prose or executes generated SQL/Python. Deterministic validation checks course resolution, tree shape, counts, credit units, and references. Initial review maps retained requirements to a few named tiers and marks broad pools ignored. Later imports create a source and requirement diff; they do not overwrite edited records or annotations. Matching uses stable external keys/locators plus explicit reconciliation when a heading changes, not an LLM guess that silently transfers overrides.

Noncourse requirements such as grades, residency, and exams are retained as evidence without scheduling effects. General education and broad elective universes need not be enumerated merely to ignore them. An unresolved included requirement prevents activation; an explicitly ignored requirement can retain source text without full normalization. A catalog change to an ignored pool should not require new conflict work unless its scheduling annotation changes.


Clique inclusion workflow
-------------------------

Implemented 2026-10-24: reusable membership extends the current clique model independently of the proposed requirement-tree migration. Direct members remain in `conflict_courses` and `conflict_sections`; `conflict_includes(program, conflict_name, included_conflict_name)` stores edges with a composite primary key and same-program foreign keys to both `conflicts` records. Multiple included groups are supported. Database triggers reject self-inclusion and transitive cycles on insert/update. Deletion of referenced groups is restricted so an edit cannot silently remove inherited membership; existing rename cascades update references.

`DB.make_conflict` accepts keyword-only `include: Sequence[str] = ()`, naming groups in the same program. Existing positional calls remain valid, and an empty direct member list permits a union-only clique. Included groups must exist before the call; this avoids deferred reference validation and matches the existing seed order. Python stores edges rather than copying members and raises a descriptive error for missing groups. Failed calls roll back the transaction using the existing API convention.

A recursive `conflict_membership_closure(program, conflict_name, source_name)` view starts with each group's own identity and follows include edges using `UNION`, not `UNION ALL`. Separate `expanded_conflict_courses` and `expanded_conflict_sections` views join that closure to direct members and deduplicate by receiving group and member. All four pairing branches in `undiscounted_conflict_pairs` consume those expanded views. Expansion precedes section enumeration and cross-listing normalization. Each receiving clique applies its own priority and boost/reduce mode to the entire expanded membership; inclusion does not inherit rule settings or exceptions. Source cliques still contribute their own rules independently. Existing per-program reductions, cross-program merging, prerequisite filtering, section-count discounts, and the Rust `load_conflicts` contract remain unchanged.

An isolated in-memory copy of the existing `data/timetable.db` replaced 103 repeated course memberships with six edges: CS core electives includes core requirements; CS math/science includes core electives; IT core electives and DevOps requirements each include core requirements; IT DevOps electives includes DevOps requirements; IT BAP electives includes core requirements. Both complete output relations matched exactly: 1,436 `undiscounted_conflict_pairs` rows and 864 `conflict_pairs` rows, including both orientations. The expanded queries took about 0.03 and 0.04 seconds respectively in this small fixture; this is feasibility evidence, not a scale benchmark. A redundant direct/transitive include preserved output, and recursive closure terminated with a deliberately introduced cycle; production validation must still reject cycles because their membership semantics are unintended.

Do not infer inclusion merely from labels or apparent nesting. Data Science's individual track cliques include core courses, whereas `all tracks combined` directly lists only track courses. Including those full cliques broadens that group's membership. Preserve it as an explicit list unless separately modeling reusable membership-only sets. Same-program membership reuse is sufficient for the demonstrated nesting; cross-program references and reusable sets independent of conflict rules are separate extensions.

The current `data/computing.py` uses 14 include links: the six demonstrated above plus three Data Science tracks and five Software Engineering tracks each including their own core requirements. This removes 272 repeated course memberships. Rebuilding `data/timetable.db` preserved all 482 expanded course memberships, eight section memberships, 1,436 undiscounted conflict rows, and 864 final conflict rows against a fresh pre-change source build. SQLite integrity and foreign-key checks passed. Existing audit findings were identical: missing spring offerings, credit/time mismatches, unmatched room preferences, and section-numbering issues. Archive inputs were not modified.


Tier policy and pair generation
-------------------------------

Recommend manual tier annotations initially. A selection ratio is not enough: seven of ten courses, two of three tracks, and twelve variable credits have different meanings, and the existing Data Science 5/7 distinction is deliberate. The LLM can suggest annotations by matching previously reviewed shapes, but only persisted effective annotations drive SQL. Do not introduce a continuously valued probability model.

`curriculum_tiers(tier_id PK, label, priority)` stores the small shared vocabulary; numeric priorities remain explicit. A requirement can have an exposure tier and a cross-child tier because participation against outside requirements and participation against sibling choices are different decisions. A separate override table can replace either annotation for a program version or a term without editing imported requirements. Resolve term override before version override before the stored annotation. Reject duplicate overrides at the same scope.

### Recursive scheduling projection

Each included node produces exposed course occurrences and internal unordered pair contributions. Every exposure has a priority; every pair has a priority and a source requirement ID. Ignored nodes and noncourse nodes produce neither. Here `max` weakens a conflict and `min` chooses the strongest contribution.

1. A course leaf exposes its course at its effective annotated tier and produces no internal pair. A pool expands to distinct course children. Repetitions remain metadata; they do not create self-conflicts.
2. An `all` node retains every child's internal pairs and joins exposed occurrences from different children. A `choose_children(k)` node does likewise when `k >= 2`; when `k = 1`, it retains internal pairs but creates no cross-child pairs. `choose_courses(k)` follows the same rule over course leaves.
3. A new cross-child pair has priority `max(left_exposure, right_exposure, node_cross_priority)`. An absent cross priority contributes no additional weakening. Exposing a course to the parent uses `max(child_exposure, node_exposure_priority)`. Parent exposure does not retroactively weaken a child's internal pairs.
4. Retain occurrence context through joins. Canonicalize the course pair and merge contributions with `min` only after generating compatible contributions. A pair omitted by one choice can still arise from another included requirement. Same-course pairs are discarded; explicit section-spread rules are separate.
5. Apply program pair overrides to the completed program contribution, then merge active program versions/programs by `min`. Apply term scope and offering expansion later; filtering courses must never change a published “choose seven” to “choose all remaining.”

For ordinary cumulative tiers, required leaves expose at A and the selected elective pool exposes and pairs internally at B. The root `all` creates A/A pairs at A and required/elective plus elective/elective pairs at B. There is no repeated list of required courses.

For Data Science, each track's `all` node creates internal pairs at 5 and exposes at 5. The `choose_children(2)` parent creates cross-track pairs at 7 while still exposing at 5. The root joins core exposure 1 to track exposure 5 at 5. Existing within-track 5 / cross-track 7 behavior therefore needs no hand-expanded cliques. For a one-track program, the choice parent creates no cross-track pairs. An ignored elective pool does not reconnect those tracks.

For the database alternative, `choose_children(1, CS 4307, IT 2300)` exposes both at the core tier but creates no pair between them. Each conflicts with other included core courses. This removes the routine need for a blanket cancellation. If another included requirement independently creates that pair, it survives; an explicit program-pair override can express a stronger advising exception.

Implement the recursive description using occurrence/ancestor relations, not recursive aggregation unsupported by SQLite. Recursive CTEs enumerate acyclic paths and propagate exposure floors. Ordinary joins identify the lowest common ancestor of two leaf occurrences and their distinct immediate branches; that ancestor's choice rule determines whether the pair is generated. Apply `MIN` grouping in subsequent nonrecursive views. Set membership can be evaluated by validated topological depth in derived cache tables when nested intersection/difference requires it; this deterministic database compilation is transactional, revision-keyed, and replaceable, never a second editable source of truth. Start with views for the common explicit-member and selector cases and measure before caching.

### Credit requirements and limits

Do not build a full degree-audit or feasible-plan enumerator as part of timetable input. For an included credit pool, use the reviewed tier to generate conservative potential pairs among pool courses and against other included requirements. Explicit one-option alternatives use choice nodes; credit arithmetic alone does not establish exclusivity, since excess credits, variable credits, repeated courses, and overlapping requirements complicate that inference.

Store subset limits and double-counting rules faithfully, but label their scheduling interpretation: `conservative_pairs` unless a reviewed structural projection or explicit pair override supplies a more precise rule. Do not claim that every produced pair can occur in an exact minimal graduation plan. Most wide-open pools are ignored entirely. If precise credit-allocation reasoning later proves necessary, extend the policy explicitly with fixtures before adding a new evaluator.


Persistent term input and editing transactions
----------------------------------------------

Use one durable database with term-scoped offerings, assignments, preferences, and overrides; keep versioned catalog/program inputs reusable across terms. Existing single-term data imports into one term. Source revisions, term input revisions, and solver contract versions are different identities.

| Tables | Main fields and invariants |
| ------ | -------------------------- |
| `terms`, `term_program_versions`, `term_course_scope` | Term dates/holidays, selected program versions, owned course IDs. Prefix ownership rules seed explicit course ownership; course overrides handle exceptions. Do not infer faculty affiliation from course ownership. |
| `offerings` | `offering_id PK, term_id, course_id, section_code, credit_value, status, scheduling_mode`; unique term/course/section identity. Distinguish active, tentative, cancelled and scheduled versus unscheduled. Explicit activation policy controls whether tentative rows contribute. |
| `activities`, `activity_offerings` | One schedulable activity owns room/time domains and faculty assignments; cross-listed offerings reference it. Normal offerings have one activity, unscheduled offerings none. Keep each registration identity and its credits without requiring aliases to share credits. |
| `activity_faculty`, room/time domain tables | Typed room and time tag targets, explicit concrete choices, and an explicit base-availability exception for selected times. Course defaults seed or supply visible defaults; activity overrides remain visible. No mixed string tag namespace. |
| `course_alternative_count_overrides` | Term/course/count/reason. Default count includes active online and unscheduled offerings, with cross-listed aliases of one activity counted once per course. Use the override for senior-project coordination, restricted alternatives, or other advising knowledge. |

Activities initially retain the solver's single-pattern/single-room shape. Engineering lecture/lab courses can be separate activities with curriculum conflicts. A section requiring multiple different weekly meeting components, linked lab-section registration, or date-limited sessions is not solved by renaming a time-slot field; diagnose unsupported inputs before activation. Extending these constraints would require separate solver work. A fixed externally scheduled activity is represented with a singleton domain, so it occupies faculty time and can conflict normally.

A broad catalog ingestion must not discard external intermediate prerequisites. Actual pair export includes only activities in the run's owned course scope. External program requirements can still motivate conflicts between two owned courses. Out-of-scope endpoints generate no scheduling constraint unless represented as explicit fixed activities. Shared faculty across excluded courses must therefore either have those fixed activities included or have equivalent availability blocks; dropping them silently would misrepresent availability.

The write service owns validation and transactions; SQLite owns keys, referential integrity, discriminant/range checks, and declarative derivation. A save checks an expected revision, validates the entire changed aggregate and affected cross-record constraints, writes all rows, increments the input revision, and commits atomically. Invalid saves leave the previous state intact. A catalog import stages a new revision; activation is a separate explicit import operation. Faculty saves are normal edits, not an additional approval workflow.

Scripts become clients of these operations. Initial seeding refuses to overwrite an existing populated target. Subsequent imports merge by stable identity and expected revision, show deletions and changed evidence, and preserve manual corrections/overrides. Never rerun destructive `data/build` against the durable database. Database migrations are incremental schema transactions. Export/import round trips preserve IDs and revision information; source snapshots and override history permit explaining changes without retaining executable Python as the authority.


Overrides and automatic processing order
----------------------------------------

Use typed override relations, not a universal rule language. All overrides carry a stable ID, reason, author, and revision; term-local overrides expire with their term. An explicit `suppress` action replaces the old NULL-as-cancellation encoding. A `set` action requires a priority. Priority-0 sources are immutable to softening operations: to remove one, edit or deactivate that particular hard source explicitly. Shared-faculty and room-exclusivity constraints cannot be cleared by curriculum overrides.

| Override | Application point |
| -------- | ----------------- |
| Catalog correction / scheduling requisite annotation | Before effective requisite graph construction; preserve imported evidence. |
| Requirement scheduling override | Before projection; may include/ignore a group or replace exposure/cross tiers. |
| Program course-pair override | After all contributions for that program, before merging programs. Set can introduce a pair; suppress removes only that program's soft contribution. Other programs still protect it. |
| Term course-pair override | After program merge, before discounts. Replaces or suppresses a soft pair for this term; set can introduce one. |
| Term activity-pair override | After expansion and automatic adjustments; exact final soft priority or suppression. Applies even to an automatically removed soft pair. It cannot weaken a hard source. |

Program pair overrides can have persistent version scope or term scope; a term row wins over its version row. At each stage, at most one active override exists for the same canonical pair and scope. Exact activity overrides win over course defaults for soft contributions. Keep suppressed pairs in the explanatory relation even though the solver relation omits them.

### Proposed SQL stages

| Stage/view family | Meaning |
| ----------------- | ------- |
| `effective_catalog_*`, `resolved_requirement_*` | Imported evidence plus corrections; resolve course sets against pinned catalog revisions and apply include/ignore and tier annotations. |
| `program_pair_contributions`, `program_course_pairs` | Recursive pair projection, provenance, local exceptions, then program-local minimum. |
| `merged_course_pairs` | Minimum across selected programs/versions, followed by term course-pair overrides. No prerequisite or section discount yet. |
| `effective_requisite_edges`, `requisite_paths`, `course_alternative_counts` | Full graph and alternatives, including unscheduled/out-of-scope courses as appropriate. Expose why a path or count applies. |
| `activity_pair_contributions`, `effective_activity_pairs` | Expand original course pairs to activities; adjust each contribution using its original course identities; merge canonical activity pairs, apply final activity overrides, and add hard shared-faculty pairs. |

For each expanded contribution: retain priority 0 immediately; otherwise apply requisite suppression, then the section discount. Preserve the current discount `p + 5 * ((count_a - 1) + (count_b - 1))`, with each count at least one, and omit results at least 10. Explicit same-course activity spreads bypass this discount. Keep hard rules and original course provenance through cross-listing expansion; never discount using an arbitrarily chosen alias course. Merge all surviving contributions by minimum priority and remove self-pairs.

For prerequisites, retain directed source edges with kind `strict_before` or `concurrent_allowed`, plus original text and origin. Recommend suppressing only paths containing at least one strict edge; pure concurrent chains retain conflict eligibility. A direct concurrent allowance prevents suppression for that ordered endpoint pair, preserving the existing direct exception. Contradictory strict/concurrent paths and strict cycles are diagnostics. Direction matters: test both directions for a symmetric conflict, without automatically making prerequisite edges symmetric.

The treatment of AND/OR prerequisite alternatives remains a policy choice. For compatibility, referenced alternatives may continue to participate in the historical heuristic, explicitly labeled as such. A future “only necessarily prior” rule needs structured requisite expressions and must not be slipped into this cleanup. The schema retains original expression evidence so that decision does not require scraping again.

`conflict_explanations` exposes pair, contributing programs/requirements, original priorities, effective annotations, applied override IDs, requisite suppression reason, section counts, discounted priority, and final disposition. Query provenance through normalized rows rather than a large concatenated explanation string. An absent solver pair can mean ignored requirement, unoffered course, out-of-scope activity, alternative branches, prerequisite removal, discount exhaustion, or explicit suppression; these must remain distinguishable in the admin view.


Faculty requests and manual coordination
----------------------------------------

| Tables | Main fields and invariants |
| ------ | -------------------------- |
| `faculty`, `term_faculty`, `faculty_availability` | Stable identity, affiliation, term presence/submission state, weekday intervals, and baseline availability. Preserve absence versus present-with-no-submission versus submitted-empty. |
| `faculty_hard_time_exclusions` | Term/faculty/concrete time-slot/reason. Hard availability is not a ranked request. Preserve current concrete-slot semantics. |
| `faculty_requests` | `request_id PK, term_id, faculty_id, kind, stated_rank, display_order, enabled, reason, revision`; kinds form a closed domain. Equal ranks represent a tier, not an identity. |
| Typed request payload/target tables | Days-off desired count and representative days; even spread; same-room clustering; room minimization; gap/cluster threshold and direction; avoid-room/time targets; time-pattern group. One validated payload per request kind. |
| `shared_day_off_groups`, participants | Group ID, two faculty members, representative days, each participant's request ID/rank. Preserve reciprocal intent, one partner per faculty, and one effective scored shared rule. |

Keep the existing 50-minute cluster gap as an explicit policy value, with shared validation and no scattered defaults. Preserve exact section credits, the usual 50 contact minutes per credit, and existing named course exceptions as database course-policy records used by both the app and audit. Do not use contact time as the definition of credit value.

One request can expand to many effects, and multiple source requests can contribute to one effective condition. Export `request_effects` and `request_effect_sources` separately so duplicated tags do not multiply penalties and shared faculty do not lose ownership. Initial effect normalization must reproduce the current balanced loader's owner/section/rank/kind grouping, including overlapping targets; preserving more source information must not silently change multiplicity or entropy weights. Same-rank identity and scoring multiplicity are distinct concepts.

For incremental migration, retain the existing stated-mode collapse as a named compatibility policy over the same source/effect relations. Do not delete it until its score behavior is compared and an explicit policy choice is made. A single loader can select the appropriate exported effect projection without reconstructing ownership from collapsed option priorities.

`anti_conflict_rules(rule_id, term_id, single_activity_id, priority, reason)` and typed course/offering target tables preserve manual coordination input. Views expand target courses to scheduled activities, canonicalize cross-listings, and deduplicate targets. Export rule IDs, not groups implicitly keyed only by the single section. Each rule yields one satisfaction condition: the single activity uses the exact same time-slot ID as at least one target. Retain the existing rules and scoring multiplicity initially. Diagnose self-targets, empty active groups, and groups with no common available slot; do not silently erase a rule. An absent single activity makes a term rule inactive with an explanation.

Explicit same-course spreads use `activity_pair_rules(rule_id, term_id, activity_a, activity_b, priority, reason)`. They are not fictitious degree requirements. The existing CS 4995 special policy can be a named manual course-set pair rule or explicit program pair overrides, with provenance identifying the advising decision. Room capacity remains available for audits; introducing enrollment/capacity optimization is outside this input migration.


Solver-facing contract and processing ownership
-----------------------------------------------

Export versioned `solver_v1_*` relations for one term, scope, and input revision. SQLite views are not parameterized endpoints: the export service selects a concrete term/scope in a read transaction and materializes a consistent read-only snapshot if needed. A solver never reads across partially completed UI edits. Keep the normalized input database authoritative; derived snapshots are replaceable artifacts.

| Relation | Required output columns |
| -------- | ----------------------- |
| `metadata`, `activities`, `offering_activity_map` | Contract version, input revision, term, scope, policy versions; activity ID/name/credits; original offering/course aliases for display and traceability. Activities exist independently of their domains. |
| `rooms`, `time_slots`, `time_slot_overlaps` | IDs/names; weekdays, start minutes, duration; canonical overlap pairs including self-overlap. Half-open time intervals preserve adjacent nonoverlap. |
| `activity_rooms`, `activity_times`, `activity_faculty` | Deduplicated hard domains and assigned faculty. No preference rank overloaded into an allowed-option column. An explicit room requirement distinguishes roomless activity from an invalid empty room domain. |
| `conflicts`, `anticonflicts`, `anticonflict_members` | One unordered distinct activity pair with final priority; rule ID/single/priority and group rows. Conflict uniqueness uses `a < b`; logical symmetry is part of the contract. |
| `faculty_requests`, typed effect relations, effect sources | Stable request/effect IDs, owner, stated rank, concrete targets, representative days, threshold parameters, and shared group IDs. No inference of request identity from rank or kind. |

A separate `solver_v1_diagnostics` relation reports invalid empty time domains, invalid required-room domains, unresolved included references, impossible manual groups, and unsupported shapes. Publication fails for errors; inactive requests and nonintersecting soft targets remain visible with an explanation. Empty option sets must never be interpreted as “drop this section.” The existing intentional roomless case remains supported.

| Owner | Responsibility |
| ----- | -------------- |
| Importer / LLM | Fetch evidence, propose normalized facts/requirements, resolve identifiers, and expose ambiguities. No authoritative conflict expansion. |
| Write service | Typed API validation, revision checks, role ownership, transactions, import reconciliation, and diagnostic enforcement. The CLI and UI use this same service layer. |
| Database | Effective inputs, set membership, curriculum pair projection, override order, prerequisite paths, alternative counts, offering scope, cross-listing normalization, availability/domain intersection, concrete request targets, and explainability. |
| Shared solver preparation | Read and validate the contract, map stable IDs to dense indexes, calculate the existing combinatorial room target, run existing faculty entropy rebucketing, merge shared requests, and build dependency adjacency. Preserve source/effect IDs. |
| SAT/search engines | Encode or evaluate prepared criteria, optimize placements, and report violations. No catalog interpretation, curriculum tier policy, or section-count discounts. |

The combinatorial room-target calculation and entropy balancing are deliberate exceptions to SQL transformation. Keep them in one shared preparation module, not independently in two solver adapters. They already depend on complete option domains and local schedule evaluation. Do not attempt a recursive SQL set-cover algorithm merely to make the loader shorter. A later solver in another language can consume an exported prepared snapshot containing resolved targets and effective priorities, avoiding reimplementation of this preparation policy.

Proposed Rust boundaries: `read_solver_snapshot(path: &Path) -> Result<SolverSnapshot>`, `prepare_input(snapshot: SolverSnapshot, policy: FacultyPriorityPolicy) -> Result<PreparedInput>`, and `load_input(config: &InputConfig) -> Result<PreparedInput>`. `InputConfig` includes explicit source/snapshot path, term/scope, and faculty priority policy. Use newtypes for IDs, stated ranks, and effective priorities; use enum payloads matching typed effect relations. Keep dense vector indexes internal. Remove bundle-then-unbundle loading, but preserve the SAT/search criteria semantics.

`solver_runs` records input revision/snapshot identity, contract and policy versions, selected scope, and solver settings. Run-specific request/effect priority assignments and shared-group mappings are output records, never edits to faculty ranks. Saved placements and violation rows reference stable activity/effect IDs and the run; reports resolve the exact revision used, not today's mutable inputs. Retain a complete immutable input snapshot per run, or a content-addressed snapshot shared by runs using identical input, so a revision number alone is never the only history.


API and file migration plan
---------------------------

The database contract supports API endpoints without making raw table writes the public API. Use aggregate operations with typed payloads and expected revisions. Suggested service signatures are `import_catalog(source, expected_revision) -> ImportDiff`, `save_program(version_id, ProgramInput, expected_revision) -> Revision`, `save_faculty_input(term_id, faculty_id, FacultyInput, expected_revision) -> Revision`, `save_override(OverrideInput, expected_revision) -> Revision`, and `export_solver_input(term_id, scope, expected_revision) -> SolverSnapshot`. Reads return the same editable shapes plus derived previews; conflict explanation is a separate read operation. Stable request IDs survive reordering and editing.

Suggested HTTP resources are `/terms/{term}/faculty/{faculty}/input`, `/program-versions/{version}`, `/terms/{term}/overrides`, `/terms/{term}/conflicts`, and `/terms/{term}/solver-input`. Use revision/ETag preconditions for writes. Catalog imports and seeding are explicit administrator operations. Do not expose arbitrary SQL or let the LLM write solver-facing output rows.

### Implementation sequence

1. Capture compatibility fixtures from the current schema and Spring 2027 input. Add `Makefile` and `data/Makefile` targets for validation and snapshot generation. Replace destructive build behavior with an explicit one-time legacy importer and a separate derived export. `mostlyclean` removes intermediates; `clean` additionally removes disposable deployment outputs; neither removes the authoritative database, configuration, backups, or saved schedules.
2. Introduce durable identities, terms/revisions, evidence/correction records, and offering/activity normalization in incremental schema migrations. Import existing Python once, preserving all manual exceptions and current emitted conflict pairs. Retain legacy clique input tables as a temporary source format until program migration is verified. Switch API writes to the database before retiring Python persistence.
3. Add course-set and requirement tables, SQL projection stages, ignored-group annotations, explicit override tables, and explanation relations. Model Computing first, including Data Science track 5/7 behavior and local alternatives. Compare outputs before adding newly interpreted catalog requirements. Then validate Mathematics and Design examples; stage Engineering only with supported activity shapes.
4. Add stable faculty requests and typed payloads; migrate the app's typed submission, installer, persistence, and assignment integration to service transactions. Import exactly the historical evidence needed by the existing app and preserve its absent/no-input/input states. Export concrete effects with provenance and introduce the common snapshot loader/preparation boundary. SAT/search adapters receive the same prepared semantics.
5. Retire legacy views and Python-as-authority only after differential checks pass. Record intentional behavior changes separately: hard conflict survival is required; corequisite-only path suppression needs the explicit policy decision below. Update this document from proposed schema to implemented invariants as each stage lands, and align `data/AGENTS.md`, `src/AGENTS.md`, and `app/ARCHITECTURE.md` with the new lifecycle.

Primary affected files are `data/schema.sql`, `data/queries.py`, `data/build`, `data/scrape`, the legacy seed files, `src/input.rs`, and `src/faculty_preferences.rs`; app boundaries are `models.py`, `preferences.py`, `semester.py`, `tools.py`, and `scripts/install_semester.py`. New schema migration files, a typed input service, and contract/export modules should have narrow ownership. `src/sat_criteria.rs`, `src/score.rs`, and reporting need identity/adapter changes only where the new prepared representation requires them. Keep solver optimization logic unchanged.


Validation and decisions
------------------------

Validate formatting/lint/type checks before tests, through Makefile targets; Python changes pass `uv run ty check`, and Rust changes run fmt, Clippy, then affected tests. Documentation-only planning does not require rebuilding the application. Browser verification applies when editing the faculty UI, using headed Chrome when a display is available and a temporary profile that is closed afterward.

### Behavioral fixtures

| Boundary | Required cases |
| -------- | -------------- |
| Requirement projection | Core/core and core/elective tiers; choose-one course; choose-one bundle with internal pairs; choose-two tracks with distinct internal/cross tiers; ignored pool produces no pairs; repeated occurrence cannot globally cancel another contribution. |
| Catalog and scope | Credit pools with variable credits; overlapping requirements; wildcard exclusion and catalog revision diff; unowned intermediate prerequisite; unoffered courses do not change choice cardinality; multiple active program versions merge by minimum. |
| Overrides and discounts | Local suppression cannot erase another program; term and activity precedence; suppression with no existing pair; exact final soft override can restore a discounted pair; priority 0 survives prerequisites/counts/soft overrides; online alternatives count; same-course spread bypasses discount. |
| Requisites and cross-listings | Strict chains, direct concurrent allowance, prereqs of coreqs and coreqs of prereqs, pure-coreq chain, cycles, and alternative-edge policy; alias collapse produces no self-conflict or double count; original course identities survive discounts. |
| Faculty and solvers | Request round-trip identity, equal-rank requests, overlapping tags, team-teaching ownership, shared-day-off joint prefixes and single scoring, stated/balanced compatibility, empty domains, roomless activity, exact-slot anticonflicts, and matching SAT/search scores on fixed complete schedules. |

Transaction fixtures additionally cover failed save rollback, stale revision rejection, import idempotence, source deletion without loss of overrides, stable section IDs through renumbering, and concurrent UI edits during snapshot export. Verify exports are repeatable from the same revision and saved schedules remain interpretable after later edits. Compare canonical rows and per-priority scores, not solver-selected schedules: multiple equal-quality schedules are expected. Measure SQL query plans and timings before introducing materialized caches.

### Decisions still requiring policy review before implementation

Recommend changing requisite suppression to require at least one strict edge, while initially retaining the existing referenced-alternative heuristic. This prevents pure-corequisite-chain suppression without introducing a full enrollment-eligibility engine. Confirm this semantic change before implementing it; hard conflict survival is already established.

Recommend explicit manual exposure/cross-tier annotations, conservative pairing for the few included credit pools, and explicit ignore annotations for wide-open pools. The initial catalog import should be reviewed as a diff against current advising policy, not treated as permission to add every published requirement to scheduling pressure. Requirement semantics and priority policy remain separate so either can evolve without rewriting the other.
