pub mod defs;
pub mod input;
//pub mod score;
//pub mod solver;
//use self::defs::*;
use self::input::*;
//use self::solver::*;

const DB_PATH: &str = "data/timetable.db";

fn main() {
    let departments = vec!["Computing".to_string(), "Math".to_string()];
    let input = match setup(DB_PATH, &departments) {
        Ok(t) => t,
        Err(msg) => {
            println!("Error in the input: {}", msg);
            return;
        }
    };

    if departments.is_empty() {
        print!("{} for all departments: ", input.term_name);
    } else if departments.len() == 1 {
        print!("{} for {}: ", input.term_name, departments[0]);
    } else {
        let mut sep = "";
        print!("{} for ", input.term_name);
        for (i, name) in departments.iter().enumerate() {
            print!("{}{}", sep, name);
            if i+2 == departments.len() && i >= 1 {
                sep = ", and ";
            } else if i+2 == departments.len() {
                sep = " and ";
            } else {
                sep = ", ";
            }
        }
        print!(": ");
    }
    println!("{} rooms, {} time slots", input.rooms.len(), input.time_slots.len());

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
        println!("faculty: {faculty}");
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
        for elt in &section.hard_conflicts {
            print!("{sep}{elt}");
            sep = " ";
        }
        if !section.hard_conflicts.is_empty() {
            println!();
        }
        sep = "    soft conflicts: ";
        for elt in &section.soft_conflicts {
            print!("{sep}{}:{}", elt.section, elt.priority);
            sep = " ";
        }
        if !section.soft_conflicts.is_empty() {
            println!();
        }
    }

    for (priority, single, group) in &input.anticonflicts {
        print!("anticonflict:{priority} {single} vs");
        for elt in group {
            print!(" {elt}");
        }
        println!();
    }

    //let iterations = 0;
    //solve(&mut solver, iterations);

}
