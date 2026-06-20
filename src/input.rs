#![allow(clippy::collapsible_if)]

use super::error::{Result, err};
use super::faculty_preferences::{FacultyPreferencePriorityPolicy, rebalance_faculty_preferences};
use super::score::*;
use super::solver::*;
use sqlite::{Connection, OpenFlags, State, Value};
use std::cmp::min;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use std::ops;
use std::time::Instant;

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
    pub faculty_preference_priority_policy: FacultyPreferencePriorityPolicy,

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
    pub days: Days,
    pub start_time: Time,
    pub duration: Duration,
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

#[derive(Clone, Copy, Ord, PartialOrd, PartialEq, Eq)]
pub struct Time {
    pub minutes: u16,
}

impl Time {
    pub fn new(minutes: u16) -> Self {
        Time { minutes }
    }
}

impl ops::Add<Duration> for Time {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self {
        Time { minutes: self.minutes + rhs.minutes }
    }
}

impl ops::Sub for Time {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Duration {
        assert!(self.minutes >= rhs.minutes);
        Duration { minutes: self.minutes - rhs.minutes }
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.minutes / 60;
        let mins = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, mins)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Duration {
    pub minutes: u16,
}

impl Duration {
    pub fn new(minutes: u16) -> Self {
        Duration { minutes }
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut n = self.minutes;
        if n == 0 {
            write!(f, "0m")?;
        } else {
            if n >= 60 {
                write!(f, "{}h", n / 60)?;
                n %= 60;
            }
            if n > 0 {
                write!(f, "{}m", n)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Days {
    pub days: u8,
}

impl Days {
    pub fn parse(weekday_raw: &str) -> Result<Self> {
        let mut days = 0;
        for day in weekday_raw.chars() {
            match day {
                'm' | 'M' => days |= 0b0000001,
                't' | 'T' => days |= 0b0000010,
                'w' | 'W' => days |= 0b0000100,
                'r' | 'R' => days |= 0b0001000,
                'f' | 'F' => days |= 0b0010000,
                's' | 'S' => days |= 0b0100000,
                'u' | 'U' => days |= 0b1000000,
                _ => return Err(format!("Unknown day of week in {}: I only understand mtwrfsu", weekday_raw).into()),
            }
        }
        Ok(Days { days })
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        for day in 0..7 {
            if self.days & (1 << day) != 0 {
                count += 1;
            }
        }
        count
    }

    pub fn is_empty(&self) -> bool {
        self.days == 0
    }

    pub fn contains(&self, day: u8) -> bool {
        self.days & (1 << day) != 0
    }

    pub fn intersect(&self, other: &Self) -> Self {
        Days { days: self.days & other.days }
    }
}

impl fmt::Display for Days {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let letters = ['M', 'T', 'W', 'R', 'F', 'S', 'U'];
        for day in 0..7 {
            if self.days & (1 << day) != 0 {
                write!(f, "{}", letters[day as usize])?;
            }
        }
        Ok(())
    }
}

pub struct DaysIter {
    days: u8,
    current_day: u8,
}

impl Iterator for DaysIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        // Check all remaining bits
        while self.current_day < 7 {
            let day = self.current_day;
            self.current_day += 1;

            // If this bit is set, return the day
            if self.days & (1 << day) != 0 {
                return Some(day);
            }
        }
        None
    }
}

impl IntoIterator for Days {
    type Item = u8;
    type IntoIter = DaysIter;

    fn into_iter(self) -> Self::IntoIter {
        DaysIter { days: self.days, current_day: 0 }
    }
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
            write!(&mut s, " assigned: ").unwrap();
            let mut sep = "";
            for &elt in &self.sections {
                write!(&mut s, "{}{}", sep, input.sections[elt].name).unwrap();
                sep = ", ";
            }
        }

        s
    }
}

