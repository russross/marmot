use super::error::Result;
use super::input::*;
use itertools::Itertools;
use rustsat::encodings::am1::{Bitwise, Commander, Encode, Pairwise};
use rustsat::instances::{BasicVarManager, Cnf, SatInstance};
use rustsat::types::constraints::CardConstraint;
use rustsat::types::{Lit, Var};
use std::collections::HashMap;
use std::iter::Iterator;

pub struct SATEncoder {
    // Core SAT variables
    pub section_time_vars: HashMap<(usize, usize), Var>, // (section, time_slot) -> var
    pub section_room_vars: HashMap<(usize, usize), Var>, // (section, room) -> var

    // Reverse lookups
    pub var_to_section_time: HashMap<Var, (usize, usize)>,
    pub var_to_section_room: HashMap<Var, (usize, usize)>,

    // Faculty time variables
    pub faculty_time_vars: HashMap<usize, FacultyTimeVars>,

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
            faculty_time_vars: HashMap::new(),
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

                //println!("using pairwise to encode at-most-1 for n={}", clause.len());
                Pairwise::from(clause).encode(&mut encoding, self.sat_instance.var_manager_mut())?;
            }

            // for at-most-1 with 10 < n ≤ 30 use bitwise (binary)
            (None | Some(1), Some(1)) if clause.len() <= 30 => {
                // for exactly-1 add the clause itself to get at-least-1
                if min.is_some() {
                    self.sat_instance.add_nary(&clause);
                }

                //println!("using bitwise to encode at-most-1 for n={}", clause.len());
                Bitwise::from(clause).encode(&mut encoding, self.sat_instance.var_manager_mut())?;
            }

            // for at-most-1 with 30 < n ≤ 100 use commander
            (None | Some(1), Some(1)) if clause.len() <= 100 => {
                // for exactly-1 add the clause itself to get at-least-1
                if min.is_some() {
                    self.sat_instance.add_nary(&clause);
                }

                //println!("using commander to encode at-most-1 for n={}", clause.len());
                Commander::<4, Pairwise>::from(clause).encode(&mut encoding, self.sat_instance.var_manager_mut())?;
            }

            // for at-least-1 we can just copy the clause in
            (Some(1), None) => self.sat_instance.add_nary(&clause),

            // for now, fall back to totalizer for all other cases
            (None, Some(k)) => {
                //println!("using totalizer for at-most-{} for n={}", k, clause.len());
                self.sat_instance.add_card_constr(CardConstraint::new_ub(clause, k));
            }
            (Some(k), None) => {
                //println!("using totalizer for at-least-{} for n={}", k, clause.len());
                self.sat_instance.add_card_constr(CardConstraint::new_lb(clause, k));
            }
            (Some(k1), Some(k2)) if k1 == k2 => {
                //println!("using totalizer for exactly-{} for n={}", k1, clause.len());
                self.sat_instance.add_card_constr(CardConstraint::new_eq(clause, k1));
            }
            _ => unimplemented!("unexpected cardinality requirement"),
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

    // Encode soft conflicts
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

    // Encode anti-conflicts
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
        input: &Input,
        criteria: &[SatCriterion],
        violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        let mut criterion_vars = Vec::new();

        // Sort criteria by time slot first
        let mut sorted_criteria = criteria.to_vec();
        sorted_criteria.sort_by_key(|c| {
            if let SatCriterion::TimeSlotPreference { time_slot, section, .. } = c {
                let faculty_count = input.sections[*section].faculty.len();
                let faculty_id = if faculty_count == 1 { input.sections[*section].faculty[0] } else { usize::MAX };
                (*time_slot, faculty_count != 1, faculty_id)
            } else {
                (usize::MAX, true, usize::MAX)
            }
        });

        // Group criteria by time slot and faculty
        for group in sorted_criteria.chunk_by(|a, b| {
            if let (
                SatCriterion::TimeSlotPreference { time_slot: ts_a, section: sec_a, .. },
                SatCriterion::TimeSlotPreference { time_slot: ts_b, section: sec_b, .. },
            ) = (a, b)
            {
                // Check if time slots match
                if ts_a != ts_b {
                    return false;
                }

                // Check faculty counts
                let faculty_count_a = input.sections[*sec_a].faculty.len();
                let faculty_count_b = input.sections[*sec_b].faculty.len();

                // Either section has != 1 faculty
                if faculty_count_a != 1 || faculty_count_b != 1 {
                    return false;
                }

                // Check if the faculty is the same
                input.sections[*sec_a].faculty[0] == input.sections[*sec_b].faculty[0]
            } else {
                false
            }
        }) {
            // Skip empty groups
            if group.is_empty() {
                continue;
            }

            // Extract first criterion to get time slot
            if let SatCriterion::TimeSlotPreference { time_slot, .. } = group[0] {
                // Collect all the sections in this group
                let sections: Vec<usize> = group
                    .iter()
                    .filter_map(|c| {
                        if let SatCriterion::TimeSlotPreference { section, .. } = c { Some(*section) } else { None }
                    })
                    .collect();

                // Collect section-time variables for this group
                let section_vars: Vec<Var> = sections
                    .iter()
                    .filter_map(|&section| self.section_time_vars.get(&(section, time_slot)).copied())
                    .collect();

                if section_vars.is_empty() {
                    continue;
                }

                // Process the group
                if !violations_permitted {
                    // Hard constraint: none of these sections can be at this time slot
                    for &var in &section_vars {
                        self.sat_instance.add_unit(var.neg_lit());
                    }
                } else {
                    // Create a single criterion variable for the entire group of sections
                    // This variable will be true if and only if the constraint is violated
                    let criterion_var = self.sat_instance.new_var();
                    criterion_vars.push(criterion_var);

                    // For each section in this group, establish the relationship:
                    // (section_at_time_slot -> criterion_var)
                    // This means: if any section is scheduled at this time slot, the criterion is violated
                    for &var in &section_vars {
                        // The implication "section_at_time_slot -> criterion_var" is encoded as:
                        // (!section_at_time_slot OR criterion_var)
                        // If section_at_time_slot is true, criterion_var must be true
                        // If section_at_time_slot is false, criterion_var can be either true or false
                        self.sat_instance.add_binary(var.neg_lit(), criterion_var.pos_lit());
                    }

                    // We also need to ensure that if NO section is scheduled at this time slot,
                    // then the criterion is NOT violated. This prevents the solver from setting
                    // criterion_var to true unnecessarily when doing optimization.
                    // Only add this clause for groups with multiple sections to avoid redundancy.
                    if section_vars.len() > 1 {
                        // The constraint is: (!criterion_var OR at_least_one_section_at_time_slot)
                        // This means: if criterion_var is false, at least one section must be scheduled
                        // at this time slot. Conversely, if all sections are NOT scheduled at this time,
                        // then criterion_var must be false.
                        let mut clause = vec![criterion_var.neg_lit()];
                        for &var in &section_vars {
                            clause.push(var.pos_lit());
                        }
                        self.sat_instance.add_nary(&clause);
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
        input: &Input,
        criteria: &[SatCriterion],
        violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        let mut criterion_vars = Vec::new();

        for criterion in criteria {
            if let SatCriterion::FacultyDaysOff { priority: _, sections, days_to_check, desired } = criterion {
                // Skip if no sections or if the days to check is empty
                if sections.is_empty() || days_to_check.is_empty() {
                    continue;
                }

                // Get the faculty index from the first section
                let faculty = match input.sections[sections[0]].faculty.first() {
                    Some(&faculty_idx) => faculty_idx,
                    None => continue, // Skip if this section has no faculty
                };

                // Collect time slot variables for each day
                let mut day_to_time_slots: Vec<Vec<Var>> = vec![Vec::new(); 7];

                {
                    let faculty_vars = self.get_faculty_time_variables(input, faculty, *days_to_check);
                    for (ts_idx, ts_var) in faculty_vars.all_time_slot_vars() {
                        for day in 0..7 {
                            if input.time_slots[ts_idx].days.contains(day) {
                                day_to_time_slots[day as usize].push(ts_var);
                            }
                        }
                    }
                }

                // Create day variables and constraints
                let mut day_vars = Vec::new();
                let relevant_days: Vec<u8> = days_to_check.into_iter().collect();

                for &day in &relevant_days {
                    let day_var = self.sat_instance.new_var();
                    day_vars.push(day_var);

                    let day_time_slots = &day_to_time_slots[day as usize];

                    if day_time_slots.is_empty() {
                        // No time slots on this day, faculty is definitely not scheduled
                        self.sat_instance.add_unit(day_var.neg_lit());
                    } else {
                        // 1. If any time slot on this day is used, the day is used
                        for &ts_var in day_time_slots {
                            // ts_var => day_var
                            self.sat_instance.add_binary(ts_var.neg_lit(), day_var.pos_lit());
                        }

                        // 2. If the day is used, at least one time slot on this day must be used
                        let mut clause = vec![day_var.neg_lit()];
                        for &ts_var in day_time_slots {
                            clause.push(ts_var.pos_lit());
                        }
                        self.sat_instance.add_nary(&clause);
                    }
                }

                // Number of days that must be scheduled = total days - desired days off
                let days_to_schedule = relevant_days.len() - desired;

                if !violations_permitted {
                    // Hard constraint: Exactly days_to_schedule days must be scheduled
                    let day_lits: Vec<Lit> = day_vars.iter().map(|&var| var.pos_lit()).collect();
                    self.encode_cardinality(day_lits, Some(days_to_schedule), Some(days_to_schedule))?;
                } else {
                    // Soft constraint: Create a criterion variable
                    let criterion_var = self.sat_instance.new_var();
                    criterion_vars.push(criterion_var);

                    // We need criterion_var to be true if and only if the number of scheduled days != days_to_schedule

                    // When desired = 0, all days must be scheduled
                    if *desired == 0 {
                        // If any day is not scheduled, the criterion is violated
                        for &day_var in &day_vars {
                            // !day_var => criterion_var
                            self.sat_instance.add_binary(day_var.neg_lit(), criterion_var.pos_lit());
                        }

                        // If all days are scheduled, criterion is not violated
                        let mut clause = Vec::new();
                        clause.push(criterion_var.neg_lit());
                        for &day_var in &day_vars {
                            clause.push(day_var.neg_lit());
                        }
                        self.sat_instance.add_nary(&clause);
                    }
                    // When desired equals all days, no days should be scheduled
                    else if *desired == relevant_days.len() {
                        // If any day is scheduled, the criterion is violated
                        for &day_var in &day_vars {
                            // day_var => criterion_var
                            self.sat_instance.add_binary(day_var.pos_lit(), criterion_var.pos_lit());
                        }

                        // If no days are scheduled, criterion is not violated
                        let mut clause = Vec::new();
                        clause.push(criterion_var.neg_lit());
                        for &day_var in &day_vars {
                            clause.push(day_var.pos_lit());
                        }
                        self.sat_instance.add_nary(&clause);
                    }
                    // General case
                    else {
                        // First handle "at least days_to_schedule": For each subset of N-k+1 days,
                        // at least one must be scheduled or criterion is violated
                        if days_to_schedule > 0 {
                            for subset in day_vars.iter().combinations(day_vars.len() - days_to_schedule + 1) {
                                let mut clause = Vec::new();
                                for &var in subset {
                                    clause.push(var.pos_lit());
                                }
                                clause.push(criterion_var.pos_lit());
                                self.sat_instance.add_nary(&clause);
                            }
                        }

                        // Then handle "at most days_to_schedule": For each subset of k+1 days,
                        // at least one must NOT be scheduled or criterion is violated
                        if days_to_schedule < day_vars.len() {
                            for subset in day_vars.iter().combinations(days_to_schedule + 1) {
                                let mut clause = Vec::new();
                                for &var in subset {
                                    clause.push(var.neg_lit());
                                }
                                clause.push(criterion_var.pos_lit());
                                self.sat_instance.add_nary(&clause);
                            }
                        }
                    }
                }
            }
        }

        Ok(criterion_vars)
    }

    fn encode_faculty_evenly_spread_group(
        &mut self,
        _input: &Input,
        _criteria: &[SatCriterion],
        _violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        //unimplemented!("encode_faculty_evenly_spread_group not implemented");
        Ok(Vec::new())
    }

    fn encode_faculty_no_room_switch_group(
        &mut self,
        _input: &Input,
        _criteria: &[SatCriterion],
        _violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        //unimplemented!("encode_faculty_no_room_switch_group not implemented");
        Ok(Vec::new())
    }

    fn encode_faculty_too_many_rooms_group(
        &mut self,
        _input: &Input,
        _criteria: &[SatCriterion],
        _violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        //unimplemented!("encode_faculty_too_many_rooms_group not implemented");
        Ok(Vec::new())
    }

    fn encode_faculty_distribution_interval_group(
        &mut self,
        _input: &Input,
        _criteria: &[SatCriterion],
        _violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        //unimplemented!("encode_faculty_distribution_interval_group not implemented");
        Ok(Vec::new())
    }

    fn encode_different_time_patterns_group(
        &mut self,
        input: &Input,
        criteria: &[SatCriterion],
        violations_permitted: bool,
    ) -> Result<Vec<Var>> {
        let mut criterion_vars = Vec::new();

        for criterion in criteria {
            if let SatCriterion::DifferentTimePatterns { sections, .. } = criterion {
                // Verify we have at least 2 sections to compare
                if sections.len() < 2 {
                    continue;
                }

                // Group time slots by pattern (num_days, duration)
                let mut time_slot_patterns: HashMap<(usize, Duration), Vec<usize>> = HashMap::new();

                // For each section, collect all its possible time slots and group them
                let mut section_time_vars: HashMap<usize, HashMap<(usize, Duration), Vec<Var>>> = HashMap::new();

                for &section in sections {
                    let mut section_patterns: HashMap<(usize, Duration), Vec<Var>> = HashMap::new();

                    // Get all time slots for this section
                    for &TimeSlotWithOptionalPriority { time_slot, .. } in &input.sections[section].time_slots {
                        // Get the pattern for this time slot
                        let time_slot_info = &input.time_slots[time_slot];
                        let pattern = (time_slot_info.days.len(), time_slot_info.duration);

                        // Add to the global pattern map
                        time_slot_patterns.entry(pattern).or_default().push(time_slot);

                        // If we have a variable for this section-time_slot, add it to the right pattern group
                        if let Some(&var) = self.section_time_vars.get(&(section, time_slot)) {
                            section_patterns.entry(pattern).or_default().push(var);
                        }
                    }

                    section_time_vars.insert(section, section_patterns);
                }

                // No patterns found or all sections only have one pattern
                if time_slot_patterns.len() <= 1 {
                    continue;
                }

                // Now we need to encode the constraint that all sections must use the same pattern

                if !violations_permitted {
                    // Hard constraint: For each pattern, create a variable representing "all sections use this pattern"
                    let mut pattern_vars: Vec<Var> = Vec::new();

                    for pattern in time_slot_patterns.keys() {
                        let pattern_var = self.sat_instance.new_var();
                        pattern_vars.push(pattern_var);

                        // For each section, if pattern_var is true, then the section must use this pattern
                        for &section in sections {
                            if let Some(section_patterns) = section_time_vars.get(&section) {
                                if let Some(vars) = section_patterns.get(pattern) {
                                    // If pattern_var is true, then section must use one of these time slots
                                    // pattern_var => (var1 OR var2 OR ...)
                                    let mut clause = vec![pattern_var.neg_lit()];
                                    for &var in vars {
                                        clause.push(var.pos_lit());
                                    }
                                    self.sat_instance.add_nary(&clause);

                                    // For each var in this pattern, var => pattern_var
                                    for &var in vars {
                                        self.sat_instance.add_binary(var.neg_lit(), pattern_var.pos_lit());
                                    }
                                } else {
                                    // This section cannot use this pattern
                                    self.sat_instance.add_unit(pattern_var.neg_lit());
                                }
                            }
                        }
                    }

                    // Exactly one pattern must be chosen
                    self.encode_cardinality(pattern_vars.iter().map(|&var| var.pos_lit()).collect(), Some(1), Some(1))?;
                } else {
                    // Soft constraint: Create a criterion variable that's true if sections have different patterns
                    let criterion_var = self.sat_instance.new_var();
                    criterion_vars.push(criterion_var);

                    // For each pair of sections, encode: If they use different patterns, criterion_var is true
                    for i in 0..sections.len() {
                        let section_a = sections[i];

                        for section_b in sections.iter().skip(i + 1) {
                            // For each pattern, get the variables for both sections
                            for pattern in time_slot_patterns.keys() {
                                let vars_a = section_time_vars
                                    .get(&section_a)
                                    .and_then(|patterns| patterns.get(pattern))
                                    .cloned()
                                    .unwrap_or_default();

                                let vars_b = section_time_vars
                                    .get(section_b)
                                    .and_then(|patterns| patterns.get(pattern))
                                    .cloned()
                                    .unwrap_or_default();

                                // Skip if either section can't use this pattern
                                if vars_a.is_empty() || vars_b.is_empty() {
                                    continue;
                                }

                                // For each var_a and var_b of different patterns
                                for &var_a in &vars_a {
                                    for other_pattern in time_slot_patterns.keys() {
                                        if pattern == other_pattern {
                                            continue;
                                        }

                                        let vars_b_other = section_time_vars
                                            .get(section_b)
                                            .and_then(|patterns| patterns.get(other_pattern))
                                            .cloned()
                                            .unwrap_or_default();

                                        for &var_b in &vars_b_other {
                                            // If var_a and var_b are both true, criterion_var must be true
                                            // (var_a AND var_b) => criterion_var
                                            self.sat_instance.add_cube_impl_lit(
                                                &[var_a.pos_lit(), var_b.pos_lit()],
                                                criterion_var.pos_lit(),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(criterion_vars)
    }

    // Get faculty time variables, creating them if they don't exist
    pub fn get_faculty_time_variables(
        &mut self,
        input: &Input,
        faculty: usize,
        days_to_check: Days,
    ) -> &FacultyTimeVars {
        // If faculty time variables already exist, return them
        if self.faculty_time_vars.contains_key(&faculty) {
            return &self.faculty_time_vars[&faculty];
        }

        // Create new faculty time variables
        let mut faculty_vars = FacultyTimeVars::new(days_to_check);
        let faculty_sections = &input.faculty[faculty].sections;

        // For each time slot that could be assigned to this faculty's sections
        let mut potential_time_slots = HashMap::new();

        // Collect all time slots for this faculty's sections
        for &section_idx in faculty_sections {
            let section = &input.sections[section_idx];
            for ts in &section.time_slots {
                // Only consider time slots on days we care about
                let time_slot = &input.time_slots[ts.time_slot];
                if !(time_slot.days.intersect(&days_to_check).is_empty()) {
                    potential_time_slots.insert(ts.time_slot, ());
                }
            }
        }

        // Create a variable for each potential time slot
        for &time_slot in potential_time_slots.keys() {
            let var = self.sat_instance.new_var();
            faculty_vars.add_time_slot_var(time_slot, var);

            // Create constraints linking this variable to section time variables
            self.link_faculty_time_var_to_sections(input, faculty, time_slot, var);
        }

        // Store and return
        self.faculty_time_vars.insert(faculty, faculty_vars);
        &self.faculty_time_vars[&faculty]
    }

    // Helper method to link a faculty time variable to section time variables
    fn link_faculty_time_var_to_sections(
        &mut self,
        input: &Input,
        faculty: usize,
        time_slot: usize,
        faculty_time_var: Var,
    ) {
        let faculty_sections = &input.faculty[faculty].sections;

        // Get all section variables for this time slot
        let mut section_vars = Vec::new();
        for &section_idx in faculty_sections {
            if let Some(&var) = self.section_time_vars.get(&(section_idx, time_slot)) {
                section_vars.push(var);
            }
        }

        if section_vars.is_empty() {
            // If no sections can be at this time, faculty can't be at this time
            self.sat_instance.add_unit(faculty_time_var.neg_lit());
            return;
        }

        // Constraint 1: If any section is at this time, faculty is at this time
        for &section_var in &section_vars {
            // section_var => faculty_time_var
            // !section_var || faculty_time_var
            self.sat_instance.add_binary(section_var.neg_lit(), faculty_time_var.pos_lit());
        }

        // Constraint 2: If faculty is at this time, at least one section is at this time
        // !faculty_time_var || (section_var_1 || section_var_2 || ...)
        let mut clause = vec![faculty_time_var.neg_lit()];
        for &section_var in &section_vars {
            clause.push(section_var.pos_lit());
        }
        self.sat_instance.add_nary(&clause);
    }
}

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

// New structure to store faculty-related time variables
#[derive(Debug)]
pub struct FacultyTimeVars {
    // Map from time slot index to the variable representing
    // whether the faculty is scheduled at that time slot
    time_slot_vars: HashMap<usize, Var>,

    // Track the days to check for this faculty
    days_to_check: Days,
}

impl FacultyTimeVars {
    // Create a new FacultyTimeVars instance for a given faculty
    pub fn new(days_to_check: Days) -> Self {
        FacultyTimeVars { time_slot_vars: HashMap::new(), days_to_check }
    }

    // Get the variable for a time slot (if it exists)
    pub fn get_time_slot_var(&self, time_slot: usize) -> Option<Var> {
        self.time_slot_vars.get(&time_slot).copied()
    }

    // Add a new time slot variable
    pub fn add_time_slot_var(&mut self, time_slot: usize, var: Var) {
        self.time_slot_vars.insert(time_slot, var);
    }

    // Get all time slot variables
    pub fn all_time_slot_vars(&self) -> Vec<(usize, Var)> {
        self.time_slot_vars.iter().map(|(&time_slot, &var)| (time_slot, var)).collect()
    }

    // Get the days to check
    pub fn days_to_check(&self) -> Days {
        self.days_to_check
    }
}
