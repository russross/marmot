use super::input::*;
use rand::Rng;

const PENALTY_FOR_UNPLACED_SECTION: isize = 1000;
const MIN_LOTTERY_TICKETS_FOR_UNPLACED_SECTION: isize = 1000;

#[derive(Clone)]
pub struct Solver {
    pub room_placements: Vec<RoomPlacements>,
    pub sections: Vec<SolverSection>,
    pub instructors: Vec<SolverInstructor>,
    pub score: isize,
    pub problems: Vec<Problem>,
}

#[derive(Clone)]
pub struct RoomPlacements {
    pub time_slot_placements: Vec<Option<usize>>,
}

#[derive(Clone)]
pub struct SolverSection {
    // the current room/time assignment for this section
    pub placement: Option<RoomTimeWithPenalty>,

    // all penalty points due to this section's placement
    // e.g., room/time penalties, curriculum conflict penalties,
    // instructor preference penalties
    //
    // any penalty that could change if this section was moved
    // goes into this score. note that some penalties may affect
    // multiple sections (like curriculum conflicts) and they
    // will be counted for all applicable sections. this is useful
    // for estimating the maximum potential benefit to moving this section
    pub penalty: isize,

    // the number of lottery tickets assigned to this section
    // higher means it is more likely to be selected for re-assignment,
    // zero means it will not be moved (but could still be displaced
    // by another section moving)
    pub tickets: isize,

    // these lists combine the relevant data across all cross-listings
    // and all instructors and account for sections that are pinned in place
    // i.e., the list of room/times does not include any entries that would
    // lead to a pinned class being displaced
    pub room_times: Vec<RoomTimeWithPenalty>,
    pub hard_conflicts: Vec<usize>,
    pub soft_conflicts: Vec<SectionWithPenalty>,

    // map from input.section[_].room_times to the effect on the score
    // if we placed this section at that room/time
    pub speculative_deltas: Vec<Option<isize>>,

    // the maximum penalty improvement possible if this section was moved
    // computed as the minimum delta in speculative_deltas
    pub speculative_delta_min: Option<isize>,
}

#[derive(Clone)]
pub struct SolverInstructor {
    // all penalty points due to this instructors placed sections
    // e.g., clusters and gaps in the schedule, distribution across days
    //
    // any penalty that could change if this instructor's schedule changes
    // goes into this score. exceptions include days/time/room preferences
    // for an individual section, since those are accounted for in the section
    // penalty.
    pub penalty: isize,
}

#[derive(Clone)]
pub struct Problem {
    pub penalty: isize,
    pub message: String,
    pub sections: Vec<usize>,
    pub instructors: Vec<usize>,
    pub time_slots: Vec<usize>,
}

pub struct PlacementLog {
    pub entries: Vec<PlacementEntry>,
}

// records the action taken, so
// Place means this class was placed (displacing it will undo)
// Displaced means this class was displaced (displacing it will undo)
pub enum PlacementEntry {
    Diplaced(usize, RoomTimeWithPenalty),
    Placed(usize),
}

impl PlacementLog {
    pub fn new() -> Self {
        PlacementLog {
            entries: Vec::new(),
        }
    }

    pub fn undo(&mut self, solver: &mut Solver, input: &Input) {
        let mut redo = PlacementLog::new();
        let mut deltas = Vec::new();
        loop {
            // play the log in reverse order
            match self.entries.pop() {
                Some(PlacementEntry::Placed(section)) => {
                    solver.remove_placement(input, section, &mut redo, &mut deltas);
                    for elt in deltas.drain(..) {
                        elt.undo(solver);
                    }
                },
                Some(PlacementEntry::Diplaced(section, room_time)) => {
                    solver.add_placement_without_displacing(input, section, &room_time, &mut redo, &mut deltas);
                    for elt in deltas.drain(..) {
                        elt.apply(solver);
                    }
                }
                None => return,
            }
        }
    }
}

pub struct PenaltyDeltas {
    pub deltas: Vec<PenaltyDelta>,
}

impl PenaltyDeltas {
    pub fn new() -> Self {
        PenaltyDeltas{ deltas: Vec::new() }
    }
}

// TODO: re-score adjacent sections, too
pub enum PenaltyDelta {
    CurriculumConflict{ sections: Vec<usize>, delta: isize },
    RoomTimePenalty{ section: usize, delta: isize },
    SectionPlaced{ section: usize, delta: isize },
}

