Timetabling SAT System
======================

Input Overview
--------------

Input is loaded from the SQLite timetable database by `input.rs`.  The solver
builds an immutable `Input` containing:

*   `rooms`: rooms usable by the selected departments.
*   `time_slots`: named meeting patterns with days, start time, and duration.
*   `time_slot_conflicts`: a symmetric overlap matrix loaded from
    `conflicting_time_slots`.
*   `faculty`: faculty names and the sections assigned to each faculty member.
*   `sections`: each section's allowed rooms, allowed time slots, faculty,
    hard conflicts, applicable criteria, and neighboring sections.
*   `criteria`: soft scoring rules loaded from conflict, anti-conflict, time
    pattern, room/time preference, and faculty preference tables.

Room and time availability can carry an optional priority.  `None` means the
room or time is acceptable without penalty.  `Some(priority)` means assignment
to that room or time creates a preference violation at that priority.

Constraint Types
----------------

The SAT solver uses the following intrinsic constraints and `SatCriterion`
types.

*   Section time assignment

    Every section must be assigned exactly one available time slot.  A section
    with no available time slots is an input error.

*   Section room assignment

    Every section with at least one available room must be assigned exactly one
    available room.  A section with no room options is allowed to have no room.

*   Room conflict

    Two sections may not use the same room at overlapping time slots.  Pairs
    already listed as hard conflicts are skipped here because the hard conflict
    makes their overlap impossible regardless of room.

*   Conflict

    Two sections should not be scheduled in overlapping time slots.  Hard
    conflicts are represented as priority `0` conflict criteria.  Soft
    conflicts use their database priority.  A section cannot conflict with
    itself.  Non-overlapping time-slot pairs do not matter.

*   AntiConflict

    A single section should be scheduled at the exact same time slot as at
    least one section from a group.  This is exact time-slot equality, not
    merely overlapping time.  The group must be non-empty, the single section
    must have time slots, and at least one group section must share an available
    time slot with the single section.

*   RoomPreference

    A section should avoid a specific available room.  One SAT criterion is
    created for each available room with a priority.  Referring to a room that
    the section cannot use is an input error.

*   TimeSlotPreference

    A section should avoid a specific available time slot.  One SAT criterion is
    created for each available time slot with a priority.  Referring to a time
    slot that the section cannot use is an input error.

*   FacultyDaysOff

    A faculty member should have exactly `desired_days_off` days with no classes
    among `days_to_check`.  Days outside `days_to_check` are ignored.  The
    faculty must have multiple sections, `days_to_check` must be non-empty, and
    `desired_days_off` cannot exceed the number of checked days.

*   FacultyEvenlySpread

    A faculty member's classes should be distributed evenly across checked days
    that have classes.  Empty checked days are ignored.  The violation occurs
    when the most-loaded non-empty checked day has more than one class more than
    the least-loaded non-empty checked day.  Requires at least two checked days
    and more than three faculty sections.

*   FacultyNoRoomSwitch

    A faculty member should not teach back-to-back classes in different rooms.
    Back-to-back means the two time slots share at least one checked day, do not
    overlap, and the gap between them is at most `max_gap_within_cluster`.  The
    rule is considered only for section pairs that have at least one common
    unpenalized room available.  Faculty with fewer than two sections are an
    input error for this criterion.

*   FacultyTooManyRooms

    A faculty member should use no more than `desired_max_rooms` distinct rooms.
    The desired maximum is computed during input loading as the smallest set of
    unpenalized rooms that can cover the faculty's schedulable sections; the
    criterion is omitted if that cannot improve over one room per section.
    `desired_max_rooms` must be positive and less than the faculty section
    count.  Trivial cases with one potential room or a desired maximum greater
    than or equal to the potential room count are skipped.

*   FacultyGapTooLong

    A gap between adjacent teaching clusters should not exceed `duration`.
    Clusters are computed per checked day after merging classes separated by at
    most `max_gap_within_cluster`.  A day with fewer than two clusters has no
    gap.  A zero duration is an input error.  If a higher-priority
    `FacultyGapTooLong` for the same faculty already catches the same gap, this
    criterion does not count it again.

*   FacultyGapTooShort

    A gap between adjacent teaching clusters should not be shorter than
    `duration`.  Cluster construction and empty-day behavior match
    `FacultyGapTooLong`.  A zero duration is an input error.  If a
    higher-priority `FacultyGapTooShort` for the same faculty already catches
    the same gap, this criterion does not count it again.

*   FacultyClusterTooLong

    A teaching cluster should not exceed `duration`.  Clusters are per checked
    day and merge adjacent classes separated by at most
    `max_gap_within_cluster`.  A zero duration is an input error.  If a
    higher-priority `FacultyClusterTooLong` for the same faculty already catches
    the same cluster, this criterion does not count it again.

*   FacultyClusterTooShort

    Teaching clusters should not be shorter than `duration`, except that the
    first too-short cluster on each day is free.  Cluster construction matches
    `FacultyClusterTooLong`.  A zero duration is an input error.  If a
    higher-priority `FacultyClusterTooShort` for the same faculty already
    catches the same cluster, this criterion does not count it again.

*   TimePatternMatch

    All sections in the group should use the same time pattern.  A pattern is
    `(number of meeting days, duration)`, not the specific days or start time.
    Groups with fewer than two sections are ignored.  Groups with fewer than two
    possible patterns are treated as invalid/trivial input by the encoder.

SAT Solver Process
------------------

`sat_solver.rs` converts `Input` to `SatCriteria`, grouped by priority.  The
solver then optimizes priorities lexicographically from `0` through the maximum
priority present:

