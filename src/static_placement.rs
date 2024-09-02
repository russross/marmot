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
    place(solver, "CS 1030-01", "Smith 107", "MWF1000+50")?;
    place(solver, "CS 1400-01", "Smith 116", "MW1630+75")?;
    place(solver, "CS 1400-02", "Smith 109", "MWF1000+50")?;
    place(solver, "CS 1400-03", "Smith 108", "TR1500+75")?;
    place(solver, "CS 1400-50", "Smith 116", "TR1630+75")?;
    place(solver, "CS 1410-01", "Smith 108", "MW1330+75")?;
    place(solver, "CS 1410-02", "Smith 107", "TR1330+75")?;
    place(solver, "CS 2420-01", "Smith 117", "MWF0900+50")?;
    place(solver, "CS 2450-01", "Smith 107", "MWF1100+50")?;
    place(solver, "CS 2450-02", "Smith 108", "MWF0800+50")?;
    place(solver, "CS 2810-01", "Smith 109", "TR1330+75")?;
    place(solver, "CS 2810-02", "Smith 109", "MW1330+75")?;
    place(solver, "CS 3005-01", "Smith 116", "MWF1000+50")?;
    place(solver, "CS 3150-01", "Smith 109", "MWF0900+50")?;
    place(solver, "CS 3310-01", "Smith 117", "TR1500+75")?;
    place(solver, "CS 3410-01", "Smith 109", "TR1200+75")?;
    place(solver, "CS 3510-01", "Smith 109", "TR1500+75")?;
    place(solver, "CS 3600-01", "Smith 116", "TR1030+75")?;
    place(solver, "CS 4307-01", "Smith 109", "MW1200+75")?;
    place(solver, "CS 4320-01", "Smith 116", "MW1330+75")?;
    place(solver, "CS 4550-01", "Smith 113", "MWF1000+50")?;
    place(solver, "CS 4600-01", "Smith 116", "TR0900+75")?;
    place(solver, "CS 4600-02", "Smith 108", "TR0900+75")?;
    place(solver, "CS 4991R-50", "Smith 116", "R1900+50")?;
    place(solver, "CS 4992R-01", "Smith 109", "F1300+50")?;
    place(solver, "IT 1100-01", "Smith 113", "TR1030+75")?;
    place(solver, "IT 1100-02", "Smith 113", "TR1200+75")?;
    place(solver, "IT 1200-01", "Smith 107", "TR0900+75")?;
    place(solver, "IT 2300-01", "Smith 107", "MW1200+75")?;
    place(solver, "IT 2300-02", "Smith 113", "MW1200+75")?;
    place(solver, "IT 2400-01", "Smith 107", "MW1500+75")?;
    place(solver, "IT 2700-01", "Smith 107", "TR1200+75")?;
    place(solver, "IT 3100-01", "Smith 107", "MWF0900+50")?;
    place(solver, "IT 3110-01", "Smith 108", "MWF0900+50")?;
    place(solver, "IT 3150-01", "Smith 107", "MWF0800+50")?;
    place(solver, "IT 3400-01", "Smith 107", "TR1500+75")?;
    place(solver, "IT 4510-01", "Smith 109", "R1800+150")?;
    place(solver, "IT 4600-01", "Smith 109", "MWF1100+50")?;
    place(solver, "IT 4990-01", "Smith 108", "W1800+150")?;
    place(solver, "SE 1400-01", "Smith 112", "MWF1100+50")?;
    place(solver, "SE 1400-02", "Smith 112", "TR0900+75")?;
    place(solver, "SE 3010-01", "Smith 109", "MW1500+75")?;
    place(solver, "SE 3100-01", "Smith 107", "T1800+150")?;
    place(solver, "SE 3200-01", "Smith 108", "TR1330+75")?;
    place(solver, "SE 3450-01", "Smith 112", "TR1200+75")?;
    place(solver, "SE 3500-01", "Smith 108", "TR1200+75")?;
    place(solver, "SE 3550-01", "Smith 108", "TR1030+75")?;
    place(solver, "SE 4200-01", "Smith 107", "MW1330+75")?;

    Ok(())
}
