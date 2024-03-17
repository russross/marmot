use super::bits::*;
use super::input::*;
use super::score::*;
use itertools::Itertools;
use rand::Rng;
use std::fmt::Write;
use std::fs;
use std::rc::Rc;

const PENALTY_FOR_UNPLACED_SECTION: isize = 1000;
const MIN_LOTTERY_TICKETS_FOR_UNPLACED_SECTION: isize = 1000;

#[derive(Clone)]
pub struct Solver {
    // the name of the term
    pub name: String,

    // the start and end dates (inclusive) of the term
    pub start: time::Date,
    pub end: time::Date,

    // every 5-minute interval during the semester, with holidays blocked out
    pub slots: Bits,

    // core schedule data
    pub rooms: Vec<Room>,
    pub time_slots: Vec<TimeSlot>,
    pub instructors: Vec<Instructor>,
    pub input_sections: Vec<InputSection>,

    // list of sections mentioned in conflict/scoring but not actually defined
    // note that a section must be created before any references to it are valid
    pub missing: Vec<String>,

    // matrix of which time slots overlap which for fast lookup
    pub time_slot_conflicts: Vec<bool>,

    // scoring data
    pub anticonflicts: Vec<(isize, usize, Vec<usize>)>,

    //
    // everything above this point becomes immutable once
    // input is finished and post-processed
    //
    pub input_locked: bool,

    // solver data
    pub sections: Vec<SolverSection>,
    pub room_placements: Vec<RoomPlacements>,
    pub score: isize,
    pub unplaced_current: usize,
    pub unplaced_best: usize,
}

#[derive(Clone)]
pub struct RoomPlacements {
    pub time_slot_placements: Vec<Option<usize>>,
}

#[derive(Clone)]
pub struct SolverSection {
    // the current room/time assignment for this section
    pub placement: Option<RoomTimeWithPenalty>,

    // the number of lottery tickets assigned to this section
    // higher means it is more likely to be selected for re-assignment,
    // zero means it will not be selected (but could still be displaced
    // by another section moving)
    pub tickets: isize,

    // scoring that will be applied specifically to this section
    pub score_criteria: Vec<Rc<dyn ScoreCriterion>>,

    // scoring info for the current placement
    pub score: SectionScore,

    // these lists combine the relevant data across all cross-listings
    // and all instructors and account for sections that are pinned in place
    // i.e., the list of room/times does not include any entries that would
    // lead to a pinned class being displaced
    pub room_times: Vec<RoomTimeWithPenalty>,
    pub instructors: Vec<usize>,
    pub hard_conflicts: Vec<usize>,
    pub soft_conflicts_list: Vec<SectionWithPenalty>,
    pub neighbors: Vec<usize>,

    // map from input.section[_].room_times to the effect on the score
    // if we placed this section at that room/time
    pub speculative_deltas: Vec<Option<isize>>,

    // the maximum penalty improvement possible if this section was moved
    // computed as the minimum delta in speculative_deltas
    pub speculative_delta_min: Option<isize>,

    // if a section is a non-primary cross listing, it is basically ignored
    // by the solver and just exists to keep the numbering in sync with input
    pub is_secondary_cross_listing: bool,
}

impl SolverSection {
    pub fn is_placed_at_time_slot(&self, time_slot: usize) -> bool {
        match self.placement {
            Some(RoomTimeWithPenalty { time_slot: ts, .. }) => time_slot == ts,
            None => false,
        }
    }
}

// Notes on scoring:
// *   A section can be scored independently of any other sections,
//     instructors, etc.
// *   For scores that affect multiple sections, the score must be
//     symmetric, e.g., if section A discovers a penalty involving
//     section B then section B must discover the same penalty
//     involving section A
// *   There is no fanout of scores, i.e., when a section is scored it
//     never reaches into another section to add scoring data
// *   Scores involving multiple sections are recorded for all
//     relevant sections (a pair with a curriculum soft conflict,
//     three classes with the same instructor that are spread out
//     too much, etc.)
// *   A score involving multiple sections is only applied to the
//     overall score onse. The section with the lowest index value
//     applies it to the global score, others only apply it to their
//     own local score.
#[derive(Clone)]
pub struct SectionScore {
    // all penalty points due to this section's placement
    // e.g., room/time penalties, curriculum conflict penalties,
    // instructor preference penalties
    //
    // any penalty that could change if this section was moved
    // goes into this score. note that some penalties may affect
    // multiple sections (like curriculum conflicts) and they
    // will be counted for all applicable sections in this field.
    // this is useful for estimating the maximum potential benefit
    // to moving this section
    pub local: isize,

    // the penalty points that this section directly contributed
    // to the overall score.
    //
    // for penalties that are calculated on multiple sections, the section
    // with the lowest index number contributes them to the final score
    // and the others only contribute to their individual scores
    // (see local above)
    //
    // subtracting this value from the overall score and then re-computing
    // this section's score is a no-op
    pub global: isize,

    // a log of all penalty scores tied to this section in its current
    // placement. when it moves or an adjacent section moves, this can
    // be used to find adjacent sections (those that shared some scoring
    // contribution with this section).
    //
    // note: these records have local and global score fields with the
    // same distinction as the ones above. the overall section scores
    // are just the sum of the deltas in the score_records list.
    //
    // these records are also used to generate score explanations when
    // a schedule is presented to the user
    pub score_records: Vec<SectionScoreRecord>,
}

