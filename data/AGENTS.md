# Marmot Input Format

This directory is scratch space for a college-wide Spring 2027 dry run using
the posted schedule in `postedschedule/`. The computing input is an inherited
starting point, not the scope of the project. It is archived separately and
need not be preserved here. The goal is to compare generated schedules with
the posted schedule and refine inputs with department chairs before use in a
future semester.

Collect all courses scheduled by college departments, and model bachelor's
and master's programs from their worksheets, excluding integrated studies
emphases. Preserve source details and inferences in comments. Infer generous
but defensible faculty availability and conservative room groups; record
uncertainties for chairs. Retain existing computing preferences, but do not
infer faculty preferences for other departments. The 2024 college and 2026
math dry runs under `../archive/` are reference material, not current truth.

Posted sections with the `SI` attribute include supplemental instruction.
Identify the supplemental meeting by comparing weekly meeting minutes with
the reported credits, using 50 minutes per credit as the normal baseline.
The mismatching meeting is generally shorter. Represent it as a separate
section with `si` appended to the section
number, assign it no faculty, and require a hard conflict with its primary
section. The instructor of record teaches the primary meeting, not the
supplemental meeting. For example, BIOL 1010-02 has TR 09:00–10:15 as the
primary meeting and W 09:00–09:50 as supplemental instruction. Inspect
ambiguous meeting patterns rather than guessing. SI records in the posted
files also occur in mathematics and chemistry.
CHEM 3300-01 (CRN 21557) is believed to have an unmarked SI meeting: use
the MWF lecture as primary and the Thursday meeting as supplemental.
BIOL 2420-03 and -07 (CRNs 20543 and 22611) also have unmarked SI: use the
MWF lecture as primary and the Tuesday or Thursday meeting as supplemental.
Exclude posted sections with a capacity of zero from the dry-run inputs.
The two BTEC 4050-01A meetings are a custom lab arrangement; represent them
as distinct faculty-assigned sections fixed at their posted times.
Section suffix A means the first half of the semester and B the second half.
The current solver checks weekly time overlaps without accounting for these
date ranges, so its conflicts can be conservative for different blocks.
Create the two sections first, then call
`add_supplemental_instruction(primary, supplemental)`. The relationship
creates the hard conflict and excludes the supplemental meeting from counts
of student choices. The supplemental section retains the offering's catalog
credit hours; its actual meeting duration is represented by its time slot.

The audit script's existing warnings are not a priority at this stage of the
dry run. Focus validation on the behavior of the new inputs and imports.

This repo is larger than the timetabling input pipeline. For input work, focus on:

- `data/build`: creates `data/timetable.db` from scratch.
- `data/schema.sql`: the SQLite schema and scheduling views the Rust solver reads.
- `data/queries.py`: the Python API used to populate the schema.
- `data/courses.py`: the generated course catalog and rotations.
- `data/computing.py`: department-wide setup for computing.
- `data/computingfaculty.py`: faculty section assignments and faculty preferences.
- `data/mathematics.py` and `data/engineering.py`: degree conflict groups.
- `data/biology.py` and `data/chemistry.py`: degree conflict groups.
- `data/design.py` and `data/earth_sciences.py`: degree conflict groups.
- `data/mathematics_faculty.py`: mathematics assignments inferred from the current posted CSV.
- `data/physics_faculty.py`, `data/chemistry_faculty.py`, and `data/biotechnology_faculty.py`: physics assignments and the currently modeled chemistry and biotechnology cases.
- `data/biology_faculty.py`, `data/engineering_faculty.py`, `data/earth_sciences_faculty.py`, and `data/design_faculty.py`: posted section imports and inference rules.
- `data/rooms.py` and `data/timeslots.py`: shared room and time-slot definitions.
- `data/postedschedule/`: downloaded schedule records by subject prefix.
- `data/compare-posted`: compares a generated placement with posted timed meetings; `--all` includes unchanged meetings.
- `scheduling.md`: faculty-facing guidance. Useful background, but the Python files above are the source of truth for what the system actually consumes.

## End-to-end flow

Input does not go directly into Rust source. The workflow is:

1. `data/build` deletes `timetable.db`, loads `schema.sql`, creates the term and holidays, then calls:
   - `courses.build_courses(db)`
   - `rooms.build_rooms(db)` and `timeslots.build_time_slots(db)`
   - `computing.build_pre(db)`
   - `computingfaculty.build_faculty(db)`
   - `mathematics_faculty.build_faculty(db)`
   - `physics_faculty.build_faculty(db)`, `engineering_faculty.build_faculty(db)`, `earth_sciences_faculty.build_faculty(db)`, and `design_faculty.build_faculty(db)`
   - `biotechnology_faculty.build_faculty(db)`, `biology_faculty.build_faculty(db)`, and `chemistry_faculty.build_faculty(db)`
   - `computing.build_post(db)`
   - `biology.build_programs(db)` and `chemistry.build_programs(db)`
   - `engineering.build_programs(db)`
   - `earth_sciences.build_programs(db)` and `design.build_programs(db)`
   - `mathematics.build_programs(db)`
