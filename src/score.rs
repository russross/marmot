use super::input::*;
use super::solver::*;
use std::fmt;
use std::fmt::Write;
use std::ops;

//
//
// Scoring data
// The score vector, and scoring criteria, score penalties, etc.
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
pub enum Criterion {
    SoftConflict { priority: u8, sections: [usize; 2] },
    AntiConflict { priority: u8, single: usize, group: Vec<usize> },
    RoomPreference { section: usize, rooms_with_priorities: Vec<RoomWithPriority> },
    TimeSlotPreference { section: usize, time_slots_with_priorities: Vec<TimeSlotWithPriority> },
    FacultySpread { faculty: usize, sections: Vec<usize>, grouped_by_days: Vec<Vec<DistributionPreference>> },
    FacultyRoomCount { priority: u8, faculty: usize, sections: Vec<usize>, desired: usize },
}

// a single change to the score due to a section's placement
#[derive(Clone)]
pub enum Penalty {
    SoftConflict { priority: u8, sections: [usize; 2] },
    AntiConflict { priority: u8, single: usize, group: Vec<usize> },
    RoomPreference { priority: u8, section: usize },
    TimeSlotPreference { priority: u8, section: usize },
    Cluster { priority: u8, faculty: usize, is_too_short: bool },
    Gap { priority: u8, faculty: usize, is_too_short: bool },
    DaysOff { priority: u8, faculty: usize, desired: usize, actual: usize },
    DaysEvenlySpread { priority: u8, faculty: usize },
    RoomCount { priority: u8, faculty: usize, desired: usize, actual: usize },
}