1.  Create a fresh CNF encoding.
2.  Create variables for every allowed `(section, room)` and
    `(section, time_slot)` assignment.
3.  Add intrinsic section assignment and room conflict clauses.
4.  Encode all criteria up to the current priority.
5.  For each encoded criterion, create one or more hallpass variables that mean
    "this violation is allowed."
6.  Constrain hallpasses at each already-processed priority to the best known
    count, and try increasing the current priority's allowed count from `0`
    upward until Kissat returns SAT.
7.  Decode true section-room and section-time variables into a `Schedule`, save
    the best schedule after each priority, and continue to the next priority.

Priority `0` is hard: if no solution exists with zero priority-`0` hallpasses,
SAT generation fails.

SAT Encoding
------------

Let `T[s,t]` mean section `s` is assigned time slot `t`, and `R[s,r]` mean
section `s` is assigned room `r`.  A hallpass variable `H` is true when the
associated criterion violation is counted.

*   Section time assignment

    For each section, add one clause containing all `T[s,*]` variables and
    pairwise `!T[s,a] OR !T[s,b]` clauses for every distinct available time
    pair.

*   Section room assignment

    For each section with rooms, add one clause containing all `R[s,*]`
    variables and pairwise `!R[s,a] OR !R[s,b]` clauses for every distinct
    available room pair.

*   Room conflict

    For each room, section pair, and overlapping time-slot pair, add
    `!T[a,ta] OR !R[a,r] OR !T[b,tb] OR !R[b,r]`.

*   Conflict

    Create `H`.  For each overlapping available time-slot pair, add
    `!T[a,ta] OR !T[b,tb] OR H`.

*   AntiConflict

    Create `H`.  For each available time slot `t` of the single section, gather
    group sections that can also use exactly `t`.  If none exist, add
    `!T[single,t] OR H`; otherwise add
    `!T[single,t] OR T[g1,t] OR ... OR T[gn,t] OR H`.

*   RoomPreference

    Create `H` and add `!R[section,room] OR H`.

*   TimeSlotPreference

    Create `H` and add `!T[section,time_slot] OR H`.

*   FacultyDaysOff

    Create a day variable `D[d]` for each checked day.  Link it bidirectionally
    with all faculty section-time variables meeting on that day:
    `D[d] -> OR(T[*])` and `T[*] -> D[d]`.  If no assignment can meet on the
    day, add `!D[d]`.  Enumerate every truth pattern of checked days whose
    number of false days is not `desired_days_off`; for each bad pattern, add a
    clause negating that exact pattern plus `H`.

*   FacultyEvenlySpread

    Create section-day variables `SD[s,d]` linked bidirectionally to the
    section's time slots that include day `d`.  Enumerate unique conflict-free
    faculty time-slot combinations, project them to `SD` truth patterns, and for
    each pattern where non-empty day counts differ by more than one, add a
    clause negating that pattern plus `H`.

*   FacultyNoRoomSwitch

    Create `H`.  For every relevant ordered section pair, checked day,
    back-to-back time-slot pair, and different room assignment, add
    `!R[s1,r1] OR !T[s1,t1] OR !R[s2,r2] OR !T[s2,t2] OR H`.

*   FacultyTooManyRooms

    Create faculty-room variables `FR[r]`.  Add `R[s,r] -> FR[r]` for each
    faculty section-room variable and `FR[r] -> OR(R[* ,r])` for each potential
    room.  Apply a totalizer at-most-`desired_max_rooms` constraint to all
    `FR` variables with hallpass `H`, so exceeding the limit implies `H`.

*   FacultyGapTooLong

    Create faculty-time-slot variables `FT[t]` linked bidirectionally to all
    faculty section-time variables using `t` on checked days.  Enumerate every
    valid non-overlapping per-day truth pattern of `FT` variables, compute
    clusters and gaps, count gaps longer than `duration` that are not already
    caught by higher-priority constraints, and for each counted violation add a
    clause negating the exact `FT` pattern plus a numbered hallpass for that
    day/violation.

*   FacultyGapTooShort

    Uses the same `FT` pattern enumeration as `FacultyGapTooLong`, but counts
    gaps shorter than `duration` and does not waive the first violation.

*   FacultyClusterTooLong

    Uses the same `FT` pattern enumeration, counts clusters longer than
    `duration`, and adds one pattern-forbidding clause with a day/violation
    hallpass for each counted cluster.

*   FacultyClusterTooShort

    Uses the same `FT` pattern enumeration, counts clusters shorter than
    `duration`, subtracts one free too-short cluster per day, and adds one
    pattern-forbidding clause with a day/violation hallpass for each remaining
    counted cluster.

*   TimePatternMatch

    Create one pattern variable `P[p]` for each distinct `(day_count, duration)`
    available to the group.  Link each section-time variable to its pattern with
    `T[s,t] -> P[p]`, and link each pattern back with `P[p] -> OR(T[*])`.
    Add `P[p1] OR ... OR P[pn] OR H` and pairwise
    `!P[pi] OR !P[pj] OR H` clauses.  Without `H`, exactly one pattern may be
    used by the group.

*   Hallpass counting

    After all criteria at a priority are encoded, their hallpass variables are
    constrained.  If zero violations are allowed, each hallpass gets `!H`.  If
    one violation is allowed and there are at most 30 hallpasses, pairwise
    at-most-one clauses are used.  Otherwise a totalizer at-most-`k` encoding is
    used.  If `k` is at least the number of hallpasses, no counting clauses are
    needed.
