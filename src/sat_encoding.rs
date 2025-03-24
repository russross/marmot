use super::error::Result;
use super::input::*;
use rustsat::encodings::am1::{Bitwise, Commander, Encode, Pairwise};
use rustsat::instances::{BasicVarManager, Cnf, SatInstance};
use rustsat::types::constraints::CardConstraint;
use rustsat::types::{Lit, Var};
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

impl SatCriterion {
    // Returns a numeric type identifier for sorting criteria by type
    pub fn get_type_id(&self) -> u8 {
        match self {
            SatCriterion::SoftConflict { .. } => 1,
            SatCriterion::AntiConflict { .. } => 2,
            SatCriterion::RoomPreference { .. } => 3,
            SatCriterion::TimeSlotPreference { .. } => 4,
            SatCriterion::FacultyDaysOff { .. } => 5,
            SatCriterion::FacultyEvenlySpread { .. } => 6,
            SatCriterion::FacultyNoRoomSwitch { .. } => 7,
            SatCriterion::FacultyTooManyRooms { .. } => 8,
            SatCriterion::FacultyDistributionInterval { .. } => 9,
            SatCriterion::DifferentTimePatterns { .. } => 10,
        }
    }
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
    pub sat_instance: SatInstance,
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
            sat_instance: SatInstance::new_with_manager(BasicVarManager::default()),
        }
    }

    pub fn encode_cardinality(&mut self, clause: Vec<Lit>, min: Option<usize>, max: Option<usize>) -> Result<()> {
        // special cases
        if clause.is_empty() {
            return Ok(());
        }
        let mut encoding = Cnf::new();
        match (min, max) {
            // trivially satisfied
            (None, Some(n)) if n >= clause.len() => (),

            // at-most-1 with n ≤ 10 use pairwise
            (None | Some(1), Some(1)) if clause.len() <= 10 => {
                // for exactly-1 add the clause itself to get at-least-1
                if min.is_some() {
                    self.sat_instance.add_nary(&clause);
                }

                println!("using pairwise to encode at-most-1 for n={}", clause.len());
                Pairwise::from(clause).encode(&mut encoding, self.sat_instance.var_manager_mut())?;
            }

            // for at-most-1 with 10 < n ≤ 30 use bitwise (binary)
            (None | Some(1), Some(1)) if clause.len() <= 30 => {
                // for exactly-1 add the clause itself to get at-least-1
                if min.is_some() {
                    self.sat_instance.add_nary(&clause);
                }

                println!("using bitwise to encode at-most-1 for n={}", clause.len());
                Bitwise::from(clause).encode(&mut encoding, self.sat_instance.var_manager_mut())?;
            }

            // for at-most-1 with 30 < n ≤ 100 use commander
            (None | Some(1), Some(1)) if clause.len() <= 30 => {
                // for exactly-1 add the clause itself to get at-least-1
                if min.is_some() {
                    self.sat_instance.add_nary(&clause);
                }

                println!("using commander to encode at-most-1 for n={}", clause.len());
                Commander::<4, Pairwise>::from(clause).encode(&mut encoding, self.sat_instance.var_manager_mut())?;
            }

            // for at-least-1 we can just copy the clause in
            (Some(1), None) => self.sat_instance.add_nary(&clause),

            // for now, fall back to totalizer for all other cases
            (None, Some(k)) => {
                println!("using totalizer for at-most-{} for n={}", k, clause.len());
                self.sat_instance.add_card_constr(CardConstraint::new_ub(clause, k));
            }
            (Some(k), None) => {
                println!("using totalizer for at-least-{} for n={}", k, clause.len());
                self.sat_instance.add_card_constr(CardConstraint::new_lb(clause, k));
            }
            (Some(k1), Some(k2)) if k1 == k2 => {
                println!("using totalizer for exactly-{} for n={}", k1, clause.len());
                self.sat_instance.add_card_constr(CardConstraint::new_eq(clause, k1));
            }
            _ => unimplemented!("unexecpted cardinality requirement"),
        }

        for c in encoding {
            self.sat_instance.add_clause(c);
        }
        Ok(())
    }

    // Initialize variables for sections and time slots
    pub fn initialize_variables(&mut self, input: &Input) {
        // Create variables for sections and time slots
        for (section_idx, section) in input.sections.iter().enumerate() {
            for ts in &section.time_slots {
                // Create variable for section scheduled at this time slot
                let time_slot = ts.time_slot;
                let var = self.sat_instance.new_var();
                self.section_time_vars.insert((section_idx, time_slot), var);
                self.var_to_section_time.insert(var, (section_idx, time_slot));
            }

            // Create variables for section and rooms
            if !section.rooms.is_empty() {
                for r in &section.rooms {
                    let room = r.room;
                    // Create variable for section scheduled in this room
                    let var = self.sat_instance.new_var();
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

            // exactly-1
            self.encode_cardinality(time_vars, Some(1), Some(1))?;
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

            // exactly-1
            self.encode_cardinality(room_vars, Some(1), Some(1))?;
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
                                self.sat_instance.add_nary(&[
                                    var_a_time.neg_lit(),
                                    var_a_room.neg_lit(),
                                    var_b_time.neg_lit(),
                                    var_b_room.neg_lit(),
                                ]);
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
                            self.sat_instance.add_binary(var_a_time.neg_lit(), var_b_time.neg_lit());
                        }
                    }
                }
            }
        }
    }

    pub fn encode_criteria_group(
        &mut self,
        input: &Input,
        criteria: &[SatCriterion],
        violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        if criteria.is_empty() {
            return Ok(Vec::new());
        }

        // Dispatch based on the type of the first criterion
        // (all criteria in the group are of the same type)
        match &criteria[0] {
            SatCriterion::SoftConflict { .. } => self.encode_soft_conflicts(input, criteria, violations_permitted),
            SatCriterion::AntiConflict { .. } => self.encode_anti_conflicts(input, criteria, violations_permitted),
            SatCriterion::RoomPreference { .. } => self.encode_room_preferences(input, criteria, violations_permitted),
            SatCriterion::TimeSlotPreference { .. } => {
                self.encode_time_slot_preferences(input, criteria, violations_permitted)
            }
            SatCriterion::FacultyDaysOff { .. } => {
                self.encode_faculty_days_off_group(input, criteria, violations_permitted)
            }
            SatCriterion::FacultyEvenlySpread { .. } => {
                self.encode_faculty_evenly_spread_group(input, criteria, violations_permitted)
            }
            SatCriterion::FacultyNoRoomSwitch { .. } => {
                self.encode_faculty_no_room_switch_group(input, criteria, violations_permitted)
            }
            SatCriterion::FacultyTooManyRooms { .. } => {
                self.encode_faculty_too_many_rooms_group(input, criteria, violations_permitted)
            }
            SatCriterion::FacultyDistributionInterval { .. } => {
                self.encode_faculty_distribution_interval_group(input, criteria, violations_permitted)
            }
            SatCriterion::DifferentTimePatterns { .. } => {
                self.encode_different_time_patterns_group(input, criteria, violations_permitted)
            }
        }
    }

    // Updated method for encoding soft conflicts as a group
    fn encode_soft_conflicts(
        &mut self,
        input: &Input,
        criteria: &[SatCriterion],
        violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        let mut criterion_vars = Vec::new();

        for criterion in criteria {
            if let SatCriterion::SoftConflict { sections, .. } = criterion {
                let section_a = sections[0];
                let section_b = sections[1];

                // Get time slots for both sections
                let time_slots_a: Vec<usize> =
                    input.sections[section_a].time_slots.iter().map(|ts| ts.time_slot).collect();
                let time_slots_b: Vec<usize> =
                    input.sections[section_b].time_slots.iter().map(|ts| ts.time_slot).collect();

                // Build collection of conflicting time slot pairs
                let mut conflict_pairs = Vec::new();

                for &time_a in &time_slots_a {
                    for &time_b in &time_slots_b {
                        // Skip if the time slots don't conflict
                        if !input.time_slot_conflicts[time_a][time_b] {
                            continue;
                        }

                        // Get the section-time variables
                        if let (Some(&var_a), Some(&var_b)) = (
                            self.section_time_vars.get(&(section_a, time_a)),
                            self.section_time_vars.get(&(section_b, time_b)),
                        ) {
                            // Add to our list of conflicts
                            conflict_pairs.push((var_a, var_b));
                        }
                    }
                }

                // If there are no possible conflicts, continue to the next criterion
                if conflict_pairs.is_empty() {
                    continue;
                }

                // For each potential conflict, if both sections are scheduled at conflicting times,
                // then the criterion is violated
                for (var_a, var_b) in conflict_pairs {
                    // When violations are permitted, create a criterion variable
                    // (!var_a || !var_b || criterion_var)
                    // This means: If both var_a and var_b are true (sections scheduled at conflicting times),
                    // then criterion_var must be true (constraint is violated)
                    if violations_permitted {
                        let criterion_var = self.sat_instance.new_var();
                        criterion_vars.push(criterion_var);
                        self.sat_instance.add_ternary(var_a.neg_lit(), var_b.neg_lit(), criterion_var.pos_lit());
                    } else {
                        self.sat_instance.add_binary(var_a.neg_lit(), var_b.neg_lit());
                    }
                }
            }
        }

        Ok(criterion_vars)
    }

    // Updated method for encoding anti-conflicts as a group
    fn encode_anti_conflicts(
        &mut self,
        input: &Input,
        criteria: &[SatCriterion],
        violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        let mut criterion_vars = Vec::new();

        for criterion in criteria {
            if let SatCriterion::AntiConflict { single, group, .. } = criterion {
                // Get time slots for the single section
                let time_slots_single: Vec<usize> =
                    input.sections[*single].time_slots.iter().map(|ts| ts.time_slot).collect();

                // Structure to track time slots and their corresponding variables
                let mut time_matches = Vec::new();

                for &time_single in &time_slots_single {
                    // Get the variable for the single section at this time
                    if let Some(&var_single) = self.section_time_vars.get(&(*single, time_single)) {
                        // Create a list of group section variables for this time slot
                        let mut group_vars = Vec::new();

                        for &group_section in group {
                            // Get matching time slots for this group section
                            for &time_slot_group in &input.sections[group_section]
                                .time_slots
                                .iter()
                                .map(|ts| ts.time_slot)
                                .collect::<Vec<_>>()
                            {
                                // Only consider exact time matches (not just overlapping times)
                                if time_single == time_slot_group {
                                    if let Some(&var_group) =
                                        self.section_time_vars.get(&(group_section, time_slot_group))
                                    {
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
                        time_slots_single.iter().filter_map(|&ts| self.section_time_vars.get(&(*single, ts))).collect();

                    // If the single section can't be scheduled, continue to the next criterion
                    if single_vars.is_empty() {
                        continue;
                    }

                    // If violations are not permitted, then the single section cannot be scheduled
                    if !violations_permitted {
                        for &var_single in &single_vars {
                            self.sat_instance.add_unit(var_single.neg_lit());
                        }
                    } else {
                        // With violations permitted, create a criterion variable that's true
                        // whenever the single section is scheduled
                        let criterion_var = self.sat_instance.new_var();
                        criterion_vars.push(criterion_var);

                        for &var_single in &single_vars {
                            self.sat_instance.add_binary(var_single.neg_lit(), criterion_var.pos_lit());
                        }

                        // Also ensure criterion is false if single section not scheduled
                        let mut at_least_one_clause = single_vars.iter().map(|&&var| var.pos_lit()).collect::<Vec<_>>();
                        at_least_one_clause.push(criterion_var.neg_lit());
                        self.sat_instance.add_nary(&at_least_one_clause);
                    }
                    continue;
                }

                // There are possible matches, so encode the constraints
                if !violations_permitted {
                    // Directly encode that if the single section is at a time,
                    // at least one group section must also be at that time
                    for (var_single, group_vars) in time_matches {
                        let mut clause = vec![var_single.neg_lit()];
                        clause.extend(group_vars);
                        self.sat_instance.add_nary(&clause);
                    }
                } else {
                    // With violations permitted, create a criterion variable
                    let criterion_var = self.sat_instance.new_var();
                    criterion_vars.push(criterion_var);

                    // For each time slot the single section could be at
                    for (var_single, group_vars) in &time_matches {
                        // If single section is at this time, one group section must also be at this time
                        let mut clause = vec![var_single.neg_lit()];
                        clause.extend(group_vars);
                        clause.push(criterion_var.pos_lit());
                        self.sat_instance.add_nary(&clause);
                    }

                    // We also need to ensure criterion is false if single section not scheduled
                    let single_vars = time_matches.iter().map(|(var, _)| var.pos_lit()).collect::<Vec<_>>();

                    let mut at_least_one_clause = single_vars;
                    at_least_one_clause.push(criterion_var.neg_lit());
                    self.sat_instance.add_nary(&at_least_one_clause);
                }
            }
        }

        Ok(criterion_vars)
    }

    fn encode_time_slot_preferences(
        &mut self,
        _input: &Input,
        criteria: &[SatCriterion],
        violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        let mut criterion_vars = Vec::new();

        for criterion in criteria {
            if let SatCriterion::TimeSlotPreference { section, time_slot, .. } = criterion {
                // If we have a variable for this section-time slot pair
                if let Some(&var) = self.section_time_vars.get(&(*section, *time_slot)) {
                    if !violations_permitted {
                        // Hard constraint: section must not be scheduled in this time slot
                        self.sat_instance.add_unit(var.neg_lit());
                    } else {
                        // Create a criterion variable for this individual section-time slot pair
                        let criterion_var = self.sat_instance.new_var();
                        criterion_vars.push(criterion_var);

                        // (section_at_time_slot => criterion_var)
                        // Encoded as: (!section_at_time_slot || criterion_var)
                        self.sat_instance.add_binary(var.neg_lit(), criterion_var.pos_lit());
                    }
                }
            }
        }

        Ok(criterion_vars)
    }

    fn encode_room_preferences(
        &mut self,
        _input: &Input,
        criteria: &[SatCriterion],
        violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        let mut criterion_vars = Vec::new();

        for criterion in criteria {
            if let SatCriterion::RoomPreference { section, room, .. } = criterion {
                // If we have a variable for this section-room pair
                if let Some(&var) = self.section_room_vars.get(&(*section, *room)) {
                    if !violations_permitted {
                        // Hard constraint: section must not be scheduled in this room
                        self.sat_instance.add_unit(var.neg_lit());
                    } else {
                        // Create a criterion variable for this individual section-room pair
                        let criterion_var = self.sat_instance.new_var();
                        criterion_vars.push(criterion_var);

                        // (section_in_room => criterion_var)
                        // Encoded as: (!section_in_room || criterion_var)
                        self.sat_instance.add_binary(var.neg_lit(), criterion_var.pos_lit());
                    }
                }
            }
        }

        Ok(criterion_vars)
    }

    fn encode_faculty_days_off_group(
        &mut self,
        _input: &Input,
        _criteria: &[SatCriterion],
        _violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        Ok(Vec::new())
    }

    fn encode_faculty_evenly_spread_group(
        &mut self,
        _input: &Input,
        _criteria: &[SatCriterion],
        _violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        Ok(Vec::new())
    }

    fn encode_faculty_no_room_switch_group(
        &mut self,
        _input: &Input,
        _criteria: &[SatCriterion],
        _violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        Ok(Vec::new())
    }

    fn encode_faculty_too_many_rooms_group(
        &mut self,
        _input: &Input,
        _criteria: &[SatCriterion],
        _violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        Ok(Vec::new())
    }

    fn encode_faculty_distribution_interval_group(
        &mut self,
        _input: &Input,
        _criteria: &[SatCriterion],
        _violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        Ok(Vec::new())
    }

    fn encode_different_time_patterns_group(
        &mut self,
        _input: &Input,
        _criteria: &[SatCriterion],
        _violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        Ok(Vec::new())
    }
}
