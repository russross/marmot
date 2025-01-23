use super::score::*;
use super::solver::*;
use itertools::Itertools;
use sqlite::{Connection, Error, OpenFlags, State, Value};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use std::time::Instant;
use time::OffsetDateTime;

//
//
// Input data
// All basic data that is loaded at the start and then
// never modified while the solver runs.
//
//

#[derive(Clone)]
pub struct Input {
    // the name of the term
    pub term_name: String,

    // core schedule data
    pub rooms: Vec<Room>,
    pub time_slots: Vec<TimeSlot>,
    pub faculty: Vec<Faculty>,
    pub sections: Vec<Section>,
    pub criteria: Vec<Criterion>,

    // matrix of which time slots overlap which for fast lookup
    pub time_slot_conflicts: Vec<Vec<bool>>,
}

#[derive(Clone)]
pub struct Room {
    pub name: String,
}

#[derive(Clone)]
pub struct TimeSlot {
    pub name: String,
    pub days: Vec<time::Weekday>,
    pub start_time: time::Time,
    pub duration: time::Duration,
}

#[derive(Clone)]
pub struct Faculty {
    pub name: String,
    pub sections: Vec<usize>,
}

#[derive(Clone)]
pub struct Section {
    // e.g.,: "CS 1410-02"
    pub name: String,

    // rooms (if any) and times available for this section
    pub rooms: Vec<RoomWithOptionalPriority>,
    pub time_slots: Vec<TimeSlotWithOptionalPriority>,

    // faculty (if any) assigned to this section
    pub faculty: Vec<usize>,

    // hard conflicts
    pub hard_conflicts: Vec<usize>,

    // scoring criteria that apply to this section
    pub criteria: Vec<usize>,

