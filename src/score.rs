use std::fmt;
use std::fmt::Write;
use std::ops;
use super::input::*;
use super::solver::*;

//
//
// Scoring data
// The score vector, and scoring criteria, score deltas, etc.
//
//

pub type ScoreLevel = i16;
pub const PRIORITY_LEVELS: usize = 20;
pub const LEVEL_FOR_UNPLACED_SECTION: u8 = 0;
//const MIN_PREFERENCE_LEVEL: usize = 10;
//const LEVEL_FOR_HARD_CONFLICT: usize = 1;
//const MIN_LOTTERY_TICKETS_FOR_UNPLACED_SECTION: isize = 1000;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score {
    pub levels: [ScoreLevel; PRIORITY_LEVELS],
}

// a score criterion to be checked when a section or one of its
// neighbors is moved
pub enum ScoreCriterion {
    SoftConflict { sections_with_priorities: Vec<SectionWithPriority> },
    RoomPreference { rooms_with_priorities: Vec<RoomWithPriority> },
    TimeSlotPreference { time_slots_with_priorities: Vec<TimeSlotWithPriority> },
}

// a single change to the score due to a section's placement
pub enum ScoreDelta {
    SoftConflict { priority: u8, sections: [usize; 2] },
    RoomPreference { priority: u8 },
    TimeSlotPreference { priority: u8 },
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

    pub fn unplaced(&self) -> ScoreLevel {
        self.levels[LEVEL_FOR_UNPLACED_SECTION as usize]
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            write!(f, "zero")
        } else {
            let mut sep = "";
            write!(f, "<")?;
            for (level, &count) in self.levels.iter().enumerate() {
                if count != 0 {
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
        let mut out = Score {
            levels: [0; PRIORITY_LEVELS],
        };
        for i in 0..PRIORITY_LEVELS {
            out.levels[i] = self.levels[i] + rhs.levels[i];
        }
        out
    }
}

impl ops::Add<u8> for Score {
    type Output = Self;

    fn add(self, rhs: u8) -> Score {
        let mut out = self;
        out.levels[rhs as usize] += 1;
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

impl ops::AddAssign<u8> for Score {
    fn add_assign(&mut self, rhs: u8) {
        self.levels[rhs as usize] += 1;
    }
}

impl ops::Sub for Score {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut out = Score {
            levels: [0; PRIORITY_LEVELS],
        };
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

impl ScoreCriterion {
    pub fn get_neighbors(&self) -> Vec<usize> {
        match self {
            ScoreCriterion::SoftConflict { sections_with_priorities } =>
                sections_with_priorities.iter().map(|elt| elt.section).collect(),

            ScoreCriterion::RoomPreference { .. } => Vec::new(),

            ScoreCriterion::TimeSlotPreference { .. } => Vec::new(),
        }
    }

    pub fn check(&self, schedule: &Schedule, input: &Input, section: usize, deltas: &mut Vec<ScoreDelta>) {
        match self {
            ScoreCriterion::SoftConflict { sections_with_priorities } => {
                // get our time slot
                let Some(my_time_slot) = schedule.placements[section].time_slot else {
                    panic!("ScoreCriterion::SoftConflict check called on unplaced section");
                };

                for &SectionWithPriority { section: other, priority } in sections_with_priorities {
                    // check for placement of the conflicting section
                    let Some(other_time_slot) = schedule.placements[other].time_slot else {
                        continue;
                    };

                    // we only care if there is an overlap
                    if !input.time_slot_conflicts[my_time_slot][other_time_slot] {
                        continue;
                    }

                    // if we make it this far, there is a soft conflict
                    deltas.push(ScoreDelta::SoftConflict { priority, sections: [section, other] });

                    // note: continue checking for other conflicts
                }
            },

            ScoreCriterion::RoomPreference { rooms_with_priorities } => {
                // get our room
                let Some(my_room) = schedule.placements[section].room else {
                    panic!("ScoreCriterion::RoomPreference check called on section with no room placement");
                };

                for &RoomWithPriority { room, priority } in rooms_with_priorities {
                    if room == my_room {
                        // record the penalty and stop looking
                        deltas.push(ScoreDelta::RoomPreference { priority });
                        break;
                    }
                }
            },

            ScoreCriterion::TimeSlotPreference { time_slots_with_priorities } => {
                // get our timeslot
                let Some(my_time_slot) = schedule.placements[section].time_slot else {
                    panic!("ScoreCriterion::TimeSlotPreference check called on section with no timeslot placement");
                };

                for &TimeSlotWithPriority { time_slot, priority } in time_slots_with_priorities {
                    if time_slot == my_time_slot {
                        // record the penalty and stop looking
                        deltas.push(ScoreDelta::TimeSlotPreference { priority });
                        break;
                    }
                }
            },
        }
    }

    pub fn debug(&self, input: &Input) -> String {
        let mut s = String::new();

        match self {
            ScoreCriterion::SoftConflict { sections_with_priorities } => {
                write!(&mut s, "soft conflicts:").unwrap();
                for &SectionWithPriority { section, priority } in sections_with_priorities {
                    write!(&mut s, " {}:{}", input.sections[section].name, priority).unwrap();
                }
            },

            ScoreCriterion::RoomPreference { rooms_with_priorities } => {
                write!(&mut s, "room preferences:").unwrap();
                for &RoomWithPriority { room, priority } in rooms_with_priorities {
                    write!(&mut s, " {}:{}", input.rooms[room].name, priority).unwrap();
                }
            },

            ScoreCriterion::TimeSlotPreference { time_slots_with_priorities } => {
                write!(&mut s, "timeslot preferences:").unwrap();
                for &TimeSlotWithPriority { time_slot, priority } in time_slots_with_priorities {
                    write!(&mut s, " {}:{}", input.time_slots[time_slot].name, priority).unwrap();
                }
            },
        }
        s
    }
}

impl ScoreDelta {
    pub fn get_scores(&self) -> (u8, Option<u8>) {
        match *self {
            ScoreDelta::SoftConflict { priority, sections: [this, other] } => {
                if this < other { (priority, Some(priority)) } else { (priority, None) }
            },
            ScoreDelta::RoomPreference { priority, .. } => (priority, Some(priority)),
            ScoreDelta::TimeSlotPreference { priority, .. } => (priority, Some(priority)),
        }
    }
}

// compute all scores for a section in its curent placement
// the section's score is fully update, including local and global
// totals and the detail log,
// but the overall solver score is not modified
pub fn compute_section_score(schedule: &mut Schedule, input: &Input, section: usize) {
    assert!(schedule.placements[section].score.local.is_zero());
    assert!(schedule.placements[section].score.global.is_zero());
    assert!(schedule.placements[section].score.deltas.is_empty());

    if schedule.placements[section].time_slot.is_none() {
        // unplaced section
        schedule.placements[section].score.local = Score::new() + LEVEL_FOR_UNPLACED_SECTION;
        schedule.placements[section].score.global = Score::new() + LEVEL_FOR_UNPLACED_SECTION;
        return;
    };

    // loop over the scoring criteria
    let mut deltas = Vec::new();
    for criterion in &input.sections[section].score_criteria {
        criterion.check(schedule, input, section, &mut deltas);
    }

    // compute the totals and apply to the main section score record
    for delta in &deltas {
        let (local, maybe_global) = delta.get_scores();
        schedule.placements[section].score.local += local;
        if let Some(global) = maybe_global {
            schedule.placements[section].score.global += global;
        }
    }
    schedule.placements[section].score.deltas = deltas;
}
