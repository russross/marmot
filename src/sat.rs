use super::error::Result;
use super::input::*;
use super::sat_encoding::*;
use super::score::*;
use super::solver::*;
use rustsat::instances::ManageVars;
use rustsat::solvers::{Solve, SolverResult};
use rustsat::types::{Assignment, Var};
use rustsat_cadical::CaDiCaL;
use rustsat_kissat::Kissat;
use std::collections::HashMap;

// Transform the input criteria into SatCriterion objects
fn transform_criteria(input: &Input) -> Vec<Vec<SatCriterion>> {
    // Initialize with empty vectors for each priority level
    let mut criteria_by_priority = Vec::new();
    for _ in 0..PRIORITY_LEVELS {
        criteria_by_priority.push(Vec::new());
    }

    // Process each criterion and distribute according to priority
    for criterion in &input.criteria {
        match criterion {
            Criterion::SoftConflict { priority, sections } => {
                let sat_criterion = SatCriterion::SoftConflict { priority: *priority, sections: *sections };
                criteria_by_priority[*priority as usize].push(sat_criterion);
            }

            Criterion::AntiConflict { priority, single, group } => {
                let sat_criterion =
                    SatCriterion::AntiConflict { priority: *priority, single: *single, group: group.clone() };
                criteria_by_priority[*priority as usize].push(sat_criterion);
            }

            Criterion::RoomPreference { section, rooms_with_priorities } => {
                for room_with_priority in rooms_with_priorities {
                    let sat_criterion = SatCriterion::RoomPreference {
                        priority: room_with_priority.priority,
                        section: *section,
                        room: room_with_priority.room,
                    };
                    criteria_by_priority[room_with_priority.priority as usize].push(sat_criterion);
                }
            }

            Criterion::TimeSlotPreference { section, time_slots_with_priorities } => {
                for time_slot_with_priority in time_slots_with_priorities {
                    let sat_criterion = SatCriterion::TimeSlotPreference {
                        priority: time_slot_with_priority.priority,
                        section: *section,
                        time_slot: time_slot_with_priority.time_slot,
                    };
                    criteria_by_priority[time_slot_with_priority.priority as usize].push(sat_criterion);
                }
            }

            Criterion::FacultyPreference {
                faculty: _,
                sections,
                days_to_check,
                days_off,
                evenly_spread,
                no_room_switch,
                too_many_rooms,
                max_gap_within_cluster,
                distribution_intervals,
            } => {
                // Process days off preference
                if let Some((priority, desired)) = days_off {
                    let sat_criterion = SatCriterion::FacultyDaysOff {
                        priority: *priority,
                        sections: sections.clone(),
                        days_to_check: *days_to_check,
                        desired: *desired,
                    };
                    criteria_by_priority[*priority as usize].push(sat_criterion);
                }

                // Process evenly spread preference
                if let Some(priority) = evenly_spread {
                    let sat_criterion = SatCriterion::FacultyEvenlySpread {
                        priority: *priority,
                        sections: sections.clone(),
                        days_to_check: *days_to_check,
                    };
                    criteria_by_priority[*priority as usize].push(sat_criterion);
                }

                // Process no room switch preference
                if let Some(priority) = no_room_switch {
                    let sat_criterion = SatCriterion::FacultyNoRoomSwitch {
                        priority: *priority,
                        sections: sections.clone(),
                        days_to_check: *days_to_check,
                        max_gap_within_cluster: *max_gap_within_cluster,
                    };
                    criteria_by_priority[*priority as usize].push(sat_criterion);
                }

                // Process too many rooms preference
                if let Some((priority, desired)) = too_many_rooms {
                    let sat_criterion = SatCriterion::FacultyTooManyRooms {
                        priority: *priority,
                        sections: sections.clone(),
                        desired: *desired,
                    };
                    criteria_by_priority[*priority as usize].push(sat_criterion);
                }

                // Process distribution intervals
                for interval in distribution_intervals {
                    match interval {
                        DistributionInterval::GapTooShort { priority, duration } => {
                            let sat_criterion = SatCriterion::FacultyDistributionInterval {
                                priority: *priority,
                                sections: sections.clone(),
                                days_to_check: *days_to_check,
                                interval_type: DistributionIntervalType::GapTooShort,
                                duration: *duration,
                                max_gap_within_cluster: *max_gap_within_cluster,
                            };
                            criteria_by_priority[*priority as usize].push(sat_criterion);
                        }
                        DistributionInterval::GapTooLong { priority, duration } => {
                            let sat_criterion = SatCriterion::FacultyDistributionInterval {
                                priority: *priority,
                                sections: sections.clone(),
                                days_to_check: *days_to_check,
                                interval_type: DistributionIntervalType::GapTooLong,
                                duration: *duration,
                                max_gap_within_cluster: *max_gap_within_cluster,
                            };
                            criteria_by_priority[*priority as usize].push(sat_criterion);
                        }
                        DistributionInterval::ClusterTooShort { priority, duration } => {
                            let sat_criterion = SatCriterion::FacultyDistributionInterval {
                                priority: *priority,
                                sections: sections.clone(),
                                days_to_check: *days_to_check,
                                interval_type: DistributionIntervalType::ClusterTooShort,
                                duration: *duration,
                                max_gap_within_cluster: *max_gap_within_cluster,
                            };
                            criteria_by_priority[*priority as usize].push(sat_criterion);
                        }
                        DistributionInterval::ClusterTooLong { priority, duration } => {
                            let sat_criterion = SatCriterion::FacultyDistributionInterval {
                                priority: *priority,
                                sections: sections.clone(),
                                days_to_check: *days_to_check,
                                interval_type: DistributionIntervalType::ClusterTooLong,
                                duration: *duration,
                                max_gap_within_cluster: *max_gap_within_cluster,
                            };
                            criteria_by_priority[*priority as usize].push(sat_criterion);
                        }
                    }
                }
            }

            Criterion::SectionsWithDifferentTimePatterns { priority, sections } => {
                let sat_criterion =
                    SatCriterion::DifferentTimePatterns { priority: *priority, sections: sections.clone() };
                criteria_by_priority[*priority as usize].push(sat_criterion);
            }
        }
    }

    criteria_by_priority
}