#[allow(clippy::new_without_default)]
impl SectionScore {
    pub fn new() -> Self {
        SectionScore {
            local: 0,
            global: 0,
            score_records: Vec::new(),
        }
    }

    pub fn new_unplaced(section: usize) -> Self {
        SectionScore {
            local: PENALTY_FOR_UNPLACED_SECTION,
            global: PENALTY_FOR_UNPLACED_SECTION,
            score_records: vec![SectionScoreRecord {
                local: PENALTY_FOR_UNPLACED_SECTION,
                global: PENALTY_FOR_UNPLACED_SECTION,
                details: SectionScoreDetails::SectionNotPlaced { section },
            }],
        }
    }

    pub fn gather_adjacent_sections(&self, adjacent: &mut Vec<usize>, exclude: &[usize]) {
        for SectionScoreRecord { details, .. } in &self.score_records {
            details.gather_adjacent_sections(adjacent, exclude);
        }
    }

    pub fn gather_score_messages(
        &self,
        solver: &Solver,
        list: &mut Vec<(isize, String)>,
        include_dups: bool,
    ) {
        for record in &self.score_records {
            record.gather_score_messages(solver, list, include_dups);
        }
    }
}

pub struct PlacementLog {
    // to undo a move, undo the entries in reverse order
    // and restore all of the scores
    pub entries: Vec<PlacementEntry>,

    // snapshot of the scores of all modified sections before the placement
    // includes sections adjacent to the sections that actually moved
    pub pre_scores: Vec<(usize, SectionScore)>,
}

// a single change to a section's placement
pub enum PlacementEntry {
    // this section was placed (displacing it will undo)
    Add(usize),

    // this section was displaced (placing it will undo)
    Remove(usize, RoomTimeWithPenalty),
}

impl PlacementLog {
    // move a section:
    // *   remove it from its old placement if applicable
    // *   displace any sections with hard conflicts in the new location
    // *   place the section in its new home
    // *   record the steps taken
    // *   update the score based on the move
    //
    // returns a log with enough information to revert the move
    pub fn move_section(
        solver: &mut Solver,
        section: usize,
        room_time: RoomTimeWithPenalty,
    ) -> Self {
        let mut entries = Vec::new();

        // move the section and record displacements
        solver.remove_placement(section, &mut entries);
        solver.displace_conflicts(section, &room_time, &mut entries);
        solver.add_placement_without_displacing(section, &room_time, &mut entries);

        // gather list of sections moved (deduped)
        let mut sections_being_moved = Vec::new();
        for elt in &entries {
            match *elt {
                PlacementEntry::Add(section) => sections_being_moved.push(section),
                PlacementEntry::Remove(section, _) => sections_being_moved.push(section),
            }
        }
        sections_being_moved.sort();
        sections_being_moved.dedup();

        let mut pre_scores = Vec::new();
        let mut adjacent = Vec::new();

        for &section in &sections_being_moved {
            // gather adjacent sections based on the old scoring
            for neighbor in &solver.sections[section].neighbors {
                if !sections_being_moved.contains(neighbor) {
                    adjacent.push(*neighbor);
                }
            }
            /*
            solver.sections[section]
                .score
                .gather_adjacent_sections(&mut adjacent, &sections_being_moved);
            */

            // undo the old score on section being moved
            solver.score -= solver.sections[section].score.global;

            // move the old score records to the log and reset the section score
            let elt = std::mem::replace(&mut solver.sections[section].score, SectionScore::new());
            pre_scores.push((section, elt));

            // compute the new score
            solver.compute_section_score(section);

            // apply it to the global score
            solver.score += solver.sections[section].score.global;

            // gather adjacent sections based on the new scoring
            for neighbor in &solver.sections[section].neighbors {
                if !sections_being_moved.contains(neighbor) {
                    adjacent.push(*neighbor);
                }
            }
            /*
            solver.sections[section]
                .score
                .gather_adjacent_sections(&mut adjacent, &sections_being_moved);
            */
        }

        // dedup adjacent section list
        adjacent.sort();
        adjacent.dedup();

        for &section in &adjacent {
            // undo the old score on adjacent section
            solver.score -= solver.sections[section].score.global;

            // move the old score records to the log and reset the adjacent section score
            let elt = std::mem::replace(&mut solver.sections[section].score, SectionScore::new());
            pre_scores.push((section, elt));

            // compute the new score
            solver.compute_section_score(section);

            // apply it to the global score
            solver.score += solver.sections[section].score.global;
        }

        PlacementLog {
            entries,
            pre_scores,
        }
    }

    pub fn revert_move(&mut self, solver: &mut Solver) {
        // the section placement functions want to record their moves,
        // but we will just throw it away afterward
        let mut dev_null = Vec::new();

        // play the log in reverse order and undo the changes
        loop {
            match self.entries.pop() {
                Some(PlacementEntry::Add(section)) => {
                    solver.remove_placement(section, &mut dev_null);
                }
                Some(PlacementEntry::Remove(section, room_time)) => {
                    solver.add_placement_without_displacing(section, &room_time, &mut dev_null);
                }
                None => break,
            }
        }

        // revert all moved sections and adjacent sections to their pre-move scores
        while let Some((section, score)) = self.pre_scores.pop() {
            let s = &mut solver.sections[section];
            solver.score -= s.score.global;
            s.score = score;
            solver.score += s.score.global;
        }
    }
}

