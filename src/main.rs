pub mod input;
pub mod score;
pub mod solver;
use self::input::*;
use self::solver::*;

const DB_PATH: &str = "timetable.db";

fn main() {
    // load input
    let departments = Vec::new();//vec!["Computing".to_string()];
    let input = match setup(DB_PATH, &departments) {
        Ok(t) => t,
        Err(msg) => {
            println!("Error in the input: {}", msg);
            return;
        }
    };
    if false {
        dump_input(&departments, &input);
    }

    let start = std::time::Instant::now();
    let warmup_seconds = 10;
    let solve_seconds = 10;

    if let Some(mut schedule) = warmup(&input, start, warmup_seconds) {
        println!("\nwarmup finished with score {}", schedule.score);
        solve(&mut schedule, &input, start, solve_seconds);
    };
}

fn dump_input(departments: &Vec<String>, input: &Input) {
    if departments.is_empty() {
        print!("{} for all departments: ", input.term_name);
    } else if departments.len() == 1 {
        print!("{} for {}: ", input.term_name, departments[0]);
    } else {
        let mut sep = "";
        print!("{} for ", input.term_name);
        for (i, name) in departments.iter().enumerate() {
            print!("{}{}", sep, name);
            if i + 2 == departments.len() && i >= 1 {
                sep = ", and ";
            } else if i + 2 == departments.len() {
                sep = " and ";
            } else {
                sep = ", ";
            }
        }
        print!(": ");
    }
    println!(
        "{} rooms, {} time slots",
        input.rooms.len(),
        input.time_slots.len()
    );

    print!("\nrooms: ");
    let mut sep = "";
    for elt in &input.rooms {
        print!("{sep}{elt}");
        sep = ", ";
    }
    println!();
    print!("\ntime slots: ");

    sep = "";
    for elt in &input.time_slots {
        print!("{sep}{elt}");
        sep = ", ";
    }
    println!();

    println!();
    for faculty in &input.faculty {
        println!("faculty: {}", faculty.debug(&input));
        for dist in &faculty.distribution {
            println!("    {dist}");
        }
    }

    for section in &input.sections {
        print!(
            "section {} with {} rooms and {} times",
            section.name,
            section.rooms.len(),
            section.time_slots.len()
        );
        if !section.faculty.is_empty() {
            print!(", faculty");
            for faculty in &section.faculty {
                print!(" {faculty}");
            }
        }
        println!();
        let mut sep = "    hard conflicts: ";
        for &elt in &section.hard_conflicts {
            print!("{}{}", sep, input.sections[elt].name);
            sep = " ";
        }
        if !section.hard_conflicts.is_empty() {
            println!();
        }
        for elt in &section.score_criteria {
            println!("    {}", elt.debug(&input));
        }
    }

    for (priority, single, group) in &input.anticonflicts {
        print!(
            "anticonflict:{priority} {} vs",
            input.sections[*single].name
        );
        let mut sep = " ";
        for &elt in group {
            print!("{}{}", sep, input.sections[elt].name);
            sep = ", ";
        }
        println!();
    }

    //let iterations = 0;
    //solve(&mut solver, iterations);
}
