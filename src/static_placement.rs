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
    if !is_primary_cross_listing(solver, section) {
        //println!("skipping non-primary course {course_raw}");
        return Ok(());
    }

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
                solver.input_sections[loser].get_name(),
                solver.rooms[loser_room].name,
                solver.time_slots[loser_time_slot].name,
                solver.input_sections[section].get_name(),
                solver.rooms[room].name,
                solver.time_slots[time_slot].name,
            );
        }
    }

    Ok(())
}

pub fn place_static(solver: &mut Solver) -> Result<(), String> {
    place(solver, "CS 1030-01", "Smith 107", "TR1330+75")?;
    place(solver, "CS 1400-01", "Smith 117", "T1800+150")?;
    place(solver, "CS 1400-02", "Smith 108", "TR1330+75")?;
    place(solver, "CS 1400-03", "Smith 108", "MWF1000+50")?;
    place(solver, "CS 1400-50", "Smith 117", "TR1630+75")?;
    place(solver, "CS 1410-01", "Smith 108", "MW1330+75")?;
    place(solver, "CS 1410-02", "Smith 107", "MWF0900+50")?;
    place(solver, "CS 2420-01", "Smith 117", "MWF1100+50")?;
    place(solver, "CS 2450-01", "Smith 108", "TR1500+75")?;
    place(solver, "CS 2450-02", "Smith 108", "MWF0800+50")?;
    place(solver, "CS 2810-01", "Smith 109", "MW1330+75")?;
    place(solver, "CS 2810-02", "Smith 109", "MW1200+75")?;
    place(solver, "CS 3005-01", "Smith 116", "MWF1000+50")?;
    place(solver, "CS 3150-01", "Smith 109", "MWF1100+50")?;
    place(solver, "CS 3310-01", "Smith 113", "MWF0900+50")?;
    place(solver, "CS 3410-01", "Smith 109", "TR1200+75")?;
    place(solver, "CS 3510-01", "Smith 109", "MWF0900+50")?;
    place(solver, "CS 3600-01", "Smith 117", "MWF1000+50")?;
    place(solver, "CS 4307-01", "Smith 109", "TR1330+75")?;
    place(solver, "CS 4320-01", "Smith 109", "TR0900+75")?;
    place(solver, "CS 4550-01", "Smith 113", "MW1500+75")?;
    place(solver, "CS 4600-01", "Smith 116", "MW1200+75")?;
    place(solver, "CS 4600-02", "Smith 108", "MW1200+75")?;
    place(solver, "CS 4991R-50", "Smith 116", "R1900+50")?;
    place(solver, "CS 4992R-01", "Smith 109", "F1300+50")?;
    place(solver, "IT 1100-01", "Smith 113", "TR1330+75")?;
    place(solver, "IT 1100-02", "Smith 113", "TR0900+75")?;
    place(solver, "IT 1200-01", "Smith 107", "TR1030+75")?;
    place(solver, "IT 2300-01", "Smith 113", "MWF0800+50")?;
    place(solver, "IT 2300-02", "Smith 113", "MWF1000+50")?;
    place(solver, "IT 2400-01", "Smith 107", "MWF1100+50")?;
    place(solver, "IT 2700-01", "Smith 107", "TR1500+75")?;
    place(solver, "IT 3100-01", "Smith 107", "MW1200+75")?;
    place(solver, "IT 3110-01", "Smith 108", "MWF1100+50")?;
    place(solver, "IT 3150-01", "Smith 107", "MW1500+75")?;
    place(solver, "IT 3400-01", "Smith 107", "TR1200+75")?;
    place(solver, "IT 4510-01", "Smith 108", "R1800+150")?;
    place(solver, "IT 4600-01", "Smith 108", "MWF0900+50")?;
    place(solver, "IT 4990-01", "Smith 108", "W1800+150")?;
    place(solver, "MATH 0900-01", "Snow 151", "TR1300+100")?;
    place(solver, "MATH 0900-02", "Snow 144", "TWRF1200+50")?;
    place(solver, "MATH 0900-03", "Snow 147", "MTWF1200+50")?;
    place(solver, "MATH 0900-04", "Snow 144", "MW1600+100")?;
    place(solver, "MATH 0900-06", "Snow 144", "TR1630+100")?;
    place(solver, "MATH 0900-07", "Snow 3", "MW1300+100")?;
    place(solver, "MATH 0900-51", "Snow 147", "MW1800+100")?;
    place(solver, "MATH 0980-03", "Snow 145", "MTWR1000+50")?;
    place(solver, "MATH 0980-05", "Snow 151", "TR1630+100")?;
    place(solver, "MATH 0980-06", "Snow 112", "MTWF0800+50")?;
    place(solver, "MATH 0980-07", "Snow 145", "MW1500+100")?;
    place(solver, "MATH 0980-08", "Snow 3", "TR1300+100")?;
    place(solver, "MATH 0980-10", "Snow 112", "MW1630+100")?;
    place(solver, "MATH 1010-03", "Snow 145", "TWRF1100+50")?;
    place(solver, "MATH 1010-04", "Snow 151", "MW1300+100")?;
    place(solver, "MATH 1010-05", "Snow 125", "MW1500+100")?;
    place(solver, "MATH 1010-06", "Snow 145", "TR1500+100")?;
    place(solver, "MATH 1010-07", "Snow 150", "MWRF1000+50")?;
    place(solver, "MATH 1010-50", "Snow 147", "TR1800+100")?;
    place(solver, "MATH 1030-01", "Snow 112", "MW1500+75")?;
    place(solver, "MATH 1030-02", "Snow 147", "TR1330+75")?;
    place(solver, "MATH 1030-03", "Snow 124", "TR1500+75")?;
    place(solver, "MATH 1030-04", "Snow 147", "TR1630+75")?;
    place(solver, "MATH 1030-05", "Snow 3", "MWF1000+50")?;
    place(solver, "MATH 1030-06", "Snow 147", "MWF0800+50")?;
    place(solver, "MATH 1040-01", "Snow 147", "MWF1000+50")?;
    place(solver, "MATH 1040-02", "Snow 125", "TR1030+75")?;
    place(solver, "MATH 1040-03", "Snow 147", "MWF1100+50")?;
    place(solver, "MATH 1040-04", "Snow 3", "MWF1200+50")?;
    place(solver, "MATH 1040-05", "Snow 125", "MW1200+75")?;
    place(solver, "MATH 1040-06", "Snow 3", "TR1500+75")?;
    place(solver, "MATH 1040-07", "Snow 151", "MW1500+75")?;
    place(solver, "MATH 1040-08", "Snow 124", "MWF0900+50")?;
    place(solver, "MATH 1040-09", "Snow 147", "TR1500+75")?;
    place(solver, "MATH 1040-10", "Snow 125", "MWF1100+50")?;
    place(solver, "MATH 1040-11", "Snow 3", "TR1630+75")?;
    place(solver, "MATH 1040-12", "Snow 145", "TR0730+75")?;
    place(solver, "MATH 1040-14", "Snow 150", "MW1630+75")?;
    place(solver, "MATH 1050-01", "Snow 125", "MTWF0800+50")?;
    place(solver, "MATH 1050-02", "Snow 145", "MTWR0900+50")?;
    place(solver, "MATH 1050-03", "Snow 3", "F1100+50")?;
    place(solver, "MATH 1050-03-ALT", "Snow 3", "MTWR1100+50")?;
    place(solver, "MATH 1050-04", "Snow 3", "MTWR0800+50")?;
    place(solver, "MATH 1050-05", "Snow 125", "TR1500+100")?;
    place(solver, "MATH 1050-06", "Snow 147", "MTWR0900+50")?;
    place(solver, "MATH 1060-01", "Snow 151", "TR0900+75")?;
    place(solver, "MATH 1060-02", "Snow 124", "TR1030+75")?;
    place(solver, "MATH 1080-01", "Snow 150", "MTWRF1100+50")?;
    place(solver, "MATH 1100-02", "Snow 150", "MW1200+75")?;
    place(solver, "MATH 1210-01", "Snow 145", "TR1300+100")?;
    place(solver, "MATH 1210-02", "Snow 3", "MW1500+100")?;
    place(solver, "MATH 1210-03", "Snow 125", "MWRF0900+50")?;
    place(solver, "MATH 1220-01", "Snow 145", "MTWF1200+50")?;
    place(solver, "MATH 1220-02", "Snow 147", "MW1300+100")?;
    place(solver, "MATH 2010-01", "Snow 124", "T1630+150")?;
    place(solver, "MATH 2020-01", "Snow 144", "TR0900+75")?;
    place(solver, "MATH 2020-02", "Snow 151", "W1630+150")?;
    place(solver, "MATH 2200-01", "Snow 151", "MWF0900+50")?;
    place(solver, "MATH 2210-01", "Snow 124", "TWRF1200+50")?;
    place(solver, "MATH 2250-01", "Snow 144", "MWRF1100+50")?;
    place(solver, "MATH 2270-01", "Snow 147", "TR1030+75")?;
    place(solver, "MATH 2280-01", "Snow 145", "MW1330+75")?;
    place(solver, "MATH 3050-01", "Snow 3", "TR0900+75")?;
    place(solver, "MATH 3200-01", "Snow 144", "MWF0800+50")?;
    place(solver, "MATH 3450-01", "Snow 125", "TR1330+75")?;
    place(solver, "MATH 3900-01", "Snow 112", "TR0900+75")?;
    place(solver, "MATH 4250-01", "Snow 147", "MW1500+100")?;
    place(solver, "MATH 4400-01", "Snow 124", "MWF1000+50")?;
    place(solver, "MATH 4410-01", "Snow 144", "T1500+75")?;
    place(solver, "MATH 4800-01", "Snow 147", "MW1645+75")?;
    place(solver, "SE 1400-01", "Smith 112", "MWF1100+50")?;
    place(solver, "SE 1400-02", "Smith 112", "TR1030+75")?;
    place(solver, "SE 3010-01", "Smith 108", "MW1500+75")?;
    place(solver, "SE 3100-01", "Smith 109", "MW1630+75")?;
    place(solver, "SE 3200-01", "Smith 108", "TR1030+75")?;
    place(solver, "SE 3450-01", "Smith 112", "TR1200+75")?;
    place(solver, "SE 3500-01", "Smith 108", "TR1200+75")?;
    place(solver, "SE 3550-01", "Smith 107", "TR0900+75")?;
    place(solver, "SE 4200-01", "Smith 107", "MW1330+75")?;

    Ok(())
}
