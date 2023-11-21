use super::input::*;
use super::solver::*;

#[derive(Clone)]
pub enum ScoreCriterion {
    SoftConflict {
        sections_with_penalties: Vec<SectionWithPenalty>,
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
        };
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
        }
    }
}

#[derive(Clone)]
pub enum SectionScoreDetails {
    SoftConflict { sections: Vec<usize> },
    RoomTimePenalty { section: usize },
    SectionNotPlaced { section: usize },
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
        }
    }
}