impl Score {
    pub fn new() -> Self {
        Score { levels: [0; PRIORITY_LEVELS] }
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

    pub fn is_placed(&self) -> bool {
        self.levels[LEVEL_FOR_UNPLACED_SECTION as usize] == 0
    }

    pub fn first_nonzero(&self) -> u8 {
        for (i, &val) in self.levels.iter().enumerate() {
            if val != 0 {
                return i as u8;
            }
        }
        self.levels.len() as u8
    }
}

impl Default for Score {
    fn default() -> Self {
        Self::new()
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
        let mut out = Score { levels: [0; PRIORITY_LEVELS] };
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

impl ops::SubAssign<u8> for Score {
    fn sub_assign(&mut self, rhs: u8) {
        self.levels[rhs as usize] -= 1;
    }
}

impl Criterion {
    pub fn get_culpable_sections(&self) -> Vec<usize> {
        match self {
            Criterion::SoftConflict { sections, .. } => sections.to_vec(),

            Criterion::AntiConflict { single, group, .. } => {
                let mut lst = group.clone();
                lst.push(*single);
                lst
            }

            Criterion::RoomPreference { section, .. } => vec![*section],

            Criterion::TimeSlotPreference { section, .. } => vec![*section],

            Criterion::FacultySpread { sections, .. } => sections.clone(),

            Criterion::FacultyRoomCount { sections, .. } => sections.clone(),
        }
    }

    pub fn check(&self, input: &Input, schedule: &Schedule) -> Vec<Penalty> {
        match self {
            &Criterion::SoftConflict { priority, sections: [section, other] } => {
                let Some(my_time_slot) = schedule.placements[section].time_slot else {
                    return Vec::new();
                };

                // check for placement of the conflicting section
                let Some(other_time_slot) = schedule.placements[other].time_slot else {
                    return Vec::new();
                };

                // we only care if there is an overlap
                if !input.time_slot_conflicts[my_time_slot][other_time_slot] {
                    return Vec::new();
                }

                // if we make it this far, there is a soft conflict
                vec![Penalty::SoftConflict { priority, sections: [section, other] }]
            }

            Criterion::AntiConflict { priority, single, group } => {
                // grab the time slot of the single section
                let Some(single_time_slot) = schedule.placements[*single].time_slot else {
                    // single section is unplaced, move on
                    return Vec::new();
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
                    return Vec::new();
                }

                // if any member of the group matches, we are okay
                if placed.iter().any(|&i| schedule.placements[i].time_slot == Some(single_time_slot)) {
                    return Vec::new();
                }
                vec![Penalty::AntiConflict { priority: *priority, single: *single, group: group.clone() }]
            }

            Criterion::RoomPreference { section, rooms_with_priorities } => {
                // get our room
                if let Some(my_room) = schedule.placements[*section].room {
                    for &RoomWithPriority { room, priority } in rooms_with_priorities {
                        if room == my_room {
                            // record the priority and stop looking
                            return vec![Penalty::RoomPreference { priority, section: *section }];
                        }
                    }
                }
                Vec::new()
            }

            Criterion::TimeSlotPreference { section, time_slots_with_priorities } => {
                // get our timeslot
                if let Some(my_time_slot) = schedule.placements[*section].time_slot {
                    for &TimeSlotWithPriority { time_slot, priority } in time_slots_with_priorities {
                        if time_slot == my_time_slot {
                            // record the priority and stop looking
                            return vec![Penalty::TimeSlotPreference { priority, section: *section }];
                        }
                    }
                }
                Vec::new()
            }

            Criterion::FacultySpread { faculty, sections, grouped_by_days } => {
                let mut penalties = Vec::new();

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
                            let TimeSlot { start_time, duration, days, .. } = &input.time_slots[time_slot];

                            // if this section is not scheduled on a day of interest, ignore it
                            if !days.contains(&day) {
                                continue;
                            }

                            schedule_by_day[i].push(Interval { start_time: *start_time, duration: *duration });
                        }
                    }
                    for day_schedule in &mut schedule_by_day {
                        day_schedule.sort_unstable();
                    }

                    // now process the individual scoring criteria
                    for pref in by_days {
                        match pref {
                            DistributionPreference::Clustering { max_gap, cluster_limits, gap_limits, .. } => {
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
                                            && end_time + *max_gap >= day[cluster_end + 1].start_time
                                        {
                                            cluster_end += 1;
                                            end_time = day[cluster_end].start_time + day[cluster_end].duration;
                                        }

                                        // how long is it?
                                        let total_duration = end_time - day[cluster_start].start_time;

                                        let mut worst_priority = u8::MAX;
                                        let mut is_too_short = false;

                                        // test cluster size against all the limits
                                        for limit in cluster_limits {
                                            match *limit {
                                                DurationWithPriority::TooShort { duration, priority } => {
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

                                                DurationWithPriority::TooLong { duration, priority } => {
                                                    if total_duration > duration && priority < worst_priority {
                                                        worst_priority = priority;
                                                        is_too_short = false;
                                                    }
                                                }
                                            }
                                        }

                                        if worst_priority < u8::MAX {
                                            penalties.push(Penalty::Cluster {
                                                priority: worst_priority,
                                                faculty: *faculty,
                                                is_too_short,
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
                                                    DurationWithPriority::TooShort { duration, priority } => {
                                                        if gap < duration && priority < worst_priority {
                                                            worst_priority = priority;
                                                            is_too_short = true;
                                                        }
                                                    }

                                                    DurationWithPriority::TooLong { duration, priority } => {
                                                        if gap > duration && priority < worst_priority {
                                                            worst_priority = priority;
                                                            is_too_short = false;
                                                        }
                                                    }
                                                }
                                            }

                                            if worst_priority < u8::MAX {
                                                penalties.push(Penalty::Gap {
                                                    priority: worst_priority,
                                                    faculty: *faculty,
                                                    is_too_short,
                                                });
                                            }
                                        }

                                        cluster_start = cluster_end + 1;
                                        cluster_end = cluster_start;
                                    }
                                }
                            }

                            &DistributionPreference::DaysOff { days_off: desired, priority, .. } => {
                                let mut actual = 0;
                                for day in &schedule_by_day {
                                    if day.is_empty() {
                                        actual += 1;
                                    }
                                }
                                if actual != desired {
                                    penalties.push(Penalty::DaysOff {
                                        priority,
                                        faculty: *faculty,
                                        desired,
                                        actual,
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
                                    penalties.push(Penalty::DaysEvenlySpread {
                                        priority,
                                        faculty: *faculty,
                                    });
                                }
                            }
                        }
                    }
                }

                penalties
            }

            Criterion::FacultyRoomCount { priority, faculty, desired, sections } => {
                let mut rooms = Vec::new();
                for &sec in sections {
                    // find when the section was placed
                    let Some(room) = schedule.placements[sec].room else {
                        continue;
                    };
                    rooms.push(room);
                }
                rooms.sort_unstable();
                rooms.dedup();

                if rooms.len() > *desired {
                    return vec![Penalty::RoomCount {
                        priority: *priority,
                        faculty: *faculty,
                        desired: *desired,
                        actual: rooms.len(),
                    }];
                }

                Vec::new()
            }
        }
    }

