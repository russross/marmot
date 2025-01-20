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
        section: usize,
        rooms_with_priorities: Vec<RoomWithPriority>,
    },
    TimeSlotPreference {
        section: usize,
        time_slots_with_priorities: Vec<TimeSlotWithPriority>,
    },
    FacultyPreference {
        faculty: usize,
        sections: Vec<usize>,
        days_to_check: Vec<time::Weekday>,
        days_off: Option<(u8, usize)>,
        evenly_spread: Option<u8>,
        no_room_switch: Option<u8>,
        too_many_rooms: Option<(u8, usize)>,
        max_gap_within_cluster: time::Duration,
        distribution_intervals: Vec<DistributionInterval>,
    },
}

#[derive(Clone)]
pub enum DistributionInterval {
    GapTooShort { priority: u8, duration: time::Duration },
    GapTooLong { priority: u8, duration: time::Duration },
    ClusterTooShort { priority: u8, duration: time::Duration },
    ClusterTooLong { priority: u8, duration: time::Duration },
}

// a single change to the score due to a section's placement
#[derive(Clone)]
pub enum Penalty {
    SoftConflict { priority: u8, sections: [usize; 2] },
    AntiConflict { priority: u8, single: usize, group: Vec<usize> },
    RoomPreference { priority: u8, section: usize, room: usize },
    TimeSlotPreference { priority: u8, section: usize, time_slot: usize },
    ClusterTooShort { priority: u8, faculty: usize, duration: time::Duration },
    ClusterTooLong { priority: u8, faculty: usize, duration: time::Duration },
    GapTooShort { priority: u8, faculty: usize, duration: time::Duration },
    GapTooLong { priority: u8, faculty: usize, duration: time::Duration },
    DaysOff { priority: u8, faculty: usize, desired: usize, actual: usize },
    DaysEvenlySpread { priority: u8, faculty: usize },
    RoomSwitch { priority: u8, faculty: usize, sections: [usize; 2], rooms: [usize; 2] },
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