pub trait ScoreCriterion {
    fn check(
        &self,
        solver: &Solver,
        section: usize,
        records: &mut Vec<SectionScoreRecord>,
    );
    fn get_neighbors(&self) -> Vec<usize>;
    fn debug(&self, solver: &Solver) -> String;
}

impl Solver {
    pub fn lock_input(&mut self) -> Result<(), String> {
        for _ in 0..self.rooms.len() {
            self.room_placements.push(RoomPlacements {
                time_slot_placements: vec![None; self.time_slots.len()],
            });
        }
        for i in 0..self.input_sections.len() {
            // non-primary cross listing sections are ignored
            if !is_primary_cross_listing(self, i) {
                self.sections.push(SolverSection {
                    placement: None,
                    tickets: 0,
                    score: SectionScore::new(),
                    room_times: Vec::new(),
                    instructors: Vec::new(),
                    hard_conflicts: Vec::new(),
                    soft_conflicts_list: Vec::new(),
                    neighbors: Vec::new(),
                    score_criteria: Vec::new(),
                    speculative_deltas: Vec::new(),
                    speculative_delta_min: None,
                    is_secondary_cross_listing: true,
                });
                continue;
            }

            // compute combined conflicts across cross-listings
            let mut hard_conflicts = Vec::new();
            let mut soft_conflicts = Vec::new();
            let mut score_criteria: Vec<Rc<dyn ScoreCriterion>> = Vec::new();
            let mut cross_listings = self.input_sections[i].cross_listings.clone();
            if cross_listings.is_empty() {
                cross_listings.push(i);
            }
            for &self_cross_listing in &cross_listings {
                for &other in &self.input_sections[self_cross_listing].hard_conflicts {
                    hard_conflicts.push(get_primary_cross_listing(&self, other));
                }
                for &SectionWithPenalty { section, penalty } in
                    &self.input_sections[self_cross_listing].soft_conflicts
                {
                    soft_conflicts.push(SectionWithPenalty {
                        section: get_primary_cross_listing(&self, section),
                        penalty,
                    });
                }
            }
            hard_conflicts.sort();
            hard_conflicts.dedup();

            // sort highest penalty first...
            soft_conflicts.sort_by_key(|elt| (elt.section, -elt.penalty));
            // ... so dedup will remove the lower penalty instances
            soft_conflicts.dedup_by_key(|elt| elt.section);

            let soft_conflicts_list = soft_conflicts.clone();
            if !soft_conflicts.is_empty() {
                score_criteria.push(Rc::new(SoftConflictCriterion {
                    sections_with_penalties: soft_conflicts,
                }));
            }

            // get all instructors across all cross-listings
            let mut instructors = Vec::new();
            for &cross_listing in &cross_listings {
                for &instructor in &self.input_sections[cross_listing].instructors {
                    instructors.push(instructor);
                }
            }
            instructors.sort();
            instructors.dedup();

            // compute the combined room/time pairs and penalties
            // considering cross-listings and instructor availability
            let mut room_times = Vec::new();
            let section = &self.input_sections[i];
            'time_slot: for &TimeWithPenalty {
                time_slot,
                penalty: mut time_slot_penalty,
            } in &section.time_slots
            {
                // make sure this time slot is acceptable to all cross-listings
                // and find the max penalty
                for &cross_listing in &section.cross_listings {
                    match self.input_sections[cross_listing]
                        .time_slots
                        .iter()
                        .find_map(|elt| {
                            if elt.time_slot == time_slot {
                                Some(elt.penalty)
                            } else {
                                None
                            }
                        }) {
                        Some(pen) => time_slot_penalty = std::cmp::max(pen, time_slot_penalty),
                        None => continue 'time_slot,
                    };
                }

                // every instructor must be available during the entire time slot
                let mut instructor_penalty = 0;
                for &instructor in &instructors {
                    match self.instructors[instructor]
                        .get_time_slot_penalty(&self.time_slots[time_slot])
                    {
                        Some(pen) => instructor_penalty = std::cmp::max(pen, instructor_penalty),
                        None => continue 'time_slot,
                    }
                }

                // cross this with every room
                'room: for &RoomWithPenalty {
                    room,
                    penalty: mut room_penalty,
                } in &section.rooms
                {
                    // make sure this room is acceptable to all cross-listings
                    // and fine the max penalty
                    for &cross_listing in &section.cross_listings {
                        match self.input_sections[cross_listing].rooms.iter().find_map(|elt| {
                            if elt.room == room {
                                Some(elt.penalty)
                            } else {
                                None
                            }
                        }) {
                            Some(pen) => room_penalty = std::cmp::max(pen, room_penalty),
                            None => continue 'room,
                        };
                    }

                    room_times.push(RoomTimeWithPenalty {
                        room,
                        time_slot,
                        penalty: std::cmp::min(
                            99,
                            time_slot_penalty + room_penalty + instructor_penalty,
                        ),
                    });
                }
            }

            // the cross-listings and instructors have to agree on at least one room and time
            if room_times.is_empty() {
                return Err(format!(
                    "section {} has no room+time combinations that work across all cross listings and instructors",
                    self.input_sections[i].get_name()
                ));
            }
            room_times.sort_by_key(|elt| (elt.room, elt.time_slot, elt.penalty));

