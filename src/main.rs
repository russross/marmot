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
    let warmup_seconds = 1;
    let total_seconds = 31;

    println!("running warmup for {} seconds", warmup_seconds);
    let Some(mut schedule) = warmup(&input, start, warmup_seconds) else {
        println!("failed to generate a schedule in the warmup stage");
        return;
    };

    let best = solve(&input, &mut schedule, start, total_seconds);
    print_schedule(&input, &best);
    print_problems(&input, &best);
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
        println!("faculty: {}", faculty.debug(input));
    }

    for section in &input.sections {
        print!("section {} with {} rooms and {} times", section.name, section.rooms.len(), section.time_slots.len());
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
    }

    for elt in &input.criteria {
        println!("    {}", elt.debug(input));
    }
}

fn print_schedule(input: &Input, schedule: &Schedule) {
    let mut rooms: Vec<usize> = schedule.placements.iter().filter_map(|Placement { room, .. }| *room).collect();
    rooms.sort_unstable();
    rooms.dedup();
    let mut time_slots: Vec<usize> = schedule.placements.iter().filter_map(|Placement { time_slot, ..}| *time_slot).collect();
    time_slots.sort_unstable();
    time_slots.dedup();
    let mut grid = Vec::new();
    let mut width = 1;
    for _ in 0..=time_slots.len() {
        grid.push(vec![("".to_string(), "".to_string()); rooms.len() + 1]);
    }
    for (i, &room) in rooms.iter().enumerate() {
        let name = input.rooms[room].name.clone();
        width = std::cmp::max(name.len(), width);
        grid[0][i + 1] = (name, "".to_string());
    }
    for (i, &time_slot) in time_slots.iter().enumerate() {
        let name = input.time_slots[time_slot].name.clone();
        width = std::cmp::max(name.len(), width);
        grid[i + 1][0] = (name, "".to_string());
    }
    for (section, Placement { time_slot, room, .. }) in schedule.placements.iter().enumerate() {
        let (Some(time_slot), Some(room)) = (time_slot, room) else {
            continue;
        };
        let x = rooms.binary_search(room).unwrap() + 1;
        let y = time_slots.binary_search(time_slot).unwrap() + 1;
        if grid[y][x] != ("".to_string(), "".to_string()) {
            panic!("two sections schedule in same room and time");
        }
        let sec = &input.sections[section];
        let section_name = sec.name.clone();
        let faculty_name = match sec.faculty.len() {
            0 => "".to_string(),
            1 => input.faculty[sec.faculty[0]].name.clone(),
            _ => format!("{}+", input.faculty[sec.faculty[0]].name.clone()),
        };
        width = std::cmp::max(section_name.len(), width);
        width = std::cmp::max(faculty_name.len(), width);
        grid[y][x] = (section_name, faculty_name);
    }
    width += 2;

    for (i, row) in grid.iter().enumerate() {
        let mut div = "+".to_string();
        let mut sec = "|".to_string();
        let mut fac = "|".to_string();
        for (sec_name, fac_name) in row {
            for _ in 0..width {
                div.push('-');
            }
            div.push('+');
            sec = format!("{} {:<width$}|", sec, sec_name, width = width - 1);
            fac = format!("{} {:<width$}|", fac, fac_name, width = width - 1);
        }
        if i == 0 {
            println!("{}", div);
        }
        println!("{}", sec);
        println!("{}", fac);
        println!("{}", div);
    }
    for (section, Placement { time_slot, room, .. }) in schedule.placements.iter().enumerate() {
        let (&Some(time_slot), None) = (time_slot, room) else {
            continue;
        };
        println!("{} at {} with no room", input.sections[section].name, input.time_slots[time_slot].name);
    }
}

fn print_problems(input: &Input, schedule: &Schedule) {
    let mut lst = Vec::new();
    for (section, placement) in schedule.placements.iter().enumerate() {
        if placement.time_slot.is_none() {
            lst.push((LEVEL_FOR_UNPLACED_SECTION, format!("{} is not placed", input.sections[section].name)));
            continue;
        }
    }
    for penalty_list in &schedule.penalties {
        for penalty in penalty_list {
            lst.push(penalty.get_score_message(input, schedule));
        }
    }
    lst.sort_unstable();
    for (priority, msg) in lst {
        if priority >= 20 {
            continue;
        }
        println!("{priority:2}: {msg}");
    }
}
