use super::cnf::Encoding;
use super::error::{Result, err};
use super::input::*;
use super::sat_criteria::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

// Encode a single criterion into the SAT instance
pub fn encode_criterion(
    input: &Input,
    encoding: &mut Encoding,
    criterion: &SatCriterion,
    sat_criteria: &SatCriteria
) -> Result<()> {
    match criterion {
        SatCriterion::Conflict { sections, priority } => {
            encode_conflict(input, encoding, *priority, *sections)
        },
        
        SatCriterion::AntiConflict { single, group, priority } => {
            encode_anti_conflict(input, encoding, *priority, *single, group)
        },
        
        SatCriterion::RoomPreference { section, room, priority } => {
            encode_room_preference(input, encoding, *priority, *section, *room)
        },

        SatCriterion::TimeSlotPreference { section, time_slot, priority } => {
            encode_time_slot_preference(input, encoding, *priority, *section, *time_slot)
        },
        
        SatCriterion::FacultyDaysOff { faculty, days_to_check, desired_days_off, priority } => {
            encode_faculty_days_off(input, encoding, *priority, *faculty, *days_to_check, *desired_days_off)
        },
        
        SatCriterion::FacultyEvenlySpread { faculty, days_to_check, priority } => {
            encode_faculty_evenly_spread(input, encoding, *priority, *faculty, *days_to_check)
        }
        
        SatCriterion::FacultyNoRoomSwitch { faculty, days_to_check, max_gap_within_cluster, priority } => {
            encode_faculty_no_room_switch(input, encoding, *priority, *faculty, *days_to_check, *max_gap_within_cluster)
        },
        
        SatCriterion::FacultyTooManyRooms { faculty, desired_max_rooms, priority } => {
            encode_faculty_too_many_rooms(input, encoding, *priority, *faculty, *desired_max_rooms)
        },
        
        SatCriterion::FacultyClusterTooLong { faculty, days_to_check, duration, max_gap_within_cluster, priority } => {
            encode_faculty_cluster_too_long(input, encoding, *priority, *faculty, *days_to_check, *duration, *max_gap_within_cluster, sat_criteria)
        },

        SatCriterion::FacultyClusterTooShort { faculty, days_to_check, duration, max_gap_within_cluster, priority } => {
            encode_faculty_cluster_too_short(input, encoding, *priority, *faculty, *days_to_check, *duration, *max_gap_within_cluster, sat_criteria)
        },

        SatCriterion::FacultyGapTooLong { faculty, days_to_check, duration, max_gap_within_cluster, priority } => {
            encode_faculty_gap_too_long(input, encoding, *priority, *faculty, *days_to_check, *duration, *max_gap_within_cluster, sat_criteria)
        },

        SatCriterion::FacultyGapTooShort { faculty, days_to_check, duration, max_gap_within_cluster, priority } => {
            encode_faculty_gap_too_short(input, encoding, *priority, *faculty, *days_to_check, *duration, *max_gap_within_cluster, sat_criteria)
        },
        
        SatCriterion::TimePatternMatch { sections, priority } => {
            encode_time_pattern_match(input, encoding, *priority, sections)
        },
    }
}

// Encode a conflict constraint
//
// A conflict specifies that two sections cannot be scheduled at conflicting times.
// This function creates a hallpass variable and adds clauses to enforce that if
// both sections are scheduled at conflicting times, the hallpass variable must be
// true (indicating a violation that is allowed).
fn encode_conflict(
    input: &Input,
    encoding: &mut Encoding,
    priority: u8,
    sections: [usize; 2]
) -> Result<()> {
    let [section_a, section_b] = sections;
    
    // Verify sections exist and make sense
    if section_a >= input.sections.len() {
        return err(format!("Section index {} in conflict not found", section_a));
    }
    if section_b >= input.sections.len() {
        return err(format!("Section index {} in conflict not found", section_b));
    }
    if section_a == section_b {
        return err(format!("Section {} cannot conflict with itself", input.sections[section_a].name));
    }
    
    // Create a hallpass variable for this constraint
    let hallpass = encoding.new_var();
    encoding.hallpass.insert(hallpass);
    let section_a_name = &input.sections[section_a].name;
    let section_b_name = &input.sections[section_b].name;
    encoding.problems.insert(hallpass, (priority, format!("{} and {} conflict", section_a_name, section_b_name)));
    
    // Check each pair of potentially conflicting time slots
    for &TimeSlotWithOptionalPriority { time_slot: time_a, .. } in &input.sections[section_a].time_slots {
        for &TimeSlotWithOptionalPriority { time_slot: time_b, .. } in &input.sections[section_b].time_slots {
            // Skip if the time slots don't conflict
            if !input.time_slot_conflicts[time_a][time_b] {
                continue;
            }
            
            // Get variables for each section-timeslot pair
            if !encoding.section_time_vars.contains_key(&(section_a, time_a)) {
                return err(format!("Missing variable for section {}, time slot {}", section_a, time_a));
            }
            if !encoding.section_time_vars.contains_key(&(section_b, time_b)) {
                return err(format!("Missing variable for section {}, time slot {}", section_b, time_b));
            }
            
            let var_a = encoding.section_time_vars[&(section_a, time_a)];
            let var_b = encoding.section_time_vars[&(section_b, time_b)];
            
            // Encode: (var_a AND var_b) -> hallpass
            // Equivalent to: (!var_a OR !var_b OR hallpass)
            encoding.add_clause(vec![-var_a, -var_b, hallpass]);
        }
    }
    
    Ok(())
}

// Encode an anti-conflict constraint
//
// An anti-conflict specifies that a single section must be scheduled at the same time
// as at least one section from a specified group. This function creates a hallpass 
// variable and adds clauses to enforce that if the single section is scheduled at a 
// time when no group section is scheduled, the hallpass variable must be true 
// (indicating a violation that is allowed).
fn encode_anti_conflict(
    input: &Input,
    encoding: &mut Encoding,
    priority: u8,
    single: usize,
    group: &[usize]
) -> Result<()> {
    // Create a hallpass variable for this constraint
    let hallpass = encoding.new_var();
    encoding.hallpass.insert(hallpass);

    // Format the group sections for the problem message
    let mut group_names = String::new();
    let mut sep = "";
    for &section in group {
        if section >= input.sections.len() {
            return err(format!("Group section index {} in anti-conflict not found", section));
        }
        group_names.push_str(sep);
        group_names.push_str(&input.sections[section].name);
        sep = " or ";
    }

    // Add the problem to the encoding
    let single_name = &input.sections[single].name;
    encoding.problems.insert(hallpass, (priority, format!("{} should be at the same time as {}", single_name, group_names)));

    // Verify sections exist
    if single >= input.sections.len() {
        return err(format!("Single section index {} in anti-conflict not found", single));
    }
    
    if input.sections[single].time_slots.is_empty() {
        return err(format!("Single section {} has no available time slots", input.sections[single].name));
    }
    
    if group.is_empty() {
        return err(format!("Anti-conflict for {} does not have any group sections", input.sections[single].name));
    }

    // Verify at least one group section shares a time slot with the single section
    let mut has_shared_time_slot = false;
    for &TimeSlotWithOptionalPriority { time_slot: single_time, .. } in &input.sections[single].time_slots {
        for &group_section in group {
            if input.sections[group_section].time_slots.iter().any(|ts| ts.time_slot == single_time) {
                has_shared_time_slot = true;
                break;
            }
        }
        if has_shared_time_slot {
            break;
        }
    }
    
    if !has_shared_time_slot {
        return err(format!("Anti-conflict for section {} has no shared time slots with any group section", input.sections[single].name));
    }
    
    // For each time slot of the single section
    for &TimeSlotWithOptionalPriority { time_slot: single_time, .. } in &input.sections[single].time_slots {
        // The variable must exist if we've initialized correctly
        if !encoding.section_time_vars.contains_key(&(single, single_time)) {
            return err(format!("Missing variable for section {}, time slot {}", single, single_time));
        }
        let single_var = encoding.section_time_vars[&(single, single_time)];
            
        // Find group sections that share this exact time slot
        let mut group_vars = Vec::new();
        for &group_section in group {
            if input.sections[group_section].time_slots.iter().any(|ts| ts.time_slot == single_time) {
                // This variable must exist if we've initialized correctly
                if !encoding.section_time_vars.contains_key(&(group_section, single_time)) {
                    return err(format!("Missing variable for section {}, time slot {}", group_section, single_time));
                }
                group_vars.push(encoding.section_time_vars[&(group_section, single_time)]);
            }
        }
        
        // If no group sections share this time slot
        if group_vars.is_empty() {
            // Encode: single_time_var -> hallpass
            // Equivalent to: (!single_time_var | hallpass)
            encoding.add_clause(vec![-single_var, hallpass]);
        } else {
            // There are some group sections that share this time slot
            // Encode: single_time_var -> (group_var_1 | group_var_2 | ... | hallpass)
            // Equivalent to: (!single_time_var | group_var_1 | group_var_2 | ... | hallpass)
            let mut clause = vec![-single_var];
            clause.extend(group_vars);
            clause.push(hallpass);
            encoding.add_clause(clause);
        }
    }
    
    Ok(())
}