            let score = SectionScore::new_unplaced(self.sections.len());
            self.unplaced_current += 1;
            self.unplaced_best += 1;
            self.score += score.global;

            let deltas = vec![None; room_times.len()];

            self.sections.push(SolverSection {
                placement: None,
                tickets: 0,
                score,
                room_times,
                instructors,
                hard_conflicts,
                soft_conflicts_list,
                neighbors: Vec::new(),
                score_criteria,
                speculative_deltas: deltas,
                speculative_delta_min: None,
                is_secondary_cross_listing: false,
            });
        }

        // build and place anticonflict rules
        for (penalty, single, group) in &self.anticonflicts {
            // use primary cross listings
            let single_primary = get_primary_cross_listing(&self, *single);
            let mut group_primaries: Vec<usize> = group
                .iter()
                .map(|&elt| get_primary_cross_listing(&self, elt))
                .collect();
            group_primaries.sort();
            group_primaries.dedup();
            if group_primaries.contains(&single_primary) {
                return Err(format!(
                    "section {} cannot be an anticonflict with itself",
                    self.input_sections[single_primary].get_name()
                ));
            }
            let criterion = Rc::new(AntiConflictCriterion {
                penalty: *penalty,
                single: single_primary,
                group: group_primaries.clone(),
            });

            self.sections[single_primary].score_criteria.push(criterion.clone());
            for &elt in &group_primaries {
                self.sections[elt].score_criteria.push(criterion.clone());
            }
        }

        // collect and place instructor distribution rules
        for instructor in 0..self.instructors.len() {
            if self.instructors[instructor].distribution.is_empty() {
                continue;
            }

            let mut sec_primaries: Vec<usize> = self.instructors[instructor]
                .sections
                .iter()
                .map(|&elt| get_primary_cross_listing(&self, elt))
                .collect();
            sec_primaries.sort();
            sec_primaries.dedup();

            let mut groups = std::collections::HashMap::<u8, Vec<DistributionPreference>>::new();
            for dist in &self.instructors[instructor].distribution {
                let days = match dist {
                    DistributionPreference::Clustering { days, .. } => days,
                    DistributionPreference::DaysOff { days, .. } => days,
                    DistributionPreference::DaysEvenlySpread { days, .. } => days,
                };

                let mut key = 0u8;
                for &day in days {
                    match day {
                        time::Weekday::Sunday => key |= 0b1000000,
                        time::Weekday::Monday => key |= 0b0100000,
                        time::Weekday::Tuesday => key |= 0b0010000,
                        time::Weekday::Wednesday => key |= 0b0001000,
                        time::Weekday::Thursday => key |= 0b0000100,
                        time::Weekday::Friday => key |= 0b0000010,
                        time::Weekday::Saturday => key |= 0b0000001,
                    }
                }
                groups.entry(key).or_default().push(dist.clone());
            }
            let mut grouped_by_days = Vec::new();
            for (_, group) in groups.drain() {
                grouped_by_days.push(group);
            }

            let ics = Rc::new(InstructorClassSpreadCriterion {
                instructor,
                sections: sec_primaries.clone(),
                grouped_by_days,
            });

            for &section in &sec_primaries {
                self.sections[section].score_criteria.push(ics.clone());
            }
        }

        // calculate theoretical minimum rooms possible for each instructor
        for instructor in 0..self.instructors.len() {
            let mut sec_primaries: Vec<usize> = self.instructors[instructor]
                .sections
                .iter()
                .map(|&elt| get_primary_cross_listing(&self, elt))
                .collect();
            sec_primaries.sort();
            sec_primaries.dedup();

            // get a list of all possible rooms the instructor could use
            let mut all_possible = Vec::new();
            for &section in &sec_primaries {
                for &RoomTimeWithPenalty { room, .. } in &self.sections[section].room_times {
                    all_possible.push(room);
                }
            }
            all_possible.sort();
            all_possible.dedup();

            // note if the loop ends without finding a solution with
            // fewer than the max number of rooms, it will leave the
            // result at the max number without bothering to prove it
            let mut k = 1;
            'min_rooms_loop: while k < sec_primaries.len() {
                'set_loop: for room_set in all_possible.iter().combinations(k) {
                    'section_loop: for &section in &sec_primaries {
                        // is this section satisfied by one of the rooms in the set?
                        for &room in &room_set {
                            if self.sections[section]
                                .room_times
                                .iter()
                                .any(|elt| elt.room == *room && elt.penalty == 0)
                            {
                                continue 'section_loop;
                            }
                        }
                        continue 'set_loop;
                    }

                    // success!
                    break 'min_rooms_loop;
                }

                k += 1;
            }

            // do not bother if the best we can do is a distinct room per section
            if k > sec_primaries.len() {
                for &sec in &sec_primaries {
                    self.sections[sec]
                        .score_criteria
                        .push(Rc::new(InstructorRoomCountCriterion {
                            instructor,
                            sections: sec_primaries.clone(),
                            desired: k,
                            penalty: 2,
                        }));
                }
            }
        }

        // gather list of neighbors for each section
        for (i, section) in self.sections.iter_mut().enumerate() {
            let mut neighbors = Vec::new();
            for elt in &section.score_criteria {
                neighbors.append(&mut elt.get_neighbors());
            }
            neighbors.retain(|&elt| elt != i);
            neighbors.sort();
            neighbors.dedup();
            section.neighbors = neighbors;
        }

        self.input_locked = true;

        Ok(())
    }

    pub fn is_placed(&self, section_i: usize) -> bool {
        self.sections[section_i].placement.is_some()
    }

    // remove a section from its current room/time placement (if any)
    // remove it from both sections and room_placements
    pub fn remove_placement(&mut self, section: usize, undo: &mut Vec<PlacementEntry>) {
        assert!(!self.sections[section].is_secondary_cross_listing);

        if let Some(RoomTimeWithPenalty {
            room, time_slot, ..
        }) = self.sections[section].placement
        {
            assert!(std::mem::take(&mut self.room_placements[room].time_slot_placements[time_slot]) == Some(section),
            "Solver::remove_placement: placement by section does not match placement by room and time");
            let rtp = std::mem::take(&mut self.sections[section].placement).unwrap();
            undo.push(PlacementEntry::Remove(section, rtp));
            self.unplaced_current += 1;
        }
    }

    pub fn add_placement_without_displacing(
        &mut self,
        section: usize,
        room_time: &RoomTimeWithPenalty,
        undo: &mut Vec<PlacementEntry>,
    ) {
        assert!(!self.sections[section].is_secondary_cross_listing);

        let &RoomTimeWithPenalty {
            room, time_slot, ..
        } = room_time;

        let old_by_section = std::mem::replace(
            &mut self.sections[section].placement,
            Some(room_time.clone()),
        );
        assert!(old_by_section.is_none());

        let old_by_room_time = std::mem::replace(
            &mut self.room_placements[room].time_slot_placements[time_slot],
            Some(section),
        );
        assert!(old_by_room_time.is_none());

        self.unplaced_current -= 1;

        undo.push(PlacementEntry::Add(section));
    }

    // remove any sections that will be in conflict with a section about to be placed
    //
    // this includes:
    // * anything in the same room in an overlapping time slot
    // * anything in the hard conflict list of this section (or a cross listing)
    //   in the same/an overlapping time slot
    pub fn displace_conflicts(
        &mut self,
        section: usize,
        room_time: &RoomTimeWithPenalty,
        undo: &mut Vec<PlacementEntry>,
    ) {
        assert!(!self.sections[section].is_secondary_cross_listing);

        // is this slot (or an overlapping time in the same room) already occupied?
        let mut evictees = Vec::new();
        for overlapping in &self.time_slots[room_time.time_slot].conflicts {
            if let Some(existing) =
                self.room_placements[room_time.room].time_slot_placements[*overlapping]
            {
                evictees.push(existing);
            }
        }

        // find any hard conflicts in overlapping time slots
        for &hard_conflict in &self.sections[section].hard_conflicts {
            if let Some(RoomTimeWithPenalty { time_slot, .. }) =
                self.sections[hard_conflict].placement
            {
                if time_slots_conflict(self, room_time.time_slot, time_slot) {
                    evictees.push(hard_conflict);
                }
            }
        }

        for elt in evictees {
            self.remove_placement(elt, undo);
        }
    }

    pub fn compute_speculative_deltas(&mut self) {
        let old_score = self.score;
        for section_i in 0..self.sections.len() {
            if self.sections[section_i].is_secondary_cross_listing {
                continue;
            }

            // only move sections that have at least some potential for improvement
            if self.sections[section_i].score.local == 0 {
                self.sections[section_i].speculative_delta_min = None;
                continue;
            }
            self.compute_speculative_deltas_section(section_i, old_score);
        }
    }

    pub fn compute_speculative_deltas_section(
        &mut self,
        section: usize,
        old_score: isize,
    ) {
        assert!(!self.sections[section].is_secondary_cross_listing);

        let current = match self.sections[section].placement {
            Some(RoomTimeWithPenalty {
                room, time_slot, ..
            }) => (room, time_slot),
            None => (usize::MAX, usize::MAX),
        };
        let mut low = None;
        for rtp_i in 0..self.sections[section].room_times.len() {
            let rtp = self.sections[section].room_times[rtp_i].clone();

            // there is no point in "moving" a section to its current placement
            if (rtp.room, rtp.time_slot) == current {
                self.sections[section].speculative_deltas[rtp_i] = None;
                continue;
            }

            let mut undo = PlacementLog::move_section(self, section, rtp);

            // see how it affected the score
            let delta = self.score - old_score;
            self.sections[section].speculative_deltas[rtp_i] = Some(delta);

            low = match low {
                Some(n) => Some(std::cmp::min(n, delta)),
                None => Some(delta),
            };

            // undo the changes
            undo.revert_move(self);
        }
        self.sections[section].speculative_delta_min = low;
    }

    pub fn select_section_to_place(&mut self) -> usize {
        self.compute_speculative_deltas();
        let mut pool_size = 0;

        // find the move that will improve the score the most
        for section in self.sections.iter_mut() {
            if section.is_secondary_cross_listing {
                assert!(section.tickets == 0);
                continue;
            }
            match section.speculative_delta_min {
                Some(delta) => {
                    // ignore MIN_LOTTERY_TICKETS_FOR_UNPLACED_SECTION
                    // if a move results in an additional placed section
                    // it will already show a 1000 point improvement, whereas
                    // using MIN_LOTTERY_TICKETS_FOR_UNPLACED_SECTION here would give
                    // the same boost to moves that do not actually increase the
                    // number of placed sections
                    section.tickets = std::cmp::max(0, -delta);
                    pool_size += section.tickets;
                }
                None => {
                    section.tickets = 0;
                }
            };
        }

        // if no move will make an improvement, use section scores instead
        // (favoring sections with bad scores and thus more potential to improve)
        if pool_size == 0 {
            for section in &mut self.sections.iter_mut() {
                if section.is_secondary_cross_listing {
                    assert!(section.tickets == 0);
                    continue;
                }

                // give everyone at least one ticket, sections with
                // bad scores get more
                section.tickets = std::cmp::max(1, section.score.local + 1);
                if section.placement.is_none() {
                    if self.unplaced_current > self.unplaced_best {
                        // favor unplaced sections, but only when we have seen more placements
                        // in the past (so we don't obsess over cycles of mutually-unplacable
                        // sections)
                        section.tickets = std::cmp::max(
                            section.tickets,
                            MIN_LOTTERY_TICKETS_FOR_UNPLACED_SECTION,
                        );
                    }
                } else if section.room_times.len() == 1 {
                    // if it is already placed and there is only one placement possible,
                    // then placing it again would be a no-op
                    section.tickets = 0;
                }
                pool_size += section.tickets;
            }
        }
        assert!(pool_size > 0);

        // pick a winner
        let mut winner = rand::thread_rng().gen_range(0..pool_size);

        // find the winner
        for (i, elt) in self.sections.iter().enumerate() {
            if winner < elt.tickets {
                return i;
            }
            winner -= elt.tickets;
        }
        panic!("cannot get here");
    }

    pub fn select_room_time_to_place(&self, section: usize) -> RoomTimeWithPenalty {
        assert!(!self.sections[section].is_secondary_cross_listing);

        let room_times = &self.sections[section].room_times;

        if room_times.len() == 1 {
            // special case: only one choice
            return room_times[0].clone();
        } else if room_times.len() == 2 && self.is_placed(section) {
            // special case: only two choices and one is current, so switch
            let Some(RoomTimeWithPenalty {
                room, time_slot, ..
            }) = self.sections[section].placement
            else {
                panic!("cannot happen");
            };
            let other = if room == room_times[0].room && time_slot == room_times[0].time_slot {
                1
            } else {
                0
            };
            return room_times[other].clone();
        }

        let mut pool_size = 0;
        for elt in &self.sections[section].speculative_deltas {
            pool_size += match elt {
                Some(delta) => std::cmp::max(0, -delta),
                None => 0,
            };
        }
        if pool_size > 0 {
            // pick a winner
            let mut winner = rand::thread_rng().gen_range(0..pool_size);

            // find the winner
            for (i, elt) in self.sections[section].speculative_deltas.iter().enumerate() {
                if let Some(delta) = elt {
                    let tickets = std::cmp::max(0, -delta);
                    if winner < tickets {
                        return self.sections[section].room_times[i].clone();
                    }
                    winner -= tickets;
                }
            }
        } else {
            // no improvements possible so go with random
            let (room, time_slot) = match self.sections[section].placement {
                Some(RoomTimeWithPenalty {
                    room, time_slot, ..
                }) => (room, time_slot),
                None => (usize::MAX, usize::MAX),
            };

            let mut rng = rand::thread_rng();
            let room_times = &self.sections[section].room_times;
            loop {
                let winner = rng.gen_range(0..room_times.len());

                // don't place it back where it already is
                if room_times[winner].room == room && room_times[winner].time_slot == time_slot {
                    continue;
                }
                return room_times[winner].clone();
            }
        }

        panic!("cannot get here");
    }

    pub fn select_section_to_place_neighborhood(&mut self) -> usize {
        let mut pool_size = 0;
        for i in 0..self.sections.len() {
            let section = &self.sections[i];
            if section.is_secondary_cross_listing {
                continue;
            }
            if section.placement.is_some() && section.room_times.len() == 1 {
                // if it is already placed and there is only one placement possible,
                // then placing it again would be a no-op
                self.sections[i].tickets = 0;
                continue;
            }

            let mut score = 1 + section.score.local;
            for &elt in &section.neighbors {
                score += self.sections[elt].score.local;
            }
            self.sections[i].tickets = score;
            pool_size += score;
        }
        assert!(pool_size > 0);

        // pick a winner
        let mut winner = rand::thread_rng().gen_range(0..pool_size);

        // find the winner
        for (i, elt) in self.sections.iter().enumerate() {
            if winner < elt.tickets {
                return i;
            }
            winner -= elt.tickets;
        }
        panic!("cannot get here");
    }

    // compute all scores for a section in its curent placement
    // the section's score is fully update, including local and global
    // totals and the detail log,
    // but the overall solver score is not modified
    pub fn compute_section_score(&mut self, section: usize) {
        assert!(!self.sections[section].is_secondary_cross_listing);

        assert!(self.sections[section].score.local == 0);
        assert!(self.sections[section].score.global == 0);
        assert!(self.sections[section].score.score_records.is_empty());

        let mut records = Vec::new();

        match self.sections[section].placement {
            Some(RoomTimeWithPenalty { penalty, .. }) => {
                // room/time penalty handled as a special case
                // since the penalty is stored as part of the placement record
                if penalty != 0 {
                    records.push(SectionScoreRecord {
                        local: penalty,
                        global: penalty,
                        details: SectionScoreDetails::RoomTimePenalty { section },
                    });
                }

                // loop over the other scoring criteria
                for elt in &self.sections[section].score_criteria {
                    elt.check(self, section, &mut records);
                }
            }
            None => {
                // unplaced sections are a special case
                records.push(SectionScoreRecord {
                    local: PENALTY_FOR_UNPLACED_SECTION,
                    global: PENALTY_FOR_UNPLACED_SECTION,
                    details: SectionScoreDetails::SectionNotPlaced { section },
                });
            }
        };

        // compute the totals and apply to the main score record
        for &SectionScoreRecord { local, global, .. } in &records {
            self.sections[section].score.local += local;
            self.sections[section].score.global += global;
        }
        self.sections[section].score.score_records = records;
    }

    pub fn print_schedule(&self) {
        let no_instructor_msg = "(no instructor)".to_string();
        let mut name_len = no_instructor_msg.len();
        for (section_i, section) in self.input_sections.iter().enumerate() {
            if self.sections[section_i].is_secondary_cross_listing {
                continue;
            }
            if !section.instructors.is_empty() {
                let plus = if section.instructors.len() == 1 { 0 } else { 1 };
                let instructor = section.instructors[0];
                name_len = std::cmp::max(
                    name_len,
                    self.instructors[instructor].name.len() + 1 + plus,
                );
            }
            let plus = if section.cross_listings.is_empty() {
                0
            } else {
                1
            };
            name_len = std::cmp::max(name_len, section.get_name().len() + plus);
        }

        for room in &self.rooms {
            name_len = std::cmp::max(name_len, room.name.len());
        }

        let mut time_len = 0;
        for time_slot in &self.time_slots {
            time_len = std::cmp::max(time_len, time_slot.name.len());
        }

        // print the top row labels
        print!("{:time_len$} ", "");
        for room in &self.rooms {
            print!("  {:^width$} ", room.name, width = name_len);
        }
        println!();

        // loop over time slots
        for (time_slot_i, time_slot) in self.time_slots.iter().enumerate() {
            // top line
            print!("{:time_len$} ", "");
            for _ in 0..self.rooms.len() {
                print!("+-{:-<name_len$}-", "");
            }
            println!("+");

            // instructor line
            print!("{:time_len$} ", time_slot.name);
            for room_i in 0..self.rooms.len() {
                if let Some(section_i) =
                    self.room_placements[room_i].time_slot_placements[time_slot_i]
                {
                    let instructors = &self.input_sections[section_i].instructors;
                    let name = if instructors.is_empty() {
                        &no_instructor_msg
                    } else {
                        &self.instructors[instructors[0]].name
                    };
                    if instructors.len() > 1 {
                        print!("| {:<width$}+ ", name, width = name_len - 1);
                    } else {
                        print!("| {:<width$} ", name, width = name_len);
                    }
                } else {
                    print!("| {:name_len$} ", "");
                }
            }
            println!("|");

            // course line
            print!("{:time_len$} ", "");
            for room_i in 0..self.rooms.len() {
                if let Some(section_i) =
                    self.room_placements[room_i].time_slot_placements[time_slot_i]
                {
                    let section = &self.input_sections[get_primary_cross_listing(self, section_i)];
                    let name = section.get_name();
                    if self.input_sections[section_i].cross_listings.len() > 1 {
                        print!("| {:<width$}+ ", name, width = name_len - 1);
                    } else {
                        print!("| {:<width$} ", name, width = name_len);
                    }
                } else {
                    print!("| {:name_len$} ", "");
                }
            }
            println!("|");
        }

        // bottom line
        print!("{:time_len$} ", "");
        for _ in 0..self.rooms.len() {
            print!("+-{:-<name_len$}-", "");
        }
        println!("+");
    }

    pub fn dump_json(&self) -> String {
        let mut s = String::new();
        let w = &mut s;

        let mut list = Vec::new();
        let gather = |f: &mut dyn FnMut(usize), i: usize| {
            if self.input_sections[i].cross_listings.is_empty() {
                f(i);
            } else {
                for &cross in &self.input_sections[i].cross_listings {
                    f(cross);
                }
            }
        };
        let join = |lst: &mut Vec<String>| -> String {
            lst.sort();
            lst.dedup();
            let s = if lst.is_empty() {
                "".to_string()
            } else {
                format!("\"{}\"", lst.join("\", \""))
            };
            lst.clear();
            s
        };

        write!(w, "window.placement = [").unwrap();
        let mut comma = "";
        for (i, section) in self.sections.iter().enumerate() {
            if section.is_secondary_cross_listing {
                continue;
            }

            writeln!(w, "{comma}\n    {{").unwrap();
            comma = ",";

            // names
            gather(&mut |i| list.push(self.input_sections[i].get_name()), i);
            writeln!(w, "        \"names\": [{}],", join(&mut list)).unwrap();

            // prefixes
            gather(&mut |i| list.push(self.input_sections[i].prefix.clone()), i);
            writeln!(w, "        \"prefixes\": [{}],", join(&mut list)).unwrap();

            // instuctors
            gather(
                &mut |i| {
                    for &elt in &self.input_sections[i].instructors {
                        list.push(self.instructors[elt].name.clone());
                    }
                },
                i,
            );
            writeln!(w, "        \"instructors\": [{}],", join(&mut list)).unwrap();
            if let Some(RoomTimeWithPenalty {
                room, time_slot, ..
            }) = section.placement
            {
                writeln!(w, "        \"is_placed\": true,").unwrap();
                writeln!(w, "        \"room\": \"{}\",", self.rooms[room].name).unwrap();
                writeln!(
                    w,
                    "        \"time_slot\": \"{}\",",
                    self.time_slots[time_slot].name
                )
                .unwrap();
            } else {
                writeln!(w, "        \"is_placed\": false,").unwrap();
            }
            let mut problems = Vec::new();
            section
                .score
                .gather_score_messages(self, &mut problems, true);
            if problems.is_empty() {
                writeln!(w, "        \"problems\": []").unwrap();
            } else {
                write!(w, "        \"problems\": [").unwrap();
                let mut c = "";
                for (score, msg) in &problems {
                    write!(
                        w,
                        "{}\n            {{ \"score\": {}, \"message\": \"{}\" }}",
                        c, score, msg
                    )
                    .unwrap();
                    c = ",";
                }
                writeln!(w, "\n        ]").unwrap();
            }
            write!(w, "    }}").unwrap();
        }
        writeln!(w, "\n];").unwrap();
        s
    }
}

