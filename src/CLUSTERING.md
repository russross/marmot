Totalizer input ordering experiments
===================================

Status and next action
----------------------

November 10 continuation selected whole first-section blocks with accumulated shared-section incidence, applied to every priority. See the continuation log below for current-dataset measurements, rejected alternatives, and repeat variability. The earlier measurements and candidate list remain research history. No fixed-probe harness or per-priority CLI option was added.

The CLI now selects `cluster` when no order is supplied. Use `--totalizer-order default` to select the creation order called `current` in the historical benchmarks below.

Keep experimental integration shallow and easy to remove. Git remains read-only unless explicitly authorized otherwise. Preserve the existing scoring semantics, hard constraints, lexicographic priority optimization, and saved optimum-prefix semantics.


Problem and evidence
--------------------

The solver permits violations using Boolean hallpasses and constrains their count separately at each priority. Optimization retains the established budgets for earlier priorities while searching the next priority. A curriculum conflict normally emits one hallpass; some faculty criteria emit several.

Construction time is trivial compared with difficult solves on the user's data. Failed probes are usually slower than successful probes. A discovered optimum of 98 at priority 6 is difficult both to establish and to retain while optimizing later priorities. Every later probe includes the priority-6 bound of at most 98. If 98 is the proven optimum, all feasible later schedules necessarily incur exactly 98 violations there, even though the CNF explicitly encodes only the upper bound.

Observed baseline command, run from `/home/russ/marmot/src`:

    time cargo run -r -- sat --search binary --stop-after-priority 6 --schedule-output none --sat-timings --totalizer-order current

The run loaded input in 2606 ms and balanced 344 effective faculty preferences from 162 stated preferences into 40 effective priorities in 88 ms. SAT conversion reported 4282 constraints across 50 priority levels. Solver-reported total time was 239.193617552 seconds; shell wall time was 4m1.969s. The completed score prefix was `[0,14,0,18,4,44,98]`.

| Priority | Criteria | Optimal allowed hallpasses | Observed behavior                  |
| -------- | -------- | -------------------------- | ---------------------------------- |
| 0        | 619      | 0                          | SAT in 0.13 s.                     |
| 1        | 727      | 14                         | Probes took 0.11–0.16 s.           |
| 2        | 65       | 0                          | All probes SAT, about 0.14 s each. |
| 3        | 253      | 18                         | Probes took 0.14–0.17 s.           |
| 4        | 76       | 4                          | Probes took 0.14–0.21 s.           |
| 5        | 640      | 44                         | First substantial UNSAT costs.     |
| 6        | 1252     | 98                         | Dominates total solving time.      |

Priority-5 baseline probes:

| Allowed | Result | Seconds |
| ------- | ------ | ------- |
| 320     | SAT    | 0.22    |
| 160     | SAT    | 0.16    |
| 80      | SAT    | 0.17    |
| 40      | UNSAT  | 2.89    |
| 60      | SAT    | 0.21    |
| 50      | SAT    | 0.17    |
| 45      | SAT    | 0.22    |
| 43      | UNSAT  | 3.75    |
| 44      | SAT    | 0.45    |

Priority-6 baseline probes; variable and clause counts describe the complete probe, including retained earlier budgets:

| Allowed | Variables | Clauses | Result | Seconds |
| ------- | --------- | ------- | ------ | ------- |
| 626     | 30636     | 1853889 | SAT    | 0.35    |
| 313     | 29699     | 1608499 | SAT    | 0.32    |
| 156     | 28604     | 1447892 | SAT    | 1.75    |
| 78      | 27438     | 1358313 | UNSAT  | 49.42   |
| 117     | 28023     | 1403865 | SAT    | 4.27    |
| 98      | 27738     | 1381863 | SAT    | 6.16    |
| 88      | 27588     | 1370138 | UNSAT  | 15.68   |
| 93      | 27663     | 1376013 | UNSAT  | 6.88    |
| 96      | 27708     | 1379526 | UNSAT  | 99.40   |
| 97      | 27723     | 1380695 | UNSAT  | 31.86   |
| 98      | 27738     | 1381863 | SAT    | 6.23    |

Priority 6 accounts for 222.32 seconds, about 93% of the solver-reported total. Its UNSAT calls account for 203.24 seconds, about 85% of that total. Difficulty is not monotonic in the bound: proving 96 impossible took much longer than proving 97 impossible. Similar formula sizes have very different solve times. The working hypothesis is that auxiliary-variable groupings affect the solver's ability to learn useful proofs; formula size alone does not explain these results.

