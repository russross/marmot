use super::bits::*;
use super::solver::Solver;

pub fn setup() -> Result<Solver, String> {
    let mut solver = super::data::input()?;
    solver.missing.sort();
    solver.missing.dedup();

    // compute the transitive closure of all prereqs
    // we will do it the easy/dumb way and just iterate until it converges
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

    // add hard conflicts between all the sections an instructor teaches
    for instructor in &solver.instructors {
        for &left in &instructor.sections {
            // we only care about primary cross listings, so ignore others
            if !solver.input_sections[left].cross_listings.is_empty()
                && solver.input_sections[left].cross_listings[0] != left
            {
                continue;
            }
            for &right in &instructor.sections {
                // we only care about primary cross listings, so ignore others
                if !solver.input_sections[right].cross_listings.is_empty()
                    && solver.input_sections[right].cross_listings[0] != right
                {
                    continue;
                }
                if left == right {
                    continue;
                };
                solver.input_sections[left].set_conflict(right, 100);
            }
        }
    }

    // handle courses with multiple sections:
    // 1. remove/relax all conflicts involving those sections (except hard conflicts)
    // 2. make sections of the same course hard conflicts with each other
    // 3. scoring criteria to spread sections across morning/afternoon, mw/tr?
    let mut counts = std::collections::HashMap::<String, usize>::new();
    for sec in &solver.input_sections {
        let key = format!("{} {}", sec.prefix, sec.course);
        *counts.entry(key).or_default() += 1;
    }
    counts.retain(|_, v| *v > 1);
    //for course_raw in counts.keys() {
    //println!("{} has multiple sections", course_raw);
    //}

    // compute time slot conflict lookup table
    // used by time_slots_conflict
    for i in 0..solver.time_slots.len() {
        for j in 0..solver.time_slots.len() {
            solver
                .time_slot_conflicts
                .push(solver.time_slots[i].conflicts.contains(&j));
        }
    }

    Ok(solver)
}

// set the conflict penalty between every pair of sections in the list
//
// a subtlety: if a course is specified without a section and resolves
// to multiple sections, the penalty between those sections will be unchanged.
// e.g.: specifying CS 101 and CS 102 will set the conflict between every
// section of CS 101 vs every CS 102, but not between the individual
// sections of CS 101 nor between the individual sections of CS 102
pub fn make_conflict_clique(
    solver: &mut Solver,
    badness: isize,
    maximize: bool,
    sections_raw: Vec<&str>,
) -> Result<(), String> {
    assert!(!solver.input_locked);

    // parse and sanity check the inputs
    if !(0..=100).contains(&badness) {
        return Err("badness for a conflict clique must be between 0 and 100".into());
    }
    if badness == 100 && !maximize {
        return Err("make_conflict_clique does not support badness=100 and !maximize".into());
    } else if badness == 0 && maximize {
        return Err("make_conflict_clique does not support badness=0 and maximize".into());
    }
    let mut courses = Vec::new();
    for section_raw in sections_raw {
        let list = find_sections_by_name(solver, section_raw)?;
        if list.is_empty() {
            solver.missing.push(section_raw.to_string());
        } else {
            courses.push(list);
        }
    }

    // process every pairing (except element vs self)
    for (left_course_i, left_course) in courses.iter().enumerate() {
        for (right_course_i, right_course) in courses.iter().enumerate() {
            if left_course_i == right_course_i {
                continue;
            }

            for &left_section_i in left_course {
                for &right_section_i in right_course {
                    let left = &mut solver.input_sections[left_section_i];
                    let old = left.get_conflict(right_section_i);
                    left.set_conflict(
                        right_section_i,
                        if maximize {
                            std::cmp::max(old, badness)
                        } else {
                            std::cmp::min(old, badness)
                        },
                    );
                }
            }
        }
    }
    Ok(())
}

pub fn add_prereqs_fn(
    solver: &mut Solver,
    course_raw: &str,
    coreqs_raw: Vec<&str>,
    prereqs_raw: Vec<&str>,
) -> Result<(), String> {
    assert!(!solver.input_locked);

    // see if the course is present in the data
    let course_list = find_sections_by_name(solver, course_raw)?;
    if course_list.is_empty() {
        solver.missing.push(course_raw.to_string());
    }

    // look up the coreq sections
    let mut coreqs = Vec::new();
    for raw in coreqs_raw {
        let mut list = find_sections_by_name(solver, raw)?;
        if list.is_empty() {
            solver.missing.push(raw.to_string());
        } else {
            coreqs.append(&mut list);
        }
    }

    // look up the prereq sections
    let mut prereqs = Vec::new();
    for raw in prereqs_raw {
        let mut list = find_sections_by_name(solver, raw)?;
        if list.is_empty() {
            solver.missing.push(raw.to_string());
        } else {
            prereqs.append(&mut list);
        }
    }

    coreqs.sort();
    coreqs.dedup();
    for course in course_list {
        let cr = &mut solver.input_sections[course].coreqs;
        for &elt in &coreqs {
            cr.push(elt);
        }
        cr.sort();
        cr.dedup();

        let pr = &mut solver.input_sections[course].prereqs;
        for &elt in &prereqs {
            pr.push(elt);
        }
        pr.sort();
        pr.dedup();
    }

    Ok(())
}

