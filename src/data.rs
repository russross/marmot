use super::bits::*;
use super::input::*;
use super::solver::Solver;
use rusqlite::{Connection, OpenFlags, Result};

const DB_PATH: &str = "timetable.db";

pub fn sql_err(err: rusqlite::Error) -> String {
    format!("{err}")
}

// build the solver object with term dates and holidays
pub fn make_solver(db: &mut Connection) -> Result<Solver, String> {
    // get the semester
    struct TermRow {
        term: String,
        start_date: String,
        end_date: String,
    }
    let term_row = db
        .query_row("
                SELECT term, start_date, end_date
                FROM terms
                WHERE current",
            [],
            |row| {
                Ok(TermRow {
                    term: row.get_unwrap(0),
                    start_date: row.get_unwrap(1),
                    end_date: row.get_unwrap(2),
                })
            },
        )
        .map_err(sql_err)?;

    let start_date = parse_date(term_row.start_date)?;
    let end_date = parse_date(term_row.end_date)?;

    // set up the term with 5-minute intervals
    let mut slots = Bits::new(date_range_slots(start_date, end_date));
    let mut day = start_date;
    let mut i = 0;
    while day <= end_date {
        for _hour in 0..24 {
            for _min in (0..60).step_by(5) {
                slots.set(i, true).unwrap();
                i += 1;
            }
        }
        day = day.next_day().unwrap();
    }

    // add the holidays
    let mut stmt = db
        .prepare("
                SELECT holiday
                FROM terms
                NATURAL JOIN holidays
                WHERE current",
        )
        .map_err(sql_err)?;
    let mut rows = stmt.query([]).map_err(sql_err)?;

    while let Some(row) = rows.next().unwrap() {
        // cross a holiday off the 5-minute interval list for the semester
        let day = parse_date(row.get_unwrap(0))?;
        if day < start_date || day > end_date {
            return Err(format!("block_out_holiday: {day} is outside the term"));
        }
        let mut index = ((day - start_date).whole_days() * 24 * 60 / 5) as usize;
        for _hour in 0..24 {
            for _min in (0..60).step_by(5) {
                slots.set(index, false).unwrap();
                index += 1;
            }
        }
    }

    // build the solver object
    Ok(Solver {
        name: term_row.term,
        start: start_date,
        end: end_date,
        slots,
        rooms: Vec::new(),
        time_slots: Vec::new(),
        instructors: Vec::new(),
        input_sections: Vec::new(),
        missing: Vec::new(),
        time_slot_conflicts: Vec::new(),
        anticonflicts: Vec::new(),

        input_locked: false,
        sections: Vec::new(),
        room_placements: Vec::new(),
        score: 0,
        unplaced_current: 0,
        unplaced_best: 0,
    })
}

fn dept_clause(departments: &Vec<String>) -> String {
    if departments.is_empty() {
        "".to_string()
    } else {
        let mut s = " AND department IN (".to_string();
        let mut sep = "";
        for _ in departments {
            s = format!("{s}{sep}?");
            sep = ", ";
        }
        format!("{s})")
    }
}

fn dept_multi_clause(departments: &Vec<String>, tables: &Vec<String>) -> String {
    let mut s = "".to_string();
    if !departments.is_empty() {
        for table in tables {
            s = format!("{s} AND {table}.department IN (");
            let mut sep = "";
            for _ in departments {
                s = format!("{s}{sep}?");
                sep = ", ";
            }
            s = format!("{s})");
        }
    }
    s
}

// load all rooms
pub fn load_rooms(
    db: &mut Connection,
    solver: &mut Solver,
    departments: &Vec<String>,
) -> Result<(), String> {
    let mut stmt = db
        .prepare(&format!("
                SELECT DISTINCT room, capacity
                FROM terms
                NATURAL JOIN courses
                NATURAL JOIN sections
                NATURAL JOIN section_time_slot_tags
                NATURAL JOIN section_room_tags
                NATURAL JOIN rooms_room_tags
                NATURAL JOIN rooms
                WHERE current{}
                ORDER BY building, room_number",
            dept_clause(departments)
        ))
        .map_err(sql_err)?;
    let mut rows = stmt
        .query(rusqlite::params_from_iter(departments))
        .map_err(sql_err)?;

    while let Some(row) = rows.next().map_err(sql_err)? {
        solver.rooms.push(Room {
            name: row.get_unwrap(0),
            capacity: row.get_unwrap(1),
            tags: Vec::new(),
        });
    }

    Ok(())
}

// load all time slots
pub fn load_time_slots(
    db: &mut Connection,
    solver: &mut Solver,
    departments: &Vec<String>,
) -> Result<(), String> {
    // example: MWF0900+5
    let re = regex::Regex::new(
        r"^([mtwrfsuMTWRFSU]+)([0-1][0-9]|2[0-3])([0-5][05])\+([1-9][0-9]?[05])$",
    )
    .unwrap();

    let mut stmt = db
        .prepare(&format!("
                SELECT DISTINCT time_slot
                FROM terms
                NATURAL JOIN courses
                NATURAL JOIN sections
                NATURAL JOIN section_time_slot_tags
                NATURAL JOIN time_slots_time_slot_tags
                NATURAL JOIN time_slots
                WHERE current{}
                ORDER BY duration * LENGTH(days), first_day, start_time, duration",
            dept_clause(departments)
        ))
        .map_err(sql_err)?;
    let mut rows = stmt
        .query(rusqlite::params_from_iter(departments))
        .map_err(sql_err)?;

    while let Some(row) = rows.next().map_err(sql_err)? {
        let time_slot: String = row.get_unwrap(0);
        let Some(caps) = re.captures(&time_slot) else {
            return Err(format!(
                "unrecognized time format: '{}' should be like 'MWF0900+50'",
                time_slot
            ));
        };
        let weekday_part = &caps[1];
        let hour_part = &caps[2];
        let minute_part = &caps[3];
        let length_part = &caps[4];

        // extract days of week
        let days = parse_days(weekday_part)?;

        // get start time
        let start_hour = hour_part.parse::<u8>().unwrap();
        let start_minute = minute_part.parse::<u8>().unwrap();
        let start_time = time::Time::from_hms(start_hour, start_minute, 0).unwrap();
        let length = length_part.parse::<i64>().unwrap();
        let duration = time::Duration::minutes(length);

        // set up the vector of 5-minute intervals used over the term
        let mut slots = Bits::new(date_range_slots(solver.start, solver.end));
        let mut i = 0;
        let mut day = solver.start;
        while day <= solver.end {
            let weekday = day.weekday();
            let active_day = days.contains(&weekday);
            let mut minutes_left = 0;
            for hour in 0..24 {
                for min in (0..60).step_by(5) {
                    if active_day && start_hour == hour && start_minute == min {
                        minutes_left = length;
                    }
                    slots.set(i, minutes_left > 0).unwrap();
                    i += 1;
                    if minutes_left > 0 {
                        minutes_left -= 5;
                    }
                }
            }
            day = day.next_day().unwrap();
        }
        slots.intersect_in_place(&solver.slots)?;

        // check for conflicts
        let mut conflicts = Vec::new();
        let my_index = solver.time_slots.len();
        for (other_index, other) in solver.time_slots.iter_mut().enumerate() {
            if !slots.is_disjoint(&other.slots)? {
                conflicts.push(other_index);
                other.conflicts.push(my_index);
            }
        }

        // a time slot always conflicts with itself
        conflicts.push(solver.time_slots.len());

        solver.time_slots.push(TimeSlot {
            name: time_slot,
            slots,
            days,
            start_time,
            duration,
            conflicts,
            tags: Vec::new(),
        });
    }

    Ok(())
}

// load faculty and their availability
pub fn load_faculty(
    db: &mut Connection,
    solver: &mut Solver,
    departments: &Vec<String>,
) -> Result<(), String> {
    {
        let mut stmt = db.prepare(&format!("
                SELECT DISTINCT faculty,
                                day_of_week, start_time, start_time + duration AS end_time, availability_penalty
                FROM terms
                NATURAL JOIN courses
                NATURAL JOIN sections
                NATURAL JOIN section_time_slot_tags
                NATURAL JOIN faculty_sections 
                NATURAL JOIN faculty_availability
                WHERE current{}
                ORDER BY faculty",
            dept_clause(departments))).map_err(sql_err)?;
        let mut rows = stmt
            .query(rusqlite::params_from_iter(departments))
            .map_err(sql_err)?;

        let mut faculty_name = String::new();
        let mut faculty_index = 0;

        while let Some(row) = rows.next().map_err(sql_err)? {
            let name: String = row.get_unwrap(0);

            // add the faculty if they do not already exist
            if name != faculty_name {
                faculty_name = name.clone();
                faculty_index = solver.instructors.len();

                let mut available_times = Vec::new();
                for _ in 0..7 {
                    available_times.push(vec![-1isize; 24 * 12]);
                }

                solver.instructors.push(Instructor {
                    name: name.clone(),
                    available_times,
                    sections: Vec::new(),
                    distribution: Vec::new(),
                });
            }

            let faculty = &mut solver.instructors[faculty_index];

            // set availability intervals
            // note: it is okay to add these many times from the join
            let day_of_week_s: String = row.get_unwrap(1);
            let day_of_week = match day_of_week_s.as_str() {
                "M" => time::Weekday::Monday,
                "T" => time::Weekday::Tuesday,
                "W" => time::Weekday::Wednesday,
                "R" => time::Weekday::Thursday,
                "F" => time::Weekday::Friday,
                "S" => time::Weekday::Saturday,
                "U" => time::Weekday::Sunday,
                _ => {
                    return Err(format!(
                        "Unknown day of week {day_of_week_s} in {name} faculty_availability"
                    ));
                }
            };
            let start_index = row.get_unwrap::<usize, usize>(2) / 5;
            let end_index = row.get_unwrap::<usize, usize>(3) / 5;
            let day = &mut faculty.available_times[day_of_week.number_days_from_sunday() as usize];
            for elt in day.iter_mut().take(end_index).skip(start_index) {
                *elt = std::cmp::max(*elt, row.get_unwrap(4));
            }
        }
    }

    {
        let mut stmt = db.prepare(&format!("
                SELECT DISTINCT faculty,
                                days_to_check, days_off, days_off_penalty, evenly_spread_penalty, max_gap_within_cluster,
                                is_cluster, is_too_short, interval_minutes, interval_penalty
                FROM terms
                NATURAL JOIN courses
                NATURAL JOIN sections
                NATURAL JOIN section_time_slot_tags
                NATURAL JOIN faculty_sections 
                NATURAL JOIN faculty_preferences
                LEFT OUTER NATURAL JOIN faculty_preference_intervals
                WHERE current{}
                ORDER BY faculty",
            dept_clause(departments))).map_err(sql_err)?;
        let mut rows = stmt
            .query(rusqlite::params_from_iter(departments))
            .map_err(sql_err)?;

        let mut faculty_name = String::new();
        let mut faculty_index = 0;
        let mut clustering_index = None;

        while let Some(row) = rows.next().map_err(sql_err)? {
            let name: String = row.get_unwrap(0);

            // is this the first row for this faculty?
            if name != faculty_name {
                faculty_name = name.clone();
                faculty_index = solver
                    .instructors
                    .iter()
                    .position(|elt| elt.name == faculty_name)
                    .unwrap();

                let faculty = &mut solver.instructors[faculty_index];
                let days_to_check: String = row.get_unwrap(1);

                // days off penalty?
                let days_off: u8 = row.get_unwrap(2);
                let days_off_penalty: isize = row.get_unwrap(3);
                if days_off_penalty > 0 {
                    faculty.distribution.push(DistributionPreference::DaysOff {
                        days: parse_days(&days_to_check)?,
                        days_off,
                        penalty: days_off_penalty,
                    });
                }

                // evenly spread penalty?
                let evenly_spread_penalty: isize = row.get_unwrap(4);
                if evenly_spread_penalty > 0 {
                    faculty
                        .distribution
                        .push(DistributionPreference::DaysEvenlySpread {
                            days: parse_days(&days_to_check)?,
                            penalty: evenly_spread_penalty,
                        });
                }

                // if there is no clustering interval than move on to the next faculty
                if row.get::<usize, bool>(6).is_err() {
                    continue;
                }

                // create the base clustering record
                let max_gap_within_cluster: i64 = row.get_unwrap(5);
                clustering_index = Some(faculty.distribution.len());
                faculty
                    .distribution
                    .push(DistributionPreference::Clustering {
                        days: parse_days(&days_to_check)?,
                        max_gap: time::Duration::minutes(max_gap_within_cluster),
                        cluster_limits: Vec::new(),
                        gap_limits: Vec::new(),
                    });
            }

            let faculty = &mut solver.instructors[faculty_index];

            let is_cluster: bool = row.get_unwrap(6);
            let is_too_short: bool = row.get_unwrap(7);
            let interval_minutes: i64 = row.get_unwrap(8);
            let interval_penalty: isize = row.get_unwrap(9);

            let dwp = if is_too_short {
                DurationWithPenalty::TooShort {
                    duration: time::Duration::minutes(interval_minutes),
                    penalty: interval_penalty,
                }
            } else {
                DurationWithPenalty::TooLong {
                    duration: time::Duration::minutes(interval_minutes),
                    penalty: interval_penalty,
                }
            };

            match &mut faculty.distribution[clustering_index.unwrap()] {
                DistributionPreference::Clustering {
                    cluster_limits,
                    gap_limits,
                    ..
                } => {
                    if is_cluster {
                        cluster_limits.push(dwp);
                    } else {
                        gap_limits.push(dwp);
                    }
                }
                _ => {
                    panic!("I swear it was here a minute ago");
                }
            }
        }
    }

    Ok(())
}

