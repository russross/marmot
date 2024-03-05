CREATE TABLE terms (
    term                TEXT NOT NULL,
    start_date          DATE NOT NULL,
    end_date            DATE NOT NULL,
    current             BOOLEAN NOT NULL,

    PRIMARY KEY (term)
) WITHOUT ROWID;

CREATE TRIGGER terms_one_active_insert
AFTER INSERT ON terms
WHEN (SELECT COUNT(1) FROM terms WHERE current) > 1
BEGIN
    SELECT RAISE(FAIL, 'only one term can be current!');
END;
CREATE TRIGGER terms_one_active_update
AFTER UPDATE ON terms
WHEN (SELECT COUNT(1) FROM terms WHERE current) > 1
BEGIN
    SELECT RAISE(FAIL, 'only one term can be current!');
END;

CREATE TABLE holidays (
    term                TEXT NOT NULL,
    holiday             DATE NOT NULL,

    PRIMARY KEY (term, holiday),
    FOREIGN KEY (term) REFERENCES terms (term) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE buildings (
    term                TEXT NOT NULL,
    building            TEXT NOT NULL,

    PRIMARY KEY (term, building),
    FOREIGN KEY (term) REFERENCES terms (term) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE rooms (
    term                TEXT NOT NULL,
    room                TEXT NOT NULL,
    building            TEXT GENERATED ALWAYS AS (substr(room, 1, instr(room, ' ') - 1)) VIRTUAL NOT NULL,
    room_number         TEXT GENERATED ALWAYS AS (substr(room, length(building) + 2)) VIRTUAL NOT NULL,
    capacity            INTEGER NOT NULL,

    CHECK (length(room_number) > 0),

    PRIMARY KEY (term, room),
    FOREIGN KEY (term, building) REFERENCES buildings (term, building) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE room_tags (
    term                TEXT NOT NULL,
    room_tag            TEXT NOT NULL,

    PRIMARY KEY (term, room_tag),
    FOREIGN KEY (term) REFERENCES terms (term) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE rooms_room_tags (
    term                TEXT NOT NULL,
    room_tag            TEXT NOT NULL,
    room                TEXT NOT NULL,

    PRIMARY KEY (term, room_tag, room),
    FOREIGN KEY (term, room_tag) REFERENCES room_tags (term, room_tag) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (term, room) REFERENCES rooms (term, room) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE time_slots (
    term                TEXT NOT NULL,
    time_slot           TEXT NOT NULL,
    days                TEXT GENERATED ALWAYS AS (substr(time_slot, 1, length(time_slot) - length(duration) - 5)) VIRTUAL NOT NULL,
    start_time          INTEGER GENERATED ALWAYS AS (CAST(substr(time_slot, -(length(duration) + 5), 2) AS INTEGER) * 60 + CAST(substr(time_slot, -(length(duration) + 3), 2) AS INTEGER)) VIRTUAL NOT NULL,
    duration            INTEGER GENERATED ALWAYS AS (CAST(substr(time_slot, instr(time_slot, '+')) AS INTEGER)) VIRTUAL NOT NULL,

    CHECK (start_time >= 0 AND start_time % 5 = 0),
    CHECK (duration > 0 AND duration % 5 = 0),
    CHECK (start_time + duration < 24*60),
    CHECK (length(days) > 0 AND instr(days, '$') = 0 AND
        replace(replace(replace(replace(replace(replace(replace('$'||days,
            '$M','$'), '$T','$'), '$W','$'), '$R','$'), '$F','$'), '$S','$'), '$U','$') = '$'),
    CHECK (days || substr('00'||CAST(start_time / 60 AS TEXT), -2) || substr('00'||CAST(start_time % 60 AS TEXT), -2) || '+' || CAST(duration AS TEXT) = time_slot),

    PRIMARY KEY (term, time_slot),
    FOREIGN KEY (term) REFERENCES terms (term) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE time_slot_tags (
    term                TEXT NOT NULL,
    time_slot_tag       TEXT NOT NULL,

    PRIMARY KEY (term, time_slot_tag),
    FOREIGN KEY (term) REFERENCES terms (term) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE time_slots_time_slot_tags (
    term                TEXT NOT NULL,
    time_slot_tag       TEXT NOT NULL,
    time_slot           TEXT NOT NULL,

    PRIMARY KEY (term, time_slot_tag, time_slot),
    FOREIGN KEY (term, time_slot_tag) REFERENCES time_slot_tags (term, time_slot_tag) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (term, time_slot) REFERENCES time_slots (term, time_slot) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE departments (
    term                TEXT NOT NULL,
    department          TEXT NOT NULL,

    PRIMARY KEY (term, department),
    FOREIGN KEY (term) REFERENCES terms (term) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE programs (
    term                TEXT NOT NULL,
    program             TEXT NOT NULL,
    department          TEXT NOT NULL,

    PRIMARY KEY (term, program),
    FOREIGN KEY (term, department) REFERENCES departments (term, department) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE faculty (
    term                TEXT NOT NULL,
    faculty             TEXT NOT NULL,
    department          TEXT NOT NULL,

    PRIMARY KEY (term, faculty),
    FOREIGN KEY (term, department) REFERENCES departments (term, department)
) WITHOUT ROWID;

CREATE TABLE availability (
    term                TEXT NOT NULL,
    faculty             TEXT NOT NULL,
    day_of_week         TEXT NOT NULL,
    start_time          INTEGER NOT NULL,
    duration            INTEGER NOT NULL,
    penalty             INTEGER NOT NULL,

    CHECK (day_of_week IN ('M', 'T', 'W', 'R', 'F', 'S', 'U')),
    CHECK (start_time >= 0 AND start_time % 5 = 0),
    CHECK (duration > 0 AND duration % 5 = 0),
    CHECK (start_time + duration < 24*60),
    CHECK (penalty >= 0 AND penalty < 100),

    PRIMARY KEY (term, faculty, day_of_week, start_time),
    FOREIGN KEY (term, faculty) REFERENCES faculty (term, faculty) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE courses (
    term                TEXT NOT NULL,
    department          TEXT NOT NULL,
    course              TEXT NOT NULL,
    course_name         TEXT NOT NULL,
    prefix              TEXT GENERATED ALWAYS AS (substr(course, 1, instr(course, ' ') - 1)) VIRTUAL NOT NULL,
    course_number       TEXT GENERATED ALWAYS AS (substr(course, instr(course, ' ') + 1)) VIRTUAL NOT NULL,

    CHECK (length(prefix) >= 1),
    CHECK (length(course_number) >= 4),

    PRIMARY KEY (term, course),
    FOREIGN KEY (term, department) REFERENCES departments (term, department) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE sections (
    term                TEXT NOT NULL,
    section             TEXT NOT NULL,
    course              TEXT GENERATED ALWAYS AS (substr(section, 1, instr(section, '-') - 1)) VIRTUAL NOT NULL,
    section_number      TEXT GENERATED ALWAYS AS (substr(section, instr(section, '-') + 1)) VIRTUAL NOT NULL,

    CHECK (length(course) >= 6),
    CHECK (length(section_number) >= 2),
    CHECK (course || '-' || section_number = section),

    PRIMARY KEY (term, section),
    FOREIGN KEY (term, course) REFERENCES courses (term, course) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE section_room_tags (
    term                TEXT NOT NULL,
    section             TEXT NOT NULL,
    room_tag            TEXT NOT NULL,
    room_penalty        INTEGER,

    PRIMARY KEY (term, section, room_tag),
    FOREIGN KEY (term, section) REFERENCES sections (term, section) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (term, room_tag) REFERENCES room_tags (term, room_tag) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE section_time_slot_tags (
    term                TEXT NOT NULL,
    section             TEXT NOT NULL,
    time_slot_tag       TEXT NOT NULL,
    time_slot_penalty   INTEGER,

    PRIMARY KEY (term, section, time_slot_tag),
    FOREIGN KEY (term, section) REFERENCES sections (term, section) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (term, time_slot_tag) REFERENCES time_slot_tags (term, time_slot_tag) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE faculty_sections (
    term                TEXT NOT NULL,
    faculty             TEXT NOT NULL,
    section             TEXT NOT NULL,

    PRIMARY KEY (term, faculty, section),
    FOREIGN KEY (term, faculty) REFERENCES faculty (term, faculty) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (term, section) REFERENCES sections (term, section) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE conflicts (
    term                TEXT NOT NULL,
    program             TEXT NOT NULL,
    conflict_name       TEXT NOT NULL,
    conflict_level      INTEGER NOT NULL,
    conflict_maximize   BOOLEAN NOT NULL,

    CHECK (conflict_level >= 0 AND conflict_level <= 100),

    PRIMARY KEY (term, program, conflict_name),
    FOREIGN KEY (term, program) REFERENCES programs (term, program) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE conflict_courses (
    term                TEXT NOT NULL,
    program             TEXT NOT NULL,
    conflict_name       TEXT NOT NULL,
    course              TEXT,

    PRIMARY KEY (term, program, conflict_name, course),
    FOREIGN KEY (term, program, conflict_name) REFERENCES conflicts (term, program, conflict_name) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (term, course) REFERENCES courses (term, course) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;

CREATE TABLE conflict_sections (
    term                TEXT NOT NULL,
    program             TEXT NOT NULL,
    conflict_name       TEXT NOT NULL,
    section             TEXT,

    PRIMARY KEY (term, program, conflict_name, section),
    FOREIGN KEY (term, program, conflict_name) REFERENCES conflicts (term, program, conflict_name) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (term, section) REFERENCES sections (term, section) ON DELETE CASCADE ON UPDATE CASCADE
) WITHOUT ROWID;