pub fn multiple_sections_reduce_penalties_fn(
    solver: &mut Solver,
    courses_raw: Vec<(&str, isize)>,
) -> Result<(), String> {
    assert!(!solver.input_locked);

    let threshold = 30;
    for (course_raw, online) in courses_raw {
        // get the sections of this course
        let course_list = find_sections_by_name(solver, course_raw)?;
        if course_list.is_empty() {
            solver.missing.push(course_raw.to_string());
            continue;
        }
        let number = (course_list.len() as isize) + online;
        if number == 1 {
            // nothing to do
            continue;
        }

        // find all the soft conflicts involving these sections
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
                    (solver.input_sections[sec_i].get_conflict(other) - 1) / (number + 1);
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

pub fn find_time_slot_by_name(solver: &Solver, name: &str) -> Result<usize, String> {
    let Some(i) = solver.time_slots.iter().position(|elt| elt.name == *name) else {
        return Err(format!("timeslot named \"{}\" not found", name));
    };
    Ok(i)
}

pub fn find_room_by_name(solver: &Solver, name: &str) -> Result<usize, String> {
    let Some(i) = solver.rooms.iter().position(|elt| elt.name == *name) else {
        return Err(format!("room named \"{}\" not found", name));
    };
    Ok(i)
}

pub fn find_section_by_name(solver: &Solver, section_raw: &String) -> Result<usize, String> {
    let (prefix, course, Some(section)) = parse_section_name(&section_raw)? else {
        return Err(format!("section name {section_raw} must include prefix, course, and section, like 'CS 1400-01'"));
    };
    solver
        .input_sections
        .iter()
        .position(|elt| elt.prefix == prefix && elt.course == course && elt.section == section)
        .ok_or("could not find section".into())
}

pub fn find_sections_by_name(solver: &Solver, course_raw: &str) -> Result<Vec<usize>, String> {
    let (prefix, course, section) = parse_section_name(course_raw)?;
    let mut list = Vec::new();
    solver.input_sections.iter().enumerate().for_each(|(i, s)| {
        if s.prefix == *prefix && s.course == *course {
            match &section {
                None => list.push(i),
                Some(name) if *name == s.section => list.push(i),
                _ => (),
            }
        }
    });
    Ok(list)
}

pub fn find_instructor_by_name(solver: &Solver, name: &str) -> Result<usize, String> {
    let Some(i) = solver.instructors.iter().position(|elt| elt.name == *name) else {
        return Err(format!("instructor named \"{}\" not found", name));
    };
    Ok(i)
}

pub fn time_slots_conflict(solver: &Solver, a: usize, b: usize) -> bool {
    solver.time_slot_conflicts[a * solver.time_slots.len() + b]
    //solver.time_slots[a].conflicts.contains(&b)
}

pub fn is_primary_cross_listing(solver: &Solver, index: usize) -> bool {
    solver.input_sections[index].cross_listings.is_empty()
        || index == solver.input_sections[index].cross_listings[0]
}

pub fn get_primary_cross_listing(solver: &Solver, index: usize) -> usize {
    if solver.input_sections[index].cross_listings.is_empty() {
        return index;
    }
    solver.input_sections[index].cross_listings[0]
}

pub fn parse_days(weekday_raw: &str) -> Result<Vec<time::Weekday>, String> {
    let mut days = Vec::new();
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

pub fn parse_section_name(name: &str) -> Result<(String, String, Option<String>), String> {
    // example CS 1400-01
    let re = regex::Regex::new(r"^([^ ]+) ([^- ]+)(?:-([^ ]+))?$").unwrap();
    let Some(caps) = re.captures(name) else {
        return Err(format!(
            "unrecognized section name format: '{}' should be like 'CS 1400-01' or 'CS 1400'",
            name
        ));
    };
    let prefix_part = caps.get(1).unwrap().as_str().to_string().to_uppercase();
    let course_part = caps.get(2).unwrap().as_str().to_string().to_uppercase();
    let section_part = caps.get(3).map(|s| s.as_str().to_string().to_uppercase());
    Ok((prefix_part, course_part, section_part))
}

pub fn date_range_slots(start: time::Date, end: time::Date) -> usize {
    let size = ((end - start).whole_days() + 1) * 24 * 60 / 5;
    if size <= 0 {
        panic!("date_range_slots must have start < end");
    }
    size as usize
}

pub fn date(year: i32, month: u8, day: u8) -> Result<time::Date, String> {
    let Ok(m) = time::Month::try_from(month) else {
        return Err(format!("date input with invalid month {}", month));
    };
    let Ok(d) = time::Date::from_calendar_date(year, m, day) else {
        return Err(format!(
            "date {}-{}-{} is invalid, should be year-month-day",
            year, month, day
        ));
    };
    Ok(d)
}

pub fn parse_date(s: String) -> Result<time::Date, String> {
    let b = s.as_bytes();
    if b.len() != 10 || b[4] != b'-' || b[7] != b'-' {
        return Err(format!("parse_date: date string [{s}] is wrong format"));
    }
    let year = i32::from_str_radix(&s[0..4], 10);
    let month = u8::from_str_radix(&s[5..7], 10);
    let day = u8::from_str_radix(&s[8..10], 10);
    let (Ok(y), Ok(m), Ok(d)) = (year, month, day) else {
        return Err(format!("parse_date: unable to parse parts of [{s}]"));
    };
    date(y, m, d)
}

#[derive(Clone)]
pub struct Room {
    pub name: String,
    pub capacity: u16,
    pub tags: Vec<String>,
}

#[derive(Clone)]
pub struct TimeSlot {
    pub name: String,
    pub slots: Bits,
    pub days: Vec<time::Weekday>,
    pub start_time: time::Time,
    pub duration: time::Duration,

    // a list of all overlapping time slots
    // a time slot IS always in its own conflict list
    pub conflicts: Vec<usize>,
    pub tags: Vec<String>,
}

#[derive(Clone)]
pub struct RoomTime {
    pub room: usize,
    pub time_slot: usize,
}

#[derive(Clone)]
pub struct TimeWithPenalty {
    pub time_slot: usize,
    pub penalty: isize,
}

#[derive(Clone)]
pub struct RoomWithPenalty {
    pub room: usize,
    pub penalty: isize,
}

#[derive(Clone)]
pub struct RoomTimeWithPenalty {
    pub room: usize,
    pub time_slot: usize,
    pub penalty: isize,
}

#[derive(Clone)]
pub struct SectionWithPenalty {
    pub section: usize,
    pub penalty: isize,
}

#[derive(Clone)]
pub struct Instructor {
    pub name: String,

    // one entry per day of the week (time::Weekday::number_days_from_sunday())
    // each day has a list of penalty values for each 5-minute slot,
    // with -1 meaning impossible
    pub available_times: Vec<Vec<isize>>,
    pub sections: Vec<usize>,
    pub distribution: Vec<DistributionPreference>,
}

impl Instructor {
    pub fn get_time_slot_penalty(&self, time_slot: &TimeSlot) -> Option<isize> {
        let start_hour = time_slot.start_time.hour() as usize;
        let start_minute = time_slot.start_time.minute() as usize;
        let minutes = time_slot.duration.whole_minutes() as usize;

        let mut penalty = 0;
        for &day_of_week in &time_slot.days {
            let day = &self.available_times[day_of_week.number_days_from_sunday() as usize];
            let start_index = start_hour * 12 + start_minute / 5;
            let end_index = start_index + minutes / 5;
            for &elt in day.iter().take(end_index).skip(start_index) {
                if elt < 0 {
                    return None;
                }
                penalty = std::cmp::max(penalty, elt);
            }
        }
        Some(penalty)
    }
}

#[derive(Clone)]
pub enum DistributionPreference {
    // classes on the same day should occur in clusters with tidy gaps between them
    Clustering {
        days: Vec<time::Weekday>,
        max_gap: time::Duration,
        cluster_limits: Vec<DurationWithPenalty>,
        gap_limits: Vec<DurationWithPenalty>,
    },

    // zero or more days from the list should be free of classes
    DaysOff {
        days: Vec<time::Weekday>,
        days_off: u8,
        penalty: isize,
    },

    // days that have classes should have the same number of classes
    DaysEvenlySpread {
        days: Vec<time::Weekday>,
        penalty: isize,
    },
}

#[derive(Clone, PartialEq)]
pub enum DurationWithPenalty {
    // a duration shorter than this gets a penalty
    TooShort {
        duration: time::Duration,
        penalty: isize,
    },

    // a duration longer than this gets a penalty
    TooLong {
        duration: time::Duration,
        penalty: isize,
    },
}

#[derive(Clone)]
pub struct InputSection {
    // prefix, e.g.: "CS"
    pub prefix: String,
    // course name, e.g.: "2810"
    pub course: String,
    // section name, e.g.: "01"
    pub section: String,

    // includes only the instructors explicity assigned to this section (not cross-listings)
    pub instructors: Vec<usize>,

    // rooms and times as input
    pub rooms: Vec<RoomWithPenalty>,
    pub time_slots: Vec<TimeWithPenalty>,

    // hard conflicts that named this section directly, not including cross-listings
    pub hard_conflicts: Vec<usize>,

    // soft conflicts that named this section directly, not including cross-listings
    pub soft_conflicts: Vec<SectionWithPenalty>,

    // if the course is cross listed it this list will have all cross-listed sections
    // in numerical order, and the first one is the canonical section (others tag along)
    // empty implies no cross listing
    pub cross_listings: Vec<usize>,

    // direct prereqs are recorded here
    // the transitive closure of prereqs is used to remove conflicts between classes
    // that cannot be taken together
    // note: the prereqs of a coreq are treated like direct prereqs
    // and the coreqs of a prereq are treated like direct prereqs
    //
    // if a course is a coreq (and optionally also a prereq) then we do nothing
    // directly but it will affect courses that require this one
    pub coreqs: Vec<usize>,
    pub prereqs: Vec<usize>,
}

impl InputSection {
    pub fn get_conflict(&self, other: usize) -> isize {
        for elt in &self.hard_conflicts {
            if *elt == other {
                return 100;
            }
        }
        for elt in &self.soft_conflicts {
            if elt.section == other {
                return elt.penalty;
            }
        }
        0
    }

    pub fn set_conflict(&mut self, other: usize, penalty: isize) {
        assert!((0..=100).contains(&penalty));
        if penalty == 0 {
            self.hard_conflicts.retain(|&elt| elt != other);
            self.soft_conflicts.retain(|elt| elt.section != other);
        } else if penalty == 100 {
            if !self.hard_conflicts.iter().any(|&elt| elt == other) {
                self.hard_conflicts.push(other);
            }
            self.soft_conflicts.retain(|elt| elt.section != other);
        } else {
            self.hard_conflicts.retain(|&elt| elt != other);
            match self
                .soft_conflicts
                .iter()
                .position(|elt| elt.section == other)
            {
                Some(i) => self.soft_conflicts[i].penalty = penalty,
                None => self.soft_conflicts.push(SectionWithPenalty {
                    section: other,
                    penalty,
                }),
            }
        }
    }

    pub fn get_name(&self) -> String {
        format!("{} {}-{}", self.prefix, self.course, self.section)
    }
}

macro_rules! conflict {
    ($input:expr,
            set hard,
            clique: $($sections:literal),+ $(,)?) => {
        make_conflict_clique($input,
            100, true,
            vec![ $($sections, )+ ])?;
    };
    ($input:expr,
            set penalty to $penalty:expr,
            clique: $($sections:literal),+ $(,)?) => {
        make_conflict_clique($input,
            $penalty, true,
            vec![ $($sections, )+ ])?;
    };
    ($input:expr,
            remove penalty,
            clique: $($sections:literal),+ $(,)?) => {
        make_conflict_clique($input,
            0, false,
            vec![ $($sections, )+ ])?;
    };
}

macro_rules! add_prereqs {
    ($input:expr,
            course: $course:literal,
            coreqs: $($coreqs:literal),+,
            prereqs: $($prereqs:literal),+ $(,)?) => {
        add_prereqs_fn($input, $course, vec![ $($coreqs, )+ ], vec![ $($prereqs, )+ ])?;
    };
    ($input:expr,
            course: $course:literal,
            coreqs: $($coreqs:literal),+ $(,)?) => {
        add_prereqs_fn($input, $course, vec![ $($coreqs, )+ ], vec![])?;
    };
    ($input:expr,
            course: $course:literal,
            prereqs: $($prereqs:literal),+ $(,)?) => {
        add_prereqs_fn($input, $course, vec![], vec![ $($prereqs, )+ ])?;
    };
}

macro_rules! course_with_online {
    ($course:literal with $online:literal online) => {
        ($course, $online)
    };
    ($course:literal) => {
        ($course, 0)
    };
}

macro_rules! multiple_sections_reduce_penalties {
    ($input:expr,
            courses: $($course:literal $(with $online:literal online)?),+ $(,)?) => {
        multiple_sections_reduce_penalties_fn($input, vec![ $(course_with_online!($course $(with $online online)?),)+ ])?;
    };
}

pub(crate) use {
    add_prereqs, conflict, course_with_online, multiple_sections_reduce_penalties,
};