// Encode a time slot preference constraint
//
// A time slot preference specifies that a section should avoid a specific time slot
// if possible. This function creates a hallpass variable and adds clauses to
// enforce that if the section is assigned to the time slot it should avoid,
// the hallpass variable must be true (indicating a violation that is allowed).
fn encode_time_slot_preference(
    input: &Input,
    encoding: &mut Encoding,
    priority: u8,
    section: usize,
    time_slot: usize
) -> Result<()> {
    // Create a hallpass variable for this constraint
    let hallpass = encoding.new_var();
    encoding.hallpass.insert(hallpass);
    
    // Add the problem to the encoding
    let section_name = &input.sections[section].name;
    let time_slot_name = &input.time_slots[time_slot].name;
    encoding.problems.insert(hallpass, (priority, format!("{} should not be at {}", section_name, time_slot_name)));

    // Verify section and time slot exist
    if section >= input.sections.len() {
        return err(format!("Section index {} in time preference not found", section));
    }
    if time_slot >= input.time_slots.len() {
        return err(format!("Time slot index {} in time preference not found", time_slot));
    }
    
    // Verify section could be assigned this time slot
    if !input.sections[section].time_slots.iter().any(|ts| ts.time_slot == time_slot) {
        return err(format!("Time slot {} is not available for section {}", 
            input.time_slots[time_slot].name, input.sections[section].name));
    }
    
    // The section-time variable must exist if we've initialized correctly
    if !encoding.section_time_vars.contains_key(&(section, time_slot)) {
        return err(format!("Missing variable for section {}, time slot {}", section, time_slot));
    }
    
    let time_var = encoding.section_time_vars[&(section, time_slot)];
    
    // Encode: time_var -> hallpass
    // Equivalent to: (!time_var OR hallpass)
    encoding.add_clause(vec![-time_var, hallpass]);
    
    Ok(())
}

// Encode a room preference constraint
//
// A room preference specifies that a section should avoid a specific room
// if possible. This function creates a hallpass variable and adds clauses to
// enforce that if the section is assigned to the room it should avoid,
// the hallpass variable must be true (indicating a violation that is allowed).
fn encode_room_preference(
    input: &Input,
    encoding: &mut Encoding,
    priority: u8,
    section: usize,
    room: usize
) -> Result<()> {
    // Create a hallpass variable for this constraint
    let hallpass = encoding.new_var();
    encoding.hallpass.insert(hallpass);
    
    // Add the problem to the encoding
    let section_name = &input.sections[section].name;
    let room_name = &input.rooms[room].name;
    encoding.problems.insert(hallpass, (priority, format!("{} should not be in {}", section_name, room_name)));

    // Verify section and room exist
    if section >= input.sections.len() {
        return err(format!("Section index {} in room preference not found", section));
    }
    if room >= input.rooms.len() {
        return err(format!("Room index {} in room preference not found", room));
    }
    
    // Verify section could be assigned this room
    if !input.sections[section].rooms.iter().any(|r| r.room == room) {
        return err(format!("Room {} is not available for section {}", 
            input.rooms[room].name, input.sections[section].name));
    }
    
    // The section-room variable must exist if we've initialized correctly
    if !encoding.section_room_vars.contains_key(&(section, room)) {
        return err(format!("Missing variable for section {}, room {}", section, room));
    }
    
    let room_var = encoding.section_room_vars[&(section, room)];
    
    // Encode: room_var -> hallpass
    // Equivalent to: (!room_var OR hallpass)
    encoding.add_clause(vec![-room_var, hallpass]);
    
    Ok(())
}


// A time pattern is defined by the number of days and the duration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pattern {
    days_count: usize,
    duration: Duration,
}

// Create pattern variables representing "at least one section uses this pattern"
//
// For each distinct time pattern found across all sections, creates a variable that
// will be true if and only if at least one of the specified sections is assigned
// to a time slot with that pattern. Adds clauses connecting section-time variables
// to pattern variables.
fn make_section_pattern_vars(
    input: &Input,
    encoding: &mut Encoding,
    sections: &[usize]
) -> Result<HashMap<Pattern, i32>> {
    // Maps patterns to their variables
    let mut pattern_to_var: HashMap<Pattern, i32> = HashMap::new();
    
    // Maps patterns to all section time variables with that pattern
    let mut pattern_to_time_vars: HashMap<Pattern, Vec<i32>> = HashMap::new();
    
    // Scan all sections and their time slots
    for &section in sections {
        if section >= input.sections.len() {
            return err(format!("Section index {} not found", section));
        }
        
        for &TimeSlotWithOptionalPriority { time_slot, .. } in &input.sections[section].time_slots {
            // Verify the section-time slot variable exists
            if !encoding.section_time_vars.contains_key(&(section, time_slot)) {
                return err(format!("Missing variable for section {}, time slot {}", section, time_slot));
            }
                
            let time_slot_data = &input.time_slots[time_slot];
            let pattern = Pattern {
                days_count: time_slot_data.days.len(),
                duration: time_slot_data.duration,
            };
            
            // Get or create a variable for this pattern
            if !pattern_to_var.contains_key(&pattern) {
                let var = encoding.new_var();
                pattern_to_var.insert(pattern, var);
                pattern_to_time_vars.insert(pattern, Vec::new());
            }
            
            // Add this section-time variable to the pattern's list
            let time_var = encoding.section_time_vars[&(section, time_slot)];
            pattern_to_time_vars.get_mut(&pattern).unwrap().push(time_var);
        }
    }
    
    // Add clauses connecting time slot variables to pattern variables
    for (pattern, &pattern_var) in &pattern_to_var {
        let time_vars = &pattern_to_time_vars[pattern];
        
        // For each section-time variable with this pattern:
        // time_var → pattern_var
        // Equivalent to: !time_var OR pattern_var
        for &time_var in time_vars {
            encoding.add_clause(vec![-time_var, pattern_var]);
        }
        
        // pattern_var → (time_var_1 OR time_var_2 OR ...)
        // Equivalent to: !pattern_var OR time_var_1 OR time_var_2 OR ...
        if !time_vars.is_empty() {
            let mut clause = vec![-pattern_var];
            clause.extend_from_slice(time_vars);
            encoding.add_clause(clause);
        }
    }
    
    Ok(pattern_to_var)
}

// Encode a time pattern match constraint
//
// A time pattern match constraint specifies that all sections in a group should
// have the same time pattern (number of days per week, duration). This function
// creates a hallpass variable and adds clauses to enforce that if sections in the 
// constraint group are assigned different time patterns, the hallpass variable must
// be true (indicating a violation that is allowed).
fn encode_time_pattern_match(
    input: &Input,
    encoding: &mut Encoding,
    priority: u8,
    sections: &[usize]
) -> Result<()> {
    // Skip if fewer than 2 sections (constraint is trivially satisfied)
    if sections.len() < 2 {
        eprintln!("Warning: TimePatternMatch constraint has fewer than 2 sections: {:?}", sections);
        return Ok(());
    }

    // Create the hallpass variable
    let hallpass = encoding.new_var();
    encoding.hallpass.insert(hallpass);
    
    // Format section names for the problem message
    let mut section_names = String::new();
    let mut sep = "";
    for &section in sections {
        if section >= input.sections.len() {
            return err(format!("Section index {} in time pattern match not found", section));
        }
        section_names.push_str(sep);
        section_names.push_str(&input.sections[section].name);
        sep = " and ";
    }
    
    encoding.problems.insert(hallpass, (priority, format!("{} should have the same time pattern", section_names)));
    
    // Create pattern variables (no hallpass involvement at this stage)
    let pattern_to_var = make_section_pattern_vars(input, encoding, sections)?;
    
    // If no patterns or only one pattern, constraint is trivially satisfied
    if pattern_to_var.len() <= 1 {
        return err(format!("TimePatternMatch constraint trivial with fewer than 2 patterns for sections: {:?}", sections));
    }
    
    // Get the list of pattern variables
    let pattern_vars: Vec<i32> = pattern_to_var.values().copied().collect();
    
    // We need to ensure exactly one pattern variable is true, or the hallpass is true
    // This is a "exactly one of N" constraint with a hallpass
    
    // First, ensure at least one pattern is used
    let mut at_least_one_clause = pattern_vars.clone();
    at_least_one_clause.push(hallpass);
    encoding.add_clause(at_least_one_clause);
    
    // Then, ensure at most one pattern is used (or hallpass is true)
    for i in 0..pattern_vars.len() {
        for j in (i+1)..pattern_vars.len() {
            // Can't have both pattern i and pattern j without hallpass
            // !pattern_i OR !pattern_j OR hallpass
            encoding.add_clause(vec![-pattern_vars[i], -pattern_vars[j], hallpass]);
        }
    }
    
    Ok(())
}

// Encode a faculty days off constraint
//
// A faculty days off constraint specifies that a faculty member should have a 
// specific number of days without classes. This function creates a hallpass variable
// and adds clauses to enforce that if the faculty member's schedule doesn't have
// the desired number of days off, the hallpass variable must be true.
fn encode_faculty_days_off(
    input: &Input,
    encoding: &mut Encoding,
    priority: u8,
    faculty: usize,
    days_to_check: Days,
    desired_days_off: usize
) -> Result<()> {
    // Create a hallpass variable for this constraint
    let hallpass = encoding.new_var();
    encoding.hallpass.insert(hallpass);
    
    // Add the problem to the encoding
    let faculty_name = &input.faculty[faculty].name;
    let days_suffix = if desired_days_off == 1 { "" } else { "s" };
    encoding.problems.insert(hallpass, (priority, 
        format!("{} wants {} day{} off", faculty_name, desired_days_off, days_suffix)));

    // Validate inputs
    if faculty >= input.faculty.len() {
        return err(format!("Faculty index {} not found in input", faculty));
    }
    if input.faculty[faculty].sections.len() <= 1 {
        return err(format!("Faculty {} must have multiple sections with a days off constraint", 
            input.faculty[faculty].name));
    }
    if days_to_check.is_empty() {
        return err(format!("Empty days_to_check for faculty {}", input.faculty[faculty].name));
    }
    if desired_days_off > days_to_check.len() {
        return err(format!(
            "Desired days off {} exceeds possible days {} for faculty {}",
            desired_days_off, days_to_check.len(), input.faculty[faculty].name
        ));
    }

    // Get day auxiliary variables: day -> variable
    let day_to_var = make_faculty_day_vars(input, encoding, faculty, &days_to_check)?;
    let days_list: Vec<u8> = days_to_check.into_iter().collect();

    // Generate a truth table of all possible day combinations
    // For n days, there are 2^n possible combinations
    let num_days = days_list.len();
    let num_combinations = 1 << num_days;
    
    // Iterate through all possible combinations
    for combo_idx in 0..num_combinations {
        // Convert the index to a binary pattern of days
        // For example, with 3 days, combo_idx=5 (binary 101) means
        // days[0] is scheduled, days[1] is not scheduled, days[2] is scheduled
        let mut day_pattern = Vec::with_capacity(num_days);
        for i in 0..num_days {
            day_pattern.push(((combo_idx >> i) & 1) == 1);
        }
        
        // Count days off in this pattern
        let days_off = day_pattern.iter().filter(|&&is_scheduled| !is_scheduled).count();
        
        // If this pattern has the correct number of days off, we're good
        if days_off == desired_days_off {
            continue;
        }
            
        // Encode that this pattern should not happen without a hallpass
        let mut clause = vec![hallpass];
        for i in 0..num_days {
            let day = days_list[i];
            let var = day_to_var[&day];
            
            // If the day is scheduled in this pattern, add -var to forbid it
            // If the day is not scheduled in this pattern, add var to forbid it
            if day_pattern[i] {
                clause.push(-var);
            } else {
                clause.push(var);
            }
        }
                
        encoding.add_clause(clause);
    }
    
    Ok(())
}