// Decode SAT solution into a Schedule
fn decode_solution(
    solution: Assignment,
    input: &Input,
    var_to_section_time: &HashMap<Var, (usize, usize)>,
    var_to_section_room: &HashMap<Var, (usize, usize)>,
) -> Schedule {
    let mut schedule = Schedule::new(input);

    // Group assignments by section
    let mut section_assignments: HashMap<usize, (Option<usize>, Option<usize>)> = HashMap::new();

    // Process time slot assignments
    for (&var, &(section, time_slot)) in var_to_section_time {
        if solution.var_value(var).to_bool_with_def(false) {
            let entry = section_assignments.entry(section).or_insert((None, None));
            entry.0 = Some(time_slot);
        }
    }

    // Process room assignments
    for (&var, &(section, room)) in var_to_section_room {
        if solution.var_value(var).to_bool_with_def(false) {
            let entry = section_assignments.entry(section).or_insert((None, None));
            entry.1 = Some(room);
        }
    }

    // Place sections in schedule using move_section to properly update scores
    for (section, (time_slot_opt, room_opt)) in section_assignments {
        if let Some(time_slot) = time_slot_opt {
            _ = move_section(input, &mut schedule, section, time_slot, &room_opt);
        }
    }

    schedule
}

// Solve at each priority level
fn solve_at_priority_level(
    input: &Input,
    criteria_by_priority: &[Vec<SatCriterion>],
    best_schedule: &mut Schedule,
    priority: usize,
    max_violations: &[usize], // Maximum allowed violations for each prior priority level
    solver_type: &str,
) -> Result<(bool, usize)> {
    let criteria_count = criteria_by_priority[priority].len();

    println!("Solving for priority level {} with {} criteria", priority, criteria_count);
    if criteria_count == 0 && priority > 0 {
        println!("    no criteria at priority level {}, skipping", priority);
        return Ok((true, 0)); // Successfully "solved" with 0 violations
    }

    // Start with attempting to satisfy all criteria at this level
    let mut k = 0;
    let mut solution_found = false;

    // Try with increasing values of k until a solution is found
    while !solution_found {
        // Create a new encoder for this attempt
        let mut encoder = SATEncoder::new();

        // Create variables for the SAT problem
        encoder.initialize_variables(input);

        // Encode basic constraints (each section gets one time slot, etc.)
        encoder.encode_basic_constraints(input)?;

        // Encode room conflicts
        encoder.encode_room_conflicts(input);

        // Encode hard conflicts
        encoder.encode_hard_conflicts(input);

        // For each priority level up to current
        // (this skips level 0, which is hard conflicts)
        for p in 1..=priority {
            // Create criterion variables and track which ones were created
            let mut criterion_vars = Vec::new();

            // For prior priority levels, use the established maximum violations
            let max_violations_for_level = if p < priority {
                max_violations[p]
            } else {
                k // For current priority level, try with current k value
            };

            // For each criterion at this priority level
            for criterion in &criteria_by_priority[p] {
                // Encode the criterion
                if let Some(criterion_var) = encoder.encode_criterion(input, criterion, max_violations_for_level > 0)? {
                    criterion_vars.push(criterion_var.pos_lit());
                }
            }

            // Only encode at-most-k constraint if we have more criterion variables than permitted violations
            if criterion_vars.len() > max_violations_for_level {
                encoder.encode_at_most_k(&criterion_vars, max_violations_for_level)?;
            }
        }

        println!(
            "    priority {}, k={} encoded with {} variables and {} clauses, solving with {}",
            priority,
            k,
            encoder.var_manager.n_used(),
            encoder.cnf.len(),
            solver_type
        );

        // Solve with the appropriate solver
        match solver_type {
            "kissat" => {
                let mut solver = Kissat::default();
                solver.add_cnf(encoder.cnf.clone())?;

                match solver.solve()? {
                    SolverResult::Sat => {
                        let solution = solver.full_solution()?;
                        *best_schedule = decode_solution(
                            solution,
                            input,
                            &encoder.var_to_section_time,
                            &encoder.var_to_section_room,
                        );
                        solution_found = true;
                    }
                    _ => {
                        k += 1;
                    }
                }
            }
            "cadical" => {
                let mut solver = CaDiCaL::default();
                solver.add_cnf(encoder.cnf.clone())?;

                match solver.solve()? {
                    SolverResult::Sat => {
                        let solution = solver.full_solution()?;
                        *best_schedule = decode_solution(
                            solution,
                            input,
                            &encoder.var_to_section_time,
                            &encoder.var_to_section_room,
                        );
                        solution_found = true;
                    }
                    _ => {
                        k += 1;
                    }
                }

                // we must find a solution for hard conflicts with no violations
                if priority == 0 && !solution_found {
                    break;
                }
            }
            _ => {
                return Err(format!("Unknown SAT solver: {} (valid values are cadical and kissat)", solver_type).into());
            }
        }
    }

    if !solution_found {
        println!("    could not find a solution for priority level {}", priority);
        return Ok((false, 0));
    }

    Ok((true, k))
}

