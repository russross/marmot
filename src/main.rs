pub mod bits;
pub mod data;
pub mod input;
pub mod score;
pub mod solver;
use self::input::*;
use self::score::*;
use self::solver::*;

fn main() {
    let term = match setup() {
        Ok(t) => t,
        Err(msg) => {
            println!("Error in the input: {}", msg);
            return;
        }
    };

    println!("term: {} from {} to {}", term.name, term.start, term.end);

    /*
    for (i, time_slot) in term.time_slots.iter().enumerate() {
        print!("time slot {}: ", time_slot.name);
        let mut sep = "";
        for elt in &time_slot.days {
            print!("{}{}", sep, elt);
            sep = ", ";
        }
        print!(" at {} for {}", time_slot.start_time, time_slot.duration);
        if time_slot.conflicts.len() > 1 {
            print!(", conflicts with ");
            sep = "";
            for elt in &time_slot.conflicts {
                if *elt == i {
                    continue;
                }
                print!("{}{}", sep, term.time_slots[*elt].name);
                sep = ", ";
            }
        }
        println!();
    }
    for room in &term.rooms {
        print!("{} {} tags:", room.name, room.capacity);
        for tag in &room.tags {
            print!(" {}", tag);
        }
        println!();
    }
    for inst in &term.instructors {
        print!("{}", inst.name);
        for twp in &inst.available_times {
            if twp.penalty == 0 {
                print!(" {}", term.time_slots[twp.time_slot].name);
            } else {
                print!(" {}:{}", term.time_slots[twp.time_slot].name, twp.penalty);
            }
        }
        println!();
    }
    */
    let solver = match Solver::new(&term) {
        Ok(s) => s,
        Err(msg) => {
            eprintln!("{}", msg);
            return;
        }
    };
    for (sec_i, sec) in term.sections.iter().enumerate() {
        let solve = &solver.sections[sec_i];
        print!("{}", sec.get_name());
        if !sec.cross_listings.is_empty() {
            for &other in sec.cross_listings.iter() {
                if sec_i == other {
                    continue;
                }
                print!(" x {}", term.sections[other].get_name());
            }
        }
        print!(" [");
        let mut sep = "";
        for &inst_i in &solve.instructors {
            print!("{sep}{}", &term.instructors[inst_i].name);
            sep = ", ";
        }
        print!("]");
        let mut prev_room = term.rooms.len();
        for rtp in solve.room_times.iter() {
            if rtp.room != prev_room {
                prev_room = rtp.room;
                print!("\n    {}:", &term.rooms[rtp.room].name);
            }
            print!(" {}", &term.time_slots[rtp.time_slot].name);
            if rtp.penalty > 0 {
                print!(":{}", rtp.penalty);
            }
        }
        println!();
        if !solve.hard_conflicts.is_empty() {
            print!("    hard conflicts:");
            for &i in solve.hard_conflicts.iter() {
                print!(" {}", term.sections[i].get_name());
            }
            println!();
        }
        for elt in &solve.score_criteria {
            match elt {
                ScoreCriterion::SoftConflict {
                    sections_with_penalties,
                } => {
                    print!("    soft conflicts:");
                    for elt in sections_with_penalties {
                        print!(" {}:{}", term.sections[elt.section].get_name(), elt.penalty);
                    }
                    println!();
                }

                ScoreCriterion::AntiConflict {
                    penalty,
                    single,
                    group,
                } => {
                    print!(
                        "    anticonflict: penalty {}, {} should be in time slot with ",
                        penalty,
                        term.sections[*single].get_name()
                    );
                    let mut sep = "";
                    for &other in group {
                        print!("{}{}", sep, term.sections[other].get_name());
                        sep = ", ";
                    }
                    println!();
                }

                ScoreCriterion::InstructorClassSpread {
                    instructor,
                    sections,
                    grouped_by_days,
                } => {
                    for group in grouped_by_days {
                        let days = match &group[0] {
                            DistributionPreference::Clustering { days, .. } => days,
                            DistributionPreference::DaysOff { days, .. } => days,
                            DistributionPreference::DaysEvenlySpread { days, .. } => days,
                        };
                        print!("    class spread for ");
                        let mut sep = "";
                        for day in days {
                            match day {
                                time::Weekday::Sunday => print!("{sep}Sun"),
                                time::Weekday::Monday => print!("{sep}Mon"),
                                time::Weekday::Tuesday => print!("{sep}Tues"),
                                time::Weekday::Wednesday => print!("{sep}Wed"),
                                time::Weekday::Thursday => print!("{sep}Thurs"),
                                time::Weekday::Friday => print!("{sep}Fri"),
                                time::Weekday::Saturday => print!("{sep}Sat"),
                            }
                            sep = ", ";
                        }
                        print!(
                            "; instructor {}; sections ",
                            term.instructors[*instructor].name
                        );
                        let mut sep = "";
                        for &sec in sections {
                            print!("{sep}{}", term.sections[sec].get_name());
                            sep = ", ";
                        }
                        println!();
                        for pref in group {
                            match pref {
                                DistributionPreference::Clustering {
                                    max_gap,
                                    cluster_limits,
                                    gap_limits,
                                    ..
                                } => {
                                    if !cluster_limits.is_empty() {
                                        print!("        cluster max:{}", max_gap);
                                        for limit in cluster_limits {
                                            match limit {
                                                DurationWithPenalty::TooShort {
                                                    duration,
                                                    penalty,
                                                } => print!(" [<{} penalty {}]", duration, penalty),
                                                DurationWithPenalty::TooLong {
                                                    duration,
                                                    penalty,
                                                } => print!(" [>{} penalty {}]", duration, penalty),
                                            }
                                        }
                                        println!();
                                    }

                                    if !gap_limits.is_empty() {
                                        print!("        gap");
                                        for limit in gap_limits {
                                            match limit {
                                                DurationWithPenalty::TooShort {
                                                    duration,
                                                    penalty,
                                                } => print!(" [<{} penalty {}]", duration, penalty),
                                                DurationWithPenalty::TooLong {
                                                    duration,
                                                    penalty,
                                                } => print!(" [>{} penalty {}]", duration, penalty),
                                            }
                                        }
                                        println!();
                                    }
                                }

                                DistributionPreference::DaysOff {
                                    days_off, penalty, ..
                                } => {
                                    println!("        days off:{} penalty {}", days_off, penalty);
                                }

                                DistributionPreference::DaysEvenlySpread { penalty, .. } => {
                                    println!("        days evenly spread penalty {}", penalty);
                                }
                            }
                        }
                    }
                }

                ScoreCriterion::InstructorRoomCount {
                    instructor,
                    sections,
                    desired,
                    penalty,
                } => {
                    print!(
                        "    room count: penalty {}, {} should fit in {} rooms, sections: ",
                        penalty, term.instructors[*instructor].name, desired,
                    );
                    let mut sep = "";
                    for &elt in sections {
                        print!("{}{}", sep, term.sections[elt].get_name());
                        sep = ", ";
                    }
                    println!();
                }
            }
        }

        /*
        if !sec.soft_conflicts.is_empty() {
            print!("    soft conflicts:");
            for elt in &sec.soft_conflicts {
                print!(
                    " {}:{}",
                    term.sections[elt.section].get_name(),
                    elt.penalty
                );
            }
            println!();
        }
        */
    }

    if !term.missing.is_empty() {
        print!("unknown courses:");
        let mut sep = " ";
        for elt in &term.missing {
            print!("{}{}", sep, elt);
            sep = ", ";
        }
        println!();
    }
    println!(
        "{} rooms, {} time slots, {} instructors, {} sections",
        term.rooms.len(),
        term.time_slots.len(),
        term.instructors.len(),
        term.sections.len(),
    );

    let iterations = 50_000_000;
    solve(solver, &term, iterations);
}
