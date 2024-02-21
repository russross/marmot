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
        if let &PlacementEntry::Remove(loser, _) = elt {
            println!(
                "section {} displaced by {} in initial load",
                solver.input_sections[loser].get_name(),
                solver.input_sections[section].get_name()
            );
        }
    }

    Ok(())
}

pub fn place_static(solver: &mut Solver) -> Result<(), String> {
    place(solver, "BIOL 1010-01", "BROWN 201", "TR0730+75")?;
    place(solver, "BIOL 1010-02", "BROWN 201", "TR0900+75")?;
    place(solver, "BIOL 1010-03", "SET 301", "MWF0800+50")?;
    place(solver, "BIOL 1010-04", "COE 121", "MWF1000+50")?;
    place(solver, "BIOL 1010-05", "SET 106", "TR1030+75")?;
    place(solver, "BIOL 1010-05-SI", "SNOW 113", "W1030+75")?;
    place(solver, "BIOL 1010-06", "BROWN 201", "MWF1100+50")?;
    place(solver, "BIOL 1010-07", "SET 105", "TR1200+75")?;
    place(solver, "BIOL 1010-08", "SNOW 151", "TR1330+75")?;
    place(solver, "BIOL 1010-09", "SET 420", "MW1630+75")?;
    place(solver, "BIOL 1010-10", "SET 301", "TR1630+75")?;
    place(solver, "BIOL 1010-11-SI", "SNOW 113", "M1030+75")?;
    place(solver, "BIOL 1010-11", "SET 106", "TR1030+75")?;
    place(solver, "BIOL 1015-04", "SET 312", "T1100+170")?;
    place(solver, "BIOL 1015-05", "SET 312", "W1100+170")?;
    place(solver, "BIOL 1015-07", "SET 312", "T1400+170")?;
    place(solver, "BIOL 1015-51", "SET 312", "T1700+170")?;
    place(solver, "BIOL 1200-01", "BROWN 201", "TR1030+75")?;
    place(solver, "BIOL 1200-02", "SET 105", "TR1500+75")?;
    place(solver, "BIOL 1610-01", "SET 106", "MTWRF0800+50")?;
    place(solver, "BIOL 1610-02", "SET 105", "MTWF1100+50")?;
    place(solver, "BIOL 1615-01", "SET 309", "T0800+170")?;
    place(solver, "BIOL 1615-02", "SET 309", "W0800+170")?;
    place(solver, "BIOL 1615-04", "SET 309", "F0800+170")?;
    place(solver, "BIOL 1615-05", "SET 309", "T1100+170")?;
    place(solver, "BIOL 1615-06", "SET 309", "W1100+170")?;
    place(solver, "BIOL 1615-07", "SET 309", "R1100+170")?;
    place(solver, "BIOL 1615-08", "SET 309", "F1100+170")?;
    place(solver, "BIOL 1615-09", "SET 309", "T1400+170")?;
    place(solver, "BIOL 1615-10", "SET 309", "W1400+170")?;
    place(solver, "BIOL 1615-11", "SET 309", "R1400+170")?;
    place(solver, "BIOL 1615-12", "SET 309", "F1400+170")?;
    place(solver, "BIOL 1615-50", "SET 309", "T1700+170")?;
    place(solver, "BIOL 1615-51", "SET 309", "W1700+170")?;
    place(solver, "BIOL 1615-52", "SET 309", "R1700+170")?;
    place(solver, "BIOL 1620-01", "SET 105", "MTWF1000+50")?;
    place(solver, "BIOL 1620-02", "SET 106", "MTRF1200+50")?;
    place(solver, "BIOL 1620-03", "SET 216", "MTWR1100+50")?;
    place(solver, "BIOL 1625-01", "SET 318", "R0800+170")?;
    place(solver, "BIOL 1625-02", "SET 318", "R1100+170")?;
    place(solver, "BIOL 1625-03", "SET 318", "W1200+170")?;
    place(solver, "BIOL 1625-04", "SET 318", "R1400+170")?;
    place(solver, "BIOL 1625-05", "SET 318", "F1100+170")?;
    place(solver, "BIOL 1625-06", "SET 318", "W1500+170")?;
    place(solver, "BIOL 1625-50", "SET 318", "R1700+170")?;
    place(solver, "BIOL 2060-01", "SET 105", "MW1500+75")?;
    place(solver, "BIOL 2065-01", "SET 304", "MW1300+110")?;
    place(solver, "BIOL 2065-02", "SET 304", "MW1700+110")?;
    place(solver, "BIOL 2300-01", "SET 216", "MW1330+50")?;
    place(solver, "BIOL 2320-01", "BROWN 201", "MWF1000+50")?;
    place(solver, "BIOL 2320-02", "SET 301", "MW1200+75")?;
    place(solver, "BIOL 2320-04", "SET 301", "MW1330+75")?;
    place(solver, "BIOL 2320-04-SI", "SET 105", "T1330+75")?;
    place(solver, "BIOL 2320-05", "SET 301", "TR1030+75")?;
    place(solver, "BIOL 2320-07", "SET 301", "TR1330+75")?;
    place(solver, "BIOL 2320-08", "SET 301", "MW1330+75")?;
    place(solver, "BIOL 2320-08-SI", "SET 105", "R1330+75")?;
    place(solver, "BIOL 2325-01", "SET 213", "MW0600+110")?;
    place(solver, "BIOL 2325-02", "SET 215", "TR0600+110")?;
    place(solver, "BIOL 2325-03", "SET 213", "MW0800+110")?;
    place(solver, "BIOL 2325-04", "SET 215", "MW0800+110")?;
    place(solver, "BIOL 2325-05", "SET 213", "TR0800+110")?;
    place(solver, "BIOL 2325-06", "SET 215", "TR0800+110")?;
    place(solver, "BIOL 2325-07", "SET 213", "MW1000+110")?;
    place(solver, "BIOL 2325-08", "SET 215", "MW1000+110")?;
    place(solver, "BIOL 2325-09", "SET 213", "TR1000+110")?;
    place(solver, "BIOL 2325-10", "SET 215", "TR1000+110")?;
    place(solver, "BIOL 2325-11", "SET 213", "MW1200+110")?;
    place(solver, "BIOL 2325-12", "SET 215", "MW1200+110")?;
    place(solver, "BIOL 2325-13", "SET 213", "TR1200+110")?;
    place(solver, "BIOL 2325-14", "SET 215", "TR1200+110")?;
    place(solver, "BIOL 2325-15", "SET 213", "MW1400+110")?;
    place(solver, "BIOL 2325-16", "SET 215", "MW1400+110")?;
    place(solver, "BIOL 2325-17", "SET 213", "TR1400+110")?;
    place(solver, "BIOL 2325-18", "SET 215", "TR1400+110")?;
    place(solver, "BIOL 2325-19", "SET 213", "MW1600+110")?;
    place(solver, "BIOL 2325-20", "SET 215", "MW1600+110")?;
    place(solver, "BIOL 2325-21", "SET 213", "TR1600+110")?;
    place(solver, "BIOL 2325-22", "SET 215", "TR1600+110")?;
    place(solver, "BIOL 2325-50", "SET 213", "MW1800+110")?;
    place(solver, "BIOL 2325-51", "SET 215", "MW1800+110")?;
    place(solver, "BIOL 2325-52", "SET 213", "TR1800+110")?;
    place(solver, "BIOL 2325-53", "SET 215", "TR1800+110")?;
    place(solver, "BIOL 2420-01", "SET 106", "MWF0900+50")?;
    place(solver, "BIOL 2420-02", "SET 106", "MWF1000+50")?;
    place(solver, "BIOL 2420-03", "SET 106", "MWF1100+50")?;
    place(solver, "BIOL 2420-04", "SET 301", "MW1500+75")?;
    place(solver, "BIOL 2420-05", "SET 301", "TR1500+75")?;
    place(solver, "BIOL 2425-01", "SET 214", "T0900+110")?;
    place(solver, "BIOL 2425-02", "SET 214", "W0900+110")?;
    place(solver, "BIOL 2425-03", "SET 214", "R0900+110")?;
    place(solver, "BIOL 2425-04", "SET 214", "F0900+110")?;
    place(solver, "BIOL 2425-05", "SET 214", "T1100+110")?;
    place(solver, "BIOL 2425-06", "SET 214", "W1100+110")?;
    place(solver, "BIOL 2425-07", "SET 214", "R1100+110")?;
    place(solver, "BIOL 2425-08", "SET 214", "F1100+110")?;
    place(solver, "BIOL 2425-09", "SET 214", "T1300+110")?;
    place(solver, "BIOL 2425-10", "SET 214", "W1300+110")?;
    place(solver, "BIOL 2425-11", "SET 214", "R1300+110")?;
    place(solver, "BIOL 2425-12", "SET 214", "F1300+110")?;
    place(solver, "BIOL 2425-13", "SET 214", "T1500+110")?;
    place(solver, "BIOL 2425-14", "SET 214", "W1500+110")?;
    place(solver, "BIOL 2425-15", "SET 214", "R1500+110")?;
    place(solver, "BIOL 2425-50", "SET 214", "T1700+110")?;
    place(solver, "BIOL 2425-51", "SET 214", "W1700+110")?;
    place(solver, "BIOL 2991R-01A", "SET 501", "W1200+50")?;
    place(solver, "BIOL 3000R-09A", "SET 105", "M0800+50")?;
    place(solver, "BIOL 3010-01", "SET 301", "MWF1100+50")?;
    place(solver, "BIOL 3010-01-SI", "SET 301", "T1200+50")?;
    place(solver, "BIOL 3010-02", "SET 301", "MWF1100+50")?;
    place(solver, "BIOL 3010-02-SI", "SET 301", "R1200+50")?;
    place(solver, "BIOL 3030-01", "SET 301", "MWF0900+50")?;
    place(solver, "BIOL 3030-01-SI", "SET 301", "T0900+50")?;
    place(solver, "BIOL 3030-02", "SET 301", "MWF0900+50")?;
    place(solver, "BIOL 3030-02-SI", "SET 301", "R0900+50")?;
    place(solver, "BIOL 3040-01", "SET 301", "MWF1000+50")?;
    place(solver, "BIOL 3045-01", "SET 216", "T1200+170")?;
    place(solver, "BIOL 3100-01", "HCC 476", "MWF1100+50")?;
    place(solver, "BIOL 3110-01", "SET 408", "R0900+75")?;
    place(solver, "BIOL 3150-01", "SET 106", "MW1330+75")?;
    place(solver, "BIOL 3155-01", "SET 216", "R1200+135")?;
    place(solver, "BIOL 3155-02", "SET 216", "T1500+170")?;
    place(solver, "BIOL 3230R-01", "SET 213", "F1330+170")?;
    place(solver, "BIOL 3230R-02", "SET 215", "F1330+170")?;
    place(solver, "BIOL 3250-01", "SET 319", "MW1330+75")?;
    place(solver, "BIOL 3300-01", "SET 501", "TR1500+75")?;
    place(solver, "BIOL 3420-01", "SNOW 128", "TR0900+75")?;
    place(solver, "BIOL 3450-01", "SET 524", "MWF1100+50")?;
    place(solver, "BIOL 3455-01", "SET 304", "T0900+170")?;
    place(solver, "BIOL 3455-02", "SET 304", "T1500+170")?;
    place(solver, "BIOL 3460-01", "SET 201", "MW1500+75")?;
    place(solver, "BIOL 4040-01", "SET 501", "W0900+50")?;
    place(solver, "BIOL 4200-01", "SNOW 208", "TR1500+50")?;
    place(solver, "BIOL 4205-01", "SNOW 208", "TR1600+170")?;
    place(solver, "BIOL 4280-01", "SET 318", "MWF0900+50")?;
    place(solver, "BIOL 4300-01", "SET 216", "MWF0900+50")?;
    place(solver, "BIOL 4305-01", "SET 308", "R0800+170")?;
    place(solver, "BIOL 4310-01", "SET 501", "TR1330+75")?;
    place(solver, "BIOL 4350-01", "SET 319", "TR1200+75")?;
    place(solver, "BIOL 4355-01", "SET 319", "T1400+170")?;
    place(solver, "BIOL 4440-01", "SNOW 208", "TR1030+75")?;
    place(solver, "BIOL 4600-01", "SET 216", "MW1200+75")?;
    place(solver, "BIOL 4605-01", "SET 216", "W1500+170")?;
    place(solver, "BIOL 4810R-01B", "SET 303", "M1400+180")?;
    place(solver, "BIOL 4890R-50", "SET 501", "W1715+110")?;
    place(solver, "BIOL 4890R-51", "SET 501", "R1715+110")?;
    place(solver, "BIOL 4910-01", "SET 501", "M0800+50")?;
    place(solver, "BIOL 4910-02", "SET 501", "R1100+50")?;
    place(solver, "BIOL 4910-03", "SET 501", "T1030+50")?;
    place(solver, "BIOL 4990R-02", "SET 303", "R1600+170")?;
    place(solver, "BIOL 4990R-50", "SET 216", "W1800+50")?;
    place(solver, "BTEC 1010-01", "SET 310", "TR1200+75")?;
    place(solver, "BTEC 2020-01", "SET 304", "TR1300+110")?;
    place(solver, "BTEC 2030-01", "SET 308", "MR1100+110")?;
    place(solver, "BTEC 2050-01", "SET 303", "T1300+110")?;
    place(solver, "BTEC 2050-01-lab", "SET 303", "T1500+50")?;
    place(solver, "BTEC 2050-02", "SET 303", "T1300+110")?;
    place(solver, "BTEC 2050-02-lab", "SET 303", "T1600+50")?;
    place(solver, "BTEC 3010-01", "SET 312", "MW1530+75")?;
    place(solver, "BTEC 4050-01A", "SET 303", "W1330+170")?;
    place(solver, "CHEM 1010-01", "SNOW 113", "TR1030+75")?;
    place(solver, "CHEM 1010-02", "SNOW 113", "TR1330+75")?;
    place(solver, "CHEM 1015-01", "SET 405", "M0900+110")?;
    place(solver, "CHEM 1015-02", "SET 405", "M1100+110")?;
    place(solver, "CHEM 1015-03", "SET 405", "M1300+110")?;
    place(solver, "CHEM 1120-01", "SNOW 216", "MTWR0900+50")?;
    place(solver, "CHEM 1125-01", "SET 404", "M1100+110")?;
    place(solver, "CHEM 1125-02", "SET 404", "M1300+110")?;
    place(solver, "CHEM 1150-01", "SET 201", "MTWR0800+50")?;
    place(solver, "CHEM 1150-02", "SET 201", "MTWR1400+50")?;
    place(solver, "CHEM 1150-03", "SNOW 216", "MTWR1200+50")?;
    place(solver, "CHEM 1155-01", "SET 405", "T1000+170")?;
    place(solver, "CHEM 1155-02", "SET 407", "W1000+170")?;
    place(solver, "CHEM 1155-03", "SET 407", "W1300+170")?;
    place(solver, "CHEM 1155-05", "SET 405", "T1600+170")?;
    place(solver, "CHEM 1155-06", "SET 405", "W0900+170")?;
    place(solver, "CHEM 1155-50", "SET 405", "T1900+170")?;
    place(solver, "CHEM 1210-01", "SET 201", "MTWR0900+50")?;
    place(solver, "CHEM 1210-02", "SET 201", "MTWR1000+50")?;
    place(solver, "CHEM 1210-03", "SNOW 216", "MTWR1300+50")?;
    place(solver, "CHEM 1215-01", "SET 407", "T0700+170")?;
    place(solver, "CHEM 1215-02", "SET 409", "R1000+170")?;
    place(solver, "CHEM 1215-03", "SET 407", "R1000+170")?;
    place(solver, "CHEM 1215-04", "SET 409", "R1300+170")?;
    place(solver, "CHEM 1215-05", "SET 407", "R1600+170")?;
    place(solver, "CHEM 1215-06", "SET 409", "R1600+170")?;
    place(solver, "CHEM 1215-50", "SET 409", "R1900+170")?;
    place(solver, "CHEM 1220-01", "SET 420", "MTWR0800+50")?;
    place(solver, "CHEM 1220-02", "SNOW 216", "MTWR1400+50")?;
    place(solver, "CHEM 1220-03", "SET 420", "MTWR1000+50")?;
    place(solver, "CHEM 1225-01", "SET 409", "T0700+170")?;
    place(solver, "CHEM 1225-02", "SET 409", "T1000+170")?;
    place(solver, "CHEM 1225-03", "SET 409", "T1300+170")?;
    place(solver, "CHEM 1225-04", "SET 407", "T1600+170")?;
    place(solver, "CHEM 1225-05", "SET 409", "T1600+170")?;
    place(solver, "CHEM 1225-50", "SET 407", "T1900+170")?;
    place(solver, "CHEM 2310-01", "SET 420", "MTWRF0900+50")?;
    place(solver, "CHEM 2310-02", "SNOW 216", "MTWRF1100+50")?;
    place(solver, "CHEM 2315-01", "SET 404", "R1000+170")?;
    place(solver, "CHEM 2315-02", "SET 404", "R1300+170")?;
    place(solver, "CHEM 2320-01", "SET 201", "MTWRF1100+50")?;
    place(solver, "CHEM 2320-02", "SET 420", "MTWRF1200+50")?;
    place(solver, "CHEM 2325-01", "SET 404", "T0900+170")?;
    place(solver, "CHEM 2325-02", "SET 404", "T1200+170")?;
    place(solver, "CHEM 2325-03", "SET 404", "T1500+170")?;
    place(solver, "CHEM 2325-04", "SET 404", "W0900+170")?;
    place(solver, "CHEM 2325-05", "SET 404", "W1200+170")?;
    place(solver, "CHEM 2325-06", "SET 404", "W1500+170")?;
    place(solver, "CHEM 2325-50", "SET 404", "T1800+170")?;
    place(solver, "CHEM 3070-01", "SET 420", "MTWR1100+50")?;
    place(solver, "CHEM 3075-01", "SNOW 103", "T1600+170")?;
    place(solver, "CHEM 3300-01", "SNOW 216", "MWF1000+50")?;
    place(solver, "CHEM 3300-01-SI", "SNOW 103", "R1500+170")?;
    place(solver, "CHEM 3510-01", "SET 420", "MW1330+75")?;
    place(solver, "CHEM 3515-01", "SET 308", "R1300+170")?;
    place(solver, "CHEM 3515-02", "SET 308", "R1600+170")?;
    place(solver, "CHEM 3520-01", "SET 201", "MW1200+75")?;
    place(solver, "CHEM 3525-01", "SET 308", "T1000+170")?;
    place(solver, "CHEM 3525-02", "SET 308", "T1300+170")?;
    place(solver, "CHEM 3525-03", "SET 308", "T1600+170")?;
    place(solver, "CHEM 4800R-01", "SNOW 204", "MTWRF1000+50")?;
    place(solver, "CHEM 4800R-02", "SNOW 204", "MTWRF1200+50")?;
    place(solver, "CHEM 4800R-03", "SNOW 204", "MTWRF1100+50")?;
    place(solver, "CHEM 4800R-04", "SNOW 204", "MTWRF1500+50")?;
    place(solver, "CHEM 4800R-06", "SNOW 204", "MTWRF1600+50")?;
    place(solver, "CHEM 4910-01", "SET 201", "F1200+50")?;
    place(solver, "ECE 2100-01", "SET 102", "MW1200+75")?;
    place(solver, "ECE 2280-01", "SET 102", "MWF1100+50")?;
    place(solver, "ECE 2285-01", "SET 101", "T0800+110")?;
    place(solver, "ECE 3500-01", "SET 523", "MW1500+75")?;
    place(solver, "ECE 3600-01", "SET 523", "MW1330+75")?;
    place(solver, "ECE 3605-01", "SET 101", "T1200+110")?;
    place(solver, "ECE 4010-01", "SET 219", "MWF1330+180")?;
    place(solver, "ECE 4510-01", "SET 523", "TR0900+75")?;
    place(solver, "ECE 4730-01", "SET 523", "MW1630+75")?;
    place(solver, "ECE 4735-01", "SET 101", "T1400+110")?;
    place(solver, "ECE 4990-01-lab", "SET 101", "F1000+110")?;
    place(solver, "ECE 4990-01", "SET 523", "MW1200+75")?;
    place(solver, "ECE 4990-02", "SET 523", "TR1030+75")?;
    place(solver, "ECE 4990-03-lab", "SET 101", "F0800+115")?;
    place(solver, "ECE 4990-03", "SET 523", "TR1630+75")?;
    place(solver, "ENVS 1010-01", "SET 524", "TR1200+75")?;
    place(solver, "ENVS 1010-03", "SET 524", "TR1330+75")?;
    place(solver, "ENVS 1010-04", "SET 524", "MW1330+75")?;
    place(solver, "ENVS 1010-05", "SNOW 113", "TR1500+75")?;
    place(solver, "ENVS 1010-06", "SNOW 113", "MW1330+75")?;
    place(solver, "ENVS 1010-07", "SNOW 128", "TR1330+75")?;
    place(solver, "ENVS 1099-01", "SET 526", "F1000+50")?;
    place(solver, "ENVS 1210-01", "SNOW 113", "TR1200+75")?;
    place(solver, "ENVS 1215-01", "SET 526", "M1300+170")?;
    place(solver, "ENVS 1215-02", "SET 526", "R1330+165")?;
    place(solver, "ENVS 2099R-50", "SET 526", "TR1800+75")?;
    place(solver, "ENVS 2210-01", "SNOW 128", "MW1200+75")?;
    place(solver, "ENVS 2700R-01", "SET 527", "F1400+170")?;
    place(solver, "ENVS 3110-01", "SET 408", "MWF1100+50")?;
    place(solver, "ENVS 3210-01", "SET 526", "TR0900+75")?;
    place(solver, "ENVS 3280-50", "SNOW 128", "TR1800+110")?;
    place(solver, "ENVS 3410-01", "SET 522", "MWF1000+50")?;
    place(solver, "ENVS 3920-50", "SNOW 113", "W1800+50")?;
    place(solver, "ENVS 4910-01", "SET 408", "F1200+50")?;
    place(solver, "GEO 1010-01", "SET 524", "TR0900+75")?;
    place(solver, "GEO 1010-50", "SNOW 128", "MW1800+75")?;
    place(solver, "GEO 1015-01", "SET 527", "W0900+110")?;
    place(solver, "GEO 1015-03", "SET 527", "T1100+110")?;
    place(solver, "GEO 1015-04", "SET 527", "T1500+110")?;
    place(solver, "GEO 1015-50", "SET 527", "T1700+110")?;
    place(solver, "GEO 1015-51", "SET 527", "T1900+110")?;
    place(solver, "GEO 1050-01", "SET 527", "W1100+110")?;
    place(solver, "GEO 1110-01", "SET 522", "TR0900+75")?;
    place(solver, "GEO 1115-01", "SET 522", "W1100+170")?;
    place(solver, "GEO 1220-01", "SET 522", "TR1030+75")?;
    place(solver, "GEO 1225-01", "SET 522", "R1630+170")?;
    place(solver, "GEO 2700R-01", "SET 527", "F1400+170")?;
    place(solver, "GEO 3110-01", "SET 408", "MWF1100+50")?;
    place(solver, "GEO 3500-01-lab", "SET 408", "R1200+170")?;
    place(solver, "GEO 3500-01", "SET 408", "TR1500+75")?;
    place(solver, "GEO 3600-01", "SET 522", "MW1500+75")?;
    place(solver, "GEO 3600-01-lab", "SET 522", "T1200+170")?;
    place(solver, "GEO 3710-01", "SET 524", "TR1500+75")?;
    place(solver, "GEO 4000R-01", "SET 527", "F1100+50")?;
    place(solver, "GEO 4910-01", "SNOW 216", "F1200+50")?;
    place(solver, "GEOG 1000-01", "SET 524", "MWF1000+50")?;
    place(solver, "GEOG 1000-01-SI", "SNOW 216", "R1000+50")?;
    place(solver, "GEOG 1000-02", "SET 524", "MW1200+75")?;
    place(solver, "GEOG 1000-03", "SNOW 113", "TR0900+75")?;
    place(solver, "GEOG 1005-01", "SET 526", "T1100+110")?;
    place(solver, "GEOG 1005-02", "SET 526", "T1300+110")?;
    place(solver, "GEOG 1005-03", "SET 526", "W0900+110")?;
    place(solver, "GEOG 1005-04", "SET 526", "W1100+110")?;
    place(solver, "GEOG 1005-05", "SET 526", "R1100+110")?;
    place(solver, "GEOG 3600-01", "SET 408", "TR1030+75")?;
    place(solver, "GEOG 3605-01", "SET 408", "T1200+170")?;
    place(solver, "GEOG 4180-01", "SET 408", "MW1330+75")?;
    place(solver, "MATH 1010-03", "SNOW 3", "MTWR1100+50")?;
    place(solver, "MATH 1010-04", "SNOW 145", "MW1300+100")?;
    place(solver, "MATH 1010-05", "SNOW 145", "TR1500+100")?;
    place(solver, "MATH 1010-06", "SNOW 145", "MW1500+100")?;
    place(solver, "MATH 1010-07", "SNOW 3", "MTWR1200+50")?;
    place(solver, "MATH 1010-50", "SNOW 147", "TR1800+100")?;
    place(solver, "MATH 1030-01", "SNOW 125", "MW1500+75")?;
    place(solver, "MATH 1030-02", "SNOW 124", "TR0730+75")?;
    place(solver, "MATH 1030-03", "SNOW 124", "TR0900+75")?;
    place(solver, "MATH 1030-04", "SNOW 125", "MW1330+75")?;
    place(solver, "MATH 1030-05", "SNOW 150", "TR1200+75")?;
    place(solver, "MATH 1030-06", "SNOW 150", "TR1330+75")?;
    place(solver, "MATH 1040-01", "SNOW 124", "MWF0800+50")?;
    place(solver, "MATH 1040-02", "SNOW 124", "MWF0900+50")?;
    place(solver, "MATH 1040-03", "SNOW 124", "MWF1000+50")?;
    place(solver, "MATH 1040-04", "SNOW 124", "MWF1200+50")?;
    place(solver, "MATH 1040-05", "SNOW 124", "MWF1100+50")?;
    place(solver, "MATH 1040-06", "SNOW 125", "TR1330+75")?;
    place(solver, "MATH 1040-07", "SNOW 151", "TR1200+75")?;
    place(solver, "MATH 1040-08", "SNOW 124", "MW1500+75")?;
    place(solver, "MATH 1040-09", "SNOW 150", "MW1200+75")?;
    place(solver, "MATH 1040-10", "SNOW 124", "TR1200+75")?;
    place(solver, "MATH 1040-11", "SNOW 124", "TR1630+75")?;
    place(solver, "MATH 1040-12", "SNOW 125", "TR1630+75")?;
    place(solver, "MATH 1040-14", "SNOW 124", "MW1630+75")?;
    place(solver, "MATH 1050-01", "SNOW 3", "MTWR0800+50")?;
    place(solver, "MATH 1050-02", "SNOW 3", "MTWR0900+50")?;
    place(solver, "MATH 1050-03", "SNOW 125", "F1100+50")?;
    place(solver, "MATH 1050-03-alt", "SNOW 125", "MTWR1100+50")?;
    place(solver, "MATH 1050-04", "SNOW 147", "MTWR1200+50")?;
    place(solver, "MATH 1050-05", "SNOW 145", "TR1300+100")?;
    place(solver, "MATH 1050-06", "SNOW 112", "MTWR1200+50")?;
    place(solver, "MATH 1060-01", "SNOW 147", "TR0900+75")?;
    place(solver, "MATH 1060-02", "SNOW 147", "TR1030+75")?;
    place(solver, "MATH 1080-01", "SNOW 145", "MTWRF1000+50")?;
    place(solver, "MATH 1100-02", "SNOW 124", "MW1330+75")?;
    place(solver, "MATH 1210-01", "SNOW 145", "MTWR1200+50")?;
    place(solver, "MATH 1210-02", "SNOW 125", "MTWR0800+50")?;
    place(solver, "MATH 1210-03", "SNOW 145", "MTWR1100+50")?;
    place(solver, "MATH 1220-01", "SNOW 147", "MTWR0800+50")?;
    place(solver, "MATH 1220-02", "SNOW 125", "MTWR0900+50")?;
    place(solver, "MATH 2010-01", "SNOW 150", "T1630+150")?;
    place(solver, "MATH 2020-01", "SNOW 150", "TR1030+75")?;
    place(solver, "MATH 2020-02", "SNOW 150", "W1630+150")?;
    place(solver, "MATH 2200-01", "SNOW 112", "TR1030+75")?;
    place(solver, "MATH 2210-01", "SNOW 112", "MTWR0900+50")?;
    place(solver, "MATH 2250-01", "SNOW 125", "MTWF1000+50")?;
    place(solver, "MATH 2270-01", "SNOW 151", "TR0900+75")?;
    place(solver, "MATH 2280-01", "SNOW 151", "MW1200+75")?;
    place(solver, "MATH 3050-01", "SNOW 151", "TR1030+75")?;
    place(solver, "MATH 3200-01", "SNOW 125", "TR1200+75")?;
    place(solver, "MATH 3450-01", "SNOW 124", "TR1030+75")?;
    place(solver, "MATH 3900-01", "SNOW 112", "MWF1000+50")?;
    place(solver, "MATH 4250-01", "SNOW 147", "MW1500+100")?;
    place(solver, "MATH 4400-01", "SNOW 124", "TR1330+75")?;
    place(solver, "MATH 4410-01", "SNOW 124", "T1500+75")?;
    place(solver, "MATH 4800-01", "SNOW 147", "MW1645+75")?;
    place(solver, "MATH 900-01", "SNOW 144", "MTWR1200+50")?;
    place(solver, "MATH 900-02", "SNOW 144", "MTWR0900+50")?;
    place(solver, "MATH 900-03", "SNOW 144", "MW1300+100")?;
    place(solver, "MATH 900-04", "SNOW 144", "MW1600+100")?;
    place(solver, "MATH 900-06", "SNOW 3", "TR1630+100")?;
    place(solver, "MATH 900-07", "SNOW 144", "TR1300+100")?;
    place(solver, "MATH 900-51", "SNOW 144", "MW1800+100")?;
    place(solver, "MATH 980-03", "SNOW 144", "MTWR1000+50")?;
    place(solver, "MATH 980-05", "SNOW 144", "TR1630+100")?;
    place(solver, "MATH 980-06", "SNOW 144", "MTWR0800+50")?;
    place(solver, "MATH 980-07", "SNOW 3", "MW1300+100")?;
    place(solver, "MATH 980-08", "SNOW 3", "TR1300+100")?;
    place(solver, "MATH 980-10", "SNOW 3", "MW1630+100")?;
    place(solver, "MECH 1100-01", "SET 226", "MW1200+75")?;
    place(solver, "MECH 1150-01", "SET 225", "TR1500+170")?;
    place(solver, "MECH 1150-02", "SET 225", "MW1500+170")?;
    place(solver, "MECH 1200-01", "SET 226", "MWF0900+50")?;
    place(solver, "MECH 1200-02", "SET 226", "MWF1000+50")?;
    place(solver, "MECH 1205-01", "SET 226", "R0800+110")?;
    place(solver, "MECH 1205-02", "SET 226", "R1000+110")?;
    place(solver, "MECH 1205-03", "SET 226", "R1200+110")?;
    place(solver, "MECH 1205-04", "SET 226", "R1400+110")?;
    place(solver, "MECH 1205-05", "SET 226", "R1600+110")?;
    place(solver, "MECH 2030-01", "SET 104", "MWF1100+50")?;
    place(solver, "MECH 2160-01", "SET 226", "MW1500+75")?;
    place(solver, "MECH 2250-01", "SET 104", "MW1200+75")?;
    place(solver, "MECH 2250-02", "SET 104", "MW1330+75")?;
    place(solver, "MECH 2255-01", "SET 101", "R0800+110")?;
    place(solver, "MECH 2255-02", "SET 101", "R1200+110")?;
    place(solver, "MECH 2255-03", "SET 101", "R1400+110")?;
    place(solver, "MECH 2255-04", "SET 101", "R1600+110")?;
    place(solver, "MECH 3250-01", "SET 104", "MW1630+75")?;
    place(solver, "MECH 3255-01", "SET 104", "T1200+110")?;
    place(solver, "MECH 3255-02", "SET 226", "T1200+110")?;
    place(solver, "MECH 3600-01", "SET 104", "MTWF0900+50")?;
    place(solver, "MECH 3602-01", "SET 104", "MTWF0900+50")?;
    place(solver, "MECH 3605-01", "SET 104", "R1400+110")?;
    place(solver, "MECH 3605-02", "SET 104", "R1600+110")?;
    place(solver, "MECH 3650-01", "SET 104", "MW1500+75")?;
    place(solver, "MECH 3655-01", "SET 104", "R0800+110")?;
    place(solver, "MECH 3655-02", "SET 104", "R1000+110")?;
    place(solver, "MECH 4010-01", "SET 219", "MWF1330+180")?;
    place(solver, "MECH 4500-01", "SET 523", "TR1500+75")?;
    place(solver, "MECH 4860R-01", "SET 102", "M0800+50")?;
    place(solver, "MECH 4990-01", "SET 523", "MW1000+110")?;
    place(solver, "MTRN 2350-01", "SET 102", "TR1000+50")?;
    place(solver, "MTRN 2355-01", "SET 102", "TR1400+110")?;
    place(solver, "PHYS 1010-01", "SET 418", "MW1630+75")?;
    place(solver, "PHYS 1015-01", "SET 410", "M1300+110")?;
    place(solver, "PHYS 1015-02", "SET 410", "M1000+110")?;
    place(solver, "PHYS 1040-50", "SET 418", "MW1800+75")?;
    place(solver, "PHYS 1045-50", "SET 418", "M1930+170")?;
    place(solver, "PHYS 1045-51", "SET 418", "T1930+170")?;
    place(solver, "PHYS 1045-52", "SET 418", "W1930+170")?;
    place(solver, "PHYS 2010-01", "SET 418", "MWRF0800+50")?;
    place(solver, "PHYS 2010-02", "SET 418", "MWRF1500+50")?;
    place(solver, "PHYS 2015-01", "SET 410", "T1200+110")?;
    place(solver, "PHYS 2015-02", "SET 410", "T1400+110")?;
    place(solver, "PHYS 2015-03", "SET 410", "T1000+110")?;
    place(solver, "PHYS 2020-01", "SET 418", "MWRF1000+50")?;
    place(solver, "PHYS 2020-02", "SET 418", "MWRF1100+50")?;
    place(solver, "PHYS 2025-01", "SET 412", "T1400+50")?;
    place(solver, "PHYS 2025-03", "SET 412", "T1600+110")?;
    place(solver, "PHYS 2025-04", "SET 412", "T1800+110")?;
    place(solver, "PHYS 2210-01", "SET 418", "MTWF1300+50")?;
    place(solver, "PHYS 2210-02", "SET 418", "MTWF0900+50")?;
    place(solver, "PHYS 2215-01", "SET 410", "R1400+110")?;
    place(solver, "PHYS 2215-02", "SET 410", "R1600+110")?;
    place(solver, "PHYS 2215-50", "SET 410", "R1800+110")?;
    place(solver, "PHYS 2220-01", "SET 418", "MTWF1400+50")?;
    place(solver, "PHYS 2225-01", "SET 412", "R1400+110")?;
    place(solver, "PHYS 2225-02", "SET 412", "R1600+110")?;
    place(solver, "PHYS 3600-01", "SET 104", "MTWF0900+50")?;
    place(solver, "PHYS 3605-01", "SET 104", "R1400+110")?;
    place(solver, "PHYS 3605-02", "SET 104", "R1600+110")?;
    place(solver, "SCI 4700-01", "SET 216", "R1530+150")?;
    place(solver, "SCI 4720-01", "SET 501", "F1400+170")?;

    place(solver, "CS 1030-01", "Smith 109", "MWF1000+50")?;
    place(solver, "CS 1400-01", "Smith 116", "MW1630+75")?;
    place(solver, "CS 1400-02", "Smith 108", "MWF1000+50")?;
    place(solver, "CS 1400-03", "Smith 108", "TR1330+75")?;
    place(solver, "CS 1400-50", "Smith 116", "T1800+150")?;
    place(solver, "CS 1410-01", "Smith 109", "MWF1100+50")?;
    place(solver, "CS 1410-02", "Smith 108", "MWF0900+50")?;
    place(solver, "CS 2420-01", "Smith 117", "MWF1100+50")?;
    place(solver, "CS 2450-01", "Smith 108", "TR1500+75")?;
    place(solver, "CS 2450-02", "Smith 108", "TR1630+75")?;
    place(solver, "CS 2810-01", "Smith 109", "MW1330+75")?;
    place(solver, "CS 2810-02", "Smith 109", "TR1200+75")?;
    place(solver, "CS 3005-01", "Smith 116", "MWF1000+50")?;
    place(solver, "CS 3150-01", "Smith 108", "MWF1100+50")?;
    place(solver, "CS 3310-01", "Smith 117", "MWF1000+50")?;
    place(solver, "CS 3410-01", "Smith 109", "MW1200+75")?;
    place(solver, "CS 3510-01", "Smith 116", "MWF0900+50")?;
    place(solver, "CS 3600-01", "Smith 113", "MW1330+75")?;
    place(solver, "CS 4307-01", "Smith 109", "TR1330+75")?;
    place(solver, "CS 4320-01", "Smith 116", "TR0900+75")?;
    place(solver, "CS 4550-01", "Smith 113", "MW1500+75")?;
    place(solver, "CS 4600-01", "Smith 116", "TR1030+75")?;
    place(solver, "CS 4600-02", "Smith 109", "TR1030+75")?;
    place(solver, "CS 4991R-50", "Smith 116", "R1900+50")?;
    place(solver, "CS 4992R-01", "Smith 109", "F1300+50")?;
    place(solver, "SE 1400-01", "Smith 112", "MWF1100+50")?;
    place(solver, "SE 1400-02", "Smith 112", "TR1030+75")?;
    place(solver, "SE 3010-01", "Smith 112", "MW1500+75")?;
    place(solver, "SE 3100-01", "Smith 108", "MWF0800+50")?;
    place(solver, "SE 3200-01", "Smith 109", "TR0900+75")?;
    place(solver, "SE 3450-01", "Smith 112", "TR1200+75")?;
    place(solver, "SE 3500-01", "Smith 108", "TR1200+75")?;
    place(solver, "SE 3550-01", "Smith 112", "MW1200+75")?;
    place(solver, "SE 4200-01", "Smith 112", "MW1330+75")?;
    place(solver, "SE 4600-01", "Smith 109", "TR1030+75")?;

    place(solver, "IT 1100-01", "Smith 113", "MWF1000+50")?;
    place(solver, "IT 1100-02", "Smith 113", "TR1200+75")?;
    place(solver, "IT 1200-01", "Smith 107", "TR0900+75")?;
    place(solver, "IT 2300-01", "Smith 113", "MW1200+75")?;
    place(solver, "IT 2300-02", "Smith 113", "TR1330+75")?;
    place(solver, "IT 2400-01", "Smith 107", "TR1030+75")?;
    place(solver, "IT 2700-01", "Smith 107", "TR1200+75")?;
    place(solver, "IT 3100-01", "Smith 107", "MW1200+75")?;
    place(solver, "IT 3110-01", "Smith 107", "MWF0900+50")?;
    place(solver, "IT 3150-01", "Smith 107", "MW1330+75")?;
    place(solver, "IT 3400-01", "Smith 107", "TR1330+75")?;
    place(solver, "IT 4510-01", "Smith 107", "R1800+150")?;
    place(solver, "IT 4600-01", "Smith 107", "MWF0800+50")?;
    place(solver, "IT 4990-01", "Smith 108", "W1800+150")?;

    Ok(())
}
