pub mod input;
pub mod score;
pub mod solver;
use self::input::*;
use self::score::*;
use self::solver::*;

const DB_PATH: &str = "timetable.db";

fn main() {
    // load input
    let departments = vec!["Computing".to_string()];
    let input = match load_input(DB_PATH, &departments) {
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
        print_schedule(&input, &schedule);
        print_problems(&input, &schedule);
        if false {
            solve(&mut schedule, &input, start, warmup_seconds + solve_seconds);
        }
    };
}

fn dump_input(departments: &[String], input: &Input) {
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
        println!("faculty: {}", faculty.debug(input));
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
            println!("    {}", elt.debug(input));
        }
    }
}

fn print_schedule(input: &Input, schedule: &Schedule) {
    let mut grid = Vec::new();
    let mut width = 1;
    for _ in 0..=input.time_slots.len() {
        grid.push(vec!["".to_string(); input.rooms.len() + 1]);
    }
    for (i, room) in input.rooms.iter().enumerate() {
        grid[0][i + 1] = room.name.clone();
        width = std::cmp::max(room.name.len(), width);
    }
    for (i, time_slot) in input.time_slots.iter().enumerate() {
        grid[i + 1][0] = time_slot.name.clone();
        width = std::cmp::max(time_slot.name.len(), width);
    }
    for (
        section,
        Placement {
            time_slot, room, ..
        },
    ) in schedule.placements.iter().enumerate()
    {
        let (Some(time_slot), Some(room)) = (time_slot, room) else {
            continue;
        };
        if !grid[time_slot + 1][room + 1].is_empty() {
            panic!("two sections schedule in same room and time");
        }
        grid[time_slot + 1][room + 1] = input.sections[section].name.clone();
        width = std::cmp::max(input.sections[section].name.len(), width);
    }
    width += 2;

    for (i, row) in grid.iter().enumerate() {
        let mut div = "+".to_string();
        let mut elt = "|".to_string();
        for column in row {
            for _ in 0..width {
                div.push('-');
            }
            div.push('+');
            elt = format!("{}{:^width$}|", elt, column, width = width);
        }
        if i == 0 {
            println!("{}", div);
        }
        println!("{}", elt);
        println!("{}", div);
    }
}

fn print_problems(input: &Input, schedule: &Schedule) {
    let mut lst = Vec::new();
    for (section, placement) in schedule.placements.iter().enumerate() {
        if placement.time_slot.is_none() {
            lst.push((
                LEVEL_FOR_UNPLACED_SECTION,
                format!("{} is not placed", input.sections[section].name),
            ));
            continue;
        }
        for delta in &placement.score.deltas {
            if let (_, Some(_)) = delta.get_scores(section) {
                lst.push(delta.get_score_message(input, schedule, section));
            }
        }
    }
    lst.sort();
    for (priority, msg) in lst {
        println!("{priority:2}: {msg}");
    }
}