// Create variables that represent whether a faculty member teaches on specific days.
//
// For each day in days_to_check, creates a variable that will be true if and only if
// at least one of the faculty member's sections is scheduled on that day.
fn make_faculty_day_vars(
    input: &Input,
    encoding: &mut Encoding,
    faculty: usize,
    days_to_check: &Days
) -> Result<HashMap<u8, i32>> {
    // Create the map of day variables we'll return
    let mut day_to_var: HashMap<u8, i32> = HashMap::new();
    
    // Create mappings to help with encoding
    let mut var_to_section_time_vars: HashMap<i32, Vec<i32>> = HashMap::new();
    
    // Initialize day variables
    for day in days_to_check.into_iter() {
        let var = encoding.new_var();
        day_to_var.insert(day, var);
        var_to_section_time_vars.insert(var, Vec::new());
    }
    
    // For each section taught by this faculty
    for &section in &input.faculty[faculty].sections {
        // For each day of interest
        for day in days_to_check.into_iter() {
            // For each time slot available to this section
            for &TimeSlotWithOptionalPriority { time_slot, .. } in &input.sections[section].time_slots {
                let time_slot_days = &input.time_slots[time_slot].days;
                
                // Only if this time slot covers this day
                if !time_slot_days.contains(day) {
                    continue;
                }
                
                // Get the variable for this section-time pair
                // It must exist if we've initialized correctly
                if !encoding.section_time_vars.contains_key(&(section, time_slot)) {
                    return err(format!("Missing variable for section {}, time slot {}", section, time_slot));
                }
                
                // Record this for encoding
                let time_var = encoding.section_time_vars[&(section, time_slot)];
                if let Some(vars) = var_to_section_time_vars.get_mut(&day_to_var[&day]) {
                    vars.push(time_var);
                }
            }
        }
    }
    
    // Add the clauses for each day variable
    for (&day_var, section_time_vars) in &var_to_section_time_vars {
        if section_time_vars.is_empty() {
            // If there are no possible section-time assignments for this day,
            // this variable must be false
            encoding.add_clause(vec![-day_var]);
            continue;
        }
        
        // Encode day_var -> (time_slot_1 OR time_slot_2 OR ...)
        // i.e. !day_var OR time_slot_1 OR time_slot_2 OR ...
        let mut clause = vec![-day_var];
        clause.extend(section_time_vars.iter());
        encoding.add_clause(clause);
        
        // Encode: (any of the time slots) -> day_var
        // i.e.: (!time_slot_1 AND !time_slot_2 AND ...) OR day_var
        // i.e.: (!time_slot_1 OR day_var) AND (!time_slot_2 OR day_var) AND ...
        for &time_var in section_time_vars {
            encoding.add_clause(vec![-time_var, day_var]);
        }
    }
    
    Ok(day_to_var)
}

// Helper functions for faculty time cluster constraints

// Create variables that represent when a faculty member is scheduled in specific time slots.
//
// Creates variables for each unique time slot that could be assigned to any of the faculty member's
// sections, but only for time slots that meet on at least one of the days in days_to_check.
pub fn make_faculty_time_slot_vars(
    input: &Input,
    encoding: &mut Encoding,
    faculty: usize,
    days_to_check: Days
) -> Result<HashMap<usize, i32>> {
    // Create the map of time slot vars we'll return
    let mut time_slot_to_var: HashMap<usize, i32> = HashMap::new();
    
    // Create mappings to help with encoding
    let mut var_to_section_time_vars: HashMap<i32, Vec<i32>> = HashMap::new();
    
    // For each section taught by this faculty
    for &section in &input.faculty[faculty].sections {
        // For each time slot available to this section
        for &TimeSlotWithOptionalPriority { time_slot, .. } in &input.sections[section].time_slots {
            let time_slot_days = input.time_slots[time_slot].days;
            
            // Only consider time slots that meet on at least one day in days_to_check
            if !days_to_check.has_common_day(&time_slot_days) {
                continue;
            }
            
            // Get or create a variable for this time slot
            if !time_slot_to_var.contains_key(&time_slot) {
                let var = encoding.new_var();
                time_slot_to_var.insert(time_slot, var);
                var_to_section_time_vars.insert(var, Vec::new());
            }
            
            // Get the section-time variable
            if !encoding.section_time_vars.contains_key(&(section, time_slot)) {
                return err(format!("Missing variable for section {}, time slot {}", section, time_slot));
            }
            
            // Record section-time variable for encoding
            let time_var = encoding.section_time_vars[&(section, time_slot)];
            var_to_section_time_vars.get_mut(&time_slot_to_var[&time_slot]).unwrap().push(time_var);
        }
    }
    
    // Encode the constraints
    for (var, section_time_vars) in var_to_section_time_vars {
        if section_time_vars.is_empty() {
            continue;
        }
        
        // Encode var -> (section1_time OR section2_time OR ...)
        // i.e., !var OR section1_time OR section2_time OR ...
        let mut clause = vec![-var];
        clause.extend(&section_time_vars);
        encoding.add_clause(clause);
        
        // Encode: (any of the section-time assignments) -> var
        // i.e., (!section1_time AND !section2_time AND ...) OR faculty_time_var
        // i.e., (!section1_time OR var) AND (!section2_time OR var) AND ...
        for &section_time_var in &section_time_vars {
            encoding.add_clause(vec![-section_time_var, var]);
        }
    }
    
    Ok(time_slot_to_var)
}

// Find clusters of time slots on a specific day.
//
// A cluster is a group of time slots that are close together in time (the gap between
// adjacent time slots is <= max_gap_within_cluster).
fn get_time_slot_clusters(
    input: &Input,
    time_slots: &[usize],
    day: u8,
    max_gap_within_cluster: Duration
) -> Result<Vec<(Time, Time)>> {
    // Filter time slots to only those that include the specified day
    let mut day_intervals = Vec::new();
    
    for &ts in time_slots {
        if ts >= input.time_slots.len() {
            return err(format!("Time slot index {} out of bounds", ts));
        }
        
        let time_slot = &input.time_slots[ts];
        
        // Skip if this time slot doesn't include the specified day
        if !time_slot.days.contains(day) {
            continue;
        }
        
        let start_time = time_slot.start_time;
        let end_time = start_time + time_slot.duration;
        
        day_intervals.push((start_time, end_time));
    }
    
    // Sort intervals by start time
    day_intervals.sort_by_key(|&(start, _)| start);
    
    // Check for overlapping intervals
    for i in 1..day_intervals.len() {
        let prev_end = day_intervals[i-1].1;
        let curr_start = day_intervals[i].0;
        
        if curr_start.minutes < prev_end.minutes {
            let (a, b) = day_intervals[i-1];
            let (c, d) = day_intervals[i];
            return err(format!("Overlapping time slots detected on day {}: {}–{} and {}–{}", day, a, b, c, d));
        }
    }
    
    // If no intervals, return empty list
    if day_intervals.is_empty() {
        return Ok(Vec::new());
    }
    
    // Merge intervals that are close together
    let mut clusters = Vec::new();
    let mut current_cluster_start = day_intervals[0].0;
    let mut current_cluster_end = day_intervals[0].1;
    
    for i in 1..day_intervals.len() {
        let (current_start, current_end) = day_intervals[i];
        
        // Calculate gap
        let gap = Duration::new(current_start.minutes.saturating_sub(current_cluster_end.minutes));
        
        // Compare directly 
        if gap.minutes <= max_gap_within_cluster.minutes {
            // If gap is small enough, extend the current cluster
            current_cluster_end = current_end;
        } else {
            // Gap is too large, start a new cluster
            clusters.push((current_cluster_start, current_cluster_end));
            current_cluster_start = current_start;
            current_cluster_end = current_end;
        }
    }
    
    // Add the final cluster
    clusters.push((current_cluster_start, current_cluster_end));
    
    Ok(clusters)
}

