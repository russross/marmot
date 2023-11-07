use super::input::*;
use rand::Rng;

#[derive(Clone)]
pub struct Solver {
    pub room_placements: Vec<RoomPlacements>,
    pub sections: Vec<SolverSection>,
    pub total_penalty: u64,
    pub problems: Vec<Problem>,
}

#[derive(Clone)]
pub struct RoomPlacements {
    pub time_slot_placements: Vec<Option<usize>>,
}

#[derive(Clone)]
pub struct SolverSection {
    pub placement: Option<RoomTime>,
    pub penalty: u64,
    pub tickets: u64,
}

#[derive(Clone)]
pub struct Problem {
    pub penalty: u64,
    pub message: String,
    pub sections: Vec<usize>,
    pub instructors: Vec<usize>,
    pub time_slots: Vec<usize>,
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
        for _ in 0..input.sections.len() {
            sections.push(SolverSection {
                placement: None,
                penalty: 0,
                tickets: 0,
            });
        }
        Solver {
            room_placements: room_placements,
            sections: sections,
            total_penalty: 0,
            problems: Vec::new(),
        }
    }

    // remove a section from its current room/time placement (if any)
    // remove it from both sections and room_placements
    pub fn remove_placement(&mut self, section: usize) {
        if let Some(RoomTime { room, time_slot }) =
            std::mem::take(&mut self.sections[section].placement)
        {
            assert!(std::mem::take(&mut self.room_placements[room].time_slot_placements[time_slot]) == Some(section),
            "Solver::remove_placement: placement by section does not match placement by room and time");
        }
    }

    pub fn add_placement(&mut self, section: usize, room_time: &RoomTime) {
        let old_by_section = std::mem::replace(
            &mut self.sections[section].placement,
            Some(RoomTime {
                room: room_time.room,
                time_slot: room_time.time_slot,
            }),
        );
        assert!(old_by_section.is_none());

        let old_by_room_time = std::mem::replace(
            &mut self.room_placements[room_time.room].time_slot_placements[room_time.time_slot],
            Some(section),
        );
        assert!(old_by_room_time.is_none());
    }

    // remove any sections that will be in conflict with a section about to be placed
    //
    // this includes:
    // * anything in the same room in an overlapping time slot
    // * anything in the hard conflict list of this section (or a cross listing)
    //   in the same/an overlapping time slot
    pub fn displace_conflicts(&mut self, input: &Input, section: usize, room_time: &RoomTime) {
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

        evictees.iter().for_each(|&i| self.remove_placement(i));
    }

    pub fn select_section_to_place(&mut self, input: &Input) -> usize {
        let mut rng = rand::thread_rng();

        // calculate lottery tickets for each section and gather total
        let mut pool_size = 0;
        for i in 0..self.sections.len() {
            self.compute_lottery_tickets(input, i);
            pool_size += self.sections[i].tickets;
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
        let room_times = &input.sections[section_i].room_times;
        let i = rng.gen_range(0..room_times.len());
        let RoomTimeWithPenalty {
            room, time_slot, ..
        } = room_times[i];
        RoomTime {
            room: room,
            time_slot: time_slot,
        }
    }

    pub fn compute_score(&mut self, input: &Input) {
        // zero out all the scores
        self.total_penalty = 0;
        self.problems.clear();
        for section in &mut self.sections.iter_mut() {
            section.penalty = 0;
        }

        // score soft conflicts
        // and add 500 for each unplaced section
        for i in 0..self.sections.len() {
            self.compute_score_section_soft_conflicts(input, i);
            if input.sections[i].is_primary_cross_listing(i) && self.sections[i].placement.is_none()
            {
                self.total_penalty += 500;
            }
        }
    }

    pub fn compute_score_section_soft_conflicts(&mut self, input: &Input, section_i: usize) {
        // calculate conflicts via the primary cross-listing
        if !input.sections[section_i].is_primary_cross_listing(section_i) {
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

            // record the penalty for both affected sections
            let msg = if my_time_slot == other_time_slot {
                format!(
                    "curriculum conflict: {}-{} and {}-{} both meet at {}",
                    input.sections[section_i].course,
                    input.sections[section_i].section,
                    input.sections[soft_conflict_section].course,
                    input.sections[soft_conflict_section].section,
                    input.time_slots[my_time_slot].name
                )
                .to_string()
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
                .to_string()
            };

            // we record it on both sections for lottery selection scoring
            self.sections[section_i].penalty += penalty;
            self.sections[soft_conflict_section].penalty += penalty;

            // but only once in the global penalty total for overall scoring
            self.total_penalty += penalty;

            // build the problem record
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
            sections.sort();
            sections.dedup();
            instructors.sort();
            instructors.dedup();
            self.problems.push(Problem {
                penalty: penalty,
                message: msg,
                sections: sections,
                instructors: instructors,
                time_slots: vec![my_time_slot, other_time_slot],
            });
        }
    }

    pub fn compute_lottery_tickets(&mut self, input: &Input, section: usize) {
        self.sections[section].tickets = if input.sections[section].cross_listings[0] != section {
            // ignore secondary cross-listings
            0
        } else {
            let mut tickets = 0;

            // unplaced sections extra tickets
            if self.sections[section].placement.is_none() {
                tickets += 1000;
            } else {
                tickets = 1;
            }

            // add penalty scores from last round
            for &cross_listing in &input.sections[section].cross_listings {
                tickets += self.sections[cross_listing].penalty;
            }
            tickets
        };
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
    let mut best_score = u64::MAX;
    for _ in 0..iterations {
        let section = solver.select_section_to_place(input);
        let room_time = solver.select_room_time_to_place(input, section);
        solver.remove_placement(section);
        solver.displace_conflicts(input, section, &room_time);
        solver.add_placement(section, &room_time);
        solver.compute_score(input);
        let score = solver.total_penalty;
        if score < best_score {
            best_score = score;
            println!();
            println!();
            solver.print_schedule(input);
            if !solver.problems.is_empty() {
                solver.problems.sort_by_key(|elt| u64::MAX - elt.penalty);
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
