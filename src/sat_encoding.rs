use super::error::Result;
use super::input::*;
use rustsat::encodings::am1::{Encode, Pairwise};
use rustsat::encodings::card::{BoundUpper, DbTotalizer};
use rustsat::instances::{BasicVarManager, Cnf, ManageVars};
use rustsat::types::{Clause, Lit, Var};
use std::collections::HashMap;

// Simplified representation of criteria for SAT solving
#[derive(Debug, Clone)]
pub enum SatCriterion {
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
        priority: u8,
        section: usize,
        room: usize,
    },
    TimeSlotPreference {
        priority: u8,
        section: usize,
        time_slot: usize,
    },
    FacultyDaysOff {
        priority: u8,
        sections: Vec<usize>,
        days_to_check: Days,
        desired: usize,
    },
    FacultyEvenlySpread {
        priority: u8,
        sections: Vec<usize>,
        days_to_check: Days,
    },
    FacultyNoRoomSwitch {
        priority: u8,
        sections: Vec<usize>,
        days_to_check: Days,
        max_gap_within_cluster: Duration,
    },
    FacultyTooManyRooms {
        priority: u8,
        sections: Vec<usize>,
        desired: usize,
    },
    FacultyDistributionInterval {
        priority: u8,
        sections: Vec<usize>,
        days_to_check: Days,
        interval_type: DistributionIntervalType,
        duration: Duration,
        max_gap_within_cluster: Duration,
    },
    DifferentTimePatterns {
        priority: u8,
        sections: Vec<usize>,
    },
}

#[derive(Debug, Clone)]
pub enum DistributionIntervalType {
    GapTooShort,
    GapTooLong,
    ClusterTooShort,
    ClusterTooLong,
}

pub struct SATEncoder {
    // Core SAT variables
    pub section_time_vars: HashMap<(usize, usize), Var>, // (section, time_slot) -> var
    pub section_room_vars: HashMap<(usize, usize), Var>, // (section, room) -> var

    // Reverse lookups
    pub var_to_section_time: HashMap<Var, (usize, usize)>,
    pub var_to_section_room: HashMap<Var, (usize, usize)>,

    // RustSAT components
    pub var_manager: BasicVarManager,
    pub cnf: Cnf,
}

impl Default for SATEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl SATEncoder {
    pub fn new() -> Self {
        Self {
            section_time_vars: HashMap::new(),
            section_room_vars: HashMap::new(),
            var_to_section_time: HashMap::new(),
            var_to_section_room: HashMap::new(),
            var_manager: BasicVarManager::default(),
            cnf: Cnf::default(),
        }
    }

    // Initialize variables for sections and time slots
    pub fn initialize_variables(&mut self, input: &Input) {
        // Create variables for sections and time slots
        for (section_idx, section) in input.sections.iter().enumerate() {
            for ts in &section.time_slots {
                // Create variable for section scheduled at this time slot
                let time_slot = ts.time_slot;
                let var = self.var_manager.new_var();
                self.section_time_vars.insert((section_idx, time_slot), var);
                self.var_to_section_time.insert(var, (section_idx, time_slot));
            }

            // Create variables for section and rooms
            if !section.rooms.is_empty() {
                for r in &section.rooms {
                    let room = r.room;
                    // Create variable for section scheduled in this room
                    let var = self.var_manager.new_var();
                    self.section_room_vars.insert((section_idx, room), var);
                    self.var_to_section_room.insert(var, (section_idx, room));
                }
            }
        }
    }

