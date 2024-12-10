PRAGMA encoding = 'UTF-8';


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

CREATE TABLE rooms (
    room                        TEXT PRIMARY KEY,
    building                    TEXT GENERATED ALWAYS AS (substr(room, 1, instr(room, ' ') - 1)) VIRTUAL NOT NULL,
    room_number                 TEXT GENERATED ALWAYS AS (substr(room, length(building) + 2)) VIRTUAL NOT NULL,
    capacity                    INTEGER NOT NULL,

    CHECK (length(room_number) > 0),

    FOREIGN KEY (building) REFERENCES buildings (building) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE room_tags (
    room_tag                    TEXT PRIMARY KEY
) WITHOUT ROWID;

CREATE TABLE rooms_room_tags (
    room_tag                    TEXT NOT NULL,
    room                        TEXT NOT NULL,

    PRIMARY KEY (room_tag, room),
    FOREIGN KEY (room_tag) REFERENCES room_tags (room_tag) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (room) REFERENCES rooms (room) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE time_slots (
    time_slot                   TEXT PRIMARY KEY,
    days                        TEXT GENERATED ALWAYS AS (substr(time_slot, 1, length(time_slot) - length(duration) - 5)) VIRTUAL NOT NULL,
    start_time                  INTEGER GENERATED ALWAYS AS (CAST(substr(time_slot, -(length(duration) + 5), 2) AS INTEGER) * 60 + CAST(substr(time_slot, -(length(duration) + 3), 2) AS INTEGER)) VIRTUAL NOT NULL,
    duration                    INTEGER GENERATED ALWAYS AS (CAST(substr(time_slot, instr(time_slot, '+')) AS INTEGER)) VIRTUAL NOT NULL,
    first_day                   INTEGER GENERATED ALWAYS AS (CAST(
        replace(replace(replace(replace(replace(replace(replace(
            substr(days, 1, 1), 'M','1'), 'T','2'), 'W','3'), 'R','4'), 'F','5'), 'S','6'), 'U','7') AS INTEGER)) VIRTUAL NOT NULL,

    CHECK (start_time >= 0 AND start_time % 5 = 0),
    CHECK (duration > 0 AND duration % 5 = 0),
    CHECK (start_time + duration < 24*60),
    CHECK (length(days) > 0 AND instr(days, '$') = 0 AND
        replace(replace(replace(replace(replace(replace(replace('$'||days,
            '$M','$'), '$T','$'), '$W','$'), '$R','$'), '$F','$'), '$S','$'), '$U','$') = '$'),
    CHECK (days || substr('00'||CAST(start_time / 60 AS TEXT), -2) || substr('00'||CAST(start_time % 60 AS TEXT), -2) || '+' || CAST(duration AS TEXT) = time_slot)
) WITHOUT ROWID;

CREATE TABLE time_slot_tags (
    time_slot_tag               TEXT PRIMARY KEY
) WITHOUT ROWID;

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

CREATE TABLE faculty (
    faculty                     TEXT PRIMARY KEY,
    department                  TEXT NOT NULL,

    FOREIGN KEY (department) REFERENCES departments (department) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE faculty_availability (
    faculty                     TEXT NOT NULL,
    day_of_week                 TEXT NOT NULL,
    start_time                  INTEGER NOT NULL,
    duration                    INTEGER NOT NULL,
    availability_penalty        INTEGER NOT NULL,

    CHECK (day_of_week IN ('M', 'T', 'W', 'R', 'F', 'S', 'U')),
    CHECK (start_time >= 0 AND start_time % 5 = 0),
    CHECK (duration > 0 AND duration % 5 = 0),
    CHECK (start_time + duration < 24*60),
    CHECK (availability_penalty >= 0 AND availability_penalty < 100),

    PRIMARY KEY (faculty, day_of_week, start_time),
    FOREIGN KEY (faculty) REFERENCES faculty (faculty) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE faculty_preferences (
    faculty                     TEXT PRIMARY KEY,
    days_to_check               TEXT NOT NULL,
    days_off                    INTEGER NOT NULL,
    days_off_penalty            INTEGER NOT NULL,
    evenly_spread_penalty       INTEGER NOT NULL,
    max_gap_within_cluster      INTEGER NOT NULL,

    CHECK (instr(days_to_check, '$') = 0 AND
        replace(replace(replace(replace(replace(replace(replace('$'||days_to_check,
            '$M','$'), '$T','$'), '$W','$'), '$R','$'), '$F','$'), '$S','$'), '$U','$') = '$'),
    CHECK (days_off >= 0 AND days_off < 7),
    CHECK (days_off_penalty >= 0 AND days_off_penalty < 100),
    CHECK (days_off_penalty > 0 OR days_off = 0),
    CHECK (days_off_penalty = 0 OR length(days_to_check) > 1),
    CHECK (evenly_spread_penalty >= 0 AND evenly_spread_penalty < 100),
    CHECK (evenly_spread_penalty = 0 OR length(days_to_check) > 1),
    CHECK (max_gap_within_cluster >= 0 AND max_gap_within_cluster < 120),

    FOREIGN KEY (faculty) REFERENCES faculty (faculty) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE faculty_preference_intervals (
    faculty                     TEXT NOT NULL,
    is_cluster                  BOOLEAN NOT NULL,       -- true => cluster, false => gap
    is_too_short                BOOLEAN NOT NULL,       -- true => too short, false => too long
    interval_minutes            INTEGER NOT NULL,
    interval_penalty            INTEGER NOT NULL,
    -- e.g., cluster shorter than 110 minutes incurs penalty 5,
    -- or    gap     longer  than 105 minutes incurs penalty 10

    CHECK (interval_minutes > 0 AND interval_minutes < 24*60),
    CHECK (interval_penalty >= 0 AND interval_penalty < 100),

    PRIMARY KEY (faculty, is_cluster, is_too_short, interval_minutes),
    FOREIGN KEY (faculty) REFERENCES faculty_preferences (faculty)
) WITHOUT ROWID;

CREATE TABLE courses (
    course                      TEXT PRIMARY KEY,
    department                  TEXT NOT NULL,
    course_name                 TEXT NOT NULL,
    prefix                      TEXT GENERATED ALWAYS AS (substr(course, 1, instr(course, ' ') - 1)) VIRTUAL NOT NULL,
    course_number               TEXT GENERATED ALWAYS AS (substr(course, instr(course, ' ') + 1)) VIRTUAL NOT NULL,

    CHECK (length(prefix) >= 1),
    CHECK (length(course_number) >= 4),

    FOREIGN KEY (department) REFERENCES departments (department) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE prereqs (
    course                      TEXT NOT NULL,
    prereq                      TEXT NOT NULL,

    PRIMARY KEY (course, prereq),
    FOREIGN KEY (course) REFERENCES courses (course) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (prereq) REFERENCES courses (course) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE coreqs (
    course                      TEXT NOT NULL,
    coreq                       TEXT NOT NULL,

    PRIMARY KEY (course, coreq),
    FOREIGN KEY (course) REFERENCES courses (course) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (coreq) REFERENCES courses (course) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE sections (
    section                     TEXT PRIMARY KEY,
    course                      TEXT GENERATED ALWAYS AS (substr(section, 1, instr(section, '-') - 1)) VIRTUAL NOT NULL,
    section_number              TEXT GENERATED ALWAYS AS (substr(section, instr(section, '-') + 1)) VIRTUAL NOT NULL,

    CHECK (length(course) >= 6),
    CHECK (length(section_number) >= 2),
    CHECK (course || '-' || section_number = section),

    FOREIGN KEY (course) REFERENCES courses (course) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE section_room_tags (
    section                     TEXT NOT NULL,
    room_tag                    TEXT NOT NULL,
    room_penalty                INTEGER NOT NULL,

    CHECK (room_penalty >= 0 AND room_penalty < 100),

    PRIMARY KEY (section, room_tag),
    FOREIGN KEY (section) REFERENCES sections (section) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (room_tag) REFERENCES room_tags (room_tag) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE section_time_slot_tags (
    section                     TEXT NOT NULL,
    time_slot_tag               TEXT NOT NULL,
    time_slot_penalty           INTEGER NOT NULL,

    CHECK (time_slot_penalty >= 0 AND time_slot_penalty < 100),

    PRIMARY KEY (section, time_slot_tag),
    FOREIGN KEY (section) REFERENCES sections (section) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (time_slot_tag) REFERENCES time_slot_tags (time_slot_tag) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE faculty_sections (
    faculty                     TEXT NOT NULL,
    section                     TEXT NOT NULL,

    PRIMARY KEY (faculty, section),
    FOREIGN KEY (faculty) REFERENCES faculty (faculty) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (section) REFERENCES sections (section) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

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

CREATE TABLE anti_conflicts (
    anti_conflict_single        TEXT PRIMARY KEY,
    anti_conflict_penalty       INTEGER NOT NULL,

    CHECK (anti_conflict_penalty > 0 AND anti_conflict_penalty <= 100),

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

    CHECK (anti_conflict_course <> substr(anti_conflict_single, 1, length(anti_conflict_course))),

    PRIMARY KEY (anti_conflict_single, anti_conflict_course),
    FOREIGN KEY (anti_conflict_single) REFERENCES anti_conflicts (anti_conflict_single) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE conflicts (
    program                     TEXT NOT NULL,
    conflict_name               TEXT NOT NULL,
    conflict_penalty            INTEGER NOT NULL,
    conflict_maximize           BOOLEAN NOT NULL,

    CHECK (conflict_penalty >= 0 AND conflict_penalty <= 100),

    PRIMARY KEY (program, conflict_name),
    FOREIGN KEY (program) REFERENCES programs (program) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

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

-- TODO
CREATE TABLE multiple_section_overrides (
    course                      TEXT PRIMARY KEY,
    section_count               INTEGER NOT NULL,

    FOREIGN KEY (course) REFERENCES courses (course) ON DELETE CASCADE ON UPDATE CASCADE) WITHOUT ROWID;

CREATE VIEW active_cross_listings (department, section, primary_section) AS
    SELECT DISTINCT department, cross_listing_sections.section, primary_section
    FROM courses
    NATURAL JOIN sections
    NATURAL JOIN cross_listing_sections
    JOIN section_time_slot_tags
        ON  section_time_slot_tags.section                  = cross_listing_sections.primary_section;

CREATE VIEW active_sections (department, course, section, secondary_section) AS
    SELECT department, course, section, section
    FROM courses
    NATURAL JOIN sections
    NATURAL JOIN section_time_slot_tags

    UNION

    SELECT department, course, active_cross_listings.primary_section, active_cross_listings.section
    FROM courses
    NATURAL JOIN sections
    NATURAL JOIN active_cross_listings
    JOIN section_time_slot_tags
        ON  section_time_slot_tags.section                  = active_cross_listings.primary_section;

CREATE VIEW active_rooms (department, room, building, room_number, capacity) AS
    SELECT DISTINCT department, room, building, room_number, capacity
    FROM active_sections
    NATURAL JOIN section_room_tags
    NATURAL JOIN rooms_room_tags
    NATURAL JOIN rooms;

CREATE VIEW active_time_slots (department, time_slot, days, start_time, duration, first_day) AS
    SELECT DISTINCT department, time_slot, days, start_time, duration, first_day
    FROM active_sections
    NATURAL JOIN section_time_slot_tags
    NATURAL JOIN time_slots_time_slot_tags
    NATURAL JOIN time_slots;

CREATE VIEW active_section_time_slots (department, section, time_slot, time_slot_penalty) AS
    SELECT department, section, time_slot, MAX(time_slot_penalty)
    FROM active_sections
    NATURAL JOIN section_time_slot_tags
    NATURAL JOIN time_slots_time_slot_tags
    GROUP BY department, section, time_slot;

CREATE VIEW active_section_rooms (department, section, room, room_penalty) AS
    SELECT department, section, room, MAX(room_penalty)
    FROM active_sections
    NATURAL JOIN section_room_tags
    NATURAL JOIN rooms_room_tags
    GROUP BY department, section, room;

CREATE VIEW active_conflicts (program, conflict_name, conflict_penalty, conflict_maximize, department, course, section) AS
    SELECT DISTINCT program, conflict_name, conflict_penalty, conflict_maximize, department, course, section
    FROM active_sections
    NATURAL JOIN conflict_courses
    NATURAL JOIN conflicts;

CREATE VIEW active_prereqs (section_department, section, prereq_department, prereq) AS
    SELECT DISTINCT sections.department AS section_department, sections.section AS section,
                    prereq_sections.department AS prereq_department, prereq_sections.section AS prereq
    FROM active_sections                                    AS sections
    JOIN prereqs
        ON  prereqs.course                                  = sections.course
    JOIN active_sections                                    AS prereq_sections
        ON  prereq_sections.course                          = prereqs.prereq;

CREATE VIEW active_coreqs (section_department, section, coreq_department, coreq) AS
    SELECT DISTINCT sections.department AS section_department, sections.section AS section,
                    coreq_sections.department AS coreq_department, coreq_sections.section AS coreq
    FROM active_sections                                    AS sections
    JOIN coreqs
        ON  coreqs.course                                   = sections.course
    JOIN active_sections                                    AS coreq_sections
        ON  coreq_sections.course                           = coreqs.coreq;

CREATE VIEW active_anti_conflicts (single_department, single_section, group_department, group_section, anti_conflict_penalty) AS
    SELECT  single_sections.department AS single_department, single_sections.section AS single_section,
            group_sections.department AS group_department, group_sections.section AS group_section,
            anti_conflict_penalty
    FROM active_sections                                    AS single_sections
    JOIN anti_conflicts
        ON  anti_conflicts.anti_conflict_single             = single_sections.secondary_section
    JOIN anti_conflict_sections
        ON  anti_conflict_sections.anti_conflict_single     = anti_conflicts.anti_conflict_single
    JOIN active_sections                                    AS group_sections
        ON  group_sections.secondary_section                = anti_conflict_sections.anti_conflict_section

    UNION

    SELECT  single_sections.department AS single_department, single_sections.section AS single_section,
            group_sections.department AS group_department, group_sections.section AS group_section,
            anti_conflict_penalty
    FROM active_sections                                    AS single_sections
    JOIN anti_conflicts
        ON  anti_conflicts.anti_conflict_single             = single_sections.secondary_section
    JOIN anti_conflict_courses
        ON  anti_conflict_courses.anti_conflict_single      = anti_conflicts.anti_conflict_single
    JOIN active_sections                                    AS group_sections
        ON  group_sections.course                           = anti_conflict_courses.anti_conflict_course;

CREATE VIEW active_faculty_availability (department, faculty, day_of_week, start_time, duration, availability_penalty) AS
    SELECT DISTINCT department, faculty,
                    day_of_week, start_time, duration, availability_penalty
    FROM active_faculty_sections
    NATURAL JOIN faculty_availability;

CREATE VIEW active_faculty_preference_intervals (department, faculty,
        days_to_check, days_off, days_off_penalty, evenly_spread_penalty, max_gap_within_cluster,
        is_cluster, is_too_short, interval_minutes, interval_penalty) AS
    SELECT DISTINCT department, faculty,
                    days_to_check, days_off, days_off_penalty, evenly_spread_penalty, max_gap_within_cluster,
                    is_cluster, is_too_short, interval_minutes, interval_penalty
    FROM active_faculty_sections
    NATURAL JOIN faculty_preferences
    LEFT OUTER NATURAL JOIN faculty_preference_intervals;

CREATE VIEW active_faculty_sections (faculty, department, course, section) AS
    SELECT faculty, department, course, section
    FROM active_sections
    NATURAL JOIN faculty_sections;

-- FIXME
-- only count if spreading requirement is in place?
-- do not count if anticonflict is in place?
-- secondary cross listings should not be confused with online
CREATE VIEW active_section_counts (department, course, section_count) AS
    WITH time_slot_courses AS (
        SELECT DISTINCT department, course
        FROM courses
        NATURAL JOIN sections
        NATURAL JOIN section_time_slot_tags
    )
    SELECT department, course, COUNT(section) AS section_count
    FROM time_slot_courses
    LEFT OUTER NATURAL JOIN sections
    GROUP BY department, course
    HAVING section_count > 1;

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
WHEN (SELECT COUNT(1) FROM terms, holidays WHERE holiday <= start_date OR holiday >= end_date) > 1
BEGIN
    SELECT RAISE(ABORT, 'holidays must be during the term');
END;
CREATE TRIGGER holidays_in_range_update
AFTER UPDATE ON holidays
WHEN (SELECT COUNT(1) FROM terms, holidays WHERE holiday <= start_date OR holiday >= end_date) > 1
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