The user also tried the same overall test with the other orderings. `shuffle` was killed after approximately twenty minutes while working on priority 5. `cluster` was killed after approximately twenty minutes on priority 6, with several probes still needed before finding the optimum. These are censored observations, not completed runtimes. The shuffle seed and exact interrupted probes were not recorded in the supplied results. Changing the global policy also changes earlier retained counters, so the priority-6 slowdown cannot yet be attributed solely to its own counter ordering.


Current implementation
----------------------

Read `AGENTS.md` for SAT criterion semantics and `../NOTES.md`, section “SAT preparation and ordering experiments,” for the operational specification.

| File              | Relevant responsibility                                                                           |
| ----------------- | ------------------------------------------------------------------------------------------------- |
| `main.rs`         | SAT CLI parsing, defaults, and final schedule/problem output selection.                           |
| `sat_solver.rs`   | Linear/binary probe loops, partial-run stopping, saves, and Kissat timing records.                |
| `sat_prep.rs`     | `PreparedCnf`, prepared priority boundaries, cached hallpass ordering, and clustering.            |
| `cnf.rs`          | Hallpass allocation, ordered/default totalizer entry points, tree construction, and Kissat calls. |
| `sat_criteria.rs` | Conversion from loaded criteria to priority-indexed SAT criteria.                                 |

`PreparedCnf::new` creates assignment variables and intrinsic clauses, then encodes all main criteria in ascending priority order once. Each prepared level records `clause_end`, `last_var`, and ordered `hallpasses`. `Encoding::hallpass_order` records creation order so emitted hallpasses can be associated with each criterion. Shared faculty-day links occur in the first priority prefix needing them.

Each probe clones the prepared template, truncates its clauses and variable range to the selected prefix, filters later-priority metadata, and adds budgets for all included priorities. Counter variables start after that prefix's last main variable. Preparation includes main criteria beyond a requested stopping level. Each probe uses a fresh Kissat solver; learned clauses are not retained between probes.

Zero budgets use negative unit clauses. A budget of one with at most 30 hallpasses uses pairwise clauses. Other nontrivial budgets use the bounded totalizer; budgets at least as large as the input count need no counting clauses. The tree divides each input slice at `len / 2` and retains only the first `k + 1` outputs per node. Merges use forward lower-bound implications; forbidding the root's `k + 1` output enforces the upper bound. No reverse counting implications are required for this use.

`Encoding::totalizer_at_most_k` sorts by variable ID. `ordered_totalizer_at_most_k` preserves the supplied order. Priority hallpass counters use the latter; faculty room-count counters retain the original sorted behavior.

Current options:

| Option                                        | Default    | Behavior                                                                                      |
| --------------------------------------------- | ---------- | --------------------------------------------------------------------------------------------- |
| `--totalizer-order current\|shuffle\|cluster` | `current`  | Selects the ordering for every priority counter.                                              |
| `--shuffle-seed N`                            | `0`        | Private per-priority shuffle RNG uses seed plus priority, with wrapping addition.             |
| `--stop-after-priority N`                     | No cutoff  | Optimizes priorities through N, inclusive; accepts 0–49.                                      |
| `--schedule-output all\|problems\|none`       | `problems` | Controls the final grid and problem list; progress/save messages remain.                      |
| `--sat-timings`                               | Off        | Prints one row per Kissat call at solver completion; accepts a bare flag or explicit boolean. |

Timings are always collected around `Encoding::solve`: they include Kissat setup, clause loading, solving, and model extraction, but exclude CNF preparation and schedule decoding/saving. Rows show the probed priority, criterion count at that priority, allowed hallpasses there, total probe variables/clauses, selected ordering, seconds to two decimals, and SAT/UNSAT/ERROR. The ordering label is printed even when the bound needs no totalizer.

`--search linear|binary` selects the probe strategy and defaults to binary. `--min-hallpass LEVEL:COUNT,...` sets starting probes; it does not lock earlier priorities to specified values or provide isolated fixed-bound tests. Binary search can repeat the final feasible bound, as the baseline shows. Probe-strategy changes are separate from the ordering experiment.

All runs save placements to the selected database. Partial runs retain a complete actual schedule score but certify only the optimized prefix. Their final save comment indicates the last resolved priority rather than claiming full completion.


What the original orderings did
--------------------------------------

`current` orders hallpasses by creation, equivalent to ascending variable ID within a priority. `input.rs::load_conflicts` reads curriculum conflict pairs with `ORDER BY section_a, section_b`. `SatCriteria::from_input` preserves their relative order while grouping by priority. Thus the baseline already groups conflicts around the same first section, with additional locality from course/section naming. It is plausible that this produces useful local counters; this explanation has not been measured directly.