2. Those Python scripts populate normalized tables in SQLite through `queries.DB`.
3. `schema.sql` defines views such as `sections_to_be_scheduled`, `time_slots_available_to_sections`, `conflict_pairs`, and `anti_conflict_pairs`.
4. The Rust solver reads those views from SQLite. The real contract is the database contents, not the Python source layout.

The raw tables preserve provenance where practical. The Rust-facing views are
allowed to be lossy because they are solver inputs, but avoid collapsing
different input sources in base tables.

Cross-listings share a physical meeting, not a curricular identity. Expand
conflict groups and apply program-specific reductions to the original listings;
then apply prerequisite exclusions and section-count discounts using those
listings' own courses. Only after those calculations may surviving pairs map to
representative sections and merge by minimum priority. Remove mapped self-pairs
and export each physical pair once with the representatives' departments.
Choosing a different representative must not change curricular pressure. A
reduction or prerequisite exclusion for one listing must not cancel a surviving
relationship from another listing. Keep regression coverage for course/course,
section/section, mixed rules, listing-specific reductions and discounts, and
representative independence in `test_dry_run.py`.

Broad elective pools contribute no conflict pressure when students choose a
modest fraction of a large list, especially across programs or departments.
Focused requirements can remain modeled, such as CS mathematics electives.
Record every retained or omitted pool judgment and its rationale in
`QUESTIONS.md` for chair review. Optional tracks may have separate internal and
core conflicts without introducing a clique across all track choices.

`./compare-posted [placement_id]` reports meeting-level differences for the
selected placement, defaulting to the newest. It maps cross-listings, SI, and
known split meetings back to their posted CRNs. Rebuilding deletes placements,
so generate a new placement before comparing after an input rebuild.

## What counts as a scheduleable section

A section is only scheduled if it has at least one time-slot tag after all input is loaded.

- In practice, this means sections created with time tags such as `'3 credit bell schedule'`, `'TR1200+75'`, or another time-slot tag/group.
- Sections with no time tags and no room tags are still recorded in the DB, but they are not part of `sections_to_be_scheduled`.
- This is how online, internship, research, and placeholder workload sections are represented in current computing input.

Examples from `data/computingfaculty.py`:

- `db.make_faculty_section('Carol Stander', 'CS 1400-40')` records an online section but does not schedule it.
- `db.make_faculty_section('Jeff Compas', 'CS 2450-01', '3 credit bell schedule', 'flex')` creates a scheduleable section with allowed time patterns and allowed rooms.

## Core data model

The current computing input uses four layers of data.

### 1. Catalog and departments

`data/courses.py` creates departments and courses with:

- `make_department(department)`
- `make_course(department, course, course_name, minimum_credit_hours, maximum_credit_hours)`
- `add_course_rotation(course, rotation)`
- optionally prereqs/coreqs

This establishes the course catalog used by sections and program-conflict rules.

### 2. Department-wide scheduling primitives

`data/computing.py` defines:

- buildings and rooms
- room tags
- concrete time slots
- time-slot tags
- extra special-case courses
- academic programs and program conflict groups
- anti-conflict rules

This file is the department-wide constraint vocabulary that faculty input builds on.

### 3. Faculty and section assignments

`data/computingfaculty.py` defines:

- faculty members
- each faculty member's baseline availability
- which sections each faculty member is teaching
- each section's allowed rooms and allowed time slots/time patterns

The main calls are:

- `make_faculty(name, department, available_intervals)`
- `make_faculty_section(faculty, section, *tags, credit_hours=None)`
- `make_section_with_no_faculty(section, *tags, credit_hours=None)`
- `assign_faculty_to_existing_section(faculty, section)`

### 4. Faculty preferences

Also in `data/computingfaculty.py`, each faculty member gets:

- `faculty_preferences(faculty, days_to_check, *preferences)`

These are soft preferences, not hard requirements, unless the preference records
hard unavailability.

## Strings and naming conventions

### Section names

Sections use `COURSE-SECTION`, for example:

- `CS 2450-01`
- `IT 1500-40A`

The course part must already exist in `courses`.

### Room names and room tags

Rooms are named like `Smith 107`. `make_room` also creates tags:

