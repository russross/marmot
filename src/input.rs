#![allow(clippy::collapsible_if)]

use super::error::Result;
use super::score::*;
use std::fmt;
use std::fmt::Write;
use std::ops;

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

    pub credit_hours: Option<CreditHours>,

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

#[derive(Clone, Copy, PartialEq)]
pub struct CreditHours {
    pub value: f64,
}

impl CreditHours {
    pub fn new(value: f64) -> Self {
        CreditHours { value }
    }
}

impl fmt::Display for CreditHours {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value == 1.0 {
            return write!(f, "1 credit");
        }
        write!(f, "{} credits", self.value)
    }
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
