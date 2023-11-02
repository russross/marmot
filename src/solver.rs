use super::input::*;
use rand::Rng;

pub struct Solver {
    pub room_placements: Vec<RoomPlacements>,
    pub sections: Vec<SolverSection>,
}

pub struct RoomPlacements {
    pub time_slot_placements: Vec<Option<usize>>,
}

pub struct SolverSection {
    pub placement: Option<RoomTime>,
    pub tickets: u64,
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
                tickets: 0,
            });
        }
        Solver {
            room_placements: room_placements,
            sections: sections,
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

    pub fn add_placement(&mut self, section: usize, room: usize, time_slot: usize) {
        if let Some(RoomTime {
            room: _r,
            time_slot: _t,
        }) = std::mem::replace(
            &mut self.sections[section].placement,
            Some(RoomTime { room, time_slot }),
        ) {
            panic!("Solver::add_placement section already filled");
        }
        if let Some(_s) = std::mem::replace(
            &mut self.room_placements[room].time_slot_placements[time_slot],
            Some(section),
        ) {
            panic!("Solver::add_placement room time pair already filled");
        }
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
        room: usize,
        time_slot: usize,
    ) {
        // is this slot (or an overlapping time in the same room) already occupied?
        let mut evictees = Vec::new();
        for overlapping in &input.time_slots[time_slot].conflicts {
            if let Some(existing) = self.room_placements[room].time_slot_placements[*overlapping] {
                evictees.push(existing);
            }
        }

        for cross_listing in &input.sections[section].cross_listings {
            for hard_conflict in &input.sections[*cross_listing].hard_conflicts {
                let main_cross_listing = input.sections[*hard_conflict].cross_listings[0];
                if let Some(RoomTime {
                    room: _r,
                    time_slot: t,
                }) = self.sections[main_cross_listing].placement
                {
                    if input.time_slots[time_slot].conflicts.contains(&t) {
                        evictees.push(main_cross_listing);
                    }
                }
            }
        }

        evictees.iter().for_each(|&i| self.remove_placement(i));
    }

    //pub fn select_section_to_place(&mut self, input: &Input) {
    //    let mut rng = rand::thread_rng();
    //    let selection = rng.gen_range(0..input.sections.len());
    //}

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
        println!("");

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

    pub fn run_stupid_solver(&mut self, term: &Input) {
        let mut rng = rand::thread_rng();
        for _ in 0..10000 {
            let section_i = rng.gen_range(0..term.sections.len());
            let section = &term.sections[section_i];
            if section.cross_listings[0] != section_i {
                continue;
            }
            let &RoomTimeWithPenalty {
                room, time_slot, ..
            } = &section.room_times[rng.gen_range(0..section.room_times.len())];
            self.remove_placement(section_i);
            self.displace_conflicts(&term, section_i, room, time_slot);
            self.add_placement(section_i, room, time_slot);
        }
    }
}
