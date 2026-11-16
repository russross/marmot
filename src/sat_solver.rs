use super::cnf::Encoding;
use super::error::{Result, err};
use super::input::*;
use super::sat_criteria::*;
use super::sat_prep::{PreparedCnf, TotalizerOrder};
use super::score::*;
use super::solver::*;
use crate::options::{SatOpts, Search};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::time::Instant;

// Probe measurements cover only Kissat setup and solving, excluding CNF preparation.
struct ProbeTiming {
    priority: u8,
    constraints: usize,
    allowed: ScoreLevel,
    variables: i32,
    clauses: usize,
    order: TotalizerOrder,
    seconds: f64,
    result: &'static str,
}

struct SatRun<'a> {
    config: &'a SatOpts,
    criteria: &'a SatCriteria,
    prepared: PreparedCnf,
    timings: Vec<ProbeTiming>,
}

impl<'a> SatRun<'a> {
    fn new(config: &'a SatOpts, input: &Input, criteria: &'a SatCriteria) -> Result<Self> {
        Ok(Self {
            config,
            criteria,
            prepared: PreparedCnf::new(input, criteria, config.totalizer_order, config.shuffle_seed)?,
            timings: Vec::new(),
        })
    }

    // Record successful, infeasible, and failed calls even when reporting is disabled.
    fn solve(&mut self, encoding: &Encoding, priority: u8, limits: &Score) -> Result<Option<HashSet<i32>>> {
        let start = Instant::now();
        let result = encoding.solve();
        self.timings.push(ProbeTiming {
            priority,
            constraints: self.criteria.criteria_at_priority(priority).len(),
            allowed: limits.levels[priority as usize],
            variables: encoding.last_var,
            clauses: encoding.clauses.len(),
            order: self.config.totalizer_order,
            seconds: start.elapsed().as_secs_f64(),
            result: match &result {
                Ok(Some(_)) => "SAT",
                Ok(None) => "UNSAT",
                Err(_) => "ERROR",
            },
        });
        result.map_err(|e| format!("Error solving SAT instance: {e}").into())
    }
}

impl Drop for SatRun<'_> {
    fn drop(&mut self) {
        if self.config.sat_timings {
            println!("\nKissat timings (constraints at probed priority; variables/clauses for complete probe):");
            for t in &self.timings {
                println!(
                    "priority={} constraints={} allowed={} variables={} clauses={} order={} seconds={:.2} result={}",
                    t.priority, t.constraints, t.allowed, t.variables, t.clauses, t.order, t.seconds, t.result
                );
            }
        }
    }
}

