use super::input::*;
use super::solver::*;
use std::fmt;
use std::fmt::Write;
use std::ops;

//
//
// Scoring data
// The score vector, and scoring criteria, score deltas, etc.
//
//

pub type ScoreLevel = i16;
pub const PRIORITY_LEVELS: usize = 20;
pub const LEVEL_FOR_UNPLACED_SECTION: u8 = 0;
pub const LEVEL_FOR_HARD_CONFLICT: u8 = 1;
pub const LEVEL_FOR_ROOM_COUNT: u8 = 19;
pub const START_LEVEL_FOR_PREFERENCES: u8 = 10;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score {
    pub levels: [ScoreLevel; PRIORITY_LEVELS],
}

// a score criterion to be checked when a section or one of its
// neighbors is moved
#[derive(Clone)]
pub enum ScoreCriterion {
    SoftConflict {
        sections_with_priorities: Vec<SectionWithPriority>,
    },
    AntiConflict {
        priority: u8,
        single: usize,
        group: Vec<usize>,
    },
    RoomPreference {
        rooms_with_priorities: Vec<RoomWithPriority>,
    },
    TimeSlotPreference {
        time_slots_with_priorities: Vec<TimeSlotWithPriority>,
    },
    FacultySpread {
        faculty: usize,
        sections: Vec<usize>,
        grouped_by_days: Vec<Vec<DistributionPreference>>,
    },
    FacultyRoomCount {
        priority: u8,
        faculty: usize,
        desired: usize,
        sections: Vec<usize>,
    },
}

// a single change to the score due to a section's placement
pub enum ScoreDelta {
    SoftConflict {
        priority: u8,
        sections: [usize; 2],
    },
    AntiConflict {
        priority: u8,
        single: usize,
        group: Vec<usize>,
    },
    RoomPreference {
        priority: u8,
    },
    TimeSlotPreference {
        priority: u8,
    },
    Cluster {
        priority: u8,
        faculty: usize,
        is_too_short: bool,
        is_primary: bool,
    },
    Gap {
        priority: u8,
        faculty: usize,
        is_too_short: bool,
        is_primary: bool,
    },
    DaysOff {
        priority: u8,
        faculty: usize,
        desired: usize,
        actual: usize,
        is_primary: bool,
    },
    DaysEvenlySpread {
        priority: u8,
        faculty: usize,
        is_primary: bool,
    },
    RoomCount {
        priority: u8,
        faculty: usize,
        desired: usize,
        actual: usize,
        is_primary: bool,
    },
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
            ScoreCriterion::SoftConflict {
                sections_with_priorities,
            } => sections_with_priorities
                .iter()
                .map(|elt| elt.section)
                .collect(),

            ScoreCriterion::AntiConflict { single, group, .. } => {
                let mut lst = group.clone();
                lst.push(*single);
                lst
            }

            ScoreCriterion::RoomPreference { .. } => Vec::new(),

            ScoreCriterion::TimeSlotPreference { .. } => Vec::new(),

            ScoreCriterion::FacultySpread { sections, .. } => sections.clone(),

