use super::input::*;
use super::solver::*;

pub fn place(
    solver: &mut Solver,
    course_raw: &str,
    room_raw: &str,
    time_slot_raw: &str,
) -> Result<(), String> {
    let mut section_list = find_sections_by_name(solver, course_raw)?;
    if section_list.len() != 1 {
        return Err(format!(
            "place_static could not find a single entry for course {}",
            course_raw
        ));
    }

    let section = section_list.pop().unwrap();
    let room = find_room_by_name(solver, room_raw)?;
    let time_slot = find_time_slot_by_name(solver, time_slot_raw)?;
    let room_time = solver.sections[section]
        .room_times
        .iter()
        .find_map(|rtp| {
            if rtp.room == room && rtp.time_slot == time_slot {
                Some(rtp.clone())
            } else {
                None
            }
        })
        .expect(&format!("no rtp found for {course_raw}"));

    // place the section
    let undo = PlacementLog::move_section(solver, section, room_time);
    solver.unplaced_best = std::cmp::min(solver.unplaced_best, solver.unplaced_current);
    for elt in &undo.entries {
        if let &PlacementEntry::Remove(
            loser,
            RoomTimeWithPenalty {
                room: loser_room,
                time_slot: loser_time_slot,
                ..
            },
        ) = elt
        {
            println!(
                "section {} in {} at {} displaced by {} in {} at {} in initial load",
                solver.input_sections[loser].name,
                solver.rooms[loser_room].name,
                solver.time_slots[loser_time_slot].name,
                solver.input_sections[section].name,
                solver.rooms[room].name,
                solver.time_slots[time_slot].name,
            );
        }
    }

    Ok(())
}

pub fn place_static(solver: &mut Solver) -> Result<(), String> {
    println!("setting static placement for {}", solver.name);

    
    Ok(())
}