pub fn load_input(
    db_path: &str,
    departments: &[String],
    faculty_preference_priority_policy: FacultyPreferencePriorityPolicy,
    show_faculty_preference_priorities: bool,
) -> Result<Input> {
    if show_faculty_preference_priorities
        && faculty_preference_priority_policy == FacultyPreferencePriorityPolicy::Stated
    {
        return err("--show-faculty-preference-priorities requires faculty preference balancing");
    }
    print!("loading input data");
    let start = Instant::now();

    let db = Connection::open_with_flags(db_path, OpenFlags::new().with_read_only().with_no_mutex())?;
    db.execute("PRAGMA foreign_keys = ON")?;
    db.execute("PRAGMA temp_store = memory")?;
    db.execute("PRAGMA mmap_size = 100000000")?;

    let mut term_name = "Unknown term".to_string();
    let mut stmt = db.prepare("SELECT term FROM terms")?;
    while stmt.next()? == State::Row {
        term_name = stmt.read(0)?;
    }

    let (rooms, room_index) = load_rooms(&db, departments)?;
    let (time_slots, time_slot_index) = load_time_slots(&db, departments)?;
    let time_slot_conflicts = load_time_slot_conflicts(&db, &time_slot_index, departments)?;
    let (mut faculty, faculty_index) = load_faculty(&db, departments)?;
    let (mut sections, section_index, mut criteria) = load_sections(&db, &room_index, &time_slot_index, departments)?;
    load_conflicts(&db, &mut sections, &section_index, &mut criteria, departments)?;
    load_anti_conflicts(&db, &sections, &section_index, &mut criteria, departments)?;
    load_time_pattern_matches(&db, &faculty_index, &section_index, &mut criteria, departments)?;
    load_faculty_section_assignments(
        &db,
        &mut faculty,
        &faculty_index,
        &mut sections,
        &section_index,
        &mut criteria,
        departments,
    )?;
    if faculty_preference_priority_policy == FacultyPreferencePriorityPolicy::EntropyBalancedV1 {
        expand_bundled_faculty_preferences(&mut criteria);
        load_owned_room_time_preferences(
            &db,
            &faculty,
            &faculty_index,
            &section_index,
            &room_index,
            &time_slot_index,
            &mut criteria,
            departments,
        )?;
    } else {
        load_collapsed_room_time_preferences(&sections, &mut criteria);
    }

    compute_neighbors(&mut sections, &criteria);
    println!(" took {}ms", start.elapsed().as_millis());

    let mut input = Input {
        term_name,
        rooms,
        time_slots,
        faculty,
        sections,
        criteria,
        faculty_preference_priority_policy,
        time_slot_conflicts,
    };
    if faculty_preference_priority_policy == FacultyPreferencePriorityPolicy::EntropyBalancedV1 {
        rebalance_faculty_preferences(&mut input, show_faculty_preference_priorities)?;
    }

    Ok(input)
}

fn load_collapsed_room_time_preferences(sections: &[Section], criteria: &mut Vec<Criterion>) {
    for (section_index, section) in sections.iter().enumerate() {
        let rooms_with_priorities: Vec<RoomWithPriority> = section
            .rooms
            .iter()
            .filter_map(|option| option.priority.map(|priority| RoomWithPriority { room: option.room, priority }))
            .collect();
        if !rooms_with_priorities.is_empty() {
            criteria.push(Criterion::RoomPreference { section: section_index, rooms_with_priorities });
        }

        let time_slots_with_priorities: Vec<TimeSlotWithPriority> = section
            .time_slots
            .iter()
            .filter_map(|option| {
                option.priority.map(|priority| TimeSlotWithPriority { time_slot: option.time_slot, priority })
            })
            .collect();
        if !time_slots_with_priorities.is_empty() {
            criteria.push(Criterion::TimeSlotPreference { section: section_index, time_slots_with_priorities });
        }
    }
}

fn expand_bundled_faculty_preferences(criteria: &mut Vec<Criterion>) {
    let mut expanded = Vec::with_capacity(criteria.len());
    for criterion in criteria.drain(..) {
        let Criterion::FacultyPreference {
            faculty,
            sections,
            days_to_check,
            days_off,
            evenly_spread,
            no_room_switch,
            too_many_rooms,
            max_gap_within_cluster,
            distribution_intervals,
        } = criterion
        else {
            expanded.push(criterion);
            continue;
        };

        let mut push = |priority: u8, kind: FacultyPreferenceKind| {
            expanded.push(Criterion::OwnedFacultyPreference(FacultyPreference {
                faculty,
                sections: sections.clone(),
                stated_priority: priority,
                priority,
                kind,
            }));
        };
        if let Some((priority, desired)) = days_off {
            push(priority, FacultyPreferenceKind::DaysOff { days_to_check, desired });
        }
        if let Some(priority) = evenly_spread {
            push(priority, FacultyPreferenceKind::EvenlySpread { days_to_check });
        }
        if let Some(priority) = no_room_switch {
            push(priority, FacultyPreferenceKind::NoRoomSwitch { days_to_check, max_gap: max_gap_within_cluster });
        }
        if let Some((priority, desired_max_rooms)) = too_many_rooms {
            push(priority, FacultyPreferenceKind::TooManyRooms { desired_max_rooms });
        }
        for interval in distribution_intervals {
            let (priority, kind) = match interval {
                DistributionInterval::GapTooLong { priority, duration } => (
                    priority,
                    FacultyPreferenceKind::GapTooLong { days_to_check, duration, max_gap: max_gap_within_cluster },
                ),
                DistributionInterval::GapTooShort { priority, duration } => (
                    priority,
                    FacultyPreferenceKind::GapTooShort { days_to_check, duration, max_gap: max_gap_within_cluster },
                ),
                DistributionInterval::ClusterTooLong { priority, duration } => (
                    priority,
                    FacultyPreferenceKind::ClusterTooLong { days_to_check, duration, max_gap: max_gap_within_cluster },
                ),
                DistributionInterval::ClusterTooShort { priority, duration } => (
                    priority,
                    FacultyPreferenceKind::ClusterTooShort { days_to_check, duration, max_gap: max_gap_within_cluster },
                ),
            };
            push(priority, kind);
        }
    }
    *criteria = expanded;
}