            ScoreCriterion::FacultyRoomCount { sections, .. } => sections.clone(),
        }
    }

    pub fn check(
        &self,
        schedule: &Schedule,
        input: &Input,
        section: usize,
        deltas: &mut Vec<ScoreDelta>,
    ) {
        // get our time slot
        let Some(my_time_slot) = schedule.placements[section].time_slot else {
            panic!("ScoreCriterion check called on unplaced section");
        };

        match self {
            ScoreCriterion::SoftConflict {
                sections_with_priorities,
            } => {
                for &SectionWithPriority {
                    section: other,
                    priority,
                } in sections_with_priorities
                {
                    // check for placement of the conflicting section
                    let Some(other_time_slot) = schedule.placements[other].time_slot else {
                        continue;
                    };

                    // we only care if there is an overlap
                    if !input.time_slot_conflicts[my_time_slot][other_time_slot] {
                        continue;
                    }

                    // if we make it this far, there is a soft conflict
                    let sections = if section < other {
                        [section, other]
                    } else {
                        [other, section]
                    };
                    deltas.push(ScoreDelta::SoftConflict { priority, sections });

                    // note: continue checking for other conflicts
                }
            }

            ScoreCriterion::AntiConflict {
                priority,
                single,
                group,
            } => {
                // grab the time slot of the single section
                let Some(single_time_slot) = schedule.placements[*single].time_slot else {
                    // single section is unplaced, move on
                    return;
                };

                // only consider placed sections from the group
                let mut placed = Vec::new();
                for &elt in group {
                    if schedule.is_placed(elt) {
                        placed.push(elt);
                    }
                }

                // no complaint if no members of the group are placed
                if placed.is_empty() {
                    return;
                }

                // if any member of the group matches, we are okay
                if placed
                    .iter()
                    .any(|&i| schedule.placements[i].time_slot == Some(single_time_slot))
                {
                    return;
                }
                deltas.push(ScoreDelta::AntiConflict {
                    priority: *priority,
                    single: *single,
                    group: group.clone(),
                });
            }

            ScoreCriterion::RoomPreference {
                rooms_with_priorities,
            } => {
                // get our room
                let Some(my_room) = schedule.placements[section].room else {
                    panic!("ScoreCriterion::RoomPreference check called on section with no room placement");
                };

                for &RoomWithPriority { room, priority } in rooms_with_priorities {
                    if room == my_room {
                        // record the priority and stop looking
                        deltas.push(ScoreDelta::RoomPreference { priority });
                        break;
                    }
                }
            }

            ScoreCriterion::TimeSlotPreference {
                time_slots_with_priorities,
            } => {
                // get our timeslot
                let Some(my_time_slot) = schedule.placements[section].time_slot else {
                    panic!("ScoreCriterion::TimeSlotPreference check called on section with no timeslot placement");
                };

                for &TimeSlotWithPriority {
                    time_slot,
                    priority,
                } in time_slots_with_priorities
                {
                    if time_slot == my_time_slot {
                        // record the priority and stop looking
                        deltas.push(ScoreDelta::TimeSlotPreference { priority });
                        break;
                    }
                }
            }

            ScoreCriterion::FacultySpread {
                faculty,
                sections,
                grouped_by_days,
            } => {
                // note: sections are sorted, so the first one is global marker
                // for each group of days, lay out the classes scheduled on those days in order
                for by_days in grouped_by_days {
                    // get the list of days
                    let days = match &by_days[0] {
                        DistributionPreference::Clustering { days, .. } => days,
                        DistributionPreference::DaysOff { days, .. } => days,
                        DistributionPreference::DaysEvenlySpread { days, .. } => days,
                    };

                    // for each day with matching index, a list of (start minute, duration minutes)
                    // of classes scheduled on that day
                    #[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
                    struct Interval {
                        start_time: time::Time,
                        duration: time::Duration,
                    }
                    let mut schedule_by_day: Vec<Vec<Interval>> = vec![Vec::new(); days.len()];

                    for &section in sections {
                        // find when the section was placed
                        let Some(time_slot) = schedule.placements[section].time_slot else {
                            continue;
                        };

                        // check each day we are interested in
                        for (i, &day) in days.iter().enumerate() {
                            let TimeSlot {
                                start_time,
                                duration,
                                days,
                                ..
                            } = &input.time_slots[time_slot];

                            // if this section is not scheduled on a day of interest, ignore it
                            if !days.contains(&day) {
                                continue;
                            }

                            schedule_by_day[i].push(Interval {
                                start_time: *start_time,
                                duration: *duration,
                            });
                        }
                    }
                    for day_schedule in &mut schedule_by_day {
                        day_schedule.sort();
                    }

                    // now process the individual scoring criteria
                    for pref in by_days {
                        match pref {
                            DistributionPreference::Clustering {
                                max_gap,
                                cluster_limits,
                                gap_limits,
                                ..
                            } => {
                                for day in &schedule_by_day {
                                    if day.is_empty() {
                                        continue;
                                    }

                                    // one too-short cluster per day is okay (to handle odd
                                    // sections)
                                    let mut too_short_okay = true;

                                    // identify one cluster at a time
                                    let mut cluster_start = 0;
                                    let mut cluster_end = 0;
                                    let mut end_time = day[0].start_time + day[0].duration;
                                    while cluster_start < day.len() {
                                        // keep adding sections while there are more and they start
                                        // soon enough after the end of the previous section
                                        while cluster_end + 1 < day.len()
                                            && end_time + *max_gap
                                                >= day[cluster_end + 1].start_time
                                        {
                                            cluster_end += 1;
                                            end_time = day[cluster_end].start_time
                                                + day[cluster_end].duration;
                                        }

                                        // how long is it?
                                        let total_duration =
                                            end_time - day[cluster_start].start_time;

                                        let mut worst_priority = u8::MAX;
                                        let mut is_too_short = false;

                                        // test cluster size against all the limits
                                        for limit in cluster_limits {
                                            match *limit {
                                                DurationWithPriority::TooShort {
                                                    duration,
                                                    priority,
                                                } => {
                                                    if total_duration < duration {
                                                        if too_short_okay {
                                                            // used up the one freebie
                                                            too_short_okay = false;
                                                            continue;
                                                        }

                                                        if priority < worst_priority {
                                                            worst_priority = priority;
                                                            is_too_short = true;
                                                        }
                                                    }
                                                }

                                                DurationWithPriority::TooLong {
                                                    duration,
                                                    priority,
                                                } => {
                                                    if total_duration > duration
                                                        && priority < worst_priority
                                                    {
                                                        worst_priority = priority;
                                                        is_too_short = false;
                                                    }
                                                }
                                            }
                                        }

                                        if worst_priority < u8::MAX {
                                            deltas.push(ScoreDelta::Cluster {
                                                priority: worst_priority,
                                                faculty: *faculty,
                                                is_too_short,
                                                is_primary: section == sections[0],
                                            });
                                        }

                                        // check the size of the gap between the end of this
                                        // cluster and the start of the next
                                        if cluster_end + 1 < day.len() {
                                            let gap = day[cluster_end + 1].start_time - end_time;

                                            // search the limits
                                            worst_priority = u8::MAX;
                                            is_too_short = false;

                                            for limit in gap_limits {
                                                match *limit {
                                                    DurationWithPriority::TooShort {
                                                        duration,
                                                        priority,
                                                    } => {
                                                        if gap < duration
                                                            && priority < worst_priority
                                                        {
                                                            worst_priority = priority;
                                                            is_too_short = true;
                                                        }
                                                    }

                                                    DurationWithPriority::TooLong {
                                                        duration,
                                                        priority,
                                                    } => {
                                                        if gap > duration
                                                            && priority < worst_priority
                                                        {
                                                            worst_priority = priority;
                                                            is_too_short = false;
                                                        }
                                                    }
                                                }
                                            }

                                            if worst_priority < u8::MAX {
                                                deltas.push(ScoreDelta::Gap {
                                                    priority: worst_priority,
                                                    faculty: *faculty,
                                                    is_too_short,
                                                    is_primary: section == sections[0],
                                                });
                                            }
                                        }

                                        cluster_start = cluster_end + 1;
                                        cluster_end = cluster_start;
                                    }
                                }
                            }

                            &DistributionPreference::DaysOff {
                                days_off: desired,
                                priority,
                                ..
                            } => {
                                let mut actual = 0;
                                for day in &schedule_by_day {
                                    if day.is_empty() {
                                        actual += 1;
                                    }
                                }
                                if actual != desired {
                                    deltas.push(ScoreDelta::DaysOff {
                                        priority,
                                        faculty: *faculty,
                                        desired,
                                        actual,
                                        is_primary: section == sections[0],
                                    });
                                }
                            }

                            &DistributionPreference::DaysEvenlySpread { priority, .. } => {
                                let mut most = 0;
                                let mut fewest = usize::MAX;
                                for day in &schedule_by_day {
                                    let count = day.len();

                                    // ignore days with no classes
                                    if count == 0 {
                                        continue;
                                    }

                                    most = std::cmp::max(most, count);
                                    fewest = std::cmp::min(fewest, count);
                                }

                                // comparing usize values, so avoid negatives
                                if most > fewest && most - fewest > 1 {
                                    deltas.push(ScoreDelta::DaysEvenlySpread {
                                        priority,
                                        faculty: *faculty,
                                        is_primary: section == sections[0],
                                    });
                                }
                            }
                        }
                    }
                }
            }

            ScoreCriterion::FacultyRoomCount {
                priority,
                faculty,
                desired,
                sections,
            } => {
                let mut rooms = Vec::new();
                for &sec in sections {
                    // find when the section was placed
                    let Some(room) = schedule.placements[sec].room else {
                        continue;
                    };
                    rooms.push(room);
                }
                rooms.sort();
                rooms.dedup();

                if rooms.len() > *desired {
                    deltas.push(ScoreDelta::RoomCount {
                        priority: *priority,
                        faculty: *faculty,
                        desired: *desired,
                        actual: rooms.len(),
                        is_primary: section == sections[0],
                    });
                }
            }
        }
    }

    pub fn debug(&self, input: &Input) -> String {
        let mut s = String::new();

        match self {
            ScoreCriterion::SoftConflict {
                sections_with_priorities,
            } => {
                write!(&mut s, "soft conflicts:").unwrap();
                for &SectionWithPriority { section, priority } in sections_with_priorities {
                    write!(&mut s, " {}:{}", input.sections[section].name, priority).unwrap();
                }
            }

            ScoreCriterion::AntiConflict {
                priority,
                single,
                group,
            } => {
                write!(&mut s, "anticonflict:{} {} vs", priority, single).unwrap();
                let mut sep = " ";
                for &elt in group {
                    write!(&mut s, "{}{}", sep, input.sections[elt].name).unwrap();
                    sep = ", ";
                }
            }

            ScoreCriterion::RoomPreference {
                rooms_with_priorities,
            } => {
                write!(&mut s, "room preferences:").unwrap();
                for &RoomWithPriority { room, priority } in rooms_with_priorities {
                    write!(&mut s, " {}:{}", input.rooms[room].name, priority).unwrap();
                }
            }

            ScoreCriterion::TimeSlotPreference {
                time_slots_with_priorities,
            } => {
                write!(&mut s, "timeslot preferences:").unwrap();
                for &TimeSlotWithPriority {
                    time_slot,
                    priority,
                } in time_slots_with_priorities
                {
                    write!(&mut s, " {}:{}", input.time_slots[time_slot].name, priority).unwrap();
                }
            }

            ScoreCriterion::FacultySpread {
                faculty,
                sections,
                grouped_by_days,
            } => {
                for group in grouped_by_days {
                    let days = match &group[0] {
                        DistributionPreference::Clustering { days, .. } => days,
                        DistributionPreference::DaysOff { days, .. } => days,
                        DistributionPreference::DaysEvenlySpread { days, .. } => days,
                    };
                    write!(
                        &mut s,
                        "class spread for {} on (",
                        input.faculty[*faculty].name
                    )
                    .unwrap();
                    let mut sep = "";
                    for day in days {
                        match day {
                            time::Weekday::Sunday => write!(&mut s, "{sep}Sun").unwrap(),
                            time::Weekday::Monday => write!(&mut s, "{sep}Mon").unwrap(),
                            time::Weekday::Tuesday => write!(&mut s, "{sep}Tues").unwrap(),
                            time::Weekday::Wednesday => write!(&mut s, "{sep}Wed").unwrap(),
                            time::Weekday::Thursday => write!(&mut s, "{sep}Thurs").unwrap(),
                            time::Weekday::Friday => write!(&mut s, "{sep}Fri").unwrap(),
                            time::Weekday::Saturday => write!(&mut s, "{sep}Sat").unwrap(),
                        }
                        sep = ", ";
                    }
                    write!(&mut s, "): [").unwrap();
                    let mut sep = "";
                    for &sec in sections {
                        write!(&mut s, "{sep}{}", input.sections[sec].name).unwrap();
                        sep = ", ";
                    }
                    writeln!(&mut s, "]").unwrap();
                    for pref in group {
                        match pref {
                            DistributionPreference::Clustering {
                                max_gap,
                                cluster_limits,
                                gap_limits,
                                ..
                            } => {
                                if !cluster_limits.is_empty() {
                                    write!(&mut s, "        cluster max:{}", max_gap).unwrap();
                                    for limit in cluster_limits {
                                        match limit {
                                            DurationWithPriority::TooShort {
                                                duration,
                                                priority,
                                            } => {
                                                write!(
                                                    &mut s,
                                                    " [<{} priority {}]",
                                                    duration, priority
                                                )
                                            }
                                            DurationWithPriority::TooLong {
                                                duration,
                                                priority,
                                            } => {
                                                write!(
                                                    &mut s,
                                                    " [>{} priority {}]",
                                                    duration, priority
                                                )
                                            }
                                        }
                                        .unwrap();
                                    }
                                    writeln!(&mut s).unwrap();
                                }

                                if !gap_limits.is_empty() {
                                    write!(&mut s, "        gap").unwrap();
                                    for limit in gap_limits {
                                        match limit {
                                            DurationWithPriority::TooShort {
                                                duration,
                                                priority,
                                            } => {
                                                write!(
                                                    &mut s,
                                                    " [<{} priority {}]",
                                                    duration, priority
                                                )
                                            }
                                            DurationWithPriority::TooLong {
                                                duration,
                                                priority,
                                            } => {
                                                write!(
                                                    &mut s,
                                                    " [>{} priority {}]",
                                                    duration, priority
                                                )
                                            }
                                        }
                                        .unwrap();
                                    }
                                    writeln!(&mut s).unwrap();
                                }
                            }

                            DistributionPreference::DaysOff {
                                days_off, priority, ..
                            } => {
                                writeln!(
                                    &mut s,
                                    "        days off:{} priority {}",
                                    days_off, priority
                                )
                                .unwrap();
                            }

                            DistributionPreference::DaysEvenlySpread { priority, .. } => {
                                writeln!(
                                    &mut s,
                                    "        days evenly spread priority {}",
                                    priority
                                )
                                .unwrap();
                            }
                        }
                    }
                }
            }

            ScoreCriterion::FacultyRoomCount {
                priority,
                faculty,
                desired,
                sections,
            } => {
                write!(
                    &mut s,
                    "faculty room count: {} desired {} priority {} sections [",
                    input.faculty[*faculty].name, desired, priority
                )
                .unwrap();
                let mut sep = "";
                for &section in sections {
                    write!(&mut s, "{}{}", sep, input.sections[section].name).unwrap();
                    sep = ", ";
                }
                write!(&mut s, "]").unwrap();
            }
        }
        s
    }
}