pub struct EvictionTracker(
    std::collections::HashMap<usize, std::collections::HashMap<usize, isize>>,
);

#[allow(clippy::new_without_default)]
impl EvictionTracker {
    pub fn new() -> Self {
        EvictionTracker(std::collections::HashMap::new())
    }

    pub fn add_eviction(&mut self, placed: usize, displaced: usize) {
        let bullies = self.0.entry(displaced).or_default();
        bullies.entry(placed).and_modify(|n| *n += 1).or_insert(1);
    }

    pub fn get_top_evictors(&self, displaced: usize, max_count: usize) -> Vec<(usize, isize)> {
        let mut lst = Vec::new();
        if let Some(bullies) = self.0.get(&displaced) {
            for (&placed, &count) in bullies.iter() {
                lst.push((placed, count));
            }
            lst.sort_by_key(|&(s, c)| (-c, s));
            lst.truncate(max_count);
        }
        lst
    }
}

pub fn solve(solver: &mut Solver, iterations: usize) {
    let mut evicted_by = EvictionTracker::new();
    let mut winner;
    let start = time::Instant::now();
    let mut best_score = solver.score + 1;
    println!("initial score = {}", solver.score);
    let mut initial = true;
    let mut pause = false;

    for iteration in 0..iterations {
        if pause {
            println!("score is currently {}", solver.score);
        }
        let section = solver.select_section_to_place();
        if pause {
            println!("picked section {}: {}", section, solver.input_sections[section].get_name());
        }
        let room_time = solver.select_room_time_to_place(section);
        if pause {
            println!("picked {} at {} penalty {}",
                solver.rooms[room_time.room].name,
                solver.time_slots[room_time.time_slot].name,
                room_time.penalty);
        }
        let undo = PlacementLog::move_section(solver, section, room_time);
        if pause && undo.entries.len() > 1 {
            for elt in &undo.entries {
                match elt {
                    PlacementEntry::Add(_) => (),
                    PlacementEntry::Remove(displaced, rtp) => {
                        println!("--> displaced {}: {} from {} at {} penalty {}",
                            *displaced,
                            solver.input_sections[*displaced].get_name(),
                            solver.rooms[rtp.room].name,
                            solver.time_slots[rtp.time_slot].name,
                            rtp.penalty);
                    },
                }
            }
        }
        solver.unplaced_best = std::cmp::min(solver.unplaced_best, solver.unplaced_current);
        for elt in &undo.entries {
            if let &PlacementEntry::Remove(loser, _) = elt {
                evicted_by.add_eviction(section, loser);
            }
        }
        let score = solver.score;
        if score < best_score {
            pause = false;
            best_score = score;
            winner = solver.clone();

            if winner.unplaced_current < 5 || initial {
                let mut problems = Vec::new();
                for i in 0..winner.sections.len() {
                    winner.sections[i].score.gather_score_messages(
                        &winner,
                        &mut problems,
                        false,
                    );
                }
                problems.sort_by_key(|(score, _)| -score);

                println!();
                println!();
                //winner.print_schedule();
                let filename = if initial { "static.js" } else { "placement.js" };
                fs::write(filename, winner.dump_json())
                    .expect("unable to write placements.js");

                if !problems.is_empty() {
                    let digits = problems[0].0.to_string().len();
                    for (score, message) in &problems {
                        if *score == PENALTY_FOR_UNPLACED_SECTION {
                            continue;
                        }
                        println!("[{:width$}]  {}", score, message, width = digits);
                    }
                    for (i, section) in winner.sections.iter().enumerate() {
                        if section.is_secondary_cross_listing || section.placement.is_some() {
                            continue;
                        }
                        print!("unplaced: {}", solver.input_sections[i].get_name());

                        // report who displaces this section the most
                        let lst = evicted_by.get_top_evictors(i, 5);
                        if !lst.is_empty() {
                            print!(" displaced by");
                            for (sec, count) in lst {
                                print!(" {}×{}", solver.input_sections[sec].get_name(), count);
                            }
                        }
                        println!();
                    }
                }
                initial = false;
            }

            let elapsed = start.elapsed();
            let rate = (iteration as f64) / elapsed.as_seconds_f64();
            println!(
                "score = {} with {} unplaced sections, solving at a rate of {}/second",
                score, winner.unplaced_current, rate as i64
            );
        }
    }
}