// Generate all possible faculty teaching patterns with day-specific time clusters.
//
// For each day in days_to_check, this function identifies possible combinations
// of time slots that could be assigned to the faculty on that day, along with
// the resulting time clusters for each combination.
//
// Returns a list of (day, pattern, clusters) tuples where:
// - day: A specific day from days_to_check
// - pattern: A list of (time_slot, is_used) pairs for the relevant time slots
// - clusters: A list of (start_time, end_time) tuples representing teaching clusters
fn get_faculty_day_patterns(
    input: &Input,
    faculty: usize,
    days_to_check: Days,
    max_gap_within_cluster: Duration,
    faculty_time_slot_vars: &HashMap<usize, i32>
) -> Result<Vec<(u8, Vec<(usize, bool)>, Vec<(Time, Time)>)>> {
    // Validate inputs
    if faculty >= input.faculty.len() {
        return err(format!("Faculty index {} out of bounds", faculty));
    }
    if days_to_check.is_empty() {
        return err(format!("Empty days_to_check for faculty {}", input.faculty[faculty].name));
    }
    if input.faculty[faculty].sections.len() <= 1 {
        return err(format!("Faculty {} must have multiple sections for cluster constraints", 
                          input.faculty[faculty].name));
    }
    
    let mut result = Vec::new();
    
    // Get all potential time slots for this faculty across all relevant days
    let potential_time_slots: HashSet<usize> = faculty_time_slot_vars.keys().cloned().collect();
    
    // Create a mapping of sections to their possible time slots
    let mut section_to_time_slots: HashMap<usize, Vec<usize>> = HashMap::new();
    for &section in &input.faculty[faculty].sections {
        let time_slots: Vec<usize> = input.sections[section].time_slots.iter()
            .map(|ts| ts.time_slot)
            .filter(|&ts| potential_time_slots.contains(&ts))
            .collect();
        
        if !time_slots.is_empty() {
            section_to_time_slots.insert(section, time_slots);
        }
    }
    
    // Process each day
    for day in days_to_check.into_iter() {
        // Get time slots that could be used on this day
        let day_time_slots: Vec<usize> = potential_time_slots.iter()
            .filter(|&&ts| input.time_slots[ts].days.contains(day))
            .cloned()
            .collect();
        
        if day_time_slots.is_empty() {
            continue;
        }
        
        // For this day, we'll generate all possible combinations of time slots being used/unused
        // Start with all time slots unused
        let mut pattern = Vec::new();
        for &ts in &day_time_slots {
            pattern.push((ts, false));
        }
        
        // Function to recursively generate all possible patterns
        fn generate_patterns(
            input: &Input,
            pattern: &mut Vec<(usize, bool)>,
            index: usize,
            faculty: usize,
            day: u8,
            max_gap: Duration,
            result: &mut Vec<(u8, Vec<(usize, bool)>, Vec<(Time, Time)>)>
        ) -> Result<()> {
            if index >= pattern.len() {
                // We have a complete pattern
                // Extract the used time slots
                let used_time_slots: Vec<usize> = pattern.iter()
                    .filter_map(|&(ts, used)| if used { Some(ts) } else { None })
                    .collect();
                
                // Skip if no time slots are used
                if used_time_slots.is_empty() {
                    return Ok(());
                }
                
                // Check if this is a valid pattern (no time conflicts)
                let mut is_valid = true;
                'outer: for i in 0..used_time_slots.len() {
                    for j in (i+1)..used_time_slots.len() {
                        if input.time_slot_conflicts[used_time_slots[i]][used_time_slots[j]] {
                            is_valid = false;
                            break 'outer;
                        }
                    }
                }
                
                if !is_valid {
                    return Ok(());
                }
                
                // Get the clusters for this pattern
                let clusters = get_time_slot_clusters(input, &used_time_slots, day, max_gap)?;
                
                // Add to result
                result.push((day, pattern.clone(), clusters));
                return Ok(());
            }
            
            // Try with this time slot unused
            pattern[index].1 = false;
            generate_patterns(input, pattern, index + 1, faculty, day, max_gap, result)?;
            
            // Try with this time slot used
            pattern[index].1 = true;
            generate_patterns(input, pattern, index + 1, faculty, day, max_gap, result)?;
            
            Ok(())
        }
        
        // Generate patterns for this day
        generate_patterns(input, &mut pattern, 0, faculty, day, max_gap_within_cluster, &mut result)?;
    }
    
    Ok(result)
}

// Helper function for encoding faculty cluster constraints.
//
// This function handles the common structure of faculty cluster constraints,
// using callback functions to determine violations based on specific criteria
// and generate appropriate descriptions.
pub fn encode_faculty_cluster_helper(
    input: &Input,
    encoding: &mut Encoding,
    faculty: usize,
    days_to_check: Days,
    max_gap_within_cluster: Duration,
    priority: u8,
    violation_counter: impl Fn(&[(Time, Time)], u8) -> usize,
    description_generator: impl Fn(usize, u8) -> String
) -> Result<()> {
    // Validate inputs
    if faculty >= input.faculty.len() {
        return err(format!("Faculty index {} out of bounds", faculty));
    }
    if days_to_check.is_empty() {
        return err(format!("Empty days_to_check for faculty {}", input.faculty[faculty].name));
    }
    if input.faculty[faculty].sections.len() <= 1 {
        return err(format!("Faculty {} must have multiple sections for cluster constraints", 
                          input.faculty[faculty].name));
    }
    
    // Create faculty time slot variables for each day
    let faculty_time_slot_vars = make_faculty_time_slot_vars(input, encoding, faculty, days_to_check)?;
    
    // Skip if no time slots are available for this faculty
    if faculty_time_slot_vars.is_empty() {
        return Ok(());
    }
    
    // Dictionary to store hallpass variables: (day, violation_number) -> hallpass_var
    let mut hallpass_vars: HashMap<(u8, usize), i32> = HashMap::new();
    
    // Local helper function to get or create hallpass variables
    fn get_or_create_hallpass(
        encoding: &mut Encoding,
        hallpass_vars: &mut HashMap<(u8, usize), i32>,
        day: u8, 
        n: usize,
        priority: u8,
        faculty_name: &str,
        description_generator: &impl Fn(usize, u8) -> String
    ) -> i32 {
        if let Some(&var) = hallpass_vars.get(&(day, n)) {
            return var;
        }

        let hallpass = encoding.new_var();
        encoding.hallpass.insert(hallpass);

        let description = description_generator(n, day);
        encoding.problems.insert(hallpass, (priority, format!("{} {}", faculty_name, description)));

        hallpass_vars.insert((day, n), hallpass);
        hallpass
    }
    
    // Get all potential day patterns with clusters
    let day_patterns = get_faculty_day_patterns(input, faculty, days_to_check, max_gap_within_cluster, &faculty_time_slot_vars)?;
    
    // Process each day pattern
    for (day, pattern, clusters) in day_patterns {
        // Use the provided callback to determine violations
        let violation_count = violation_counter(&clusters, day);
        
        // Skip if no violations in this pattern
        if violation_count == 0 {
            continue;
        }
        
        // Build the base clause (the pattern literals)
        let mut base_clause = Vec::new();
        
        // For each (time_slot, is_used) pair in the pattern
        for &(time_slot, is_used) in &pattern {
            // Skip if this time slot isn't relevant to this faculty on this day
            if !faculty_time_slot_vars.contains_key(&time_slot) {
                continue;
            }
            
            let var = faculty_time_slot_vars[&time_slot];
            
            // Add the appropriate literal to the base clause
            if is_used {
                // If the time slot is used in this pattern, add !var to clause
                base_clause.push(-var);
            } else {
                // If the time slot is not used in this pattern, add var to clause
                base_clause.push(var);
            }
        }
        
        // For each violation in this pattern, create a clause
        for i in 1..=violation_count {
            // Get the appropriate hallpass variable
            let hallpass = get_or_create_hallpass(
                encoding,
                &mut hallpass_vars,
                day,
                i,
                priority,
                &input.faculty[faculty].name,
                &description_generator
            );
            
            // Create a clause with the base literals plus this hallpass
            let mut clause = base_clause.clone();
            clause.push(hallpass);
            
            // Add the clause to the encoding
            encoding.add_clause(clause);
        }
    }
    
    Ok(())
}

// Extension trait to add common day operations
trait DaysExt {
    fn has_common_day(&self, other: &Days) -> bool;
}

impl DaysExt for Days {
    fn has_common_day(&self, other: &Days) -> bool {
        (self.days & other.days) != 0
    }
}

// Faculty Cluster and Gap constraint encoders

// Encode a faculty cluster too long constraint.
//
// A faculty cluster too long constraint specifies that a faculty member should not
// have continuous teaching blocks (clusters) that exceed a specified duration.
pub fn encode_faculty_cluster_too_long(
    input: &Input,
    encoding: &mut Encoding,
    priority: u8,
    faculty: usize,
    days_to_check: Days,
    max_duration: Duration,
    max_gap_within_cluster: Duration,
    sat_criteria: &SatCriteria
) -> Result<()> {
    // Find higher-priority cluster-too-long constraints for this faculty
    let mut higher_priority_durations: Vec<Duration> = Vec::new();
    
    // Check each priority level below the current one
    for p in 0..priority {
        // Get criteria at this priority level
        for criterion in sat_criteria.criteria_at_priority(p) {
            if let SatCriterion::FacultyClusterTooLong { faculty: f, duration, .. } = criterion {
                if *f == faculty {
                    higher_priority_durations.push(*duration);
                }
            }
        }
    }
    
    // Validate specific inputs for this constraint type
    if max_duration.minutes == 0 {
        return err(format!("Non-positive maximum duration for faculty {}", input.faculty[faculty].name));
    }

    // Create a function that detects "too long" clusters, only counting those
    // that aren't already covered by higher-priority constraints
    let count_too_long_clusters = move |clusters: &[(Time, Time)], _day: u8| -> usize {
        let mut violation_count = 0;
        
        for &(start_time, end_time) in clusters {
            // Calculate the duration of this cluster
            let cluster_duration = end_time - start_time;
            
            // Check if this cluster exceeds the maximum allowed duration for this constraint
            if cluster_duration > max_duration {
                // Now check if it's already caught by a higher-priority constraint
                let already_caught = higher_priority_durations.iter()
                    .any(|&higher_dur| cluster_duration > higher_dur);
                
                // Only count as a violation if not already caught by higher priority constraint
                if !already_caught {
                    violation_count += 1;
                }
            }
        }
        
        violation_count
    };
    
    // Create a function that generates descriptions for violations
    let generate_too_long_description = move |i: usize, day: u8| -> String {
        // Convert day number to name for readability
        let day_name = match day {
            0 => "Monday",
            1 => "Tuesday",
            2 => "Wednesday",
            3 => "Thursday",
            4 => "Friday",
            5 => "Saturday",
            6 => "Sunday",
            _ => "Unknown day",
        };
        
        if i == 1 {
            format!("has a teaching cluster longer than {} on {}", max_duration, day_name)
        } else if i == 2 {
            format!("has a 2nd long teaching cluster on {}", day_name)
        } else if i == 3 {
            format!("has a 3rd long teaching cluster on {}", day_name)
        } else {
            format!("has a {}th long teaching cluster on {}", i, day_name)
        }
    };
    
    // Use the helper function to handle the encoding
    encode_faculty_cluster_helper(
        input,
        encoding,
        faculty,
        days_to_check,
        max_gap_within_cluster,
        priority,
        count_too_long_clusters,
        generate_too_long_description
    )
}