// Main driver function to generate a schedule using iterative SAT solving
pub fn generate_schedule(input: &Input, solver_type: &str) -> Result<Schedule> {
    // Transform criteria from the input
    let criteria_by_priority = transform_criteria(input);

    // Store the best schedule found
    let mut best_schedule = Schedule::new(input);

    // Keep track of minimum violations required at each priority level
    let mut max_violations = vec![0; PRIORITY_LEVELS];

    // Iteratively solve for each priority level (starting with 0 which is hard constraints)
    for priority in 0..PRIORITY_LEVELS {
        // Solve at this priority level
        match solve_at_priority_level(
            input,
            &criteria_by_priority,
            &mut best_schedule,
            priority,
            &max_violations,
            solver_type,
        )? {
            (true, k) => {
                // Update max violations for this level
                max_violations[priority] = k;
            }
            (false, _) => {
                println!("Failed to find solution at priority level {}, keeping best schedule so far", priority);
                break;
            }
        };
    }

    println!("Final solution maximum violations per priority level:");
    print_max_violations(&max_violations);

    Ok(best_schedule)
}

// Helper function to display maximum violations
fn print_max_violations(max_violations: &[usize]) {
    for (priority, &violations) in max_violations.iter().enumerate() {
        if violations > 0 || priority == 0 {
            println!("  Priority level {}: {} violations", priority, violations);
        }
    }
}
