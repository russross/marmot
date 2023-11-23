use super::input::*;
use super::score::*;
use rand::Rng;

const PENALTY_FOR_UNPLACED_SECTION: isize = 1000;
const MIN_LOTTERY_TICKETS_FOR_UNPLACED_SECTION: isize = 1000;

#[derive(Clone)]
pub struct Solver {
    pub sections: Vec<SolverSection>,
    pub room_placements: Vec<RoomPlacements>,
    pub score: isize,
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
    pub score_criteria: Vec<ScoreCriterion>,

    // scoring info for the current placement
    pub score: SectionScore,

    // these lists combine the relevant data across all cross-listings
    // and all instructors and account for sections that are pinned in place
    // i.e., the list of room/times does not include any entries that would
    // lead to a pinned class being displaced
    pub room_times: Vec<RoomTimeWithPenalty>,
    pub instructors: Vec<usize>,
    pub hard_conflicts: Vec<usize>,

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
        input: &Input,
        list: &mut Vec<(isize, String)>,
    ) {
        for record in &self.score_records {
            record.gather_score_messages(solver, input, list);
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
        input: &Input,
        section: usize,
        room_time: RoomTimeWithPenalty,
    ) -> Self {
        let mut entries = Vec::new();

        // move the section and record displacements
        solver.remove_placement(section, &mut entries);
        solver.displace_conflicts(input, section, &room_time, &mut entries);
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
            solver.sections[section]
                .score
                .gather_adjacent_sections(&mut adjacent, &sections_being_moved);

            // undo the old score on section being moved
            solver.score -= solver.sections[section].score.global;

            // move the old score records to the log and reset the section score
            let elt = std::mem::replace(&mut solver.sections[section].score, SectionScore::new());
            pre_scores.push((section, elt));

            // compute the new score
            solver.compute_section_score(input, section);

            // apply it to the global score
            solver.score += solver.sections[section].score.global;

            // gather adjacent sections based on the new scoring
            solver.sections[section]
                .score
                .gather_adjacent_sections(&mut adjacent, &sections_being_moved);
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
            solver.compute_section_score(input, section);

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

impl Solver {
    pub fn new(input: &Input) -> Result<Self, String> {
        let mut room_placements = Vec::with_capacity(input.rooms.len());
        for _ in 0..input.rooms.len() {
            room_placements.push(RoomPlacements {
                time_slot_placements: vec![None; input.time_slots.len()],
            });
        }
        let mut sections = Vec::new();
        let mut total_score = 0;
        for i in 0..input.sections.len() {
            // non-primary cross listing sections are ignored
            if !input.is_primary_cross_listing(i) {
                sections.push(SolverSection {
                    placement: None,
                    tickets: 0,
                    score: SectionScore::new(),
                    room_times: Vec::new(),
                    instructors: Vec::new(),
                    hard_conflicts: Vec::new(),
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
            let mut score_criteria = Vec::new();
            let mut cross_listings = input.sections[i].cross_listings.clone();
            if cross_listings.is_empty() {
                cross_listings.push(i);
            }
            for &self_cross_listing in &cross_listings {
                for &other in &input.sections[self_cross_listing].hard_conflicts {
                    hard_conflicts.push(input.get_primary_cross_listing(other));
                }
                for &SectionWithPenalty { section, penalty } in
                    &input.sections[self_cross_listing].soft_conflicts
                {
                    soft_conflicts.push(SectionWithPenalty {
                        section: input.get_primary_cross_listing(section),
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

            if !soft_conflicts.is_empty() {
                score_criteria.push(ScoreCriterion::SoftConflict {
                    sections_with_penalties: soft_conflicts,
                });
            }

            // compute the combined room/time pairs and penalties across cross-listings
            let room_times = if cross_listings.len() == 1 {
                // only a single section, so keep the original input data
                input.sections[i].room_times.clone()
            } else {
                // this is the intersection of room/time availability with max penalty
                let mut rtp = Vec::new();
                'a: for &RoomTimeWithPenalty {
                    room, time_slot, ..
                } in &input.sections[i].room_times
                {
                    // every cross-listing must have this slot
                    let mut worst_penalty = 0;
                    for &section_i in &cross_listings {
                        match input.sections[section_i]
                            .room_times
                            .iter()
                            .find(|elt| elt.room == room && elt.time_slot == time_slot)
                        {
                            Some(RoomTimeWithPenalty { penalty, .. }) => {
                                worst_penalty = std::cmp::max(worst_penalty, *penalty)
                            }
                            None => continue 'a,
                        }
                    }
                    rtp.push(RoomTimeWithPenalty {
                        room,
                        time_slot,
                        penalty: worst_penalty,
                    });
                }

                // the cross-listed sections have to agree on at least one room and time
                if rtp.is_empty() {
                    return Err(format!(
                        "cross-listing that includes {}-{} has no viable room+time combination",
                        input.sections[i].course, input.sections[i].section
                    ));
                }
                rtp.sort_by_key(|elt| (elt.room, elt.time_slot, elt.penalty));
                rtp
            };

            // get all instructors across all cross-listings
            let mut instructors = Vec::new();
            for &cross_listing in &cross_listings {
                for &instructor in &input.sections[cross_listing].instructors {
                    instructors.push(instructor);
                }
            }
            instructors.sort();
            instructors.dedup();

            let score = SectionScore::new_unplaced(sections.len());
            total_score += score.global;

            sections.push(SolverSection {
                placement: None,
                tickets: 0,
                score,
                room_times,
                instructors,
                hard_conflicts,
                score_criteria,
                speculative_deltas: vec![None; input.sections[i].room_times.len()],
                speculative_delta_min: None,
                is_secondary_cross_listing: false,
            });
        }

        // build and place anticonflict rules
        for (penalty, single, group) in &input.anticonflicts {
            // use primary cross listings
            let single_primary = input.get_primary_cross_listing(*single);
            let mut group_primaries: Vec<usize> = group
                .iter()
                .map(|&elt| input.get_primary_cross_listing(elt))
                .collect();
            group_primaries.sort();
            group_primaries.dedup();
            if group_primaries.contains(&single_primary) {
                return Err(format!(
                    "section {}-{} cannot be an anticonflict with itself",
                    input.sections[single_primary].course, input.sections[single_primary].section
                ));
            }
            let criterion = ScoreCriterion::AntiConflict {
                penalty: *penalty,
                single: single_primary,
                group: group_primaries.clone(),
            };

            sections[single_primary]
                .score_criteria
                .push(criterion.clone());
            for &elt in &group_primaries {
                sections[elt].score_criteria.push(criterion.clone());
            }
        }

        Ok(Solver {
            room_placements,
            sections,
            score: total_score,
        })
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
        input: &Input,
        section: usize,
        room_time: &RoomTimeWithPenalty,
        undo: &mut Vec<PlacementEntry>,
    ) {
        assert!(!self.sections[section].is_secondary_cross_listing);

        // is this slot (or an overlapping time in the same room) already occupied?
        let mut evictees = Vec::new();
        for overlapping in &input.time_slots[room_time.time_slot].conflicts {
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
                if input.time_slots_conflict(room_time.time_slot, time_slot) {
                    evictees.push(hard_conflict);
                }
            }
        }

        for elt in evictees {
            self.remove_placement(elt, undo);
        }
    }

    pub fn compute_speculative_deltas(&mut self, input: &Input) {
        let old_score = self.score;
        for section_i in 0..self.sections.len() {
            if self.sections[section_i].is_secondary_cross_listing {
                continue;
            }
            self.compute_speculative_deltas_section(input, section_i, old_score);
        }
    }

    pub fn compute_speculative_deltas_section(
        &mut self,
        input: &Input,
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

            let mut undo = PlacementLog::move_section(self, input, section, rtp);

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

    pub fn select_section_to_place_slow(&mut self, input: &Input) -> usize {
        let mut rng = rand::thread_rng();

        self.compute_speculative_deltas(input);
        let mut pool_size = 0;
        for section in self.sections.iter_mut() {
            if section.is_secondary_cross_listing {
                section.tickets = 0;
                continue;
            }
            match section.speculative_delta_min {
                Some(delta) => {
                    section.tickets = std::cmp::max(1, -delta + 1);
                    if section.placement.is_none() {
                        section.tickets = std::cmp::min(
                            section.tickets,
                            MIN_LOTTERY_TICKETS_FOR_UNPLACED_SECTION,
                        );
                    }
                    pool_size += section.tickets;
                }
                None => {
                    section.tickets = 0;
                }
            };
        }
        assert!(pool_size > 0);

        // pick a winner
        let mut winner = rng.gen_range(0..pool_size);

        // find the winner
        for (i, elt) in self.sections.iter().enumerate() {
            if winner < elt.tickets {
                return i;
            }
            winner -= elt.tickets;
        }
        panic!("cannot get here");
    }

    pub fn select_room_time_to_place_slow(
        &self,
        _input: &Input,
        section: usize,
    ) -> RoomTimeWithPenalty {
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

        let mut rng = rand::thread_rng();
        let mut pool_size = 0;
        for elt in &self.sections[section].speculative_deltas {
            pool_size += match elt {
                Some(delta) => std::cmp::max(1, -delta + 1),
                None => 0,
            };
        }
        assert!(pool_size > 0);

        // pick a winner
        let mut winner = rng.gen_range(0..pool_size);

        // find the winner
        for (i, elt) in self.sections[section].speculative_deltas.iter().enumerate() {
            if let Some(delta) = elt {
                let tickets = std::cmp::max(1, -delta + 1);
                if winner < tickets {
                    return self.sections[section].room_times[i].clone();
                }
                winner -= tickets;
            }
        }
        panic!("cannot get here");
    }

    pub fn select_section_to_place_fast(&mut self, input: &Input) -> usize {
        let mut rng = rand::thread_rng();

        let mut pool_size = 0;
        for (i, section) in self.sections.iter_mut().enumerate() {
            if section.is_secondary_cross_listing {
                continue;
            }
            section.tickets = std::cmp::max(1, section.score.local + 1);
            if section.placement.is_none() {
                section.tickets =
                    std::cmp::max(section.tickets, MIN_LOTTERY_TICKETS_FOR_UNPLACED_SECTION);
            } else if input.sections[i].room_times.len() == 1 {
                // if it is already placed and there is only one placement possible,
                // then placing it again would be a no-op
                section.tickets = 0;
            }
            pool_size += section.tickets;
        }
        assert!(pool_size > 0);

        // pick a winner
        let mut winner = rng.gen_range(0..pool_size);

        // find the winner
        for (i, elt) in self.sections.iter().enumerate() {
            if winner < elt.tickets {
                return i;
            }
            winner -= elt.tickets;
        }
        panic!("cannot get here");
    }

    pub fn select_room_time_to_place_fast(
        &mut self,
        input: &Input,
        section: usize,
    ) -> RoomTimeWithPenalty {
        assert!(!self.sections[section].is_secondary_cross_listing);

        let is_placed = self.is_placed(section);
        let choices = self.sections[section].room_times.len();
        if !is_placed && choices > 1 || is_placed && choices > 2 {
            self.compute_speculative_deltas_section(input, section, self.score);
        }
        self.select_room_time_to_place_slow(input, section)
    }

    pub fn select_room_time_to_place_random(
        &mut self,
        _input: &Input,
        section: usize,
    ) -> RoomTimeWithPenalty {
        assert!(!self.sections[section].is_secondary_cross_listing);

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

    // compute all scores for a section in its curent placement
    // the section's score is fully update, including local and global
    // totals and the detail log,
    // but the overall solver score is not modified
    pub fn compute_section_score(&mut self, input: &Input, section: usize) {
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
                    elt.check(self, input, section, &mut records);
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

    pub fn print_schedule(&self, input: &Input) {
        let no_instructor_msg = "(no instructor)".to_string();
        let mut name_len = no_instructor_msg.len();
        for (section_i, section) in input.sections.iter().enumerate() {
            if self.sections[section_i].is_secondary_cross_listing {
                continue;
            }
            if !section.instructors.is_empty() {
                let plus = if section.instructors.len() == 1 { 0 } else { 1 };
                let instructor = section.instructors[0];
                name_len = std::cmp::max(
                    name_len,
                    input.instructors[instructor].name.len() + 1 + plus,
                );
            }
            let plus = if section.cross_listings.is_empty() {
                0
            } else {
                1
            };
            name_len = std::cmp::max(
                name_len,
                section.course.len() + section.section.len() + 1 + plus,
            );
        }

        for room in &input.rooms {
            name_len = std::cmp::max(name_len, room.name.len());
        }

        let mut time_len = 0;
        for time_slot in &input.time_slots {
            time_len = std::cmp::max(time_len, time_slot.name.len());
        }

        // print the top row labels
        print!("{:time_len$} ", "");
        for room in &input.rooms {
            print!("  {:^width$} ", room.name, width = name_len);
        }
        println!();

        // loop over time slots
        for (time_slot_i, time_slot) in input.time_slots.iter().enumerate() {
            // top line
            print!("{:time_len$} ", "");
            for _ in 0..input.rooms.len() {
                print!("+-{:-<name_len$}-", "");
            }
            println!("+");

            // instructor line
            print!("{:time_len$} ", time_slot.name);
            for room_i in 0..input.rooms.len() {
                if let Some(section_i) =
                    self.room_placements[room_i].time_slot_placements[time_slot_i]
                {
                    let instructors = &input.sections[section_i].instructors;
                    let name = if instructors.is_empty() {
                        &no_instructor_msg
                    } else {
                        &input.instructors[instructors[0]].name
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
            for room_i in 0..input.rooms.len() {
                if let Some(section_i) =
                    self.room_placements[room_i].time_slot_placements[time_slot_i]
                {
                    let section = &input.sections[input.get_primary_cross_listing(section_i)];
                    let name = format!("{}-{}", section.course, section.section);
                    if input.sections[section_i].cross_listings.len() > 1 {
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
        for _ in 0..input.rooms.len() {
            print!("+-{:-<name_len$}-", "");
        }
        println!("+");
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

pub fn solve(mut solver: Solver, input: &Input, iterations: usize) {
    let mut evicted_by = EvictionTracker::new();
    let mut winner = solver.clone();
    let start = time::Instant::now();
    let mut best_score = solver.score;
    println!("initial score = {}", solver.score);

    for iteration in 0..iterations {
        let section = solver.select_section_to_place_fast(input);
        let room_time = solver.select_room_time_to_place_random(input, section);
        let undo = PlacementLog::move_section(&mut solver, input, section, room_time);
        for elt in &undo.entries {
            if let &PlacementEntry::Remove(loser, _) = elt {
                evicted_by.add_eviction(section, loser);
            }
        }
        let score = solver.score;
        if score < best_score
        /*|| iterations == 2*solver.sections.len()*/
        {
            if score < best_score {
                best_score = score;
                winner = solver.clone();
                /*
                if iteration < 2 * solver.sections.len() {
                    continue
                }
                */
            }

            println!();
            println!();
            //winner.print_schedule(input);
            println!("score = {}", score);
            let mut problems = Vec::new();
            for i in 0..winner.sections.len() {
                winner.sections[i]
                    .score
                    .gather_score_messages(&winner, input, &mut problems);
            }
            problems.sort_by_key(|(score, _)| -score);

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
                    print!(
                        "unplaced: {}-{}",
                        input.sections[i].course, input.sections[i].section
                    );

                    // report who displaces this section the most
                    let lst = evicted_by.get_top_evictors(i, 5);
                    if !lst.is_empty() {
                        print!(" displaced by");
                        for (sec, count) in lst {
                            print!(
                                " {}-{}×{}",
                                input.sections[sec].course, input.sections[sec].section, count
                            );
                        }
                    }
                    println!();
                }
            }

            let elapsed = start.elapsed();
            let rate = (iteration as f64) / elapsed.as_seconds_f64();
            println!("solving at a rate of {}/second", rate as i64);
        }
    }
}