    // Encode basic constraints that must always be satisfied
    pub fn encode_basic_constraints(&mut self, input: &Input) -> Result<()> {
        // For each section: exactly one time slot
        for (section_idx, section) in input.sections.iter().enumerate() {
            // get the SAT variables for these time slots
            let time_slots_for_section: Vec<_> = section.time_slots.iter().map(|ts| ts.time_slot).collect();
            let time_vars: Vec<Lit> = time_slots_for_section
                .iter()
                .filter_map(|&ts| self.section_time_vars.get(&(section_idx, ts)))
                .map(|&var| var.pos_lit())
                .collect();

            if time_vars.is_empty() {
                continue; // Skip if no time slots available
            }

            // at least one time slot
            self.cnf.add_clause(Clause::from_iter(time_vars.iter().copied()));

            // at most one time slot using pairwise encoding
            Pairwise::from_iter(time_vars.iter().copied()).encode(&mut self.cnf, &mut self.var_manager)?;
        }

        // for each section with rooms: exactly one room
        for section_idx in 0..input.sections.len() {
            let rooms_for_section: Vec<_> = input.sections[section_idx].rooms.iter().map(|r| r.room).collect();

            if rooms_for_section.is_empty() {
                continue;
            }

            // get the SAT variables for these rooms
            let room_vars: Vec<Lit> = rooms_for_section
                .iter()
                .filter_map(|&r| self.section_room_vars.get(&(section_idx, r)))
                .map(|&var| var.pos_lit())
                .collect();

            if room_vars.is_empty() {
                continue; // Skip if no rooms available
            }

            // at least one room
            self.cnf.add_clause(Clause::from_iter(room_vars.iter().copied()));

            // at most one room using pairwise encoding
            Pairwise::from_iter(room_vars.iter().copied()).encode(&mut self.cnf, &mut self.var_manager)?;
        }

        Ok(())
    }

    // Encode constraints that prevent room conflicts
    pub fn encode_room_conflicts(&mut self, input: &Input) {
        // for each room
        for room_idx in 0..input.rooms.len() {
            // find all sections that could be scheduled in this room
            let mut sections_for_room: Vec<usize> = Vec::new();

            for (section_idx, section) in input.sections.iter().enumerate() {
                if section.rooms.iter().any(|r| r.room == room_idx) {
                    sections_for_room.push(section_idx);
                }
            }

            // no possible conflicts if there are not at least two sections
            if sections_for_room.len() < 2 {
                continue;
            }

            // for each pair of sections
            for i in 0..sections_for_room.len() {
                let section_a_idx = sections_for_room[i];

                for &section_b_idx in sections_for_room.iter().skip(i + 1) {
                    // skip pairs that are already in hard conflict with each other
                    // they will be covered in the hard conflict encoding
                    if input.sections[section_a_idx].hard_conflicts.contains(&section_b_idx)
                        || input.sections[section_b_idx].hard_conflicts.contains(&section_a_idx)
                    {
                        continue;
                    }

                    // for each time slot pair
                    for &time_a in
                        &input.sections[section_a_idx].time_slots.iter().map(|ts| ts.time_slot).collect::<Vec<_>>()
                    {
                        for &time_b in
                            &input.sections[section_b_idx].time_slots.iter().map(|ts| ts.time_slot).collect::<Vec<_>>()
                        {
                            // skip if the time slots don't conflict
                            if !input.time_slot_conflicts[time_a][time_b] {
                                continue;
                            }

                            // get the section-time and section-room variables
                            if let (Some(&var_a_time), Some(&var_b_time), Some(&var_a_room), Some(&var_b_room)) = (
                                self.section_time_vars.get(&(section_a_idx, time_a)),
                                self.section_time_vars.get(&(section_b_idx, time_b)),
                                self.section_room_vars.get(&(section_a_idx, room_idx)),
                                self.section_room_vars.get(&(section_b_idx, room_idx)),
                            ) {
                                // add conflict clause: ~(A_time && A_room && B_time && B_room)
                                // which is equivalent to (!A_time || !A_room || !B_time || !B_room)
                                self.cnf.add_clause(Clause::from_iter([
                                    var_a_time.neg_lit(),
                                    var_a_room.neg_lit(),
                                    var_b_time.neg_lit(),
                                    var_b_room.neg_lit(),
                                ]));
                            }
                        }
                    }
                }
            }
        }
    }

