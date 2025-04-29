use super::cnf::Encoding;
use super::error::{Result, err};
use super::input::*;
use super::sat_criteria::*;
use super::sat_encoders::*;
use super::score::*;
use super::solver::*;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::time::Instant;

// Generate a schedule using the SAT-based approach.
//
// This function encodes the scheduling problem as a SAT instance, solves it using an
// incremental approach to minimize violations at each priority level, and then constructs
// a schedule from the solution.
pub fn generate_schedule(input: &Input) -> Result<Schedule> {
    let start_time = Instant::now();
    println!("Starting SAT-based schedule generation");

    // Convert the input to a SAT criteria structure
    let sat_criteria = SatCriteria::from_input(input)?;
    println!(
        "Loaded {} constraints across {} priority levels",
        sat_criteria.total_criteria_count(),
        sat_criteria.max_priority() + 1
    );

    // Track maximum violations allowed using a score vector
    let mut max_violations = Score::new();

    // the best schedule so far
    let mut best = None;
    println!("Searching for minimal score:");

    // Process each priority level in order
    let max_priority = sat_criteria.max_priority();
    for priority in 0..=max_priority {
        let constraints = sat_criteria.criteria_at_priority(priority);
        if constraints.is_empty() && priority > 0 {
            continue;
        }

        // Solve at this priority level, updating max_violations in place
        best = solve_at_priority_level(input, &sat_criteria, priority, &mut max_violations)?;
        if best.is_none() {
            if priority == 0 {
                return err("Failed to find a solution that satisfies hard constraints");
            }
            println!("  Failed to find solution at priority level {}, keeping best schedule so far", priority);
            break;
        }
    }
    println!("\r{}    ", max_violations);

    if let Some(schedule) = best {
        // Sanity check: compare max_violations with schedule score
        if max_violations != schedule.score {
            println!("\nWARNING: Inconsistency detected in violation counts:");
            println!("  Search algorithm found: {}", max_violations);
            println!("  Solution reports:       {}", schedule.score);
        }

        println!("Total solving time: {:?}\n", start_time.elapsed());

        Ok(schedule)
    } else {
        err("finished search without finding a viable schedule")
    }
}

// Solve for a specific priority level, finding minimum violations.
fn solve_at_priority_level(
    input: &Input,
    sat_criteria: &SatCriteria,
    priority: u8,
    max_violations: &mut Score,
) -> Result<Option<Schedule>> {
    // Get constraints at this priority level
    let constraints = sat_criteria.criteria_at_priority(priority);
    let criteria_count = constraints.len();

    // Reset violations at this priority level to start at 0
    max_violations.levels[priority as usize] = 0;

    // Try to solve with increasing number of violations until we find a solution
    while max_violations.levels[priority as usize] <= criteria_count as i16 {
        // Create the SAT instance using current violations from max_violations
        let encoding = create_sat_instance(input, sat_criteria, max_violations, priority)?;

        // print progress display
        print!("\r<");
        let mut sep = "";
        for p in 0..=priority {
            if max_violations.levels[p as usize] > 0 || p == priority {
                print!("{}{}×{}", sep, p, max_violations.levels[p as usize]);
                sep = ",";
            }
        }
        let _ = std::io::stdout().flush();

        // Solve the SAT instance
        match encoding.solve() {
            Ok(Some(model)) => {
                // Convert the SAT solution into a schedule
                return Ok(Some(decode_solution(input, &encoding, &model, priority)?));
            }
            Ok(None) => {
                // Increment violations at this priority level and try again
                max_violations.levels[priority as usize] += 1;

                // quit early if we fail to handle hard constraints
                if priority == 0 {
                    return err("No solution using only hard constraints");
                }
            }
            Err(e) => {
                return err(format!("Error solving SAT instance: {}", e));
            }
        }
    }

    Ok(None)
}

// Create a SAT instance for the timetabling problem.
fn create_sat_instance(
    input: &Input,
    sat_criteria: &SatCriteria,
    max_violations: &Score,
    current_priority: u8,
) -> Result<Encoding> {
    // Create CNF formula
    let mut encoding = Encoding::new();

    // Create the basic variables
    create_basic_variables(input, &mut encoding)?;

    // Encode the basic constraints
    encode_basic_constraints(input, &mut encoding)?;

    // Encode room conflicts
    encode_room_conflicts(input, &mut encoding)?;

    // Encode all constraints up to and including the current priority level
    for p in 0..=current_priority {
        // Determine max violations allowed for this priority level
        let violations = max_violations.levels[p as usize];

        // Encode constraints at this priority level
        encode_constraints(input, sat_criteria, &mut encoding, p, violations)?;
    }

    // Return the CNF formula and variable mappings
    Ok(encoding)
}