`shuffle` permutes individual hallpasses once per priority during preparation. A fixed seed reproduces the permutation for a fixed creation sequence, but existing unordered iteration elsewhere means it does not guarantee identical full CNFs across processes.

`cluster` maps each criterion to its distinct participating sections. Faculty criteria include all their faculty's sections; shared requests include both owners. Two criteria receive an edge weighted by the number of shared sections. A greedy walk starts at the highest weighted-degree criterion, follows the strongest edge from the last selected criterion, and restarts at the strongest remaining vertex when needed. Ties prefer higher degree, then original order. Multiple hallpasses from one criterion remain adjacent.

Two limitations distinguish this walk from a suitable hierarchical grouping: it scores only the last selected criterion rather than the whole selected group, and it ignores totalizer split boundaries. For example, after `(A,B)` and `(A,C)`, it can follow `(C,D)` and abandon the remaining A-related constraints. Even adjacent related criteria may land on opposite sides of a large subtree boundary.


Candidate experiments, in recommended order
------------------------------------------

### 1. Subtree-local accumulated proximity

Preserve current order outside small existing tree ranges. Reorder only inside actual subtrees containing approximately 16, 32, or 64 hallpasses. Use the real recursive `len / 2` ranges rather than assuming power-of-two boundaries.

Within a range, choose each next criterion using the sum of its shared-section edge weights to the selected group, with original order as the tie-breaker. Keep a criterion's multiple hallpasses together where applicable. This limits disruption and tests a specific correction to the failed walk. Start with curriculum conflicts at priority 6, where the criterion/hallpass relationship is simple. This is a proposed adaptation, not a published guarantee.

### 2. Preserve section or course blocks

Treat existing runs with the same first section as indivisible blocks. Reorder blocks using accumulated shared-section affinity, retaining current order within each block. A separate variant groups by the first section's course while preserving section order within each course. The hypothesis is that section-centered or course-centered summaries explain part of the baseline advantage.

Course identity should come from an established project representation or a documented section-name convention; do not silently invent a parser for names. Keep the two variants separate so their effects can be distinguished.

### 3. Refine the order against the actual totalizer tree

Start from current order. Consider exchanges across sibling boundaries that reduce section dependencies crossing those boundaries, while keeping the same subtree sizes. One candidate cost is `sum(n_v * |S(left_v) intersect S(right_v)|)` over internal nodes, where `n_v` is the number of hallpasses below the node and `S` is the union of participating sections below a child. Lower cost favors keeping shared dependencies out of large splits.

The size weighting makes a root-level separation more costly than a leaf-level separation. An unweighted sum of shared-section intersections is unsuitable: for a section occurring in a fixed number of leaves, its total number of branching intersections is invariant under reordering. The weighted metric is still only a hypothesis; evaluate whether it predicts runtime before using it as an optimization objective.

### 4. Clause-based proximity through assignment variables

Use the prepared main clauses through the relevant priority to derive connections through all variables, then restrict the resulting order to that priority's hallpasses. Direct hallpass co-occurrence alone misses most curriculum relationships because hallpasses interact through section-time variables.

Exclude hallpass counting clauses from the analysis so the counter being investigated does not create artificial connections. Include intrinsic/hard clauses and relevant earlier-priority main clauses. Known section-assignment at-most-one groups can supply structural information directly, without rediscovering them from CNF. Short-clause weighting and accumulated proximity have direct research support, but an exact implementation can have substantial initialization cost on million-clause inputs. Analysis remains a once-per-initialization operation, not per probe.


Research to return to
---------------------

