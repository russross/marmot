use sqlite::{Connection, Error, OpenFlags, State, Value};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use std::time::Instant;
use super::score::*;

//
//
// Input data
// All basic data that is loaded at the start and then
// never modified while the solver runs.
//
//

pub struct Input {
    // the name of the term
    pub term_name: String,

    // core schedule data
    pub rooms: Vec<Room>,
    pub time_slots: Vec<TimeSlot>,
    pub faculty: Vec<Faculty>,
    pub sections: Vec<Section>,
    pub anticonflicts: Vec<(u8, usize, Vec<usize>)>,

    // matrix of which time slots overlap which for fast lookup
    pub time_slot_conflicts: Vec<Vec<bool>>,
}

pub struct Room {
    pub name: String,
}

pub struct TimeSlot {
    pub name: String,
    pub days: Vec<time::Weekday>,
    pub start_time: time::Time,
    pub duration: time::Duration,
}

pub struct Faculty {
    pub name: String,
    pub sections: Vec<usize>,
    pub distribution: Vec<DistributionPreference>,
}

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

    // scoring that will be applied specifically to this section
    pub score_criteria: Vec<ScoreCriterion>,

    // any section that might have a scoring interaction with this section
    pub neighbors: Vec<usize>,
}

pub struct RoomWithOptionalPriority {
    pub room: usize,
    pub priority: Option<u8>,
}

pub struct TimeSlotWithOptionalPriority {
    pub time_slot: usize,
    pub priority: Option<u8>,
}

pub struct RoomWithPriority {
    pub room: usize,
    pub priority: u8,
}

pub struct TimeSlotWithPriority {
    pub time_slot: usize,
    pub priority: u8,
}

pub struct SectionWithPriority {
    pub section: usize,
    pub priority: u8,
}

pub enum DistributionPreference {
    // classes on the same day should occur in clusters with tidy gaps between them
    Clustering {
        days: Vec<time::Weekday>,
        max_gap: time::Duration,
        cluster_limits: Vec<DurationWithPriority>,
        gap_limits: Vec<DurationWithPriority>,
    },

    // zero or more days from the list should be free of classes
    DaysOff {
        days: Vec<time::Weekday>,
        days_off: u8,
        priority: u8,
    },

    // days that have classes should have the same number of classes
    DaysEvenlySpread {
        days: Vec<time::Weekday>,
        priority: u8,
    },
}

#[derive(Clone, PartialEq)]
pub enum DurationWithPriority {
    // a duration shorter than this gets a penalty
    TooShort {
        duration: time::Duration,
        priority: u8,
    },

    // a duration longer than this gets a penalty
    TooLong {
        duration: time::Duration,
        priority: u8,
    },
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

impl fmt::Display for DistributionPreference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (name, days) = match self {
            DistributionPreference::Clustering { days, .. } => ("clustering", days),
            DistributionPreference::DaysOff { days, .. } => ("days off", days),
            DistributionPreference::DaysEvenlySpread { days, .. } => ("evenly spread", days),
        };
        write!(f, "{} using days (", name)?;
        let mut sep = "";
        for day in days {
            match day {
                time::Weekday::Sunday => write!(f, "{sep}Sun"),
                time::Weekday::Monday => write!(f, "{sep}Mon"),
                time::Weekday::Tuesday => write!(f, "{sep}Tues"),
                time::Weekday::Wednesday => write!(f, "{sep}Wed"),
                time::Weekday::Thursday => write!(f, "{sep}Thurs"),
                time::Weekday::Friday => write!(f, "{sep}Fri"),
                time::Weekday::Saturday => write!(f, "{sep}Sat"),
            }?;
            sep = ",";
        }
        write!(f, ") ")?;
        match self {
            DistributionPreference::Clustering {
                max_gap,
                cluster_limits,
                gap_limits,
                ..
            } => {
                write!(f, "max gap:{}", max_gap)?;
                if !cluster_limits.is_empty() {
                    write!(f, " ### cluster")?;
                    for limit in cluster_limits {
                        match limit {
                            DurationWithPriority::TooShort { duration, priority } => {
                                write!(f, " [<{} priority {}]", duration, priority)?;
                            }
                            DurationWithPriority::TooLong { duration, priority } => {
                                write!(f, " [>{} priority {}]", duration, priority)?;
                            }
                        }
                    }
                }
                if !gap_limits.is_empty() {
                    write!(f, " ### gap")?;
                    for limit in gap_limits {
                        match limit {
                            DurationWithPriority::TooShort { duration, priority } => {
                                write!(f, " [<{} priority {}]", duration, priority)?;
                            }
                            DurationWithPriority::TooLong { duration, priority } => {
                                write!(f, " [>{} priority {}]", duration, priority)?;
                            }
                        }
                    }
                }
            }

            DistributionPreference::DaysOff {
                days_off, priority, ..
            } => {
                write!(
                    f,
                    "wants {} day{} off priority {}",
                    days_off,
                    if *days_off == 1 { "" } else { "s" },
                    priority
                )?;
            }

            DistributionPreference::DaysEvenlySpread { priority, .. } => {
                write!(f, "priority {}", priority)?;
            }
        }
        Ok(())
    }
}