    pub fn debug(&self, input: &Input) -> String {
        let mut s = String::new();

        match self {
            &Criterion::SoftConflict { priority, sections: [_, section] } => {
                write!(&mut s, "soft conflict:").unwrap();
                write!(&mut s, " {}:{}", input.sections[section].name, priority).unwrap();
            }

            Criterion::AntiConflict { priority, single, group } => {
                write!(&mut s, "anticonflict:{} {} vs", priority, single).unwrap();
                let mut sep = " ";
                for &elt in group {
                    write!(&mut s, "{}{}", sep, input.sections[elt].name).unwrap();
                    sep = ", ";
                }
            }

            Criterion::RoomPreference { rooms_with_priorities, .. } => {
                write!(&mut s, "room preferences:").unwrap();
                for &RoomWithPriority { room, priority } in rooms_with_priorities {
                    write!(&mut s, " {}:{}", input.rooms[room].name, priority).unwrap();
                }
            }

            Criterion::TimeSlotPreference { time_slots_with_priorities, .. } => {
                write!(&mut s, "timeslot preferences:").unwrap();
                for &TimeSlotWithPriority { time_slot, priority } in time_slots_with_priorities {
                    write!(&mut s, " {}:{}", input.time_slots[time_slot].name, priority).unwrap();
                }
            }

            Criterion::FacultySpread { faculty, sections, grouped_by_days } => {
                for group in grouped_by_days {
                    let days = match &group[0] {
                        DistributionPreference::Clustering { days, .. } => days,
                        DistributionPreference::DaysOff { days, .. } => days,
                        DistributionPreference::DaysEvenlySpread { days, .. } => days,
                    };
                    write!(&mut s, "class spread for {} on (", input.faculty[*faculty].name).unwrap();
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
                            DistributionPreference::Clustering { max_gap, cluster_limits, gap_limits, .. } => {
                                if !cluster_limits.is_empty() {
                                    write!(&mut s, "        cluster max:{}", max_gap).unwrap();
                                    for limit in cluster_limits {
                                        match limit {
                                            DurationWithPriority::TooShort { duration, priority } => {
                                                write!(&mut s, " [<{} priority {}]", duration, priority)
                                            }
                                            DurationWithPriority::TooLong { duration, priority } => {
                                                write!(&mut s, " [>{} priority {}]", duration, priority)
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
                                            DurationWithPriority::TooShort { duration, priority } => {
                                                write!(&mut s, " [<{} priority {}]", duration, priority)
                                            }
                                            DurationWithPriority::TooLong { duration, priority } => {
                                                write!(&mut s, " [>{} priority {}]", duration, priority)
                                            }
                                        }
                                        .unwrap();
                                    }
                                    writeln!(&mut s).unwrap();
                                }
                            }

                            DistributionPreference::DaysOff { days_off, priority, .. } => {
                                writeln!(&mut s, "        days off:{} priority {}", days_off, priority).unwrap();
                            }

                            DistributionPreference::DaysEvenlySpread { priority, .. } => {
                                writeln!(&mut s, "        days evenly spread priority {}", priority).unwrap();
                            }
                        }
                    }
                }
            }

            Criterion::FacultyRoomCount { priority, faculty, desired, sections } => {
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

impl Penalty {
    pub fn get_priority(&self) -> u8 {
        match *self {
            Penalty::SoftConflict { priority, .. } => priority,

            Penalty::AntiConflict { priority, .. } => priority,

            Penalty::RoomPreference { priority, .. } => priority,

            Penalty::TimeSlotPreference { priority, .. } => priority,

            Penalty::Cluster { priority, .. } => priority,

            Penalty::Gap { priority, .. } => priority,

            Penalty::DaysOff { priority, .. } => priority,

            Penalty::DaysEvenlySpread { priority, .. } => priority,

            Penalty::RoomCount { priority, .. } => priority,
        }
    }

    pub fn get_score_message(&self, input: &Input, schedule: &Schedule) -> (u8, String) {
        match self {
            &Penalty::SoftConflict { priority, sections: [a, b] } => {
                let ts_a = schedule.placements[a].time_slot.unwrap();
                let ts_b = schedule.placements[b].time_slot.unwrap();
                if ts_a == ts_b {
                    (
                        priority,
                        format!(
                            "course conflict: {} and {} both meet at {}",
                            input.sections[a].name, input.sections[b].name, input.time_slots[ts_a].name
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

            Penalty::AntiConflict { priority, single, group } => {
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

            &Penalty::RoomPreference { priority, section } => {
                let room = schedule.placements[section].room.unwrap();
                (
                    priority,
                    format!("room preference: {} meets in {}", input.sections[section].name, input.rooms[room].name),
                )
            }

            &Penalty::TimeSlotPreference { priority, section } => {
                let time_slot = schedule.placements[section].time_slot.unwrap();
                (
                    priority,
                    format!(
                        "time slot preference: {} meets at {}",
                        input.sections[section].name, input.time_slots[time_slot].name
                    ),
                )
            }

            &Penalty::Cluster { priority, faculty, is_too_short, .. } => (
                priority,
                format!(
                    "class cluster preference: {} has a cluster of classes that is too {}",
                    input.faculty[faculty].name,
                    if is_too_short { "short" } else { "long" }
                ),
            ),

            &Penalty::Gap { priority, faculty, is_too_short, .. } => (
                priority,
                format!(
                    "gap preference: {} has a gap between clusters of classes that is too {}",
                    input.faculty[faculty].name,
                    if is_too_short { "short" } else { "long" }
                ),
            ),

            &Penalty::DaysOff { priority, faculty, desired, actual, .. } => (
                priority,
                format!(
                    "days off: {} wanted {} day{} off but got {}",
                    input.faculty[faculty].name,
                    desired,
                    if desired == 1 { "" } else { "s" },
                    actual
                ),
            ),

            &Penalty::DaysEvenlySpread { priority, faculty, .. } => (
                priority,
                format!("uneven spread: {} has more classes some days than others", input.faculty[faculty].name),
            ),

            &Penalty::RoomCount { priority, faculty, desired, actual, .. } => (
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