// Encode a faculty cluster too short constraint.
//
// A faculty cluster too short constraint specifies that a faculty member should not
// have continuous teaching blocks (clusters) that are shorter than a specified duration.
// The first "too short" cluster per day is allowed without penalty.
pub fn encode_faculty_cluster_too_short(
    input: &Input,
    encoding: &mut Encoding,
    priority: u8,
    faculty: usize,
    days_to_check: Days,
    min_duration: Duration,
    max_gap_within_cluster: Duration,
    sat_criteria: &SatCriteria
) -> Result<()> {
    // Find higher-priority cluster-too-short constraints for this faculty
    let mut higher_priority_durations: Vec<Duration> = Vec::new();
    
    // Check each priority level below the current one
    for p in 0..priority {
        // Get criteria at this priority level
        for criterion in sat_criteria.criteria_at_priority(p) {
            if let SatCriterion::FacultyClusterTooShort { faculty: f, duration, .. } = criterion {
                if *f == faculty {
                    higher_priority_durations.push(*duration);
                }
            }
        }
    }
    
    // Validate specific inputs for this constraint type
    if min_duration.minutes == 0 {
        return err(format!("Non-positive minimum duration for faculty {}", input.faculty[faculty].name));
    }

    // Create a function that detects "too short" clusters, allowing the first one,
    // and only counting those that aren't already covered by higher-priority constraints
    let count_too_short_clusters = move |clusters: &[(Time, Time)], _day: u8| -> usize {
        let mut violation_count = 0;
        
        for &(start_time, end_time) in clusters {
            // Calculate the duration of this cluster
            let cluster_duration = end_time - start_time;
            
            // Check if this cluster is shorter than the minimum allowed duration
            if cluster_duration < min_duration {
                // Check if it's already caught by a higher-priority constraint
                // For "too short" constraints, a higher priority constraint would have a LARGER minimum
                let already_caught = higher_priority_durations.iter()
                    .any(|&higher_dur| cluster_duration < higher_dur);
                
                // Only count as a violation if not already caught by higher priority constraint
                if !already_caught {
                    violation_count += 1;
                }
            }
        }
        
        // Important difference from "too long": 
        // First "too short" cluster per day is allowed without penalty
        if violation_count > 0 {
            violation_count - 1
        } else {
            0
        }
    };
    
    // Create a function that generates descriptions for violations
    let generate_too_short_description = move |i: usize, day: u8| -> String {
        // Convert day number to name for readability
        let day_name = match day {
            0 => "Monday",
            1 => "Tuesday",
            2 => "Wednesday",
            3 => "Thursday",
            4 => "Friday",
            5 => "Saturday",
            6 => "Sunday",
            _ => "Unknown day",
        };
        
        // Note: For "too short", i starts at 1 but describes the 2nd violation on that day
        // because the first violation is allowed without penalty
        if i == 1 {
            format!("has a teaching cluster shorter than {} on {}", min_duration, day_name)
        } else if i == 2 {
            format!("has a 2nd short teaching cluster on {}", day_name)
        } else if i == 3 {
            format!("has a 3rd short teaching cluster on {}", day_name)
        } else {
            format!("has a {}th short teaching cluster on {}", i+1, day_name)
        }
    };
    
    // Use the helper function to handle the encoding
    encode_faculty_cluster_helper(
        input,
        encoding,
        faculty,
        days_to_check,
        max_gap_within_cluster,
        priority,
        count_too_short_clusters,
        generate_too_short_description
    )
}

// Encode a faculty gap too long constraint.
//
// A faculty gap too long constraint specifies that a faculty member should not
// have gaps between teaching clusters that exceed a specified duration.
pub fn encode_faculty_gap_too_long(
    input: &Input,
    encoding: &mut Encoding,
    priority: u8,
    faculty: usize,
    days_to_check: Days,
    max_gap_duration: Duration,
    max_gap_within_cluster: Duration,
    sat_criteria: &SatCriteria
) -> Result<()> {
    // Find higher-priority gap-too-long constraints for this faculty
    let mut higher_priority_durations: Vec<Duration> = Vec::new();

    // Check each priority level below the current one
    for p in 0..priority {
        // Get criteria at this priority level
        for criterion in sat_criteria.criteria_at_priority(p) {
            if let SatCriterion::FacultyGapTooLong { faculty: f, duration, .. } = criterion {
                if *f == faculty {
                    higher_priority_durations.push(*duration);
                }
            }
        }
    }
    
    // Validate specific inputs for this constraint type
    if max_gap_duration.minutes == 0 {
        return err(format!("Non-positive maximum gap duration for faculty {}", input.faculty[faculty].name));
    }

    // Create a function that detects "too long" gaps
    let count_too_long_gaps = move |clusters: &[(Time, Time)], _day: u8| -> usize {
        let mut violation_count = 0;
        
        // Need at least 2 clusters to have a gap
        if clusters.len() < 2 {
            return 0;
        }
        
        // Check each gap between clusters
        for i in 1..clusters.len() {
            let prev_end_time = clusters[i-1].1;
            let curr_start_time = clusters[i].0;
            
            // Calculate the gap duration
            let gap_duration = curr_start_time - prev_end_time;
            
            // Check if this gap exceeds the maximum allowed duration
            if gap_duration > max_gap_duration {
                // Check if it's already caught by a higher-priority constraint
                // For "too long" gaps, a higher priority constraint would have a SMALLER maximum
                let already_caught = higher_priority_durations.iter()
                    .any(|&higher_dur| gap_duration > higher_dur);
                
                // Only count as a violation if not already caught by higher priority constraint
                if !already_caught {
                    violation_count += 1;
                }
            }
        }
        
        violation_count
    };
    
    // Create a function that generates descriptions for violations
    let generate_too_long_gap_description = move |i: usize, day: u8| -> String {
        // Convert day number to name for readability
        let day_name = match day {
            0 => "Monday",
            1 => "Tuesday",
            2 => "Wednesday",
            3 => "Thursday",
            4 => "Friday",
            5 => "Saturday",
            6 => "Sunday",
            _ => "Unknown day",
        };
        
        if i == 1 {
            format!("has a gap longer than {} on {}", max_gap_duration, day_name)
        } else if i == 2 {
            format!("has a 2nd long gap on {}", day_name)
        } else if i == 3 {
            format!("has a 3rd long gap on {}", day_name)
        } else {
            format!("has a {}th long gap on {}", i, day_name)
        }
    };
    
    // Use the helper function to handle the encoding
    encode_faculty_cluster_helper(
        input,
        encoding,
        faculty,
        days_to_check,
        max_gap_within_cluster,
        priority,
        count_too_long_gaps,
        generate_too_long_gap_description
    )
}

// Encode a faculty gap too short constraint.
//
// A faculty gap too short constraint specifies that a faculty member should not
// have gaps between teaching clusters that are shorter than a specified duration.
pub fn encode_faculty_gap_too_short(
    input: &Input,
    encoding: &mut Encoding,
    priority: u8,
    faculty: usize,
    days_to_check: Days,
    min_gap_duration: Duration,
    max_gap_within_cluster: Duration,
    sat_criteria: &SatCriteria
) -> Result<()> {
    // Find higher-priority gap-too-short constraints for this faculty
    let mut higher_priority_durations: Vec<Duration> = Vec::new();
    
    // Check each priority level below the current one
    for p in 0..priority {
        // Get criteria at this priority level
        for criterion in sat_criteria.criteria_at_priority(p) {
            if let SatCriterion::FacultyGapTooShort { faculty: f, duration, .. } = criterion {
                if *f == faculty {
                    higher_priority_durations.push(*duration);
                }
            }
        }
    }
    
    // Validate specific inputs for this constraint type
    if min_gap_duration.minutes == 0 {
        return err(format!("Non-positive minimum gap duration for faculty {}", input.faculty[faculty].name));
    }

    // Create a function that detects "too short" gaps
    let count_too_short_gaps = move |clusters: &[(Time, Time)], _day: u8| -> usize {
        let mut violation_count = 0;
        
        // Need at least 2 clusters to have a gap
        if clusters.len() < 2 {
            return 0;
        }
        
        // Check each gap between clusters
        for i in 1..clusters.len() {
            let prev_end_time = clusters[i-1].1;
            let curr_start_time = clusters[i].0;
            
            // Calculate the gap duration
            let gap_duration = curr_start_time - prev_end_time;
            
            // Check if this gap is shorter than the minimum allowed duration
            if gap_duration < min_gap_duration {
                // Check if it's already caught by a higher-priority constraint
                // For "too short" gaps, a higher priority constraint would have a LARGER minimum
                let already_caught = higher_priority_durations.iter()
                    .any(|&higher_dur| gap_duration < higher_dur);
                
                // Only count as a violation if not already caught by higher priority constraint
                if !already_caught {
                    violation_count += 1;
                }
            }
        }
        
        // Unlike cluster_too_short, we don't ignore the first violation for gaps
        violation_count
    };
    
    // Create a function that generates descriptions for violations
    let generate_too_short_gap_description = move |i: usize, day: u8| -> String {
        // Convert day number to name for readability
        let day_name = match day {
            0 => "Monday",
            1 => "Tuesday",
            2 => "Wednesday",
            3 => "Thursday",
            4 => "Friday",
            5 => "Saturday",
            6 => "Sunday",
            _ => "Unknown day",
        };
        
        if i == 1 {
            format!("has a gap shorter than {} on {}", min_gap_duration, day_name)
        } else if i == 2 {
            format!("has a 2nd short gap on {}", day_name)
        } else if i == 3 {
            format!("has a 3rd short gap on {}", day_name)
        } else {
            format!("has a {}th short gap on {}", i, day_name)
        }
    };
    
    // Use the helper function to handle the encoding
    encode_faculty_cluster_helper(
        input,
        encoding,
        faculty,
        days_to_check,
        max_gap_within_cluster,
        priority,
        count_too_short_gaps,
        generate_too_short_gap_description
    )
}