    // any section that might have a scoring interaction with this section
    pub neighbors: Vec<usize>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct RoomWithOptionalPriority {
    pub room: usize,
    pub priority: Option<u8>,
}

#[derive(Clone)]
pub struct TimeSlotWithOptionalPriority {
    pub time_slot: usize,
    pub priority: Option<u8>,
}

#[derive(Clone)]
pub struct RoomWithPriority {
    pub room: usize,
    pub priority: u8,
}

#[derive(Clone)]
pub struct TimeSlotWithPriority {
    pub time_slot: usize,
    pub priority: u8,
}

#[derive(Clone)]
pub struct SectionWithPriority {
    pub section: usize,
    pub priority: u8,
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for TimeSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Faculty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if !self.sections.is_empty() {
            write!(f, " assigned[")?;
            let mut sep = "";
            for elt in &self.sections {
                write!(f, "{sep}{elt}")?;
                sep = ",";
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}

impl Faculty {
    pub fn debug(&self, input: &Input) -> String {
        let mut s = String::new();

        write!(&mut s, "{}", self.name).unwrap();
        if !self.sections.is_empty() {
            write!(&mut s, " assigned[").unwrap();
            let mut sep = "";
            for &elt in &self.sections {
                write!(&mut s, "{}{}", sep, input.sections[elt].name).unwrap();
                sep = ", ";
            }
            write!(&mut s, "]").unwrap();
        }

        s
    }
}

pub fn load_input(db_path: &str, departments: &[String]) -> Result<Input, String> {
    print!("loading input data");
    let start = Instant::now();

    let db =
        Connection::open_with_flags(db_path, OpenFlags::new().with_read_only().with_no_mutex()).map_err(sql_err)?;
    db.execute("PRAGMA foreign_keys = ON").map_err(sql_err)?;
    db.execute("PRAGMA temp_store = memory").map_err(sql_err)?;
    db.execute("PRAGMA mmap_size = 100000000").map_err(sql_err)?;

    let mut term_name = "Unknown term".to_string();
    let mut stmt = db.prepare("SELECT term FROM terms").map_err(sql_err)?;
    while stmt.next().map_err(sql_err)? == State::Row {
        term_name = stmt.read(0).map_err(sql_err)?;
    }

    let (rooms, room_index) = load_rooms(&db, departments)?;
    let (time_slots, time_slot_index) = load_time_slots(&db, departments)?;
    let time_slot_conflicts = load_time_slot_conflicts(&db, &time_slot_index, departments)?;
    let (mut faculty, faculty_index) = load_faculty(&db, departments)?;
    let (mut sections, section_index, mut criteria) = load_sections(&db, &room_index, &time_slot_index, departments)?;
    load_conflicts(&db, &mut sections, &section_index, &mut criteria, departments)?;
    load_anti_conflicts(&db, &mut sections, &section_index, &mut criteria, departments)?;
    load_faculty_section_assignments(
        &db,
        &mut faculty,
        &faculty_index,
        &mut sections,
        &section_index,
        &mut criteria,
        departments,
    )?;
    compute_neighbors(&mut sections, &criteria);
    println!(": {}ms", start.elapsed().as_millis());

    Ok(Input { term_name, rooms, time_slots, faculty, sections, criteria, time_slot_conflicts })
}

// load all rooms
pub fn load_rooms(db: &Connection, departments: &[String]) -> Result<(Vec<Room>, HashMap<String, usize>), String> {
    let dept_in = dept_clause(departments, &["department".into()], true);
    let mut stmt = db
        .prepare(format!(
            "
            SELECT DISTINCT room
            FROM rooms_used_by_departments
            {}
            ORDER BY building, CAST (room_number AS INTEGER), room_number",
            dept_in
        ))
        .map_err(sql_err)?;
    stmt.bind_iter(as_values(departments)).map_err(sql_err)?;

    let mut rooms = Vec::new();
    let mut room_index = HashMap::new();
    while stmt.next().map_err(sql_err)? == State::Row {
        let room = Room { name: stmt.read(0).map_err(sql_err)? };
        room_index.insert(room.name.clone(), rooms.len());
        rooms.push(room);
    }

    Ok((rooms, room_index))
}

// load all time slots
pub fn load_time_slots(
    db: &Connection,
    departments: &[String],
) -> Result<(Vec<TimeSlot>, HashMap<String, usize>), String> {
    let dept_in = dept_clause(departments, &["department".into()], true);
    let mut stmt = db
        .prepare(format!(
            "
            SELECT DISTINCT time_slot, days, start_time, duration
            FROM time_slots_used_by_departments_materialized
            {}
            ORDER BY first_day, start_time, duration, duration * LENGTH(days)",
            dept_in
        ))
        .map_err(sql_err)?;
    stmt.bind_iter(as_values(departments)).map_err(sql_err)?;

    let mut time_slots = Vec::new();
    let mut time_slot_index = HashMap::new();
    while stmt.next().map_err(sql_err)? == State::Row {
        let name: String = stmt.read(0).map_err(sql_err)?;
        let days: String = stmt.read(1).map_err(sql_err)?;
        let start_time_i: i64 = stmt.read(2).map_err(sql_err)?;
        let start_time = start_time_i as u32;
        let duration: i64 = stmt.read(3).map_err(sql_err)?;
        let time_slot = TimeSlot {
            name,
            days: parse_days(&days)?,
            start_time: time::Time::from_hms((start_time / 60) as u8, (start_time % 60) as u8, 0).unwrap(),
            duration: time::Duration::minutes(duration),
        };
        time_slot_index.insert(time_slot.name.clone(), time_slots.len());
        time_slots.push(time_slot);
    }

    Ok((time_slots, time_slot_index))
}

// load all time slots
pub fn load_time_slot_conflicts(
    db: &Connection,
    time_slot_index: &HashMap<String, usize>,
    departments: &[String],
) -> Result<Vec<Vec<bool>>, String> {
    let dept_in = dept_clause(departments, &["ts_a.department".into(), "ts_b.department".into()], true);
    let mut stmt = db
        .prepare(format!(
            "
            SELECT time_slot_a, time_slot_b
            FROM conflicting_time_slots
            JOIN time_slots_used_by_departments_materialized AS ts_a
                ON  time_slot_a = ts_a.time_slot
            JOIN time_slots_used_by_departments_materialized AS ts_b
                ON  time_slot_b = ts_b.time_slot
            {}",
            dept_in
        ))
        .map_err(sql_err)?;
    stmt.bind_iter(as_values(&double_vec(departments))).map_err(sql_err)?;

    let time_slot_len = time_slot_index.len();
    let mut conflicts = vec![vec![false; time_slot_len]; time_slot_len];
    while stmt.next().map_err(sql_err)? == State::Row {
        let a: String = stmt.read(0).map_err(sql_err)?;
        let b: String = stmt.read(1).map_err(sql_err)?;
        let &aa = time_slot_index
            .get(&a)
            .ok_or(format!("time slot {a} reported as conflict but not found in usable time slots"))?;
        let &bb = time_slot_index
            .get(&b)
            .ok_or(format!("time slot {b} reported as conflict but not found in usable time slots"))?;

        // conflicts go both ways
        conflicts[aa][bb] = true;
        conflicts[bb][aa] = true;
    }

    Ok(conflicts)
}

// load faculty
pub fn load_faculty(db: &Connection, departments: &[String]) -> Result<(Vec<Faculty>, HashMap<String, usize>), String> {
    let mut faculty_list = Vec::new();
    let mut faculty_lookup = HashMap::new();

    {
        let dept_in = dept_clause(departments, &["department".into()], true);
        let mut stmt = db
            .prepare(format!(
                "
                SELECT DISTINCT faculty
                FROM faculty_sections_to_be_scheduled
                {}
                ORDER BY faculty",
                dept_in
            ))
            .map_err(sql_err)?;
        stmt.bind_iter(as_values(departments)).map_err(sql_err)?;

        while stmt.next().map_err(sql_err)? == State::Row {
            let faculty = Faculty { name: stmt.read(0).map_err(sql_err)?, sections: Vec::new() };
            faculty_lookup.insert(faculty.name.clone(), faculty_list.len());
            faculty_list.push(faculty);
        }
    }

    Ok((faculty_list, faculty_lookup))
}

// load sections and the room/time combinations (plus penalties) associated with them
#[allow(clippy::type_complexity)]
pub fn load_sections(
    db: &Connection,
    room_index: &HashMap<String, usize>,
    time_slot_index: &HashMap<String, usize>,
    departments: &[String],
) -> Result<(Vec<Section>, HashMap<String, usize>, Vec<Criterion>), String> {
    let mut sections = Vec::new();
    let mut section_index = HashMap::new();
    let mut criteria = Vec::new();

    // load and create sections and their time slots
    {
        let dept_in = dept_clause(departments, &["department".into()], true);
        let mut stmt = db
            .prepare(format!(
                "
                SELECT DISTINCT section, time_slot, time_slot_priority
                FROM time_slots_available_to_sections_materialized
                {}
                ORDER BY section",
                dept_in
            ))
            .map_err(sql_err)?;
        stmt.bind_iter(as_values(departments)).map_err(sql_err)?;

        let mut section_name = String::new();
        while stmt.next().map_err(sql_err)? == State::Row {
            let new_section_name: String = stmt.read(0).map_err(sql_err)?;
            let time_slot_name: String = stmt.read(1).map_err(sql_err)?;
            let priority: Option<i64> = stmt.read(2).map_err(sql_err)?;

            // is this a new section?
            if new_section_name != section_name {
                section_name = new_section_name.clone();
                let section = Section {
                    name: new_section_name.clone(),
                    rooms: Vec::new(),
                    time_slots: Vec::new(),
                    faculty: Vec::new(),
                    hard_conflicts: Vec::new(),
                    criteria: Vec::new(),
                    neighbors: Vec::new(),
                };
                section_index.insert(new_section_name.clone(), sections.len());
                sections.push(section);
            }

            let time_slot = *time_slot_index.get(&time_slot_name).ok_or(format!(
                "section {} references time slot {} but I cannot find it",
                section_name, time_slot_name
            ))?;
            sections
                .last_mut()
                .unwrap()
                .time_slots
                .push(TimeSlotWithOptionalPriority { time_slot, priority: priority.map(|elt| elt as u8) });
        }
    }

    // add rooms
    {
        let dept_in = dept_clause(departments, &["department".into()], true);
        let mut stmt = db
            .prepare(format!(
                "
                SELECT DISTINCT section, room, room_priority
                FROM rooms_available_to_sections
                {}
                ORDER BY section",
                dept_in
            ))
            .map_err(sql_err)?;
        stmt.bind_iter(as_values(departments)).map_err(sql_err)?;

        let mut section_name = String::new();
        let mut index = None;

        while stmt.next().map_err(sql_err)? == State::Row {
            let new_section_name: String = stmt.read(0).map_err(sql_err)?;
            let room_name: String = stmt.read(1).map_err(sql_err)?;
            let priority: Option<i64> = stmt.read(2).map_err(sql_err)?;

            if new_section_name != section_name {
                section_name = new_section_name.clone();
                index = Some(
                    section_index
                        .get(&new_section_name)
                        .ok_or(format!("section {} not found but has room {} assigned", section_name, room_name))?,
                );
            }

            let room = *room_index
                .get(&room_name)
                .ok_or(format!("section {} assigned to room {} but room not found", section_name, room_name))?;
            sections[*index.unwrap()]
                .rooms
                .push(RoomWithOptionalPriority { room, priority: priority.map(|elt| elt as u8) });
        }
    }

    // create the scoring criteria for time slot and room preferences
    // and create unplaced criteria
    for (section_i, section) in sections.iter_mut().enumerate() {
        let rooms_with_priorities: Vec<RoomWithPriority> = section
            .rooms
            .iter()
            .filter_map(|RoomWithOptionalPriority { room, priority }| {
                priority.map(|p| RoomWithPriority { room: *room, priority: p })
            })
            .collect();
        if !rooms_with_priorities.is_empty() {
            criteria.push(Criterion::RoomPreference { section: section_i, rooms_with_priorities });
        }
        let time_slots_with_priorities: Vec<TimeSlotWithPriority> = section
            .time_slots
            .iter()
            .filter_map(|TimeSlotWithOptionalPriority { time_slot, priority }| {
                priority.map(|p| TimeSlotWithPriority { time_slot: *time_slot, priority: p })
            })
            .collect();
        if !time_slots_with_priorities.is_empty() {
            criteria.push(Criterion::TimeSlotPreference { section: section_i, time_slots_with_priorities });
        }
    }

    Ok((sections, section_index, criteria))
}

pub fn load_conflicts(
    db: &Connection,
    sections: &mut [Section],
    section_index: &HashMap<String, usize>,
    criteria: &mut Vec<Criterion>,
    departments: &[String],
) -> Result<(), String> {
    let dept_in = dept_clause(departments, &["department_a".into(), "department_b".into()], false);
    let mut stmt = db
        .prepare(format!(
            "
            SELECT DISTINCT section_a, section_b, priority
            FROM conflict_pairs_materialized
            WHERE section_a < section_b
            {}
            ORDER BY section_a, section_b",
            dept_in
        ))
        .map_err(sql_err)?;
    stmt.bind_iter(as_values(&double_vec(departments))).map_err(sql_err)?;

    while stmt.next().map_err(sql_err)? == State::Row {
        let section_a: String = stmt.read(0).map_err(sql_err)?;
        let section_b: String = stmt.read(1).map_err(sql_err)?;
        let priority: i64 = stmt.read(2).map_err(sql_err)?;

        let index_a =
            *section_index.get(&section_a).ok_or(format!("section_a {section_a} from conflict pair not found"))?;
        let index_b =
            *section_index.get(&section_b).ok_or(format!("section_b {section_b} from conflict pair not found"))?;
        if priority < LEVEL_FOR_HARD_CONFLICT as i64 || priority >= START_LEVEL_FOR_PREFERENCES as i64 {
            return Err(format!("conflict pair {section_a} vs {section_b} has invalid priority of {priority}"));
        }
        let priority = priority as u8;
        if priority == LEVEL_FOR_HARD_CONFLICT {
            sections[index_a].hard_conflicts.push(index_b);
            sections[index_b].hard_conflicts.push(index_a);
        } else {
            criteria.push(Criterion::SoftConflict { priority, sections: [index_a, index_b] });
        }
    }

    Ok(())
}

pub fn load_anti_conflicts(
    db: &Connection,
    _sections: &mut [Section],
    section_index: &HashMap<String, usize>,
    criteria: &mut Vec<Criterion>,
    departments: &[String],
) -> Result<(), String> {
    let dept_in = dept_clause(departments, &["single_department".into(), "group_department".into()], true);
    // note: single_section can only have one anticonflict rule
    let mut stmt = db
        .prepare(format!(
            "
            SELECT DISTINCT single_section, group_section, priority
            FROM anti_conflict_pairs
            {}
            ORDER BY single_section, priority, group_section",
            dept_in
        ))
        .map_err(sql_err)?;
    stmt.bind_iter(as_values(&double_vec(departments))).map_err(sql_err)?;

    let mut criterion = None;

    while stmt.next().map_err(sql_err)? == State::Row {
        let new_single_name: String = stmt.read(0).map_err(sql_err)?;
        let new_single = *section_index
            .get(&new_single_name)
            .ok_or(format!("anticonflict for unknown section {new_single_name}"))?;
        let other_name: String = stmt.read(1).map_err(sql_err)?;
        let other =
            *section_index.get(&other_name).ok_or(format!("anticonflict references unknown section {other_name}"))?;
        if new_single == other {
            panic!("anticonflict: single and group names must differ");
        }
        let pri: i64 = stmt.read(2).map_err(sql_err)?;

        // existing anti conflict?
        match &mut criterion {
            Some(Criterion::AntiConflict { single, group, .. }) if *single == new_single => {
                group.push(other);
            }
            _ => {
                // start a new one
                if let Some(elt) = criterion {
                    criteria.push(elt);
                }
                criterion =
                    Some(Criterion::AntiConflict { priority: pri as u8, single: new_single, group: vec![other] });
            }
        }
    }

    // close the final one out
    if let Some(elt) = criterion {
        criteria.push(elt);
    }

    Ok(())
}

pub fn load_faculty_section_assignments(
    db: &Connection,
    faculty_list: &mut [Faculty],
    faculty_index: &HashMap<String, usize>,
    sections: &mut [Section],
    section_index: &HashMap<String, usize>,
    criteria: &mut Vec<Criterion>,
    departments: &[String],
) -> Result<(), String> {
    let mut faculty_sections = Vec::new();
    for _ in 0..faculty_list.len() {
        faculty_sections.push(Vec::new());
    }
    {
        // link sections to faculty
        let dept_in = dept_clause(departments, &["department".into()], true);
        let mut stmt = db
            .prepare(format!(
                "
                SELECT DISTINCT faculty, section
                FROM faculty_sections_to_be_scheduled
                {}
                ORDER BY faculty, section",
                dept_in
            ))
            .map_err(sql_err)?;
        stmt.bind_iter(as_values(departments)).map_err(sql_err)?;

        while stmt.next().map_err(sql_err)? == State::Row {
            let faculty_name: String = stmt.read(0).map_err(sql_err)?;
            let section_name: String = stmt.read(1).map_err(sql_err)?;
            let faculty_i = *faculty_index
                .get(&faculty_name)
                .ok_or(format!("faculty {faculty_name} not found in mapping to section {section_name}"))?;
            let section_i = *section_index
                .get(&section_name)
                .ok_or(format!("section {section_name} not found in mapping to faculty {faculty_name}"))?;

            faculty_sections[faculty_i].push(section_i);
        }
    }

    // calculate theoretical minimum rooms possible for each faculty
    let mut min_rooms = vec![None; faculty_list.len()];
    for faculty in 0..faculty_sections.len() {
        // fill in section.faculty and faculty.sections lists
        faculty_list[faculty].sections = faculty_sections[faculty].clone();
        for &section in &faculty_sections[faculty] {
            sections[section].faculty.push(faculty);
        }

        // get a list of all possible rooms the faculty could use
        let mut all_possible = Vec::new();
        let mut section_list = Vec::new();
        for &section in &faculty_sections[faculty] {
            // ignore sections that do not have rooms or are not satisfied with any room
            if !sections[section].rooms.iter().any(|elt| elt.priority.is_none()) {
                continue;
            }
            section_list.push(section);
            for RoomWithOptionalPriority { room, priority } in &sections[section].rooms {
                // only record desired rooms
                if priority.is_none() {
                    all_possible.push(*room);
                }
            }
        }
        all_possible.sort_unstable();
        all_possible.dedup();

        if section_list.len() <= 1 || all_possible.len() <= 1 {
            continue;
        }

        // note if the loop ends without finding a solution with
        // fewer than the max number of rooms, it will leave the
        // result at the max number without bothering to prove it
        let mut k = 1;
        'min_rooms_loop: while k < section_list.len() {
            'set_loop: for room_set in all_possible.iter().combinations(k) {
                'section_loop: for &section in &section_list {
                    // is this section satisfied by one of the rooms in the set?
                    for &room in &room_set {
                        if sections[section].rooms.iter().any(|elt| elt.room == *room) {
                            continue 'section_loop;
                        }
                    }
                    continue 'set_loop;
                }

                // success!
                break 'min_rooms_loop;
            }

            k += 1;
        }

