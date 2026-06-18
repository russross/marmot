# Marmot Input Format

This repo is larger than the timetabling input pipeline. For input work, focus on:

- `data/build`: creates `data/timetable.db` from scratch.
- `data/schema.sql`: the SQLite schema and scheduling views the Rust solver reads.
- `data/queries.py`: the Python API used to populate the schema.
- `data/courses.py`: the generated course catalog and rotations.
- `data/computing.py`: department-wide setup for computing.
- `data/computingfaculty.py`: faculty section assignments and faculty preferences.
- `scheduling.md`: faculty-facing guidance. Useful background, but the Python files above are the source of truth for what the system actually consumes.

## End-to-end flow

Input does not go directly into Rust source. The workflow is:

1. `data/build` deletes `timetable.db`, loads `schema.sql`, creates the term and holidays, then calls:
   - `courses.build_courses(db)`
   - `computing.build_pre(db)`
   - `computingfaculty.build_faculty(db)`
   - `computing.build_post(db)`
2. Those Python scripts populate normalized tables in SQLite through `queries.DB`.
3. `schema.sql` defines views such as `sections_to_be_scheduled`, `time_slots_available_to_sections`, `conflict_pairs`, and `anti_conflict_pairs`.
4. The Rust solver reads those views from SQLite. The real contract is the database contents, not the Python source layout.

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
- `make_course(department, course, course_name)`
- `add_course_rotation(course, term)`
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
- `make_faculty_section(faculty, section, *tags)`
- `make_section_with_no_faculty(section, *tags)`
- `assign_faculty_to_existing_section(faculty, section)`

### 4. Faculty preferences

Also in `data/computingfaculty.py`, each faculty member gets:

- `faculty_preferences(faculty, days_to_check, *preferences)`

These are soft preferences, not hard requirements, unless the preference changes availability directly.

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

### Optional priorities on section tags

Tags can be suffixed with `:N`, for example `tag:12`.

- `0` or omitted means the tag is fully allowed.
- nonzero values create a soft penalty at that priority if the solver uses that room or time slot.

This is stored in:

- `section_room_tags.room_priority`
- `section_time_slot_tags.time_slot_priority`

Current computing input mostly leaves base section tags unprioritized and adds softer avoid rules later through faculty preferences.

## Faculty availability input

Availability is passed as a list of `TimeInterval(days, start, end, priority=25)`.

Example default used in computing:

- `TimeInterval('MTWR', '0900', '1630')`
- `TimeInterval('F', '0900', '1200')`

This means:

- faculty are available at those times
- if a priority less than 25 is attached, the time is available but carries a soft penalty
- unavailable time is represented by not covering it, or by explicitly subtracting it later with `UnavailableTimeSlot`

Faculty availability intersects with section time tags. A time slot is available to a section only if:

- the section is allowed in that slot, and
- every assigned faculty member is available for the entire slot

## Faculty preference system

Faculty preferences are created by `faculty_preferences(faculty, days_to_check, *prefs)`.

`days_to_check` is a compact day string, usually `MT` in current computing data. It tells the solver which days should represent the weekly distribution pattern for preferences like day-off and evenly-spread requests.

Priority behavior in `queries.py`:

- faculty preference priorities live in the range `10..25`
- if a preference omits `priority=...`, priorities are assigned in list order starting at `10`
- `WantADayOff()` consumes two priority slots
- `AvoidSectionInRooms(...)` does not consume a slot; it shares the next priority level unless explicitly given one
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

- `WantClassesEvenlySpreadAcrossDays()`
  Prefers balanced teaching counts across the representative days.
  This is only permitted when the faculty has more than 3 scheduleable sections. The SAT encoder rejects it for `<=3` sections.
  The balance check is by section count on each representative day, ignoring representative days with zero classes. With `days_to_check='MT'`, the preference is satisfied when the nonempty MWF-side and TR-side counts differ by at most 1.

- `WantBackToBackClassesInTheSameRoom()`
  Penalizes room switches inside a same-day teaching cluster.
  It only matters for back-to-back section pairs that can share at least one room with no room-priority penalty.

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
  Tags must resolve to existing time-slot tags or concrete time slots; invalid concrete slots raise an input error.

- `AvoidSectionInRooms(section, [room_or_tag, ...])`
  Makes the listed rooms or room tags undesirable for that specific section.
  Unlike most faculty preferences, this does not consume a priority slot.

- `UseSameTimePattern([section_a, section_b, ...])`
  Adds a soft rule that the listed sections should use the same time pattern.
  In practice this compares section time patterns, not specific start times alone.
  Only use this when there are at least two sections and at least two distinct candidate time patterns across them; otherwise the SAT encoder treats it as invalid/trivial input.

## Department-wide conflict input

`data/computing.py` also encodes student-facing conflict structure.

### Programs and conflict groups

Calls:

- `make_program(program, department)`
- `make_conflict(program, conflict_name, priority, boost_or_reduce, courses_or_sections)`

Semantics:

- priorities here are `0..9`
- `0` is a hard conflict
- `1..9` are soft curriculum-conflict priorities
- `boost` means "students likely need both, so avoid overlap"
- `reduce` means "students usually need only one, so reduce or cancel overlap pressure"

These rules are combined in SQL views into section-to-section conflict pairs. The solver does not read `make_conflict` rows directly; it reads the derived `conflict_pairs` view.

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

- `add_multiple_section_override(course, section_count)`
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
- `10..25`: faculty and section soft preferences

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