// Generate a schedule using the SAT-based approach.
//
// This function encodes the scheduling problem as a SAT instance, solves it using an
// incremental approach to minimize violations at each priority level, and then constructs
// a schedule from the solution.
pub fn generate_schedule(
    config: &SatOpts,
    input: &Input,
    mut save: impl FnMut(&Schedule, &str) -> Result<()>,
) -> Result<Schedule> {
    let start_time = Instant::now();

    // Convert the input to a SAT criteria structure
    let sat_criteria = SatCriteria::from_input(input)?;
    validate_minimum_hallpasses(config, &sat_criteria)?;
    let mut run = SatRun::new(config, input, &sat_criteria)?;
    println!(
        "Loaded {} constraints across {} priority levels",
        sat_criteria.total_criteria_count(),
        sat_criteria.max_priority() + 1
    );

    // Track maximum violations allowed using a score vector
    let mut max_violations = Score::new();

    // the best schedule so far
    let mut best = None;

    // Process each priority level in order
    let max_priority = sat_criteria.max_priority().min(config.stop_after_priority.unwrap_or(MAX_PRIORITY));
    for priority in 0..=max_priority {
        let constraints = sat_criteria.criteria_at_priority(priority);
        if constraints.is_empty() && priority > 0 {
            continue;
        }

        // solve at this priority level, updating max_violations in place
        best = if config.search != Search::Linear {
            solve_binary(input, &mut run, priority, &mut max_violations, |schedule| {
                save(schedule, &format!("SAT generated: probing priority level {priority}"))?;
                Ok(())
            })?
        } else {
            solve_at_priority_level(input, &mut run, priority, &mut max_violations)?
        };
        if best.is_none() {
            if priority == 0 {
                return err("Failed to find a solution that satisfies hard constraints");
            }
            println!("  Failed to find solution at priority level {}, keeping best schedule so far", priority);
            break;
        }

        // save after each priority level
        let msg = if priority == sat_criteria.max_priority() {
            "SAT generated: complete".to_string()
        } else {
            format!("SAT generated: up to priority level {}", priority)
        };
        let Some(schedule) = &mut best else {
            unreachable!("best schedule cannot be none");
        };
        schedule.optimum_score_prefix = schedule.score.levels[..=priority as usize].to_vec();
        save(schedule, msg.as_str())?;
    }
    println!("\r{}\x1b[K", max_violations);

    if let Some(schedule) = best {
        // Sanity check: compare max_violations with schedule score
        if max_violations.levels[..=max_priority as usize] != schedule.score.levels[..=max_priority as usize] {
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

fn validate_minimum_hallpasses(config: &SatOpts, sat_criteria: &SatCriteria) -> Result<()> {
    if config.search != Search::Linear {
        return Ok(());
    }
    for priority in 1..=MAX_PRIORITY {
        let minimum = config.minimum_hallpasses.at_priority(priority);
        let constraint_count = sat_criteria.criteria_at_priority(priority).len();
        if minimum as usize > constraint_count {
            let suffix = if constraint_count == 1 { "" } else { "s" };
            return err(format!(
                "Minimum hallpass count {minimum} exceeds the {constraint_count} constraint{suffix} at priority level {priority}"
            ));
        }
    }

    Ok(())
}

// Erase the remainder of the line because speculative counts can shrink between probes.
fn print_progress(limits: &Score, priority: u8, range: Option<(ScoreLevel, ScoreLevel)>) {
    print!("\r<");
    let mut sep = "";
    for p in 0..=priority {
        if limits.levels[p as usize] > 0 || p == priority {
            print!("{}{}×{}", sep, p, limits.levels[p as usize]);
            sep = ",";
        }
    }
    if let Some((lower, upper)) = range {
        print!("∈[{lower},{upper}]");
    }
    print!("\x1b[K");
    let _ = std::io::stdout().flush();
}

// Solve for a specific priority level, finding minimum violations.
fn solve_at_priority_level(
    input: &Input,
    run: &mut SatRun<'_>,
    priority: u8,
    max_violations: &mut Score,
) -> Result<Option<Schedule>> {
    // Reset violations at this priority level to the configured starting point.
    max_violations.levels[priority as usize] = run.config.minimum_hallpasses.at_priority(priority);

    // Try to solve with increasing number of violations until we find a solution
    loop {
        // Create the SAT instance using current violations from max_violations
        let encoding = run.prepared.instance(max_violations, priority);
        let hallpass_count = encoding.hallpasses.get(&priority).map_or(0, HashSet::len);
        let hallpass_count = ScoreLevel::try_from(hallpass_count)
            .map_err(|_| format!("Priority level {priority} has too many hallpasses to score"))?;

        print_progress(max_violations, priority, None);

        // Solve the SAT instance
        match run.solve(&encoding, priority, max_violations) {
            Ok(Some(model)) => {
                // Convert the SAT solution into a schedule
                let schedule = decode_solution(input, &encoding, &model)?;
                verify_solution_score(input, &schedule, model_problems(&encoding, &model), priority);
                return Ok(Some(schedule));
            }
            Ok(None) => {
                // quit early if we fail to handle hard constraints
                if priority == 0 {
                    return err("No solution using only hard constraints");
                }

                // Once all hallpasses are allowed, a larger limit cannot change the result.
                if max_violations.levels[priority as usize] >= hallpass_count {
                    return Ok(None);
                }

                // Increment violations at this priority level and try again
                max_violations.levels[priority as usize] += 1;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

// Binary probes retain feasible refinements while only certifying completed priorities.
fn solve_binary(
    input: &Input,
    run: &mut SatRun<'_>,
    priority: u8,
    limits: &mut Score,
    mut save: impl FnMut(&Schedule) -> Result<()>,
) -> Result<Option<Schedule>> {
    let index = priority as usize;
    let count = run.prepared.hallpass_count(priority);
    let mut upper = ScoreLevel::try_from(count)
        .map_err(|_| format!("Priority level {priority} has too many hallpasses to score"))?;
    let mut lower = run.config.minimum_hallpasses.at_priority(priority);
    if lower > upper {
        return err(format!(
            "Minimum hallpass count {lower} exceeds the {upper} hallpasses at priority level {priority}"
        ));
    }
    if priority == 0 {
        upper = 0;
    }
    let mut probe = lower;
    // Feasible probes lower the ceiling; infeasible probes exclude their entire prefix.
    loop {
        limits.levels[index] = probe;
        print_progress(limits, priority, Some((lower, upper)));
        let encoding = run.prepared.instance(limits, priority);
        match run.solve(&encoding, priority, limits)? {
            Some(model) => {
                let mut schedule = decode_solution(input, &encoding, &model)?;
                schedule.optimum_score_prefix = limits.levels[..index].to_vec();
                save(&schedule)?;
                upper = probe;
                if lower == upper {
                    limits.levels[index] = upper;
                    verify_solution_score(input, &schedule, model_problems(&encoding, &model), priority);
                    return Ok(Some(schedule));
                }
            }
            None => {
                if priority == 0 {
                    return err("No solution using only hard constraints");
                }
                if probe == upper {
                    return Ok(None);
                }
                lower = probe + 1;
            }
        }
        probe = lower + (upper - lower) / 2;
    }
}

// Create the basic variables for sections, time slots, and rooms.
pub(super) fn create_basic_variables(input: &Input, encoding: &mut Encoding) -> Result<()> {
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
pub(super) fn encode_basic_constraints(input: &Input, encoding: &mut Encoding) -> Result<()> {
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
pub(super) fn encode_room_conflicts(input: &Input, encoding: &mut Encoding) -> Result<()> {
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

// Decode a SAT solution into a schedule.
fn decode_solution(input: &Input, encoding: &Encoding, model: &HashSet<i32>) -> Result<Schedule> {
    // Create reverse lookup tables to find section/room and section/time from var
    let var_to_section_room: HashMap<i32, (usize, usize)> =
        encoding.section_room_vars.iter().map(|(&k, &v)| (v, k)).collect();
    let var_to_section_time: HashMap<i32, (usize, usize)> =
        encoding.section_time_vars.iter().map(|(&k, &v)| (v, k)).collect();

    let mut section_to_room: HashMap<usize, usize> = HashMap::new();
    let mut section_to_time_slot: HashMap<usize, usize> = HashMap::new();

    // Process all positive variable assignments
    for &var in model {
        if var <= 0 {
            continue;
        }

        if let Some(&(section, room)) = var_to_section_room.get(&var) {
            section_to_room.insert(section, room);
        } else if let Some(&(section, time_slot)) = var_to_section_time.get(&var) {
            section_to_time_slot.insert(section, time_slot);
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

    Ok(schedule)
}

// Preserve the model's reported violations independently of placement decoding.
fn model_problems(encoding: &Encoding, model: &HashSet<i32>) -> Vec<(u8, String)> {
    model.iter().filter(|&&var| var > 0).filter_map(|var| encoding.problems.get(var).cloned()).collect()
}

// Compare scores only when the current priority's allowed count is established.
fn verify_solution_score(input: &Input, schedule: &Schedule, mut problems: Vec<(u8, String)>, priority: u8) {
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
                    lst.push(penalty.get_score_message(input, schedule));
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
}

#[cfg(test)]
mod tests {
    use super::{SatCriteria, SatCriterion, SatOpts, validate_minimum_hallpasses};
    use crate::MinimumHallpasses;

    #[test]
    fn binary_search_certifies_empty_levels_and_rejects_excess_hints() {
        let input = crate::input::Input {
            term_name: String::new(),
            rooms: vec![],
            time_slots: vec![],
            faculty: vec![],
            sections: vec![],
            criteria: vec![],
            time_slot_conflicts: vec![],
            faculty_preference_priority_policy: crate::faculty_preferences::FacultyPreferencePriorityPolicy::Stated,
        };
        let criteria = SatCriteria::new();
        let mut limits = crate::score::Score::new();
        let config = SatOpts::default();
        let mut run = super::SatRun::new(&config, &input, &criteria).unwrap();
        let mut saves = 0;
        let result = super::solve_binary(&input, &mut run, 0, &mut limits, |schedule| {
            saves += 1;
            assert!(schedule.optimum_score_prefix.is_empty());
            Ok(())
        })
        .unwrap();
        assert!(result.is_some());
        assert_eq!(saves, 1);
        assert_eq!(limits.levels[1], 0);
        assert_eq!(run.timings.len(), 1);
        assert_eq!(run.timings[0].result, "SAT");
        assert!(!config.sat_timings);
        let config = SatOpts { minimum_hallpasses: MinimumHallpasses::parse("1:1").unwrap(), ..SatOpts::default() };
        let mut run = super::SatRun::new(&config, &input, &criteria).unwrap();
        assert!(super::solve_binary(&input, &mut run, 1, &mut limits, |_| Ok(())).is_err());
    }

    #[test]
    fn rejects_a_minimum_hallpass_count_above_the_constraint_count() {
        let mut criteria = SatCriteria::new();
        criteria.add_criterion(SatCriterion::Conflict { sections: [0, 1], priority: 7 });
        let config = SatOpts {
            search: crate::Search::Linear,
            minimum_hallpasses: MinimumHallpasses::parse("7:2").unwrap(),
            ..SatOpts::default()
        };

        let error = validate_minimum_hallpasses(&config, &criteria).unwrap_err();

        assert_eq!(error.to_string(), "Minimum hallpass count 2 exceeds the 1 constraint at priority level 7");
    }

    // A hard preference fixes the section at slot zero. Soft preferences at
    // that slot are unavoidable; preferences at slot one supply unused capacity.
    fn probe_input(count: usize, optimum: usize) -> crate::input::Input {
        use crate::input::TimeSlotWithPriority;
        use crate::score::Criterion;
        let mut input = crate::shared_day_off_tests::input();
        input.faculty.clear();
        input.sections.truncate(1);
        let section = &mut input.sections[0];
        section.rooms.clear();
        section.faculty.clear();
        section.neighbors.clear();
        section.time_slots.truncate(2);
        section.criteria = (0..=count).collect();
        input.criteria = (0..=count)
            .map(|i| Criterion::TimeSlotPreference {
                section: 0,
                time_slots_with_priorities: vec![TimeSlotWithPriority {
                    time_slot: usize::from(i == 0 || i > optimum),
                    priority: u8::from(i != 0),
                }],
            })
            .collect();
        input
    }

    #[test]
    fn searches_find_every_boundary_and_respect_minimums() {
        use crate::Search;
        // Cover odd widths and minima above the true optimum without depending on chosen models.
        for count in [1, 15, 16, 17, 32] {
            for optimum in 0..=count {
                for minimum in [0, count / 2] {
                    let input = probe_input(count, optimum);
                    let criteria = SatCriteria::from_input(&input).unwrap();
                    for search in [Search::Linear, Search::Binary] {
                        let config = SatOpts {
                            search,
                            minimum_hallpasses: MinimumHallpasses::parse(&format!("1:{minimum}")).unwrap(),
                            ..SatOpts::default()
                        };
                        let mut run = super::SatRun::new(&config, &input, &criteria).unwrap();
                        let mut limits = crate::score::Score::new();
                        let result = if search == Search::Linear {
                            super::solve_at_priority_level(&input, &mut run, 1, &mut limits)
                        } else {
                            super::solve_binary(&input, &mut run, 1, &mut limits, |schedule| {
                                assert_eq!(schedule.optimum_score_prefix, vec![0]);
                                Ok(())
                            })
                        }
                        .unwrap()
                        .unwrap();
                        assert_eq!(limits.levels[1], optimum.max(minimum) as crate::score::ScoreLevel);
                        assert_eq!(result.score.levels[1], optimum as crate::score::ScoreLevel);
                    }
                }
            }
        }
    }

    #[test]
    fn binary_probes_default_zero_and_explicit_minimum_first() {
        use crate::Search;
        for (optimum, minimum, expected_first, expected_second) in
            [(0, None, 0, None), (3, None, 0, Some(16)), (3, Some(2), 2, Some(17)), (3, Some(4), 4, None)]
        {
            let input = probe_input(32, optimum);
            let criteria = SatCriteria::from_input(&input).unwrap();
            let config = SatOpts {
                search: Search::Binary,
                minimum_hallpasses: minimum
                    .map(|value| MinimumHallpasses::parse(&format!("1:{value}")).unwrap())
                    .unwrap_or_default(),
                ..SatOpts::default()
            };
            let mut run = super::SatRun::new(&config, &input, &criteria).unwrap();
            let mut limits = crate::score::Score::new();
            super::solve_binary(&input, &mut run, 1, &mut limits, |_| Ok(())).unwrap().unwrap();
            let probes: Vec<_> = run.timings.iter().map(|t| t.allowed).collect();
            assert_eq!(probes.first(), Some(&expected_first));
            assert_eq!(probes.get(1), expected_second.as_ref());
            assert_eq!(limits.levels[1], optimum.max(minimum.unwrap_or(0)) as crate::score::ScoreLevel);
        }
    }

    #[test]
    fn binary_handles_hard_constraints_infeasible_bounds_and_save_errors() {
        use crate::Search;
        let mut input = probe_input(1, 0);
        let config = SatOpts { search: Search::Binary, ..SatOpts::default() };
        let criteria = SatCriteria::from_input(&input).unwrap();
        let mut limits = crate::score::Score::new();
        let mut run = super::SatRun::new(&config, &input, &criteria).unwrap();
        super::solve_binary(&input, &mut run, 0, &mut limits, |_| Ok(())).unwrap().unwrap();
        assert_eq!(run.timings.len(), 1);
        assert_eq!(run.timings[0].allowed, 0);

        // Saving a feasible refinement must propagate failure immediately.
        let error =
            super::solve_binary(&input, &mut run, 1, &mut limits, |_| crate::error::err("save failed")).err().unwrap();
        assert_eq!(error.to_string(), "save failed");

        // Forbid the other available slot at priority zero as well. Even an
        // unrestricted soft budget cannot repair infeasible hard constraints.
        input.criteria.push(crate::score::Criterion::TimeSlotPreference {
            section: 0,
            time_slots_with_priorities: vec![crate::input::TimeSlotWithPriority { time_slot: 0, priority: 0 }],
        });
        input.sections[0].criteria.push(2);
        let criteria = SatCriteria::from_input(&input).unwrap();
        let mut run = super::SatRun::new(&config, &input, &criteria).unwrap();
        assert!(super::solve_binary(&input, &mut run, 0, &mut limits, |_| Ok(())).is_err());
        assert!(super::solve_binary(&input, &mut run, 1, &mut limits, |_| Ok(())).unwrap().is_none());
        assert_eq!(run.timings.last().unwrap().allowed, 1);
        assert_eq!(run.timings.last().unwrap().result, "UNSAT");
    }
}