    // Encode hard conflict constraints
    pub fn encode_hard_conflicts(&mut self, input: &Input) {
        // for each section
        for section_idx in 0..input.sections.len() {
            let section = &input.sections[section_idx];

            // get time slots for this section
            let time_slots: Vec<usize> = section.time_slots.iter().map(|ts| ts.time_slot).collect();

            // for each hard conflict
            for &conflict_idx in &section.hard_conflicts {
                let conflict_section = &input.sections[conflict_idx];

                // get time slots for the other section
                let conflict_time_slots: Vec<usize> =
                    conflict_section.time_slots.iter().map(|ts| ts.time_slot).collect();

                // for each time slot pair
                for &time_a in &time_slots {
                    for &time_b in &conflict_time_slots {
                        // skip if the time slots don't conflict
                        if !input.time_slot_conflicts[time_a][time_b] {
                            continue;
                        }

                        // get the section-time variables
                        if let (Some(&var_a_time), Some(&var_b_time)) = (
                            self.section_time_vars.get(&(section_idx, time_a)),
                            self.section_time_vars.get(&(conflict_idx, time_b)),
                        ) {
                            // add conflict clause: ~(A_time && B_time)
                            // which is equivalent to: (!A_time || !B_time)
                            self.cnf.add_clause(Clause::from_iter([var_a_time.neg_lit(), var_b_time.neg_lit()]));
                        }
                    }
                }
            }
        }
    }

    // Encode at-most-k constraint for a set of criterion variables
    pub fn encode_at_most_k(&mut self, criterion_vars: &[Lit], k: usize) -> Result<()> {
        if criterion_vars.is_empty() || k >= criterion_vars.len() {
            // No constraint needed if we can violate all criteria
            return Ok(());
        }

        // Use the DbTotalizer encoder for at-most-k
        DbTotalizer::from_iter(criterion_vars.iter().copied()).encode_ub(
            0..=k,
            &mut self.cnf,
            &mut self.var_manager,
        )?;

        Ok(())
    }

    // Encode a specific criterion
    pub fn encode_criterion(
        &mut self,
        input: &Input,
        criterion: &SatCriterion,
        violations_permitted: bool,
    ) -> Result<Option<Var>> {
        match criterion {
            SatCriterion::SoftConflict { priority: _, sections } => {
                self.encode_soft_conflict(input, sections, violations_permitted)
            }
            SatCriterion::AntiConflict { priority: _, single, group } => {
                self.encode_anti_conflict(input, *single, group, violations_permitted)
            }
            SatCriterion::RoomPreference { priority: _, section, room } => {
                self.encode_room_preference(input, *section, *room, violations_permitted)
            }
            SatCriterion::TimeSlotPreference { priority: _, section, time_slot } => {
                self.encode_time_slot_preference(input, *section, *time_slot, violations_permitted)
            }
            SatCriterion::FacultyDaysOff { priority: _, sections, days_to_check, desired } => {
                self.encode_faculty_days_off(input, sections, *days_to_check, *desired, violations_permitted)
            }
            SatCriterion::FacultyEvenlySpread { priority: _, sections, days_to_check } => {
                self.encode_faculty_evenly_spread(input, sections, *days_to_check, violations_permitted)
            }
            SatCriterion::FacultyNoRoomSwitch { priority: _, sections, days_to_check, max_gap_within_cluster } => self
                .encode_faculty_no_room_switch(
                    input,
                    sections,
                    *days_to_check,
                    *max_gap_within_cluster,
                    violations_permitted,
                ),
            SatCriterion::FacultyTooManyRooms { priority: _, sections, desired } => {
                self.encode_faculty_too_many_rooms(input, sections, *desired, violations_permitted)
            }
            SatCriterion::FacultyDistributionInterval {
                priority: _,
                sections,
                days_to_check,
                interval_type,
                duration,
                max_gap_within_cluster,
            } => self.encode_faculty_distribution_interval(
                input,
                sections,
                *days_to_check,
                interval_type,
                *duration,
                *max_gap_within_cluster,
                violations_permitted,
            ),
            SatCriterion::DifferentTimePatterns { priority: _, sections } => {
                self.encode_different_time_patterns(input, sections, violations_permitted)
            }
        }
    }