pub fn setup(db_path: &str, departments: &Vec<String>) -> Result<Input, String> {
    print!("loading input data");
    let start = Instant::now();

    let db =
        Connection::open_with_flags(db_path, OpenFlags::new().with_read_only().with_no_mutex())
            .map_err(sql_err)?;
    db.execute("PRAGMA foreign_keys = ON").map_err(sql_err)?;
    db.execute("PRAGMA temp_store = memory").map_err(sql_err)?;
    db.execute("PRAGMA mmap_size = 100000000")
        .map_err(sql_err)?;

    let mut term_name = "Unknown term".to_string();
    let mut stmt = db.prepare("SELECT term FROM terms").map_err(sql_err)?;
    while let Ok(State::Row) = stmt.next() {
        term_name = stmt.read(0).map_err(sql_err)?;
    }

    let (rooms, room_index) = load_rooms(&db, &departments)?;
    let (time_slots, time_slot_index) = load_time_slots(&db, &departments)?;
    let time_slot_conflicts = load_time_slot_conflicts(&db, &time_slot_index, &departments)?;
    let (mut faculty, faculty_index) = load_faculty(&db, &departments)?;
    let (mut sections, section_index) =
        load_sections(&db, &room_index, &time_slot_index, &departments)?;
    load_conflicts(&db, &mut sections, &section_index, &departments)?;
    let anticonflicts = load_anti_conflicts(&db, &section_index, &departments)?;
    load_faculty_section_assignments(
        &db,
        &mut faculty,
        &faculty_index,
        &mut sections,
        &section_index,
        &departments,
    )?;

    // TODO: compute neighbors, etc.

    println!(": {}ms", start.elapsed().as_millis());

    Ok(Input {
        term_name,
        rooms,
        time_slots,
        faculty,
        sections,
        anticonflicts,
        time_slot_conflicts,
    })
}