// load sections and the room/time combinations (plus penalties) associated with them
pub fn load_sections(
    db: &mut Connection,
    solver: &mut Solver,
    departments: &Vec<String>,
) -> Result<(), String> {
    {
        let mut stmt = db
            .prepare(&format!("
                    SELECT DISTINCT section, time_slot, MAX(time_slot_penalty)
                    FROM terms
                    NATURAL JOIN courses
                    NATURAL JOIN sections
                    NATURAL JOIN section_time_slot_tags
                    NATURAL JOIN time_slots_time_slot_tags
                    WHERE current{}
                    GROUP BY section, time_slot
                    ORDER BY section",
                dept_clause(departments)
            ))
            .map_err(sql_err)?;
        let mut rows = stmt
            .query(rusqlite::params_from_iter(departments))
            .map_err(sql_err)?;

        let mut prev_name = String::new();

        while let Some(row) = rows.next().map_err(sql_err)? {
            let name: String = row.get_unwrap(0);

            // add the section if it does not already exist
            if name != prev_name {
                prev_name = name.clone();

                let (prefix, course, Some(section)) = parse_section_name(&name)? else {
                    return Err(format!("section name {name} must include prefix, course, and section, like 'CS 1400-01'"));
                };

                solver.input_sections.push(InputSection {
                    prefix,
                    course,
                    section,
                    instructors: Vec::new(),
                    rooms: Vec::new(),
                    time_slots: Vec::new(),
                    hard_conflicts: Vec::new(),
                    soft_conflicts: Vec::new(),
                    cross_listings: Vec::new(),
                    coreqs: Vec::new(),
                    prereqs: Vec::new(),
                });
            }

            let len = solver.input_sections.len();
            let section = &mut solver.input_sections[len - 1];

            let time_slot_name: String = row.get_unwrap(1);
            let time_slot = solver
                .time_slots
                .iter()
                .position(|elt| elt.name == time_slot_name)
                .unwrap();
            let penalty: isize = row.get_unwrap(2);
            section
                .time_slots
                .push(TimeWithPenalty { time_slot, penalty });
        }
    }

    {
        let mut stmt = db
            .prepare(&format!("
                    SELECT DISTINCT section, room, MAX(room_penalty)
                    FROM terms
                    NATURAL JOIN courses
                    NATURAL JOIN sections
                    NATURAL JOIN section_time_slot_tags
                    NATURAL JOIN section_room_tags
                    NATURAL JOIN rooms_room_tags
                    WHERE current{}
                    GROUP BY section, room
                    ORDER BY section",
                dept_clause(departments)
            ))
            .map_err(sql_err)?;
        let mut rows = stmt
            .query(rusqlite::params_from_iter(departments))
            .map_err(sql_err)?;

        let mut prev_name = String::new();
        let mut index = 0;

        while let Some(row) = rows.next().map_err(sql_err)? {
            let name: String = row.get_unwrap(0);

            // look up the section if it is different than the previous one
            if name != prev_name {
                prev_name = name.clone();
                index = find_section_by_name(solver, &name)?;
            }

            let section = &mut solver.input_sections[index];
            let room_name: String = row.get_unwrap(1);
            let room = solver
                .rooms
                .iter()
                .position(|elt| elt.name == room_name)
                .unwrap();
            let penalty: isize = row.get_unwrap(2);

            section.rooms.push(RoomWithPenalty { room, penalty });
        }
    }

    {
        let mut stmt = db
            .prepare(&format!("
                    SELECT DISTINCT faculty, section
                    FROM terms
                    NATURAL JOIN courses
                    NATURAL JOIN sections
                    NATURAL JOIN section_time_slot_tags
                    NATURAL JOIN faculty_sections
                    WHERE current{}
                    ORDER BY faculty",
                dept_clause(departments)
            ))
            .map_err(sql_err)?;
        let mut rows = stmt
            .query(rusqlite::params_from_iter(departments))
            .map_err(sql_err)?;

        let mut faculty_name = String::new();
        let mut faculty_index = 0;

        while let Some(row) = rows.next().map_err(sql_err)? {
            let name: String = row.get_unwrap(0);
            let section_name: String = row.get_unwrap(1);

            // look up the faculty if it is different than the previous one
            if name != faculty_name {
                faculty_name = name.clone();
                faculty_index = solver
                    .instructors
                    .iter()
                    .position(|elt| elt.name == name)
                    .unwrap();
            }

            let section_index = find_section_by_name(solver, &section_name)?;
            solver.instructors[faculty_index]
                .sections
                .push(section_index);
            solver.input_sections[section_index]
                .instructors
                .push(faculty_index);
        }
    }

    Ok(())
}

pub fn load_cross_listings(
    db: &mut Connection,
    solver: &mut Solver,
    departments: &Vec<String>,
) -> Result<(), String> {
    let mut stmt = db
        .prepare(&format!("
                SELECT DISTINCT cross_listing_name, section
                FROM terms
                NATURAL JOIN courses
                NATURAL JOIN sections
                NATURAL JOIN section_time_slot_tags
                NATURAL JOIN cross_listing_sections
                WHERE current{}
                ORDER BY cross_listing_name",
            dept_clause(departments)
        ))
        .map_err(sql_err)?;
    let mut rows = stmt
        .query(rusqlite::params_from_iter(departments))
        .map_err(sql_err)?;

    let mut prev_name = String::new();
    let mut sections = Vec::new();

    while let Some(row) = rows.next().map_err(sql_err)? {
        let name: String = row.get_unwrap(0);

        // start a new cross listing
        if name != prev_name {
            prev_name = name.clone();
            sections.clear();
        }

        let section_name: String = row.get_unwrap(1);
        let section_index = find_section_by_name(solver, &section_name)?;
        sections.push(section_index);

        if sections.len() > 1 {
            sections.sort();
            for &index in &sections {
                solver.input_sections[index].cross_listings = sections.clone();
            }
        }
    }

    Ok(())
}