- every room automatically gets its own tag matching the full room name
- extra tags can alias groups of rooms, such as `flex`, `macs`, `pcs`, `stadium`

A section can then allow one or more room tags instead of listing rooms individually.

### Time slot names

Concrete time slots are strings like:

- `MWF0900+50`
- `TR1030+75`
- `M1630+150`

Format:

- day letters from `MTWRFSU`
- four-digit start time in 24-hour `HHMM`
- `+duration_minutes`

Examples:

- `MWF0900+50` means M/W/F at 9:00 for 50 minutes.
- `TR1200+75` means T/R at noon for 75 minutes.

### Time-slot tags

`make_time_slot` can attach group tags to concrete time slots, for example:

- `3 credit bell schedule`
- `MWF 3×50 bell schedule`
- `2×75 bell schedule`
- `TR 2×75 bell schedule`

Sections usually reference these tags rather than enumerating every allowed time slot.

## How section tags work

`make_faculty_section(..., *tags)` and `make_section_with_no_faculty(..., *tags)` accept a mixed list of tags.

Each tag is interpreted as:

- a room tag, if it matches an existing room tag
- otherwise a time-slot tag, if it matches an existing time-slot tag
- otherwise a new concrete time slot, if the string parses as a time-slot name

Important consequences:

- Exact room names work because every room is also a room tag.
- Exact time-slot strings work even if they were not predeclared; `queries.py` will create the concrete time slot on demand.
- A section can mix room tags and time tags in the same call.

Examples:

- `db.make_faculty_section('Phil Daley', 'IT 2400-01', '3 credit bell schedule', 'Smith 107')`
- `db.make_faculty_section('Lora Klein', 'SA 1400-01', 'TR0930+80')`
- `db.make_faculty_section('Curtis Larsen', 'CS 6300-50', 'T1800+150', 'W1800+150', 'R1800+150', 'Smith 116')`

Section tags do not carry soft priorities. They only define the allowed rooms
and time slots for a section. Use `AvoidSectionInRooms(...)` and
`AvoidSectionInTimeSlots(...)` for soft section-specific restrictions.

Science lecture-room tags are based on the posted room's provisional capacity.
They include the posted room and college-owned SET/Snow science lecture rooms
at least as large; floated Brown, COE, GCB, and UHB rooms are not added as
alternatives for other posted rooms. The posted room itself remains allowed.
Ordinary 110- and 170-minute labs may use staggered daytime starts on the same
posted day or day pair; SI meetings of 50 minutes may use weekday daytime
slots. Special split labs and outside-hours meetings retain their explicit
times. HURCTR remains fixed pending instructor and travel information.

Implementation details:

- `section_room_tags` and `section_time_slot_tags` are pure allowed-set tables.
- `faculty_section_room_preferences` and `faculty_section_time_slot_preferences` store faculty-authored soft restrictions for assigned sections.
- Section-specific preferences do not add rooms or times to a section. The views apply them only to the concrete room/time intersection between the section tags and the preference tags.
- Multiple positive priorities for the same effective room/time collapse with `MIN(priority)`.

Section credit hours
--------------------

Every section stores one exact credit value. The section creation handlers infer it for a
fixed-credit catalog course. A variable-credit course must pass `credit_hours=...`; input
validation rejects a missing value or one outside the catalog range. Faculty assignment to
an existing shared section does not set credits because the section creator owns that value.

## Faculty availability input

Availability is passed as a list of `TimeInterval(days, start, end)`.

Example default used in computing:

- `TimeInterval('MTWR', '0900', '1630')`
- `TimeInterval('F', '0900', '1200')`

This means:

- faculty are available at those times
- soft time penalties are stored separately by `AvoidTimeSlot`
- unavailable time is represented by not covering it, or by explicitly excluding a concrete slot with `UnavailableTimeSlot`

Faculty availability intersects with section time tags. A time slot is available to a section only if:

- the section is allowed in that slot, and
- every assigned faculty member is available for the entire slot
- no assigned faculty member has marked the concrete slot unavailable

An explicit section time slot makes the assigned faculty available for that
section at that time, but `UnavailableTimeSlot(...)` still excludes it.

Implementation details:

- `faculty_availability` stores base teaching ranges only.
- `faculty_time_slot_preferences` stores concrete slots from `AvoidTimeSlot`.
- `faculty_unavailable_time_slots` stores concrete slots from `UnavailableTimeSlot`.
- `assign_faculty_to_existing_section(...)` must not mutate availability. The explicit-section-time exception is applied in `time_slots_available_to_sections`.
- `time_slots_available_to_faculty` intentionally does not know about section-specific explicit time slots.