    // Encode soft conflict constraint
    fn encode_soft_conflict(
        &mut self,
        input: &Input,
        sections: &[usize; 2],
        violations_permitted: bool,
    ) -> Result<Option<Var>> {
        let section_a = sections[0];
        let section_b = sections[1];

        // Get time slots for both sections
        let time_slots_a: Vec<usize> = input.sections[section_a].time_slots.iter().map(|ts| ts.time_slot).collect();

        let time_slots_b: Vec<usize> = input.sections[section_b].time_slots.iter().map(|ts| ts.time_slot).collect();

        // Build collection of conflicting time slot pairs
        let mut conflict_pairs = Vec::new();

        for &time_a in &time_slots_a {
            for &time_b in &time_slots_b {
                // Skip if the time slots don't conflict
                if !input.time_slot_conflicts[time_a][time_b] {
                    continue;
                }

                // Get the section-time variables
                if let (Some(&var_a), Some(&var_b)) =
                    (self.section_time_vars.get(&(section_a, time_a)), self.section_time_vars.get(&(section_b, time_b)))
                {
                    // Add to our list of conflicts
                    conflict_pairs.push((var_a, var_b));
                }
            }
        }

        // If there are no possible conflicts, no need for constraint
        if conflict_pairs.is_empty() {
            return Ok(None);
        }

        if !violations_permitted {
            // When violations aren't permitted, directly encode that conflicting
            // assignments cannot both be true
            for (var_a, var_b) in conflict_pairs {
                // (!var_a || !var_b) - Both cannot be true
                self.cnf.add_clause(Clause::from_iter([var_a.neg_lit(), var_b.neg_lit()]));
            }
            Ok(None)
        } else {
            // When violations are permitted, create a criterion variable
            let criterion_var = self.var_manager.new_var();

            // For each potential conflict, if both sections are scheduled at conflicting times,
            // then the criterion is violated
            for (var_a, var_b) in conflict_pairs {
                // (!var_a || !var_b || criterion_var)
                // This means: If both var_a and var_b are true (sections scheduled at conflicting times),
                // then criterion_var must be true (constraint is violated)
                self.cnf.add_clause(Clause::from_iter([var_a.neg_lit(), var_b.neg_lit(), criterion_var.pos_lit()]));
            }

            Ok(Some(criterion_var))
        }
    }

    // Encode anti-conflict constraint
    fn encode_anti_conflict(
        &mut self,
        input: &Input,
        single: usize,
        group: &Vec<usize>,
        violations_permitted: bool,
    ) -> Result<Option<Var>> {
        // Get time slots for the single section
        let time_slots_single: Vec<usize> = input.sections[single].time_slots.iter().map(|ts| ts.time_slot).collect();

        // Structure to track time slots and their corresponding variables
        let mut time_matches = Vec::new();

        for &time_single in &time_slots_single {
            // Get the variable for the single section at this time
            if let Some(&var_single) = self.section_time_vars.get(&(single, time_single)) {
                // Create a list of group section variables for this time slot
                let mut group_vars = Vec::new();

                for &group_section in group {
                    // Get matching time slots for this group section
                    for &time_slot_group in
                        &input.sections[group_section].time_slots.iter().map(|ts| ts.time_slot).collect::<Vec<_>>()
                    {
                        // Only consider exact time matches (not just overlapping times)
                        if time_single == time_slot_group {
                            if let Some(&var_group) = self.section_time_vars.get(&(group_section, time_slot_group)) {
                                group_vars.push(var_group.pos_lit());
                            }
                        }
                    }
                }

                // If we found group variables for this time slot
                if !group_vars.is_empty() {
                    time_matches.push((var_single, group_vars));
                }
            }
        }

        // If there are no possible matches, check if the single section can be scheduled
        if time_matches.is_empty() {
            let single_vars: Vec<_> =
                time_slots_single.iter().filter_map(|&ts| self.section_time_vars.get(&(single, ts))).collect();

            // If the single section can't be scheduled, no constraint is needed
            if single_vars.is_empty() {
                return Ok(None);
            }

            // If violations are not permitted, then the single section cannot be scheduled
            if !violations_permitted {
                for &var_single in &single_vars {
                    self.cnf.add_clause(Clause::from_iter([var_single.neg_lit()]));
                }
                return Ok(None);
            } else {
                // With violations permitted, create a criterion variable that's true
                // whenever the single section is scheduled
                let criterion_var = self.var_manager.new_var();

                for &var_single in &single_vars {
                    self.cnf.add_clause(Clause::from_iter([var_single.neg_lit(), criterion_var.pos_lit()]));
                }

                // Also ensure criterion is false if single section not scheduled
                let mut at_least_one_clause = single_vars.iter().map(|&&var| var.pos_lit()).collect::<Vec<_>>();
                at_least_one_clause.push(criterion_var.neg_lit());
                self.cnf.add_clause(Clause::from_iter(at_least_one_clause));

                return Ok(Some(criterion_var));
            }
        }

        // There are possible matches, so encode the constraints
        if !violations_permitted {
            // Directly encode that if the single section is at a time,
            // at least one group section must also be at that time
            for (var_single, group_vars) in time_matches {
                let mut clause = vec![var_single.neg_lit()];
                clause.extend(group_vars);
                self.cnf.add_clause(Clause::from_iter(clause));
            }
            Ok(None)
        } else {
            // With violations permitted, create a criterion variable
            let criterion_var = self.var_manager.new_var();

            // For each time slot the single section could be at
            for (var_single, group_vars) in &time_matches {
                // If single section is at this time, one group section must also be at this time
                let mut clause = vec![var_single.neg_lit()];
                clause.extend(group_vars);
                clause.push(criterion_var.pos_lit());
                self.cnf.add_clause(Clause::from_iter(clause));
            }

            // We also need to ensure criterion is false if single section not scheduled
            let single_vars = time_matches.iter().map(|(var, _)| var.pos_lit()).collect::<Vec<_>>();

            let mut at_least_one_clause = single_vars;
            at_least_one_clause.push(criterion_var.neg_lit());
            self.cnf.add_clause(Clause::from_iter(at_least_one_clause));

            Ok(Some(criterion_var))
        }
    }

