use super::input::*;
use rand::Rng;

const PENALTY_FOR_UNPLACED_SECTION: i64 = 1000;
const MIN_LOTTERY_TICKETS_FOR_UNPLACED_SECTION: i64 = 1000;

#[derive(Clone)]
pub struct Solver {
    pub room_placements: Vec<RoomPlacements>,
    pub sections: Vec<SolverSection>,
    pub score: i64,
    pub problems: Vec<Problem>,
}

#[derive(Clone)]
pub struct RoomPlacements {
    pub time_slot_placements: Vec<Option<usize>>,
}

#[derive(Clone)]
pub struct SolverSection {
    pub placement: Option<RoomTime>,
    pub penalty: i64,
    pub tickets: i64,

    // map from input.section[_].room_times to the effect on the score
    // if we placed this section at that room/time
    pub speculative_deltas: Vec<Option<i64>>,
    pub speculative_delta_min: Option<i64>,
}

#[derive(Clone)]
pub struct Problem {
    pub penalty: i64,
    pub message: String,
    pub sections: Vec<usize>,
    pub instructors: Vec<usize>,
    pub time_slots: Vec<usize>,
}

pub struct UndoLog {
    pub entries: Vec<UndoEntry>,
}

pub enum UndoEntry {
    Placed(usize),
    Displaced(usize, RoomTime),
}

impl UndoLog {
    pub fn new() -> Self {
        UndoLog {
            entries: Vec::new(),
        }
    }

    pub fn undo(&mut self, solver: &mut Solver) {
        let mut redo = UndoLog::new();
        loop {
            // play the log in reverse order
            match self.entries.pop() {
                Some(UndoEntry::Placed(section)) => solver.remove_placement(section, &mut redo),
                Some(UndoEntry::Displaced(section, room_time)) => {
                    solver.add_placement_without_displacing(section, room_time, &mut redo)
                }
                None => return,
            }
        }
    }
}

impl Solver {
    pub fn new(input: &Input) -> Self {
        let mut room_placements = Vec::with_capacity(input.rooms.len());
        for _ in 0..input.rooms.len() {
            room_placements.push(RoomPlacements {
                time_slot_placements: vec![None; input.time_slots.len()],
            });
        }
        let mut sections = Vec::with_capacity(input.sections.len());
        for i in 0..input.sections.len() {
            sections.push(SolverSection {
                placement: None,
                penalty: 0,
                tickets: 0,
                speculative_deltas: vec![None; input.sections[i].room_times.len()],
                speculative_delta_min: None,
            });
        }
        Solver {
            room_placements,
            sections,
            score: 0,
            problems: Vec::new(),
        }
    }

    pub fn is_placed(&self, section_i: usize) -> bool {
        self.sections[section_i].placement.is_some()
    }

    pub fn set_placement(
        &mut self,
        input: &Input,
        section_i: usize,
        room_time: RoomTime,
        undo: &mut UndoLog,
    ) {
        self.remove_placement(section_i, undo);
        self.displace_conflicts(input, section_i, &room_time, undo);
        self.add_placement_without_displacing(section_i, room_time, undo);
    }

    // remove a section from its current room/time placement (if any)
    // remove it from both sections and room_placements
    pub fn remove_placement(&mut self, section: usize, undo: &mut UndoLog) {
        if let Some(rt) = std::mem::take(&mut self.sections[section].placement) {
            assert!(std::mem::take(&mut self.room_placements[rt.room].time_slot_placements[rt.time_slot]) == Some(section),
            "Solver::remove_placement: placement by section does not match placement by room and time");

            undo.entries.push(UndoEntry::Displaced(section, rt));
        }
    }