impl ScoreDelta {
    pub fn get_scores(&self, section: usize) -> (u8, Option<u8>) {
        match *self {
            ScoreDelta::SoftConflict {
                priority,
                sections: [first, _],
            } => (
                priority,
                if section == first {
                    Some(priority)
                } else {
                    None
                },
            ),

            ScoreDelta::AntiConflict {
                priority, single, ..
            } => (
                priority,
                if section == single {
                    Some(priority)
                } else {
                    None
                },
            ),

            ScoreDelta::RoomPreference { priority, .. } => (priority, Some(priority)),

            ScoreDelta::TimeSlotPreference { priority, .. } => (priority, Some(priority)),

            ScoreDelta::Cluster {
                priority,
                is_primary,
                ..
            } => (priority, if is_primary { Some(priority) } else { None }),

            ScoreDelta::Gap {
                priority,
                is_primary,
                ..
            } => (priority, if is_primary { Some(priority) } else { None }),

            ScoreDelta::DaysOff {
                priority,
                is_primary,
                ..
            } => (priority, if is_primary { Some(priority) } else { None }),

            ScoreDelta::DaysEvenlySpread {
                priority,
                is_primary,
                ..
            } => (priority, if is_primary { Some(priority) } else { None }),

            ScoreDelta::RoomCount {
                priority,
                is_primary,
                ..
            } => (priority, if is_primary { Some(priority) } else { None }),
        }
    }

