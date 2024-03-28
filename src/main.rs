pub mod bits;
pub mod data;
pub mod input;
pub mod score;
pub mod solver;
pub mod static_placement;
use self::data::*;
use self::input::*;
use self::solver::*;
use self::static_placement::*;

fn main() {
    let mut solver = match setup() {
        Ok(t) => t,
        Err(msg) => {
            println!("Error in the input: {}", msg);
            return;
        }
    };

    match solver.lock_input() {
        Err(msg) => {
            eprintln!("{}", msg);
            return;
        }
        _ => (),
    }
    /*
    for (sec_i, sec) in input.input_sections.iter().enumerate() {
        let solve = &solver.sections[sec_i];
        print!("{}", sec.get_name());
        if !sec.cross_listings.is_empty() {
            for &other in sec.cross_listings.iter() {
                if sec_i == other {
                    continue;
                }
                print!(" x {}", input.input_sections[other].get_name());
            }
        }
        print!(" [");
        let mut sep = "";
        for &inst_i in &solve.instructors {
            print!("{sep}{}", &input.instructors[inst_i].name);
            sep = ", ";
        }
        print!("]");
        let mut prev_room = input.rooms.len();
        for rtp in solve.room_times.iter() {
            if rtp.room != prev_room {
                prev_room = rtp.room;
                print!("\n    {}:", &input.rooms[rtp.room].name);
            }
            print!(" {}", &input.time_slots[rtp.time_slot].name);
            if rtp.penalty > 0 {
                print!(":{}", rtp.penalty);
            }
        }
        println!();
        if !solve.hard_conflicts.is_empty() {
            print!("    hard conflicts:");
            for &i in solve.hard_conflicts.iter() {
                print!(" {}", input.input_sections[i].get_name());
            }
            println!();
        }
        for elt in &solve.score_criteria {
            println!("    {}", elt.debug(&input))
        }
    }
    */

    /*
    if !input.missing.is_empty() {
        print!("unknown courses:");
        let mut sep = " ";
        for elt in &input.missing {
            print!("{}{}", sep, elt);
            sep = ", ";
        }
        println!();
    }
    */
    println!(
        "{} rooms, {} time slots, {} instructors, {} sections",
        solver.rooms.len(),
        solver.time_slots.len(),
        solver.instructors.len(),
        solver.input_sections.len(),
    );

    /*
    for section in &solver.input_sections {
        println!(
            "section {} with {} rooms and {} times",
            section.get_name(),
            section.rooms.len(),
            section.time_slots.len()
        );
    }
    */

    // set up the static schedule
    //place_static(&mut solver).unwrap();

    let iterations = 50_000_000;
    solve(&mut solver, iterations);

    dump_cs(&solver);
}

fn dump_cs(solver: &Solver) {
    println!("course_data = {{");
    for i in 0..solver.input_sections.len() {
        let section = &solver.input_sections[i];
        if section.prefix != "CS" && section.prefix != "SE" && section.prefix != "IT" {
            continue;
        }
        let solsec = &solver.sections[i];
        println!("    \"{}\": {{", section.get_name());
        println!("        \"room_times\": {{");
        for &RoomTimeWithPenalty {
            room,
            time_slot,
            penalty,
        } in &solsec.room_times
        {
            println!(
                "            (\"{}\", \"{}\", {}),",
                solver.rooms[room].name, solver.time_slots[time_slot].name, penalty
            );
        }
        println!("        }},");
        println!("        \"hard\": {{");
        for &hard in &solsec.hard_conflicts {
            let other = &solver.input_sections[hard];
            if other.prefix != "CS" && other.prefix != "SE" && other.prefix != "IT" {
                continue;
            }
            println!("            \"{}\",", other.get_name());
        }
        println!("        }},");
        println!("        \"soft\": {{");
        for &SectionWithPenalty {
            section: sec,
            penalty,
        } in &solsec.soft_conflicts
        {
            let other = &solver.input_sections[sec];
            if other.prefix != "CS" && other.prefix != "SE" && other.prefix != "IT" {
                continue;
            }
            println!("            \"{}\": {},", other.get_name(), penalty);
        }
        println!("        }},");
        println!("    }},");
    }
    println!("}}");
}