// Generate all possible valid time slot combinations for a faculty member.
fn get_faculty_time_slot_combos(
    input: &Input,
    faculty: usize,
    days_to_check: Days
) -> Result<Vec<Vec<(usize, usize)>>> {
    // Validate inputs
    if faculty >= input.faculty.len() {
        return err(format!("Faculty index {} not found in input", faculty));
    }
    
    let faculty_sections = &input.faculty[faculty].sections;
    if faculty_sections.is_empty() {
        return err(format!("No sections found for faculty {}", input.faculty[faculty].name));
    }
    
    // Get time slots for each section
    let mut section_time_slots: Vec<Vec<(usize, usize)>> = Vec::new();
    
    for &section in faculty_sections {
        if section >= input.sections.len() {
            return err(format!("Section index {} out of bounds", section));
        }
        
        let section_obj = &input.sections[section];
        if section_obj.time_slots.is_empty() {
            return err(format!("No time slots available for section {}", section_obj.name));
        }
        
        // Filter for time slots that include at least one day from days_to_check
        let mut valid_time_slots = Vec::new();
        for &TimeSlotWithOptionalPriority { time_slot, .. } in &section_obj.time_slots {
            let time_slot_obj = &input.time_slots[time_slot];
            
            // Check if this time slot meets on any of the days we're checking
            let meets_on_day_to_check = days_to_check.into_iter()
                .any(|day| time_slot_obj.days.contains(day));
            
            if meets_on_day_to_check {
                valid_time_slots.push((section, time_slot));
            }
        }
        
        // Only add to section_time_slots if there are valid time slots
        if !valid_time_slots.is_empty() {
            section_time_slots.push(valid_time_slots);
        }
    }
    
    // This constraint should not exist if there is nothing to do
    if section_time_slots.is_empty() {
        return err(format!("No valid time slots for faculty {}", input.faculty[faculty].name));
    }
    
    // Generate all possible combinations using itertools::Itertools
    let mut valid_combinations = Vec::new();
    
    for combo in section_time_slots.into_iter().multi_cartesian_product() {
        // Check if this combination has any conflicts
        let mut has_conflict = false;
        
        // Check each pair of time slots for conflicts
        for i in 0..combo.len() {
            for j in (i + 1)..combo.len() {
                // Extract the time slot indices from the pairs
                let (_, time_slot_i) = combo[i];
                let (_, time_slot_j) = combo[j];
                
                if input.time_slot_conflicts[time_slot_i][time_slot_j] {
                    has_conflict = true;
                    break;
                }
            }
            
            if has_conflict {
                break;
            }
        }
        
        // If no conflicts, add this combination
        if !has_conflict {
            valid_combinations.push(combo);
        }
    }
    
    Ok(valid_combinations)
}

fn get_unique_section_day_patterns(
    input: &Input,
    faculty: usize,
    section_day_pairs: &[(usize, u8)],
    days_to_check: Days
) -> Result<Vec<Vec<bool>>> {
    // Generate all valid time slot combinations for this faculty
    let all_combos = get_faculty_time_slot_combos(input, faculty, days_to_check)?;
    
    // If no valid combinations, return an empty list
    if all_combos.is_empty() {
        return Ok(Vec::new());
    }
    
    // Group the section-day pairs by section for faster lookup
    let mut section_days: HashMap<usize, Vec<(usize, u8)>> = HashMap::new();
    for (idx, &(section, day)) in section_day_pairs.iter().enumerate() {
        section_days.entry(section).or_default().push((idx, day));
    }
    
    // Track unique patterns using a HashSet of Vecs
    let mut unique_patterns = HashSet::new();
    
    // Process each combination
    for combo in all_combos {
        // Create a result array filled with False initially
        let mut result = vec![false; section_day_pairs.len()];
        
        // Create section to timeslot mapping for this combo
        let mut section_to_timeslot = HashMap::new();
        for &(section, time_slot) in &combo {
            section_to_timeslot.insert(section, time_slot);
        }
        
        // Check if all required sections are in this combo
        if !section_days.keys().all(|k| section_to_timeslot.contains_key(k)) {
            continue;
        }
        
        // Process each section in the combination
        for (&section, section_day_indices) in &section_days {
            let time_slot = section_to_timeslot[&section];
            
            // Get the time slot's days once for each section
            if time_slot >= input.time_slots.len() {
                // Skip this combo if a time slot is out of bounds
                return err(format!("Time slot index {} out of bounds", time_slot));
            }
            
            let time_slot_days = &input.time_slots[time_slot].days;
            
            // Check each day for this section and update the result
            for &(idx, day) in section_day_indices {
                result[idx] = time_slot_days.contains(day);
            }
        }
        
        // Convert to a tuple for hashability
        let pattern_tuple = result.clone();
        
        // Add this pattern to the unique set
        unique_patterns.insert(pattern_tuple);
    }
    
    // Return the unique patterns
    Ok(unique_patterns.into_iter().collect())
}

// Create variables that represent when a faculty member's sections are scheduled on specific days.
fn make_faculty_section_day_vars(
    input: &Input,
    encoding: &mut Encoding,
    faculty: usize,
    days_to_check: Days
) -> Result<HashMap<(usize, u8), i32>> {
    // Create the map of vars we'll return
    let mut section_day_to_var: HashMap<(usize, u8), i32> = HashMap::new();
    
    // Create mappings to help with encoding
    let mut var_to_time_slot_vars: HashMap<i32, Vec<i32>> = HashMap::new();
    
    // For each section taught by this faculty
    for &section in &input.faculty[faculty].sections {
        // For each day of interest
        for day in days_to_check.into_iter() {
            // For each time slot available to this section
            for &TimeSlotWithOptionalPriority { time_slot, .. } in &input.sections[section].time_slots {
                let time_slot_days = &input.time_slots[time_slot].days;
                
                // Skip time slots that don't include this day
                if !time_slot_days.contains(day) {
                    continue;
                }
                
                // Get or create a variable for this (section, day)
                if !section_day_to_var.contains_key(&(section, day)) {
                    let var = encoding.new_var();
                    section_day_to_var.insert((section, day), var);
                    var_to_time_slot_vars.insert(var, Vec::new());
                }
                
                // Get the section-time variable - must exist if we've initialized correctly
                if !encoding.section_time_vars.contains_key(&(section, time_slot)) {
                    return err(format!("Missing variable for section {}, time slot {}", section, time_slot));
                }
                
                // Record section-time variable for encoding
                let time_var = encoding.section_time_vars[&(section, time_slot)];
                var_to_time_slot_vars.get_mut(&section_day_to_var[&(section, day)]).unwrap().push(time_var);
            }
        }
    }
    
    // Encode the constraints
    for (&var, time_slot_vars) in &var_to_time_slot_vars {
        if time_slot_vars.is_empty() {
            continue;
        }
        
        // Encode var -> (time_slot_1 OR time_slot_2 OR ...)
        // i.e., !var OR time_slot_1 OR time_slot_2 OR ...
        let mut clause = vec![-var];
        clause.extend(time_slot_vars);
        encoding.add_clause(clause);
        
        // Encode: (any of the time slots) -> var
        // i.e., (!time_slot_1 AND !time_slot_2 AND ...) OR section_day_var
        // i.e., (!time_slot_1 OR var) AND (!time_slot_2 OR var) AND ...
        for &time_var in time_slot_vars {
            encoding.add_clause(vec![-time_var, var]);
        }
    }
    
    Ok(section_day_to_var)
}