// load all rooms
pub fn load_rooms(
    db: &Connection,
    departments: &Vec<String>,
) -> Result<(Vec<Room>, HashMap<String, usize>), String> {
    let dept_in = dept_clause(departments, &vec!["department".into()], true);
    let mut stmt = db
        .prepare(&format!(
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
        let room = Room {
            name: stmt.read(0).map_err(sql_err)?,
        };
        room_index.insert(room.name.clone(), rooms.len());
        rooms.push(room);
    }

    Ok((rooms, room_index))
}

// load all time slots
pub fn load_time_slots(
    db: &Connection,
    departments: &Vec<String>,
) -> Result<(Vec<TimeSlot>, HashMap<String, usize>), String> {
    let dept_in = dept_clause(departments, &vec!["department".into()], true);
    let mut stmt = db
        .prepare(&format!(
            "
            SELECT DISTINCT time_slot, days, start_time, duration
            FROM time_slots_used_by_departments_materialized
            {}
            ORDER BY duration * LENGTH(days), first_day, start_time, duration",
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
            name: name,
            days: parse_days(&days)?,
            start_time: time::Time::from_hms((start_time / 60) as u8, (start_time % 60) as u8, 0)
                .unwrap(),
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
    departments: &Vec<String>,
) -> Result<Vec<Vec<bool>>, String> {
    let dept_in = dept_clause(
        departments,
        &vec!["ts_a.department".into(), "ts_b.department".into()],
        true,
    );
    let mut stmt = db
        .prepare(&format!(
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
    stmt.bind_iter(as_values(&double_vec(departments)))
        .map_err(sql_err)?;

    let time_slot_len = time_slot_index.len();
    let mut conflicts = vec![vec![false; time_slot_len]; time_slot_len];
    while stmt.next().map_err(sql_err)? == State::Row {
        let a: String = stmt.read(0).map_err(sql_err)?;
        let b: String = stmt.read(1).map_err(sql_err)?;
        let &aa = time_slot_index.get(&a).ok_or(format!(
            "time slot {a} reported as conflict but not found in usable time slots"
        ))?;
        let &bb = time_slot_index.get(&b).ok_or(format!(
            "time slot {b} reported as conflict but not found in usable time slots"
        ))?;

        // conflicts go both ways
        conflicts[aa][bb] = true;
        conflicts[bb][aa] = true;
    }

    Ok(conflicts)
}

// load faculty and their availability
pub fn load_faculty(
    db: &Connection,
    departments: &Vec<String>,
) -> Result<(Vec<Faculty>, HashMap<String, usize>), String> {
    let mut faculty_list = Vec::new();
    let mut faculty_lookup = HashMap::new();

    {
        let dept_in = dept_clause(departments, &vec!["department".into()], true);
        let mut stmt = db
            .prepare(&format!(
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
            let faculty = Faculty {
                name: stmt.read(0).map_err(sql_err)?,
                sections: Vec::new(),
                distribution: Vec::new(),
            };
            faculty_lookup.insert(faculty.name.clone(), faculty_list.len());
            faculty_list.push(faculty);
        }
    }
    {
        let dept_in = dept_clause(departments, &vec!["department".into()], true);
        let mut stmt = db.prepare(&format!("
                SELECT  faculty,
                        days_to_check, days_off, days_off_priority, evenly_spread_priority, max_gap_within_cluster,
                        is_cluster, is_too_short, interval_minutes, interval_priority
                FROM faculty_to_be_scheduled_preference_intervals
                {}
                ORDER BY faculty, is_cluster, interval_minutes",
            dept_in)).map_err(sql_err)?;
        stmt.bind_iter(as_values(departments)).map_err(sql_err)?;

        let mut name = String::new();
        let mut index = 0;
        let mut clustering_index = None;
        while stmt.next().map_err(sql_err)? == State::Row {
            let new_name: String = stmt.read(0).map_err(sql_err)?;

            // is this the first row for this faculty?
            if new_name != name {
                name = new_name;
                index = *faculty_lookup
                    .get(&name)
                    .ok_or(format!("faculty not found for {name} but prefs found"))?;
                let faculty = &mut faculty_list[index];
                let days_to_check: String = stmt.read(1).map_err(sql_err)?;

                // days off penalty?
                let days_off_opt: Option<i64> = stmt.read(2).map_err(sql_err)?;
                let days_off_priority_opt: Option<i64> = stmt.read(3).map_err(sql_err)?;
                if let (Some(days_off), Some(priority)) = (days_off_opt, days_off_priority_opt) {
                    faculty.distribution.push(DistributionPreference::DaysOff {
                        days: parse_days(&days_to_check)?,
                        days_off: days_off as u8,
                        priority: priority as u8,
                    });
                }

                // evenly spread penalty?
                let evenly_spread_priority: Option<i64> = stmt.read(4).map_err(sql_err)?;
                if let Some(priority) = evenly_spread_priority {
                    faculty
                        .distribution
                        .push(DistributionPreference::DaysEvenlySpread {
                            days: parse_days(&days_to_check)?,
                            priority: priority as u8,
                        });
                }

                // if there is no clustering interval than move on to the next faculty
                let is_cluster: Option<i64> = stmt.read(6).map_err(sql_err)?;
                if is_cluster.is_none() {
                    continue;
                }

                // create the base clustering record
                let max_gap_within_cluster: i64 = stmt.read(5).map_err(sql_err)?;
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

            let faculty = &mut faculty_list[index];

            let is_cluster: i64 = stmt.read(6).map_err(sql_err)?;
            let is_too_short: i64 = stmt.read(7).map_err(sql_err)?;
            let interval_minutes: i64 = stmt.read(8).map_err(sql_err)?;
            let interval_priority: i64 = stmt.read(9).map_err(sql_err)?;

            let dwp = if is_too_short != 0 {
                DurationWithPriority::TooShort {
                    duration: time::Duration::minutes(interval_minutes),
                    priority: interval_priority as u8,
                }
            } else {
                DurationWithPriority::TooLong {
                    duration: time::Duration::minutes(interval_minutes),
                    priority: interval_priority as u8,
                }
            };

            match &mut faculty.distribution[clustering_index.unwrap()] {
                DistributionPreference::Clustering {
                    cluster_limits,
                    gap_limits,
                    ..
                } => {
                    if is_cluster != 0 {
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

    Ok((faculty_list, faculty_lookup))
}

// load sections and the room/time combinations (plus penalties) associated with them
pub fn load_sections(
    db: &Connection,
    room_index: &HashMap<String, usize>,
    time_slot_index: &HashMap<String, usize>,
    departments: &Vec<String>,
) -> Result<(Vec<Section>, HashMap<String, usize>), String> {
    let mut sections = Vec::new();
    let mut section_index = HashMap::new();

    // load and create sections and their time slots
    {
        let dept_in = dept_clause(departments, &vec!["department".into()], true);
        let mut stmt = db
            .prepare(&format!(
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
                    score_criteria: Vec::new(),
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
                .push(TimeSlotWithOptionalPriority {
                    time_slot,
                    priority: priority.map(|elt| elt as u8),
                });
        }
    }

    // add rooms
    {
        let dept_in = dept_clause(departments, &vec!["department".into()], true);
        let mut stmt = db
            .prepare(&format!(
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
                index = Some(section_index.get(&new_section_name).ok_or(format!(
                    "section {} not found but has room {} assigned",
                    section_name, room_name
                ))?);
            }

            let room = *room_index.get(&room_name).ok_or(format!(
                "section {} assigned to room {} but room not found",
                section_name, room_name
            ))?;
            sections[*index.unwrap()].rooms.push(RoomWithOptionalPriority {
                room,
                priority: priority.map(|elt| elt as u8),
            });
        }
    }

    Ok((sections, section_index))
}

pub fn load_conflicts(
    db: &Connection,
    sections: &mut Vec<Section>,
    section_index: &HashMap<String, usize>,
    departments: &Vec<String>,
) -> Result<(), String> {
    let dept_in = dept_clause(
        departments,
        &vec!["department_a".into(), "department_b".into()],
        true,
    );
    let mut stmt = db
        .prepare(&format!(
            "
            SELECT DISTINCT section_a, section_b, priority
            FROM conflict_pairs_materialized
            {}
            ORDER BY section_a, section_b",
            dept_in
        ))
        .map_err(sql_err)?;
    stmt.bind_iter(as_values(&double_vec(departments)))
        .map_err(sql_err)?;

    let mut soft_conflict_lists = Vec::new();
    for _ in 0..sections.len() {
        soft_conflict_lists.push(Vec::new());
    }

    while stmt.next().map_err(sql_err)? == State::Row {
        let section_a: String = stmt.read(0).map_err(sql_err)?;
        let section_b: String = stmt.read(1).map_err(sql_err)?;
        let priority: i64 = stmt.read(2).map_err(sql_err)?;

        let index_a = *section_index.get(&section_a).ok_or(format!(
            "section_a {section_a} from conflict pair not found"
        ))?;
        let index_b = *section_index.get(&section_b).ok_or(format!(
            "section_b {section_b} from conflict pair not found"
        ))?;
        if priority < 1 || priority >= 10 {
            return Err(format!(
                "conflict pair {section_a} vs {section_b} has invalid priority of {priority}"
            ));
        }
        if priority == 1 {
            sections[index_a].hard_conflicts.push(index_b);
        } else {
            soft_conflict_lists[index_a].push(SectionWithPriority {
                section: index_b,
                priority: priority as u8,
            });
        }
    }

    // add all non-empty soft conflict lists as score criteria
    for (section, list) in soft_conflict_lists.into_iter().enumerate() {
        if !list.is_empty() {
            sections[section]
                .score_criteria
                .push(ScoreCriterion::SoftConflict {
                    sections_with_priorities: list,
                });
        }
    }

    Ok(())
}

pub fn load_anti_conflicts(
    db: &Connection,
    section_index: &HashMap<String, usize>,
    departments: &Vec<String>,
) -> Result<Vec<(u8, usize, Vec<usize>)>, String> {
    let mut anticonflicts = Vec::new();

    let dept_in = dept_clause(
        departments,
        &vec!["single_department".into(), "group_department".into()],
        true,
    );
    let mut stmt = db
        .prepare(&format!(
            "
            SELECT DISTINCT single_section, group_section, priority
            FROM anti_conflict_pairs
            {}
            ORDER BY single_section",
            dept_in
        ))
        .map_err(sql_err)?;
    stmt.bind_iter(as_values(&double_vec(departments)))
        .map_err(sql_err)?;

    let mut single_name = String::new();
    let mut group = Vec::new();
    let mut priority: u8 = 0;

    while stmt.next().map_err(sql_err)? == State::Row {
        let new_single_name: String = stmt.read(0).map_err(sql_err)?;
        let other_name: String = stmt.read(1).map_err(sql_err)?;
        let other = *section_index.get(&other_name).ok_or(format!(
            "anticonflict references unknown section {other_name}"
        ))?;
        let pri: i64 = stmt.read(2).map_err(sql_err)?;

        // start a new anti conflict
        if new_single_name != single_name {
            // close the previous one out
            if !group.is_empty() {
                group.sort();
                group.dedup();
                let entry = (
                    priority,
                    *section_index
                        .get(&single_name)
                        .ok_or(format!("anticonflict for unknown section {single_name}"))?,
                    std::mem::take(&mut group),
                );
                anticonflicts.push(entry);
            }

            // start a new one
            single_name = new_single_name;
            priority = pri as u8;
        }
        group.push(other);
    }

    // close the final one out
    if !group.is_empty() {
        let entry = (
            priority,
            *section_index
                .get(&single_name)
                .ok_or(format!("anticonflict for unknown section {single_name}"))?,
            std::mem::take(&mut group),
        );
        anticonflicts.push(entry);
    }

    Ok(anticonflicts)
}

pub fn load_faculty_section_assignments(
    db: &Connection,
    faculty: &mut Vec<Faculty>,
    faculty_index: &HashMap<String, usize>,
    sections: &mut Vec<Section>,
    section_index: &HashMap<String, usize>,
    departments: &Vec<String>,
) -> Result<(), String> {
    // link sections to faculty
    let dept_in = dept_clause(departments, &vec!["department".into()], true);
    let mut stmt = db
        .prepare(&format!(
            "
            SELECT DISTINCT faculty, section
            FROM faculty_sections_to_be_scheduled
            {}",
            dept_in
        ))
        .map_err(sql_err)?;
    stmt.bind_iter(as_values(departments)).map_err(sql_err)?;

    while stmt.next().map_err(sql_err)? == State::Row {
        let faculty_name: String = stmt.read(0).map_err(sql_err)?;
        let section_name: String = stmt.read(1).map_err(sql_err)?;
        let faculty_i = *faculty_index.get(&faculty_name).ok_or(format!(
            "faculty {faculty_name} not found in mapping to section {section_name}"
        ))?;
        let section_i = *section_index.get(&section_name).ok_or(format!(
            "section {section_name} not found in mapping to faculty {faculty_name}"
        ))?;

        faculty[faculty_i].sections.push(section_i);
        sections[section_i].faculty.push(faculty_i);
    }

    Ok(())
}

fn sql_err(err: Error) -> String {
    err.to_string()
}

fn as_values(list: &Vec<String>) -> Vec<(usize, Value)> {
    let mut out = Vec::new();
    for (i, elt) in list.iter().enumerate() {
        out.push((i + 1, Value::String(elt.clone())));
    }
    out
}

fn double_vec(list: &Vec<String>) -> Vec<String> {
    let mut out = Vec::new();
    for elt in list {
        out.push(elt.clone());
    }
    for elt in list {
        out.push(elt.clone());
    }
    out
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
            _ => {
                return Err(format!(
                    "Unknown day of week in {}: I only understand mtwrfsu",
                    weekday_raw
                ))
            }
        }
    }
    Ok(days)
}