#[allow(clippy::too_many_arguments)]
fn load_owned_room_time_preferences(
    db: &Connection,
    faculty_list: &[Faculty],
    faculty_index: &HashMap<String, usize>,
    section_index: &HashMap<String, usize>,
    room_index: &HashMap<String, usize>,
    time_slot_index: &HashMap<String, usize>,
    criteria: &mut Vec<Criterion>,
    departments: &[String],
) -> Result<()> {
    let dept_in = dept_clause(departments, &["department".into()], true);
    let mut stmt = db.prepare(format!(
        "SELECT DISTINCT faculty, section, preference_priority, room
         FROM faculty_room_preferences_to_be_scheduled
         {dept_in}
         ORDER BY faculty, section, preference_priority, room"
    ))?;
    stmt.bind_iter(as_values(departments))?;
    let mut current: Option<(usize, usize, u8, Vec<usize>)> = None;
    while stmt.next()? == State::Row {
        let faculty_name: String = stmt.read(0)?;
        let section_name: String = stmt.read(1)?;
        let priority = stmt.read::<i64, _>(2)? as u8;
        let room_name: String = stmt.read(3)?;
        let faculty = *faculty_index.get(&faculty_name).ok_or(format!("unknown faculty {faculty_name}"))?;
        let section = *section_index.get(&section_name).ok_or(format!("unknown section {section_name}"))?;
        let room = *room_index.get(&room_name).ok_or(format!("unknown room {room_name}"))?;
        if !matches!(&current, Some((f, s, p, _)) if *f == faculty && *s == section && *p == priority) {
            if let Some((old_faculty, old_section, old_priority, rooms)) = current.take() {
                criteria.push(Criterion::OwnedFacultyPreference(FacultyPreference {
                    faculty: old_faculty,
                    sections: faculty_list[old_faculty].sections.clone(),
                    stated_priority: old_priority,
                    priority: old_priority,
                    kind: FacultyPreferenceKind::AvoidRooms { section: old_section, rooms },
                }));
            }
            current = Some((faculty, section, priority, Vec::new()));
        }
        current.as_mut().unwrap().3.push(room);
    }
    if let Some((faculty, section, priority, rooms)) = current {
        criteria.push(Criterion::OwnedFacultyPreference(FacultyPreference {
            faculty,
            sections: faculty_list[faculty].sections.clone(),
            stated_priority: priority,
            priority,
            kind: FacultyPreferenceKind::AvoidRooms { section, rooms },
        }));
    }

    let mut stmt = db.prepare(format!(
        "SELECT DISTINCT faculty, section, preference_priority, time_slot
         FROM faculty_time_preferences_to_be_scheduled
         {dept_in}
         ORDER BY faculty, section, preference_priority, time_slot"
    ))?;
    stmt.bind_iter(as_values(departments))?;
    let mut current: Option<(usize, usize, u8, Vec<usize>)> = None;
    while stmt.next()? == State::Row {
        let faculty_name: String = stmt.read(0)?;
        let section_name: String = stmt.read(1)?;
        let priority = stmt.read::<i64, _>(2)? as u8;
        let time_slot_name: String = stmt.read(3)?;
        let faculty = *faculty_index.get(&faculty_name).ok_or(format!("unknown faculty {faculty_name}"))?;
        let section = *section_index.get(&section_name).ok_or(format!("unknown section {section_name}"))?;
        let time_slot = *time_slot_index.get(&time_slot_name).ok_or(format!("unknown time slot {time_slot_name}"))?;
        if !matches!(&current, Some((f, s, p, _)) if *f == faculty && *s == section && *p == priority) {
            if let Some((old_faculty, old_section, old_priority, time_slots)) = current.take() {
                criteria.push(Criterion::OwnedFacultyPreference(FacultyPreference {
                    faculty: old_faculty,
                    sections: faculty_list[old_faculty].sections.clone(),
                    stated_priority: old_priority,
                    priority: old_priority,
                    kind: FacultyPreferenceKind::AvoidTimeSlots { section: old_section, time_slots },
                }));
            }
            current = Some((faculty, section, priority, Vec::new()));
        }
        current.as_mut().unwrap().3.push(time_slot);
    }
    if let Some((faculty, section, priority, time_slots)) = current {
        criteria.push(Criterion::OwnedFacultyPreference(FacultyPreference {
            faculty,
            sections: faculty_list[faculty].sections.clone(),
            stated_priority: priority,
            priority,
            kind: FacultyPreferenceKind::AvoidTimeSlots { section, time_slots },
        }));
    }

    Ok(())
}

