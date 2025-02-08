use super::input::*;
use super::score::*;
use super::solver::*;
use std::cmp::max;

pub fn print_schedule(input: &Input, schedule: &Schedule) {
    let mut rooms: Vec<usize> = schedule.placements.iter().filter_map(|Placement { room, .. }| *room).collect();
    rooms.sort_unstable();
    rooms.dedup();
    let mut time_slots: Vec<usize> = schedule
        .placements
        .iter()
        .filter_map(|Placement { time_slot, room, .. }| if room.is_some() { *time_slot } else { None })
        .collect();
    time_slots.sort_unstable();
    time_slots.dedup();
    let mut grid = Vec::new();
    let mut width = 1;
    for _ in 0..=time_slots.len() {
        grid.push(vec![("".to_string(), "".to_string()); rooms.len() + 1]);
    }
    for (i, &room) in rooms.iter().enumerate() {
        let name = input.rooms[room].name.clone();
        width = max(name.len(), width);
        grid[0][i + 1] = (name, "".to_string());
    }
    for (i, &time_slot) in time_slots.iter().enumerate() {
        let name = input.time_slots[time_slot].name.clone();
        width = max(name.len(), width);
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
        width = max(section_name.len(), width);
        width = max(faculty_name.len(), width);
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

pub fn print_problems(input: &Input, schedule: &Schedule) {
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
    lst.sort_unstable_by(|a, b| {
        if a.0 != b.0 && (a.0 < START_LEVEL_FOR_PREFERENCES || b.0 < START_LEVEL_FOR_PREFERENCES) {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });
    for (priority, msg) in lst {
        if priority < START_LEVEL_FOR_PREFERENCES {
            println!("{priority:2}: {msg}");
        } else {
            println!("{msg}");
        }
    }
}

pub fn dump_input(departments: &[String], input: &Input) {
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

    print!("\nRooms: ");
    let mut sep = "";
    for elt in &input.rooms {
        print!("{sep}{elt}");
        sep = ", ";
    }
    println!();
    print!("\nTime slots: ");
    sep = "";
    for elt in &input.time_slots {
        print!("{sep}{elt}");
        sep = ", ";
    }
    println!();

    println!("\nFaculty:");
    for faculty in &input.faculty {
        println!("{}", faculty.debug(input));
    }

    println!("\nSections:");
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

    println!("\nScoring criteria:");
    for elt in &input.criteria {
        println!("{}", elt.debug(input));
    }
}

pub fn commas<T: TryInto<i64>>(n: T) -> String {
    let mut n = n.try_into().unwrap_or(0);
    let mut minus = "";
    if n < 0 {
        n = -n;
        minus = "-";
    }
    let mut s = String::new();
    loop {
        if n < 1000 {
            s = format!("{}{}", n, s);
            break;
        }
        s = format!(",{:03}{}", n % 1000, s);
        n /= 1000;
    }
    format!("{minus}{s}")
}

pub fn ms_to_string(ms: u128) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 10000 {
        format!("{:.1}s", (ms as f64) / 1000.0)
    } else {
        sec_to_string((ms as u64) / 1000)
    }
}

pub fn string_to_sec(duration: &str) -> Result<u64, String> {
    let mut seconds = 0;
    let mut digits = 0;
    for ch in duration.chars() {
        match ch {
            '0'..='9' => {
                digits *= 10;
                digits += ch.to_digit(10).unwrap();
            }
            'h' => {
                seconds += digits * 60 * 60;
                digits = 0;
            }
            'm' => {
                seconds += digits * 60;
                digits = 0;
            }
            's' => {
                seconds += digits;
                digits = 0;
            }
            _ => return Err(format!("failed to parse {duration}; expected, e.g., 2h5m13s")),
        }
    }
    if digits != 0 {
        Err(format!("failed to parse {duration}; expected, e.g.: 2h5m13s but found extra digits at end"))
    } else {
        Ok(seconds as u64)
    }
}

pub fn sec_to_string(seconds: u64) -> String {
    if seconds < 60 {
        return format!("{}s", seconds);
    }
    if seconds < 3600 && seconds % 60 == 0 {
        return format!("{}m", seconds / 60);
    }
    if seconds < 3600 {
        return format!("{}m{:02}s", seconds / 60, seconds % 60);
    }
    if seconds % 3600 == 0 {
        return format!("{}h", seconds / 3600);
    }
    if seconds % 60 == 0 {
        return format!("{}h{}m", seconds / 3600, (seconds % 3600) / 60);
    }
    format!("{}h{:02}m{:02}s", seconds / 3600, (seconds % 3600) / 60, seconds % 60)
}
