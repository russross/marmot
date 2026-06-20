--
--
-- INPUT DATA
--
--

BEGIN;

CREATE TABLE terms (
    term                        TEXT PRIMARY KEY,
    start_date                  DATE NOT NULL,
    end_date                    DATE NOT NULL
) WITHOUT ROWID;

CREATE TABLE holidays (
    holiday                     DATE PRIMARY KEY
) WITHOUT ROWID;

CREATE TABLE buildings (
    building                    TEXT PRIMARY KEY
) WITHOUT ROWID;

-- Room is the natural key used everywhere else. building and room_number are
-- derived from that key so room tags, placements, and solver output cannot
-- drift from the printable room name.
CREATE TABLE rooms (
    room                        TEXT PRIMARY KEY,
    building                    TEXT GENERATED ALWAYS AS (SUBSTR(room, 1, INSTR(room, ' ') - 1)) VIRTUAL NOT NULL,
    room_number                 TEXT GENERATED ALWAYS AS (SUBSTR(room, LENGTH(building) + 2)) VIRTUAL NOT NULL,
    capacity                    INTEGER NOT NULL,

    CHECK (LENGTH(room_number) > 0),

    FOREIGN KEY (building) REFERENCES buildings (building) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE room_tags (
    room_tag                    TEXT PRIMARY KEY
) WITHOUT ROWID;

-- Tags are the input vocabulary for allowed room sets. A tag may name one
-- room or a group of rooms; section_room_tags chooses tags, and views expand
-- them to concrete rooms.
CREATE TABLE rooms_room_tags (
    room_tag                    TEXT NOT NULL,
    room                        TEXT NOT NULL,

    PRIMARY KEY (room_tag, room),
    FOREIGN KEY (room_tag) REFERENCES room_tags (room_tag) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (room) REFERENCES rooms (room) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

-- time_slot is the canonical encoded meeting pattern. The generated columns
-- are query keys derived from that string, so callers cannot disagree about
-- day order, start time, duration, or first-day sorting.
CREATE TABLE time_slots (
    time_slot                   TEXT PRIMARY KEY,
    days                        TEXT GENERATED ALWAYS AS (SUBSTR(time_slot, 1, LENGTH(time_slot) - LENGTH(duration) - 5)) VIRTUAL NOT NULL,
    start_time                  INTEGER GENERATED ALWAYS AS (CAST(SUBSTR(time_slot, -(LENGTH(duration) + 5), 2) AS INTEGER) * 60 + CAST(SUBSTR(time_slot, -(LENGTH(duration) + 3), 2) AS INTEGER)) VIRTUAL NOT NULL,
    duration                    INTEGER GENERATED ALWAYS AS (CAST(SUBSTR(time_slot, INSTR(time_slot, '+')) AS INTEGER)) VIRTUAL NOT NULL,
    first_day                   INTEGER GENERATED ALWAYS AS (CAST(
        REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(
            SUBSTR(days, 1, 1), 'M','1'), 'T','2'), 'W','3'), 'R','4'), 'F','5'), 'S','6'), 'U','7') AS INTEGER)) VIRTUAL NOT NULL,

    CHECK (start_time >= 0 AND start_time % 5 = 0),
    CHECK (duration > 0 AND duration % 5 = 0),
    CHECK (start_time + duration < 24*60),
    CHECK (LENGTH(days) > 0 AND INSTR(days, '$') = 0 AND
        REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE('$'||days,
            '$M','$'), '$T','$'), '$W','$'), '$R','$'), '$F','$'), '$S','$'), '$U','$') = '$'),
    CHECK (days || SUBSTR('00'||CAST(start_time / 60 AS TEXT), -2) || SUBSTR('00'||CAST(start_time % 60 AS TEXT), -2) || '+' || CAST(duration AS TEXT) = time_slot)
) WITHOUT ROWID;

CREATE TABLE time_slot_tags (
    time_slot_tag               TEXT PRIMARY KEY
) WITHOUT ROWID;

