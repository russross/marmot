use std::collections::HashMap;
use super::bits::*;
use super::input::*;
use super::solver::Solver;
use rusqlite::{Connection, OpenFlags, Result};

const DB_PATH: &str = "timetable.db";

pub fn setup() -> Result<Solver, String> {
    let departments = vec!["Computing".to_string()];
    //let departments = vec![];

    let mut db = Connection::open_with_flags(
        DB_PATH,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(sql_err)?;
    db.pragma_update(None, "foreign_keys", "ON").map_err(sql_err)?;
    db.pragma_update(None, "temp_store", "memory").map_err(sql_err)?;
    db.pragma_update(None, "mmap_size", "100000000").map_err(sql_err)?;

    let mut solver = make_solver(&mut db)?;

    println!("loading rooms, times");
    load_rooms(&mut db, &mut solver, &departments)?;
    load_time_slots(&mut db, &mut solver, &departments)?;

    println!("loading sections");
    load_sections(&mut db, &mut solver, &departments)?;

    println!("loading conflicts");
    load_conflicts(&mut db, &mut solver, &departments)?;
    load_prereqs(&mut db, &mut solver, &departments)?;
    //discount_multiple_sections(&mut db, &mut solver, &departments)?;
    load_anti_conflicts(&mut db, &mut solver, &departments)?;

    println!("loading faculty");
    load_faculty(&mut db, &mut solver, &departments)?;
    load_faculty_section_assignments(&mut db, &mut solver, &departments)?;

    Ok(solver)
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
    let mut stmt = db.prepare("SELECT holiday FROM active_holidays").map_err(sql_err)?;
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

// load all rooms
pub fn load_rooms(
    db: &mut Connection,
    solver: &mut Solver,
    departments: &Vec<String>,
) -> Result<(), String> {
    let dept_in = dept_clause(departments, &vec!["department".into()], true);
    let mut stmt = db.prepare(&format!("
            SELECT room
            FROM active_rooms
            {}
            ORDER BY building, room_number",
        dept_in)).map_err(sql_err)?;
    let mut rows = stmt
        .query(rusqlite::params_from_iter(departments))
        .map_err(sql_err)?;

    while let Some(row) = rows.next().map_err(sql_err)? {
        solver.rooms.push(Room {
            name: row.get_unwrap(0),
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
    {
        // example: MWF0900+5
        let re = regex::Regex::new(
            r"^([mtwrfsuMTWRFSU]+)([0-1][0-9]|2[0-3])([0-5][05])\+([1-9][0-9]?[05])$",
        )
        .unwrap();

        let dept_in = dept_clause(departments, &vec!["department".into()], true);
        let mut stmt = db.prepare(&format!("
                SELECT time_slot
                FROM active_time_slots
                {}
                ORDER BY duration * LENGTH(days), first_day, start_time, duration",
            dept_in)).map_err(sql_err)?;
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
            });
        }
    }

    // compute time slot conflict lookup table
    // used by time_slots_conflict
    for i in 0..solver.time_slots.len() {
        for j in 0..solver.time_slots.len() {
            solver
                .time_slot_conflicts
                .push(solver.time_slots[i].conflicts.contains(&j));
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
    // load and create sections and their time slots
    {
        let dept_in = dept_clause(departments, &vec!["department".into()], true);
        let mut stmt = db.prepare(&format!("
                    SELECT section, time_slot, time_slot_penalty
                    FROM active_section_time_slots
                    {}
                    ORDER BY section", dept_in)).map_err(sql_err)?;
        let mut rows = stmt.query(rusqlite::params_from_iter(departments)).map_err(sql_err)?;

        let mut section_name = String::new();
        while let Some(row) = rows.next().map_err(sql_err)? {
            let new_section_name: String = row.get_unwrap(0);
            let time_slot_name: String = row.get_unwrap(1);
            let penalty: isize = row.get_unwrap(2);

            // is this a new section?
            if new_section_name != section_name {
                section_name = new_section_name.clone();
                solver.input_sections.push(InputSection {
                    name: new_section_name,
                    instructors: Vec::new(),
                    rooms: Vec::new(),
                    time_slots: Vec::new(),
                    hard_conflicts: Vec::new(),
                    soft_conflicts: Vec::new(),
                    coreqs: Vec::new(),
                    prereqs: Vec::new(),
                });
            }

            let time_slot = solver
                .time_slots
                .iter()
                .position(|elt| elt.name == time_slot_name)
                .unwrap();
            let len = solver.input_sections.len();
            solver.input_sections[len - 1]
                .time_slots.push(TimeWithPenalty { time_slot, penalty });
        }
    }

    // add rooms
    {
        let dept_in = dept_clause(departments, &vec!["department".into()], true);
        let mut stmt = db.prepare(&format!("
                    SELECT section, room, room_penalty
                    FROM active_section_rooms
                    {}
                    ORDER BY section", dept_in)).map_err(sql_err)?;
        let mut rows = stmt.query(rusqlite::params_from_iter(departments)) .map_err(sql_err)?;

        let mut section_name = String::new();
        let mut section_index = None;

        while let Some(row) = rows.next().map_err(sql_err)? {
            let new_section_name: String = row.get_unwrap(0);
            let room_name: String = row.get_unwrap(1);
            let penalty: isize = row.get_unwrap(2);

            if new_section_name != section_name {
                let mut index = section_index.unwrap_or(0);
                while solver.input_sections[index].name != new_section_name {
                    index += 1;
                }
                section_name = new_section_name;
                section_index = Some(index);
            }

            let room = solver
                .rooms
                .iter()
                .position(|elt| elt.name == room_name)
                .unwrap();
            solver.input_sections[section_index.unwrap()]
                .rooms.push(RoomWithPenalty { room, penalty });
        }
    }

    Ok(())
}

// a subtlety: if a course is specified without a section and resolves
// to multiple sections, the penalty between those sections will be unchanged.
// e.g.: specifying CS 101 and CS 102 will set the conflict between every
// section of CS 101 vs every CS 102, but not between the individual
// sections of CS 101 nor between the individual sections of CS 102
pub fn load_conflicts(db: &mut Connection, solver: &mut Solver, departments: &Vec<String>) -> Result<(), String> {
    // load and merge conflicts for all programs
    let dept_in = dept_clause(departments, &vec!["department".into()], true);
    let mut stmt = db.prepare(&format!("
        SELECT program, conflict_name, conflict_penalty, conflict_maximize, course, section
        FROM active_conflicts
        {}
        ORDER BY program, conflict_name, conflict_maximize DESC, course", dept_in)).map_err(sql_err)?;
    let mut rows = stmt
        .query(rusqlite::params_from_iter(departments))
        .map_err(sql_err)?;

    struct Clique {
        penalty: isize,
        maximize: bool,
        courses: Vec<Vec<usize>>,
    }
    
    let mut all_programs = Vec::new();

    // first gather and group all of the entries
    let mut single_program = Vec::new();
    let mut clique = Clique{ penalty: 0, maximize: true, courses: Vec::new() };
    let mut clique_name = ("".into(), "".into());
    let mut sections_in_course = Vec::new();
    let mut course_name = None;

    while let Some(row) = rows.next().map_err(sql_err)? {
        let new_clique_name: (String, String) = (row.get_unwrap(0), row.get_unwrap(1));
        let penalty: isize = row.get_unwrap(2);
        let maximize: bool = row.get_unwrap(3);
        let new_course: Option<String> = row.get_unwrap(4);
        let section: String = row.get_unwrap(5);

        // possibly close out course
        if !sections_in_course.is_empty() && (new_course.is_none() || new_course != course_name || new_clique_name != clique_name) {
            clique.courses.push(std::mem::take(&mut sections_in_course));
        }

        // possibly close out clique
        if new_clique_name != clique_name {
            if clique.courses.len() <= 1 {
                clique.courses.clear();
            } else {
                single_program.push(std::mem::replace(&mut clique, Clique{ penalty, maximize, courses: Vec::new() }));
            }
        }

        // possibly close out program
        if new_clique_name.0 != clique_name.0 && !single_program.is_empty() {
            all_programs.push(std::mem::take(&mut single_program));
        }

        clique_name = new_clique_name;
        clique.penalty = penalty;
        clique.maximize = maximize;
        course_name = new_course;

        let index = find_section_by_name(solver, &section)?;
        sections_in_course.push(index);
    }

    // close out final entries
    if !sections_in_course.is_empty() {
        clique.courses.push(std::mem::take(&mut sections_in_course));
    }
    if clique.courses.len() > 1 {
        single_program.push(std::mem::replace(&mut clique, Clique{ penalty: 0, maximize: true, courses: Vec::new() }));
    }
    if !single_program.is_empty() {
        all_programs.push(std::mem::take(&mut single_program));
    }

    // now process the conflicts
    let mut all_conflicts: HashMap<usize, HashMap<usize, isize>> = HashMap::new();

    for program in &all_programs {
        // build all the conflits for this program
        let mut program_conflicts: HashMap<usize, HashMap<usize, isize>> = HashMap::new();

        for clique in program {
            let mut sections = Vec::new();
            for course in &clique.courses {
                for &section in course {
                    for &other in &sections {
                        let old = program_conflicts.entry(section).or_default().entry(other).or_insert(clique.penalty);
                        *old = if clique.maximize {
                            std::cmp::max(*old, clique.penalty)
                        } else {
                            std::cmp::min(*old, clique.penalty)
                        };
                    }
                }
                for &section in course {
                    sections.push(section);
                }
            }
        }

        // merge the program conflicts into the overall conflict list
        for (&section, others) in program_conflicts.iter() {
            for (&other, &penalty) in others.iter() {
                let old = all_conflicts.entry(section).or_default().entry(other).or_insert(penalty);
                *old = std::cmp::max(*old, penalty);
            }
        }
    }

    // copy it all into the solver
    for (&section, others) in all_conflicts.iter() {
        for (&other, &penalty) in others.iter() {
            if penalty > 0 {
                solver.input_sections[section].set_conflict(other, penalty);
                solver.input_sections[other].set_conflict(section, penalty);
            }
        }
    }

    Ok(())
}

// load prereqs and coreqs
pub fn load_prereqs(
    db: &mut Connection,
    solver: &mut Solver,
    departments: &Vec<String>,
) -> Result<(), String> {
    // add prereqs
    {
        let dept_in = dept_clause(departments, &vec!["section_department".into(), "prereq_department".into()], true);
        let mut stmt = db
            .prepare(&format!("
                    SELECT section, prereq
                    FROM active_prereqs
                    {}", dept_in)).map_err(sql_err)?;

        let mut departments_x2 = Vec::new();
        departments_x2.extend(departments);
        departments_x2.extend(departments);
        let mut rows = stmt
            .query(rusqlite::params_from_iter(departments_x2))
            .map_err(sql_err)?;

        while let Some(row) = rows.next().map_err(sql_err)? {
            let section: String = row.get_unwrap(0);
            let prereq: String = row.get_unwrap(1);

            let index = find_section_by_name(solver, &section)?;
            let prereq_index = find_section_by_name(solver, &prereq)?;
            solver.input_sections[index].prereqs.push(prereq_index);
        }
    }

    // add coreqs
    {
        let dept_in = dept_clause(departments, &vec!["section_department".into(), "coreq_department".into()], true);
        let mut stmt = db
            .prepare(&format!("
                    SELECT section, coreq
                    FROM active_coreqs
                    {}", dept_in)).map_err(sql_err)?;

        let mut departments_x2 = Vec::new();
        departments_x2.extend(departments);
        departments_x2.extend(departments);
        let mut rows = stmt
            .query(rusqlite::params_from_iter(departments_x2))
            .map_err(sql_err)?;

        while let Some(row) = rows.next().map_err(sql_err)? {
            let section: String = row.get_unwrap(0);
            let coreq: String = row.get_unwrap(1);

            let index = find_section_by_name(solver, &section)?;
            let coreq_index = find_section_by_name(solver, &coreq)?;
            solver.input_sections[index].coreqs.push(coreq_index);
        }
    }

    // compute the transitive closure of all prereqs
    // we will do it the easy/dumb way and just iterate until it converges
    //
    // the transitive closure of prereqs is used to remove conflicts between classes
    // that cannot be taken together
    // note: the prereqs of a coreq are treated like direct prereqs
    // and the coreqs of a prereq are treated like direct prereqs
    //
    // if a course is a coreq (and optionally also a prereq) then we do nothing
    // directly but it will affect courses that require this one
    let mut changed = true;
    while changed {
        changed = false;

        for sec_i in 0..solver.input_sections.len() {
            let mut new_list = Vec::new();
            for &pre in &solver.input_sections[sec_i].prereqs {
                // keep the prereq
                new_list.push(pre);

                // add the prereq's prereqs
                for &elt in &solver.input_sections[pre].prereqs {
                    new_list.push(elt);
                }

                // and the prereq's coreqs
                for &elt in &solver.input_sections[pre].coreqs {
                    new_list.push(elt);
                }
            }
            for &co in &solver.input_sections[sec_i].coreqs {
                // and the coreq's prereqs
                for &elt in &solver.input_sections[co].prereqs {
                    new_list.push(elt);
                }
            }

            // but filter out the coreqs themselves
            new_list.retain(|elt| !solver.input_sections[sec_i].coreqs.contains(elt));

            new_list.sort();
            new_list.dedup();
            if new_list.len() != solver.input_sections[sec_i].prereqs.len() {
                changed = true;
                solver.input_sections[sec_i].prereqs = new_list;
            } else {
                for i in 0..new_list.len() {
                    if new_list[i] != solver.input_sections[sec_i].prereqs[i] {
                        changed = true;
                        solver.input_sections[sec_i].prereqs = new_list;
                        break;
                    }
                }
            }
        }
    }

    // remove all conflicts between courses and their prereqs
    for sec_i in 0..solver.input_sections.len() {
        for pre_i in 0..solver.input_sections[sec_i].prereqs.len() {
            let prereq = solver.input_sections[sec_i].prereqs[pre_i];

            // delete the conflict unless it is marked as a hard conflict
            if (1..=99).contains(&solver.input_sections[sec_i].get_conflict(prereq)) {
                solver.input_sections[sec_i].set_conflict(prereq, 0);
                solver.input_sections[prereq].set_conflict(sec_i, 0);
            }
        }
    }

    Ok(())
}

//
// TODO: does not handle cross listings
// TODO: only sections with hard conflicts between them (or online) count. count + explicit
// overrides?
//
/*
pub fn discount_multiple_sections(db: &mut Connection, solver: &mut Solver, cross_listings: &HashMap<String, String>, departments: &Vec<String>) -> Result<(), String> {
    // discount conflicts for courses with multiple sections
    let threshold = 30;
    let dept_in = dept_clause(departments, &vec!["department".into()], true);
    let mut stmt = db.prepare(&format!("
        SELECT course, section_count
        FROM active_section_counts
        {}", dept_in)).map_err(sql_err)?;
    let mut rows = stmt
        .query(rusqlite::params_from_iter(departments))
        .map_err(sql_err)?;

    while let Some(row) = rows.next().map_err(sql_err)? {
        let course: String = row.get_unwrap(0);
        let count: isize = row.get_unwrap(1);

        let course_list = find_sections_by_name(solver, &course)?;
        for sec_i in course_list {
            let others: Vec<usize> = solver.input_sections[sec_i]
                .soft_conflicts
                .iter()
                .map(|elt| elt.section)
                .collect();
            for other in others {
                let old_score = solver.input_sections[sec_i].get_conflict(other);
                if old_score >= 100 || old_score <= 0 {
                    continue;
                }
                let mut new_score =
                    (solver.input_sections[sec_i].get_conflict(other) - 1) / (count + 1);
                if new_score < threshold {
                    new_score = 0;
                }

                // set in both directions
                solver.input_sections[sec_i].set_conflict(other, new_score);
                solver.input_sections[other].set_conflict(sec_i, new_score);
            }
        }
    }

    Ok(())
}
*/

pub fn load_anti_conflicts(
    db: &mut Connection,
    solver: &mut Solver,
    departments: &Vec<String>,
) -> Result<(), String> {
    let dept_in = dept_clause(departments, &vec!["single_department".into(), "group_department".into()], true);
    let mut stmt = db.prepare(&format!("
        SELECT single_section, group_section, anti_conflict_penalty
        FROM active_anti_conflicts
        {}
        ORDER BY single_section", dept_in)).map_err(sql_err)?;

    let mut departments_x2 = Vec::new();
    departments_x2.extend(departments);
    departments_x2.extend(departments);
    let mut rows = stmt
        .query(rusqlite::params_from_iter(departments_x2))
        .map_err(sql_err)?;

    let mut single_name = String::new();
    let mut group = Vec::new();
    let mut penalty = 0;

    while let Some(row) = rows.next().map_err(sql_err)? {
        let new_single_name: String = row.get_unwrap(0);
        let other_name: String = row.get_unwrap(1);
        let other = find_section_by_name(solver, &other_name)?;
        let pen: isize = row.get_unwrap(2);

        // start a new anti conflict
        if new_single_name != single_name {
            // close the previous one out
            if !group.is_empty() {
                group.sort();
                group.dedup();
                let entry = (
                    penalty,
                    find_section_by_name(solver, &single_name)?,
                    std::mem::take(&mut group),
                );
                solver.anticonflicts.push(entry);
            }

            // start a new one
            single_name = new_single_name;
            penalty = pen;
        }
        group.push(other);
    }

    // close the final one out
    if !group.is_empty() {
        let entry = (
            penalty,
            find_section_by_name(solver, &single_name)?,
            std::mem::take(&mut group),
        );
        solver.anticonflicts.push(entry);
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
        let dept_in = dept_clause(departments, &vec!["department".into()], true);
        let mut stmt = db.prepare(&format!("
                SELECT faculty, day_of_week, start_time, start_time + duration AS end_time, availability_penalty
                FROM active_faculty_availability
                {}
                ORDER BY faculty", dept_in)).map_err(sql_err)?;
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
        let dept_in = dept_clause(departments, &vec!["department".into()], true);
        let mut stmt = db.prepare(&format!("
                SELECT  faculty,
                        days_to_check, days_off, days_off_penalty, evenly_spread_penalty, max_gap_within_cluster,
                        is_cluster, is_too_short, interval_minutes, interval_penalty
                FROM active_faculty_preference_intervals
                {}
                ORDER BY faculty", dept_in)).map_err(sql_err)?;
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

pub fn load_faculty_section_assignments(
    db: &mut Connection,
    solver: &mut Solver,
    departments: &Vec<String>,
) -> Result<(), String> {
    // link sections to faculty
    {
        let dept_in = dept_clause(departments, &vec!["department".into()], true);
        let mut stmt = db
            .prepare(&format!("
                    SELECT faculty, section
                    FROM active_faculty_sections
                    {}
                    ORDER BY faculty", dept_in)).map_err(sql_err)?;
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
            solver.instructors[faculty_index].sections.push(section_index);
            solver.input_sections[section_index].instructors.push(faculty_index);

            // intersect faculty availability with section time slots
            // note: this combines instructor and section time penalties
            let old_time_slots = std::mem::take(&mut solver.input_sections[section_index].time_slots);
            for &TimeWithPenalty{ time_slot, penalty } in &old_time_slots {
                match solver.instructors[faculty_index].get_time_slot_penalty(&solver.time_slots[time_slot]) {
                    Some(pen) => solver.input_sections[section_index].time_slots.push(
                        TimeWithPenalty{ time_slot, penalty: std::cmp::min(99, pen + penalty) }
                    ),
                    None => (),
                }
            }
            if solver.input_sections[section_index].time_slots.is_empty() {
                return Err(format!("assigning faculty {faculty_name} to section {section_name} left not viable time slots"));
            }
        }
    }

    // add hard conflicts between all the sections an instructor teaches
    for instructor in &solver.instructors {
        for &left in &instructor.sections {
            for &right in &instructor.sections {
                if left == right {
                    continue;
                };
                solver.input_sections[left].set_conflict(right, 100);
            }
        }
    }

    Ok(())
}

fn sql_err(err: rusqlite::Error) -> String {
    format!("{err}")
}

fn dept_clause(departments: &Vec<String>, columns: &Vec<String>, with_where: bool) -> String {
    let mut s = "".to_string();
    if !departments.is_empty() {
        for (i, col) in columns.iter().enumerate() {
            s = if i == 0 && with_where {
                format!("WHERE {col} IN (")
            } else {
                format!("{s} AND {col} IN (")
            };
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
