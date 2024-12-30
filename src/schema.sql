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
    building                    TEXT GENERATED ALWAYS AS (SUBSTR(room, 1, INSTR(room, ' ') - 1)) VIRTUAL NOT NULL,
    room_number                 TEXT GENERATED ALWAYS AS (SUBSTR(room, LENGTH(building) + 2)) VIRTUAL NOT NULL,
    capacity                    INTEGER NOT NULL,

    CHECK (LENGTH(room_number) > 0),

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

CREATE VIEW time_slot_conflicts (department, time_slot_a, time_slot_b) AS
    SELECT a.department, a.time_slot, b.time_slot
    FROM active_time_slots AS a
    JOIN active_time_slots AS b
    ON
        a.department = b.department
        AND CASE
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
    availability_penalty        INTEGER,

    CHECK (day_of_week IN ('M', 'T', 'W', 'R', 'F', 'S', 'U')),
    CHECK (start_time >= 0 AND start_time % 5 = 0),
    CHECK (duration > 0 AND duration % 5 = 0),
    CHECK (start_time + duration < 24*60),
    CHECK (availability_penalty IS NULL OR availability_penalty > 0 AND availability_penalty < 100),

    PRIMARY KEY (faculty, day_of_week, start_time),
    FOREIGN KEY (faculty) REFERENCES faculty (faculty) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE faculty_preferences (
    faculty                     TEXT PRIMARY KEY,
    days_to_check               TEXT NOT NULL,
    days_off                    INTEGER,
    days_off_penalty            INTEGER,
    evenly_spread_penalty       INTEGER,
    max_gap_within_cluster      INTEGER NOT NULL,

    CHECK (INSTR(days_to_check, '$') = 0 AND
        REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE('$'||days_to_check,
            '$M','$'), '$T','$'), '$W','$'), '$R','$'), '$F','$'), '$S','$'), '$U','$') = '$'),
    CHECK (days_off IS NULL OR days_off >= 0 AND days_off < 7),
    CHECK (days_off_penalty IS NULL OR days_off_penalty > 0 AND days_off_penalty < 100),
    CHECK (days_off_penalty IS NULL AND days_off IS NULL OR days_off_penalty IS NOT NULL AND days_off IS NOT NULL),
    CHECK (days_off_penalty IS NULL OR LENGTH(days_to_check) > 1),
    CHECK (evenly_spread_penalty IS NULL OR evenly_spread_penalty > 0 AND evenly_spread_penalty < 100),
    CHECK (evenly_spread_penalty IS NULL OR LENGTH(days_to_check) > 1),
    CHECK (max_gap_within_cluster >= 0 AND max_gap_within_cluster < 120),

    FOREIGN KEY (faculty) REFERENCES faculty (faculty) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE faculty_preference_intervals (
    faculty                     TEXT NOT NULL,
    is_cluster                  BOOLEAN NOT NULL,       -- true => cluster, false => gap
    is_too_short                BOOLEAN NOT NULL,       -- true => too short, false => too long
    interval_minutes            INTEGER NOT NULL,
    interval_penalty            INTEGER,
    -- e.g., cluster shorter than 110 minutes incurs penalty 5,
    -- or    gap     longer  than 105 minutes incurs penalty 10

    CHECK (interval_minutes > 0 AND interval_minutes < 24*60),
    CHECK (interval_penalty IS NULL OR interval_penalty > 0 AND interval_penalty < 100),

    PRIMARY KEY (faculty, is_cluster, is_too_short, interval_minutes),
    FOREIGN KEY (faculty) REFERENCES faculty_preferences (faculty)
) WITHOUT ROWID;

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
    course                      TEXT GENERATED ALWAYS AS (SUBSTR(section, 1, INSTR(section, '-') - 1)) VIRTUAL NOT NULL,
    section_number              TEXT GENERATED ALWAYS AS (SUBSTR(section, INSTR(section, '-') + 1)) VIRTUAL NOT NULL,

    CHECK (LENGTH(course) >= 6),
    CHECK (LENGTH(section_number) >= 2),
    CHECK (course || '-' || section_number = section),

    FOREIGN KEY (course) REFERENCES courses (course) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE section_room_tags (
    section                     TEXT NOT NULL,
    room_tag                    TEXT NOT NULL,
    room_penalty                INTEGER,

    CHECK (room_penalty IS NULL OR room_penalty > 0 AND room_penalty < 100),

    PRIMARY KEY (section, room_tag),
    FOREIGN KEY (section) REFERENCES sections (section) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (room_tag) REFERENCES room_tags (room_tag) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE section_time_slot_tags (
    section                     TEXT NOT NULL,
    time_slot_tag               TEXT NOT NULL,
    time_slot_penalty           INTEGER,

    CHECK (time_slot_penalty IS NULL OR time_slot_penalty > 0 AND time_slot_penalty < 100),

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

    CHECK (anti_conflict_course <> SUBSTR(anti_conflict_single, 1, LENGTH(anti_conflict_course))),

    PRIMARY KEY (anti_conflict_single, anti_conflict_course),
    FOREIGN KEY (anti_conflict_single) REFERENCES anti_conflicts (anti_conflict_single) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE conflicts (
    program                     TEXT NOT NULL,
    conflict_name               TEXT NOT NULL,
    conflict_penalty            INTEGER,
    conflict_maximize           BOOLEAN NOT NULL,

    CHECK (conflict_penalty IS NULL OR conflict_penalty > 0 AND conflict_penalty <= 100),
    CHECK (conflict_penalty IS NOT NULL OR NOT conflict_maximize),

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
        ON section_time_slot_tags.section = cross_listing_sections.primary_section;

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
        ON section_time_slot_tags.section = active_cross_listings.primary_section;

CREATE VIEW active_rooms (department, room, building, room_number, capacity) AS
    SELECT DISTINCT department, room, building, room_number, capacity
    FROM active_sections
    NATURAL JOIN section_room_tags
    NATURAL JOIN rooms_room_tags
    NATURAL JOIN rooms;

CREATE VIEW active_time_slots (department, time_slot, days, start_time, duration, first_day) AS
    SELECT DISTINCT department, time_slot, days, start_time, duration, first_day
    FROM active_section_time_slots
    NATURAL JOIN time_slots;

CREATE VIEW faculty_time_slots (faculty, time_slot, faculty_time_slot_penalty) AS
    WITH overlapping_intervals (faculty, faculty_minutes, penalty, time_slot, time_slot_minutes) AS (
        SELECT  faculty,
                    -- time slot is entirely inside availability
                CASE
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
                availability_penalty,
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
    )

    SELECT faculty, time_slot, MAX(penalty)
    FROM overlapping_intervals
    GROUP BY faculty, time_slot, time_slot_minutes
    HAVING SUM(faculty_minutes) = time_slot_minutes;

CREATE VIEW active_section_time_slots (department, section, time_slot, time_slot_penalty) AS
    WITH per_faculty (department, section, faculty, time_slot, time_slot_penalty) AS (
        SELECT department, section, faculty, time_slot,
            CASE WHEN time_slot_penalty IS NULL THEN faculty_time_slot_penalty
                 WHEN faculty_time_slot_penalty IS NULL THEN time_slot_penalty
                 ELSE MAX(time_slot_penalty, faculty_time_slot_penalty) END
        FROM active_sections
        NATURAL JOIN section_time_slot_tags
        NATURAL JOIN time_slots_time_slot_tags
        NATURAL JOIN faculty_sections
        NATURAL JOIN faculty_time_slots
    ),

    group_faculty (department, section, time_slot, time_slot_penalty, faculty_assigned) AS (
        SELECT department, section, time_slot, MAX(time_slot_penalty), COUNT(faculty)
        FROM per_faculty
        GROUP BY department, section, time_slot
    ),

    faculty_count (section, total_faculty_assigned) AS (
        SELECT section, COUNT(1) as faculty_assigned
        FROM faculty_sections
        GROUP BY section
    ),

    intersect_faculty (department, section, time_slot, time_slot_penalty) AS (
        SELECT department, section, time_slot, time_slot_penalty
        FROM group_faculty
        NATURAL JOIN faculty_count
        WHERE faculty_assigned = total_faculty_assigned
    )

    -- time slots where all faculty are available
    SELECT department, section, time_slot, time_slot_penalty
    FROM intersect_faculty

    UNION

    -- time slots for section with no faculty assigned
    SELECT department, section, time_slot, 0
    FROM active_sections
    NATURAL JOIN section_time_slot_tags
    NATURAL JOIN time_slots_time_slot_tags
    NATURAL LEFT OUTER JOIN faculty_sections
    WHERE faculty is NULL;

CREATE VIEW active_section_rooms (department, section, room, room_penalty) AS
    SELECT department, section, room, MAX(room_penalty)
    FROM active_sections
    NATURAL JOIN section_room_tags
    NATURAL JOIN rooms_room_tags
    GROUP BY department, section, room;

-- all section-to-section conflicts (where section_a < section_b)
-- this combines the different conflicts within programs, accounts
-- for minimizing and maximizing conflicts, and then merges everything
-- across programs
-- but does not account for prereqs and multiple sections
CREATE VIEW active_undiscounted_conflicts (department_a, course_a, section_a, department_b, course_b, section_b, penalty) AS
    -- expand conflict_courses cliques
    -- note: does NOT create conflicts between sections within a course
    --       for that, see paired_conflict_sections
    WITH paired_conflict_courses_courses AS (
        SELECT conflicts.program, conflicts.conflict_name, conflict_penalty, conflict_maximize, s1.section AS section_a, s2.section AS section_b
        FROM conflicts
        JOIN conflict_courses c1
            ON  c1.program                                      =  conflicts.program
            AND c1.conflict_name                                =  conflicts.conflict_name
        JOIN sections s1
            ON  s1.course                                       =  c1.course
        JOIN conflict_courses c2
            ON  c2.program                                      =  conflicts.program
            AND c2.conflict_name                                =  conflicts.conflict_name
        JOIN sections s2
            ON  s2.course                                       =  c2.course
        WHERE   c2.course                                       <> c1.course
            AND s2.section                                      >  s1.section
    ),

    -- expand conflict_sections cliques
    paired_conflict_sections_sections AS (
        SELECT conflicts.program, conflicts.conflict_name, conflict_penalty, conflict_maximize, s1.section AS section_a, s2.section AS section_b
        FROM conflicts
        JOIN conflict_sections s1
            ON  s1.program                                      =  conflicts.program
            AND s1.conflict_name                                =  conflicts.conflict_name
        JOIN conflict_sections s2
            ON  s2.program                                      =  conflicts.program
            AND s2.conflict_name                                =  conflicts.conflict_name
        WHERE   s2.section                                      >  s1.section
    ),

    -- expand conflict_sections -> conflict_courses
    paired_conflict_sections_courses AS (
        SELECT conflicts.program, conflicts.conflict_name, conflict_penalty, conflict_maximize, s1.section AS section_a, s2.section AS section_b
        FROM conflicts
        JOIN conflict_sections s1
            ON  s1.program                                      =  conflicts.program
            AND s1.conflict_name                                =  conflicts.conflict_name
        JOIN conflict_courses c2
            ON  c2.program                                      =  conflicts.program
            AND c2.conflict_name                                =  conflicts.conflict_name
        JOIN sections s2
            ON  s2.course                                       =  c2.course
        WHERE   s2.section                                      >  s1.section
    ),

    -- expand conflict_courses -> conflict_sections
    paired_conflict_courses_sections AS (
        SELECT conflicts.program, conflicts.conflict_name, conflict_penalty, conflict_maximize, s1.section AS section_a, s2.section AS section_b
        FROM conflicts
        JOIN conflict_courses c1
            ON  c1.program                                      =  conflicts.program
            AND c1.conflict_name                                =  conflicts.conflict_name
        JOIN sections s1
            ON  s1.course                                       =  c1.course
        JOIN conflict_sections s2
            ON  s2.program                                      =  conflicts.program
            AND s2.conflict_name                                =  conflicts.conflict_name
        WHERE   s2.section                                      >  s1.section
    ),

    -- merge all section conflicts derived from sections or courses
    paired_conflicts AS (
        SELECT * FROM paired_conflict_courses_courses
        UNION
        SELECT * FROM paired_conflict_sections_sections
        UNION
        SELECT * FROM paired_conflict_sections_courses
        UNION
        SELECT * FROM paired_conflict_courses_sections
    ),

    -- combine conflicts within a program, tracking maximizing and minimizing conflicts
    per_program_conflicts AS (
        SELECT program, section_a, section_b,
            MAX(CASE WHEN conflict_maximize THEN conflict_penalty ELSE NULL END) as max_penalty,
            MIN(CASE WHEN conflict_maximize THEN NULL ELSE conflict_penalty END) as min_penalty
        FROM paired_conflicts
        GROUP BY program, section_a, section_b
    ),

    -- apply minimizing conflicts to reduce penalties
    reduced_conflicts AS (
        SELECT program, section_a, section_b,
            CASE WHEN max_penalty IS NOT NULL AND min_penalty IS NOT NULL THEN MIN(max_penalty, min_penalty)
                 WHEN max_penalty IS NOT NULL THEN max_penalty
                 ELSE 0
            END AS penalty
        FROM per_program_conflicts
        WHERE penalty IS NOT NULL
    )

    -- merge conflicts across programs
    SELECT  as_a.department, as_a.course, section_a,
            as_b.department, as_b.course, section_b, MAX(penalty)
    FROM reduced_conflicts
    JOIN active_sections AS as_a
        ON section_a = as_a.section
    JOIN active_sections AS as_b
        ON section_b = as_b.section
    GROUP BY as_a.department, section_a, as_b.department, section_b;

-- list of courses with prereqs, including prereqs of prereqs, etc.
-- the prereq of a prereq counts,
-- the coreq of a prereq counts,
-- the prereq of a coreq counts.
-- however, an immediate coreq will not appear in this list
-- (but its prereqs will).
-- we do not follow coreqs transitively (is that even a thing?)
CREATE VIEW prereq_transitive_closure (course, prereq) AS
    -- treat coreqs and prereqs as the same...
    WITH merged_pre_and_co AS (
        SELECT course, prereq FROM prereqs
        UNION
        SELECT course, coreq as prereq FROM coreqs
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

-- a list of the number of sections of each course with multiple sections
-- includes online, etc., and computed value is overriden by
-- multiple_section_overrides
--
-- note: use multiple_section_overrides to handle
--       cross listings, anticonflicts, etc.
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
            iif(multiple_section_overrides.section_count IS NULL,
                all_sections.section_count,
                multiple_section_overrides.section_count) AS final_count
        FROM all_sections
        LEFT OUTER JOIN multiple_section_overrides
            ON all_sections.course = multiple_section_overrides.course
    )
    SELECT DISTINCT department, course, final_count
    FROM with_overrides
    NATURAL JOIN active_sections
    WHERE final_count > 1;

-- the fully-processed list of section-to-section conflicts
-- accounts for conflits across programs, prereqs/coreqs, and multiple section discounts
--
-- note: discounting formula is hard-coded, as is minimum conflict value
CREATE VIEW active_conflicts (department_a, section_a, department_b, section_b, penalty) AS
    -- remove conflicts when there is a prereq relationship
    -- and discount multiple sections
    WITH merged (department_a, section_a, department_b, section_b, penalty) AS (
        SELECT department_a, section_a, department_b, section_b,
            CASE WHEN undiscounted.penalty = 100
                    THEN undiscounted.penalty
                 WHEN counts_a.section_count IS NOT NULL AND counts_b.section_count IS NOT NULL
                    THEN ((undiscounted.penalty - 1) / (counts_a.section_count + 1) - 1) / (counts_b.section_count + 1)
                 WHEN counts_a.section_count IS NOT NULL
                    THEN  (undiscounted.penalty - 1) / (counts_a.section_count + 1)
                 WHEN counts_b.section_count IS NOT NULL
                    THEN  (undiscounted.penalty - 1) / (counts_b.section_count + 1)
                 ELSE
                    undiscounted.penalty
            END AS discounted_penalty
        FROM active_undiscounted_conflicts AS undiscounted
        -- hack: doing this as two left outer joins is much faster
        -- than a join condition with an OR to consider both cases
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
        AND discounted_penalty >= 30

        UNION

        -- merge with hard conflict for courses with the same instructor
        SELECT  sec_a.department, sec_a.section, sec_b.department, sec_b.section, 100
        FROM active_faculty_sections AS sec_a
        JOIN active_faculty_sections AS sec_b
            ON  sec_a.faculty                                   = sec_b.faculty
            AND sec_a.section                                   < sec_b.section
    )

    SELECT department_a, section_a, department_b, section_b, MAX(penalty)
    FROM merged
    GROUP BY department_a, section_a, department_b, section_b;

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

CREATE VIEW active_faculty_preference_intervals (department, faculty,
        days_to_check, days_off, days_off_penalty, evenly_spread_penalty, max_gap_within_cluster,
        is_cluster, is_too_short, interval_minutes, interval_penalty) AS
    SELECT DISTINCT department, faculty,
                    days_to_check, days_off, days_off_penalty, evenly_spread_penalty, max_gap_within_cluster,
                    is_cluster, is_too_short, interval_minutes, interval_penalty
    FROM active_faculty_sections
    NATURAL JOIN faculty_preferences
    NATURAL LEFT OUTER JOIN faculty_preference_intervals;

CREATE VIEW active_faculty_sections (faculty, department, course, section) AS
    SELECT faculty, department, course, section
    FROM active_sections
    NATURAL JOIN faculty_sections;

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