    pub fn add_placement_without_displacing(
        &mut self,
        section: usize,
        room_time: RoomTime,
        undo: &mut UndoLog,
    ) {
        let RoomTime { room, time_slot } = room_time;
        let old_by_section =
            std::mem::replace(&mut self.sections[section].placement, Some(room_time));
        assert!(old_by_section.is_none());

        let old_by_room_time = std::mem::replace(
            &mut self.room_placements[room].time_slot_placements[time_slot],
            Some(section),
        );
        assert!(old_by_room_time.is_none());

        undo.entries.push(UndoEntry::Placed(section));
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
        room_time: &RoomTime,
        undo: &mut UndoLog,
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
        for &hard_conflict in &input.sections[section].hard_conflicts_combined {
            if let Some(RoomTime { time_slot, .. }) = self.sections[hard_conflict].placement {
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
        for section_i in 0..input.sections.len() {
            self.compute_speculative_deltas_section(input, section_i, old_score);
        }
    }

    pub fn compute_speculative_deltas_section(
        &mut self,
        input: &Input,
        section_i: usize,
        old_score: i64,
    ) {
        if !input.is_primary_cross_listing(section_i) {
            return;
        }
        let current = match self.sections[section_i].placement {
            Some(RoomTime { room, time_slot }) => (room, time_slot),
            None => (usize::MAX, usize::MAX),
        };
        let mut low = None;
        for rtp_i in 0..input.sections[section_i].room_times.len() {
            let RoomTimeWithPenalty {
                room, time_slot, ..
            } = input.sections[section_i].room_times[rtp_i];
            if (room, time_slot) == current {
                self.sections[section_i].speculative_deltas[rtp_i] = None;
            } else {
                let rt = RoomTime { room, time_slot };
                let mut undo = UndoLog::new();
                self.set_placement(input, section_i, rt, &mut undo);

                // see how it affected the score
                self.compute_score(input, false);
                let delta = self.score - old_score;
                self.sections[section_i].speculative_deltas[rtp_i] = Some(delta);

                low = match low {
                    Some(n) => Some(std::cmp::min(n, delta)),
                    None => Some(delta),
                };

                // undo the changes
                undo.undo(self);
            }
        }
        self.sections[section_i].speculative_delta_min = low;
    }

    pub fn select_section_to_place(&mut self, input: &Input) -> usize {
        let mut rng = rand::thread_rng();

        self.compute_speculative_deltas(&input);
        let mut pool_size = 0;
        for (i, section) in self.sections.iter_mut().enumerate() {
            if !input.is_primary_cross_listing(i) {
                continue;
            }
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

    pub fn select_room_time_to_place(&self, input: &Input, section_i: usize) -> RoomTime {
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
                    let RoomTimeWithPenalty {
                        room, time_slot, ..
                    } = input.sections[section_i].room_times[i];
                    return RoomTime { room, time_slot };
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
            if !input.is_primary_cross_listing(i) {
                assert!(section.tickets == 0);
                continue;
            }
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

    pub fn select_room_time_to_place_fast(&mut self, input: &Input, section_i: usize) -> RoomTime {
        self.compute_speculative_deltas_section(input, section_i, self.score);
        self.select_room_time_to_place(input, section_i)
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
        for i in 0..self.sections.len() {
            self.compute_score_section_soft_conflicts(input, i, gather_problems);
            if input.is_primary_cross_listing(i) && !self.is_placed(i) {
                self.score += PENALTY_FOR_UNPLACED_SECTION;
            }

            self.compute_score_room_and_time_penalties(input, i, gather_problems);
        }
    }

    pub fn compute_score_section_soft_conflicts(
        &mut self,
        input: &Input,
        section_i: usize,
        gather_problems: bool,
    ) {
        // calculate conflicts via the primary cross-listing
        if !input.is_primary_cross_listing(section_i) {
            return;
        }

        // grab the time slot we are placed in; quit if not placed
        let Some(RoomTime {
            time_slot: my_time_slot,
            ..
        }) = self.sections[section_i].placement
        else {
            return;
        };

        // look at the conflicts across all cross-listings
        for &SectionWithPenalty {
            section: soft_conflict_section,
            penalty,
        } in &input.sections[section_i].soft_conflicts_combined
        {
            // we will discover each conflict twice (A conflicts with B and B conflicts with A),
            // so only check when starting with the lower-numbered section
            if section_i >= soft_conflict_section {
                continue;
            }

            // check for placement of the conflicting course
            let Some(RoomTime {
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

            // we record it on both sections for lottery selection scoring
            self.sections[section_i].penalty += penalty;
            self.sections[soft_conflict_section].penalty += penalty;

            // but only once in the global penalty total for overall scoring
            self.score += penalty;

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
    ) {
        // calculate conflicts via the primary cross-listing
        if !input.is_primary_cross_listing(section_i) {
            return;
        }

        // grab the time slot we are placed in; quit if not placed
        let Some(RoomTime {
            room, time_slot, ..
        }) = self.sections[section_i].placement
        else {
            return;
        };

        // find the penalty associated with this placement
        let elt = &input.sections[section_i];
        let Some(RoomTimeWithPenalty { penalty, .. }) = elt
            .room_times
            .iter()
            .find(|elt| elt.room == room && elt.time_slot == time_slot)
        else {
            panic!(
                "section {}-{} placed at room+time that does not appear in its list",
                elt.course, elt.section
            );
        };

        // everyone happy?
        if *penalty == 0 {
            return;
        }

        self.sections[section_i].penalty += *penalty;
        self.score += *penalty;

        if !gather_problems {
            return;
        }

        // build the problem record
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
            penalty: *penalty,
            message,
            sections,
            instructors,
            time_slots,
        });
    }

    pub fn print_schedule(&self, input: &Input) {
        let mut name_len = 0;
        for (instructor_i, instructor) in input.instructors.iter().enumerate() {
            for &section_i in &instructor.sections {
                let section = &input.sections[section_i];
                if section.cross_listings.len() > 1 {
                    name_len =
                        std::cmp::max(name_len, section.course.len() + section.section.len() + 2);
                } else {
                    name_len =
                        std::cmp::max(name_len, section.course.len() + section.section.len() + 1);
                }
                if section.instructors[0] == instructor_i {
                    if section.instructors.len() > 1 {
                        name_len = std::cmp::max(name_len, instructor.name.len() + 1);
                    } else {
                        name_len = std::cmp::max(name_len, instructor.name.len());
                    }
                }
            }
        }

        let mut room_len = 0;
        for room in &input.rooms {
            room_len = std::cmp::max(room_len, room.name.len());
        }
        name_len = std::cmp::max(name_len, room_len);

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
                    let name = &input.instructors[instructors[0]].name;
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

pub fn solve(input: &Input, iterations: usize) {
    let mut solver = Solver::new(input);
    let mut best_score = i64::MAX;
    for _ in 0..iterations {
        let section = solver.select_section_to_place(input);
        let room_time = solver.select_room_time_to_place(input, section);
        let mut undo = UndoLog::new();
        solver.set_placement(input, section, room_time, &mut undo);
        solver.compute_score(input, false);
        let score = solver.score;
        if score < best_score {
            best_score = score;
            println!();
            println!();
            solver.compute_score(input, true);
            solver.print_schedule(input);
            println!("score = {}", score);
            if !solver.problems.is_empty() {
                solver.problems.sort_by_key(|elt| -elt.penalty);
                let digits = solver.problems[0].penalty.to_string().len();
                for problem in &solver.problems {
                    println!(
                        "[{:width$}]  {}",
                        problem.penalty,
                        problem.message,
                        width = digits
                    );
                }
                for (i, section) in solver.sections.iter().enumerate() {
                    if section.placement.is_some() {
                        continue;
                    }
                    if input.sections[i].cross_listings.len() > 1
                        && input.sections[i].cross_listings[0] != i
                    {
                        continue;
                    }
                    println!(
                        "unplaced: {}-{}",
                        input.sections[i].course, input.sections[i].section
                    );
                }
            }
        }
    }
}