        // do not bother if the best we can do is a distinct room per section
        if k >= section_list.len() {
            continue;
        }

        min_rooms[faculty] = Some(k);
    }

    // load faculty spread preferences
    let mut prefs = vec![None; faculty_list.len()];
    {
        let dept_in = dept_clause(departments, &["department".into()], true);
        let mut stmt = db
            .prepare(format!(
                "
                SELECT  faculty, days_to_check,
                        days_off, days_off_priority, evenly_spread_priority,
                        no_room_switch_priority, too_many_rooms_priority,
                        max_gap_within_cluster,
                        is_cluster, is_too_short, interval_minutes, interval_priority
                FROM faculty_to_be_scheduled_preference_intervals
                {}
                ORDER BY faculty, is_cluster, is_too_short, interval_minutes",
                dept_in
            ))
            .map_err(sql_err)?;
        stmt.bind_iter(as_values(departments)).map_err(sql_err)?;

        while stmt.next().map_err(sql_err)? == State::Row {
            let name: String = stmt.read(0).map_err(sql_err)?;
            let index = *faculty_index.get(&name).ok_or(format!("faculty not found for {name} but prefs found"))?;

            // is this the first row for this faculty?
            if prefs[index].is_none() {
                let days_to_check: String = stmt.read(1).map_err(sql_err)?;
                let days_to_check = parse_days(&days_to_check)?;

                // days off penalty?
                let days_off_opt: Option<i64> = stmt.read(2).map_err(sql_err)?;
                let days_off_priority_opt: Option<i64> = stmt.read(3).map_err(sql_err)?;
                let days_off = if let (Some(days_off), Some(priority)) = (days_off_opt, days_off_priority_opt) {
                    Some((priority as u8, days_off as usize))
                } else {
                    None
                };

                // evenly spread penalty?
                let evenly_spread_priority: Option<i64> = stmt.read(4).map_err(sql_err)?;
                let evenly_spread = evenly_spread_priority.map(|priority| priority as u8);

                // no room switch penalty?
                let no_room_switch_priority: Option<i64> = stmt.read(5).map_err(sql_err)?;
                let no_room_switch = no_room_switch_priority.map(|priority| priority as u8);

                // too many rooms penalty?
                let too_many_rooms_priority: Option<i64> = stmt.read(6).map_err(sql_err)?;
                let too_many_rooms = if let Some(priority) = too_many_rooms_priority {
                    min_rooms[index].map(|k| (priority as u8, k))
                } else {
                    None
                };

                let max_gap_within_cluster: i64 = stmt.read(7).map_err(sql_err)?;
                let max_gap_within_cluster = time::Duration::minutes(max_gap_within_cluster);

                // create the base record
                prefs[index] = Some(Criterion::FacultyPreference {
                    faculty: index,
                    sections: faculty_sections[index].clone(),
                    days_to_check,
                    days_off,
                    evenly_spread,
                    no_room_switch,
                    too_many_rooms,
                    max_gap_within_cluster,
                    distribution_intervals: Vec::new(),
                });
            }

            // if there is no clustering interval than move on to the next faculty
            let is_cluster: Option<i64> = stmt.read(8).map_err(sql_err)?;
            if is_cluster.is_none() {
                continue;
            }

            let is_too_short: i64 = stmt.read(9).map_err(sql_err)?;
            let interval_minutes: i64 = stmt.read(10).map_err(sql_err)?;
            let duration = time::Duration::minutes(interval_minutes);
            let interval_priority: i64 = stmt.read(11).map_err(sql_err)?;
            let priority = interval_priority as u8;

            let interval = match (is_cluster.unwrap() != 0, is_too_short != 0) {
                (true, true) => DistributionInterval::ClusterTooShort { priority, duration },
                (true, false) => DistributionInterval::ClusterTooLong { priority, duration },
                (false, true) => DistributionInterval::GapTooShort { priority, duration },
                (false, false) => DistributionInterval::GapTooLong { priority, duration },
            };

            let Some(Criterion::FacultyPreference { distribution_intervals, .. }) = &mut prefs[index] else {
                panic!("I swear it was here a minute ago!");
            };

            distribution_intervals.push(interval);
        }
    }