## Faculty preference system

Faculty preferences are created by `faculty_preferences(faculty, days_to_check, *prefs)`.

`days_to_check` is a compact day string, usually `MT` in current computing data. It tells the solver which days should represent the weekly distribution pattern for preferences like day-off and evenly-spread requests.

Priority behavior in `queries.py`:

- explicit and automatic stated faculty preference priorities live in the range
  `10..99`; entropy balancing maps them to effective priorities `10..49`
- if a preference omits `priority=...`, priorities are assigned in list order starting at `10`
- faculty preference rows only reach the solver for faculty who have at least one scheduleable section in `sections_to_be_scheduled`
- `days_to_check` must contain at least two representative days for day-distribution preferences; `faculty_preferences(..., 'M', ...)` is not valid for `WantADayOff()`, `DoNotWantADayOff()`, or `WantClassesEvenlySpreadAcrossDays()`

### Preferences that change faculty-wide schedule shape

- `WantADayOff()`
  Prefers exactly one of the representative days in `days_to_check` to have no classes.
  In current computing input with `days_to_check='MT'`, this means preferring all classes on the MWF side or all classes on the TR side.
  SAT encoding requires the faculty to have more than one scheduleable section.

- `DoNotWantADayOff()`
  Prefers teaching on both representative days.
  This is evaluated by whether each representative day has any class at all, not by balancing counts or minutes.

- `WantSameDayOffAs('Other Faculty')`
  Requires both faculty to have exactly one empty representative day and for that
  day to match. Each faculty lists a reciprocal request at their own desired
  rank, with identical `days_to_check` of at least two days and at least one
  schedulable section each. A faculty member may participate in only one pair.
  Individual `WantADayOff()` requests are optional; this request does not
  introduce a separate individual fallback preference. Directed rows in
  `faculty_shared_day_off_preferences` retain both ranks for entropy-prefix
  construction. References may point forward within the build transaction.
  `data/build` calls `validate_shared_day_off_preferences()` after input loading.
  The pair becomes one effective preference, and a failure counts once at its
  joint effective priority.

- `WantClassesEvenlySpreadAcrossDays()`
  Prefers balanced teaching counts across the representative days.
  This is only permitted when the faculty has more than 3 scheduleable sections. The SAT encoder rejects it for `<=3` sections.
  The balance check is by section count on each representative day, ignoring representative days with zero classes. With `days_to_check='MT'`, the preference is satisfied when the nonempty MWF-side and TR-side counts differ by at most 1.

- `WantBackToBackClassesInTheSameRoom()`
  Penalizes room switches inside a same-day teaching cluster.
  It only matters for back-to-back section pairs that can share at least one room with no room preference penalty.

- `WantClassesPackedIntoAsFewRoomsAsPossible()`
  Prefers using as few distinct rooms as possible across a faculty member's scheduled sections.
  The target room count is computed from desired rooms only. If the solver cannot derive a target strictly better than "one room per section", this preference becomes trivial and is omitted.

### Cluster and gap preferences

These reason about same-day runs of classes.

- A cluster is one or more back-to-back classes with no break larger than `max_gap_within_cluster`. That DB field is currently fixed at 50 minutes by `queries.py`.
- A gap is time between clusters.
- The SAT encoding uses the same fixed 50-minute threshold for `AvoidClassCluster...`, `AvoidGapBetweenClassClusters...`, and `WantBackToBackClassesInTheSameRoom()`.

Available preference constructors:

- `AvoidGapBetweenClassClustersShorterThan('1h30m')`
- `AvoidGapBetweenClassClustersLongerThan('1h45m')`
- `AvoidClassClusterShorterThan('1h50m')`
- `AvoidClassClusterLongerThan('2h45m')`

Durations may be integers in minutes or strings like `2h45m`.

### Preferences that alter availability

- `AvoidTimeSlot('MWF0900+50')`
  Keeps the slot available but marks it as undesirable at the next faculty preference priority.

- `UnavailableTimeSlot('TR1500+75')`
  Removes the slot from faculty availability entirely. This is effectively a hard constraint.

### Section-specific faculty preferences

- `AvoidSectionInTimeSlots(section, [time_slot_or_tag, ...])`
  Makes the listed time slots or time-slot tags undesirable for that specific section at the chosen preference priority.
  Tags must resolve to existing time-slot tags or concrete time slots. The preference applies only to the intersection with the section's allowed time slots.

- `AvoidSectionInRooms(section, [room_or_tag, ...])`
  Makes the listed rooms or room tags undesirable for that specific section at the chosen preference priority.
  The preference applies only to the intersection with the section's allowed rooms.