// load all rooms
pub fn load_rooms(db: &Connection, departments: &[String]) -> Result<(Vec<Room>, HashMap<String, usize>)> {
    let dept_in = dept_clause(departments, &["department".into()], true);
    let mut stmt = db.prepare(format!(
        "
            SELECT DISTINCT room
            FROM rooms_used_by_departments
            {}
            ORDER BY building, CAST (room_number AS INTEGER), room_number",
        dept_in
    ))?;
    stmt.bind_iter(as_values(departments))?;

    let mut rooms = Vec::new();
    let mut room_index = HashMap::new();
    while stmt.next()? == State::Row {
        let room = Room { name: stmt.read(0)? };
        room_index.insert(room.name.clone(), rooms.len());
        rooms.push(room);
    }

    Ok((rooms, room_index))
}

// load all time slots
pub fn load_time_slots(db: &Connection, departments: &[String]) -> Result<(Vec<TimeSlot>, HashMap<String, usize>)> {
    let dept_in = dept_clause(departments, &["department".into()], true);
    let mut stmt = db.prepare(format!(
        "
            SELECT DISTINCT time_slot, days, start_time, duration
            FROM time_slots_used_by_departments
            {}
            ORDER BY first_day, start_time, duration, duration * LENGTH(days)",
        dept_in
    ))?;
    stmt.bind_iter(as_values(departments))?;

    let mut time_slots = Vec::new();
    let mut time_slot_index = HashMap::new();
    while stmt.next()? == State::Row {
        let name: String = stmt.read(0)?;
        let days: String = stmt.read(1)?;
        let days = Days::parse(&days)?;
        let start_time: i64 = stmt.read(2)?;
        let start_time = Time::new(start_time as u16);
        let duration: i64 = stmt.read(3)?;
        let duration = Duration::new(duration as u16);
        let time_slot = TimeSlot { name, days, start_time, duration };
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
) -> Result<Vec<Vec<bool>>> {
    let dept_in = dept_clause(departments, &["ts_a.department".into(), "ts_b.department".into()], true);
    let mut stmt = db.prepare(format!(
        "
            SELECT time_slot_a, time_slot_b
            FROM conflicting_time_slots
            JOIN time_slots_used_by_departments AS ts_a
                ON  time_slot_a = ts_a.time_slot
            JOIN time_slots_used_by_departments AS ts_b
                ON  time_slot_b = ts_b.time_slot
            {}",
        dept_in
    ))?;
    stmt.bind_iter(as_values(&double_vec(departments)))?;

    let time_slot_len = time_slot_index.len();
    let mut conflicts = vec![vec![false; time_slot_len]; time_slot_len];
    while stmt.next()? == State::Row {
        let a: String = stmt.read(0)?;
        let b: String = stmt.read(1)?;
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
pub fn load_faculty(db: &Connection, departments: &[String]) -> Result<(Vec<Faculty>, HashMap<String, usize>)> {
    let mut faculty_list = Vec::new();
    let mut faculty_lookup = HashMap::new();

    {
        let dept_in = dept_clause(departments, &["department".into()], true);
        let mut stmt = db.prepare(format!(
            "
                SELECT DISTINCT faculty
                FROM faculty_sections_to_be_scheduled
                {}
                ORDER BY faculty",
            dept_in
        ))?;
        stmt.bind_iter(as_values(departments))?;

        while stmt.next()? == State::Row {
            let faculty = Faculty { name: stmt.read(0)?, sections: Vec::new() };
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
) -> Result<(Vec<Section>, HashMap<String, usize>, Vec<Criterion>)> {
    let mut sections = Vec::new();
    let mut section_index = HashMap::new();
    let criteria = Vec::new();

    // load and create sections and their time slots
    {
        let dept_in = dept_clause(departments, &["department".into()], true);
        let mut stmt = db.prepare(format!(
            "
                SELECT DISTINCT section, time_slot, time_slot_priority
                FROM time_slots_available_to_sections
                {}
                ORDER BY section",
            dept_in
        ))?;
        stmt.bind_iter(as_values(departments))?;

        let mut section_name = String::new();
        while stmt.next()? == State::Row {
            let new_section_name: String = stmt.read(0)?;
            let time_slot_name: String = stmt.read(1)?;
            let priority: Option<i64> = stmt.read(2)?;

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
        let mut stmt = db.prepare(format!(
            "
                SELECT DISTINCT section, room, room_priority
                FROM rooms_available_to_sections
                {}
                ORDER BY section",
            dept_in
        ))?;
        stmt.bind_iter(as_values(departments))?;

        let mut section_name = String::new();
        let mut index = None;

        while stmt.next()? == State::Row {
            let new_section_name: String = stmt.read(0)?;
            let room_name: String = stmt.read(1)?;
            let priority: Option<i64> = stmt.read(2)?;

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

    Ok((sections, section_index, criteria))
}

pub fn load_conflicts(
    db: &Connection,
    sections: &mut [Section],
    section_index: &HashMap<String, usize>,
    criteria: &mut Vec<Criterion>,
    departments: &[String],
) -> Result<()> {
    let dept_in = dept_clause(departments, &["department_a".into(), "department_b".into()], false);
    let mut stmt = db.prepare(format!(
        "
            SELECT DISTINCT section_a, section_b, priority
            FROM conflict_pairs
            WHERE section_a < section_b
            {}
            ORDER BY section_a, section_b",
        dept_in
    ))?;
    stmt.bind_iter(as_values(&double_vec(departments)))?;

    while stmt.next()? == State::Row {
        let section_a: String = stmt.read(0)?;
        let section_b: String = stmt.read(1)?;
        let priority: i64 = stmt.read(2)?;

        let index_a =
            *section_index.get(&section_a).ok_or(format!("section_a {section_a} from conflict pair not found"))?;
        let index_b =
            *section_index.get(&section_b).ok_or(format!("section_b {section_b} from conflict pair not found"))?;
        if priority < LEVEL_FOR_HARD_CONFLICT as i64 || priority >= START_LEVEL_FOR_PREFERENCES as i64 {
            return Err(format!("conflict pair {section_a} vs {section_b} has invalid priority of {priority}").into());
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
    _sections: &[Section],
    section_index: &HashMap<String, usize>,
    criteria: &mut Vec<Criterion>,
    departments: &[String],
) -> Result<()> {
    let dept_in = dept_clause(departments, &["single_department".into(), "group_department".into()], true);
    // note: single_section can only have one anticonflict rule
    let mut stmt = db.prepare(format!(
        "
            SELECT DISTINCT single_section, group_section, priority
            FROM anti_conflict_pairs
            {}
            ORDER BY single_section, priority, group_section",
        dept_in
    ))?;
    stmt.bind_iter(as_values(&double_vec(departments)))?;

    let mut criterion = None;

    while stmt.next()? == State::Row {
        let new_single_name: String = stmt.read(0)?;
        let new_single = *section_index
            .get(&new_single_name)
            .ok_or(format!("anticonflict for unknown section {new_single_name}"))?;
        let other_name: String = stmt.read(1)?;
        let other =
            *section_index.get(&other_name).ok_or(format!("anticonflict references unknown section {other_name}"))?;
        if new_single == other {
            panic!("anticonflict: single and group names must differ");
        }
        let pri: i64 = stmt.read(2)?;

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

pub fn load_time_pattern_matches(
    db: &Connection,
    faculty_index: &HashMap<String, usize>,
    section_index: &HashMap<String, usize>,
    criteria: &mut Vec<Criterion>,
    departments: &[String],
) -> Result<()> {
    let dept_in = dept_clause(departments, &["department".into()], true);
    let mut stmt = db.prepare(format!(
        "
            SELECT DISTINCT faculty, preference_name, preference_priority, section
            FROM faculty_time_pattern_preferences_to_be_scheduled
            {}
            ORDER BY faculty, preference_name, preference_priority, section",
        dept_in
    ))?;
    stmt.bind_iter(as_values(departments))?;

    let mut criterion: Option<(String, usize, u8, Vec<usize>)> = None;
    while stmt.next()? == State::Row {
        let faculty_name: String = stmt.read(0)?;
        let group_name: String = stmt.read(1)?;
        let pri: i64 = stmt.read(2)?;
        let section_name: String = stmt.read(3)?;
        let faculty = *faculty_index
            .get(&faculty_name)
            .ok_or(format!("time pattern preference references unknown faculty {faculty_name}"))?;
        let section = *section_index
            .get(&section_name)
            .ok_or(format!("time pattern match {group_name} references unknown section {section_name}"))?;
        let key = format!("{faculty_name}\0{group_name}");

        match &mut criterion {
            Some((name, _, _, sections)) if *name == key => {
                sections.push(section);
            }
            _ => {
                if let Some((_, old_faculty, old_priority, sections)) = criterion
                    && sections.len() > 1
                {
                    criteria.push(Criterion::OwnedFacultyPreference(FacultyPreference {
                        faculty: old_faculty,
                        sections: sections.clone(),
                        stated_priority: old_priority,
                        priority: old_priority,
                        kind: FacultyPreferenceKind::TimePatternMatch { sections },
                    }));
                }
                criterion = Some((key, faculty, pri as u8, vec![section]));
            }
        }
    }

    if let Some((_, faculty, priority, sections)) = criterion
        && sections.len() > 1
    {
        criteria.push(Criterion::OwnedFacultyPreference(FacultyPreference {
            faculty,
            sections: sections.clone(),
            stated_priority: priority,
            priority,
            kind: FacultyPreferenceKind::TimePatternMatch { sections },
        }));
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
) -> Result<()> {
    let mut faculty_sections = Vec::new();
    for _ in 0..faculty_list.len() {
        faculty_sections.push(Vec::new());
    }
    {
        // link sections to faculty
        let dept_in = dept_clause(departments, &["department".into()], true);
        let mut stmt = db.prepare(format!(
            "
                SELECT DISTINCT faculty, section
                FROM faculty_sections_to_be_scheduled
                {}
                ORDER BY faculty, section",
            dept_in
        ))?;
        stmt.bind_iter(as_values(departments))?;

        while stmt.next()? == State::Row {
            let faculty_name: String = stmt.read(0)?;
            let section_name: String = stmt.read(1)?;
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

        let mut min_k = usize::MAX;
        'set_loop: for key in 1..(1 << all_possible.len()) {
            // count the bits
            let mut bit_count = 0;
            for i in 0..all_possible.len() {
                if key & (1 << i) != 0 {
                    bit_count += 1;
                }
            }

            // we only care about values of k < section count
            if bit_count >= section_list.len() {
                continue;
            }

            // check every section
            'section_loop: for &section in &section_list {
                // is this section satisfied by one of the rooms in the set?
                for (i, &room) in all_possible.iter().enumerate() {
                    // only consider rooms in the set
                    if key & (1 << i) != 0 && sections[section].rooms.iter().any(|elt| elt.room == room) {
                        continue 'section_loop;
                    }
                }

                // failed to find a suitable room
                continue 'set_loop;
            }

            // success
            min_k = min(min_k, bit_count);
        }

        // do not bother if the best we can do is a distinct room per section
        if min_k >= section_list.len() {
            continue;
        }

        min_rooms[faculty] = Some(min_k);
    }

    // load faculty spread preferences
    let mut prefs = vec![None; faculty_list.len()];
    {
        let dept_in = dept_clause(departments, &["department".into()], true);
        let mut stmt = db.prepare(format!(
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
        ))?;
        stmt.bind_iter(as_values(departments))?;

        while stmt.next()? == State::Row {
            let name: String = stmt.read(0)?;
            let index = *faculty_index.get(&name).ok_or(format!("faculty not found for {name} but prefs found"))?;

            // is this the first row for this faculty?
            if prefs[index].is_none() {
                let days_to_check: String = stmt.read(1)?;
                let days_to_check = Days::parse(&days_to_check)?;

                // days off penalty?
                let days_off_opt: Option<i64> = stmt.read(2)?;
                let days_off_priority_opt: Option<i64> = stmt.read(3)?;
                let days_off = if let (Some(days_off), Some(priority)) = (days_off_opt, days_off_priority_opt) {
                    Some((priority as u8, days_off as usize))
                } else {
                    None
                };

                // evenly spread penalty?
                let evenly_spread_priority: Option<i64> = stmt.read(4)?;
                let evenly_spread = evenly_spread_priority.map(|priority| priority as u8);

                // no room switch penalty?
                let no_room_switch_priority: Option<i64> = stmt.read(5)?;
                let no_room_switch = no_room_switch_priority.map(|priority| priority as u8);

                // too many rooms penalty?
                let too_many_rooms_priority: Option<i64> = stmt.read(6)?;
                let too_many_rooms = if let Some(priority) = too_many_rooms_priority {
                    min_rooms[index].map(|k| (priority as u8, k))
                } else {
                    None
                };

                let max_gap_within_cluster: i64 = stmt.read(7)?;
                let max_gap_within_cluster = Duration::new(max_gap_within_cluster as u16);

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
            let is_cluster: Option<i64> = stmt.read(8)?;
            if is_cluster.is_none() {
                continue;
            }
            let is_cluster = is_cluster.unwrap() != 0;

            let is_too_short: i64 = stmt.read(9)?;
            let is_too_short = is_too_short != 0;
            let duration: i64 = stmt.read(10)?;
            let duration = Duration::new(duration as u16);
            let interval_priority: i64 = stmt.read(11)?;
            let priority = interval_priority as u8;

            let interval = match (is_cluster, is_too_short) {
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

pub fn save_schedule(
    db_path: &str,
    input: &Input,
    schedule: &Schedule,
    comment: &str,
    existing_id: Option<i64>,
) -> Result<i64> {
    let db = Connection::open_with_flags(db_path, OpenFlags::new().with_read_write().with_full_mutex())?;
    db.execute("PRAGMA foreign_keys = ON")?;
    db.execute("PRAGMA busy_timeout = 10000")?;
    db.execute("BEGIN")?;
    let optimum_score_prefix_json =
        format!("[{}]", schedule.optimum_score_prefix.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
    let root_id = if let Some(id) = existing_id {
        // delete old schedule with this id and update base record
        let mut stmt = db.prepare(
            "DELETE FROM placement_sections
            WHERE placement_id = ?",
        )?;
        stmt.bind((1, id))?;
        while stmt.next()? != State::Done {
            // no return rows expected
        }
        stmt = db.prepare(
            "DELETE FROM placement_penalties
            WHERE placement_id = ?",
        )?;
        stmt.bind((1, id))?;
        while stmt.next()? != State::Done {
            // no return rows expected
        }
        stmt = db.prepare(
            "UPDATE placements
            SET score = ?,
            sort_score = ?,
            optimum_score_prefix = ?,
            faculty_preference_priority_policy = ?,
            comment = ?,
            modified_at = DATETIME('now', 'localtime')
            WHERE placement_id = ?",
        )?;
        stmt.bind((1, format!("{}", schedule.score).as_str()))?;
        stmt.bind((2, schedule.score.sortable().as_str()))?;
        stmt.bind((3, optimum_score_prefix_json.as_str()))?;
        stmt.bind((4, input.faculty_preference_priority_policy.database_name()))?;
        stmt.bind((5, comment))?;
        stmt.bind((6, id))?;
        if stmt.next()? != State::Done {
            panic!("no rows expected for update");
        }
        id
    } else {
        // create new base record and capture id
        let mut stmt = db.prepare(
            "INSERT INTO placements
                (score, sort_score, optimum_score_prefix, faculty_preference_priority_policy,
                 comment, created_at, modified_at)
            VALUES (?, ?, ?, ?, ?, DATETIME('now', 'localtime'), DATETIME('now', 'localtime'))
            RETURNING placement_id",
        )?;
        stmt.bind((1, format!("{}", schedule.score).as_str()))?;
        stmt.bind((2, schedule.score.sortable().as_str()))?;
        stmt.bind((3, optimum_score_prefix_json.as_str()))?;
        stmt.bind((4, input.faculty_preference_priority_policy.database_name()))?;
        stmt.bind((5, comment))?;
        let mut id = -1;
        while stmt.next()? == State::Row {
            id = stmt.read(0)?;
        }
        println!("saved schedule with new placement id: {}", id);
        id
    };

    // insert all the placements
    for (section, placement) in schedule.placements.iter().enumerate() {
        let Some(time_slot) = placement.time_slot else {
            // skip unplaced sections
            continue;
        };
        let mut stmt = db.prepare(
            "INSERT INTO placement_sections (placement_id, section, time_slot, room)
            VALUES (?, ?, ?, ?)",
        )?;
        stmt.bind((1, root_id))?;
        stmt.bind((2, input.sections[section].name.as_str()))?;
        stmt.bind((3, input.time_slots[time_slot].name.as_str()))?;
        stmt.bind((4, placement.room.map(|room| input.rooms[room].name.as_str())))?;
        while stmt.next()? != State::Done {
            // no return rows expected
        }
    }

    // gather the penalties
    let mut penalties = Vec::new();
    for (section, placement) in schedule.placements.iter().enumerate() {
        if placement.time_slot.is_none() {
            penalties.push((
                LEVEL_FOR_UNPLACED_SECTION,
                format!("{} is not placed", input.sections[section].name),
                vec![section],
                input.sections[section].faculty.clone(),
            ));
        }
    }
    for penalty_list in &schedule.penalties {
        for penalty in penalty_list {
            let (priority, msg) = penalty.get_score_message(input, schedule);
            let sections = penalty.get_sections(input);
            let mut faculty = penalty.faculty().map_or_else(Vec::new, |owner| vec![owner]);
            if faculty.is_empty() {
                for &section in &sections {
                    faculty.extend_from_slice(&input.sections[section].faculty);
                }
                faculty.sort_unstable();
                faculty.dedup();
            }
            penalties.push((priority, msg, sections, faculty));
        }
    }

    // insert them
    for (priority, msg, sections, faculty) in penalties {
        let mut stmt = db.prepare(
            "INSERT INTO placement_penalties (placement_id, priority, message)
                VALUES (?, ?, ?) RETURNING placement_penalty_id",
        )?;
        stmt.bind((1, root_id))?;
        stmt.bind((2, priority as i64))?;
        stmt.bind((3, msg.as_str()))?;
        let mut id = -1;
        while stmt.next()? == State::Row {
            id = stmt.read(0)?;
        }

        for section in sections {
            let mut stmt = db.prepare(
                "INSERT INTO placement_penalty_sections (placement_penalty_id, section)
                    VALUES (?, ?)",
            )?;
            stmt.bind((1, id))?;
            stmt.bind((2, input.sections[section].name.as_str()))?;
            if stmt.next()? != State::Done {
                panic!("no results expected from insert");
            }
        }

        for f in faculty {
            let mut stmt = db.prepare(
                "INSERT INTO placement_penalty_faculty (placement_penalty_id, faculty)
                    VALUES (?, ?)",
            )?;
            stmt.bind((1, id))?;
            stmt.bind((2, input.faculty[f].name.as_str()))?;
            if stmt.next()? != State::Done {
                panic!("no results expected from insert");
            }
        }
    }
    db.execute("COMMIT")?;

    Ok(root_id)
}

pub fn load_schedule(
    db_path: &str,
    input: &Input,
    schedule: &mut Schedule,
    maybe_placement_id: Option<i64>,
) -> Result<()> {
    let db = Connection::open_with_flags(db_path, OpenFlags::new().with_read_only().with_no_mutex())?;
    db.execute("PRAGMA foreign_keys = ON")?;
    db.execute("PRAGMA temp_store = memory")?;
    db.execute("PRAGMA mmap_size = 100000000")?;

    let placement_id: i64;
    let saved_score: String;
    let optimum_score_prefix_json: String;

    // find the best-scoring schedule already in the DB
    // or a specific ID as requested
    let mut stmt = db.prepare(
        "SELECT placement_id, score, optimum_score_prefix
        FROM placements
        WHERE faculty_preference_priority_policy = ?
          AND (placement_id = ? OR ?)
        ORDER BY sort_score, modified_at DESC
        LIMIT 1",
    )?;
    let (q_id, q_override) = if let Some(id) = maybe_placement_id {
        (id, 0) // search on the given id
    } else {
        (0, 1) // search by sort score
    };
    stmt.bind((1, input.faculty_preference_priority_policy.database_name()))?;
    stmt.bind((2, q_id))?;
    stmt.bind((3, q_override))?;

    if let State::Row = stmt.next()? {
        placement_id = stmt.read(0)?;
        saved_score = stmt.read(1)?;
        optimum_score_prefix_json = stmt.read(2)?;
    } else {
        return Err(format!(
            "no placement found for faculty preference priority policy {}",
            input.faculty_preference_priority_policy.database_name()
        )
        .into());
    };
    if stmt.next()? != State::Done {
        panic!("multiple rows returned from single-row query");
    }

    let mut stmt = db.prepare(
        "SELECT modified_at
            FROM placements
            WHERE placement_id = ?",
    )?;
    stmt.bind((1, placement_id))?;
    let mut found = false;
    while stmt.next()? == State::Row {
        let modified_at: String = stmt.read(0)?;
        println!("loading schedule {}, which was last updated at {}", placement_id, modified_at);
        found = true;
    }
    if !found {
        return Err(format!("schedule {} not found", placement_id).into());
    }

    let mut stmt = db.prepare(
        "SELECT section, time_slot, room
            FROM placement_sections
            WHERE placement_id = ?",
    )?;
    stmt.bind((1, placement_id))?;

    while stmt.next()? == State::Row {
        let section_name: String = stmt.read(0)?;
        let time_slot_name: String = stmt.read(1)?;
        let maybe_room_name: Option<String> = stmt.read(2)?;

        let Some((section, _)) = input.sections.iter().enumerate().find(|(_, elt)| elt.name == section_name) else {
            return Err(format!("load_schedule cannot find section {} referenced in placement", section_name).into());
        };
        let Some((time_slot, _)) = input.time_slots.iter().enumerate().find(|(_, elt)| elt.name == time_slot_name)
        else {
            return Err(
                format!("load_schedule cannot find time slot {} referenced in placement", time_slot_name).into()
            );
        };
        let maybe_room = if let Some(room_name) = maybe_room_name {
            let Some((room, _)) = input.rooms.iter().enumerate().find(|(_, elt)| elt.name == room_name) else {
                return Err(format!("load_schedule cannot find room {} referenced in placement", room_name).into());
            };
            Some(room)
        } else {
            None
        };

        let _undo = move_section(input, schedule, section, time_slot, &maybe_room);
    }

    // does the generated score match the saved score?
    if format!("{}", schedule.score) != saved_score {
        return err(format!(
            "for placement with ID {} the saved score of {} does not match the computed score of {}",
            placement_id, saved_score, schedule.score
        ));
    }

    // parse the SAT-proved score prefix
    schedule.optimum_score_prefix = parse_score_array(&optimum_score_prefix_json)?;

    Ok(())
}

fn parse_score_array(json_str: &str) -> Result<Vec<ScoreLevel>> {
    // check if the string starts with '[' and ends with ']'
    if !json_str.starts_with('[') || !json_str.ends_with(']') {
        return err("saved optimum score prefix is not a JSON array");
    }

    // remove brackets and split on commas
    let mut result = Vec::new();
    for item in json_str[1..json_str.len() - 1].split(',') {
        match item.trim().parse::<i16>() {
            Ok(value) => result.push(value as ScoreLevel),
            Err(_) => return err(format!("Failed to parse '{}' as i16", item)),
        }
    }

    Ok(result)
}