// Create the basic variables for sections, time slots, and rooms.
fn create_basic_variables(input: &Input, encoding: &mut Encoding) -> Result<()> {
    // Create section-room variables
    for section_i in 0..input.sections.len() {
        let section = &input.sections[section_i];
        for room_i in section.rooms.iter().map(|r| r.room) {
            let var = encoding.new_var();
            encoding.section_room_vars.insert((section_i, room_i), var);
        }
    }

    // Create section-time variables
    for section_i in 0..input.sections.len() {
        let section = &input.sections[section_i];
        for time_slot_i in section.time_slots.iter().map(|t| t.time_slot) {
            let var = encoding.new_var();
            encoding.section_time_vars.insert((section_i, time_slot_i), var);
        }
    }

    Ok(())
}

// Encode the basic constraints of the timetabling problem:
// 1. Each section must be assigned exactly one time slot
// 2. Each section must be assigned exactly one room (if it has available rooms)
fn encode_basic_constraints(input: &Input, encoding: &mut Encoding) -> Result<()> {
    // Group variables by section for easier processing
    let mut section_to_rooms: HashMap<usize, Vec<i32>> = HashMap::new();
    let mut section_to_times: HashMap<usize, Vec<i32>> = HashMap::new();

    // Organize variables by section
    for (&(section, _room), &var) in &encoding.section_room_vars {
        section_to_rooms.entry(section).or_default().push(var);
    }

    for (&(section, _time_slot), &var) in &encoding.section_time_vars {
        section_to_times.entry(section).or_default().push(var);
    }

    // Constraint 1: Each section must be assigned exactly one room (if it has available rooms)
    for (_section, room_vars) in section_to_rooms {
        if room_vars.is_empty() {
            continue;
        }

        // At least one room must be assigned
        encoding.add_clause(room_vars.clone());

        // At most one room must be assigned
        encoding.pairwise_at_most_one(&room_vars);
    }

    // Constraint 2: Each section must be assigned exactly one time slot
    for (section, time_vars) in section_to_times {
        if time_vars.is_empty() {
            return err(format!("Section {} has no available time slots", input.sections[section].name));
        }

        // At least one time slot must be assigned
        encoding.add_clause(time_vars.clone());

        // At most one time slot must be assigned
        encoding.pairwise_at_most_one(&time_vars);
    }

    Ok(())
}

// Encode the constraint that two sections cannot be in the same room
// at overlapping time slots.
fn encode_room_conflicts(input: &Input, encoding: &mut Encoding) -> Result<()> {
    // Group sections by room
    let mut room_to_sections: HashMap<usize, Vec<usize>> = HashMap::new();

    for &(section, room) in encoding.section_room_vars.keys() {
        room_to_sections.entry(room).or_default().push(section);
    }

    // For each room, prevent overlapping section assignments
    for (room, sections) in room_to_sections {
        // Skip if only one section can use this room
        if sections.len() < 2 {
            continue;
        }

        // For each pair of sections that could use this room
        for i in 0..sections.len() {
            for j in (i + 1)..sections.len() {
                // skip if these two sections are in hard conflict with each other
                if input.sections[sections[i]].hard_conflicts.contains(&sections[j]) {
                    continue;
                }
                encode_room_conflict(input, encoding, sections[i], sections[j], room)?;
            }
        }
    }

    Ok(())
}

// Encode the constraint that two sections cannot be in the same room
// at overlapping time slots.
fn encode_room_conflict(
    input: &Input,
    encoding: &mut Encoding,
    section_a: usize,
    section_b: usize,
    room: usize,
) -> Result<()> {
    // Get room variables for both sections - must exist if we've initialized correctly
    if !encoding.section_room_vars.contains_key(&(section_a, room)) {
        return err(format!("Missing variable for section {}, room {}", section_a, room));
    }
    if !encoding.section_room_vars.contains_key(&(section_b, room)) {
        return err(format!("Missing variable for section {}, room {}", section_b, room));
    }

    let room_var_a = encoding.section_room_vars[&(section_a, room)];
    let room_var_b = encoding.section_room_vars[&(section_b, room)];

    // Get time slots for both sections
    let section_a_time_slots: Vec<usize> = input.sections[section_a].time_slots.iter().map(|ts| ts.time_slot).collect();
    let section_b_time_slots: Vec<usize> = input.sections[section_b].time_slots.iter().map(|ts| ts.time_slot).collect();

    // Check each pair of potentially conflicting time slots
    for &time_a in &section_a_time_slots {
        for &time_b in &section_b_time_slots {
            // Skip if the time slots don't conflict
            if !input.time_slot_conflicts[time_a][time_b] {
                continue;
            }

            // Get time slot variables - must exist if we've initialized correctly
            if !encoding.section_time_vars.contains_key(&(section_a, time_a)) {
                return err(format!("Missing variable for section {}, time slot {}", section_a, time_a));
            }
            if !encoding.section_time_vars.contains_key(&(section_b, time_b)) {
                return err(format!("Missing variable for section {}, time slot {}", section_b, time_b));
            }

            let time_var_a = encoding.section_time_vars[&(section_a, time_a)];
            let time_var_b = encoding.section_time_vars[&(section_b, time_b)];

            // Add clause: ~(A_time & A_room & B_time & B_room)
            // Which is equivalent to: (!A_time | !A_room | !B_time | !B_room)
            encoding.add_clause(vec![-time_var_a, -room_var_a, -time_var_b, -room_var_b]);
        }
    }

    Ok(())
}