            Criterion::FacultyPreference { sections, .. } => sections.clone(),
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
                            return vec![Penalty::RoomPreference { priority, section: *section, room }];
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
                            return vec![Penalty::TimeSlotPreference { priority, section: *section, time_slot }];
                        }
                    }
                }
                Vec::new()
            }

            Criterion::FacultyPreference {
                faculty,
                sections,
                days_to_check,
                days_off,
                evenly_spread,
                no_room_switch,
                too_many_rooms,
                max_gap_within_cluster,
                distribution_intervals,
            } => {
                let mut penalties = Vec::new();

                // for each day in days_to_check, a list of (start time, duration, maybe_room)
                // of sections scheduled on that day
                #[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
                struct Interval {
                    start_time: time::Time,
                    duration: time::Duration,
                    end_time: time::Time,
                    section: usize,
                    maybe_room: Option<usize>,
                }
                let mut schedule_by_day: Vec<Vec<Interval>> = vec![Vec::new(); days_to_check.len()];

                for &section in sections {
                    // find when the section was placed
                    let Some(time_slot) = schedule.placements[section].time_slot else {
                        continue;
                    };

                    // check each day we are interested in
                    for (i, &day) in days_to_check.iter().enumerate() {
                        let TimeSlot { start_time, duration, days, .. } = &input.time_slots[time_slot];

                        // if this section is not scheduled on a day of interest, ignore it
                        if !days.contains(&day) {
                            continue;
                        }

                        schedule_by_day[i].push(Interval {
                            section,
                            start_time: *start_time,
                            duration: *duration,
                            end_time: *start_time + *duration,
                            maybe_room: schedule.placements[section].room,
                        });
                    }
                }
                for day_schedule in &mut schedule_by_day {
                    day_schedule.sort_unstable();
                }

                // now process the individual scoring criteria
                if let &Some((priority, desired)) = days_off {
                    let mut actual = 0;
                    for day in &schedule_by_day {
                        if day.is_empty() {
                            actual += 1;
                        }
                    }
                    if actual != desired {
                        penalties.push(Penalty::DaysOff { priority, faculty: *faculty, desired, actual });
                    }
                }

                if let &Some(priority) = evenly_spread {
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
                        penalties.push(Penalty::DaysEvenlySpread { priority, faculty: *faculty });
                    }
                }

                if let Some(priority) = no_room_switch {
                    for day in &schedule_by_day {
                        for pair in day.windows(2) {
                            let (a, b) = (&pair[0], &pair[1]);

                            // are these back-to-back?
                            if b.start_time - a.end_time > *max_gap_within_cluster {
                                continue;
                            }

                            // are they both in rooms?
                            if let (Some(room_a), Some(room_b)) = (a.maybe_room, b.maybe_room) {
                                // are they different rooms?
                                if room_a == room_b {
                                    continue;
                                }

                                // is it possible for them to be in the same room without penalty?
                                if input.sections[a.section]
                                    .rooms
                                    .iter()
                                    .any(|rwp| rwp.priority.is_none() && input.sections[b.section].rooms.contains(rwp))
                                {
                                    penalties.push(Penalty::RoomSwitch {
                                        priority: *priority,
                                        faculty: *faculty,
                                        sections: [a.section, b.section],
                                        rooms: [room_a, room_b],
                                    });
                                }
                            }
                        }
                    }
                }

                if let &Some((priority, desired)) = too_many_rooms {
                    let mut rooms: Vec<usize> =
                        sections.iter().filter_map(|&section| schedule.placements[section].room).collect();
                    rooms.sort_unstable();
                    rooms.dedup();
                    if rooms.len() != desired {
                        penalties.push(Penalty::RoomCount {
                            priority,
                            faculty: *faculty,
                            desired,
                            actual: rooms.len(),
                        });
                    }
                }

                if !distribution_intervals.is_empty() {
                    for day in &schedule_by_day {
                        if day.is_empty() {
                            continue;
                        }
                        let clusters: Vec<(time::Time, time::Time)> = day
                            .chunk_by(|a, b| b.start_time - a.end_time <= *max_gap_within_cluster)
                            .map(|chunk| (chunk.first().unwrap().start_time, chunk.last().unwrap().end_time))
                            .collect();
                        let gaps: Vec<time::Duration> = clusters.windows(2).map(|pair| pair[1].0 - pair[0].1).collect();

                        // ignore one too-short cluster per day
                        let mut too_short_okay = true;
                        for &(start_time, end_time) in &clusters {
                            let mut too_short_priority = u8::MAX;
                            let mut too_long_priority = u8::MAX;
                            let actual = end_time - start_time;
                            for interval in distribution_intervals {
                                match *interval {
                                    DistributionInterval::ClusterTooShort { priority, duration } => {
                                        if actual < duration {
                                            too_short_priority = std::cmp::min(priority, too_short_priority);
                                        }
                                    }
                                    DistributionInterval::ClusterTooLong { priority, duration } => {
                                        if actual > duration {
                                            too_long_priority = std::cmp::min(priority, too_long_priority);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            if too_short_priority < u8::MAX {
                                if too_short_okay {
                                    too_short_okay = false;
                                } else {
                                    penalties.push(Penalty::ClusterTooShort {
                                        priority: too_short_priority,
                                        faculty: *faculty,
                                        duration: actual,
                                    });
                                }
                            }
                            if too_long_priority < u8::MAX {
                                penalties.push(Penalty::ClusterTooLong {
                                    priority: too_long_priority,
                                    faculty: *faculty,
                                    duration: actual,
                                });
                            }
                        }

                        for &actual in &gaps {
                            let mut too_short_priority = u8::MAX;
                            let mut too_long_priority = u8::MAX;
                            for interval in distribution_intervals {
                                match *interval {
                                    DistributionInterval::GapTooShort { priority, duration } => {
                                        if actual < duration {
                                            too_short_priority = std::cmp::min(priority, too_short_priority);
                                        }
                                    }
                                    DistributionInterval::GapTooLong { priority, duration } => {
                                        if actual > duration {
                                            too_long_priority = std::cmp::min(priority, too_long_priority);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            if too_short_priority < u8::MAX {
                                penalties.push(Penalty::GapTooShort {
                                    priority: too_short_priority,
                                    faculty: *faculty,
                                    duration: actual,
                                });
                            }
                            if too_long_priority < u8::MAX {
                                penalties.push(Penalty::GapTooLong {
                                    priority: too_long_priority,
                                    faculty: *faculty,
                                    duration: actual,
                                });
                            }
                        }
                    }
                }

                penalties
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
                write!(&mut s, "time slot preferences:").unwrap();
                for &TimeSlotWithPriority { time_slot, priority } in time_slots_with_priorities {
                    write!(&mut s, " {}:{}", input.time_slots[time_slot].name, priority).unwrap();
                }
            }

            Criterion::FacultyPreference {
                faculty,
                sections,
                days_to_check,
                days_off,
                evenly_spread,
                no_room_switch,
                too_many_rooms,
                max_gap_within_cluster: _max_gap_within_cluster,
                distribution_intervals,
            } => {
                write!(&mut s, "faculty preference: {} with [", input.faculty[*faculty].name).unwrap();
                let mut sep = "";
                for &section in sections {
                    write!(&mut s, "{}{}", sep, input.sections[section].name).unwrap();
                    sep = ",";
                }
                write!(&mut s, "] using days [").unwrap();
                sep = "";
                for &day in days_to_check {
                    match day {
                        time::Weekday::Monday => write!(&mut s, "{sep}Mon"),
                        time::Weekday::Tuesday => write!(&mut s, "{sep}Tues"),
                        time::Weekday::Wednesday => write!(&mut s, "{sep}Wed"),
                        time::Weekday::Thursday => write!(&mut s, "{sep}Thurs"),
                        time::Weekday::Friday => write!(&mut s, "{sep}Fri"),
                        time::Weekday::Saturday => write!(&mut s, "{sep}Sat"),
                        time::Weekday::Sunday => write!(&mut s, "{sep}Sun"),
                    }
                    .unwrap();
                    sep = ",";
                }
                writeln!(&mut s, ")").unwrap();

                if let &Some((priority, days)) = days_off {
                    writeln!(&mut s, "    {}: wants {} day{} off", priority, days, if days == 1 { "" } else { "s" })
                        .unwrap();
                }

                if let &Some(priority) = evenly_spread {
                    writeln!(&mut s, "    {}: wants classes evenly spread across days", priority).unwrap();
                }

                if let &Some(priority) = no_room_switch {
                    writeln!(&mut s, "    {}: wants no back-to-back room switches", priority).unwrap();
                }

                if let &Some((priority, desired)) = too_many_rooms {
                    writeln!(
                        &mut s,
                        "    {}: wants to only use {} room{}",
                        priority,
                        desired,
                        if desired == 1 { "" } else { "s" }
                    )
                    .unwrap();
                }

                for interval in distribution_intervals {
                    let (priority, kind, shortlong, duration) = match *interval {
                        DistributionInterval::GapTooShort { priority, duration } => {
                            (priority, "gap", "shorter", duration)
                        }
                        DistributionInterval::GapTooLong { priority, duration } => {
                            (priority, "gap", "longer", duration)
                        }
                        DistributionInterval::ClusterTooShort { priority, duration } => {
                            (priority, "cluster", "shorter", duration)
                        }
                        DistributionInterval::ClusterTooLong { priority, duration } => {
                            (priority, "cluster", "longer", duration)
                        }
                    };
                    writeln!(&mut s, "    {}: {} should not be {} than {}", priority, kind, shortlong, duration)
                        .unwrap();
                }
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

            Penalty::ClusterTooShort { priority, .. } => priority,

            Penalty::ClusterTooLong { priority, .. } => priority,

            Penalty::GapTooShort { priority, .. } => priority,

            Penalty::GapTooLong { priority, .. } => priority,

            Penalty::DaysOff { priority, .. } => priority,

            Penalty::DaysEvenlySpread { priority, .. } => priority,

            Penalty::RoomSwitch { priority, .. } => priority,

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
                            "{} and {} both meet at {}",
                            input.sections[a].name, input.sections[b].name, input.time_slots[ts_a].name
                        ),
                    )
                } else {
                    (
                        priority,
                        format!(
                            "{} at {} overlaps {} at {}",
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
                (*priority, format!("{} should be at the same time as {}", input.sections[*single].name, group_names))
            }

            &Penalty::RoomPreference { priority, section, room } => {
                (priority, format!("{} is assigned to {}", input.sections[section].name, input.rooms[room].name))
            }

            &Penalty::TimeSlotPreference { priority, section, time_slot } => (
                priority,
                format!("{} is scheduled at {}", input.sections[section].name, input.time_slots[time_slot].name),
            ),

            &Penalty::ClusterTooShort { priority, faculty, duration } => (
                priority,
                format!("{} has a cluster of classes that is only {} long", input.faculty[faculty].name, duration,),
            ),

            &Penalty::ClusterTooLong { priority, faculty, duration } => (
                priority,
                format!("{} has a run of back-to-back classes that lasts {}", input.faculty[faculty].name, duration,),
            ),

            &Penalty::GapTooShort { priority, faculty, duration } => (
                priority,
                format!(
                    "{} has a break between clusters of classes that is only {} long",
                    input.faculty[faculty].name, duration,
                ),
            ),

            &Penalty::GapTooLong { priority, faculty, duration } => (
                priority,
                format!("{} has to wait {} between clusters of classes", input.faculty[faculty].name, duration,),
            ),

            &Penalty::DaysOff { priority, faculty, desired, actual: _actual } => (
                priority,
                if desired == 0 {
                    format!("{} has a day off but does not want one", input.faculty[faculty].name)
                } else {
                    format!("{} wants a day off but did not get one", input.faculty[faculty].name)
                },
            ),

            &Penalty::DaysEvenlySpread { priority, faculty } => {
                (priority, format!("{} has more classes some days than others", input.faculty[faculty].name))
            }

            &Penalty::RoomSwitch { priority, faculty, sections: [section_a, section_b], rooms: [room_a, room_b] } => (
                priority,
                format!(
                    "{} has {} and {} back-to-back and has to move from {} to {}",
                    input.faculty[faculty].name,
                    input.sections[section_a].name,
                    input.sections[section_b].name,
                    input.rooms[room_a].name,
                    input.rooms[room_b].name,
                ),
            ),

            &Penalty::RoomCount { priority, faculty, desired, actual } => (
                priority,
                format!(
                    "{}'s classes are spread across {} room{} instead of being packed into {}",
                    input.faculty[faculty].name,
                    actual,
                    if actual == 1 { "" } else { "s" },
                    desired
                ),
            ),
        }
    }
}
