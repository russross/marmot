use super::input::*;
use super::solver::*;

#[derive(Clone)]
pub enum ScoreCriterion {
    // if this section overlaps any section in the list,
    // the associated penalty is applied
    SoftConflict {
        sections_with_penalties: Vec<SectionWithPenalty>,
    },

    // section single must share a time slot with a section from group
    // if it does not, the penalty is applied locally to every section
    // in the group and globally for the single section
    AntiConflict {
        penalty: isize,
        single: usize,
        group: Vec<usize>,
    },

    InstructorClassSpread {
        instructor: usize,
        sections: Vec<usize>,
        grouped_by_days: Vec<Vec<DistributionPreference>>,
    },

    InstructorRoomCount {
        instructor: usize,
        sections: Vec<usize>,
        desired: usize,
        penalty: isize,
    },
}

impl ScoreCriterion {
    pub fn check(
        &self,
        solver: &Solver,
        input: &Input,
        section: usize,
        records: &mut Vec<SectionScoreRecord>,
    ) {
        match self {
            ScoreCriterion::SoftConflict {
                sections_with_penalties,
            } => {
                // grab the time slot we are placed in
                let Some(RoomTimeWithPenalty {
                    time_slot: my_time_slot,
                    ..
                }) = solver.sections[section].placement
                else {
                    panic!("check called on unplaced section");
                };

                for &SectionWithPenalty {
                    section: other,
                    penalty,
                } in sections_with_penalties
                {
                    // check for placement of the conflicting course
                    let Some(RoomTimeWithPenalty {
                        time_slot: other_time_slot,
                        ..
                    }) = solver.sections[other].placement
                    else {
                        continue;
                    };

                    // we only care if there is an overlap
                    if !input.time_slots_conflict(my_time_slot, other_time_slot) {
                        continue;
                    }

                    // if we make it this far, there is a soft conflict
                    records.push(SectionScoreRecord {
                        local: penalty,
                        global: if section < other { penalty } else { 0 },
                        details: SectionScoreDetails::SoftConflict {
                            sections: vec![section, other],
                        },
                    });
                }
            }

            ScoreCriterion::AntiConflict {
                penalty,
                single,
                group,
            } => {
                // grab the time slot of the single section
                let Some(RoomTimeWithPenalty { time_slot, .. }) =
                    solver.sections[*single].placement
                else {
                    // single section is unplaced, move on
                    return;
                };

                // only consider placed sections from the group
                let mut placed = Vec::new();
                for &elt in group {
                    if solver.is_placed(elt) {
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
                    .any(|&i| solver.sections[i].is_placed_at_time_slot(time_slot))
                {
                    return;
                }
                records.push(SectionScoreRecord {
                    local: *penalty,
                    global: if section == *single { *penalty } else { 0 },
                    details: SectionScoreDetails::AntiConflict {
                        single: *single,
                        group: placed,
                    },
                });
            }

            ScoreCriterion::InstructorClassSpread {
                instructor,
                sections,
                grouped_by_days,
            } => {
                let section_of_record = *sections.iter().min().unwrap();

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
                    let mut schedule_by_day: Vec<Vec<(time::Time, time::Duration)>> =
                        vec![Vec::new(); days.len()];

                    for &section in sections {
                        // find when the section was placed
                        let Some(RoomTimeWithPenalty { time_slot, .. }) =
                            solver.sections[section].placement
                        else {
                            continue;
                        };

                        // check each day we are interested in
                        for (i, &day) in days.iter().enumerate() {
                            let ts = &input.time_slots[time_slot];

                            // if this section is not scheduled on a day of interest, ignore it
                            if !ts.days.contains(&day) {
                                continue;
                            }

                            let interval = (ts.start_time, ts.duration);

                            schedule_by_day[i].push(interval);
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
                                    let mut end_time = day[0].0 + day[0].1;
                                    while cluster_start < day.len() {
                                        // keep adding sections while there are more and they start
                                        // soon enough after the end of the previous section
                                        while cluster_end + 1 < day.len()
                                            && end_time + *max_gap >= day[cluster_end + 1].0
                                        {
                                            cluster_end += 1;
                                            end_time = day[cluster_end].0 + day[cluster_end].1;
                                        }

                                        // how long is it?
                                        let total_duration = end_time - day[cluster_start].0;

                                        let mut worst_penalty = 0;
                                        let mut is_too_short = false;

                                        // test cluster size against all the limits
                                        for limit in cluster_limits {
                                            match *limit {
                                                DurationWithPenalty::TooShort {
                                                    duration,
                                                    penalty,
                                                } => {
                                                    if total_duration < duration {
                                                        if too_short_okay {
                                                            // used up the one freebie
                                                            too_short_okay = false;
                                                            continue;
                                                        }

                                                        if penalty > worst_penalty {
                                                            worst_penalty = penalty;
                                                            is_too_short = true;
                                                        }
                                                    }
                                                }

                                                DurationWithPenalty::TooLong {
                                                    duration,
                                                    penalty,
                                                } => {
                                                    if total_duration > duration
                                                        && penalty > worst_penalty
                                                    {
                                                        worst_penalty = penalty;
                                                        is_too_short = false;
                                                    }
                                                }
                                            }
                                        }

                                        if worst_penalty > 0 {
                                            records.push(SectionScoreRecord {
                                                local: worst_penalty,
                                                global: if section == section_of_record {
                                                    worst_penalty
                                                } else {
                                                    0
                                                },
                                                details: SectionScoreDetails::Cluster {
                                                    instructor: *instructor,
                                                    is_too_short,
                                                    sections: sections.clone(),
                                                },
                                            });
                                        }

                                        // check the size of the gap between the end of this
                                        // cluster and the start of the next
                                        if cluster_end + 1 < day.len() {
                                            let gap = day[cluster_end + 1].0 - end_time;

                                            // search the limits
                                            worst_penalty = 0;
                                            is_too_short = false;

                                            for limit in gap_limits {
                                                match *limit {
                                                    DurationWithPenalty::TooShort {
                                                        duration,
                                                        penalty,
                                                    } => {
                                                        if gap < duration && penalty > worst_penalty
                                                        {
                                                            worst_penalty = penalty;
                                                            is_too_short = true;
                                                        }
                                                    }

                                                    DurationWithPenalty::TooLong {
                                                        duration,
                                                        penalty,
                                                    } => {
                                                        if gap > duration && penalty > worst_penalty
                                                        {
                                                            worst_penalty = penalty;
                                                            is_too_short = false;
                                                        }
                                                    }
                                                }
                                            }

                                            if worst_penalty > 0 {
                                                records.push(SectionScoreRecord {
                                                    local: worst_penalty,
                                                    global: if section == section_of_record {
                                                        worst_penalty
                                                    } else {
                                                        0
                                                    },
                                                    details: SectionScoreDetails::Gap {
                                                        instructor: *instructor,
                                                        is_too_short,
                                                        sections: sections.clone(),
                                                    },
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
                                penalty,
                                ..
                            } => {
                                let mut actual = 0;
                                for day in &schedule_by_day {
                                    if day.is_empty() {
                                        actual += 1;
                                    }
                                }
                                if actual != desired {
                                    records.push(SectionScoreRecord {
                                        local: penalty,
                                        global: if section == section_of_record {
                                            penalty
                                        } else {
                                            0
                                        },
                                        details: SectionScoreDetails::DaysOff {
                                            instructor: *instructor,
                                            desired,
                                            actual,
                                            sections: sections.clone(),
                                        },
                                    });
                                }
                            }

                            &DistributionPreference::DaysEvenlySpread { penalty, .. } => {
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

                                if most > fewest && most - fewest > 1 {
                                    records.push(SectionScoreRecord {
                                        local: penalty,
                                        global: if section == section_of_record {
                                            penalty
                                        } else {
                                            0
                                        },
                                        details: SectionScoreDetails::DaysEvenlySpread {
                                            instructor: *instructor,
                                            sections: sections.clone(),
                                        },
                                    });
                                }
                            }
                        }
                    }
                }
            }

            ScoreCriterion::InstructorRoomCount {
                instructor,
                sections,
                desired,
                penalty,
            } => {
                let section_of_record = *sections.iter().min().unwrap();
                let mut rooms = Vec::new();
                for &sec in sections {
                    // find when the section was placed
                    let Some(RoomTimeWithPenalty { room, .. }) = solver.sections[sec].placement
                    else {
                        continue;
                    };
                    rooms.push(room);
                }
                rooms.sort();
                rooms.dedup();

                if rooms.len() > *desired {
                    records.push(SectionScoreRecord {
                        local: *penalty,
                        global: if section == section_of_record {
                            *penalty
                        } else {
                            0
                        },
                        details: SectionScoreDetails::TooManyRooms {
                            instructor: *instructor,
                            desired: *desired,
                            actual: rooms.len(),
                            sections: sections.clone(),
                        },
                    });
                }
            }
        }
    }

    pub fn get_neighbors(&self) -> Vec<usize> {
        match self {
            ScoreCriterion::SoftConflict {
                sections_with_penalties,
                ..
            } => sections_with_penalties
                .iter()
                .map(|elt| elt.section)
                .collect(),

            ScoreCriterion::AntiConflict { single, group, .. } => {
                let mut list = group.clone();
                list.push(*single);
                list
            }

            ScoreCriterion::InstructorClassSpread { sections, .. } => sections.clone(),

            ScoreCriterion::InstructorRoomCount { sections, .. } => sections.clone(),
        }
    }
}

#[derive(Clone)]
pub struct SectionScoreRecord {
    pub local: isize,
    pub global: isize,
    pub details: SectionScoreDetails,
}

impl SectionScoreRecord {
    pub fn gather_score_messages(
        &self,
        solver: &Solver,
        input: &Input,
        list: &mut Vec<(isize, String)>,
    ) {
        match self {
            SectionScoreRecord { global: 0, .. } => {
                // ignore any record with no global score contribution
            }
            SectionScoreRecord {
                details: SectionScoreDetails::SoftConflict { sections },
                global,
                ..
            } => {
                assert!(sections.len() == 2);
                let (a, b) = (sections[0], sections[1]);

                let Some(RoomTimeWithPenalty {
                    time_slot: ts_a, ..
                }) = solver.sections[a].placement
                else {
                    panic!("RoomTimePenalty on unplaced section");
                };
                let Some(RoomTimeWithPenalty {
                    time_slot: ts_b, ..
                }) = solver.sections[b].placement
                else {
                    panic!("RoomTimePenalty on unplaced section");
                };
                let message = if ts_a == ts_b {
                    format!(
                        "soft conflict: {} and {} both meet at {}",
                        input.sections[a].get_name(),
                        input.sections[b].get_name(),
                        input.time_slots[ts_a].name
                    )
                } else {
                    format!(
                        "soft conflict: {} at {} overlaps {} at {}",
                        input.sections[a].get_name(),
                        input.time_slots[ts_a].name,
                        input.sections[b].get_name(),
                        input.time_slots[ts_b].name
                    )
                };
                list.push((*global, message));
            }

            SectionScoreRecord {
                details: SectionScoreDetails::RoomTimePenalty { section },
                global,
                ..
            } => {
                let Some(RoomTimeWithPenalty {
                    room, time_slot, ..
                }) = solver.sections[*section].placement
                else {
                    panic!("RoomTimePenalty on unplaced section");
                };
                let elt = &input.sections[*section];

                let message = format!(
                    "room/time combination: {} meets in {} at {}",
                    elt.get_name(),
                    input.rooms[room].name,
                    input.time_slots[time_slot].name
                );
                list.push((*global, message));
            }

            SectionScoreRecord {
                details: SectionScoreDetails::SectionNotPlaced { section },
                global,
                ..
            } => {
                let message = format!("unplaced section: {}", input.sections[*section].get_name());
                list.push((*global, message));
            }

            SectionScoreRecord {
                details: SectionScoreDetails::AntiConflict { single, group },
                global,
                ..
            } => {
                let message = if group.len() == 1 {
                    let other = group[0];
                    format!(
                        "anticonflict: section {} is not at the same time as {}",
                        input.sections[*single].get_name(),
                        input.sections[other].get_name()
                    )
                } else {
                    let mut s = format!(
                        "anticonflict: section {} is not at the same time as ",
                        input.sections[*single].get_name()
                    );
                    let mut or = "";
                    for elt in group {
                        s.push_str(or);
                        or = " or ";
                        s.push_str(&input.sections[*elt].get_name());
                    }
                    s
                };
                list.push((*global, message));
            }

            SectionScoreRecord {
                details:
                    SectionScoreDetails::Cluster {
                        instructor,
                        is_too_short,
                        ..
                    },
                global,
                ..
            } => {
                let message = format!(
                    "class cluster: instructor {} has a cluster of classes that is too {}",
                    input.instructors[*instructor].name,
                    if *is_too_short { "short" } else { "long" }
                );
                list.push((*global, message));
            }

            SectionScoreRecord {
                details:
                    SectionScoreDetails::Gap {
                        instructor,
                        is_too_short,
                        ..
                    },
                global,
                ..
            } => {
                let message = format!(
                    "class cluster: instructor {} has a gap between clusters of classes that is too {}",
                    input.instructors[*instructor].name,
                    if *is_too_short { "short" } else {"long"}
                );
                list.push((*global, message));
            }

            SectionScoreRecord {
                details:
                    SectionScoreDetails::DaysOff {
                        instructor,
                        desired,
                        actual,
                        ..
                    },
                global,
                ..
            } => {
                let message = format!(
                    "days off: instructor {} wanted {} day{} off but got {}",
                    input.instructors[*instructor].name,
                    desired,
                    if *desired == 1 { "" } else { "s" },
                    actual
                );
                list.push((*global, message));
            }

            SectionScoreRecord {
                details: SectionScoreDetails::DaysEvenlySpread { instructor, .. },
                global,
                ..
            } => {
                let message = format!(
                    "class spread: instructor {} does not have classes evenly spread across days",
                    input.instructors[*instructor].name
                );
                list.push((*global, message));
            }

            SectionScoreRecord {
                details:
                    SectionScoreDetails::TooManyRooms {
                        instructor,
                        desired,
                        actual,
                        ..
                    },
                global,
                ..
            } => {
                let message = format!(
                    "room placement: instructor {} wanted all classes in {} room{} but got {} room{}",
                    input.instructors[*instructor].name,
                    desired,
                    if *desired == 1 { "" } else { "s" },
                    actual,
                    if *actual == 1 { "" } else { "s" },
                );
                list.push((*global, message));
            }
        }
    }
}

#[derive(Clone)]
pub enum SectionScoreDetails {
    SoftConflict {
        sections: Vec<usize>,
    },
    RoomTimePenalty {
        section: usize,
    },
    SectionNotPlaced {
        section: usize,
    },
    AntiConflict {
        single: usize,
        group: Vec<usize>,
    },
    Cluster {
        instructor: usize,
        is_too_short: bool,
        sections: Vec<usize>,
    },
    Gap {
        instructor: usize,
        is_too_short: bool,
        sections: Vec<usize>,
    },
    DaysOff {
        instructor: usize,
        desired: u8,
        actual: u8,
        sections: Vec<usize>,
    },
    DaysEvenlySpread {
        instructor: usize,
        sections: Vec<usize>,
    },
    TooManyRooms {
        instructor: usize,
        desired: usize,
        actual: usize,
        sections: Vec<usize>,
    },
}

impl SectionScoreDetails {
    // gather sections that are involved in the score, but skip any in the exclude list
    pub fn gather_adjacent_sections(&self, adjacent: &mut Vec<usize>, exclude: &[usize]) {
        match self {
            SectionScoreDetails::SoftConflict { sections } => {
                for section in sections {
                    if !exclude.contains(section) {
                        adjacent.push(*section);
                    }
                }
            }
            SectionScoreDetails::RoomTimePenalty { section } => {
                if !exclude.contains(section) {
                    adjacent.push(*section);
                }
            }
            SectionScoreDetails::SectionNotPlaced { section } => {
                if !exclude.contains(section) {
                    adjacent.push(*section);
                }
            }
            SectionScoreDetails::AntiConflict { single, group } => {
                if !exclude.contains(single) {
                    adjacent.push(*single);
                }
                for section in group {
                    if !exclude.contains(section) {
                        adjacent.push(*section);
                    }
                }
            }
            SectionScoreDetails::Cluster { sections, .. } => {
                for section in sections {
                    if !exclude.contains(section) {
                        adjacent.push(*section);
                    }
                }
            }
            SectionScoreDetails::Gap { sections, .. } => {
                for section in sections {
                    if !exclude.contains(section) {
                        adjacent.push(*section);
                    }
                }
            }
            SectionScoreDetails::DaysOff { sections, .. } => {
                for section in sections {
                    if !exclude.contains(section) {
                        adjacent.push(*section);
                    }
                }
            }
            SectionScoreDetails::DaysEvenlySpread { sections, .. } => {
                for section in sections {
                    if !exclude.contains(section) {
                        adjacent.push(*section);
                    }
                }
            }
            SectionScoreDetails::TooManyRooms { sections, .. } => {
                for section in sections {
                    if !exclude.contains(section) {
                        adjacent.push(*section);
                    }
                }
            }
        }
    }
}