[The Impact of Literal Sorting on Cardinality Constraint Encodings, AAAI 2025](https://www.cs.cmu.edu/~jereeves/research/AAAI25-paper.pdf), by Reeves, Boucinha de Sá, Hsu, Martins, and Heule, is the most directly relevant paper. Its premise is that changing leaf groups changes the abstractions available for clause learning even when encoding size is unchanged. Natural order beat occurrence sorting and even the best of five random permutations on their benchmark set. Proximity approaches improved on natural order. Their tests used CaDiCaL and MaxSAT-derived instances, so transfer to Kissat and timetable constraints remains experimental.

The paper's proximity algorithm accumulates variable scores through shared clauses, favoring short clauses; an extension gives strong connections to variables in detected at-most-one groups. It is not a last-vertex greedy walk. The paper also studies community ordering and clause coverage, offering diagnostics for whether groups summarize comparable amounts of the formula. Expensive global analysis can outweigh benefits on easy formulas.

Supporting resources:

*   [Authors' implementation, LiteralSorting](https://github.com/jreeves3/LiteralSorting).
*   [Reeves's dissertation, Cardinality Constraints in Boolean Satisfiability Solving, chapter 4](https://www.cs.cmu.edu/~jereeves/research/jereeves_phd_csd_2025.pdf).
*   [Author's introductory explanation](https://www.cs.cmu.edu/~jereeves/research/cardinality-constraints/).

Cardinality networks, sequential counters, and incremental/core-guided optimization remain possible separate experiments. They are not the immediate next step. Construction overhead is not the reported bottleneck, and replacing the counting encoding would confound this investigation into ordering.


Resume checklist
----------------

1. Preserve the input database snapshot and baseline configuration. Record its identity/hash, solver build, balancing policy, seed, and exact command. Repeat the current baseline to assess cross-process variation. Use a scratch database copy when benchmark placements should not enter working history.
2. Add a narrow way to override ordering only at priority 6, retaining `current` at priorities 0–5. Keep selection and ordering analysis in `sat_prep.rs`; avoid changes to criterion semantics or solver strategy.
3. Add a small fixed-probe test path or harness. Use earlier budgets `[0,14,0,18,4,44]` and priority-6 probes at 78, 96, 97, and 98. These are dataset-specific measurements, not permanent assumptions about optima. The existing CLI cannot directly select this probe set.
4. Implement only subtree-local accumulated proximity first. Compare its subtree-size variants against current order on identical probes. Record SAT/UNSAT, runtime, variable/clause counts, preparation cost, and timeouts. If exact formula isolation is needed, reuse one prepared main CNF so unrelated unordered iteration cannot change between treatments.
5. Retain an ordering only if fixed-probe improvements survive a complete binary-search run through priority 6 and then a run through priority 8. Later probes retain earlier budgets, so improving one proof can still make subsequent optimization worse.

Failed runs and timeouts are results; record the active priority/bound and cutoff rather than assigning a completed runtime. Compare the same probes before comparing total runs with different search paths. No claim that an alternative improves this timetable is supported yet.


Validation state
----------------

The implemented preparation/ordering infrastructure passed 34 Rust tests before this research note. Tests cover small exhaustive placement/budget equivalence across all three methods, exclusion of future-priority clauses, reuse of shared faculty-day links, and counts around a limit of 98. CLI checks on temporary database copies exercised all three orderings, both search strategies, output modes, timing rows, and saved partial optimum prefixes.

The last Clippy run reported an existing `items_after_test_module` warning in `input.rs`; it was not suppressed or changed. For future Rust changes, run formatting, then Clippy, then affected tests. Documentation-only edits require no solver rerun. Git has not been used to checkout, reset, or commit these experiments.


November 10 continuation: subtree-local proximity
------------------------------------------------

The active dataset snapshot (copied from `../data/timetable.db` to `/tmp/marmot-order.db`) has SHA-256 `83f5ebe009b46fb2a8362362625d9510391d51c6253c0456c1593eae9fe31149`. Git was clean at experiment start. Release build uses the existing Cargo.lock and default balanced faculty preferences, seed 0. This input produces 3488 criteria, so the older 4282-criterion measurements above are historical rather than directly comparable.

Command: `marmot sat --db-path /tmp/marmot-order.db --min-hallpass 1:12,3:14,6:94 --search linear --stop-after-priority 6 --sat-timings --schedule-output none --totalizer-order current` (replace current with cluster for candidates). Schedule writes go only to the scratch copy. Initial screening uses a 300-second whole-process timeout and sequential runs.

Candidate family: retain creation order at all priorities except 6. Recursively split the criterion list at len/2 until each subtree has at most 16, 32, or 64 criteria. Within each terminal range start with its first criterion, then maximize cumulative shared-section count against all already selected criteria; break ties by original position. Hallpasses from each criterion remain together. The criterion-count split matches hallpass splits for single-hallpass curriculum rules; it is only approximate when a criterion emits multiple hallpasses. No scoring or counter encoding changes.

Size-16 formatting, Clippy, and the four sat_prep tests pass. Measurements pending below.

Initial clean measurements (seconds):

| Candidate | 6:94 UNSAT | 6:95 SAT | Solver total | Outcome |
| --------- | ---------- | -------- | ------------ | ------- |
| current   | 225.09     | 2.13     | 228.64       | baseline |
| local 16  | 249.74     | 7.12     | 258.34       | slower |

Both completed with prefix `[0,12,0,14,0,0,95]`. Priority-6 probes have 21953/21968 variables and 1335973/1337126 clauses at bounds 94/95 respectively. A runner race briefly overlapped preliminary size-16/32 processes; those were terminated and discarded before the clean sequential measurements. The scratch database contains their placement history, which is not solver input.

Local-32 timed out at 300 seconds still probing 6:94 (no result). Local-64 completed in 270.46 solver seconds: 234.66 UNSAT and 34.41 SAT. Both local-16 and local-64 retain the baseline formula sizes and score prefix. All three local scales therefore fail the initial screen; no finer size tuning is indicated.

Next independent candidates preserve priorities 0–5: (1) preserve consecutive runs with the same first participating section, then reorder those blocks by cumulative shared-section incidence, starting with the first block; (2) stable sort criteria by their last participating section, testing the other endpoint of curriculum pairs. Neither parses course names.

Whole first-section blocks at priority 6 completed in 201.72 seconds, with 160.44 UNSAT and 39.74 SAT: about 12% faster overall than the initial baseline, despite a much slower feasible call. Formula sizes and prefix agree. This is provisional until repeats. Global application to all priorities and repeat baseline/block measurements are queued; the successful policy is not yet selected.

Second-endpoint grouping timed out at 300 seconds on 6:94. Global first-section block ordering completed in 172.88 solver seconds (164.37 UNSAT, 7.09 SAT), about 24% faster than the first baseline. Earlier counters are reordered too; all score prefixes and formula sizes still agree. Baseline and priority-6-only block repeats are running next. A final independent weighting test uses distinct block-section overlap instead of repeated pair incidences to avoid favoring large blocks.

The second baseline process timed out at 300 seconds on 6:94, versus 225.09 seconds for that proof in the first process. This is substantial cross-process variation; the initial 12%/24% improvements are screening observations, not statistically established speedups or isolated-formula comparisons. Existing unordered construction is a known confound. Repeats of the block policies are necessary before selection.

The priority-6-only block repeat completed in 282.15 seconds (261.59 UNSAT, 19.15 SAT), confirming substantial variability. Distinct-section block weighting completed in 247.79 seconds (170.76 UNSAT, 75.55 SAT), losing its proof-time gain on the feasible call. Retain repeated-incidence weighting rather than tune weights or subtree sizes further in this round.

The production implementation applies whole-block accumulated incidence at every priority, without a dataset-specific priority check. Each block is a consecutive run of criteria whose sorted participating-section lists share the same first section. Starting at the first block preserves the natural anchor; subsequent choices maximize total overlap with all selected blocks, with original block position breaking ties. Internal criterion order and all emitted hallpasses remain intact. Empty domains form ordinary runs and all hallpasses remain present. This is a shallow replacement of the previous walk in `sat_prep.rs`; no CLI, solver-search, scoring, counter-tree, or dependency changes were made. `../NOTES.md` now describes that workflow.

Final source validation: cargo fmt, cargo clippy, and all five sat_prep tests pass, including exhaustive placement/budget equivalence and a new whole-run preservation case. The old Clippy warning recorded above did not occur in these runs. Working input database remains untouched; all benchmark saves went to the scratch database. Git remained read-only.

Final global-block repeat completed in 235.94 seconds (219.98 UNSAT, 14.54 SAT). Both global-block runs completed below the cutoff, versus one completed baseline and one censored baseline. The best completed run is 24% faster than the first baseline, but the second global-block run is 3% slower than that baseline. This is evidence for retaining the block heuristic as the best tested replacement, not a robust 24% speedup claim.

| Policy                                            | Solver totals, seconds        | Screen result |
| ------------------------------------------------- | ----------------------------- | ------------- |
| current                                           | 228.64; timeout 300 at 6:94   | baseline varies substantially |
| subtree-local accumulated, 16                     | 258.34                        | rejected |
| subtree-local accumulated, 32                     | timeout 300 at 6:94           | rejected |
| subtree-local accumulated, 64                     | 270.46                        | rejected |
| first-section blocks, priority 6 only             | 201.72; 282.15                | weaker than global blocks |
| last-section grouping, priority 6 only            | timeout 300 at 6:94           | rejected |
| first-section blocks, global, repeated incidence  | 172.88; 235.94                | retained |
| first-section blocks, global, distinct overlap    | 247.79                        | rejected |

Stop this round without finer tuning: all three local scales regress, changing the anchoring endpoint fails, and removing repeated-incidence weights loses on the SAT call. Whole-block preservation is the useful high-level direction from this screen. Tree-cut optimization and clause-based proximity remain unimplemented research candidates; neither has new evidence of likely gains here. With the observed cross-process spread, their next useful prerequisite is an isolated prepared-CNF comparison rather than further small heuristic adjustments. Binary-search and through-priority-8 validation from the older research checklist were not run; the selection targets the requested current-data linear probes through priority 6. No stronger claim about later priorities is supported.