// Encode a faculty evenly spread constraint.
//
// A faculty evenly spread constraint specifies that a faculty member's classes
// should be evenly distributed across days with classes. This function creates
// a hallpass variable and adds clauses to enforce that if the faculty member's
// schedule isn't evenly spread, the hallpass variable must be true.
pub fn encode_faculty_evenly_spread(
    input: &Input,
    encoding: &mut Encoding,
    priority: u8,
    faculty: usize,
    days_to_check: Days
) -> Result<()> {
    // Create a hallpass variable for this constraint
    let hallpass = encoding.new_var();
    encoding.hallpass.insert(hallpass);
    
    // Add the problem to the encoding
    let faculty_name = &input.faculty[faculty].name;
    encoding.problems.insert(hallpass, (priority, format!("{} wants sections evenly spread across days", faculty_name)));

    // Validate inputs
    if faculty >= input.faculty.len() {
        return err(format!("Faculty index {} not found in input", faculty));
    }
    if days_to_check.len() <= 1 {
        return err(format!("Need at least two days to spread out classes for faculty {}", 
            input.faculty[faculty].name));
    }
    if input.faculty[faculty].sections.len() <= 3 {
        return err(format!("Faculty {} must have >3 sections to use evenly spread constraint", 
            input.faculty[faculty].name));
    }

    // Get section-day variables
    let section_day_to_var = make_faculty_section_day_vars(input, encoding, faculty, days_to_check)?;
    let section_day_list: Vec<(usize, u8)> = section_day_to_var.keys().cloned().collect();

    // Get all possible section-day combinations
    let patterns = get_unique_section_day_patterns(input, faculty, &section_day_list, days_to_check)?;

    // Iterate through all patterns
    for combo in patterns {
        // Track scheduled days and section counts
        let mut scheduled_days: HashMap<u8, usize> = HashMap::new();
        
        for (is_scheduled, (_section, day)) in combo.iter().zip(section_day_list.iter()) {
            if *is_scheduled {
                *scheduled_days.entry(*day).or_insert(0) += 1;
            }
        }

        // Skip if no days are scheduled
        if scheduled_days.is_empty() {
            continue;
        }

        // Check if distribution is uneven
        let mut min_sections = usize::MAX;
        let mut max_sections = 0;
        
        for &count in scheduled_days.values() {
            min_sections = std::cmp::min(min_sections, count);
            max_sections = std::cmp::max(max_sections, count);
        }
        
        // If the spread is good (difference <= 1), skip this pattern
        if max_sections - min_sections <= 1 {
            continue;
        }

        // Encode that this pattern should not happen without a hallpass
        let mut clause = vec![hallpass];
        
        for (is_scheduled, key) in combo.iter().zip(section_day_list.iter()) {
            let var = section_day_to_var[key];
            if *is_scheduled {
                clause.push(-var);
            } else {
                clause.push(var);
            }
        }

        encoding.add_clause(clause);
    }

    Ok(())
}

// Create variables that represent if a faculty is teaching in a specific room at a specific time.
//
// These variables are true if and only if at least one section taught by this faculty
// is scheduled in the given room at the given time slot.
fn make_faculty_room_time_vars(
    input: &Input,
    encoding: &mut Encoding,
    faculty: usize,
    days_to_check: Days
) -> Result<HashMap<(usize, usize), i32>> {
    // Create the map of variables we'll return
    let mut room_time_to_var: HashMap<(usize, usize), i32> = HashMap::new();
    
    // Create mappings to help with encoding
    let mut var_to_section_room_time_pairs: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    
    // For each section taught by this faculty
    for &section in &input.faculty[faculty].sections {
        // For each room available to this section
        for &RoomWithOptionalPriority { room, .. } in &input.sections[section].rooms {
            // Get the room variable
            if !encoding.section_room_vars.contains_key(&(section, room)) {
                continue;
            }
            let room_var = encoding.section_room_vars[&(section, room)];
            
            // For each time slot available to this section
            for &TimeSlotWithOptionalPriority { time_slot, .. } in &input.sections[section].time_slots {
                let time_slot_days = input.time_slots[time_slot].days;
                
                // Only consider time slots that meet on at least one day in days_to_check
                if !days_to_check.has_common_day(&time_slot_days) {
                    continue;
                }
                
                // Get the time slot variable
                if !encoding.section_time_vars.contains_key(&(section, time_slot)) {
                    continue;
                }
                let time_var = encoding.section_time_vars[&(section, time_slot)];
                
                // Get or create a variable for this (room, time_slot) combination
                if !room_time_to_var.contains_key(&(room, time_slot)) {
                    let var = encoding.new_var();
                    room_time_to_var.insert((room, time_slot), var);
                    var_to_section_room_time_pairs.insert(var, Vec::new());
                }
                
                // Record this (section_room, section_time) pair for encoding
                var_to_section_room_time_pairs.get_mut(&room_time_to_var[&(room, time_slot)])
                    .unwrap()
                    .push((room_var, time_var));
            }
        }
    }
    
    // Now add the clauses that define these variables
    for (frt_var, section_pairs) in var_to_section_room_time_pairs {
        // For each section's (room_var, time_var) pair:
        // (section_room AND section_time) → faculty_room_time
        // Equivalent to: (!section_room OR !section_time OR faculty_room_time)
        for (room_var, time_var) in section_pairs {
            encoding.add_clause(vec![-room_var, -time_var, frt_var]);
        }
        
        // faculty_room_time → OR(section_room AND section_time)
        // This direction is not strictly necessary for the encoding to work
        // and would require complex clauses, so we skip it as in the Python version
    }
    
    Ok(room_time_to_var)
}

// Find all pairs of sections that are back-to-back on the same day for this faculty, 
// along with their common valid rooms.
fn get_back_to_back_section_pairs(
    input: &Input,
    faculty: usize,
    days_to_check: Days,
    max_gap: Duration
) -> Result<Vec<(u8, [usize; 2], Vec<usize>)>> {
    // Validate inputs
    if faculty >= input.faculty.len() {
        return err(format!("Faculty index {} not found in input", faculty));
    }
    if days_to_check.is_empty() {
        return err(format!("Empty days_to_check for faculty {}", input.faculty[faculty].name));
    }
    
    // Get all sections for this faculty
    let faculty_sections = &input.faculty[faculty].sections;
    
    // Skip if faculty only has 0 or 1 section
    if faculty_sections.len() <= 1 {
        return Ok(Vec::new());
    }
    
    let mut result = Vec::new();
    
    // For each pair of sections
    for i in 0..faculty_sections.len() {
        let section_a = faculty_sections[i];
        for j in (i+1)..faculty_sections.len() {
            let section_b = faculty_sections[j];
            
            // Find rooms that both sections can use without preferences
            let mut common_rooms = Vec::new();
            for RoomWithOptionalPriority { room: room_a, priority: prio_a } in &input.sections[section_a].rooms {
                // Only consider rooms with no priority (no preference against)
                if prio_a.is_some() {
                    continue;
                }
                
                // Check if this room is also available to section_b without preference
                for RoomWithOptionalPriority { room: room_b, priority: prio_b } in &input.sections[section_b].rooms {
                    if room_a == room_b && prio_b.is_none() {
                        common_rooms.push(*room_a);
                        break;
                    }
                }
            }
            
            // Skip if no common valid rooms
            if common_rooms.is_empty() {
                continue;
            }
            
            // Check each day in days_to_check
            for day in days_to_check.into_iter() {
                // Get time slots that meet on this day
                let a_day_slots: Vec<_> = input.sections[section_a].time_slots.iter()
                    .filter(|ts| input.time_slots[ts.time_slot].days.contains(day))
                    .collect();
                    
                let b_day_slots: Vec<_> = input.sections[section_b].time_slots.iter()
                    .filter(|ts| input.time_slots[ts.time_slot].days.contains(day))
                    .collect();
                
                // Skip if either section has no time slots on this day
                if a_day_slots.is_empty() || b_day_slots.is_empty() {
                    continue;
                }
                
                // Check if they can be back-to-back (a before b)
                let mut found_pair = false;
                for ts_a in &a_day_slots {
                    let a_end = input.time_slots[ts_a.time_slot].start_time + input.time_slots[ts_a.time_slot].duration;
                    
                    for ts_b in &b_day_slots {
                        let b_start = input.time_slots[ts_b.time_slot].start_time;
                        
                        // Check if a ends before b starts
                        if a_end.minutes <= b_start.minutes {
                            // Calculate the gap
                            let gap = Duration::new(b_start.minutes.saturating_sub(a_end.minutes));
                            
                            // If the gap is within our max_gap, this pair is back-to-back
                            if gap.minutes <= max_gap.minutes {
                                result.push((day, [section_a, section_b], common_rooms.clone()));
                                found_pair = true;
                                break;
                            }
                        }
                    }
                    
                    if found_pair {
                        break;
                    }
                }
                
                // Check the reverse order (b before a) if no pair found yet
                if !found_pair {
                    for ts_b in &b_day_slots {
                        let b_end = input.time_slots[ts_b.time_slot].start_time + input.time_slots[ts_b.time_slot].duration;
                        
                        for ts_a in &a_day_slots {
                            let a_start = input.time_slots[ts_a.time_slot].start_time;
                            
                            // Check if b ends before a starts
                            if b_end.minutes <= a_start.minutes {
                                // Calculate the gap
                                let gap = Duration::new(a_start.minutes.saturating_sub(b_end.minutes));
                                
                                // If the gap is within our max_gap, this pair is back-to-back
                                if gap.minutes <= max_gap.minutes {
                                    result.push((day, [section_b, section_a], common_rooms.clone()));
                                    found_pair = true;
                                    break;
                                }
                            }
                        }
                        
                        if found_pair {
                            break;
                        }
                    }
                }
            }
        }
    }
    
    Ok(result)
}

// Check if two time slots are back-to-back (within max_gap of each other).
fn is_back_to_back(
    input: &Input,
    ts1: usize,
    ts2: usize,
    max_gap: Duration
) -> bool {
    // Get time slot objects
    let time_slot1 = &input.time_slots[ts1];
    let time_slot2 = &input.time_slots[ts2];
    
    // Get start and end times
    let start1 = time_slot1.start_time;
    let end1 = start1 + time_slot1.duration;
    let start2 = time_slot2.start_time;
    let end2 = start2 + time_slot2.duration;
    
    // Check if they share any days
    let common_days = time_slot1.days.intersect(&time_slot2.days);
    if common_days.is_empty() {
        return false;
    }
    
    // Check if they are back-to-back
    if end1.minutes <= start2.minutes {
        let gap = Duration::new(start2.minutes.saturating_sub(end1.minutes));
        return gap.minutes <= max_gap.minutes;
    } else if end2.minutes <= start1.minutes {
        let gap = Duration::new(start1.minutes.saturating_sub(end2.minutes));
        return gap.minutes <= max_gap.minutes;
    }
    
    // Overlapping time slots are not back-to-back
    false
}

