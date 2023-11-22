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
                    if solver.sections[elt].placement.is_some() {
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
                        "soft conflict: {}-{} and {}-{} both meet at {}",
                        input.sections[a].course,
                        input.sections[a].section,
                        input.sections[b].course,
                        input.sections[b].section,
                        input.time_slots[ts_a].name
                    )
                } else {
                    format!(
                        "soft conflict: {}-{} at {} overlaps {}-{} at {}",
                        input.sections[a].course,
                        input.sections[a].section,
                        input.time_slots[ts_a].name,
                        input.sections[b].course,
                        input.sections[b].section,
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
                    "room/time combination: {}-{} meets in {} at {}",
                    elt.course,
                    elt.section,
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
                let message = format!(
                    "unplaced section: {}-{}",
                    input.sections[*section].course, input.sections[*section].section
                );
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
                        "anticonflict: section {}-{} is not at the same time as {}-{}",
                        input.sections[*single].course,
                        input.sections[*single].section,
                        input.sections[other].course,
                        input.sections[other].section
                    )
                } else {
                    let mut s = format!(
                        "anticonflict: section {}-{} is not at the same time as ",
                        input.sections[*single].course, input.sections[*single].section
                    );
                    let mut or = "";
                    for elt in group {
                        s.push_str(or);
                        or = " or ";
                        s.push_str(&input.sections[*elt].course);
                        s.push('-');
                        s.push_str(&input.sections[*elt].section);
                    }
                    s
                };
                list.push((*global, message));
            }
        }
    }
}

#[derive(Clone)]
pub enum SectionScoreDetails {
    SoftConflict { sections: Vec<usize> },
    RoomTimePenalty { section: usize },
    SectionNotPlaced { section: usize },
    AntiConflict { single: usize, group: Vec<usize> },
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
        }
    }
}