// Encode constraints at a specific priority level.
fn encode_constraints(
    input: &Input,
    sat_criteria: &SatCriteria,
    encoding: &mut Encoding,
    priority: u8,
    max_violations: i16,
) -> Result<()> {
    // Collect all hallpass variables for this priority level
    encoding.hallpass.clear();

    // Get all constraints at this priority level
    let constraints = sat_criteria.criteria_at_priority(priority);

    // Encode each constraint
    for constraint in constraints {
        encode_criterion(input, encoding, constraint, sat_criteria)?;
    }

    // Collect hallpass variables to avoid borrow conflicts
    let hallpass_vars: Vec<i32> = encoding.hallpass.iter().copied().collect();

    // Apply cardinality constraint if needed
    if max_violations == 0 {
        // No violations allowed: force all hallpass variables to be false
        for &var in &hallpass_vars {
            encoding.add_clause(vec![-var]);
        }
    } else if !hallpass_vars.is_empty() && max_violations < hallpass_vars.len() as i16 {
        // Limited violations allowed: add cardinality constraint
        if max_violations == 1 && hallpass_vars.len() <= 30 {
            encoding.pairwise_at_most_one(&hallpass_vars);
        } else {
            encoding.totalizer_at_most_k(&hallpass_vars, max_violations as usize, None);
        }
    }

    // Clear hallpass variables for this priority level
    encoding.hallpass.clear();

    Ok(())
}

// Decode a SAT solution into a schedule.
fn decode_solution(input: &Input, encoding: &Encoding, model: &HashSet<i32>, priority: u8) -> Result<Schedule> {
    // Create reverse lookup tables to find section/room and section/time from var
    let var_to_section_room: HashMap<i32, (usize, usize)> =
        encoding.section_room_vars.iter().map(|(&k, &v)| (v, k)).collect();
    let var_to_section_time: HashMap<i32, (usize, usize)> =
        encoding.section_time_vars.iter().map(|(&k, &v)| (v, k)).collect();

    let mut section_to_room: HashMap<usize, usize> = HashMap::new();
    let mut section_to_time_slot: HashMap<usize, usize> = HashMap::new();
    let mut problems: Vec<(u8, String)> = Vec::new();

    // Process all positive variable assignments
    for &var in model {
        if var <= 0 {
            continue;
        }

        if let Some(&(section, room)) = var_to_section_room.get(&var) {
            section_to_room.insert(section, room);
        } else if let Some(&(section, time_slot)) = var_to_section_time.get(&var) {
            section_to_time_slot.insert(section, time_slot);
        } else if let Some(&(priority, ref message)) = encoding.problems.get(&var) {
            problems.push((priority, message.clone()));
        }
    }

    // build the schedule
    let mut schedule = Schedule::new(input);

    // place each section
    for (section, time_slot) in section_to_time_slot {
        let room = section_to_room.get(&section).copied();

        // use move_section so constraints are checked and scores are computed
        let _log = move_section(input, &mut schedule, section, time_slot, &room);
    }

    // Verify that the schedule score matches our expectation
    let mut expected_score = Score::new();
    for (priority, _) in &problems {
        expected_score += *priority;
    }

    // Compare the schedule score with our expected score
    let mut found_score = schedule.score;
    for elt in &mut found_score.levels[(priority + 1) as usize..] {
        *elt = 0;
    }
    if found_score != expected_score {
        println!("\nSchedule score doesn't match expected score from problems up to priority {}", priority);
        println!("  Schedule score:  {}", found_score);
        {
            let mut lst: Vec<(u8, String)> = Vec::new();
            for penalty_list in &schedule.penalties {
                for penalty in penalty_list {
                    lst.push(penalty.get_score_message(input, &schedule));
                }
            }
            lst.sort_unstable_by(|a, b| if a.0 != b.0 { a.0.cmp(&b.0) } else { a.1.cmp(&b.1) });
            for (p, msg) in lst {
                if p > priority {
                    continue;
                }
                println!("      {p:2}: {msg}");
            }
        }
        println!("  Expected score:  {}", expected_score);
        {
            problems.sort_unstable_by(|a, b| if a.0 != b.0 { a.0.cmp(&b.0) } else { a.1.cmp(&b.1) });
            for (p, msg) in problems {
                println!("      {p:2}: {msg}");
            }
        }
    }

    Ok(schedule)
}