    pub fn get_score_message(
        &self,
        input: &Input,
        schedule: &Schedule,
        section: usize,
    ) -> (u8, String) {
        match self {
            &ScoreDelta::SoftConflict {
                priority,
                sections: [a, b],
            } => {
                let ts_a = schedule.placements[a].time_slot.unwrap();
                let ts_b = schedule.placements[b].time_slot.unwrap();
                if ts_a == ts_b {
                    (
                        priority,
                        format!(
                            "course conflict: {} and {} both meet at {}",
                            input.sections[a].name,
                            input.sections[b].name,
                            input.time_slots[ts_a].name
                        ),
                    )
                } else {
                    (
                        priority,
                        format!(
                            "course conflict: {} at {} overlaps {} at {}",
                            input.sections[a].name,
                            input.time_slots[ts_a],
                            input.sections[b].name,
                            input.time_slots[ts_b].name
                        ),
                    )
                }
            }

            ScoreDelta::AntiConflict {
                priority,
                single,
                group,
            } => {
                let mut group_names = String::new();
                let mut sep = "";
                for elt in group {
                    group_names.push_str(sep);
                    sep = " or ";
                    group_names.push_str(&input.sections[*elt].name);
                }
                (
                    *priority,
                    format!(
                        "anticonflict: {} is not at the same time as {}",
                        input.sections[*single].name, group_names
                    ),
                )
            }

            &ScoreDelta::RoomPreference { priority } => {
                let room = schedule.placements[section].room.unwrap();
                (
                    priority,
                    format!(
                        "room preference: {} meets in {}",
                        input.sections[section].name, input.rooms[room].name
                    ),
                )
            }

            &ScoreDelta::TimeSlotPreference { priority } => {
                let time_slot = schedule.placements[section].time_slot.unwrap();
                (
                    priority,
                    format!(
                        "time slot preference: {} meets at {}",
                        input.sections[section].name, input.time_slots[time_slot].name
                    ),
                )
            }

            &ScoreDelta::Cluster {
                priority,
                faculty,
                is_too_short,
                ..
            } => (
                priority,
                format!(
                    "class cluster preference: {} has a cluster of classes that is too {}",
                    input.faculty[faculty].name,
                    if is_too_short { "short" } else { "long" }
                ),
            ),

            &ScoreDelta::Gap {
                priority,
                faculty,
                is_too_short,
                ..
            } => (
                priority,
                format!(
                    "gap preference: {} has a gap between clusters of classes is too {}",
                    input.faculty[faculty].name,
                    if is_too_short { "short" } else { "long" }
                ),
            ),

            &ScoreDelta::DaysOff {
                priority,
                faculty,
                desired,
                actual,
                ..
            } => (
                priority,
                format!(
                    "days off: {} wanted {} day{} off but got {}",
                    input.faculty[faculty].name,
                    desired,
                    if desired == 1 { "" } else { "s" },
                    actual
                ),
            ),

            &ScoreDelta::DaysEvenlySpread {
                priority, faculty, ..
            } => (
                priority,
                format!(
                    "uneven spread: {} has more classes some days than others",
                    input.faculty[faculty].name
                ),
            ),

            &ScoreDelta::RoomCount {
                priority,
                faculty,
                desired,
                actual,
                ..
            } => (
                priority,
                format!(
                    "room placement: {}'s classes are spread across {} room{} instead of {}",
                    input.faculty[faculty].name,
                    actual,
                    if actual == 1 { "" } else { "s" },
                    desired
                ),
            ),
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
        let (local, maybe_global) = delta.get_scores(section);
        schedule.placements[section].score.local += local;
        if let Some(global) = maybe_global {
            schedule.placements[section].score.global += global;
        }
    }
    schedule.placements[section].score.deltas = deltas;
}