// Encode a faculty no room switch constraint.
//
// A faculty no room switch constraint specifies that a faculty member should not
// have to switch rooms between back-to-back classes, where back-to-back means the
// gap between classes is <= max_gap_within_cluster. This function creates a hallpass
// variable and adds clauses to enforce that if the faculty member teaches in different
// rooms in back-to-back time slots, the hallpass variable must be true.
pub fn encode_faculty_no_room_switch(
    input: &Input,
    encoding: &mut Encoding,
    priority: u8,
    faculty: usize,
    days_to_check: Days,
    max_gap_within_cluster: Duration
) -> Result<()> {
    // Validate inputs
    if faculty >= input.faculty.len() {
        return err(format!("Faculty index {} not found in input", faculty));
    }
    
    // Skip if faculty has only one section (can't have room switches)
    if input.faculty[faculty].sections.len() <= 1 {
        return err(format!("Faculty {} should not have no room switch criterion with < 2 sections", 
            input.faculty[faculty].name));
    }
    
    // Create a hallpass variable for this constraint
    let hallpass = encoding.new_var();
    encoding.hallpass.insert(hallpass);
    
    // Add the problem to the encoding
    let faculty_name = &input.faculty[faculty].name;
    encoding.problems.insert(hallpass, (priority, 
        format!("{} should not switch rooms between back-to-back classes", faculty_name)));

    // Create faculty_room_time variables
    let faculty_room_time_vars = make_faculty_room_time_vars(input, encoding, faculty, days_to_check)?;
    
    // If no faculty_room_time variables were created, complain
    if faculty_room_time_vars.is_empty() {
        return err(format!("{} no room switch constraint found no rooms/time slots to consider", 
            input.faculty[faculty].name));
    }
    
    // Get all back-to-back section pairs with their common valid rooms
    let back_to_back_section_pairs = get_back_to_back_section_pairs(input, faculty, days_to_check, max_gap_within_cluster)?;
    
    // For each back-to-back section pair:
    for (day, sections, common_rooms) in back_to_back_section_pairs {
        let [section1, section2] = sections;
        
        // Skip if no common rooms (shouldn't happen due to filtering in get_back_to_back_section_pairs)
        if common_rooms.is_empty() {
            continue;
        }
        
        // Get available time slots for these sections on this day
        let section1_time_slots: Vec<_> = input.sections[section1].time_slots.iter()
            .filter(|ts| input.time_slots[ts.time_slot].days.contains(day))
            .map(|ts| ts.time_slot)
            .collect();
            
        let section2_time_slots: Vec<_> = input.sections[section2].time_slots.iter()
            .filter(|ts| input.time_slots[ts.time_slot].days.contains(day))
            .map(|ts| ts.time_slot)
            .collect();
        
        // For each possible time slot assignment to these sections
        for &ts1 in &section1_time_slots {
            for &ts2 in &section2_time_slots {
                // Skip if time slots don't form a back-to-back pair
                if !is_back_to_back(input, ts1, ts2, max_gap_within_cluster) {
                    continue;
                }
                
                // For each possible room assignment, check if using different rooms
                for &RoomWithOptionalPriority { room: room1, .. } in &input.sections[section1].rooms {
                    for &RoomWithOptionalPriority { room: room2, .. } in &input.sections[section2].rooms {
                        // Skip if it's the same room (no switch)
                        if room1 == room2 {
                            continue;
                        }
                        
                        // Skip if none of the rooms is in common_rooms
                        if !common_rooms.contains(&room1) && !common_rooms.contains(&room2) {
                            continue;
                        }
                        
                        // Get section-room and section-time variables
                        if !encoding.section_room_vars.contains_key(&(section1, room1)) ||
                           !encoding.section_time_vars.contains_key(&(section1, ts1)) ||
                           !encoding.section_room_vars.contains_key(&(section2, room2)) ||
                           !encoding.section_time_vars.contains_key(&(section2, ts2)) {
                            continue;
                        }
                        
                        let sr1_var = encoding.section_room_vars[&(section1, room1)];
                        let st1_var = encoding.section_time_vars[&(section1, ts1)];
                        let sr2_var = encoding.section_room_vars[&(section2, room2)];
                        let st2_var = encoding.section_time_vars[&(section2, ts2)];
                        
                        // Encode: (sr1 AND st1 AND sr2 AND st2) → hallpass
                        // This means: if section1 is in room1 at time1 and section2 is in room2 at time2,
                        // then there must be a hallpass for this constraint
                        // Equivalent to: (!sr1 OR !st1 OR !sr2 OR !st2 OR hallpass)
                        encoding.add_clause(vec![-sr1_var, -st1_var, -sr2_var, -st2_var, hallpass]);
                    }
                }
            }
        }
    }
    
    Ok(())
}

// Encode a faculty too many rooms constraint.
//
// A faculty too many rooms constraint specifies that a faculty member should not
// be scheduled in more than a minimum necessary number of rooms. This function 
// creates room usage variables and adds constraints to enforce that the number of 
// rooms used does not exceed the desired maximum, or the hallpass variable must be true.
fn encode_faculty_too_many_rooms(
    input: &Input,
    encoding: &mut Encoding,
    priority: u8,
    faculty: usize,
    desired_max_rooms: usize
) -> Result<()> {
    // Create a hallpass variable for this constraint
    let hallpass = encoding.new_var();
    encoding.hallpass.insert(hallpass);
    
    // Add the problem to the encoding
    let faculty_name = &input.faculty[faculty].name;
    let rooms_suffix = if desired_max_rooms == 1 { "" } else { "s" };
    encoding.problems.insert(hallpass, (priority, 
        format!("{} should use at most {} room{}", faculty_name, desired_max_rooms, rooms_suffix)));

    // Validate inputs
    if faculty >= input.faculty.len() {
        return err(format!("Faculty index {} not found in input", faculty));
    }
    if desired_max_rooms == 0 {
        return err(format!("Non-positive desired_max_rooms for faculty {}", faculty_name));
    }
    if desired_max_rooms >= input.faculty[faculty].sections.len() {
        return err(format!("desired_max_rooms == # of sections for {} with too many rooms criterion", faculty_name));
    }
    
    // Get the faculty's sections
    let faculty_sections = &input.faculty[faculty].sections;
    
    // Skip if faculty has no sections
    if faculty_sections.is_empty() {
        return Ok(());
    }
    
    // Get all potential rooms this faculty could be assigned
    let mut potential_rooms = HashSet::new();
    for &section in faculty_sections {
        if section >= input.sections.len() {
            return err(format!("Section index {} out of bounds", section));
        }
        
        for room_with_opt_priority in &input.sections[section].rooms {
            potential_rooms.insert(room_with_opt_priority.room);
        }
    }
    
    // Skip if no available rooms or just one room (constraint is trivially satisfied)
    if potential_rooms.len() <= 1 {
        return Ok(());
    }
    
    // Skip if the desired max is greater than or equal to the number of potential rooms
    // (constraint is trivially satisfied)
    if desired_max_rooms >= potential_rooms.len() {
        return Ok(());
    }
    
    // Create variables for "faculty uses this room"
    let mut faculty_room_vars = HashMap::new();
    for &room in &potential_rooms {
        faculty_room_vars.insert(room, encoding.new_var());
    }
    
    // For each section, connect the section-room variables to the faculty-room variables
    for &section in faculty_sections {
        for &RoomWithOptionalPriority { room, .. } in &input.sections[section].rooms {
            if !potential_rooms.contains(&room) {
                continue;
            }
            
            // Get the section-room variable
            if !encoding.section_room_vars.contains_key(&(section, room)) {
                return err(format!("Missing variable for section {}, room {}", section, room));
            }
            let section_room_var = encoding.section_room_vars[&(section, room)];
            
            // Connect: section_room_var -> faculty_room_var
            // If this section is assigned to this room, then the faculty uses this room
            // Equivalent to: !section_room_var OR faculty_room_var
            encoding.add_clause(vec![-section_room_var, faculty_room_vars[&room]]);
        }
    }
    
    // For each room, connect the faculty-room variable to at least one section-room variable
    for &room in &potential_rooms {
        // Collect all section-room variables for this room and faculty
        let mut section_room_vars = Vec::new();
        for &section in faculty_sections {
            if input.sections[section].rooms.iter().any(|r| r.room == room) {
                if !encoding.section_room_vars.contains_key(&(section, room)) {
                    return err(format!("Missing variable for section {}, room {}", section, room));
                }
                section_room_vars.push(encoding.section_room_vars[&(section, room)]);
            }
        }
        
        // If no sections can use this room, the faculty-room variable must be false
        if section_room_vars.is_empty() {
            encoding.add_clause(vec![-faculty_room_vars[&room]]);
            continue;
        }
        
        // faculty_room_var -> (section1_room OR section2_room OR ...)
        // If faculty uses this room, at least one section must be assigned to it
        // Equivalent to: !faculty_room_var OR section1_room OR section2_room OR ...
        let mut clause = vec![-faculty_room_vars[&room]];
        clause.extend_from_slice(&section_room_vars);
        encoding.add_clause(clause);
    }
    
    // If there are more potential rooms than the desired maximum, apply the constraint
    if faculty_room_vars.len() > desired_max_rooms {
        // Apply the at-most-k constraint using the totalizer encoding
        // We give it our hallpass variable
        let faculty_room_var_values: Vec<i32> = faculty_room_vars.values().copied().collect();
        encoding.totalizer_at_most_k(&faculty_room_var_values, desired_max_rooms, Some(hallpass));
    }
    
    Ok(())
}