impl PenaltyDelta {
    pub fn apply(&self, solver: &mut Solver) {
        match self {
            PenaltyDelta::CurriculumConflict{ sections, delta } => {
                for &section in sections {
                    solver.sections[section].penalty += *delta;
                }
                solver.score += delta;
            },
            PenaltyDelta::RoomTimePenalty{ section, delta } => {
                solver.sections[*section].penalty += *delta;
                solver.score += *delta;
            },
            PenaltyDelta::SectionPlaced{ delta, .. }  => {
                solver.score += *delta;
            },
        }
    }

    pub fn undo(&self, solver: &mut Solver) {
        match self {
            PenaltyDelta::CurriculumConflict{ sections, delta } => {
                for &section in sections {
                    solver.sections[section].penalty -= *delta;
                }
                solver.score -= delta;
            },
            PenaltyDelta::RoomTimePenalty{ section, delta } => {
                solver.sections[*section].penalty -= *delta;
                solver.score -= *delta;
            },
            PenaltyDelta::SectionPlaced{ delta, .. }  => {
                solver.score -= *delta;
            },
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
        for i in 0..input.sections.len() {
            if !input.is_primary_cross_listing(i) {
                break;
            }

            // compute combined conflicts across cross-listings
            let mut hard_conflicts = Vec::new();
            let mut soft_conflicts = Vec::new();
            for &self_cross_listing in &input.sections[i].cross_listings {
                for &other in &input.sections[self_cross_listing].hard_conflicts {
                    hard_conflicts.push(input.sections[other].get_primary_cross_listing());
                }
                for &SectionWithPenalty { section, penalty } in
                    &input.sections[self_cross_listing].soft_conflicts
                {
                    soft_conflicts.push(SectionWithPenalty {
                        section: input.sections[section].get_primary_cross_listing(),
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

            // compute the combined room/time pairs and penalties across cross-listings
            let room_times = if input.sections[i].cross_listings.len() == 1 {
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
                    for &section_i in &input.sections[i].cross_listings {
                        match input.sections[section_i].room_times.iter().find(|elt| elt.room == room && elt.time_slot == time_slot)
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

            sections.push(SolverSection {
                placement: None,
                penalty: 0,
                room_times,
                hard_conflicts,
                soft_conflicts,
                tickets: 0,
                speculative_deltas: vec![None; input.sections[i].room_times.len()],
                speculative_delta_min: None,
            });
        }
        let mut instructors = Vec::new();
        for _i in 0..input.instructors.len() {
            instructors.push(SolverInstructor{ penalty: 0 })
        }
        Ok(Solver {
            room_placements,
            sections,
            instructors,
            score: 0,
            problems: Vec::new(),
        })
    }

    pub fn is_placed(&self, section_i: usize) -> bool {
        self.sections[section_i].placement.is_some()
    }

    pub fn place_with_displacements(
        &mut self,
        input: &Input,
        section: usize,
        room_time: RoomTimeWithPenalty,
        undo: &mut PlacementLog,
    ) {
        let mut deltas = Vec::new();
        self.remove_placement(input, section, undo, &mut deltas);
        for elt in deltas.drain(..) {
            elt.undo(self);
        }
        self.add_placement_without_displacing(input, section, &room_time, undo, &mut deltas);
        for elt in deltas.drain(..) {
            elt.apply(self);
        }
        self.displace_conflicts(input, section, &room_time, undo, &mut deltas);
        for elt in deltas.drain(..) {
            elt.undo(self);
        }
    }

    // remove a section from its current room/time placement (if any)
    // remove it from both sections and room_placements
    pub fn remove_placement(&mut self, input: &Input, section: usize, undo: &mut PlacementLog, deltas: &mut Vec<PenaltyDelta>) {
        if let Some(RoomTimeWithPenalty{ room, time_slot, .. }) = self.sections[section].placement {
            self.compute_score_section(input, section, false, deltas);
            deltas.push(PenaltyDelta::SectionPlaced{ section, delta: -PENALTY_FOR_UNPLACED_SECTION });
            assert!(std::mem::take(&mut self.room_placements[room].time_slot_placements[time_slot]) == Some(section),
            "Solver::remove_placement: placement by section does not match placement by room and time");
            let rtp = std::mem::take(&mut self.sections[section].placement).unwrap();
            undo.entries.push(PlacementEntry::Diplaced(section, rtp));
        }
    }

    pub fn add_placement_without_displacing(
        &mut self,
        input: &Input,
        section: usize,
        room_time: &RoomTimeWithPenalty,
        undo: &mut PlacementLog,
        deltas: &mut Vec<PenaltyDelta>,
    ) {
        let &RoomTimeWithPenalty { room, time_slot, .. } = room_time;

        let old_by_section =
            std::mem::replace(&mut self.sections[section].placement, Some(room_time.clone()));
        assert!(old_by_section.is_none());

        let old_by_room_time = std::mem::replace(
            &mut self.room_placements[room].time_slot_placements[time_slot],
            Some(section),
        );
        assert!(old_by_room_time.is_none());

        self.compute_score_section(input, section, false, deltas);
        deltas.push(PenaltyDelta::SectionPlaced{ section, delta: -PENALTY_FOR_UNPLACED_SECTION });
        undo.entries.push(PlacementEntry::Placed(section));
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
        undo: &mut PlacementLog,
        deltas: &mut Vec<PenaltyDelta>,
    ) {
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
            if let Some(RoomTimeWithPenalty { time_slot, .. }) = self.sections[hard_conflict].placement {
                if input.time_slots_conflict(room_time.time_slot, time_slot) {
                    evictees.push(hard_conflict);
                }
            }
        }

        for elt in evictees {
            self.remove_placement(input, elt, undo, deltas);
        }
    }

    pub fn compute_speculative_deltas(&mut self, input: &Input) {
        let old_score = self.score;
        for section_i in 0..self.sections.len() {
            self.compute_speculative_deltas_section(input, section_i, old_score);
        }
    }

    pub fn compute_speculative_deltas_section(
        &mut self,
        input: &Input,
        section_i: usize,
        old_score: isize,
    ) {
        let current = match self.sections[section_i].placement {
            Some(RoomTimeWithPenalty { room, time_slot, .. }) => (room, time_slot),
            None => (usize::MAX, usize::MAX),
        };
        let mut low = None;
        for rtp_i in 0..self.sections[section_i].room_times.len() {
            let rtp = self.sections[section_i].room_times[rtp_i].clone();

            // there is no point in "moving" a section to its current placement
            if (rtp.room, rtp.time_slot) == current {
                self.sections[section_i].speculative_deltas[rtp_i] = None;
                continue;
            }

            let mut undo = PlacementLog::new();
            self.place_with_displacements(input, section_i, rtp, &mut undo);

            // see how it affected the score
            //self.compute_score(input, false);
            let delta = self.score - old_score;
            self.sections[section_i].speculative_deltas[rtp_i] = Some(delta);

            low = match low {
                Some(n) => Some(std::cmp::min(n, delta)),
                None => Some(delta),
            };

            // undo the changes
            undo.undo(self, input);
        }
        self.sections[section_i].speculative_delta_min = low;
    }

    pub fn select_section_to_place(&mut self, input: &Input) -> usize {
        let mut rng = rand::thread_rng();

        self.compute_speculative_deltas(&input);
        let mut pool_size = 0;
        for section in self.sections.iter_mut() {
            match section.speculative_delta_min {
                Some(delta) => {
                    section.tickets = std::cmp::max(1, -delta+1);
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

    pub fn select_room_time_to_place(&self, section_i: usize) -> RoomTimeWithPenalty {
        let room_times = &self.sections[section_i].room_times;

        if room_times.len() == 1 {
            // special case: only one choice
            return room_times[0].clone();
        } else if room_times.len() == 2 && self.is_placed(section_i) {
            // special case: only two choices and one is current, so switch
            let Some(RoomTimeWithPenalty{ room, time_slot, .. }) = self.sections[section_i].placement else {
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
        for elt in &self.sections[section_i].speculative_deltas {
            pool_size += match elt {
                Some(delta) => std::cmp::max(1, -delta+1),
                None => 0,
            };
        }
        assert!(pool_size > 0);

        // pick a winner
        let mut winner = rng.gen_range(0..pool_size);

        // find the winner
        for (i, elt) in self.sections[section_i]
            .speculative_deltas
            .iter()
            .enumerate()
        {
            if let Some(delta) = elt {
                let tickets = std::cmp::max(1, -delta+1);
                if winner < tickets {
                    return self.sections[section_i].room_times[i].clone();
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
            section.tickets = std::cmp::max(1, section.penalty+1);
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

    pub fn select_room_time_to_place_fast(&mut self, input: &Input, section_i: usize) -> RoomTimeWithPenalty {
        let is_placed = self.is_placed(section_i);
        let choices = self.sections[section_i].room_times.len();
        if !is_placed && choices > 1 || is_placed && choices > 2 {
            self.compute_speculative_deltas_section(input, section_i, self.score);
        }
        self.select_room_time_to_place(section_i)
    }

    pub fn compute_score(&mut self, input: &Input, gather_problems: bool) {
        // zero out all the scores
        self.score = 0;
        self.problems.clear();
        for section in &mut self.sections.iter_mut() {
            section.penalty = 0;
        }

        // score soft conflicts
        // and add a big penalty for each unplaced section
        let mut deltas = Vec::new();
        for i in 0..self.sections.len() {
            self.compute_score_section(input, i, gather_problems, &mut deltas);
            for elt in deltas.drain(..) {
                elt.apply(self);
            }

            if !self.is_placed(i) {
                self.score += PENALTY_FOR_UNPLACED_SECTION;
            }

        }
    }

    pub fn compute_score_section(&mut self, input: &Input, section: usize, gather_problems: bool, deltas: &mut Vec<PenaltyDelta>) {
        self.compute_score_room_and_time_penalties(input, section, gather_problems, deltas);
        self.compute_score_section_soft_conflicts(input, section, gather_problems, deltas);
    }

    pub fn compute_score_section_soft_conflicts(
        &mut self,
        input: &Input,
        section_i: usize,
        gather_problems: bool,
        deltas: &mut Vec<PenaltyDelta>,
    ) {
        // grab the time slot we are placed in; quit if not placed
        let Some(RoomTimeWithPenalty {
            time_slot: my_time_slot,
            ..
        }) = self.sections[section_i].placement
        else {
            return;
        };

        // look at the conflicts across all cross-listings
        for sp_i in 0..self.sections[section_i].soft_conflicts.len() {
            let &SectionWithPenalty{ section: soft_conflict_section, penalty } = &self.sections[section_i].soft_conflicts[sp_i];

            // we will discover each conflict twice (A conflicts with B and B conflicts with A),
            // so only check when starting with the lower-numbered section
            if section_i >= soft_conflict_section {
                continue;
            }

            // check for placement of the conflicting course
            let Some(RoomTimeWithPenalty {
                time_slot: other_time_slot,
                ..
            }) = self.sections[soft_conflict_section].placement
            else {
                continue;
            };

            // we only care if there is an overlap
            if !input.time_slots_conflict(my_time_slot, other_time_slot) {
                continue;
            }

            // if we make it this far, there is a soft conflict
            deltas.push(PenaltyDelta::CurriculumConflict{
                sections: vec![section_i, soft_conflict_section],
                delta: penalty,
            });

            if !gather_problems {
                continue;
            }

            // build the problem record
            let message = if my_time_slot == other_time_slot {
                format!(
                    "curriculum conflict: {}-{} and {}-{} both meet at {}",
                    input.sections[section_i].course,
                    input.sections[section_i].section,
                    input.sections[soft_conflict_section].course,
                    input.sections[soft_conflict_section].section,
                    input.time_slots[my_time_slot].name
                )
            } else {
                format!(
                    "curriculum conflict: {}-{} at {} overlaps {}-{} at {}",
                    input.sections[section_i].course,
                    input.sections[section_i].section,
                    input.time_slots[my_time_slot].name,
                    input.sections[soft_conflict_section].course,
                    input.sections[soft_conflict_section].section,
                    input.time_slots[other_time_slot].name
                )
            };

            let mut sections = Vec::new();
            let mut instructors = Vec::new();
            for &elt in &input.sections[section_i].cross_listings {
                sections.push(elt);
                for &inst in &input.sections[elt].instructors {
                    instructors.push(inst);
                }
            }
            for &elt in &input.sections[soft_conflict_section].cross_listings {
                sections.push(elt);
                for &inst in &input.sections[elt].instructors {
                    instructors.push(inst);
                }
            }
            let mut time_slots = vec![my_time_slot, other_time_slot];
            sections.sort();
            sections.dedup();
            instructors.sort();
            instructors.dedup();
            time_slots.sort();
            time_slots.dedup();
            self.problems.push(Problem {
                penalty,
                message,
                sections,
                instructors,
                time_slots,
            });
        }
    }

    pub fn compute_score_room_and_time_penalties(
        &mut self,
        input: &Input,
        section_i: usize,
        gather_problems: bool,
        deltas: &mut Vec<PenaltyDelta>,
    ) {
        // grab the time slot we are placed in; quit if not placed or no penalty
        let Some(RoomTimeWithPenalty { room, time_slot, penalty }) = self.sections[section_i].placement else {
            return;
        };
        if penalty == 0 {
            return;
        }

        deltas.push(PenaltyDelta::RoomTimePenalty{ section: section_i, delta: penalty });

        if !gather_problems {
            return;
        }

        // build the problem record
        let elt = &input.sections[section_i];
        let message = format!(
            "section room/time preference: {}-{} meets in {} at {}",
            elt.course, elt.section, input.rooms[room].name, input.time_slots[time_slot].name
        );

        let mut sections = Vec::new();
        let mut instructors = Vec::new();
        for &elt in &input.sections[section_i].cross_listings {
            sections.push(elt);
            for &inst in &input.sections[elt].instructors {
                instructors.push(inst);
            }
        }
        let mut time_slots = vec![time_slot];
        sections.sort();
        sections.dedup();
        instructors.sort();
        instructors.dedup();
        time_slots.sort();
        time_slots.dedup();
        self.problems.push(Problem {
            penalty,
            message,
            sections,
            instructors,
            time_slots,
        });
    }

    pub fn print_schedule(&self, input: &Input) {
        let no_instructor_msg = "(no instructor)".to_string();
        let mut name_len = no_instructor_msg.len();
        for (section_i, section) in input.sections.iter().enumerate() {
            if !section.instructors.is_empty() {
                let plus = if section.instructors.len() == 1 { 0 } else { 1 };
                let instructor = section.instructors[0];
                name_len = std::cmp::max(name_len, input.instructors[instructor].name.len() + 1 + plus);
            }
            if section.cross_listings[0] == section_i {
                let plus = if section.cross_listings.len() == 1 { 0 } else { 1 };
                name_len = std::cmp::max(name_len, section.course.len() + section.section.len() + 1 + plus);
            }
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
                    let section = &input.sections[input.sections[section_i].cross_listings[0]];
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

pub struct EvictionTracker(std::collections::HashMap<usize, std::collections::HashMap<usize, isize>>);

impl EvictionTracker {
    pub fn new() -> Self {
        EvictionTracker(std::collections::HashMap::new())
    }

    pub fn add_eviction(&mut self, placed: usize, displaced: usize) {
        if !self.0.contains_key(&displaced) {
            self.0.insert(displaced, std::collections::HashMap::new());
        }
        let bullies = self.0.get_mut(&displaced).unwrap();
        let count = match bullies.get(&placed) {
            Some(n) => n+1,
                None => 1 as isize,
        };
        bullies.insert(placed, count);
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
    solver.compute_score(input, false);
    let mut best_score = solver.score;

    for iteration in 0..iterations {
        print!("{} ", iteration);
        let section = solver.select_section_to_place(input);
        let room_time = solver.select_room_time_to_place(section);
        let mut undo = PlacementLog::new();
        solver.place_with_displacements(input, section, room_time, &mut undo);
        for elt in &undo.entries {
            if let &PlacementEntry::Diplaced(loser, _) = elt {
                evicted_by.add_eviction(section, loser);
            }
        }
        let score = solver.score;
        if score < best_score || iterations == 2*solver.sections.len() {
            println!();
            if score < best_score {
                best_score = score;
                solver.compute_score(input, true);
                assert!(score == solver.score);
                winner = solver.clone();
                if iteration < 2 * solver.sections.len() {
                    continue
                }
            }

            println!();
            println!();
            //winner.print_schedule(input);
            println!("score = {}", score);
            if !winner.problems.is_empty() {
                winner.problems.sort_by_key(|elt| -elt.penalty);
                let digits = winner.problems[0].penalty.to_string().len();
                for problem in &winner.problems {
                    println!(
                        "[{:width$}]  {}",
                        problem.penalty,
                        problem.message,
                        width = digits
                    );
                }
                for (i, section) in winner.sections.iter().enumerate() {
                    if section.placement.is_some() {
                        continue;
                    }
                    if input.sections[i].cross_listings.len() > 1
                        && input.sections[i].cross_listings[0] != i
                    {
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
                            print!(" {}-{}×{}", input.sections[sec].course, input.sections[sec].section, count);
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