pub fn load_anti_conflicts(
    db: &mut Connection,
    solver: &mut Solver,
    departments: &Vec<String>,
) -> Result<(), String> {
    let dept_in = dept_multi_clause(departments, &vec!["courses".into(), "other_courses".into()]);
    let mut stmt = db.prepare(&format!("
        SELECT DISTINCT sections.section AS single_section, other_sections.section AS group_section, anti_conflict_penalty
        FROM terms
        NATURAL JOIN courses
        NATURAL JOIN sections
        NATURAL JOIN section_time_slot_tags
        JOIN anti_conflicts
            ON  anti_conflicts.term                             = terms.term
            AND anti_conflicts.anti_conflict_single             = sections.section
        NATURAL JOIN anti_conflict_sections
        JOIN sections                                           AS other_sections
            ON  other_sections.term                             = terms.term
            AND other_sections.section                          = anti_conflict_section
        JOIN courses                                            AS other_courses
            ON  other_courses.term                              = terms.term
            AND other_courses.course                            = other_sections.course
        JOIN section_time_slot_tags                             AS other_section_time_slot_tags
            ON  other_section_time_slot_tags.term               = terms.term
            AND other_section_time_slot_tags.section            = other_sections.section
        WHERE terms.current{dept_in}

        UNION

        SELECT DISTINCT sections.section, other_sections.section, anti_conflict_penalty
        FROM terms
        NATURAL JOIN courses
        NATURAL JOIN sections
        NATURAL JOIN section_time_slot_tags
        JOIN anti_conflicts
            ON  anti_conflicts.term                             = terms.term
            AND anti_conflicts.anti_conflict_single             = sections.section
        NATURAL JOIN anti_conflict_courses
        JOIN courses                                            AS other_courses
            ON  other_courses.term                              = terms.term
            AND other_courses.course                            = anti_conflict_course
        JOIN sections                                           AS other_sections
            ON  other_sections.term                             = terms.term
            AND other_sections.course                           = other_courses.course
        JOIN section_time_slot_tags                             AS other_section_time_slot_tags
            ON  other_section_time_slot_tags.term               = terms.term
            AND other_section_time_slot_tags.section            = other_sections.section
        WHERE terms.current{dept_in}

        ORDER BY sections.section")).map_err(sql_err)?;

    let mut departments_x4 = Vec::new();
    departments_x4.extend(departments);
    departments_x4.extend(departments);
    departments_x4.extend(departments);
    departments_x4.extend(departments);
    let mut rows = stmt
        .query(rusqlite::params_from_iter(departments_x4))
        .map_err(sql_err)?;

    let mut single_name = String::new();
    let mut group = Vec::new();
    let mut penalty = 0;

    while let Some(row) = rows.next().map_err(sql_err)? {
        let name: String = row.get_unwrap(0);
        let other_name: String = row.get_unwrap(1);
        let other = find_section_by_name(solver, &other_name)?;
        let pen: isize = row.get_unwrap(2);
        // start a new anti conflict
        if name != single_name {
            // close the previous one out
            if !single_name.is_empty() && !group.is_empty() {
                let entry = (
                    penalty,
                    find_section_by_name(solver, &single_name)?,
                    std::mem::take(&mut group),
                );
                solver.anticonflicts.push(entry);
            }

            // start a new one
            single_name = name.clone();
            penalty = pen;
        }
        group.push(other);
    }

    // close the final one out
    if !single_name.is_empty() && !group.is_empty() {
        let entry = (
            penalty,
            find_section_by_name(solver, &single_name)?,
            std::mem::take(&mut group),
        );
        solver.anticonflicts.push(entry);
    }

    Ok(())
}

pub fn input() -> Result<Solver, String> {
    let departments = vec!["Computing".to_string(), "Math".to_string()];
    //let departments = vec![];

    let mut db = Connection::open_with_flags(
        DB_PATH,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(sql_err)?;

    let mut solver = make_solver(&mut db)?;
    let mut t = &mut solver;
    println!("loading rooms, times");
    load_rooms(&mut db, t, &departments)?;
    load_time_slots(&mut db, t, &departments)?;
    println!("loading faculty");
    load_faculty(&mut db, t, &departments)?;
    println!("loading sections");
    load_sections(&mut db, t, &departments)?;
    load_cross_listings(&mut db, t, &departments)?;
    load_anti_conflicts(&mut db, t, &departments)?;

    println!("loading conflicts");
    input_computing_conflicts(&mut t)?;
    input_set_conflicts(&mut t)?;
    println!("loading multiples, prereqs");
    input_multiples(&mut t)?;
    input_prereqs(&mut t)?;
    println!("doing postprocessing");

    Ok(solver)
}

pub fn input_computing_conflicts(t: &mut Solver) -> Result<(), String> {
    conflict!(t, set hard,
            clique: "CS 2420", "CS 2450", "CS 2810", "CS 3005"); // 3rd/4th semester classes
    conflict!(t, set hard,
            clique: "CS 2420", "CS 2450", "CS 2810"); // grad plan: 2nd year fall
    conflict!(t, set hard,
            clique: "CS 3005", "CS 3520", "SE 3200"); // grad plan: 2nd year spring
    conflict!(t, set hard,
            clique: "CS 3310", "CS 3400", "CS 3530"); // grad plan: 3nd year fall
    conflict!(t, set hard,
            clique: "CS 3510", "CS 4307", "CS 4550"); // grad plan: 3nd year spring
    conflict!(t, set hard,
            clique: "CS 4300"); // grad plan: 4th year fall
    conflict!(t, set hard,
            clique: "CS 3600", "CS 4600"); // grad plan: 4th year spring

    // CS core
    conflict!(t, set penalty to 99,
            clique: "CS 1030", "CS 1400", "CS 1410", "CS 2420", "CS 2450", "CS 2810", "CS 3005",
                    "CS 3150", "CS 3310", "CS 3400", "CS 3410", "CS 3510", "CS 3520", "CS 3530", "CS 3600",
                    "CS 4300", "CS 4307", "CS 4320", "CS 4550", "CS 4600",
                    "SE 3200");

    // CS electives
    conflict!(t, set penalty to 30,
            clique: "CS 1030", "CS 1400", "CS 1410", "CS 2420", "CS 2450", "CS 2810", "CS 3005",
                    "CS 3150", "CS 3310", "CS 3400", "CS 3410", "CS 3510", "CS 3520", "CS 3530", "CS 3600",
                    "CS 4300", "CS 4307", "CS 4320", "CS 4550", "CS 4600",
                    "SE 3200",
                    "SE 3010", "SE 3020", "SE 3100", "SE 3200", "SE 3400", "SE 4200",
                    "IT 2700", "IT 3100", "IT 3110", "IT 4200");

    // CS math and science
    conflict!(t, set penalty to 50,
            clique: "CS 1030", "CS 1400", "CS 1410", "CS 2420", "CS 2450", "CS 2810", "CS 3005",
                    "CS 3150", "CS 3310", "CS 3400", "CS 3410", "CS 3510", "CS 3520", "CS 3530", "CS 3600",
                    "CS 4300", "CS 4307", "CS 4320", "CS 4550", "CS 4600",
                    "SE 3200",
                    "MATH 1210", "MATH 1220", "BIOL 1610", "BIOL 1615", "PHYS 2210", "PHYS 2215");

    // DS: TODO
    conflict!(t, set penalty to 45,
            clique: "CS 2500", "CS 2810", "CS 3005");

    // SE core
    conflict!(t, set penalty to 99,
            clique: "CS 1030", "CS 1400", "CS 1410", "CS 2420", "CS 2450", "CS 2810",
                    "CS 3150", "CS 3310", "CS 3510", "CS 4307",
                    "IT 2300",
                    "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400",
                    "SE 4200", "SE 4600"); // IT 1100 SE 1400

    // Entrepreneurial and marketing track
    conflict!(t, set penalty to 45,
            clique: "CS 1030", "CS 1400", "CS 1410", "CS 2420", "CS 2450", "CS 2810",
                    "CS 3150", "CS 3310", "CS 3510", "CS 4307",
                    "IT 1100", "IT 2300",
                    "SE 1400", "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400", "SE 3500", "SE 3550",
                    "SE 4200", "SE 4600"); // IT 1100 SE 1400

    // DevOps track
    conflict!(t, set penalty to 45,
            clique: "CS 1030", "CS 1400", "CS 1410", "CS 2420", "CS 2450", "CS 2810",
                    "CS 3150", "CS 3310", "CS 3510", "CS 4307",
                    "IT 2300", "IT 3110", "IT 3300", "IT 4200",
                    "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400",
                    "SE 4200", "SE 4600"); // IT 1100 SE 1400

    // Application track
    conflict!(t, set penalty to 45,
            clique: "CS 1030", "CS 1400", "CS 1410", "CS 2420", "CS 2450", "CS 2810",
                    "CS 3150", "CS 3310", "CS 3500", "CS 3510", "CS 4307",
                    "IT 2300",
                    "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400", "SE 3450",
                    "SE 4200", "SE 4600"); // IT 1100 SE 1400

    // Data science track
    conflict!(t, set penalty to 45,
            clique: "CS 1030", "CS 1400", "CS 1410", "CS 2420", "CS 2450", "CS 2810",
                    "CS 3150", "CS 3310", "CS 3510", "CS 4300", "CS 4307", "CS 4320",
                    "IT 2300",
                    "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400",
                    "SE 4200", "SE 4600"); // IT 1100 SE 1400

    // IT conflicts
    //conflict!(t, set penalty to 50, clique: "IT 1100", "IT 1200"); // when there is only one in-person section of each
    conflict!(t, set penalty to 99,
            clique: "IT 2300", "IT 2400", "IT 2500", "IT 2700",
                    "IT 3100", "IT 3110", "IT 3150", "IT 3300", "IT 3400",
                    "IT 4100", "IT 4200", "IT 4310", "IT 4400", "IT 4510", "IT 4600");

    // IT choose 2 section
    conflict!(t, set penalty to 60,
            clique: "CS 3005",
                    "IT 2300", "IT 2400", "IT 2500", "IT 2700",
                    "IT 3100", "IT 3110", "IT 3150", "IT 3300", "IT 3400",
                    "IT 4100", "IT 4200", "IT 4310", "IT 4400", "IT 4510", "IT 4600",
                    "SE 3200", "SE 3400");

    conflict!(t, remove penalty, clique: "CS 4307", "IT 2300"); // students take either CS4307 or IT2300 but not both so no conflict

    // TODO:
    // should anticonflict automatically zero out any penalty? maybe as a later pass?
    //anticonflict!(t, set penalty to 50, clique: "SE 1400", "IT 1100"); // temporarily removed because of new hire planning

    Ok(())
}

pub fn input_set_conflicts(t: &mut Solver) -> Result<(), String> {
    // envs envs emphasis
    conflict!(t, set penalty to 99,
            clique: "ENVS 1210", "ENVS 1215",
                    "ENVS 2210",
                    "GEO 1110", "GEO 1115",
                    "GEOG 3600", "GEOG 3605",
                    "CHEM 1210", "CHEM 1215",
                    "CHEM 1220", "CHEM 1225",
                    "BIOL 1610", "BIOL 1615",
                    "MATH 1060",
        
                    "ENVS 2700R",
                    "ENVS 4910",
                    "ENVS 3920");

    // envs geo emphasis
    conflict!(t, set penalty to 99,
            clique: "ENVS 1210", "ENVS 1215",
                    "ENVS 2210",
                    "GEO 1110", "GEO 1115",
                    "GEOG 3600", "GEOG 3605",
                    "CHEM 1210", "CHEM 1215",
                    "CHEM 1220", "CHEM 1225",
                    "BIOL 1610", "BIOL 1615",
                    "MATH 1060",

                    "GEO 1220", "GEO 1225",
                    "GEO 2700R");

    // remove penalty between classes and their prereqs
    add_prereqs!(t, course: "CHEM 1210", prereqs: "MATH 1050");
    add_prereqs!(t, course: "CHEM 1210", prereqs: "MATH 1050");
    add_prereqs!(t, course: "CHEM 1210", prereqs: "MATH 1050");
    add_prereqs!(t, course: "ENVS 2700R", prereqs: "ENVS 1210", "ENVS 1215");
    add_prereqs!(t, course: "GEO 1220", prereqs: "GEO 1110", "GEO 1115");
    add_prereqs!(t, course: "GEO 12250", prereqs: "GEO 1110", "GEO 1115");
    add_prereqs!(t, course: "MATH 1060", prereqs: "MATH 1050");

    ////reduce scores by section count + lil more
    //conflict!(t, two section reduction: "BIOL 1610");
    //conflict!(t, two section reduction: "ENVS 1215");
    //conflict!(t, two section reduction: "MATH 1060");
    //conflict!(t, three section reduction: "CHEM 1210");
    //conflict!(t, three section reduction: "CHEM 1215");
    //
    ////multiple section scheduling conflict with themselves
    //conflict!(t, set hard, clique: "CHEM 1210-01", "CHEM 1210-02");
    //conflict!(t, set hard, clique: "CHEM 1220-01", "CHEM 1220-02");
    //conflict!(t, set hard, clique: "BIOL 1610-01", "BIOL 1610-02");
    //
    ////class and coreq lab conflict
    //conflict!(t, set hard, clique: "ENVS 1210-01", "ENVS 1215-01", "ENVS 1215-02");
    //conflict!(t, set hard, clique: "GEO 1110", "GEO 1115");
    //conflict!(t, set hard, clique: "GEO 1220", "GEO 1225");
    //conflict!(t, set hard, clique: "GEOG 3600", "GEOG 3605");

    // envs envs emphasis
    conflict!(t, set penalty to 99,
            clique: "ENVS 1210", "ENVS 1215", "ENVS 2210",
                    "GEO 1110", "GEO 1115",
                    "GEOG 3600", "GEOG 3605",
                    "CHEM 1210", "CHEM 1220",
                    "BIOL 1610",
                    "MATH 1060",
                    "ENVS 2700R", "ENVS 4910", "ENVS 3920");

    // envs geo emphasis
    conflict!(t, set penalty to 99,
            clique: "ENVS 1210", "ENVS 1215", "ENVS 2210",
                    "GEO 1110", "GEO 1115",
                    "GEOG 3600", "GEOG 3605",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "BIOL 1610", "BIOL 1615",
                    "MATH 1060",
                    "GEO 1220", "GEO 1225", "GEO 2700R");

    // geological sciences
    conflict!(t, set penalty to 99,
            clique: "BIOL 3110",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "GEO 1110", "GEO 1115", "GEO 1220", "GEO 1225", "GEO 2700R", "GEO 2990R",
                    "GEO 3060", "GEO 3180", "GEO 3200", "GEO 3500", "GEO 3550",
                    "GEO 3600", "GEO 3700", "GEO 3710", "GEO 4600", "GEO 4800R",
                    "GEOG 3600", "GEOG 3605",
                    "MATH 1210",
                    "PHYS 2010", "PHYS 2015", "PHYS 2210", "PHYS 2215",
                    "PHYS 2020", "PHYS 2025", "PHYS 2220", "PHYS 2225",
                    "GEO 3000", "GEO 3910",
                    "ENVS 3910", "ENVS 3920", "ENVS 3930",
                    "GEOG 3930");

    // bioinformatics core
    conflict!(t, set penalty to 99,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625",
                    "BIOL 3010", "BIOL 3300", "BIOL 3030", "BIOL 4010", "BIOL 4300",
                    "BIOL 4305", "BIOL 4310", "BIOL 4320", "BIOL 4810R", "BIOL 4910",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "CS 1400", "CS 1410", "CS 2420", "CS 2450", "CS 3310",
                    "IT 1100", "IT 2300",
                    "MATH 1210", "MATH 3060");
    // bioinformatics pick one tech lab course
    conflict!(t, set penalty to 30,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625",
                    "BIOL 3010", "BIOL 3300", "BIOL 3030", "BIOL 4010", "BIOL 4300",
                    "BIOL 4305", "BIOL 4310", "BIOL 4320", "BIOL 4810R", "BIOL 4910",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "CS 1400", "CS 1410", "CS 2420", "CS 2450", "CS 3310",
                    "IT 1100", "IT 2300",
                    "MATH 1210", "MATH 3060",

                    "BTEC 2010", "BTEC 2020", "BTEC 2030", "BTEC 2050", "BIOL 2300");

    //bio_education emphasis
    conflict!(t, set penalty to 99,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "BIOL 3010", "BIOL 3030", "HIST 1700", "POLS 1100", "FSHD 1500", "PSY 1010", "PSY 1100",
                    "EDUC 1010", "EDUC 2010", "EDUC 2400", "EDUC 2500", "EDUC 3110", "EDUC 2700", "MATH 1050",
                    "BIOL 2320", "BIOL 2325", "BIOL 3140", "BIOL 3145", "BIOL 2420", "BIOL 2425", "BIOL 4500", "BIOL 4505",
                    "BIOL 3040", "BIOL 3045", "BIOL 2060", "BIOL 2065", "BIOL 3450", "BIOL 3455", "BIOL 3550", "BIOL 3555",
                    "BIOL 2400", "BIOL 2405", "BIOL 3200", "BIOL 3205", "BIOL 4260", "BIOL 4265", "BIOL 4270",
                    "BIOL 4275", "BIOL 4350", "BIOL 4355", "BIOL 4380", "BIOL 4385", "BIOL 4411", "BIOL 4415", "BIOL 4440",
                    "SCI 2600", "SCI 4700",
                    "SCED 3720", "SCED 4100", "SCED 4200", "SCED 4600", "SCED 4300", "SCED 4900", "SCED 4989");

    //bio bio-sciences
    conflict!(t, set penalty to 99,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 3010", "BIOL 3030",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315", "CHEM 2320", "CHEM 2325",
                    "MATH 1210",
                    "BIOL 3040", "BIOL 3045", "BIOL 3155",
                    "MATH 3060",
                    "BIOL 4910");
    conflict!(t, set penalty to 45,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 3010", "BIOL 3030",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315", "CHEM 2320", "CHEM 2325",
                    "MATH 1210",
                    "BIOL 3040", "BIOL 3045", "BIOL 3155",
                    "MATH 3060",
                    "BIOL 4910",

                    "PHYS 2010", "PHYS 2015", "PHYS 2020", "PHYS 2025", "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225",

                    "BTEC 2010", "BTEC 2020", "BTEC 2030", "BTEC 2050", "BIOL 2300",

                    "BIOL 3450", "BIOL 3455", "BIOL 3550", "BIOL 3555",

                    "BIOL 3420", "BIOL 4500", "BIOL 4505", "BIOL 4600", "BIOL 4605",

                    "BIOL 3200", "BIOL 3205", "BIOL 4260", "BIOL 4265", "BIOL 4270", "BIOL 4275",
                    "BIOL 4280", "BIOL 4350", "BIOL 4355", "BIOL 4380", "BIOL 4385", "BIOL 4411", "BIOL 4415", "BIOL 4440");
    conflict!(t, set penalty to 30,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 3010", "BIOL 3030",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315", "CHEM 2320", "CHEM 2325",
                    "MATH 1210",
                    "BIOL 3040", "BIOL 3045", "BIOL 3155",
                    "MATH 3060",
                    "BIOL 4910",

                    "PHYS 2010", "PHYS 2015", "PHYS 2020", "PHYS 2025", "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225",

                    "BTEC 2010", "BTEC 2020", "BTEC 2030", "BTEC 2050", "BIOL 2300",

                    "BIOL 3450", "BIOL 3455", "BIOL 3550", "BIOL 3555",

                    "BIOL 3420", "BIOL 4500", "BIOL 4505", "BIOL 4600", "BIOL 4605",

                    "BIOL 3200", "BIOL 3205", "BIOL 4260", "BIOL 4265", "BIOL 4270", "BIOL 4275",
                    "BIOL 4280", "BIOL 4350", "BIOL 4355", "BIOL 4380", "BIOL 4385", "BIOL 4411", "BIOL 4415", "BIOL 4440",

                    "BTEC 3020", "CHEM 3510", "CHEM 3515", "CHEM 3520", "CHEM 3525",
                    "BTEC 3010", "BTEC 3040", "BTEC 3050", "BTEC 4020", "BTEC 4040", "BTEC 4050", "BTEC 4060");

    //bio biomed
    conflict!(t, set penalty to 99,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 3010", "BIOL 3030", "BIOL 3040",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315",
                    "CHEM 2320", "CHEM 2325", "CHEM 3510", "CHEM 3515",
                    "PHYS 2010", "PHYS 2015", "PHYS 2020", "PHYS 2025",
                    "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225",
                    "BIOL 2320", "BIOL 2325", "BIOL 3420",
                    "MATH 3060",
                    "BIOL 3155", "BIOL 3450", "BIOL 3455", "BIOL 3550", "BIOL 3555", "BIOL 4910",
                    "BTEC 2010", "BTEC 2020", "BTEC 2030", "BTEC 2050",
                    "BIOL 2300",
                    "PSY 2400", "PSY 3460", "PSY 3710");
    conflict!(t, set penalty to 30,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 3010", "BIOL 3030", "BIOL 3040",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315",
                    "CHEM 2320", "CHEM 2325", "CHEM 3510", "CHEM 3515",
                    "PHYS 2010", "PHYS 2015", "PHYS 2020", "PHYS 2025",
                    "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225",
                    "BIOL 2320", "BIOL 2325", "BIOL 3420",
                    "MATH 3060",
                    "BIOL 3155", "BIOL 3450", "BIOL 3455", "BIOL 3550", "BIOL 3555", "BIOL 4910",
                    "BTEC 2010", "BTEC 2020", "BTEC 2030", "BTEC 2050",
                    "BIOL 2300",
                    "PSY 2400", "PSY 3460", "PSY 3710",

                    "BIOL 3000R", "BIOL 3100", "BIOL 3110", "BIOL 3120", "BIOL 3140", "BIOL 3145",
                    "BIOL 3230R", "BIOL 3250", "BIOL 3360", "BIOL 3460", "BIOL 3470",
                    "BIOL 4300", "BIOL 4305", "BIOL 4440", "BIOL 4930R",
                    "CHEM 3520", "CHEM 3525",
                    "MATH 1210");

    //bio natural sciences
    conflict!(t, set penalty to 99,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 2400", "BIOL 2405",
                    "BIOL 3010", "BIOL 3030", "BIOL 3040", "BIOL 3045", "BIOL 3110", "BIOL 3120", "BIOL 4910",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "ENVS 1210", "ENVS 1215",
                    "GEO 1110", "GEO 1115",
                    "GEOG 3600", "GEOG 3605",
                    "MATH 1040", "MATH 1050",
                    "PHYS 1010", "PHYS 1015", "PHYS 2010", "PHYS 2015");
    conflict!(t, set penalty to 45,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 2400", "BIOL 2405",
                    "BIOL 3010", "BIOL 3030", "BIOL 3040", "BIOL 3045", "BIOL 3110", "BIOL 3120", "BIOL 4910",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "ENVS 1210", "ENVS 1215",
                    "GEO 1110", "GEO 1115",
                    "GEOG 3600", "GEOG 3605",
                    "MATH 1040", "MATH 1050",
                    "PHYS 1010", "PHYS 1015", "PHYS 2010", "PHYS 2015",
                    "BIOL 3200", "BIOL 3340", "BIOL 3345", "BIOL 4200", "BIOL 4205", "BIOL 4260",
                    "BIOL 4265", "BIOL 4270", "BIOL 4275", "BIOL 4280", "BIOL 4350", "BIOL 4355",
                    "BIOL 4380", "BIOL 4385", "BIOL 4411", "BIOL 4415", "BIOL 4440", "BIOL 4600", "BIOL 4605");
    conflict!(t, set penalty to 30,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 2400", "BIOL 2405",
                    "BIOL 3010", "BIOL 3030", "BIOL 3040", "BIOL 3045", "BIOL 3110", "BIOL 3120", "BIOL 4910",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "ENVS 1210", "ENVS 1215",
                    "GEO 1110", "GEO 1115",
                    "GEOG 3600", "GEOG 3605",
                    "MATH 1040", "MATH 1050",
                    "PHYS 1010", "PHYS 1015", "PHYS 2010", "PHYS 2015",
                    "BIOL 3200", "BIOL 3340", "BIOL 3345", "BIOL 4200", "BIOL 4205", "BIOL 4260",
                    "BIOL 4265", "BIOL 4270", "BIOL 4275", "BIOL 4280", "BIOL 4350", "BIOL 4355",
                    "BIOL 4380", "BIOL 4385", "BIOL 4411", "BIOL 4415", "BIOL 4440", "BIOL 4600", "BIOL 4605",
                    "BIOL 3100", "BIOL 3140", "BIOL 3145", "BIOL 3250", "BIOL 3360", "BIOL 3450", "BIOL 3455",
                    "BIOL 3550", "BIOL 3555", "BIOL 4300", "BIOL 4305", "BIOL 4500", "BIOL 4505",
                    "BIOL 4810R", "BIOL 4930R",
                    "GEOG 4140", "GEOG 4180",
                    "MATH 1210", "MATH 3060",
                    "BIOL 3155");

    //bio integrated edu sciences
    conflict!(t, set penalty to 99,
            clique: "HIST 1700", "POLS 1100", "FSHD 1500", "PSY 1010", "PSY 1100",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "PHYS 2010", "PHYS 2015",
                    "MATH 1050", "MATH 1060", "MATH 1080",
                    "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 2320", "BIOL 2325",
                    "BIOL 3140", "BIOL 3145", "BIOL 2420", "BIOL 2425", "BIOL 4500", "BIOL 4505",
                    "BIOL 3010", "BIOL 3030", "BIOL 3040", "BIOL 3045", "BIOL 2060", "BIOL 2065",
                    "BIOL 3450", "BIOL 3455", "BIOL 3550", "BIOL 3555",
                    "BIOL 2400", "BIOL 2405", "BIOL 3200", "BIOL 3205",
                    "BIOL 4260", "BIOL 4265", "BIOL 4270", "BIOL 4275", "BIOL 4350", "BIOL 4355",
                    "BIOL 4380", "BIOL 4385", "BIOL 4411", "BIOL 4415", "BIOL 4440",
                    "GEO 1110", "GEO 1115",
                    "PHYS 1040", "PHYS 1045",
                    "SCI 2600",
                    "EDUC 1010", "EDUC 2010", "EDUC 2400", "EDUC 2500", "EDUC 3110",
                    "SCI 4700",
                    "SCED 3720", "SCED 4100", "SCED 4200", "SCED 4600", "SCED 4300", "SCED 4900", "SCED 4989");

    //chemistry chemistry major
    conflict!(t, set penalty to 99,
            clique: "MATH 1210", "MATH 1220",
                    "BIOL 1610", "BIOL 1615",
                    "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "CHEM 2310", "CHEM 2315", "CHEM 2320", "CHEM 2325", "CHEM 2600", "CHEM 2990R",
                    "CHEM 3000", "CHEM 3005", "CHEM 3060", "CHEM 3065", "CHEM 3070", "CHEM 3075",
                    "CHEM 3100", "CHEM 3300", "CHEM 3510", "CHEM 3515", "CHEM 3520", "CHEM 3525",
                    "CHEM 4100", "CHEM 4800R", "CHEM 4910", "CHEM 4200", "CHEM 4310", "CHEM 4510", "CHEM 4610");
    conflict!(t, set penalty to 30,
            clique: "MATH 1210", "MATH 1220",
                    "BIOL 1610", "BIOL 1615",
                    "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225",
                    "MATH 2210", "MATH 2250", "MATH 2270", "MATH 2280", "MATH 3060",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "CHEM 2310", "CHEM 2315", "CHEM 2320", "CHEM 2325", "CHEM 2600", "CHEM 2990R",
                    "CHEM 3000", "CHEM 3005", "CHEM 3060", "CHEM 3065", "CHEM 3070", "CHEM 3075",
                    "CHEM 3100", "CHEM 3300", "CHEM 3510", "CHEM 3515", "CHEM 3520", "CHEM 3525",
                    "CHEM 4100", "CHEM 4800R", "CHEM 4910", "CHEM 4200", "CHEM 4310", "CHEM 4510", "CHEM 4610");

    //chem molecular biology
    conflict!(t, set penalty to 99,
            clique: "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315",
                    "CHEM 2320", "CHEM 2325", "CHEM 2600", "CHEM 2990R",
                    "CHEM 3000", "CHEM 3005", "CHEM 3060", "CHEM 3065", "CHEM 3070", "CHEM 3075", "CHEM 3100",
                    "CHEM 3300", "CHEM 3510", "CHEM 3515", "CHEM 3520", "CHEM 3525", "CHEM 4910",
                    "BIOL 1610", "BIOL 1615", "BIOL 3030", "BIOL 3550", "BIOL 3555", "BIOL 4300", "BIOL 4305",
                    "MATH 1210", "MATH 1220",
                    "PHYS 2010", "PHYS 2015", "PHYS 2020", "PHYS 2025",
                    "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225",
                    "CHEM 4800R",
                    "BIOL 4810R", "BIOL 4890R");
    conflict!(t, set penalty to 30,
            clique: "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315",
                    "CHEM 2320", "CHEM 2325", "CHEM 2600", "CHEM 2990R",
                    "CHEM 3000", "CHEM 3005", "CHEM 3060", "CHEM 3065", "CHEM 3070", "CHEM 3075", "CHEM 3100",
                    "CHEM 3300", "CHEM 3510", "CHEM 3515", "CHEM 3520", "CHEM 3525", "CHEM 4910",
                    "BIOL 1610", "BIOL 1615", "BIOL 3030", "BIOL 3550", "BIOL 3555", "BIOL 4300", "BIOL 4305",
                    "MATH 1210", "MATH 1220",
                    "PHYS 2010", "PHYS 2015", "PHYS 2020", "PHYS 2025",
                    "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225",
                    "CHEM 4800R",
                    "BIOL 4810R", "BIOL 4890R",

                    "CHEM 4100", "CHEM 4610",
                    "BIOL 3010", "BIOL 3250", "BIOL 3360", "BIOL 3420",
                    "BIOL 3450", "BIOL 3455", "BIOL 3470", "BIOL 3460", "BIOL 4400");

    //chem physical sciences
    conflict!(t, set penalty to 99,
            clique: "SCI 4700",
                    "SCED 3720", "SCED 4100", "SCED 4200", "SCED 4600", "SCED 4300", "SCED 4900", "SCED 4989",
                    "HIST 1700", "POLS 1100", "FSHD 1500", "PSY 1010", "PSY 1100",
                    "EDUC 1010", "EDUC 2010", "EDUC 2400", "EDUC 2500", "EDUC 3110",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315", "CHEM 3000",
                    "GEO 1110", "GEO 1115", "GEO 1220", "GEO 1225", "GEO 3060",
                    "PHYS 1040", "PHYS 1045", "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225", "PHYS 3710",
                    "BIOL 1610", "BIOL 1615",
                    "MATH 1210", "MATH 1220",
                    "SCI 2600", "SCI 4800R",
                    "CHEM 3510",
                    "PHYS 3400");

    // math bs
    conflict!(t, set penalty to 99,
            clique: "MATH 1210", "MATH 1220", "MATH 2200", "MATH 2210", "MATH 2270", "MATH 2280",
					"MATH 3200", "MATH 3400", "MATH 3900", "MATH 4000", "MATH 4900",
					"CS 1400",
					"PHYS 2210", "PHYS 2215");
    conflict!(t, set penalty to 30,
            clique: "MATH 1210", "MATH 1220", "MATH 2200", "MATH 2210", "MATH 2270", "MATH 2280",
					"MATH 3200", "MATH 3400", "MATH 3900", "MATH 4000", "MATH 4900",
					"CS 1400",
					"PHYS 2210", "PHYS 2215",
					"MATH 3000", "MATH 3100", "MATH 3150", "MATH 3210", "MATH 3450",
					"MATH 3500", "MATH 3605", "MATH 3700",
					"MATH 4010", "MATH 4100", "MATH 4200", "MATH 4250", "MATH 4550",
					"MATH 4800", "MATH 4890R",
                    "PHYS 2220", "PHYS 2225");

    // math acm data analytics
    conflict!(t, set penalty to 99,
            clique: "CS 1400", "CS 1410",
					"MATH 1210", "MATH 1220", "MATH 2200", "CS 3310",
					"MATH 2210", "MATH 2270", "MATH 2280", "MATH 3400", "MATH 3700",
					"MATH 4250", "MATH 4800", "MATH 4890R", "MATH 4900",
					"COMM 1020",
					"MATH 2050", "MATH 3050", "MATH 3450",
					"ISA 2010", "ISA 3020", "ISA 4060", "ISA 4070",
					"IT 1100", "IT 2300", "IT 2400", "IT 4310");
    conflict!(t, set penalty to 30,
            clique: "CS 1400", "CS 1410",
					"MATH 1210", "MATH 1220", "MATH 2200", "CS 3310",
					"MATH 2210", "MATH 2270", "MATH 2280", "MATH 3400", "MATH 3700",
					"MATH 4250", "MATH 4800", "MATH 4890R", "MATH 4900",
					"COMM 1020",
					"MATH 2050", "MATH 3050", "MATH 3450",
					"ISA 2010", "ISA 3020", "ISA 4060", "ISA 4070",
					"IT 1100", "IT 2300", "IT 2400", "IT 4310",
					"CS 3005", "FIN 4380", "IT 4510",
					"MATH 3100", "MATH 3150", "MATH 3120", "MATH 3200", "MATH 3500",
					"MATH 3900", "MATH 3905", "MATH 4000", "MATH 4005", "MATH 4010",
					"MATH 4100", "MATH 4200", "MATH 4330", "MATH 4550", "MATH 4890R",
					"MGMT 4040",
					"XSCI 3800");

    // math education
    conflict!(t, set penalty to 99,
            clique: "MATH 1040", "MATH 1210", "MATH 1220", "MATH 2200", "MATH 2210",
					"MATH 2270", "MATH 2280", "MATH 3000", "MATH 3010", "MATH 3020",
					"MATH 3100", "MATH 3120", "MATH 3200", "MATH 3400", "MATH 4000",
					"CS 1400",
					"PHYS 2210", "PHYS 2215");

    // math acm comp math
    conflict!(t, set penalty to 99,
            clique: "CS 1400", "CS 1410",
					"MATH 1210", "MATH 1220", "MATH 2200", "CS 3310",
					"MATH 2210", "MATH 2270", "MATH 2280", "MATH 3400", "MATH 3700",
					"MATH 4250", "MATH 4800", "MATH 4890R", "MATH 4900",
					"CS 2420", "CS 3005",
					"COMM 1020",
					"MATH 2050", "MATH 3150", "MATH 3500", "MATH 4550",
					"MECH 2010", "MECH 2030",
					"PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225");
    conflict!(t, set penalty to 30,
            clique: "CS 1400", "CS 1410",
					"MATH 1210", "MATH 1220", "MATH 2200", "CS 3310",
					"MATH 2210", "MATH 2270", "MATH 2280", "MATH 3400", "MATH 3700",
					"MATH 4250", "MATH 4800", "MATH 4890R", "MATH 4900",
					"CS 2420", "CS 3005",
					"COMM 1020",
					"MATH 2050", "MATH 3150", "MATH 3500", "MATH 4550",
					"MECH 2010", "MECH 2030",
					"PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225",
					"MATH 3050", "MATH 3450", "MATH 3120", "MATH 3100", "MATH 3900", "MATH 3905",
					"MATH 4000", "MATH 4005", "MATH 4010", "MATH 4100", "MATH 4330",
					"MATH 3200", "MATH 4200",
					"MECH 3600", "MECH 3700", "MECH 3705",
					"PHYS 3400", "PHYS 3710",
					"XSCI 3800");

    // math acm actuarial science
    conflict!(t, set penalty to 99,
            clique: "CS 1400", "CS 1410",
					"MATH 1210", "MATH 1220", "MATH 2200", "CS 3310",
					"MATH 2210", "MATH 2270", "MATH 2280", "MATH 3400", "MATH 3700",
					"MATH 4250", "MATH 4800", "MATH 4890R", "MATH 4900",
					"ACCT 2010", "ACCT 2020",
					"ISA 2010",
					"COMM 1020",
					"CS 2420",
					"ECON 2010", "ECON 2020",
					"FIN 3150",
					"ISA 3020",
					"MATH 3410", "MATH 3450", "MATH 4400", "MATH 4410",
					"STAT 2040");
    conflict!(t, set penalty to 30,
            clique: "CS 1400", "CS 1410",
					"MATH 1210", "MATH 1220", "MATH 2200", "CS 3310",
					"MATH 2210", "MATH 2270", "MATH 2280", "MATH 3400", "MATH 3700",
					"MATH 4250", "MATH 4800", "MATH 4890R", "MATH 4900",
					"ACCT 2010", "ACCT 2020",
					"ISA 2010",
					"COMM 1020",
					"CS 2420",
					"ECON 2010", "ECON 2020",
					"FIN 3150",
					"ISA 3020",
					"MATH 3410", "MATH 3450", "MATH 4400", "MATH 4410",
					"STAT 2040",
					"ECON 3010", "ECON 3020", "ECON 3500",
					"FIN 4380",
					"MGMT 4040",
					"MATH 3050", "MATH 3120", "MATH 3150", "MATH 3200", "MATH 3100",
					"MATH 3500", "MATH 3900", "MATH 3905", "MATH 4000", "MATH 4005",
					"MATH 4200", "MATH 4010", "MATH 4100", "MATH 4330", "MATH 4550");

    // complete one of the following sets, etc.
    // note: conflicts between coreqs will be reinstituted later
    conflict!(t, remove penalty,
            clique: "PHYS 2010", "PHYS 2015", "PHYS 2020", "PHYS 2025",
                    "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225");

    // complete one technical lab course
    conflict!(t, remove penalty,
            clique: "BTEC 2010", "BTEC 2020", "BTEC 2030", "BTEC 2050", "BIOL 2300");

    // complete one of the following sets, etc.
    // note: conflicts between coreqs will be reinstituted later
    conflict!(t, remove penalty,
            clique: "BIOL 3450", "BIOL 3455", "BIOL 3550", "BIOL 3555");

    // complete one of the following sets, etc.
    // note: conflicts between coreqs will be reinstituted later
    conflict!(t, remove penalty,
            clique: "BIOL 3420", "BIOL 4500", "BIOL 4505", "BIOL 4600", "BIOL 4605");

    // complete one of the following sets, etc.
    // note: conflicts between coreqs will be reinstituted later
    conflict!(t, remove penalty,
            clique: "BIOL 3200", "BIOL 3205", "BIOL 4260", "BIOL 4265", "BIOL 4270", "BIOL 4275",
                    "BIOL 4280", "BIOL 4350", "BIOL 4355", "BIOL 4380", "BIOL 4385", "BIOL 4411", "BIOL 4415", "BIOL 4440",
    );
    conflict!(t, remove penalty, clique: "MATH 1050", "MATH 1080");
    conflict!(t, remove penalty, clique: "MATH 1050", "MATH 1080");

    // complete one of the following sets, etc.
    // note: conflicts between coreqs will be reinstituted later
    conflict!(t, remove penalty, clique: "BIOL 2320", "BIOL 2325", "BIOL 3140", "BIOL 3145");
    conflict!(t, remove penalty, clique: "BIOL 2420", "BIOL 2425", "BIOL 4500", "BIOL 4505");
    conflict!(t, remove penalty, clique: "BIOL 2060", "BIOL 2065", "BIOL 3450", "BIOL 3455", "BIOL 3550", "BIOL 3555");

    //complete one of the following
    conflict!(t,remove penalty, clique: "CHEM 2310", "CHEM 2315", "CHEM 3000");
    conflict!(t,remove penalty, clique: "CHEM 3510", "PHYS 3400");

    // either math discrete math or cs discrete math
    conflict!(t, remove penalty, clique: "MATH 2200", "CS 3310");

    // take two classes or one that fill the same prereqs
    conflict!(t, remove penalty, clique: "MATH 1050", "MATH 1080");
    conflict!(t, remove penalty, clique: "MATH 1060", "MATH 1080");
    conflict!(t, remove penalty, clique: "MATH 2050", "MATH 3060");
    conflict!(t, remove penalty, clique: "MATH 2270", "MATH 2250");
    conflict!(t, remove penalty, clique: "MATH 2280", "MATH 2250");

    Ok(())
}

pub fn input_prereqs(t: &mut Solver) -> Result<(), String> {
    add_prereqs!(t, course: "CS 1400", prereqs: "CS 1030", "MATH 1010");
    add_prereqs!(t, course: "CS 1410", prereqs: "CS 1400");
    add_prereqs!(t, course: "CS 2420", prereqs: "CS 1410");
    add_prereqs!(t, course: "CS 2450", prereqs: "CS 1410");
    add_prereqs!(t, course: "CS 2500", prereqs: "CS 1410");
    add_prereqs!(t, course: "CS 2810", prereqs: "CS 1410");
    add_prereqs!(t, course: "CS 3005", prereqs: "CS 1410");
    add_prereqs!(t, course: "CS 3150", prereqs: "CS 2420", "CS 2810");
    add_prereqs!(t, course: "CS 3310", prereqs: "CS 1410", "MATH 1100", "MATH 1210");
    add_prereqs!(t, course: "CS 3400", prereqs: "CS 2420", "CS 2810", "CS 3005");
    add_prereqs!(t, course: "CS 3410", prereqs: "CS 2420", "CS 2810");
    add_prereqs!(t, course: "CS 3500", prereqs: "CS 3005");
    add_prereqs!(t, course: "CS 3510", prereqs: "CS 2420", "CS 2810", "CS 3310");
    add_prereqs!(t, course: "CS 3520", prereqs: "CS 2420", "CS 2810");
    add_prereqs!(t, course: "CS 3530", coreqs: "CS 3310", prereqs: "CS 2420", "CS 2810", "CS 3310");
    add_prereqs!(t, course: "CS 3600", prereqs: "CS 2420", "CS 3005");
    add_prereqs!(t, course: "CS 4300", prereqs: "CS 2420", "CS 2810", "CS 3005");
    add_prereqs!(t, course: "CS 4307", prereqs: "CS 2420", "CS 2810");
    add_prereqs!(t, course: "CS 4310", prereqs: "CS 4307", "IT 2300");
    add_prereqs!(t, course: "CS 4320", prereqs: "CS 2420", "CS 2810", "CS 3005");
    add_prereqs!(t, course: "CS 4400", prereqs: "CS 2420", "CS 2810");
    add_prereqs!(t, course: "CS 4410", prereqs: "CS 2420", "CS 2810");
    add_prereqs!(t, course: "CS 4550", prereqs: "CS 2420", "CS 2810", "CS 3005");
    add_prereqs!(t, course: "CS 4600", prereqs: "CS 2420", "CS 2810", "CS 3005"); // sorta
    add_prereqs!(t, course: "CS 4991R", prereqs: "CS 1400");
    add_prereqs!(t, course: "CS 4992R", prereqs: "CS 2420", "CS 2810");

    add_prereqs!(t, course: "SE 3010", prereqs: "CS 2420", "CS 3005");
    add_prereqs!(t, course: "SE 3020", prereqs: "CS 2420", "CS 3005");
    add_prereqs!(t, course: "SE 3100", prereqs: "CS 2450");
    add_prereqs!(t, course: "SE 3150", prereqs: "CS 2450");
    add_prereqs!(t, course: "SE 3200", prereqs: "CS 1410", "SE 1400", "CS 2810");
    add_prereqs!(t, course: "SE 3400", prereqs: "SE 1400");
    add_prereqs!(t, course: "SE 3450", prereqs: "SE 1400");
    add_prereqs!(t, course: "SE 4600", prereqs: "CS 2420", "CS 2450", "CS 2810", "CS 3005", "SE 1400", "SE 3200"); // sorta
    add_prereqs!(t, course: "SE 4200", prereqs: "SE 3200");

    add_prereqs!(t, course: "IT 2300", prereqs: "CS 1400", "IT 1100", "CS 1410");
    add_prereqs!(t, course: "IT 2400", coreqs: "IT 1100", "IT 1500", prereqs: "IT 1100", "IT 1500");
    add_prereqs!(t, course: "IT 2500", prereqs: "IT 2400");
    add_prereqs!(t, course: "IT 2700", prereqs: "CS 1400", "IT 2400");
    add_prereqs!(t, course: "IT 3100", prereqs: "CS 1400", "IT 1100", "IT 2400", "CS 3150");
    add_prereqs!(t, course: "IT 3110", prereqs: "CS 1410", "IT 3100");
    add_prereqs!(t, course: "IT 3150", prereqs: "IT 2400");
    add_prereqs!(t, course: "IT 3300", prereqs: "IT 2400", "IT 1100", "CS 3150");
    add_prereqs!(t, course: "IT 3400", prereqs: "IT 2400");
    add_prereqs!(t, course: "IT 4100", prereqs: "IT 3100");
    add_prereqs!(t, course: "IT 4200", prereqs: "CS 1400", "IT 2400", "CS 2810");
    add_prereqs!(t, course: "IT 4310", prereqs: "IT 2300");
    add_prereqs!(t, course: "IT 4400", prereqs: "IT 3400");
    add_prereqs!(t, course: "IT 4600", prereqs: "CS 1410", "IT 2400"); // sorta
    add_prereqs!(t, course: "IT 4510", prereqs: "CS 1410", "IT 3100");

    // scraped data
    add_prereqs!(t, course: "BIOL 1610", coreqs: "BIOL 1615");
    add_prereqs!(t, course: "BIOL 1615", coreqs: "BIOL 1610");
    add_prereqs!(t, course: "BIOL 1620", coreqs: "BIOL 1625", prereqs: "BIOL 1610");
    add_prereqs!(t, course: "BIOL 1625", coreqs: "BIOL 1620", prereqs: "BIOL 1615", "BIOL 1615A");
    add_prereqs!(t, course: "BIOL 2060", coreqs: "BIOL 2065", prereqs: "BIOL 1010", "BIOL 1200", "BIOL 1610");
    add_prereqs!(t, course: "BIOL 2065", coreqs: "BIOL 2060");
    add_prereqs!(t, course: "BIOL 2320", coreqs: "BIOL 2325");
    add_prereqs!(t, course: "BIOL 2325", coreqs: "BIOL 2320");
    add_prereqs!(t, course: "BIOL 2400", coreqs: "BIOL 2405");
    add_prereqs!(t, course: "BIOL 2405", coreqs: "BIOL 2400");
    add_prereqs!(t, course: "BIOL 2420", coreqs: "BIOL 2425");
    add_prereqs!(t, course: "BIOL 2425", coreqs: "BIOL 2420");
    add_prereqs!(t, course: "BIOL 3000R", prereqs: "HLOC 2000");
    add_prereqs!(t, course: "BIOL 3010", prereqs: "BIOL 1620");
    add_prereqs!(t, course: "BIOL 3030", prereqs: "BIOL 1610");
    add_prereqs!(t, course: "BIOL 3040", prereqs: "BIOL 1620");
    add_prereqs!(t, course: "BIOL 3045", coreqs: "BIOL 3040", prereqs: "BIOL 1620");
    add_prereqs!(t, course: "BIOL 3100", prereqs: "BIOL 3010", "BIOL 3030", "BIOL 3040");
    add_prereqs!(t, course: "BIOL 3110", prereqs: "ENGL 2010", "BIOL 3010", "BIOL 3030", "BIOL 3040");
    add_prereqs!(t, course: "BIOL 3120", prereqs: "ENGL 2010", "BIOL 3010", "BIOL 3030", "BIOL 3040");
    add_prereqs!(t, course: "BIOL 3140", coreqs: "BIOL 3145", prereqs: "BIOL 3010");
    add_prereqs!(t, course: "BIOL 3145", coreqs: "BIOL 3140");
    add_prereqs!(t, course: "BIOL 3155", prereqs: "BIOL 3010", "BIOL 3030", "MATH 3060");
    add_prereqs!(t, course: "BIOL 3200", prereqs: "BIOL 3010", "BIOL 3030");
    add_prereqs!(t, course: "BIOL 3205", coreqs: "BIOL 3200", prereqs: "BIOL 3010", "BIOL 3030");
    add_prereqs!(t, course: "BIOL 3230R", prereqs: "BIOL 2320", "BIOL 2325");
    add_prereqs!(t, course: "BIOL 3250", prereqs: "BIOL 3030");
    add_prereqs!(t, course: "BIOL 3300", prereqs: "CS 1400", "IT 1100");
    add_prereqs!(t, course: "BIOL 3340", coreqs: "BIOL 3345", prereqs: "MATH 1010");
    add_prereqs!(t, course: "BIOL 3345", coreqs: "BIOL 3340", prereqs: "BIOL 1625", "BIOL 1625A", "BIOL 2405");
    add_prereqs!(t, course: "BIOL 3360", prereqs: "BIOL 3010", "BIOL 3030");
    add_prereqs!(t, course: "BIOL 3420", prereqs: "BIOL 1610");
    add_prereqs!(t, course: "BIOL 3450", coreqs: "BIOL 3455", prereqs: "BIOL 3030", "CHEM 1220");
    add_prereqs!(t, course: "BIOL 3455", coreqs: "BIOL 3450");
    add_prereqs!(t, course: "BIOL 3460", prereqs: "BIOL 3010", "BIOL 3030");
    add_prereqs!(t, course: "BIOL 3470", prereqs: "BIOL 3010", "CHEM 3510");
    add_prereqs!(t, course: "BIOL 3550", coreqs: "BIOL 3555", prereqs: "BIOL 3030", "CHEM 2310");
    add_prereqs!(t, course: "BIOL 3555", coreqs: "BIOL 3550", prereqs: "CHEM 2315");
    add_prereqs!(t, course: "BIOL 4010", prereqs: "BIOL 3030");
    add_prereqs!(t, course: "BIOL 4200", coreqs: "BIOL 4205", prereqs: "BIOL 1620");
    add_prereqs!(t, course: "BIOL 4205", coreqs: "BIOL 4200");
    add_prereqs!(t, course: "BIOL 4260", coreqs: "BIOL 4265", prereqs: "BIOL 3040", "BIOL 3045");
    add_prereqs!(t, course: "BIOL 4265", coreqs: "BIOL 4260", prereqs: "BIOL 3040", "BIOL 3045");
    add_prereqs!(t, course: "BIOL 4270", coreqs: "BIOL 4275", prereqs: "BIOL 3040", "BIOL 3045");
    add_prereqs!(t, course: "BIOL 4275", coreqs: "BIOL 4270", prereqs: "BIOL 3040", "BIOL 3045");
    add_prereqs!(t, course: "BIOL 4280", prereqs: "BIOL 3040");
    add_prereqs!(t, course: "BIOL 4300", coreqs: "BIOL 4305", prereqs: "BIOL 3030", "CHEM 1220");
    add_prereqs!(t, course: "BIOL 4305", coreqs: "BIOL 4300");
    add_prereqs!(t, course: "BIOL 4310", prereqs: "BIOL 3300");
    add_prereqs!(t, course: "BIOL 4320", prereqs: "BIOL 3300");
    add_prereqs!(t, course: "BIOL 4350", coreqs: "BIOL 4355", prereqs: "BIOL 3010", "BIOL 3030");
    add_prereqs!(t, course: "BIOL 4355", coreqs: "BIOL 4350", prereqs: "BIOL 3010", "BIOL 3030");
    add_prereqs!(t, course: "BIOL 4380", coreqs: "BIOL 4385", prereqs: "BIOL 3040", "BIOL 3010");
    add_prereqs!(t, course: "BIOL 4385", coreqs: "BIOL 4380");
    add_prereqs!(t, course: "BIOL 4400", prereqs: "BIOL 2320", "BIOL 2325", "BIOL 2420", "BIOL 2425");
    add_prereqs!(t, course: "BIOL 4411", coreqs: "BIOL 4415");
    add_prereqs!(t, course: "BIOL 4415", coreqs: "BIOL 4411", prereqs: "BIOL 3045");
    add_prereqs!(t, course: "BIOL 4440", prereqs: "BIOL 1620");
    add_prereqs!(t, course: "BIOL 4500", coreqs: "BIOL 4505", prereqs: "BIOL 3010", "BIOL 3030", "CHEM 1220");
    add_prereqs!(t, course: "BIOL 4505", coreqs: "BIOL 4500", prereqs: "CHEM 1225");
    add_prereqs!(t, course: "BIOL 4600", coreqs: "BIOL 4605", prereqs: "BIOL 3010", "BIOL 3030", "CHEM 1220");
    add_prereqs!(t, course: "BIOL 4605", coreqs: "BIOL 4600", prereqs: "CHEM 1225");
    add_prereqs!(t, course: "BIOL 4910", prereqs: "ENGL 2010");
    add_prereqs!(t, course: "BIOL 4930R", prereqs: "BIOL 3110");
    add_prereqs!(t, course: "BTEC 2010", prereqs: "BTEC 1010", "BIOL 1610", "BIOL 1620", "BIOL 1620");
    add_prereqs!(t, course: "BTEC 2020", prereqs: "BTEC 1010", "BIOL 1610", "BIOL 1620", "BIOL 1620");
    add_prereqs!(t, course: "BTEC 2030", prereqs: "BTEC 1010", "BIOL 1610", "BIOL 1620", "BIOL 1620");
    add_prereqs!(t, course: "BTEC 2050", prereqs: "BIOL 1610", "BIOL 1620");
    add_prereqs!(t, course: "BTEC 3010", prereqs: "BIOL 3030");
    add_prereqs!(t, course: "BTEC 3020", prereqs: "BTEC 2020");
    add_prereqs!(t, course: "BTEC 3050", prereqs: "BTEC 2050");
    add_prereqs!(t, course: "BTEC 4020", prereqs: "BTEC 3020");
    add_prereqs!(t, course: "BTEC 4040", prereqs: "BTEC 3050");
    add_prereqs!(t, course: "BTEC 4050", prereqs: "BTEC 3050");
    add_prereqs!(t, course: "BTEC 4060", prereqs: "BTEC 4050");
    add_prereqs!(t, course: "CHEM 1210", coreqs: "CHEM 1215", prereqs: "MATH 1050");
    add_prereqs!(t, course: "CHEM 1215", coreqs: "CHEM 1210");
    add_prereqs!(t, course: "CHEM 1220", coreqs: "CHEM 1225", prereqs: "CHEM 1210");
    add_prereqs!(t, course: "CHEM 1225", coreqs: "CHEM 1220", prereqs: "CHEM 1215");
    add_prereqs!(t, course: "CHEM 2310", coreqs: "CHEM 2315", prereqs: "CHEM 1220");
    add_prereqs!(t, course: "CHEM 2315", coreqs: "CHEM 2310", prereqs: "CHEM 1225");
    add_prereqs!(t, course: "CHEM 2320", coreqs: "CHEM 2325", prereqs: "CHEM 2310");
    add_prereqs!(t, course: "CHEM 2325", coreqs: "CHEM 2320", prereqs: "CHEM 2315");
    add_prereqs!(t, course: "CHEM 2600", prereqs: "CHEM 1220", "CHEM 1225");
    add_prereqs!(t, course: "CHEM 3000", coreqs: "CHEM 3005", prereqs: "CHEM 1220");
    add_prereqs!(t, course: "CHEM 3005", coreqs: "CHEM 3000", prereqs: "CHEM 1225");
    add_prereqs!(t, course: "CHEM 3060", prereqs: "BIOL 3110");
    add_prereqs!(t, course: "CHEM 3065", coreqs: "CHEM 3060", prereqs: "PHYS 2015", "PHYS 2215", "CHEM 2325");
    add_prereqs!(t, course: "CHEM 3070", prereqs: "PHYS 2010", "PHYS 2210", "CHEM 2320", "MATH 1220");
    add_prereqs!(t, course: "CHEM 3075", coreqs: "CHEM 3070", prereqs: "PHYS 2015", "PHYS 2215", "CHEM 2325");
    add_prereqs!(t, course: "CHEM 3100", prereqs: "CHEM 2320");
    add_prereqs!(t, course: "CHEM 3300", prereqs: "CHEM 3000", "CHEM 3005");
    add_prereqs!(t, course: "CHEM 3510", coreqs: "CHEM 3515", prereqs: "BIOL 1610", "CHEM 2320");
    add_prereqs!(t, course: "CHEM 3515", coreqs: "CHEM 3510", prereqs: "BIOL 1615", "CHEM 2325");
    add_prereqs!(t, course: "CHEM 3520", coreqs: "CHEM 3525", prereqs: "CHEM 3510");
    add_prereqs!(t, course: "CHEM 3525", coreqs: "CHEM 3520", prereqs: "CHEM 3515");
    add_prereqs!(t, course: "CHEM 4100", prereqs: "CHEM 3100");
    add_prereqs!(t, course: "CHEM 4200", prereqs: "CHEM 2320");
    add_prereqs!(t, course: "CHEM 4310", prereqs: "CHEM 2320", "CHEM 2325");
    add_prereqs!(t, course: "CHEM 4510", prereqs: "CHEM 2320", "CHEM 2325", "CHEM 3000", "CHEM 3005");
    add_prereqs!(t, course: "CHEM 4610", prereqs: "CHEM 3520");
    add_prereqs!(t, course: "CHEM 4800R", prereqs: "CHEM 2320", "CHEM 2325", "ENGL 2010", "ENGL 2010A");
    add_prereqs!(t, course: "CHEM 4910", prereqs: "CHEM 2320", "CHEM 2325", "ENGL 2010");
    add_prereqs!(t, course: "EDUC 3110", prereqs: "FSHD 1500", "PSY 1010", "PSY 1100");
    add_prereqs!(t, course: "ENER 3310", prereqs: "MATH 1050", "CHEM 1210");
    add_prereqs!(t, course: "ENER 4310", prereqs: "ENER 2310", "GEO 2050");
    add_prereqs!(t, course: "ENVS 1210", coreqs: "ENVS 1215");
    add_prereqs!(t, course: "ENVS 1215", coreqs: "ENVS 1210");
    add_prereqs!(t, course: "ENVS 2210", prereqs: "ENVS 1210", "ENVS 1215", "MATH 1050", "CHEM 1210", "CHEM 1215");
    add_prereqs!(t, course: "ENVS 2700R", prereqs: "ENVS 1210", "ENVS 1215");
    add_prereqs!(t, course: "ENVS 3280", prereqs: "ENVS 2210");
    add_prereqs!(t, course: "ENVS 3410", prereqs: "ENVS 2210", "CHEM 1210");
    add_prereqs!(t, course: "ENVS 3510", prereqs: "ENVS 2210", "GEO 2050");
    add_prereqs!(t, course: "ENVS 4080", prereqs: "ENVS 3410", "ENVS 2700R");
    add_prereqs!(t, course: "GEO 1110", coreqs: "GEO 1115");
    add_prereqs!(t, course: "GEO 1115", coreqs: "GEO 1110");
    add_prereqs!(t, course: "GEO 1220", coreqs: "GEO 1225", prereqs: "GEO 1110");
    add_prereqs!(t, course: "GEO 1225", coreqs: "GEO 1220", prereqs: "GEO 1115");
    add_prereqs!(t, course: "GEO 2050", prereqs: "GEO 1110", "GEO 1115");
    add_prereqs!(t, course: "GEO 2700R", prereqs: "GEO 1110", "GEO 1115");
    add_prereqs!(t, course: "GEO 3000", prereqs: "GEO 1110", "GEO 1115");
    add_prereqs!(t, course: "GEO 3060", prereqs: "GEO 1110", "GEO 1115");
    add_prereqs!(t, course: "GEO 3180", prereqs: "GEO 1110", "GEO 1115");
    add_prereqs!(t, course: "GEO 3200", prereqs: "GEO 1110", "GEO 1115", "MATH 1050", "CHEM 1210", "CHEM 1215");
    add_prereqs!(t, course: "GEO 3400", prereqs: "GEO 1110", "GEO 1115", "CHEM 1210", "CHEM 1215");
    add_prereqs!(t, course: "GEO 3500", coreqs: "GEOG 3600", "GEOG 3605", prereqs: "GEO 1110", "GEO 1115", "MATH 1060", "CHEM 1210", "CHEM 1215");
    add_prereqs!(t, course: "GEO 3550", prereqs: "GEO 1220", "GEO 1225");
    add_prereqs!(t, course: "GEO 3600", prereqs: "GEO 1110", "GEO 1115", "Math 1050", "CHEM 1210", "CHEM 1215", "GEO 3200");
    add_prereqs!(t, course: "GEO 3700", prereqs: "GEO 1110", "GEO 1115", "MATH 1060", "MATH 1080");
    add_prereqs!(t, course: "GEO 4600", prereqs: "GEO 2700R", "GEO 3550", "GEO 3700");
    add_prereqs!(t, course: "GEO 4800R", prereqs: "GEO 2700R");
    add_prereqs!(t, course: "GEOG 2410", prereqs: "ENVS 1210", "BIOL 1610");
    add_prereqs!(t, course: "GEOG 3600", coreqs: "GEOG 3605");
    add_prereqs!(t, course: "GEOG 3605", coreqs: "GEOG 3600");
    add_prereqs!(t, course: "GEOG 4140", prereqs: "GEOG 3600", "GEOG 3605");
    add_prereqs!(t, course: "GEOG 4180", prereqs: "GEOG 3600", "GEOG 3605");

    add_prereqs!(t, course: "MATH 1040", prereqs: "MATH 0980");
    add_prereqs!(t, course: "MATH 1050", prereqs: "MATH 1010", "MATH 1000");
    add_prereqs!(t, course: "MATH 1060", prereqs: "MATH 1050");
    add_prereqs!(t, course: "MATH 1080", prereqs: "MATH 1010", "MATH 1000");
    add_prereqs!(t, course: "MATH 1210", prereqs: "MATH 1050", "MATH 1060", "MATH 1080");
    add_prereqs!(t, course: "MATH 1220", prereqs: "MATH 1210");
    add_prereqs!(t, course: "MATH 2050", prereqs: "MATH 1040");
    add_prereqs!(t, course: "MATH 2200", prereqs: "MATH 1210");
    add_prereqs!(t, course: "MATH 2210", prereqs: "MATH 1220");
    add_prereqs!(t, course: "MATH 2250", prereqs: "Math 1220");
    add_prereqs!(t, course: "MATH 2270", prereqs: "MATH 1210");
    add_prereqs!(t, course: "MATH 2280", prereqs: "MATH 1220");
    add_prereqs!(t, course: "MATH 3000", prereqs: "MATH 1220");
    add_prereqs!(t, course: "MATH 3010", prereqs: "MATH 1210");
    add_prereqs!(t, course: "MATH 3020", prereqs: "MATH 1210");
    add_prereqs!(t, course: "MATH 3050", prereqs: "MATH 2050", "MATH 3060");
    add_prereqs!(t, course: "MATH 3060", prereqs: "MATH 1210");
    add_prereqs!(t, course: "MATH 3100", prereqs: "MATH 2200");
    add_prereqs!(t, course: "MATH 3120", prereqs: "MATH 2200", "CS 3310");
    add_prereqs!(t, course: "MATH 3150", prereqs: "MATH 2210", "MATH 2270", "MATH 2280");
    add_prereqs!(t, course: "MATH 3200", prereqs: "MATH 3120");
    add_prereqs!(t, course: "MATH 3400", prereqs: "MATH 1220");
    add_prereqs!(t, course: "MATH 3410", prereqs: "MATH 3400");
    add_prereqs!(t, course: "MATH 3450", prereqs: "MATH 3400");
    add_prereqs!(t, course: "MATH 3500", prereqs: "MATH 2270", "MATH 2280", "MATH 2250");
    add_prereqs!(t, course: "MATH 3605", prereqs: "MATH 1210");
    add_prereqs!(t, course: "MATH 3700", prereqs: "MATH 2280");
    add_prereqs!(t, course: "MATH 3900", prereqs: "MATH 3120");
    add_prereqs!(t, course: "MATH 3905", prereqs: "CS 1400");
    add_prereqs!(t, course: "MATH 4000", prereqs: "MATH 2270");
    add_prereqs!(t, course: "MATH 4005", prereqs: "CS 1400", "MATH 2250", "MATH 2270");
    add_prereqs!(t, course: "MATH 4010", prereqs: "MATH 4000");
    add_prereqs!(t, course: "MATH 4100", prereqs: "MATH 2210", "MATH 3120");
    add_prereqs!(t, course: "MATH 4200", prereqs: "MATH 3200");
    add_prereqs!(t, course: "MATH 4250", coreqs: "MATH 2280", prereqs: "CS 1400");
    add_prereqs!(t, course: "MATH 4330", prereqs: "MATH 2270", "MATH 3120");
    add_prereqs!(t, course: "MATH 4400", prereqs: "MATH 1100");
    add_prereqs!(t, course: "MATH 4410", prereqs: "MATH 4400");
    add_prereqs!(t, course: "MATH 4500", prereqs: "MATH 1210");
    add_prereqs!(t, course: "MATH 4550", prereqs: "MATH 3500");
    conflict!(t, set hard, clique: "MATH 4250", "MATH 2280");

    add_prereqs!(t, course: "PHYS 1010", prereqs: "MATH 1010");
    add_prereqs!(t, course: "PHYS 1015", coreqs: "PHYS 1010");
    add_prereqs!(t, course: "PHYS 1040", coreqs: "PHYS 1045");
    add_prereqs!(t, course: "PHYS 1045", coreqs: "PHYS 1040");
    add_prereqs!(t, course: "PHYS 2010", coreqs: "PHYS 2015", prereqs: "MATH 1060", "MATH 1080");
    add_prereqs!(t, course: "PHYS 2015", coreqs: "PHYS 2010");
    add_prereqs!(t, course: "PHYS 2020", coreqs: "PHYS 2025", prereqs: "PHYS 2010");
    add_prereqs!(t, course: "PHYS 2025", coreqs: "PHYS 2020", prereqs: "PHYS 2015");
    add_prereqs!(t, course: "PHYS 2210", coreqs: "PHYS 2215", prereqs: "MATH 1210", "MATH 1220");
    add_prereqs!(t, course: "PHYS 2215", coreqs: "PHYS 2210");
    add_prereqs!(t, course: "PHYS 2220", coreqs: "PHYS 2225", prereqs: "MATH 1220", "PHYS 2210");
    add_prereqs!(t, course: "PHYS 2225", coreqs: "PHYS 2220", prereqs: "PHYS 2215");
    add_prereqs!(t, course: "PHYS 3400", prereqs: "PHYS 2220");
    add_prereqs!(t, course: "PHYS 3710", prereqs: "MATH 1220", "PHYS 2220");
    add_prereqs!(t, course: "POLS 1100", prereqs: "ENGL 1010", "ENGL 1010D");
    add_prereqs!(t, course: "PSY 2400", prereqs: "PSY 1010");
    add_prereqs!(t, course: "PSY 3460", prereqs: "PSY 1010");
    add_prereqs!(t, course: "PSY 3710", prereqs: "BIOL 1010", "BIOL 1610", "PSY 1010");
    add_prereqs!(t, course: "SCED 4900", coreqs: "SCED 4989");
    add_prereqs!(t, course: "SE 3100", prereqs: "SE 2450", "CS 2450", "WEB 3450");

    Ok(())
}

pub fn input_multiples(t: &mut Solver) -> Result<(), String> {
    multiple_sections_reduce_penalties!(t,
            courses:
                "BIOL 1010", "BIOL 1015", "BIOL 1200", "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625",
                "BIOL 2065", "BIOL 2320", "BIOL 2325", "BIOL 2420", "BIOL 2425", "BIOL 3010", "BIOL 3030",
                "BIOL 3155", "BIOL 3230R", "BIOL 3455", "BIOL 4890R", "BIOL 4910", "BIOL 4990R",
                "BTEC 2050",
                "CHEM 1010", "CHEM 1015", "CHEM 1125", "CHEM 1150", "CHEM 1155", "CHEM 1210", "CHEM 1215",
                "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315", "CHEM 2320", "CHEM 2325", "CHEM 3300",
                "CHEM 3515", "CHEM 3525", "CHEM 4800R",
                "CS 1400", "CS 1410" with 1 online, "CS 2450", "CS 2810", "CS 4600",
                "ECE 4990",
                "ENVS 1010", "ENVS 1215",
                "GEO 1010", "GEO 1015", "GEO 3500", "GEO 3600",
                "GEOG 1000", "GEOG 1005",
                "IT 1100" with 1 online, "IT 1500" with 3 online, "IT 2300",
                "MATH 900", "MATH 980", "MATH 1010", "MATH 1030", "MATH 1040", "MATH 1050", "MATH 1060",
                "MATH 1210", "MATH 1220", "MATH 2020",
                "MECH 1150", "MECH 1200", "MECH 1205", "MECH 2250", "MECH 2255", "MECH 3255", "MECH 3605", "MECH 3655",
                "PHYS 1015", "PHYS 1045", "PHYS 2010", "PHYS 2015", "PHYS 2020", "PHYS 2025",
                "PHYS 2210", "PHYS 2215", "PHYS 2225", "PHYS 3605",
                "SE 1400" with 2 online);

    // multiple-section courses must be taught at different times
    // TODO:
    //multiple_sections_spread_out!(t, days: "mt", times: "0800-1200", "1200-1630",
    //        courses: "CS 1400", "CS 1410", "CS 2450", "CS 2810", "IT 1100", "SE 1400");
    conflict!(t, set hard, clique: "CS 1400-01", "CS 1400-02", "CS 1400-03", "CS 1400-50");
    conflict!(t, set hard, clique: "CS 1410-01", "CS 1410-02");
    conflict!(t, set hard, clique: "CS 2450-01", "CS 2450-02");
    conflict!(t, set hard, clique: "CS 2810-01", "CS 2810-02");
    conflict!(t, set hard, clique: "IT 1100-01", "IT 1100-02");
    conflict!(t, set hard, clique: "IT 2300-01", "IT 2300-02");
    conflict!(t, set hard, clique: "SE 1400-01", "SE 1400-02");

    Ok(())
}