- `UseSameTimePattern([section_a, section_b, ...])`
  Adds a soft rule that the listed sections should use the same time pattern.
  In practice this compares section time patterns, not specific start times alone.
  Only use this when there are at least two sections and at least two distinct candidate time patterns across them; otherwise the SAT encoder treats it as invalid/trivial input.

`UseSameTimePattern` is stored in `faculty_time_pattern_matches` and
`faculty_time_pattern_match_sections` so the source faculty remains visible.
Compatibility views named `time_pattern_matches` and
`time_pattern_match_sections` keep the Rust input unchanged.

## Auditing input

Run `data/audit` after rebuilding. In addition to general data checks, it
reports section-specific Avoid preferences that do not intersect any assigned
room or time. Those are not build errors because a broad or stale tag can be
harmless, but they usually mean the preference has no effect.

## Department-wide conflict input

`data/computing.py` also encodes student-facing conflict structure.

### Programs and conflict groups

Calls:

- `make_program(program, department)`
- `make_conflict(program, conflict_name, priority, boost_or_reduce, courses_or_sections, *, include=())`

Semantics:

- priorities here are `0..9`
- `0` is a hard conflict
- `1..9` are soft curriculum-conflict priorities
- `boost` means "students likely need both, so avoid overlap"
- `reduce` means "students usually need only one, so reduce or cancel overlap pressure"

These rules are combined in SQL views into section-to-section conflict pairs. The solver does not read `make_conflict` rows directly; it reads the derived `conflict_pairs` view.

`include=['core requirements']` includes all direct and transitive members of existing groups in the same program. Multiple included groups and an empty direct member list are supported. The receiving group applies its own priority and boost/reduce mode; each included group still contributes its own independent rule. Membership is stored as edges in `conflict_includes` and expanded by SQL, with duplicates removed before pairing. Missing references and inclusion cycles are rejected. Referenced groups must be unlinked before deletion; renaming a group cascades to references. Define dependencies before groups that include them.

Computing uses this for:

- core requirement groups
- track groups
- "only need one of these" reductions
- "spread out multiple sections of the same course" soft conflicts

### Anti-conflicts

Call:

- `add_anti_conflict(priority, single_section, group)`

Meaning:

- the named `single_section` should be kept available against at least one member of the group
- group entries can be full section names or whole course names
- priority is again in the `0..9` range

Current computing data uses:

- `db.add_anti_conflict(5, 'CS 1030-01', ['CS 1400'])`

This is a niche mechanism. It is not the same as ordinary conflict avoidance.

## Features present in the API but mostly unused in current computing input

- `add_cross_listing(primary, sections)`
  Secondary cross-listed sections inherit scheduling from the primary section and may not carry their own room/time/faculty rows.
  Raw section credits remain distinct; solver-facing rows use the primary section's credits for the shared placement.

- `add_multiple_section_override(course, override_section_count)`
  Adjusts how curriculum-conflict discounts are computed for courses with multiple sections.

These matter if new input starts using them, but they are not central to the current computing workflow.

## Practical conventions used by the current computing data

- Default faculty availability is MTWR `09:00-16:30` and F `09:00-12:00`.
- Most in-person 3-credit classes use the time-slot tag `3 credit bell schedule`.
- Common room tags are `flex`, `macs`, `pcs`, and `stadium`.
- Online sections are still listed for workload/reporting, but they usually carry no room or time tags and therefore are not scheduled.
- Placeholder faculty entries such as `DS Hire` are valid and useful when multiple sections must be kept non-overlapping before a real hire is known.
- Department-wide time vocabulary and room vocabulary are created before faculty assignments, so faculty input can reference shared tags by name.

## Priority ranges summary

- `0`: hard conflicts and unplaced-section penalty level
- `1..9`: program conflicts and anti-conflicts
- `10..49`: effective faculty and section soft preferences; automatically
  assigned stated faculty ranks can extend through `99` before balancing

Lower numbers are more important.

## If you need to update input

For computing, the usual places to edit are:

- `data/courses.py` for course catalog changes
- `data/computing.py` for rooms, time patterns, programs, and department-wide conflict structure
- `data/computingfaculty.py` for faculty assignments, section constraints, availability, and preferences

After changes, rebuild with:

```bash
cd data
./build
```

That regenerates `timetable.db`, which is the artifact the Rust solver consumes.

Note: during all planning and preparing work, timetable.db is a transient asset that we rebuild on a whim using the build script. Do not back it up or hesitate to make changes to it. It only becomes durable when all input is stable and we generate the final schedule. Think of it more as a transformation pipeline for the input than a database.