    // Encode room preference constraint
    fn encode_room_preference(
        &mut self,
        _input: &Input,
        _section: usize,
        _room: usize,
        _violations_permitted: bool,
    ) -> Result<Option<Var>> {
        unimplemented!("Room preference encoding not yet implemented");
    }

    // Encode time slot preference constraint
    fn encode_time_slot_preference(
        &mut self,
        _input: &Input,
        _section: usize,
        _time_slot: usize,
        _violations_permitted: bool,
    ) -> Result<Option<Var>> {
        unimplemented!("Time slot preference encoding not yet implemented");
    }

    // Encode faculty days off constraint
    fn encode_faculty_days_off(
        &mut self,
        _input: &Input,
        _sections: &[usize],
        _days_to_check: Days,
        _desired: usize,
        _violations_permitted: bool,
    ) -> Result<Option<Var>> {
        unimplemented!("Faculty days off encoding not yet implemented");
    }

    // Encode faculty evenly spread constraint
    fn encode_faculty_evenly_spread(
        &mut self,
        _input: &Input,
        _sections: &[usize],
        _days_to_check: Days,
        _violations_permitted: bool,
    ) -> Result<Option<Var>> {
        unimplemented!("Faculty evenly spread encoding not yet implemented");
    }

    // Encode faculty no room switch constraint
    fn encode_faculty_no_room_switch(
        &mut self,
        _input: &Input,
        _sections: &[usize],
        _days_to_check: Days,
        _max_gap_within_cluster: Duration,
        _violations_permitted: bool,
    ) -> Result<Option<Var>> {
        unimplemented!("Faculty no room switch encoding not yet implemented");
    }

    // Encode faculty too many rooms constraint
    fn encode_faculty_too_many_rooms(
        &mut self,
        _input: &Input,
        _sections: &[usize],
        _desired: usize,
        _violations_permitted: bool,
    ) -> Result<Option<Var>> {
        unimplemented!("Faculty too many rooms encoding not yet implemented");
    }

    // Encode faculty distribution interval constraint
    #[allow(clippy::too_many_arguments)]
    fn encode_faculty_distribution_interval(
        &mut self,
        _input: &Input,
        _sections: &[usize],
        _days_to_check: Days,
        _interval_type: &DistributionIntervalType,
        _duration: Duration,
        _max_gap_within_cluster: Duration,
        _violations_permitted: bool,
    ) -> Result<Option<Var>> {
        unimplemented!("Faculty distribution interval encoding not yet implemented");
    }

    // Encode different time patterns constraint
    fn encode_different_time_patterns(
        &mut self,
        _input: &Input,
        _sections: &[usize],
        _violations_permitted: bool,
    ) -> Result<Option<Var>> {
        unimplemented!("Different time patterns encoding not yet implemented");
    }
}