    // create scoring criteria for faculty spread preferences
    prefs.into_iter().flatten().for_each(|elt| criteria.push(elt));

    Ok(())
}

fn compute_neighbors(sections: &mut [Section], criteria: &[Criterion]) {
    for (i, criterion) in criteria.iter().enumerate() {
        let neighbors = criterion.get_culpable_sections();
        for &section in &neighbors {
            sections[section].criteria.push(i);
            sections[section].neighbors.append(&mut neighbors.clone());
        }
    }
    for (i, section) in sections.iter_mut().enumerate() {
        section.neighbors.retain(|&elt| elt != i);
        section.neighbors.sort_unstable();
        section.neighbors.dedup();
    }
}

fn sql_err(err: Error) -> String {
    err.to_string()
}

fn time_err(err: time::error::Format) -> String {
    format!("{}", err)
}

fn as_values(list: &[String]) -> Vec<(usize, Value)> {
    let mut out = Vec::new();
    for (i, elt) in list.iter().enumerate() {
        out.push((i + 1, Value::String(elt.clone())));
    }
    out
}

fn double_vec(list: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for elt in list {
        out.push(elt.clone());
    }
    for elt in list {
        out.push(elt.clone());
    }
    out
}

fn dept_clause(departments: &[String], columns: &[String], with_where: bool) -> String {
    let mut s = "".to_string();
    if !departments.is_empty() {
        for (i, col) in columns.iter().enumerate() {
            s = if i == 0 && with_where { format!("WHERE {col} IN (") } else { format!("{s} AND {col} IN (") };
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

pub fn parse_days(weekday_raw: &str) -> Result<Vec<time::Weekday>, String> {
    let mut days = Vec::with_capacity(weekday_raw.len());
    for day in weekday_raw.chars() {
        match day {
            'm' | 'M' => days.push(time::Weekday::Monday),
            't' | 'T' => days.push(time::Weekday::Tuesday),
            'w' | 'W' => days.push(time::Weekday::Wednesday),
            'r' | 'R' => days.push(time::Weekday::Thursday),
            'f' | 'F' => days.push(time::Weekday::Friday),
            's' | 'S' => days.push(time::Weekday::Saturday),
            'u' | 'U' => days.push(time::Weekday::Sunday),
            _ => return Err(format!("Unknown day of week in {}: I only understand mtwrfsu", weekday_raw)),
        }
    }
    Ok(days)
}

pub fn save_schedule(db_path: &str, input: &Input, schedule: &Schedule, comment: &str, existing_id: Option<i64>) -> Result<i64, String> {
    let db =
        Connection::open_with_flags(db_path, OpenFlags::new().with_read_write().with_full_mutex()).map_err(sql_err)?;
    db.execute("PRAGMA foreign_keys = ON").map_err(sql_err)?;
    db.execute("PRAGMA busy_timeout = 10000").map_err(sql_err)?;
    db.execute("BEGIN").map_err(sql_err)?;
    let root_id = if let Some(id) = existing_id {
        // delete old schedule with this id and update base record
        let mut stmt = db.prepare("DELETE FROM placement_sections WHERE placement_id = ?").map_err(sql_err)?;
        stmt.bind((1, id)).map_err(sql_err)?;
        while stmt.next().map_err(sql_err)? != State::Done {
            // no return rows expected
        }
        stmt = db.prepare("DELETE FROM placement_penalties WHERE placement_id = ?").map_err(sql_err)?;
        stmt.bind((1, id)).map_err(sql_err)?;
        while stmt.next().map_err(sql_err)? != State::Done {
            // no return rows expected
        }
        stmt = db.prepare("UPDATE placements SET score = ?, comment = ?, modified_at = ? WHERE placement_id = ?").map_err(sql_err)?;
        stmt.bind((1, format!("{}", schedule.score).as_str())).map_err(sql_err)?;
        stmt.bind((2, comment)).map_err(sql_err)?;
        let now = OffsetDateTime::now_local().unwrap();
        let stamp = now.format(&time::format_description::well_known::Iso8601::DEFAULT).map_err(time_err)?;
        stmt.bind((3, stamp.as_str())).map_err(sql_err)?;
        stmt.bind((4, id)).map_err(sql_err)?;
        while stmt.next().map_err(sql_err)? != State::Done {
            // no return rows expected
        }
        id
    } else {
        // create new base record and capture id
        let mut stmt = db.prepare("INSERT INTO placements (score, comment, created_at, modified_at) VALUES (?, ?, ?, ?) RETURNING placement_id").map_err(sql_err)?;
        stmt.bind((1, format!("{}", schedule.score).as_str())).map_err(sql_err)?;
        stmt.bind((2, comment)).map_err(sql_err)?;
        let now = OffsetDateTime::now_local().unwrap();
        let stamp = now.format(&time::format_description::well_known::Iso8601::DEFAULT).map_err(time_err)?;
        stmt.bind((3, stamp.as_str())).map_err(sql_err)?;
        stmt.bind((4, stamp.as_str())).map_err(sql_err)?;
        let mut id = -1;
        while stmt.next().map_err(sql_err)? == State::Row {
            id = stmt.read(0).map_err(sql_err)?;
        }
        id
    };

    // insert all the placements
    for (section, placement) in schedule.placements.iter().enumerate() {
        let Some(time_slot) = placement.time_slot else {
            // skip unplaced sections
            continue;
        };
        let mut stmt = db.prepare("INSERT INTO placement_sections (placement_id, section, time_slot, room) VALUES (?, ?, ?, ?)").map_err(sql_err)?;
        stmt.bind((1, root_id)).map_err(sql_err)?;
        stmt.bind((2, input.sections[section].name.as_str())).map_err(sql_err)?;
        stmt.bind((3, input.time_slots[time_slot].name.as_str())).map_err(sql_err)?;
        stmt.bind((4, placement.room.map(|room| input.rooms[room].name.as_str()))).map_err(sql_err)?;
        while stmt.next().map_err(sql_err)? != State::Done {
            // no return rows expected
        }
    }

    // gather the penalties
    let mut penalties = Vec::new();
    for (section, placement) in schedule.placements.iter().enumerate() {
        if placement.time_slot.is_none() {
            penalties.push((LEVEL_FOR_UNPLACED_SECTION, format!("{} is not placed", input.sections[section].name), vec![section]));
        }
    }
    for penalty_list in &schedule.penalties {
        for penalty in penalty_list {
            let (priority, msg) = penalty.get_score_message(input, schedule);
            let sections = penalty.get_sections(input);
            penalties.push((priority, msg, sections));
        }
    }

    // insert them
    for (priority, msg, sections) in penalties {
        let mut stmt = db.prepare("INSERT INTO placement_penalties (placement_id, priority, message) VALUES (?, ?, ?) RETURNING placement_penalty_id").map_err(sql_err)?;
        stmt.bind((1, root_id)).map_err(sql_err)?;
        stmt.bind((2, priority as i64)).map_err(sql_err)?;
        stmt.bind((3, msg.as_str())).map_err(sql_err)?;
        let mut id = -1;
        while stmt.next().map_err(sql_err)? == State::Row {
            id = stmt.read(0).map_err(sql_err)?;
        }

        for section in sections {
            let mut stmt = db.prepare("INSERT INTO placement_penalty_sections (placement_penalty_id, section) VALUES (?, ?)").map_err(sql_err)?;
            stmt.bind((1, id)).map_err(sql_err)?;
            stmt.bind((2, input.sections[section].name.as_str())).map_err(sql_err)?;
            while stmt.next().map_err(sql_err)? != State::Done {
                // no return rows expected
            }
        }
    }
    db.execute("COMMIT").map_err(sql_err)?;

    Ok(root_id)
}

