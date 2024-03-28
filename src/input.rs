use super::bits::*;
use super::solver::Solver;

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

    // instructors assigned to this section
    pub instructors: Vec<usize>,

    // rooms and times as input
    pub rooms: Vec<RoomWithPenalty>,
    pub time_slots: Vec<TimeWithPenalty>,

    // hard conflicts
    pub hard_conflicts: Vec<usize>,

    // soft conflicts
    pub soft_conflicts: Vec<SectionWithPenalty>,

    // the transitive closure of prereqs and coreqs
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