-- Time-slot tags are the input vocabulary for allowed time sets. A tag can be
-- broad, such as a bell-schedule group, or can be the same string as one
-- concrete time_slot for an explicit assignment.
CREATE TABLE time_slots_time_slot_tags (
    time_slot_tag               TEXT NOT NULL,
    time_slot                   TEXT NOT NULL,

    PRIMARY KEY (time_slot_tag, time_slot),
    FOREIGN KEY (time_slot_tag) REFERENCES time_slot_tags (time_slot_tag) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (time_slot) REFERENCES time_slots (time_slot) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE departments (
    department                  TEXT PRIMARY KEY
) WITHOUT ROWID;

CREATE TABLE programs (
    program                     TEXT PRIMARY KEY,
    department                  TEXT NOT NULL,

    FOREIGN KEY (department) REFERENCES departments (department) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

-- affiliation is deliberately not named department. It is faculty provenance,
-- while courses.department and solver-facing department columns describe course
-- ownership. Cross-department teaching is represented through faculty_sections.
CREATE TABLE faculty (
    faculty                     TEXT PRIMARY KEY,
    affiliation                 TEXT NOT NULL,

    FOREIGN KEY (affiliation) REFERENCES departments (department) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

-- Base availability is just when the faculty member can normally teach. It is
-- not section-specific and carries no soft priority; hard blocks and soft time
-- penalties live in the concrete time-slot tables below.
CREATE TABLE faculty_availability (
    faculty                     TEXT NOT NULL,
    day_of_week                 TEXT NOT NULL,
    start_time                  INTEGER NOT NULL,
    duration                    INTEGER NOT NULL,
    readable                    TEXT GENERATED ALWAYS AS (day_of_week||SUBSTR('00'||CAST(start_time / 60 AS TEXT), -2) || SUBSTR('00'||CAST(start_time % 60 AS TEXT), -2) || '-' || SUBSTR('00'||CAST((start_time + duration) / 60 AS TEXT), -2) || SUBSTR('00'||CAST((start_time + duration) % 60 AS TEXT), -2)) VIRTUAL NOT NULL,

    CHECK (day_of_week IN ('M', 'T', 'W', 'R', 'F', 'S', 'U')),
    CHECK (start_time >= 0 AND start_time % 5 = 0),
    CHECK (duration > 0 AND duration % 5 = 0),
    CHECK (start_time + duration < 24*60),

    PRIMARY KEY (faculty, day_of_week, start_time),
    FOREIGN KEY (faculty) REFERENCES faculty (faculty) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

-- Hard exclusions are concrete time slots, not intervals. Keeping them at the
-- same granularity as solver assignments makes the exclusion auditable and
-- lets explicit section times obey hard blocks.
CREATE TABLE faculty_unavailable_time_slots (
    faculty                     TEXT NOT NULL,
    time_slot                   TEXT NOT NULL,

    PRIMARY KEY (faculty, time_slot),
    FOREIGN KEY (faculty) REFERENCES faculty (faculty) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (time_slot) REFERENCES time_slots (time_slot) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

-- General faculty time preferences apply to concrete time slots, independent
-- of any one section. Section-specific time preferences are kept separately so
-- explicit section assignments can be audited without mutating availability.
CREATE TABLE faculty_time_slot_preferences (
    faculty                     TEXT NOT NULL,
    time_slot                   TEXT NOT NULL,
    time_slot_priority          INTEGER NOT NULL,

    CHECK (time_slot_priority >= 10 AND time_slot_priority < 26),

    PRIMARY KEY (faculty, time_slot),
    FOREIGN KEY (faculty) REFERENCES faculty (faculty) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (time_slot) REFERENCES time_slots (time_slot) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

-- One row stores the scalar faculty preferences that apply across the faculty
-- member's schedulable sections. Interval-shaped preferences are separated
-- below because a faculty member can have several gap/cluster rules.
CREATE TABLE faculty_preferences (
    faculty                     TEXT PRIMARY KEY,
    days_to_check               TEXT NOT NULL,
    days_off                    INTEGER,
    days_off_priority           INTEGER,
    evenly_spread_priority      INTEGER,
    no_room_switch_priority     INTEGER,
    too_many_rooms_priority     INTEGER,
    max_gap_within_cluster      INTEGER NOT NULL,

    CHECK (INSTR(days_to_check, '$') = 0 AND
        REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE('$'||days_to_check,
            '$M','$'), '$T','$'), '$W','$'), '$R','$'), '$F','$'), '$S','$'), '$U','$') = '$'),
    CHECK (days_off IS NULL OR days_off >= 0 AND days_off < 7),
    CHECK (days_off_priority IS NULL OR days_off_priority >= 10 AND days_off_priority < 26),
    CHECK (days_off_priority IS NULL AND days_off IS NULL OR days_off_priority IS NOT NULL AND days_off IS NOT NULL),
    CHECK (days_off_priority IS NULL OR LENGTH(days_to_check) > 1),
    CHECK (evenly_spread_priority IS NULL OR evenly_spread_priority >= 10 AND evenly_spread_priority < 26),
    CHECK (evenly_spread_priority IS NULL OR LENGTH(days_to_check) > 1),
    CHECK (no_room_switch_priority IS NULL OR no_room_switch_priority >= 10 AND no_room_switch_priority < 26),
    CHECK (too_many_rooms_priority IS NULL OR too_many_rooms_priority >= 10 AND too_many_rooms_priority < 26),
    CHECK (max_gap_within_cluster >= 0 AND max_gap_within_cluster < 120),

    FOREIGN KEY (faculty) REFERENCES faculty (faculty) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

-- Gap and cluster preferences share one compact shape. is_cluster chooses the
-- measured object, and is_too_short chooses which side of interval_minutes is a
-- violation.
CREATE TABLE faculty_preference_intervals (
    faculty                     TEXT NOT NULL,
    is_cluster                  BOOLEAN NOT NULL,       -- true => cluster, false => gap
    is_too_short                BOOLEAN NOT NULL,       -- true => too short, false => too long
    interval_minutes            INTEGER NOT NULL,
    interval_priority           INTEGER,
    -- e.g., cluster shorter than 110 minutes with priority 16,
    -- or    gap     longer  than 105 minutes with priority 11

    CHECK (interval_minutes > 0 AND interval_minutes < 24*60),
    CHECK (interval_priority IS NULL OR interval_priority >= 10 AND interval_priority < 26),

    PRIMARY KEY (faculty, is_cluster, is_too_short, interval_minutes),
    FOREIGN KEY (faculty) REFERENCES faculty_preferences (faculty)
) WITHOUT ROWID;

-- Courses own the department relationship used by solver filtering. The
-- prefix and course_number generated columns keep catalog parsing local to the
-- natural course key instead of duplicating it in input data.
CREATE TABLE courses (
    course                      TEXT PRIMARY KEY,
    department                  TEXT NOT NULL,
    course_name                 TEXT NOT NULL,
    prefix                      TEXT GENERATED ALWAYS AS (SUBSTR(course, 1, INSTR(course, ' ') - 1)) VIRTUAL NOT NULL,
    course_number               TEXT GENERATED ALWAYS AS (SUBSTR(course, INSTR(course, ' ') + 1)) VIRTUAL NOT NULL,

    CHECK (LENGTH(prefix) >= 1),
    CHECK (LENGTH(course_number) >= 4),

    FOREIGN KEY (department) REFERENCES departments (department) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE course_rotations (
    course                      TEXT NOT NULL,
    rotation                    TEXT NOT NULL,

    CHECK (rotation IN ('fall', 'spring', 'summer')),

    PRIMARY KEY (course, rotation),
    FOREIGN KEY (course) REFERENCES courses (course)
) WITHOUT ROWID;

-- Directional course relationships. conflict_pairs uses their transitive
-- closure to avoid penalizing courses students are expected to take in order.
CREATE TABLE prereqs (
    course                      TEXT NOT NULL,
    prereq                      TEXT NOT NULL,

    PRIMARY KEY (course, prereq),
    FOREIGN KEY (course) REFERENCES courses (course) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (prereq) REFERENCES courses (course) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

-- Coreqs participate in prereq traversal for indirect prerequisites, but
-- immediate coreqs are removed from that closure because they are meant to be
-- taken together.
CREATE TABLE coreqs (
    course                      TEXT NOT NULL,
    coreq                       TEXT NOT NULL,

    PRIMARY KEY (course, coreq),
    FOREIGN KEY (course) REFERENCES courses (course) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (coreq) REFERENCES courses (course) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

-- The section key embeds the course key. Generated columns preserve natural
-- joins from sections to courses while keeping the source input as the familiar
-- COURSE-SECTION string.
CREATE TABLE sections (
    section                     TEXT PRIMARY KEY,
    course                      TEXT GENERATED ALWAYS AS (SUBSTR(section, 1, INSTR(section, '-') - 1)) VIRTUAL NOT NULL,
    section_number              TEXT GENERATED ALWAYS AS (SUBSTR(section, INSTR(section, '-') + 1)) VIRTUAL NOT NULL,

    CHECK (LENGTH(course) >= 6),
    CHECK (LENGTH(section_number) >= 2),
    CHECK (course || '-' || section_number = section),

    FOREIGN KEY (course) REFERENCES courses (course) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE INDEX sections_course ON sections (course, section);

-- Section tags define allowed rooms only. Soft room penalties are not stored
-- here because they are faculty-authored preferences, not changes to the
-- section's allowed set.
CREATE TABLE section_room_tags (
    section                     TEXT NOT NULL,
    room_tag                    TEXT NOT NULL,

    PRIMARY KEY (section, room_tag),
    FOREIGN KEY (section) REFERENCES sections (section) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (room_tag) REFERENCES room_tags (room_tag) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

-- Section time tags define allowed time slots. A concrete time_slot used as a
-- tag is an explicit section assignment; views decide how that interacts with
-- faculty availability and hard unavailable slots.
CREATE TABLE section_time_slot_tags (
    section                     TEXT NOT NULL,
    time_slot_tag               TEXT NOT NULL,

    PRIMARY KEY (section, time_slot_tag),
    FOREIGN KEY (section) REFERENCES sections (section) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (time_slot_tag) REFERENCES time_slot_tags (time_slot_tag) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

-- Faculty assignment is intentionally independent of faculty affiliation and
-- course department. The intersection of this table with schedulable sections
-- determines which faculty Rust loads for selected course departments.
CREATE TABLE faculty_sections (
    faculty                     TEXT NOT NULL,
    section                     TEXT NOT NULL,

    PRIMARY KEY (faculty, section),
    FOREIGN KEY (faculty) REFERENCES faculty (faculty) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (section) REFERENCES sections (section) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE INDEX faculty_sections_section ON faculty_sections (section, faculty);

-- Section tags define which rooms/times are possible. These tables record
-- faculty-authored soft restrictions against those possible choices. The tag
-- can be broader or narrower than the section tag; views apply the preference
-- only to the actual room/time intersection.
CREATE TABLE faculty_section_room_preferences (
    faculty                     TEXT NOT NULL,
    section                     TEXT NOT NULL,
    room_tag                    TEXT NOT NULL,
    room_priority               INTEGER NOT NULL,

    CHECK (room_priority >= 10 AND room_priority < 26),

    PRIMARY KEY (faculty, section, room_tag),
    FOREIGN KEY (faculty, section) REFERENCES faculty_sections (faculty, section) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (room_tag) REFERENCES room_tags (room_tag) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE faculty_section_time_slot_preferences (
    faculty                     TEXT NOT NULL,
    section                     TEXT NOT NULL,
    time_slot_tag               TEXT NOT NULL,
    time_slot_priority          INTEGER NOT NULL,

    CHECK (time_slot_priority >= 10 AND time_slot_priority < 26),

    PRIMARY KEY (faculty, section, time_slot_tag),
    FOREIGN KEY (faculty, section) REFERENCES faculty_sections (faculty, section) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (time_slot_tag) REFERENCES time_slot_tags (time_slot_tag) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

-- Cross-listing data is split so the primary section is explicitly declared
-- once, and each secondary section maps to it. Triggers below enforce that
-- secondary sections do not carry separate room, time, or faculty data.
CREATE TABLE cross_listings (
    primary_section             TEXT PRIMARY KEY
) WITHOUT ROWID;

CREATE TABLE cross_listing_sections (
    section                     TEXT NOT NULL,
    primary_section             TEXT NOT NULL,

    PRIMARY KEY (section, primary_section),
    FOREIGN KEY (primary_section) REFERENCES cross_listings (primary_section) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;
CREATE UNIQUE INDEX primary_section ON cross_listing_sections (section);

-- Anti-conflicts model "schedule this with at least one of those" rather than
-- avoidance. The single side is the section that receives the criterion; the
-- group side is supplied by anti_conflict_sections and anti_conflict_courses.
CREATE TABLE anti_conflicts (
    anti_conflict_single        TEXT PRIMARY KEY,
    anti_conflict_priority      INTEGER NOT NULL,

    CHECK (anti_conflict_priority >= 0 AND anti_conflict_priority < 10),

    FOREIGN KEY (anti_conflict_single) REFERENCES sections (section) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE anti_conflict_sections (
    anti_conflict_single        TEXT NOT NULL,
    anti_conflict_section       TEXT NOT NULL,

    CHECK (anti_conflict_single <> anti_conflict_section),

    PRIMARY KEY (anti_conflict_single, anti_conflict_section),
    FOREIGN KEY (anti_conflict_single) REFERENCES anti_conflicts (anti_conflict_single) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE anti_conflict_courses (
    anti_conflict_single        TEXT NOT NULL,
    anti_conflict_course        TEXT NOT NULL,

    CHECK (anti_conflict_course <> SUBSTR(anti_conflict_single, 1, LENGTH(anti_conflict_course))),

    PRIMARY KEY (anti_conflict_single, anti_conflict_course),
    FOREIGN KEY (anti_conflict_single) REFERENCES anti_conflicts (anti_conflict_single) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

-- Faculty time-pattern groups keep faculty provenance in the raw tables. The
-- compatibility views below derive global group names for the Rust input shape.
CREATE TABLE faculty_time_pattern_matches (
    faculty                     TEXT NOT NULL,
    time_pattern_match_name     TEXT NOT NULL,
    time_pattern_match_priority INTEGER NOT NULL,

    CHECK (time_pattern_match_priority >= 10 AND time_pattern_match_priority < 26),

    PRIMARY KEY (faculty, time_pattern_match_name),
    FOREIGN KEY (faculty) REFERENCES faculty (faculty) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE faculty_time_pattern_match_sections (
    faculty                     TEXT NOT NULL,
    time_pattern_match_name     TEXT NOT NULL,
    time_pattern_match_section  TEXT NOT NULL,

    PRIMARY KEY (faculty, time_pattern_match_name, time_pattern_match_section),
    FOREIGN KEY (faculty, time_pattern_match_name) REFERENCES faculty_time_pattern_matches (faculty, time_pattern_match_name) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (time_pattern_match_section) REFERENCES sections (section) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

-- A conflict group is a named rule within one program. boost_priority means
-- the rule can strengthen a conflict; false means it can weaken or cancel one.
-- NULL priority is only valid for the reduce/cancel case.
CREATE TABLE conflicts (
    program                     TEXT NOT NULL,
    conflict_name               TEXT NOT NULL,
    conflict_priority           INTEGER,
    boost_priority              BOOLEAN NOT NULL,

    CHECK (conflict_priority IS NULL OR conflict_priority >= 0 AND conflict_priority < 10),
    CHECK (conflict_priority IS NOT NULL OR NOT boost_priority),

    PRIMARY KEY (program, conflict_name),
    FOREIGN KEY (program) REFERENCES programs (program) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

-- Conflict groups can name whole courses, individual sections, or both. Views
-- expand these raw references into concrete section pairs after cross-listing
-- canonicalization.
CREATE TABLE conflict_courses (
    program                     TEXT NOT NULL,
    conflict_name               TEXT NOT NULL,
    course                      TEXT NOT NULL,

    PRIMARY KEY (program, conflict_name, course),
    FOREIGN KEY (program, conflict_name) REFERENCES conflicts (program, conflict_name) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (course) REFERENCES courses (course) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE conflict_sections (
    program                     TEXT NOT NULL,
    conflict_name               TEXT NOT NULL,
    section                     TEXT NOT NULL,

    PRIMARY KEY (program, conflict_name, section),
    FOREIGN KEY (program, conflict_name) REFERENCES conflicts (program, conflict_name) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (section) REFERENCES sections (section) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

-- Overrides replace the raw count of sections for conflict discounting. They
-- are for cases where "number of rows in sections" is not the right count of
-- student choices.
CREATE TABLE multiple_section_overrides (
    course                      TEXT PRIMARY KEY,
    override_section_count      INTEGER NOT NULL,

    FOREIGN KEY (course) REFERENCES courses (course) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

-- Placements and placement penalties are solver output/history, not input
-- constraints. They live in the same database so generated schedules can be
-- inspected against the exact input that produced them.
CREATE TABLE placements (
    placement_id                INTEGER PRIMARY KEY,
    score                       TEXT NOT NULL,
    sort_score                  TEXT NOT NULL,
    optimum_score_prefix        TEXT NOT NULL,
    faculty_preference_priority_policy TEXT NOT NULL DEFAULT 'stated',
    comment                     TEXT NOT NULL,
    created_at                  TEXT NOT NULL,
    modified_at                 TEXT NOT NULL,

    CHECK (faculty_preference_priority_policy IN ('stated', 'entropy-balanced-v1'))
);

CREATE TABLE placement_sections (
    placement_id                INTEGER NOT NULL,
    section                     TEXT NOT NULL,
    time_slot                   TEXT NOT NULL,
    room                        TEXT,

    PRIMARY KEY (placement_id, section),
    FOREIGN KEY (placement_id) REFERENCES placements (placement_id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (section) REFERENCES sections (section) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (time_slot) REFERENCES time_slots (time_slot) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (room) REFERENCES rooms (room) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE placement_penalties (
    placement_penalty_id        INTEGER PRIMARY KEY,
    placement_id                INTEGER NOT NULL,
    priority                    INTEGER NOT NULL,
    message                     TEXT NOT NULL,

    -- Generated priorities are governed by Rust's MAX_PRIORITY. Input-side
    -- preference tables retain their stricter independently managed bounds.
    CHECK (priority >= 0 AND priority <= 255),
    FOREIGN KEY (placement_id) REFERENCES placements (placement_id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE placement_penalty_sections (
    placement_penalty_id        INTEGER NOT NULL,
    section                     TEXT NOT NULL,

    PRIMARY KEY (placement_penalty_id, section),
    FOREIGN KEY (placement_penalty_id) REFERENCES placement_penalties (placement_penalty_id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (section) REFERENCES sections (section) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE placement_penalty_faculty (
    placement_penalty_id        INTEGER NOT NULL,
    faculty                     TEXT NOT NULL,

    PRIMARY KEY (placement_penalty_id, faculty),
    FOREIGN KEY (placement_penalty_id) REFERENCES placement_penalties (placement_penalty_id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (faculty) REFERENCES faculty (faculty) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;


--
--
-- TRIGGERS ON INPUT DATA
--
--


CREATE TRIGGER terms_one_insert
AFTER INSERT ON terms
WHEN (SELECT COUNT(1) FROM terms) > 1
BEGIN
    SELECT RAISE(ABORT, 'only one term allowed');
END;
CREATE TRIGGER terms_one_update
AFTER UPDATE ON terms
WHEN (SELECT COUNT(1) FROM terms) > 1
BEGIN
    SELECT RAISE(ABORT, 'only one term allowed');
END;

CREATE TRIGGER holidays_in_range_insert
AFTER INSERT ON holidays
WHEN (SELECT COUNT(1) FROM terms, holidays WHERE holiday <= start_date OR holiday >= end_date) > 0
BEGIN
    SELECT RAISE(ABORT, 'holidays must be during the term');
END;
CREATE TRIGGER holidays_in_range_update
AFTER UPDATE ON holidays
WHEN (SELECT COUNT(1) FROM terms, holidays WHERE holiday <= start_date OR holiday >= end_date) > 0
BEGIN
    SELECT RAISE(ABORT, 'holidays must be during the term');
END;

CREATE TRIGGER no_secondary_cross_listing_room_tags_insert
AFTER INSERT ON cross_listing_sections
WHEN (
    SELECT COUNT(1)
    FROM section_room_tags
    NATURAL JOIN cross_listing_sections
) > 0
BEGIN
    SELECT RAISE(ABORT, 'secondary section in cross listing cannot be assigned room tags');
END;
CREATE TRIGGER no_secondary_cross_listing_room_tags_update
AFTER UPDATE ON cross_listing_sections
WHEN (
    SELECT COUNT(1)
    FROM section_room_tags
    NATURAL JOIN cross_listing_sections
) > 0
BEGIN
    SELECT RAISE(ABORT, 'secondary section in cross listing cannot be assigned room tags');
END;

CREATE TRIGGER no_secondary_cross_listing_time_slot_tags_insert
AFTER INSERT ON cross_listing_sections
WHEN (
    SELECT COUNT(1)
    FROM section_time_slot_tags
    NATURAL JOIN cross_listing_sections
) > 0
BEGIN
    SELECT RAISE(ABORT, 'secondary section in cross listing cannot be assigned time_slot tags');
END;
CREATE TRIGGER no_secondary_cross_listing_time_slot_tags_update
AFTER UPDATE ON cross_listing_sections
WHEN (
    SELECT COUNT(1)
    FROM section_time_slot_tags
    NATURAL JOIN cross_listing_sections
) > 0
BEGIN
    SELECT RAISE(ABORT, 'secondary section in cross listing cannot be assigned time_slot tags');
END;

CREATE TRIGGER no_secondary_cross_listing_faculty_insert
AFTER INSERT ON cross_listing_sections
WHEN (
    SELECT COUNT(1)
    FROM faculty_sections
    NATURAL JOIN cross_listing_sections
) > 0
BEGIN
    SELECT RAISE(ABORT, 'secondary section in cross listing cannot be assigned to faculty');
END;
CREATE TRIGGER no_secondary_cross_listing_faculty_update
AFTER UPDATE ON cross_listing_sections
WHEN (
    SELECT COUNT(1)
    FROM faculty_sections
    NATURAL JOIN cross_listing_sections
) > 0
BEGIN
    SELECT RAISE(ABORT, 'secondary section in cross listing cannot be assigned to faculty');
END;

CREATE TRIGGER no_room_tags_for_secondary_cross_listing_insert
AFTER INSERT ON section_room_tags
WHEN (
    SELECT COUNT(1)
    FROM section_room_tags
    NATURAL JOIN cross_listing_sections
) > 0
BEGIN
    SELECT RAISE(ABORT, 'secondary section in cross listing cannot be assigned room tags');
END;
CREATE TRIGGER no_room_tags_for_secondary_cross_listing_update
AFTER UPDATE ON section_room_tags
WHEN (
    SELECT COUNT(1)
    FROM section_room_tags
    NATURAL JOIN cross_listing_sections
) > 0
BEGIN
    SELECT RAISE(ABORT, 'secondary section in cross listing cannot be assigned room tags');
END;

CREATE TRIGGER no_time_slot_tags_for_secondary_cross_listing_insert
AFTER INSERT ON section_time_slot_tags
WHEN (
    SELECT COUNT(1)
    FROM section_time_slot_tags
    NATURAL JOIN cross_listing_sections
) > 0
BEGIN
    SELECT RAISE(ABORT, 'secondary section in cross listing cannot be assigned time_slot tags');
END;
CREATE TRIGGER no_time_slot_tags_for_secondary_cross_listing_update
AFTER UPDATE ON section_time_slot_tags
WHEN (
    SELECT COUNT(1)
    FROM section_time_slot_tags
    NATURAL JOIN cross_listing_sections
) > 0
BEGIN
    SELECT RAISE(ABORT, 'secondary section in cross listing cannot be assigned time_slot tags');
END;

CREATE TRIGGER no_faculty_for_secondary_cross_listing_insert
AFTER INSERT ON faculty_sections
WHEN (
    SELECT COUNT(1)
    FROM faculty_sections
    NATURAL JOIN cross_listing_sections
) > 0
BEGIN
    SELECT RAISE(ABORT, 'secondary section in cross listing cannot be assigned to faculty');
END;
CREATE TRIGGER no_faculty_for_secondary_cross_listing_update
AFTER UPDATE ON faculty_sections
WHEN (
    SELECT COUNT(1)
    FROM faculty_sections
    NATURAL JOIN cross_listing_sections
) > 0
BEGIN
    SELECT RAISE(ABORT, 'secondary section in cross listing cannot be assigned to faculty');
END;


--
--
-- END OF INPUT DATA
--
-- Everything that follows is processed from the data above
--
-- The views below are the semantic boundary between normalized input rows and
-- solver-ready input. Raw tables keep provenance and closed-world facts:
-- catalog ownership, faculty affiliation, section tags, faculty availability,
-- preferences, cross-listings, and curriculum rules. Views deliberately change
-- shape into the vocabulary the solver consumes:
--   * sections are canonicalized through cross-listing primary sections;
--   * room/time tags become concrete rooms and concrete time slots;
--   * faculty availability and preferences are intersected with section tags;
--   * curriculum rules become section-pair penalties.
--
-- The column name "department" in solver-facing views means the department that
-- owns the course/section being scheduled. It is the correct filter for Rust
-- input loading. Faculty affiliation remains separate raw input because a
-- faculty member can teach sections owned by another department.
--

-- Solver input keeps the old time-pattern shape. The raw tables keep faculty
-- provenance, and the derived name prevents groups from different faculty from
-- accidentally collapsing together.
CREATE VIEW time_pattern_matches (time_pattern_match_name, time_pattern_match_priority) AS
    SELECT CAST(LENGTH(faculty) AS TEXT) || ':' || faculty || time_pattern_match_name,
           time_pattern_match_priority
    FROM faculty_time_pattern_matches;

CREATE VIEW time_pattern_match_sections (time_pattern_match_name, time_pattern_match_section) AS
    SELECT CAST(LENGTH(faculty) AS TEXT) || ':' || faculty || time_pattern_match_name,
           time_pattern_match_section
    FROM faculty_time_pattern_match_sections;

-- Faculty-owned preferences expanded to the concrete solver options they can
-- penalize. These views retain provenance so independent preferences from
-- instructors sharing a section do not collapse into one minimum priority.
CREATE VIEW faculty_time_preferences_to_be_scheduled
        (faculty, department, section, time_slot, preference_priority) AS
    SELECT pref.faculty, scheduled.department, scheduled.section,
           pref.time_slot, pref.time_slot_priority
    FROM faculty_time_slot_preferences AS pref
    JOIN faculty_sections_to_be_scheduled AS scheduled
        ON scheduled.faculty = pref.faculty
    JOIN time_slots_available_to_sections AS available
        ON available.section = scheduled.section
        AND available.time_slot = pref.time_slot

    UNION

    SELECT pref.faculty, scheduled.department, scheduled.section,
           concrete.time_slot, pref.time_slot_priority
    FROM faculty_section_time_slot_preferences AS pref
    JOIN faculty_sections_to_be_scheduled AS scheduled
        ON scheduled.faculty = pref.faculty
        AND scheduled.section = pref.section
    JOIN time_slots_time_slot_tags AS concrete
        ON concrete.time_slot_tag = pref.time_slot_tag
    JOIN time_slots_available_to_sections AS available
        ON available.section = scheduled.section
        AND available.time_slot = concrete.time_slot;

CREATE VIEW faculty_room_preferences_to_be_scheduled
        (faculty, department, section, room, preference_priority) AS
    SELECT pref.faculty, scheduled.department, scheduled.section,
           concrete.room, pref.room_priority
    FROM faculty_section_room_preferences AS pref
    JOIN faculty_sections_to_be_scheduled AS scheduled
        ON scheduled.faculty = pref.faculty
        AND scheduled.section = pref.section
    JOIN rooms_room_tags AS concrete
        ON concrete.room_tag = pref.room_tag
    JOIN rooms_available_to_sections AS available
        ON available.section = scheduled.section
        AND available.room = concrete.room;

CREATE VIEW faculty_time_pattern_preferences_to_be_scheduled
        (faculty, department, preference_name, preference_priority, section) AS
    SELECT pattern.faculty, scheduled.department,
           pattern.time_pattern_match_name, pattern.time_pattern_match_priority,
           member.time_pattern_match_section
    FROM faculty_time_pattern_matches AS pattern
    JOIN faculty_time_pattern_match_sections AS member
        ON member.faculty = pattern.faculty
        AND member.time_pattern_match_name = pattern.time_pattern_match_name
    JOIN sections_to_be_scheduled AS scheduled
        ON scheduled.section = member.time_pattern_match_section;


-- Every pair of time slots that overlap on at least one day and overlap in
-- clock time. The view includes both directions and self-overlaps. The solver
-- wants a symmetric conflict matrix, and a time slot conflicts with itself for
-- room-use and section-pair criteria.
CREATE VIEW conflicting_time_slots (time_slot_a, time_slot_b) AS
    SELECT a.time_slot, b.time_slot
    FROM time_slots AS a
    JOIN time_slots AS b
    WHERE CASE
        WHEN INSTR(a.time_slot, 'M') > 0 AND INSTR(b.time_slot, 'M') > 0 THEN 1
        WHEN INSTR(a.time_slot, 'T') > 0 AND INSTR(b.time_slot, 'T') > 0 THEN 1
        WHEN INSTR(a.time_slot, 'W') > 0 AND INSTR(b.time_slot, 'W') > 0 THEN 1
        WHEN INSTR(a.time_slot, 'R') > 0 AND INSTR(b.time_slot, 'R') > 0 THEN 1
        WHEN INSTR(a.time_slot, 'F') > 0 AND INSTR(b.time_slot, 'F') > 0 THEN 1
        WHEN INSTR(a.time_slot, 'S') > 0 AND INSTR(b.time_slot, 'S') > 0 THEN 1
        WHEN INSTR(a.time_slot, 'U') > 0 AND INSTR(b.time_slot, 'U') > 0 THEN 1
        ELSE 0 END = 1
    AND CASE
        WHEN a.start_time + a.duration <= b.start_time THEN 0
        WHEN a.start_time >= b.start_time + b.duration THEN 0
        ELSE 1 END = 1;

-- Schedulable sections are sections with at least one time-slot tag after input
-- loading. Sections with no time-slot tags remain in the raw section table for
-- catalog/audit purposes, but they are invisible to the solver.
--
-- Cross-listings are canonicalized here. The solver schedules one section, the
-- primary section, but curriculum rules and faculty assignments may reference a
-- secondary listing. In this view:
--   * department and course come from the original listed section;
--   * section is the primary section that receives room/time assignments;
--   * secondary_section is the original section name used by raw rules.
--
-- This is why later views join anti-conflicts and conflict rules through
-- secondary_section, but room/time availability through section.
CREATE VIEW sections_to_be_scheduled (department, course, section, secondary_section) AS
    -- Secondary cross-listed sections become schedulable only when the primary
    -- section has time-slot tags. Secondary rows cannot carry their own room,
    -- time, or faculty rows; triggers above enforce that raw-data invariant.
    WITH schedulable_cross_listings (department, section, primary_section) AS (
        SELECT DISTINCT department, cross_listing_sections.section, primary_section
        FROM courses
        NATURAL JOIN sections
        NATURAL JOIN cross_listing_sections
        JOIN section_time_slot_tags
            ON section_time_slot_tags.section = cross_listing_sections.primary_section
    )

    SELECT department, course, section, section
    FROM courses
    NATURAL JOIN sections
    NATURAL JOIN section_time_slot_tags

    UNION

    SELECT department, course, schedulable_cross_listings.primary_section, schedulable_cross_listings.section
    FROM courses
    NATURAL JOIN sections
    NATURAL JOIN schedulable_cross_listings
    JOIN section_time_slot_tags
        ON section_time_slot_tags.section = schedulable_cross_listings.primary_section;

-- The time slots that a section can be assigned to and the associated soft
-- priority. Section tags define the allowed set. Faculty base availability,
-- hard unavailable slots, general time preferences, and section-specific time
-- preferences are merged here into the solver-facing shape.
--
-- A row exists only when the concrete time slot is allowed by the section and
-- all assigned faculty can teach it. If a section has no assigned faculty, its
-- section tags alone determine available time slots.
--
-- Soft priorities are penalties. NULL means no penalty. When multiple
-- faculty or preference sources apply to the same concrete time slot, MIN()
-- keeps the strongest penalty because lower priority numbers are more
-- important to the solver.
CREATE VIEW time_slots_available_to_sections (department, section, time_slot, time_slot_priority) AS
    -- Expand section time-slot tags to concrete time slots. A tag that is
    -- identical to a concrete time_slot is an explicit assignment. Explicit
    -- assignments can bypass base faculty availability for that section only,
    -- but they do not bypass hard unavailability.
    WITH section_time_slots (department, section, time_slot, explicitly_assigned) AS (
        SELECT  department,
                section,
                time_slot,
                MAX(CASE WHEN time_slot_tag = time_slot THEN 1 ELSE 0 END)
        FROM sections_to_be_scheduled
        NATURAL JOIN section_time_slot_tags
        NATURAL JOIN time_slots_time_slot_tags
        GROUP BY department, section, time_slot
    ),

    -- Expand faculty-authored section-specific time preferences to concrete
    -- time slots. These preferences never add allowed times; the later join
    -- applies them only to the section tag intersection.
    section_time_preferences (faculty, section, time_slot, section_time_slot_priority) AS (
        SELECT  faculty,
                section,
                time_slot,
                MIN(time_slot_priority)
        FROM faculty_section_time_slot_preferences
        NATURAL JOIN time_slots_time_slot_tags
        GROUP BY faculty, section, time_slot
    ),

    -- For each assigned faculty/section/time, gather the two independent
    -- sources of time penalties:
    --   * faculty_time_slot_priority from general faculty time preferences;
    --   * section_time_slot_priority from section-specific preferences.
    --
    -- time_slots_available_to_faculty excludes hard unavailable slots for base
    -- availability. blocked is checked again here because explicit section
    -- assignments can skip the base-availability row.
    per_faculty_inputs (department, section, faculty, time_slot, faculty_time_slot_priority, section_time_slot_priority) AS (
        SELECT  st.department,
                st.section,
                fs.faculty,
                st.time_slot,
                COALESCE(ft.faculty_time_slot_priority, direct_pref.time_slot_priority),
                pref.section_time_slot_priority
        FROM section_time_slots AS st
        NATURAL JOIN faculty_sections AS fs
        LEFT OUTER JOIN time_slots_available_to_faculty AS ft
            ON  ft.faculty = fs.faculty
            AND ft.time_slot = st.time_slot
        LEFT OUTER JOIN faculty_unavailable_time_slots AS blocked
            ON  blocked.faculty = fs.faculty
            AND blocked.time_slot = st.time_slot
        -- General time preferences still apply when an explicit section time
        -- bypasses base faculty availability and therefore has no ft row.
        LEFT OUTER JOIN faculty_time_slot_preferences AS direct_pref
            ON  direct_pref.faculty = fs.faculty
            AND direct_pref.time_slot = st.time_slot
        LEFT OUTER JOIN section_time_preferences AS pref
            ON  pref.faculty = fs.faculty
            AND pref.section = st.section
            AND pref.time_slot = st.time_slot
        -- Explicit section times make the assigned faculty available for that
        -- section/time without mutating base availability. Hard unavailable
        -- slots still win and remove the option entirely.
        WHERE blocked.faculty IS NULL
        AND (ft.faculty IS NOT NULL OR st.explicitly_assigned = 1)
    ),

    -- Collapse the two penalty sources for one faculty member. NULL has
    -- identity behavior: if only one source penalizes the time, keep it; if
    -- both do, keep the stronger penalty.
    per_faculty (department, section, faculty, time_slot, time_slot_priority) AS (
        SELECT  department,
                section,
                faculty,
                time_slot,
                CASE
                    WHEN faculty_time_slot_priority IS NULL THEN section_time_slot_priority
                    WHEN section_time_slot_priority IS NULL THEN faculty_time_slot_priority
                    WHEN faculty_time_slot_priority < section_time_slot_priority THEN faculty_time_slot_priority
                    ELSE section_time_slot_priority
                END
        FROM per_faculty_inputs
    ),

    -- A section with multiple faculty can use a time only if every assigned
    -- faculty member has a row in per_faculty. This CTE counts the faculty who
    -- survived availability and hard-unavailability filtering.
    group_faculty (department, section, time_slot, time_slot_priority, faculty_assigned) AS (
        SELECT department, section, time_slot, MIN(time_slot_priority), COUNT(faculty)
        FROM per_faculty
        GROUP BY department, section, time_slot
    ),

    -- Total assigned faculty comes from raw faculty_sections, not from
    -- availability-derived rows. Comparing this count with group_faculty is
    -- the intersection step across all instructors assigned to the section.
    faculty_count (section, total_faculty_assigned) AS (
        SELECT section, COUNT(1)
        FROM faculty_sections
        GROUP BY section
    ),

    -- Keep only times where every assigned faculty member is available. This
    -- is the semantic center of the view: section allowed-set intersection plus
    -- faculty availability intersection.
    intersect_faculty (department, section, time_slot, time_slot_priority) AS (
        SELECT department, section, time_slot, time_slot_priority
        FROM group_faculty
        NATURAL JOIN faculty_count
        WHERE faculty_assigned = total_faculty_assigned
    )

    -- time slots where all faculty are available
    SELECT department, section, time_slot, time_slot_priority
    FROM intersect_faculty

    UNION

    -- time slots for section with no faculty assigned
    SELECT department, section, time_slot, NULL
    FROM section_time_slots
    NATURAL LEFT OUTER JOIN faculty_sections
    WHERE faculty IS NULL;

-- The rooms that a section can be assigned to and the associated soft
-- priority. Room tags define the allowed set; faculty-section preferences
-- apply only to the intersection between the preference tag and allowed rooms.
--
-- Rooms differ from time slots because faculty availability does not constrain
-- them. The view expands section room tags to concrete rooms and overlays
-- faculty-authored room penalties. NULL room_priority means no room penalty.
CREATE VIEW rooms_available_to_sections (department, section, room, room_priority) AS
    -- The concrete room allowed set for each schedulable section.
    WITH section_rooms (department, section, room) AS (
        SELECT DISTINCT department, section, room
        FROM sections_to_be_scheduled
        NATURAL JOIN section_room_tags
        NATURAL JOIN rooms_room_tags
    ),

    -- Section-specific room preferences expanded to concrete rooms. These
    -- preferences do not grant access to rooms; they only penalize rooms that
    -- already appear in section_rooms.
    section_room_preferences (faculty, section, room, room_priority) AS (
        SELECT  faculty,
                section,
                room,
                MIN(room_priority)
        FROM faculty_section_room_preferences
        NATURAL JOIN rooms_room_tags
        GROUP BY faculty, section, room
    )

    SELECT department, section, room, MIN(room_priority)
    FROM section_rooms
    NATURAL LEFT OUTER JOIN section_room_preferences
    GROUP BY department, section, room;

-- Rooms that a course-owning department uses in solver input. This is a
-- department-scoped projection of the raw allowed room tags before faculty
-- room preferences matter; it exists so Rust loads only rooms that can appear
-- in the selected departments' sections.
CREATE VIEW rooms_used_by_departments (department, room, building, room_number, capacity) AS
    SELECT DISTINCT department, room, building, room_number, capacity
    FROM sections_to_be_scheduled
    NATURAL JOIN section_room_tags
    NATURAL JOIN rooms_room_tags
    NATURAL JOIN rooms;
    
-- Time slots that a course-owning department uses in solver input. Unlike
-- rooms_used_by_departments, this uses time_slots_available_to_sections, so the
-- set is already filtered by faculty availability and hard unavailable slots.
-- Rust therefore loads only time slots that can actually be assigned to at
-- least one selected section.
CREATE VIEW time_slots_used_by_departments (department, time_slot, days, start_time, duration, first_day) AS
    SELECT DISTINCT department, time_slot, days, start_time, duration, first_day
    FROM time_slots_available_to_sections
    NATURAL JOIN time_slots;

-- General faculty availability by time slot. This view does not include the
-- explicit-section-time exception because that exception depends on which
-- section the faculty is teaching; time_slots_available_to_sections applies it.
--
-- Raw faculty_availability stores day/time intervals. This view converts them
-- to concrete time_slot rows only when the intervals cover every meeting minute
-- of the time slot across all days in that time_slot. Partial coverage is not
-- enough. General faculty time preferences become penalties here; hard
-- unavailable concrete slots remove rows.
CREATE VIEW time_slots_available_to_faculty (faculty, time_slot, faculty_time_slot_priority) AS
    -- Compute how many meeting minutes of each concrete time_slot are covered
    -- by each overlapping availability interval.
    WITH overlapping_intervals (faculty, faculty_minutes, time_slot, time_slot_minutes) AS (
        SELECT  faculty,
                CASE
                    -- time slot is entirely inside availability
                    WHEN fa.start_time <= ts.start_time AND fa.start_time + fa.duration >=
                         ts.start_time + ts.duration
                        THEN ts.duration
                    -- availability is entirely inside time slot
                    WHEN fa.start_time >= ts.start_time AND fa.start_time + fa.duration <=
                         ts.start_time + ts.duration
                        THEN fa.duration
                    -- availability starts first
                    WHEN fa.start_time <= ts.start_time
                        THEN fa.start_time + fa.duration - ts.start_time
                    -- time slot starts first
                    ELSE
                        ts.start_time + ts.duration - fa.start_time
                    END,
                time_slot,
                LENGTH(ts.days) * ts.duration AS time_slot_minutes
        FROM faculty_availability AS fa
        JOIN time_slots AS ts
                 -- skip if availability is after time_slot
        ON  CASE WHEN fa.start_time >= ts.start_time + ts.duration THEN 0
                 -- skip if time slot is after availability
                 WHEN fa.start_time + fa.duration <= ts.start_time THEN 0
                 -- skip if availability is on wrong day
                 WHEN INSTR(ts.days, fa.day_of_week) = 0 THEN 0
                 -- else we have some overlap
                 ELSE 1 END = 1
    ),

    -- A faculty/time_slot pair is base-available only when all meeting minutes
    -- in the time slot are covered by availability intervals. GROUP BY
    -- includes time_slot_minutes so the HAVING condition compares one slot's
    -- total covered minutes to that slot's required minutes.
    base_available (faculty, time_slot) AS (
        SELECT faculty, time_slot
        FROM overlapping_intervals
        GROUP BY faculty, time_slot, time_slot_minutes
        HAVING SUM(faculty_minutes) = time_slot_minutes
    )

    SELECT base.faculty, base.time_slot, MIN(pref.time_slot_priority)
    FROM base_available AS base
    LEFT OUTER JOIN faculty_unavailable_time_slots AS blocked
        ON  blocked.faculty = base.faculty
        AND blocked.time_slot = base.time_slot
    LEFT OUTER JOIN faculty_time_slot_preferences AS pref
        ON  pref.faculty = base.faculty
        AND pref.time_slot = base.time_slot
    -- UnavailableTimeSlot is a hard exclusion, not a priority-0 penalty.
    WHERE blocked.faculty IS NULL
    GROUP BY base.faculty, base.time_slot;

-- Faculty assigned to schedulable sections. department is the course-owning
-- department and is intentionally not faculty.affiliation. Rust filters this
-- view by course department, then pulls in all faculty whose assigned sections
-- intersect those departments.
CREATE VIEW faculty_sections_to_be_scheduled (faculty, department, course, section) AS
    SELECT faculty, department, course, section
    FROM sections_to_be_scheduled
    NATURAL JOIN faculty_sections;

-- Faculty preference rows scoped to course-owning departments that have
-- schedulable sections for the faculty member. A single faculty preference can
-- appear once per course department taught by that faculty member; this is
-- intentional because Rust loads preferences under the same department filter
-- used for sections. DISTINCT removes duplicate rows within one department
-- when a faculty member teaches multiple sections there.
CREATE VIEW faculty_to_be_scheduled_preference_intervals (faculty, department,
        days_to_check, days_off, days_off_priority, evenly_spread_priority,
        no_room_switch_priority, too_many_rooms_priority, max_gap_within_cluster,
        is_cluster, is_too_short, interval_minutes, interval_priority) AS
    SELECT DISTINCT faculty, department,
                    days_to_check, days_off, days_off_priority, evenly_spread_priority,
                    no_room_switch_priority, too_many_rooms_priority, max_gap_within_cluster,
                    is_cluster, is_too_short, interval_minutes, interval_priority
    FROM faculty_sections_to_be_scheduled
    NATURAL JOIN faculty_preferences
    NATURAL LEFT OUTER JOIN faculty_preference_intervals;

-- Raw conflict input is program-centric: a conflict group can name courses,
-- sections, or a mix of both, and programs can either boost or reduce conflict
-- priority. This view expands those program rules into concrete schedulable
-- section pairs, canonicalizes cross-listed sections to their primary section,
-- and merges duplicate rules across programs.
--
-- This view intentionally stops before prereq/coreq removal and multiple-
-- section discounting. conflict_pairs applies those final solver-facing
-- adjustments.
CREATE VIEW undiscounted_conflict_pairs (department_a, course_a, section_a, department_b, course_b, section_b, priority) AS
    -- Expand course-vs-course conflict cliques to section pairs. Two sections
    -- of the same course are not paired here; multiple-section discounting is
    -- handled later and same-course spreads should not conflict by default.
    WITH paired_conflict_courses_courses AS (
        SELECT conflicts.program, conflicts.conflict_name, conflict_priority, boost_priority,
            COALESCE(x1.primary_section, s1.section) AS section_a,
            COALESCE(x2.primary_section, s2.section) AS section_b
        FROM conflicts
        JOIN conflict_courses c1
            ON  c1.program                                      =  conflicts.program
            AND c1.conflict_name                                =  conflicts.conflict_name
        JOIN sections s1
            ON  s1.course                                       =  c1.course
        LEFT OUTER JOIN cross_listing_sections x1
            ON  x1.section                                      =  s1.section
        JOIN conflict_courses c2
            ON  c2.program                                      =  conflicts.program
            AND c2.conflict_name                                =  conflicts.conflict_name
        JOIN sections s2
            ON  s2.course                                       =  c2.course
        LEFT OUTER JOIN cross_listing_sections x2
            ON  x2.section                                      =  s2.section
        WHERE   c2.course                                       <> c1.course
    ),

    -- Expand section-vs-section conflict cliques. Raw section names may be
    -- secondary cross-listing names, so COALESCE maps each side to the primary
    -- section that the solver actually schedules.
    paired_conflict_sections_sections AS (
        SELECT conflicts.program, conflicts.conflict_name, conflict_priority, boost_priority,
            COALESCE(x1.primary_section, s1.section) AS section_a,
            COALESCE(x2.primary_section, s2.section) AS section_b
        FROM conflicts
        JOIN conflict_sections s1
            ON  s1.program                                      =  conflicts.program
            AND s1.conflict_name                                =  conflicts.conflict_name
        LEFT OUTER JOIN cross_listing_sections x1
            ON  x1.section                                      =  s1.section
        JOIN conflict_sections s2
            ON  s2.program                                      =  conflicts.program
            AND s2.conflict_name                                =  conflicts.conflict_name
        LEFT OUTER JOIN cross_listing_sections x2
            ON  x2.section                                      =  s2.section
        WHERE   s2.section                                      <> s1.section
    ),

    -- Expand section-vs-course rules. The explicit role names matter here:
    -- s1 is a raw section named by a conflict rule; c2/s2 enumerate sections of
    -- a raw course named by the same rule.
    paired_conflict_sections_courses AS (
        SELECT conflicts.program, conflicts.conflict_name, conflict_priority, boost_priority,
            COALESCE(x1.primary_section, s1.section) AS section_a,
            COALESCE(x2.primary_section, s2.section) AS section_b
        FROM conflicts
        JOIN conflict_sections s1
            ON  s1.program                                      =  conflicts.program
            AND s1.conflict_name                                =  conflicts.conflict_name
        LEFT OUTER JOIN cross_listing_sections x1
            ON  x1.section                                      =  s1.section
        JOIN conflict_courses c2
            ON  c2.program                                      =  conflicts.program
            AND c2.conflict_name                                =  conflicts.conflict_name
        JOIN sections s2
            ON  s2.course                                       =  c2.course
        LEFT OUTER JOIN cross_listing_sections x2
            ON  x2.section                                      =  s2.section
        WHERE   s2.section                                      <> s1.section
    ),

    -- Expand course-vs-section rules, the mirror of the previous CTE. Keeping
    -- both directions preserves the ordered pair shape consumed downstream.
    paired_conflict_courses_sections AS (
        SELECT conflicts.program, conflicts.conflict_name, conflict_priority, boost_priority,
            COALESCE(x1.primary_section, s1.section) AS section_a,
            COALESCE(x2.primary_section, s2.section) AS section_b
        FROM conflicts
        JOIN conflict_courses c1
            ON  c1.program                                      =  conflicts.program
            AND c1.conflict_name                                =  conflicts.conflict_name
        JOIN sections s1
            ON  s1.course                                       =  c1.course
        LEFT OUTER JOIN cross_listing_sections x1
            ON  x1.section                                      =  s1.section
        JOIN conflict_sections s2
            ON  s2.program                                      =  conflicts.program
            AND s2.conflict_name                                =  conflicts.conflict_name
        LEFT OUTER JOIN cross_listing_sections x2
            ON  x2.section                                      =  s2.section
        WHERE   s2.section                                      <> s1.section
    ),

    -- Merge all section conflicts derived from sections or courses. UNION
    -- removes duplicate raw expansions before priority reduction.
    paired_conflicts AS (
        SELECT * FROM paired_conflict_courses_courses
        UNION
        SELECT * FROM paired_conflict_sections_sections
        UNION
        SELECT * FROM paired_conflict_sections_courses
        UNION
        SELECT * FROM paired_conflict_courses_sections
    ),

    -- Combine conflicts within one program for one ordered section pair.
    -- boost_priority rows try to make a conflict stronger (lower number);
    -- non-boost rows try to make it weaker or cancel it. The NULL-as-10
    -- sentinel lets MAX notice an explicit cancellation request.
    per_program_conflicts AS (
        SELECT program, section_a, section_b,
            -- highest priority is lowest number
            MIN(conflict_priority) FILTER (WHERE boost_priority) AS highest_priority,
            -- lowest priority is highest number, but NULL means the conflict should be canceled
            -- so we change NULL to 10 (normal range is [0,9]) so MAX will not ignore it
            MAX(CASE WHEN conflict_priority IS NULL THEN 10 ELSE conflict_priority END) FILTER (WHERE NOT boost_priority) AS lowest_priority
        FROM paired_conflicts
        GROUP BY program, section_a, section_b
    ),

    -- Apply reduce/cancel rows to produce one priority per program/section
    -- pair. Priority 0 is hard and cannot be weakened.
    reduced_conflicts AS (
        SELECT program, section_a, section_b,
            CASE
                -- never reduce a hard conflict
                WHEN highest_priority = 0 then 0
                -- lowest_priority = 10 => the conflict should be canceled
                WHEN lowest_priority = 10 then NULL
                -- lowest_priority wins when both are set, but only if it actually lowers the priority (increases number)
                WHEN highest_priority IS NOT NULL AND lowest_priority IS NOT NULL THEN MAX(highest_priority, lowest_priority)
                -- absence of lowest_priority means just use highest_priority
                WHEN highest_priority IS NOT NULL THEN highest_priority
                -- if there is no highest_priority (no boost_priority entries) then no priority, i.e.,
                -- lowest_priority should never introduce a priority
                ELSE NULL
            END AS priority
        FROM per_program_conflicts
        WHERE priority IS NOT NULL
    )

    -- Merge conflicts across programs. department_a and department_b are
    -- course-owning departments for the scheduled sections, not program
    -- departments. If multiple programs produce the same ordered pair, the
    -- strongest remaining priority wins.
    SELECT  as_a.department, as_a.course, section_a,
            as_b.department, as_b.course, section_b, MIN(priority)
    FROM reduced_conflicts
    JOIN sections_to_be_scheduled AS as_a
        ON section_a = as_a.section
    JOIN sections_to_be_scheduled AS as_b
        ON section_b = as_b.section
    GROUP BY as_a.department, section_a, as_b.department, section_b;

-- Courses that should not be conflict-penalized because one course is a
-- prerequisite path for the other. We treat prereqs and coreqs as edges while
-- walking the graph so prereqs of coreqs and coreqs of prereqs count. Immediate
-- coreqs are removed at the end because coreqs are meant to be taken together
-- and should still be eligible for ordinary conflict rules.
--
-- The output is directional: course depends on prereq. conflict_pairs probes
-- both directions with two left joins.
CREATE VIEW prereq_transitive_closure (course, prereq) AS
    -- treat coreqs and prereqs as the same...
    WITH merged_pre_and_co AS (
        SELECT course, prereq FROM prereqs
        UNION
        SELECT course, coreq AS prereq FROM coreqs
    ),

    -- ... to build the transitive closure of prereqs ...
    recursive_prereqs (course, prereq) AS (
        SELECT course, prereq
        FROM merged_pre_and_co

        UNION

        SELECT r.course, p.prereq
        FROM recursive_prereqs AS r
        JOIN merged_pre_and_co AS p
            ON r.prereq = p.course
    )

    -- ... then remove actual coreqs
    SELECT recursive_prereqs.course, recursive_prereqs.prereq
    FROM recursive_prereqs
    LEFT OUTER JOIN coreqs
        ON  recursive_prereqs.course = coreqs.course
        AND recursive_prereqs.prereq = coreqs.coreq
    WHERE coreqs.coreq IS NULL;

-- Courses with multiple ways for a student to take the course. This includes
-- unscheduled/online sections on purpose: even if the solver is not placing an
-- online section, its existence can reduce the cost of a conflict because
-- students have an alternative.
--
-- multiple_section_overrides corrects special cases where the raw section count
-- is not the right student-choice count, such as cross-listing or anti-conflict
-- modeling. Only courses with final_count > 1 matter for discounting.
CREATE VIEW section_counts (department, course, section_count) AS
    -- get raw section counts including online sections
    WITH all_sections AS (
        SELECT department, course, COUNT(section) AS section_count
        FROM courses
        NATURAL JOIN sections
        GROUP BY department, course
    ),
    with_overrides AS (
        SELECT department, all_sections.course AS course,
            -- use the override if present, but otherwise the all_sections count
            IIF(multiple_section_overrides.override_section_count IS NULL,
                all_sections.section_count,
                multiple_section_overrides.override_section_count) AS final_count
        FROM all_sections
        LEFT OUTER JOIN multiple_section_overrides
            ON all_sections.course = multiple_section_overrides.course
    )
    SELECT DISTINCT department, course, final_count
    FROM with_overrides
    NATURAL JOIN sections_to_be_scheduled
    WHERE final_count > 1;

-- Fully processed section-to-section conflict criteria for the solver. This
-- view starts from expanded curriculum conflicts, removes pairs connected by a
-- prereq/coreq path, discounts conflicts when either course has multiple
-- student choices, and adds hard conflicts for sections taught by the same
-- faculty member.
--
-- Conflict priority range is [0,9]. Priority 0 is hard. Multiple-section
-- discounting adds 5 per extra section and drops the pair when the result
-- reaches 10. Same-course spreads are not discounted because they represent
-- alternative offerings of the same course rather than two independent courses.
CREATE VIEW conflict_pairs (department_a, section_a, department_b, section_b, priority) AS
    -- Remove conflicts when there is a prereq relationship and discount
    -- multiple sections. This branch produces soft and hard curriculum
    -- conflicts after final adjustment.
    WITH merged (department_a, section_a, department_b, section_b, priority) AS (
        SELECT department_a, section_a, department_b, section_b,
            CASE WHEN undiscounted.priority = 0
                    -- hard conflicts are never reduced
                    THEN undiscounted.priority
                 WHEN counts_a.section_count IS NOT NULL AND counts_b.section_count IS NOT NULL AND counts_a.course = counts_b.course
                    -- no discount for spreads
                    THEN undiscounted.priority
                 WHEN counts_a.section_count IS NOT NULL AND counts_b.section_count IS NOT NULL
                    THEN undiscounted.priority + 5 * ((counts_a.section_count-1) + (counts_b.section_count-1))
                 WHEN counts_a.section_count IS NOT NULL
                    THEN undiscounted.priority + 5 *  (counts_a.section_count-1)
                 WHEN counts_b.section_count IS NOT NULL
                    THEN undiscounted.priority + 5 *  (counts_b.section_count-1)
                 ELSE
                    undiscounted.priority
            END AS discounted_priority
        FROM undiscounted_conflict_pairs AS undiscounted
        -- Two left joins are intentionally used instead of one OR join. The
        -- prereq relation is directional, and this shape lets SQLite use the
        -- simple equality predicates on both probes.
        LEFT OUTER JOIN prereq_transitive_closure AS pre_1
            ON  undiscounted.course_a                           = pre_1.course
            AND undiscounted.course_b                           = pre_1.prereq
        LEFT OUTER JOIN prereq_transitive_closure AS pre_2
            ON  undiscounted.course_a                           = pre_2.prereq
            AND undiscounted.course_b                           = pre_2.course
        LEFT OUTER JOIN section_counts AS counts_a
            ON  counts_a.course                                 = undiscounted.course_a
        LEFT OUTER JOIN section_counts AS counts_b
            ON  counts_b.course                                 = undiscounted.course_b
        WHERE pre_1.course IS NULL AND pre_1.prereq IS NULL AND pre_2.course IS NULL AND pre_2.prereq IS NULL
        AND discounted_priority < 10

        UNION

        -- Sections taught by the same instructor are hard conflicts, even if no
        -- curriculum conflict group mentions them.
        SELECT  sec_a.department, sec_a.section, sec_b.department, sec_b.section, 0
        FROM faculty_sections_to_be_scheduled AS sec_a
        JOIN faculty_sections_to_be_scheduled AS sec_b
            ON  sec_a.faculty                                   =  sec_b.faculty
            AND sec_a.section                                   <> sec_b.section
    )

    SELECT department_a, section_a, department_b, section_b, MIN(priority)
    FROM merged
    GROUP BY department_a, section_a, department_b, section_b;

-- Anti-conflicts are the opposite of ordinary conflicts: the single section
-- should be scheduled at the same concrete time as at least one section in the
-- group. Raw rules are expressed against original section/course names, so the
-- joins use secondary_section to preserve cross-listing semantics while
-- returning primary scheduled sections to Rust.
CREATE VIEW anti_conflict_pairs (single_department, single_section, group_department, group_section, priority) AS
    SELECT  single_sections.department AS single_department, single_sections.section AS single_section,
            group_sections.department AS group_department, group_sections.section AS group_section,
            anti_conflict_priority
    FROM sections_to_be_scheduled                           AS single_sections
    JOIN anti_conflicts
        ON  anti_conflicts.anti_conflict_single             = single_sections.secondary_section
    JOIN anti_conflict_sections
        ON  anti_conflict_sections.anti_conflict_single     = anti_conflicts.anti_conflict_single
    JOIN sections_to_be_scheduled                           AS group_sections
        ON  group_sections.secondary_section                = anti_conflict_sections.anti_conflict_section

    UNION

    SELECT  single_sections.department AS single_department, single_sections.section AS single_section,
            group_sections.department AS group_department, group_sections.section AS group_section,
            anti_conflict_priority
    FROM sections_to_be_scheduled                           AS single_sections
    JOIN anti_conflicts
        ON  anti_conflicts.anti_conflict_single             = single_sections.secondary_section
    JOIN anti_conflict_courses
        ON  anti_conflict_courses.anti_conflict_single      = anti_conflicts.anti_conflict_single
    JOIN sections_to_be_scheduled                           AS group_sections
        ON  group_sections.course                           = anti_conflict_courses.anti_conflict_course;

COMMIT;
