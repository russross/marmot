use std::fmt;
use std::ops;
//use std::rc::Rc;

type ScoreLevel = i16;
const PRIORITY_LEVELS: usize = 20;
//const MIN_PREFERENCE_LEVEL: usize = 10;
//const LEVEL_FOR_UNPLACED_SECTION: usize = 0;
//const LEVEL_FOR_HARD_CONFLICT: usize = 1;
//const MIN_LOTTERY_TICKETS_FOR_UNPLACED_SECTION: isize = 1000;

pub struct InputData {
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

#[derive(Clone)]
pub struct Room {
    pub name: String,
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone)]
pub struct TimeSlot {
    pub name: String,
    pub days: Vec<time::Weekday>,
    pub start_time: time::Time,
    pub duration: time::Duration,
}

impl fmt::Display for TimeSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone)]
pub struct Faculty {
    pub name: String,
    pub sections: Vec<usize>,
    pub distribution: Vec<DistributionPreference>,
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

#[derive(Clone)]
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
            DistributionPreference::Clustering { max_gap, cluster_limits, gap_limits, .. } => {
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

            DistributionPreference::DaysOff { days_off, priority, .. } => {
                write!(f, "wants {} day{} off priority {}", days_off, if *days_off == 1 { "" } else { "s" }, priority)?;
            }

            DistributionPreference::DaysEvenlySpread { priority, .. } => {
                write!(f, "priority {}", priority)?;
            }
        }
        Ok(())
    }
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

pub struct Section {
    // e.g.,: "CS 1410-02"
    pub name: String,

    // rooms (if any) and times available for this section
    pub rooms: Vec<RoomWithPriority>,
    pub time_slots: Vec<TimeWithPriority>,

    // faculty (if any) assigned to this section
    pub faculty: Vec<usize>,

    // hard conflicts
    pub hard_conflicts: Vec<usize>,

    // soft conflicts
    pub soft_conflicts: Vec<SectionWithPriority>,

    // scoring that will be applied specifically to this section
    //pub score_criteria: Vec<Rc<dyn ScoreCriterion>>,

    // any section that might have a scoring interaction with this section
    //pub neighbors: Vec<usize>,
}

#[derive(Clone)]
pub struct RoomWithPriority {
    pub room: usize,
    pub priority: Option<u8>,
}

#[derive(Clone)]
pub struct TimeWithPriority {
    pub time_slot: usize,
    pub priority: Option<u8>,
}

#[derive(Clone)]
pub struct SectionWithPriority {
    pub section: usize,
    pub priority: u8,
}

#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub struct Score {
    pub levels: [ScoreLevel; PRIORITY_LEVELS],
}

impl Score {
    pub fn new() -> Self {
        Score {
            levels: [0; PRIORITY_LEVELS],
        }
    }

    pub fn is_zero(&self) -> bool {
        for i in 0..PRIORITY_LEVELS {
            if self.levels[i] != 0 {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            write!(f, "zero")
        } else {
            let mut sep = "";
            write!(f, "<")?;
            for (level, count) in self.levels.iter().enumerate() {
                if level != 0 {
                    write!(f, "{sep}{level}×{count}")?;
                    sep = ",";
                }
            }
            write!(f, ">")
        }
    }
}

impl ops::Add for Score {
    type Output = Self;

    fn add(self, rhs: Self) -> Score {
        let mut out = Score { levels: [0; PRIORITY_LEVELS] };
        for i in 0..PRIORITY_LEVELS {
            out.levels[i] = self.levels[i] + rhs.levels[i];
        }
        out
    }
}

impl ops::AddAssign for Score {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..PRIORITY_LEVELS {
            self.levels[i] += rhs.levels[i];
        }
    }
}

impl ops::Sub for Score {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut out = Score { levels: [0; PRIORITY_LEVELS] };
        for i in 0..PRIORITY_LEVELS {
            out.levels[i] = self.levels[i] - rhs.levels[i];
        }
        out
    }
}

impl ops::SubAssign for Score {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..PRIORITY_LEVELS {
            self.levels[i] -= rhs.levels[i];
        }
    }
}

