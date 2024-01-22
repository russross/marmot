use super::input::*;

pub fn input() -> Result<Input, String> {
    let mut t = Input::new("Spring 2024", date(2024, 1, 8)?, date(2024, 4, 25)?);

    holiday!(t, 2024, 1, 15);
    holiday!(t, 2024, 2, 19);
    holiday!(t, 2024, 3, 11);
    holiday!(t, 2024, 3, 12);
    holiday!(t, 2024, 3, 13);
    holiday!(t, 2024, 3, 14);
    holiday!(t, 2024, 3, 15);

    room!(t, name: "Smith 107", capacity: 32, tags: "flex");
    room!(t, name: "Smith 108", capacity: 32, tags: "flex");
    room!(t, name: "Smith 109", capacity: 32, tags: "flex");
    room!(t, name: "Smith 112", capacity: 24, tags: "macs");
    room!(t, name: "Smith 113", capacity: 24, tags: "pcs");
    room!(t, name: "Smith 116", capacity: 38, tags: "stadium");
    room!(t, name: "Smith 117", capacity: 38, tags: "stadium");

    //    time!(t, name: "MWF0800+50", tags: "3 credit bell schedule", "3×50", "mwf");
    //    time!(t, name: "MWF0900+50", tags: "3 credit bell schedule", "3×50", "mwf");
    //    time!(t, name: "MWF1000+50", tags: "3 credit bell schedule", "3×50", "mwf");
    //    time!(t, name: "MWF1100+50", tags: "3 credit bell schedule", "3×50", "mwf");
    //    time!(t, name: "MW1200+75", tags: "3 credit bell schedule", "2×75", "mw");
    //    time!(t, name: "MW1330+75", tags: "3 credit bell schedule", "2×75", "mw");
    //    time!(t, name: "MW1500+75", tags: "3 credit bell schedule", "2×75", "mw");
    //    time!(t, name: "MW1630+75", tags: "3 credit bell schedule", "2×75", "mw");
    //    time!(t, name: "TR0730+75", tags: "3 credit bell schedule", "2×75", "tr");
    //    time!(t, name: "TR0900+75", tags: "3 credit bell schedule", "2×75", "tr");
    //    time!(t, name: "TR1030+75", tags: "3 credit bell schedule", "2×75", "tr");
    //    time!(t, name: "TR1200+75", tags: "3 credit bell schedule", "2×75", "tr");
    //    time!(t, name: "TR1330+75", tags: "3 credit bell schedule", "2×75", "tr");
    //    time!(t, name: "TR1500+75", tags: "3 credit bell schedule", "2×75", "tr");
    //    time!(t, name: "TR1630+75", tags: "3 credit bell schedule", "2×75", "tr");
    //    time!(t, name: "T1800+150", tags: "3 credit evening");
    //    time!(t, name: "W1800+150", tags: "3 credit evening");
    //    time!(t, name: "R1800+150", tags: "3 credit evening");
    time!(t, name: "R1900+50");
    time!(t, name: "F1300+50");

    input_times(&mut t)?;
    input_computing(&mut t)?;
    input_set(&mut t)?;
    input_multiples(&mut t)?;
    input_prereqs(&mut t)?;

    crosslist!(t, "GEO 2700R-01" cross-list with "ENVS 2700R-01");
    crosslist!(t, "ENVS 3110-01" cross-list with "GEO 3110-01");
    crosslist!(t, "ECE 4010-01" cross-list with "MECH 4010-01");
    crosslist!(t, "MECH 3600-01" cross-list with "MECH 3602-01" cross-list with "PHYS 3600-01");
    crosslist!(t, "MECH 3605-01" cross-list with "PHYS 3605-01");
    crosslist!(t, "MECH 3605-02" cross-list with "PHYS 3605-02");

    crosslist!(t, "BIOL 1010-05" cross-list with "BIOL 1010-11");
    crosslist!(t, "BIOL 2320-04" cross-list with "BIOL 2320-08");
    crosslist!(t, "BTEC 2050-01" cross-list with "BTEC 2050-02");

    Ok(t)
}

pub fn input_computing(t: &mut Input) -> Result<(), String> {
    instructor!(t,
        name:
            "Bart Stander",
        available:
            "MWF 0900-1200",
            "MW  1200-1330" with penalty 10,
            "MW  1330-1630",
            "TR  1030-1200",
            "TR  1330-1500",
            "TR  1500-1630" with penalty 10,
    );
    default_clustering!(t, instructor: "Bart Stander", days: "mt", days off: 1);
    section!(t, course: "CS 2420-01",
            instructor: "Bart Stander",
            rooms and times: "stadium", "flex" with penalty 10, "3×50");
    section!(t, course: "CS 3310-01",
            instructor: "Bart Stander",
            rooms and times: "stadium", "pcs", "3 credit bell schedule");
    section!(t, course: "CS 3600-01",
            instructor: "Bart Stander",
            rooms and times: "pcs", "stadium" with penalty 10, "3 credit bell schedule");
    section!(t, course: "CS 4550-01",
            instructor: "Bart Stander",
            rooms and times: "pcs", "3 credit bell schedule");

    instructor!(t,
        name:
            "Carol Stander",
        available:
            "MWF 1000-1200",
            "MW  1200-1330" with penalty 10,
            "MW  1330-1500",
            "TR  1330-1500" with penalty 5,
    );
    default_clustering!(t, instructor: "Carol Stander", days: "mt");
    section!(t, course: "CS 1030-01",
            instructor: "Carol Stander",
            rooms and times: "flex", "3 credit bell schedule");
    section!(t, course: "CS 1410-02",
            instructor: "Carol Stander",
            rooms and times: "flex", "3 credit bell schedule");

    instructor!(t,
        name:
            "Curtis Larsen",
        available:
            "MWF 0900-1100",
            "MWF 1100-1200" with penalty 10,
            "MW  1200-1330" with penalty 10,
            "MW  1330-1630",
            "TR  0900-1030",
            "TR  1030-1330" with penalty 10,
            "TR  1330-1630",
    );
    default_clustering!(t, instructor: "Curtis Larsen", days: "mt", days off: 0);
    section!(t, course: "CS 3005-01",
            instructor: "Curtis Larsen",
            rooms and times: "Smith 116", "3×50");
    section!(t, course: "CS 3510-01",
            instructor: "Curtis Larsen",
            rooms and times: "Smith 116", "flex" with penalty 1, "3 credit bell schedule", "tr" with penalty 10);
    section!(t, course: "CS 4320-01",
            instructor: "Curtis Larsen",
            rooms and times: "Smith 116", "flex" with penalty 1, "3×50" with penalty 10, "2×75");
    section!(t, course: "CS 4600-01",
            instructor: "Curtis Larsen",
            rooms and times: "Smith 116", "flex" with penalty 1, "3 credit bell schedule", "tr" with penalty 10);

    instructor!(t,
        name:
            "DJ Holt",
        available:
            "MW 1200-1500",
            "MW 1500-1630" with penalty 10,
            "TR 0900-1500",
            "TR 1500-1630" with penalty 10,
    );
    default_clustering!(t, instructor: "DJ Holt", days: "mt", days off: 0);
    section!(t, course: "SE 3010-01",
            instructor: "DJ Holt",
            rooms and times: "flex", "macs", "MW1500+75"); // same day as SE4200
    section!(t, course: "SE 4200-01",
            instructor: "DJ Holt",
            rooms and times: "flex", "macs", "MW1330+75");
    section!(t, course: "SE 4600-01",
            instructor: "DJ Holt",
            rooms and times: "flex", "3 credit bell schedule");
    section!(t, course: "CS 4600-02",
            instructor: "DJ Holt",
            rooms and times: "flex", "3 credit bell schedule");
    crosslist!(t, "SE 4600-01" cross-list with "CS 4600-02");
    anticonflict!(t, set penalty to 50, single: "CS 4600-01", group: "CS 4600-02");

    instructor!(t,
        name:
            "Eric Pedersen",
        available:
            "TR  1200-1330",
    );
    section!(t, course: "SE 3500-01",
            instructor: "Eric Pedersen",
            rooms and times: "flex", "TR1200+75");

    instructor!(t,
        name:
            "Jay Sneddon",
        available:
            "MWF 0800-0900" with penalty 15,
            "MWF 0900-1200" with penalty 10,
            "MW  1200-1630",
            "TR  0900-1500",
            "TR  1500-1630" with penalty 5,
    );
    default_clustering!(t, instructor: "Jay Sneddon", days: "mt", days off: 0);
    section!(t, course: "IT 1200-01",
            instructor: "Jay Sneddon",
            rooms and times: "Smith 107", "tr");
    section!(t, course: "IT 2300-01",
            instructor: "Jay Sneddon",
            rooms and times: "Smith 107", "3 credit bell schedule");
    section!(t, course: "IT 2700-01",
            instructor: "Jay Sneddon",
            rooms and times: "Smith 107", "tr");
    section!(t, course: "IT 3150-01",
            instructor: "Jay Sneddon",
            rooms and times: "Smith 107", "mw", "mwf" with penalty 5);
    section!(t, course: "IT 3400-01",
            instructor: "Jay Sneddon",
            rooms and times: "Smith 107", "3 credit bell schedule");

    instructor!(t,
        name:
            "Jeff Compas",
        available:
            "MWF 0800-0900",
            "MW  1630-1800",
            "TR  1630-1800",
            "T   1800-2030",
    );
    section!(t, course: "CS 1400-03",
            instructor: "Jeff Compas",
            rooms and times: "stadium", "3 credit bell schedule", "3 credit evening");
    section!(t, course: "CS 1400-04",
            instructor: "Jeff Compas",
            rooms and times: "stadium", "3 credit bell schedule", "3 credit evening");
    section!(t, course: "CS 2450-02",
            instructor: "Jeff Compas",
            rooms and times: "flex", "3 credit bell schedule", "3 credit evening");
    section!(t, course: "SE 3100-01",
            instructor: "Jeff Compas",
            rooms and times: "flex", "3 credit bell schedule", "3 credit evening");

    instructor!(t,
        name:
            "Joe Francom",
        available:
            "MWF 0900-1200",
            "MW  1330-1500",
    );
    default_clustering!(t, instructor: "Joe Francom", days: "mt", days off: 1);
    section!(t, course: "IT 3110-01",
            instructor: "Joe Francom",
            rooms and times: "flex", "3 credit bell schedule");
    // See Phil Daley for IT 4510-01

    instructor!(t,
        name:
            "Lora Klein",
        available:
            "TR 0900-1500",
            "MW 1500-1630" with penalty 15,
    );
    default_clustering!(t, instructor: "Lora Klein", days: "mt");
    section!(t, course: "SE 3200-01",
            instructor: "Lora Klein",
            rooms and times: "Smith 107" with penalty 5, "flex", "3 credit bell schedule");
    //course: CS1410 ACE MW 9:30-10:45am, INV 112
    //course: CS1410 ACE MW 12:00-1:15pm, INV 112

    instructor!(t,
        name:
            "Matt Kearl",
        available:
            "MW 1200-1330",
            "TR 0900-1330",
    );
    default_clustering!(t, instructor: "Matt Kearl", days: "mt", days off: 1);
    section!(t, course: "SE 3450-01",
            instructor: "Matt Kearl",
            rooms and times: "flex", "macs", "3 credit bell schedule");
    section!(t, course: "SE 3550-01",
            instructor: "Matt Kearl",
            rooms and times: "flex", "macs", "3 credit bell schedule");
    section!(t, course: "SE 1400-01",
            instructor: "Matt Kearl",
            rooms and times: "macs", "3 credit bell schedule");

    instructor!(t,
        name:
            "Phil Daley",
        available:
            "MWF 0900-1200",
            "MW  1200-1500",
            "MW  1500-1630" with penalty 10,
            "TR  0900-1500",
            "TR  1500-1630" with penalty 10,
    );
    default_clustering!(t, instructor: "Phil Daley", days: "mt", days off: 0);
    section!(t, course: "IT 1100-01",
            instructor: "Phil Daley",
            rooms and times: "pcs", "3 credit bell schedule");
    section!(t, course: "IT 1100-02",
            instructor: "Phil Daley",
            rooms and times: "pcs", "3 credit bell schedule");
    section!(t, course: "IT 2400-01",
            instructor: "Phil Daley",
            rooms and times: "Smith 107", "3 credit bell schedule");
    section!(t, course: "IT 3100-01",
            instructor: "Phil Daley",
            rooms and times: "Smith 107", "3 credit bell schedule");
    // avoid IT 4510 so Phil can shadow Joe
    section!(t, course: "IT 4510-01",
            instructor: "Joe Francom" and "Phil Daley",
            rooms and times: "flex", "3 credit bell schedule");

    instructor!(t,
       name:
           "Ren Quinn",
       available:
           "MWF 0900-1200",
           "TR  1200-1330" with penalty 5,
           "TR  1330-1630",
           "R   1900-2000",
           "F   1300-1400",
    );
    default_clustering!(t, instructor: "Ren Quinn", days: "mt", days off: 0);
    section!(t, course: "CS 1400-01",
            instructor: "Ren Quinn",
            rooms and times: "flex", "3 credit bell schedule");
    section!(t, course: "CS 1400-02",
            instructor: "Ren Quinn",
            rooms and times: "flex", "3 credit bell schedule");
    section!(t, course: "CS 1410-01",
            instructor: "Ren Quinn",
            rooms and times: "flex", "3 credit bell schedule");
    section!(t, course: "CS 2450-01",
            instructor: "Ren Quinn",
            rooms and times: "flex", "3 credit bell schedule");
    section!(t, course: "CS 3150-01",
            instructor: "Ren Quinn",
            rooms and times: "flex", "3 credit bell schedule");
    section!(t, course: "CS 4991-01",
            instructor: "Ren Quinn",
            rooms and times: "Smith 116", "R1900+50");
    section!(t, course: "CS 4992R-01",
            instructor: "Ren Quinn",
            rooms and times: "Smith 109", "F1300+50");

    instructor!(t,
        name:
            "Russ Ross",
        available:
            "MTWR 1200-1500",
            //"MTWR 1500-1630" with penalty 10,
    );
    default_clustering!(t, instructor: "Russ Ross", days: "mt", days off: 0);
    section!(t, course: "CS 2810-01",
            instructor: "Russ Ross",
            rooms and times: "Smith 109", "3 credit bell schedule");
    section!(t, course: "CS 2810-02",
            instructor: "Russ Ross",
            rooms and times: "Smith 109", "3 credit bell schedule");
    section!(t, course: "CS 3410-01",
            instructor: "Russ Ross",
            rooms and times: "Smith 109", "3 credit bell schedule");
    section!(t, course: "CS 4307-01",
            instructor: "Russ Ross",
            rooms and times: "Smith 109", "3 credit bell schedule");

    instructor!(t,
        name:
            "Rex Frisbey",
        available:
            "MWF 1100-1200",
    );
    section!(t, course: "SE 1400-02",
            instructor: "Rex Frisbey",
            rooms and times: "macs", "3 credit bell schedule");

    instructor!(t,
        name:
            "Jamie Bennion",
        available:
            "W 1800-2030",
    );
    section!(t, course: "IT 4990-01",
            instructor: "Jamie Bennion",
            rooms and times: "flex", "3 credit evening");

    conflict!(t, set hard,
            clique: "CS 2420", "CS 2450", "CS 2810", "CS 3005"); // 3rd/4th semester classes
    conflict!(t, set hard,
            clique: "CS 2420", "CS 2450", "CS 2810"); // grad plan: 2nd year fall
    conflict!(t, set hard,
            clique: "CS 3005", "CS 3520", "SE 3200"); // grad plan: 2nd year spring
    conflict!(t, set hard,
            clique: "CS 3310", "CS 3400", "SE 3530"); // grad plan: 3nd year fall
    conflict!(t, set hard,
            clique: "CS 3510", "CS 4307", "SE 4550"); // grad plan: 3nd year spring
    conflict!(t, set hard,
            clique: "CS 4300"); // grad plan: 4th year fall
    conflict!(t, set hard,
            clique: "CS 3600", "CS 4600"); // grad plan: 4th year spring

    // CS core
    conflict!(t, set penalty to 99,
            clique: "CS 1030", "CS 1400", "CS 1410", "CS 2420", "CS 2450", "CS 2810", "CS 3005",
                    "CS 3150", "CS 3310", "CS 3400", "CS 3410", "CS 3510", "CS 3520", "CS 3530", "CS 3600",
                    "CS 4300", "CS 4307", "CS 4320", "CS 4550", "CS 4600",
                    "SE 3200");

    // CS electives
    conflict!(t, set penalty to 30,
            clique: "CS 1030", "CS 1400", "CS 1410", "CS 2420", "CS 2450", "CS 2810", "CS 3005",
                    "CS 3150", "CS 3310", "CS 3400", "CS 3410", "CS 3510", "CS 3520", "CS 3530", "CS 3600",
                    "CS 4300", "CS 4307", "CS 4320", "CS 4550", "CS 4600",
                    "SE 3200",
                    "SE 3010", "SE 3020", "SE 3100", "SE 3200", "SE 3400", "SE 4200",
                    "IT 2700", "IT 3100", "IT 3110", "IT 4200");

    // CS math and science
    conflict!(t, set penalty to 50,
            clique: "CS 1030", "CS 1400", "CS 1410", "CS 2420", "CS 2450", "CS 2810", "CS 3005",
                    "CS 3150", "CS 3310", "CS 3400", "CS 3410", "CS 3510", "CS 3520", "CS 3530", "CS 3600",
                    "CS 4300", "CS 4307", "CS 4320", "CS 4550", "CS 4600",
                    "SE 3200",
                    "MATH 1210", "MATH 1220", "BIOL 1610", "BIOL 1615", "PHYS 2210", "PHYS 2215");

    // DS: TODO
    conflict!(t, set penalty to 45,
            clique: "CS 2500", "CS 2810", "CS 3005");

    // SE upper division core
    conflict!(t, set penalty to 99,
            clique: /*"CS 2450",*/ "CS 3150", "CS 3310", "CS 3510", "CS 4307",
                    "IT 2300",
                    "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400",
                    "SE 4200", "SE 4600"); // IT 1100, SE 1400

    // Entrepreneurial and marketing track
    conflict!(t, set penalty to 45,
            clique: /*"CS 2450",*/ "CS 3150", "CS 3310", "CS 3510", "CS 4307",
                    "IT 2300",
                    "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400", "SE 3500", "SE 3550",
                    "SE 4200", "SE 4600"); // IT 1100 SE 1400

    // DevOps track
    conflict!(t, set penalty to 45,
            clique: /*"CS 2450",*/ "CS 3150", "CS 3310", "CS 3510", "CS 4307",
                    "IT 2300", "IT 3110", "IT 3300", "IT 4200",
                    "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400",
                    "SE 4200", "SE 4600"); // IT 1100 SE 1400

    // Application track
    conflict!(t, set penalty to 45,
            clique: /*"CS 2450",*/ "CS 3150", "CS 3310", "CS 3500", "CS 3510", "CS 4307",
                    "IT 2300",
                    "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400", "SE 3450",
                    "SE 4200", "SE 4600"); // IT 1100 SE 1400

    // Data science track
    conflict!(t, set penalty to 45,
            clique: /*"CS 2450",*/ "CS 3150", "CS 3310", "CS 3510", "CS 4300", "CS 4307", "CS 4320",
                    "IT 2300",
                    "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400",
                    "SE 4200", "SE 4600"); // IT 1100 SE 1400

    // IT conflicts
    //conflict!(t, set penalty to 50, clique: "IT 1100", "IT 1200"); // when there is only one in-person section of each
    conflict!(t, set penalty to 99,
            clique: "IT 2300", "IT 2400", "IT 2500", "IT 2700",
                    "IT 3100", "IT 3110", "IT 3150", "IT 3300", "IT 3400",
                    "IT 4100", "IT 4200", "IT 4310", "IT 4400", "IT 4510", "IT 4600");

    // IT choose 2 section
    conflict!(t, set penalty to 60,
            clique: "CS 3005",
                    "IT 2300", "IT 2400", "IT 2500", "IT 2700",
                    "IT 3100", "IT 3110", "IT 3150", "IT 3300", "IT 3400",
                    "IT 4100", "IT 4200", "IT 4310", "IT 4400", "IT 4510", "IT 4600",
                    "SE 3200", "SE 3400");

    conflict!(t, remove penalty, clique: "CS 4307", "IT 2300"); // students take either CS4307 or IT2300 but not both so no conflict

    // courses that must be scheduled at the same time
    // TODO:
    // should anticonflict automatically zero out any penalty? maybe as a later pass?
    anticonflict!(t, set penalty to 50, single: "CS 1030-01", group: "CS 1400");
    //anticonflict!(t, set penalty to 50, clique: "SE 1400", "IT 1100"); // temporarily removed because of new hire planning

    Ok(())
}

pub fn input_times(t: &mut Input) -> Result<(), String> {
    time!(t, name: "F0800+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "F0900+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "F1000+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "F1100+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "F1200+50", tags: "1 credit extended bell schedule");
    time!(t, name: "M0800+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "M0900+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "M1000+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "M1100+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "M1200+50", tags: "1 credit extended bell schedule");
    time!(t, name: "R0800+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "R0900+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "R1000+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "R1030+50", tags: "1 credit extended bell schedule");
    time!(t, name: "R1100+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "R1200+50", tags: "1 credit extended bell schedule", "1 credit extended bell schedule");
    time!(t, name: "R1300+50", tags: "1 credit extended bell schedule");
    time!(t, name: "R1400+50", tags: "1 credit extended bell schedule");
    time!(t, name: "R1500+50", tags: "1 credit extended bell schedule");
    time!(t, name: "R1600+50", tags: "1 credit extended bell schedule");
    time!(t, name: "R1800+50", tags: "1 credit evening");
    time!(t, name: "T0800+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "T0900+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "T1000+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "T1030+50", tags: "1 credit extended bell schedule");
    time!(t, name: "T1100+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "T1200+50", tags: "1 credit extended bell schedule", "1 credit extended bell schedule");
    time!(t, name: "T1300+50", tags: "1 credit extended bell schedule");
    time!(t, name: "T1400+50", tags: "1 credit extended bell schedule");
    time!(t, name: "T1500+50", tags: "1 credit extended bell schedule");
    time!(t, name: "T1600+50", tags: "1 credit extended bell schedule");
    time!(t, name: "T1800+50", tags: "1 credit evening");
    time!(t, name: "W0800+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "W0900+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "W1000+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "W1100+50", tags: "1 credit bell schedule", "1 credit extended bell schedule");
    time!(t, name: "W1200+50", tags: "1 credit extended bell schedule");
    time!(t, name: "W1800+50", tags: "1 credit evening");
    time!(t, name: "MF0800+50", tags: "2 credit lecture");
    time!(t, name: "MF0900+50", tags: "2 credit lecture");
    time!(t, name: "MF1000+50", tags: "2 credit lecture");
    time!(t, name: "MF1100+50", tags: "2 credit lecture");
    time!(t, name: "MW0730+50", tags: "2 credit lecture");
    time!(t, name: "MW0800+50", tags: "2 credit lecture");
    time!(t, name: "MW0900+50", tags: "2 credit lecture");
    time!(t, name: "MW1000+50", tags: "2 credit lecture");
    time!(t, name: "MW1100+50", tags: "2 credit lecture");
    time!(t, name: "MW1200+50", tags: "2 credit lecture");
    time!(t, name: "MW1330+50", tags: "2 credit lecture");
    time!(t, name: "MW1500+50", tags: "2 credit lecture");
    time!(t, name: "MW1630+50", tags: "2 credit lecture");
    time!(t, name: "TR0730+50", tags: "2 credit lecture");
    time!(t, name: "TR0900+50", tags: "2 credit lecture");
    time!(t, name: "TR1000+50");
    time!(t, name: "TR1030+50", tags: "2 credit lecture");
    time!(t, name: "TR1200+50", tags: "2 credit lecture");
    time!(t, name: "TR1330+50", tags: "2 credit lecture");
    time!(t, name: "TR1500+50", tags: "2 credit lecture");
    time!(t, name: "TR1630+50", tags: "2 credit lecture");
    time!(t, name: "WF0800+50", tags: "2 credit lecture");
    time!(t, name: "WF0900+50", tags: "2 credit lecture");
    time!(t, name: "WF1000+50", tags: "2 credit lecture");
    time!(t, name: "WF1100+50", tags: "2 credit lecture");
    time!(t, name: "MWF0800+50", tags: "3 credit bell schedule", "3×50", "mwf");
    time!(t, name: "MWF0900+50", tags: "3 credit bell schedule", "3×50", "mwf");
    time!(t, name: "MWF1000+50", tags: "3 credit bell schedule", "3×50", "mwf");
    time!(t, name: "MWF1100+50", tags: "3 credit bell schedule", "3×50", "mwf");
    time!(t, name: "MWF1200+50");
    time!(t, name: "MTRF0800+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTRF0900+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTRF1000+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTRF1100+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTRF1200+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTRF1300+50", tags: "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTRF1400+50", tags: "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTRF1500+50", tags: "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTWF0800+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTWF0900+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTWF1000+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTWF1100+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTWF1200+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTWF1300+50", tags: "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTWF1400+50", tags: "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTWF1500+50", tags: "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTWR0800+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTWR0900+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTWR1000+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTWR1100+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTWR1200+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTWR1300+50", tags: "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTWR1400+50", tags: "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTWR1500+50", tags: "4 credit 4×50 extended bell schedule");
    time!(t, name: "MWRF0800+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MWRF0900+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MWRF1000+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MWRF1100+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MWRF1200+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "MWRF1300+50", tags: "4 credit 4×50 extended bell schedule");
    time!(t, name: "MWRF1400+50", tags: "4 credit 4×50 extended bell schedule");
    time!(t, name: "MWRF1500+50", tags: "4 credit 4×50 extended bell schedule");
    time!(t, name: "TWRF0800+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "TWRF0900+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "TWRF1000+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "TWRF1100+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "TWRF1200+50", tags: "4 credit bell schedule", "4 credit 4×50 bell schedule", "4 credit 4×50 extended bell schedule");
    time!(t, name: "TWRF1300+50", tags: "4 credit 4×50 extended bell schedule");
    time!(t, name: "TWRF1400+50", tags: "4 credit 4×50 extended bell schedule");
    time!(t, name: "TWRF1500+50", tags: "4 credit 4×50 extended bell schedule");
    time!(t, name: "MTWRF0800+50", tags: "5 credit bell schedule", "5 credit extended bell schedule");
    time!(t, name: "MTWRF0900+50", tags: "5 credit bell schedule", "5 credit extended bell schedule");
    time!(t, name: "MTWRF1000+50", tags: "5 credit bell schedule", "5 credit extended bell schedule");
    time!(t, name: "MTWRF1100+50", tags: "5 credit bell schedule", "5 credit extended bell schedule");
    time!(t, name: "MTWRF1200+50", tags: "5 credit bell schedule", "5 credit extended bell schedule");
    time!(t, name: "MTWRF1300+50", tags: "5 credit extended bell schedule");
    time!(t, name: "MTWRF1400+50", tags: "5 credit extended bell schedule");
    time!(t, name: "MTWRF1500+50", tags: "5 credit extended bell schedule");
    time!(t, name: "MTWRF1600+50", tags: "5 credit extended bell schedule");
    time!(t, name: "M1030+75");
    time!(t, name: "R0900+75");
    time!(t, name: "R1330+75");
    time!(t, name: "T1330+75");
    time!(t, name: "T1500+75");
    time!(t, name: "W1030+75");
    time!(t, name: "MW0730+75", tags: "3 credit bell schedule", "2×75", "mw");
    time!(t, name: "MW1200+75", tags: "3 credit bell schedule", "2×75", "mw");
    time!(t, name: "MW1330+75", tags: "3 credit bell schedule", "2×75", "mw");
    time!(t, name: "MW1500+75", tags: "3 credit bell schedule", "2×75", "mw");
    time!(t, name: "MW1530+75");
    time!(t, name: "MW1630+75", tags: "3 credit bell schedule", "2×75", "mw");
    time!(t, name: "MW1645+75");
    time!(t, name: "MW1800+75");
    time!(t, name: "TR0730+75", tags: "3 credit bell schedule", "2×75", "tr");
    time!(t, name: "TR0900+75", tags: "3 credit bell schedule", "2×75", "tr");
    time!(t, name: "TR1030+75", tags: "3 credit bell schedule", "2×75", "tr");
    time!(t, name: "TR1200+75", tags: "3 credit bell schedule", "2×75", "tr");
    time!(t, name: "TR1330+75", tags: "3 credit bell schedule", "2×75", "tr");
    time!(t, name: "TR1500+75", tags: "3 credit bell schedule", "2×75", "tr");
    time!(t, name: "TR1630+75", tags: "3 credit bell schedule", "2×75", "tr");
    time!(t, name: "TR1800+75");
    time!(t, name: "MW1300+100", tags: "4 credit bell schedule", "4 credit 2×100 bell schedule");
    time!(t, name: "MW1500+100", tags: "4 credit bell schedule", "4 credit 2×100 bell schedule");
    time!(t, name: "MW1600+100");
    time!(t, name: "MW1630+100");
    time!(t, name: "MW1800+100");
    time!(t, name: "TR1300+100", tags: "4 credit bell schedule", "4 credit 2×100 bell schedule");
    time!(t, name: "TR1500+100", tags: "4 credit bell schedule", "4 credit 2×100 bell schedule");
    time!(t, name: "TR1630+100");
    time!(t, name: "TR1800+100");
    time!(t, name: "F0800+110", tags: "2 hour lab");
    time!(t, name: "F0900+110", tags: "2 hour lab");
    time!(t, name: "F1000+110", tags: "2 hour lab");
    time!(t, name: "F1100+110", tags: "2 hour lab");
    time!(t, name: "F1200+110", tags: "2 hour lab");
    time!(t, name: "F1300+110", tags: "2 hour lab");
    time!(t, name: "M0800+110", tags: "2 hour lab");
    time!(t, name: "M0900+110", tags: "2 hour lab");
    time!(t, name: "M1000+110", tags: "2 hour lab");
    time!(t, name: "M1100+110", tags: "2 hour lab");
    time!(t, name: "M1200+110", tags: "2 hour lab");
    time!(t, name: "M1300+110", tags: "2 hour lab");
    time!(t, name: "M1400+110", tags: "2 hour lab");
    time!(t, name: "M1500+110", tags: "2 hour lab");
    time!(t, name: "M1600+110", tags: "2 hour lab");
    time!(t, name: "M1700+110", tags: "2 hour lab");
    time!(t, name: "R0800+110", tags: "2 hour lab");
    time!(t, name: "R0900+110", tags: "2 hour lab");
    time!(t, name: "R1000+110", tags: "2 hour lab");
    time!(t, name: "R1100+110", tags: "2 hour lab");
    time!(t, name: "R1200+110", tags: "2 hour lab");
    time!(t, name: "R1300+110", tags: "2 hour lab");
    time!(t, name: "R1400+110", tags: "2 hour lab");
    time!(t, name: "R1500+110", tags: "2 hour lab");
    time!(t, name: "R1600+110", tags: "2 hour lab");
    time!(t, name: "R1700+110", tags: "2 hour lab");
    time!(t, name: "R1715+110");
    time!(t, name: "R1800+110", tags: "2 hour lab evening");
    time!(t, name: "R1900+110", tags: "2 hour lab late evening");
    time!(t, name: "T0800+110", tags: "2 hour lab");
    time!(t, name: "T0900+110", tags: "2 hour lab");
    time!(t, name: "T1000+110", tags: "2 hour lab");
    time!(t, name: "T1100+110", tags: "2 hour lab");
    time!(t, name: "T1200+110", tags: "2 hour lab");
    time!(t, name: "T1300+110", tags: "2 hour lab");
    time!(t, name: "T1400+110", tags: "2 hour lab");
    time!(t, name: "T1500+110", tags: "2 hour lab");
    time!(t, name: "T1600+110", tags: "2 hour lab");
    time!(t, name: "T1700+110", tags: "2 hour lab");
    time!(t, name: "T1800+110", tags: "2 hour lab evening");
    time!(t, name: "T1900+110", tags: "2 hour lab late evening");
    time!(t, name: "W0800+110", tags: "2 hour lab");
    time!(t, name: "W0900+110", tags: "2 hour lab");
    time!(t, name: "W1000+110", tags: "2 hour lab");
    time!(t, name: "W1100+110", tags: "2 hour lab");
    time!(t, name: "W1200+110", tags: "2 hour lab");
    time!(t, name: "W1300+110", tags: "2 hour lab");
    time!(t, name: "W1400+110", tags: "2 hour lab");
    time!(t, name: "W1500+110", tags: "2 hour lab");
    time!(t, name: "W1600+110", tags: "2 hour lab");
    time!(t, name: "W1700+110", tags: "2 hour lab");
    time!(t, name: "W1715+110");
    time!(t, name: "W1800+110", tags: "2 hour lab evening");
    time!(t, name: "W1900+110", tags: "2 hour lab late evening");
    time!(t, name: "MR1100+110");
    time!(t, name: "MW0600+110", tags: "4 hour lab");
    time!(t, name: "MW0800+110", tags: "4 hour lab");
    time!(t, name: "MW0900+110", tags: "4 hour lab");
    time!(t, name: "MW1000+110", tags: "4 hour lab");
    time!(t, name: "MW1100+110", tags: "4 hour lab");
    time!(t, name: "MW1200+110", tags: "4 hour lab");
    time!(t, name: "MW1300+110", tags: "4 hour lab");
    time!(t, name: "MW1400+110", tags: "4 hour lab");
    time!(t, name: "MW1500+110", tags: "4 hour lab");
    time!(t, name: "MW1600+110", tags: "4 hour lab");
    time!(t, name: "MW1700+110", tags: "4 hour lab");
    time!(t, name: "MW1800+110", tags: "4 hour lab");
    time!(t, name: "TR0600+110", tags: "4 hour lab");
    time!(t, name: "TR0800+110", tags: "4 hour lab");
    time!(t, name: "TR0900+110", tags: "4 hour lab");
    time!(t, name: "TR1000+110", tags: "4 hour lab");
    time!(t, name: "TR1100+110", tags: "4 hour lab");
    time!(t, name: "TR1200+110", tags: "4 hour lab");
    time!(t, name: "TR1300+110", tags: "4 hour lab");
    time!(t, name: "TR1400+110", tags: "4 hour lab");
    time!(t, name: "TR1500+110", tags: "4 hour lab");
    time!(t, name: "TR1600+110", tags: "4 hour lab");
    time!(t, name: "TR1700+110", tags: "4 hour lab");
    time!(t, name: "TR1800+110", tags: "4 hour lab");
    time!(t, name: "F0800+115");
    time!(t, name: "R1200+135");
    time!(t, name: "R1530+150");
    time!(t, name: "R1800+150", tags: "3 credit evening");
    time!(t, name: "T1630+150");
    time!(t, name: "T1800+150", tags: "3 credit evening");
    time!(t, name: "W1630+150");
    time!(t, name: "W1800+150", tags: "3 credit evening");
    time!(t, name: "R1330+165");
    time!(t, name: "F0800+170");
    time!(t, name: "F1100+170");
    time!(t, name: "F1330+170");
    time!(t, name: "F1400+170");
    time!(t, name: "M1100+170");
    time!(t, name: "M1300+170");
    time!(t, name: "M1930+170");
    time!(t, name: "R0800+170");
    time!(t, name: "R1000+170");
    time!(t, name: "R1100+170");
    time!(t, name: "R1200+170");
    time!(t, name: "R1300+170");
    time!(t, name: "R1400+170");
    time!(t, name: "R1500+170");
    time!(t, name: "R1600+170");
    time!(t, name: "R1630+170");
    time!(t, name: "R1700+170");
    time!(t, name: "R1900+170");
    time!(t, name: "T0700+170");
    time!(t, name: "T0800+170");
    time!(t, name: "T0900+170");
    time!(t, name: "T1000+170");
    time!(t, name: "T1100+170");
    time!(t, name: "T1200+170");
    time!(t, name: "T1300+170");
    time!(t, name: "T1400+170");
    time!(t, name: "T1500+170");
    time!(t, name: "T1600+170");
    time!(t, name: "T1700+170");
    time!(t, name: "T1800+170");
    time!(t, name: "T1900+170");
    time!(t, name: "T1930+170");
    time!(t, name: "W0800+170");
    time!(t, name: "W0900+170");
    time!(t, name: "W1000+170");
    time!(t, name: "W1100+170");
    time!(t, name: "W1200+170");
    time!(t, name: "W1300+170");
    time!(t, name: "W1330+170");
    time!(t, name: "W1400+170");
    time!(t, name: "W1500+170");
    time!(t, name: "W1700+170");
    time!(t, name: "W1930+170");
    time!(t, name: "MW1500+170");
    time!(t, name: "TR1500+170");
    time!(t, name: "TR1600+170");
    time!(t, name: "M1400+180");
    time!(t, name: "MWF1330+180");
    time!(t, name: "S1000+300");

    Ok(())
}

pub fn input_set(t: &mut Input) -> Result<(), String> {
    room!(t, name: "BROWN 201", capacity: 65);
    room!(t, name: "COE 121", capacity: 50);
    room!(t, name: "HCC 476", capacity: 20);
    room!(t, name: "SET 101", capacity: 18);
    room!(t, name: "SET 102", capacity: 18);
    room!(t, name: "SET 104", capacity: 40);
    room!(t, name: "SET 105", capacity: 60, tags: "Science medium lecture", "Science small lecture");
    room!(t, name: "SET 106", capacity: 125, tags: "Science large lecture", "Science medium lecture", "Science small lecture");
    room!(t, name: "SET 201", capacity: 65, tags: "Science medium lecture", "Science small lecture");
    room!(t, name: "SET 213", capacity: 20);
    room!(t, name: "SET 214", capacity: 20);
    room!(t, name: "SET 215", capacity: 20);
    room!(t, name: "SET 216", capacity: 24);
    room!(t, name: "SET 219", capacity: 24);
    room!(t, name: "SET 225", capacity: 20);
    room!(t, name: "SET 226", capacity: 40);
    room!(t, name: "SET 301", capacity: 65, tags: "Science medium lecture", "Science small lecture");
    room!(t, name: "SET 303", capacity: 12);
    room!(t, name: "SET 304", capacity: 18);
    room!(t, name: "SET 308", capacity: 24);
    room!(t, name: "SET 309", capacity: 20);
    room!(t, name: "SET 310", capacity: 14);
    room!(t, name: "SET 312", capacity: 20);
    room!(t, name: "SET 318", capacity: 24);
    room!(t, name: "SET 319", capacity: 24);
    room!(t, name: "SET 404", capacity: 16);
    room!(t, name: "SET 405", capacity: 24);
    room!(t, name: "SET 407", capacity: 24);
    room!(t, name: "SET 408", capacity: 15);
    room!(t, name: "SET 409", capacity: 24);
    room!(t, name: "SET 410", capacity: 24);
    room!(t, name: "SET 412", capacity: 24);
    room!(t, name: "SET 418", capacity: 48, tags: "Science small lecture");
    room!(t, name: "SET 420", capacity: 48, tags: "Science small lecture");
    room!(t, name: "SET 501", capacity: 20);
    room!(t, name: "SET 522", capacity: 24);
    room!(t, name: "SET 523", capacity: 24);
    room!(t, name: "SET 524", capacity: 45, tags: "Science small lecture");
    room!(t, name: "SET 526", capacity: 24);
    room!(t, name: "SET 527", capacity: 24);
    room!(t, name: "SNOW 103", capacity: 16);
    room!(t, name: "SNOW 112", capacity: 42, tags: "Math lecture");
    room!(t, name: "SNOW 113", capacity: 36);
    room!(t, name: "SNOW 124", capacity: 42, tags: "Math lecture");
    room!(t, name: "SNOW 125", capacity: 42, tags: "Math lecture");
    room!(t, name: "SNOW 128", capacity: 40, tags: "Science small lecture", "Science Snow lecture");
    room!(t, name: "SNOW 144", capacity: 42, tags: "Math lecture");
    room!(t, name: "SNOW 145", capacity: 42, tags: "Math lecture");
    room!(t, name: "SNOW 147", capacity: 42, tags: "Math lecture");
    room!(t, name: "SNOW 150", capacity: 42, tags: "Math lecture");
    room!(t, name: "SNOW 151", capacity: 42, tags: "Math lecture");
    room!(t, name: "SNOW 204", capacity: 10);
    room!(t, name: "SNOW 208", capacity: 24, tags: "Science small lecture", "Science Snow lecture");
    room!(t, name: "SNOW 216", capacity: 45, tags: "Science small lecture", "Science Snow lecture");
    room!(t, name: "SNOW 3", capacity: 42, tags: "Math lecture");

    instructor!(t,
        name:
            "Alexander R Tye",
        available:
            "MTWRF 0800-1700",
    );
    // F1400+170, F1400+170, R1200+170, TR1500+75

    instructor!(t,
        name:
            "Amanda Fa'onelua",
        available:
            "MTWRF 0800-1700",
    );
    // TR1300+100

    instructor!(t,
        name:
            "Amber Rose Mortensen",
        available:
            "MTWRF 0800-1700",
    );
    // MWF0900+50, MWF1000+50, MWF1100+50, TR1030+75
    default_clustering!(t, instructor: "Amber Rose Mortensen", days: "mt");

    instructor!(t,
        name:
            "Andrew C Schiller",
        available:
            "MTWR 0800-1800",
            "F 0800-1700",
    );
    // MW1200+75, MW1500+170, T1200+110, TR1500+170
    default_clustering!(t, instructor: "Andrew C Schiller", days: "mt");

    instructor!(t,
        name:
            "Andrew Gregory Toth",
        available:
            "MTWRF 0800-1700",
    );
    // MW1200+75

    instructor!(t,
        name:
            "Bhuvaneswari Sambandham",
        available:
            "MTWRF 0800-1700",
    );
    // MTWF1000+50, MTWR1100+50, MW1200+75
    default_clustering!(t, instructor: "Bhuvaneswari Sambandham", days: "mt");

    instructor!(t,
        name:
            "Bing Jiang",
        available:
            "MTWF 0800-1700",
            "R 0800-1800",
    );
    // F1000+110, MW1200+75, MWF0900+50, R1400+110, R1600+110

    instructor!(t,
        name:
            "Brant A Ross",
        available:
            "MTWRF 0800-1700",
    );
    // MWF1330+180, MWF1330+180

    instructor!(t,
        name:
            "Bruford P Reynolds",
        available:
            "MTWRF 0800-1700",
    );
    // TR1000+50, TR1400+110

    instructor!(t,
        name:
            "Bryan K Stevens",
        available:
            "MWF 0800-1700",
            "TR 0700-1700",
    );
    // TR0730+75, TR0900+75, TR1030+75

    instructor!(t,
        name:
            "Christina M Quinn",
        available:
            "MWRF 0800-1700",
            "T 0700-1700",
    );
    // R1000+170, R1300+170, T0700+170, T1000+170, T1300+170, W1300+170

    instructor!(t,
        name:
            "Christina Pondell",
        available:
            "MTWRF 0800-1700",
    );
    // F1000+50, M1300+170, R1330+165, T1100+110, T1300+110, TR0900+75

    instructor!(t,
        name:
            "Christopher Kirk DeMacedo",
        available:
            "M 0800-2300",
            "TWRF 0800-1700",
    );
    // M1930+170, T1200+110, T1400+110

    instructor!(t,
        name:
            "Clare C Banks",
        available:
            "MTWRF 0800-1700",
    );
    // MTWR0800+50, MTWR1200+50

    instructor!(t,
        name:
            "Costel Ionita",
        available:
            "MTWRF 0800-1700",
    );
    // F1100+50, MTWR0800+50, MTWR0900+50, MTWR1100+50, TR1200+75
    default_clustering!(t, instructor: "Costel Ionita", days: "mt");

    instructor!(t,
        name:
            "Craig D Seegmiller",
        available:
            "MWF 0800-1700",
            "TR 0700-1700",
    );
    // MTWR1200+50, TR0730+75, TR0900+75
    default_clustering!(t, instructor: "Craig D Seegmiller", days: "mt");

    instructor!(t,
        name:
            "Curtis B Walker",
        available:
            "MTWRF 0800-1700",
    );
    // MW1330+75, MW1330+75, R1330+75, T1330+75, T1400+170, TR1200+75
    default_clustering!(t, instructor: "Curtis B Walker", days: "mt");

    instructor!(t,
        name:
            "Cutler Cowdin",
        available:
            "MWF 0800-1700",
            "TR 0800-1900",
    );
    // R1600+170, T1600+170

    instructor!(t,
        name:
            "David Brent Christensen",
        available:
            "MTWRF 0800-1700",
    );
    // R0800+110, R1000+110, R1400+110, T1200+110

    instructor!(t,
        name:
            "David J Burr",
        available:
            "MWF 0800-1700",
            "TR 0800-2200",
    );
    // R1900+170, T1600+170, T1900+170

    instructor!(t,
        name:
            "David M Syndergaard",
        available:
            "MW 0800-2000",
            "TRF 0800-1700",
    );
    // M1300+110, MW1630+75, MW1800+75

    instructor!(t,
        name:
            "David R Black",
        available:
            "MWRF 0800-1700",
            "T 0800-1900",
    );
    // T1700+110

    instructor!(t,
        name:
            "David W Bean",
        available:
            "MTRF 0800-1700",
            "W 0800-1800",
    );
    // F1100+170, R1400+170, W1500+170

    instructor!(t,
        name:
            "Dawn Lashell Kidd-Thomas",
        available:
            "MTWRF 0800-1700",
    );
    // TR1300+100

    instructor!(t,
        name:
            "Del William Smith",
        available:
            "TR 1330-1900",
    );
    // TR1330+75, TR1500+50, TR1600+170

    instructor!(t,
        name:
            "Diana L Reese",
        available:
            "MTWRF 0800-1700",
    );
    // MTWR0900+50, MTWR1000+50, MTWRF1200+50, MTWRF1600+50
    default_clustering!(t, instructor: "Diana L Reese", days: "mt");

    instructor!(t,
        name:
            "Divya Singh",
        available:
            "MW 0800-1800",
            "TRF 0800-1700",
    );
    // MW1000+110, MW1500+75, MW1630+75, T1200+110

    instructor!(t,
        name:
            "Donald H Warner",
        available:
            "MTWRF 0800-1700",
    );
    // MW1500+75

    instructor!(t,
        name:
            "Douglas J Sainsbury",
        available:
            "MTWRF 0800-1700",
    );
    // MTWRF0800+50, TR1200+75, W1200+50

    instructor!(t,
        name:
            "Elizabeth Karen Ludlow",
        available:
            "MTWRF 0800-1700",
    );
    // MW1300+100, MW1500+75

    instructor!(t,
        name:
            "Erin E O'Brien",
        available:
            "MRF 0800-1700",
            "TW 0800-1800",
    );
    // MW1200+75, T1500+170, W1500+170

    instructor!(t,
        name:
            "Gabriela Chilom",
        available:
            "MTWF 0800-1700",
            "R 0800-1800",
    );
    // MTWR0800+50, MTWR1400+50, MTWRF1500+50, MWF1000+50, R1500+170

    instructor!(t,
        name:
            "Geoffrey Smith",
        available:
            "MTWRF 0800-1700",
    );
    // MTWR1100+50, TR1500+75

    instructor!(t,
        name:
            "Glorimar L Aponte-Kline",
        available:
            "MTWRF 0800-1700",
    );
    // TR0900+75, TR1030+75, TR1330+75

    instructor!(t,
        name:
            "Greg L Melton",
        available:
            "MTWRF 0800-1700",
    );
    // MW1330+75, MW1500+75, T1200+170, TR0900+75, W0900+110

    instructor!(t,
        name:
            "Hugo Elio Angeles",
        available:
            "MWF 0800-1700",
            "TR 0800-2000",
    );
    // TR1800+75

    instructor!(t,
        name:
            "Hung Yu Shih",
        available:
            "MTWRF 0800-1700",
    );
    // T1300+110, T1300+110, T1500+50, T1600+50, W1330+170

    instructor!(t,
        name:
            "Jacson Parker",
        available:
            "MWF 0800-1700",
            "TR 0800-1900",
    );
    // R1600+170, T1600+170

    instructor!(t,
        name:
            "James David Meidell",
        available:
            "MW 0800-1800",
            "TF 0800-1700",
            "R 0800-2000",
    );
    // MW1630+75, R1700+170

    instructor!(t,
        name:
            "James P Fitzgerald",
        available:
            "MTWRF 0800-1700",
    );
    // MWF0800+50, MWF0900+50, MWF1000+50

    instructor!(t,
        name:
            "Jameson C Hardy",
        available:
            "MTWRF 0800-1700",
    );
    // MTWR0900+50, MTWRF1000+50, MW1200+75, TR1200+75

    instructor!(t,
        name:
            "Janice M Hayden",
        available:
            "MTWRF 0800-1700",
    );
    // TR0900+75, W1100+170

    instructor!(t,
        name:
            "Jared M Hancock",
        available:
            "MTWRF 0800-1700",
    );
    // M1100+110, MTWR0800+50, MTWR0900+50, MTWR1400+50, W1000+170

    instructor!(t,
        name:
            "Jeffrey Anderson",
        available:
            "MW 0800-1800",
            "TRF 0800-1700",
    );
    // MW1630+75, T1400+110, TR0900+75

    instructor!(t,
        name:
            "Jeffrey P Harrah",
        available:
            "MRF 0800-1700",
            "TW 0800-1900",
    );
    // T1630+150, TR1030+75, TR1200+75, TR1330+75, W1630+150

    instructor!(t,
        name:
            "Jeffrey V Yule",
        available:
            "MTWRF 0800-1700",
    );
    // M1030+75, MWF1100+50, TR1030+75, TR1030+75, W1030+75

    instructor!(t,
        name:
            "Jennifer A Meyer",
        available:
            "MTWRF 0800-1700",
    );
    // MW1200+75, MW1330+75, R1300+170, T1300+170

    instructor!(t,
        name:
            "Jennifer L Ciaccio",
        available:
            "MTWRF 0800-1700",
    );
    // MTRF1200+50, MWF0900+50, R0900+75, W1200+170

    instructor!(t,
        name:
            "Jerald D Harris",
        available:
            "MTWF 0800-1700",
            "R 0800-2000",
    );
    // MWF1000+50, MWF1100+50, MWF1100+50, R1000+50, R1630+170, TR1030+75

    instructor!(t,
        name:
            "Jeremy W Bakelar",
        available:
            "MWRF 0800-1700",
            "T 0800-1800",
    );
    // MW1500+75, MWF1100+50, T0900+170, T1500+170, TR1300+110

    instructor!(t,
        name:
            "Jesse William Breinholt",
        available:
            "MTWRF 0800-1700",
    );
    // TR1500+75

    instructor!(t,
        name:
            "Jie Liu",
        available:
            "MTWRF 0800-1700",
    );
    // T1500+75, TR1030+75, TR1200+75, TR1330+75

    instructor!(t,
        name:
            "John E Wolfe",
        available:
            "MTWRF 0800-1700",
    );
    // MWF1100+50

    instructor!(t,
        name:
            "Jose C Saraiva",
        available:
            "MF 0800-1700",
            "T 0800-1800",
            "W 0800-2300",
            "R 0800-2000",
    );
    // R1600+110, R1800+110, T1600+110, W1930+170

    instructor!(t,
        name:
            "Joseph B Platt",
        available:
            "MTWRF 0800-1700",
    );
    // R1100+170

    instructor!(t,
        name:
            "Kameron J Eves",
        available:
            "MTWF 0800-1700",
            "R 0800-1800",
    );
    // MW1500+75, MWF1100+50, R1600+110, TR1030+75

    instructor!(t,
        name:
            "Karen L Bauer",
        available:
            "MTWRF 0800-1700",
    );
    // MTWF1000+50, MTWF1100+50, MWF0800+50, TR1500+75

    instructor!(t,
        name:
            "Kathryn E Ott",
        available:
            "MTWRF 0800-1700",
    );
    // MW1300+100

    instructor!(t,
        name:
            "Kerby Robinson",
        available:
            "MTWRF 0800-1700",
    );
    // F1330+170

    instructor!(t,
        name:
            "Kim C Jolley",
        available:
            "MW 0800-1900",
            "TRF 0800-1700",
    );
    // MW1300+110, MW1700+110

    instructor!(t,
        name:
            "Marius Van der Merwe",
        available:
            "MTRF 0800-1700",
            "W 0800-1900",
    );
    // MWF1000+50, T1200+170, W0900+50, W1800+50

    instructor!(t,
        name:
            "Mark L Dickson",
        available:
            "MTWRF 1530-1800",
    );
    // R1530+150

    instructor!(t,
        name:
            "Marshall Topham",
        available:
            "MTWRF 0800-1700",
    );
    // MW1330+75

    instructor!(t,
        name:
            "Martina Gaspari",
        available:
            "MTWRF 0800-1700",
    );
    // MR1100+110, MW1330+75, MWF0900+50, MWF1000+50, R0800+170

    instructor!(t,
        name:
            "Marzieh Ghasemi",
        available:
            "MTWRF 0800-1700",
    );
    // MW1200+75, MWF1000+50, TR1200+75, TR1500+75

    instructor!(t,
        name:
            "Md Sazib Hasan",
        available:
            "MTWRF 0800-1700",
    );
    // TR0900+75, TR1030+75

    instructor!(t,
        name:
            "Megan R Liljenquist",
        available:
            "MTF 0800-1700",
            "W 0800-1800",
            "R 0800-1900",
    );
    // R1600+170, W1500+170

    instructor!(t,
        name:
            "Megen E Kepas",
        available:
            "MTWRF 0800-1700",
    );
    // MW1330+75, MW1500+75, R1200+135

    instructor!(t,
        name:
            "Michael N Paxman",
        available:
            "MWF 0800-1700",
            "TR 0800-1900",
    );
    // TR1630+100

    instructor!(t,
        name:
            "Nathan St Andre",
        available:
            "MTWRF 0800-1700",
    );
    // TR1200+75

    instructor!(t,
        name:
            "Nikell Dodge",
        available:
            "MWF 0800-1700",
            "TR 0800-1800",
    );
    // TR1630+75

    instructor!(t,
        name:
            "Odean Bowler",
        available:
            "MTWRF 0800-1700",
    );
    // MW1500+100, TR1500+100

    instructor!(t,
        name:
            "Paul H Shirley",
        available:
            "MWRF 0800-1700",
            "T 0800-2200",
    );
    // T1600+170, T1900+170

    instructor!(t,
        name:
            "Paula Manuele Temple",
        available:
            "MTWRF 0800-1700",
    );
    // MTWR1200+50, MW1300+100, MW1500+75, TR1300+100

    instructor!(t,
        name:
            "Randy Klabacka",
        available:
            "MTWRF 0800-1700",
    );
    // MW1330+50, MWF0900+50, MWF0900+50, R0900+50, T0900+50, TR1330+75

    instructor!(t,
        name:
            "Rick L Peirce",
        available:
            "MWRF 0800-1700",
            "T 0800-2300",
    );
    // T1930+170

    instructor!(t,
        name:
            "Rico Del Sesto",
        available:
            "MTWRF 0800-1700",
    );
    // MTWRF0900+50, MTWRF1000+50, MTWRF1100+50

    instructor!(t,
        name:
            "Rita Rae Osborn",
        available:
            "MTWRF 0800-1700",
    );
    // M0800+50

    instructor!(t,
        name:
            "Robert T Reimer",
        available:
            "MW 0800-1800",
            "TRF 0800-1700",
    );
    // MW1630+75

    instructor!(t,
        name:
            "Ross C Decker",
        available:
            "MTWRF 0800-1700",
    );
    // TR0900+75, TR1030+75

    instructor!(t,
        name:
            "Russell C Reid",
        available:
            "MTWF 0800-1700",
            "R 0800-1800",
    );
    // MTWF0900+50, MTWF0900+50, MW1500+75, R0800+110, R1000+110, R1200+110, R1400+110, R1600+110

    instructor!(t,
        name:
            "Ryan C McConnell",
        available:
            "MWF 0800-1700",
            "TR 0800-1800",
    );
    // TR1630+75

    instructor!(t,
        name:
            "Sai C Radavaram",
        available:
            "MWF 0800-1700",
            "TR 0800-1800",
    );
    // F0800+115, MW1330+75, MWF1100+50, T0800+110, TR1630+75

    instructor!(t,
        name:
            "Samuel K Tobler",
        available:
            "MTWRF 0800-1700",
    );
    // MTWF1300+50, MTWF1400+50

    instructor!(t,
        name:
            "Sarah Morgan Black",
        available:
            "MTWRF 0800-1700",
    );
    // TR1030+75, TR1330+75

    instructor!(t,
        name:
            "Scott A Skeen",
        available:
            "MTWRF 0800-1700",
    );
    // M0800+50, MW1200+75, MW1330+75, MWF1000+50, R0800+110, R1200+110, TR1500+75

    instructor!(t,
        name:
            "Scott B Griffin",
        available:
            "MTWRF 0800-1700",
    );
    // F1330+170, MW1200+75

    instructor!(t,
        name:
            "Scott E Bulloch",
        available:
            "MTWF 0800-1700",
            "R 0800-1900",
    );
    // R1600+170

    instructor!(t,
        name:
            "Scott Patrick Hicks",
        available:
            "MW 0800-2000",
            "TRF 0800-1700",
    );
    // MW1600+100, MW1800+100

    instructor!(t,
        name:
            "Steven K Sullivan",
        available:
            "MTWRF 0800-1700",
    );
    // MWRF0800+50, MWRF1000+50, MWRF1100+50

    instructor!(t,
        name:
            "Steven McKay Sullivan",
        available:
            "MTWRF 0800-1700",
    );
    // MTWR0900+50, MWF1000+50, TR1030+75

    instructor!(t,
        name:
            "Teisha Richan",
        available:
            "MTWRF 0800-1700",
    );
    // R1000+170, R1300+170, T0900+170, T1200+170, W0900+170, W1200+170

    instructor!(t,
        name:
            "Trevor K Johnson",
        available:
            "MTWRF 0800-1700",
    );
    // MTWR1200+50, MW1330+75

    instructor!(t,
        name:
            "Tye K Rogers",
        available:
            "MTWRF 0800-1700",
    );
    // MTWR0800+50, MTWR1000+50, MWF1100+50, TR1330+75

    instructor!(t,
        name:
            "Vinodh Kumar Chellamuthu",
        available:
            "MW 0800-1800",
            "TRF 0800-1700",
    );
    // MW1500+100, MW1645+75

    instructor!(t,
        name:
            "Violeta Adina Ionita",
        available:
            "MTWRF 0800-1700",
    );
    // MTWR0800+50, MTWR0900+50, MTWR1100+50, MTWR1200+50

    instructor!(t,
        name:
            "Wendy E Schatzberg",
        available:
            "MWRF 0800-1700",
            "T 0800-1900",
    );
    // F1200+50, MTWR1000+50, MTWR1100+50, MTWRF1200+50, T1600+170

    instructor!(t,
        name:
            "Zhenyu Jin",
        available:
            "MTWRF 0800-1700",
    );
    // MW1200+75, MW1330+75, T1200+170, TR1030+75, W0900+110

    // BIOL 1010-01: General Biology (LS)
    // assigned to BROWN 201 at TR0730+75
    section!(t, course: "BIOL 1010-01",
                instructor: "Bryan K Stevens",
                rooms and times:
                    "BROWN 201",
                    "3 credit bell schedule",
    );

    // BIOL 1010-02: General Biology (LS)
    // assigned to BROWN 201 at TR0900+75
    section!(t, course: "BIOL 1010-02",
                instructor: "Bryan K Stevens",
                rooms and times:
                    "BROWN 201",
                    "3 credit bell schedule",
    );

    // BIOL 1010-03: General Biology (LS)
    // assigned to SET 301 at MWF0800+50
    section!(t, course: "BIOL 1010-03",
                instructor: "Karen L Bauer",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 1010-04: General Biology (LS)
    // assigned to COE 121 at MWF1000+50
    section!(t, course: "BIOL 1010-04",
                instructor: "Martina Gaspari",
                rooms and times:
                    "COE 121",
                    "3 credit bell schedule",
    );

    // BIOL 1010-05: General Biology: Supplemental Instruction (LS)
    // assigned to SET 106 at TR1030+75
    section!(t, course: "BIOL 1010-05",
                instructor: "Jeffrey V Yule",
                rooms and times:
                    "Science large lecture",
                    "3 credit bell schedule",
    );

    // BIOL 1010-05-alt: General Biology: Supplemental Instruction (LS)
    // assigned to SNOW 113 at W1030+75
    section!(t, course: "BIOL 1010-05-SI",
                //instructor: "Jeffrey V Yule",
                rooms and times:
                    "SNOW 113",
                    "W1030+75",
    );
    conflict!(t, set hard,
            clique: "BIOL 1010-05", "BIOL 1010-05-SI",
    );

    // BIOL 1010-06: General Biology (LS)
    // assigned to BROWN 201 at MWF1100+50
    section!(t, course: "BIOL 1010-06",
                instructor: "Jeffrey V Yule",
                rooms and times:
                    "BROWN 201",
                    "3 credit bell schedule",
    );

    // BIOL 1010-07: General Biology (LS)
    // assigned to SET 105 at TR1200+75
    section!(t, course: "BIOL 1010-07",
                instructor: "Nathan St Andre",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 1010-08: General Biology (LS)
    // assigned to SNOW 151 at TR1330+75
    section!(t, course: "BIOL 1010-08",
                instructor: "Del William Smith",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // BIOL 1010-09: General Biology (LS)
    // assigned to SET 420 at MW1630+75
    section!(t, course: "BIOL 1010-09",
                instructor: "James David Meidell",
                rooms and times:
                    "Science small lecture",
                    "3 credit bell schedule",
    );

    // BIOL 1010-10: General Biology (LS)
    // assigned to SET 301 at TR1630+75
    section!(t, course: "BIOL 1010-10",
                instructor: "Nikell Dodge",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 1010-11: General Biology (LS)
    // assigned to SNOW 113 at M1030+75
    section!(t, course: "BIOL 1010-11-SI",
                //instructor: "Jeffrey V Yule",
                rooms and times:
                    "SNOW 113",
                    "M1030+75",
    );

    // BIOL 1010-11-alt: General Biology (LS)
    // assigned to SET 106 at TR1030+75
    section!(t, course: "BIOL 1010-11",
                instructor: "Jeffrey V Yule",
                rooms and times:
                    "Science large lecture",
                    "3 credit bell schedule",
    );
    conflict!(t, set hard,
            clique: "BIOL 1010-11", "BIOL 1010-11-SI",
    );

    // BIOL 1010-50: General Biology (LS)
    // assigned to SNOW 112 at TR1800+75
    section!(t, course: "BIOL 1010-50",
                rooms and times:
                    "Math lecture",
                    "TR1800+75",
    );

    // BIOL 1015-03: General Biology Lab (LAB)
    // assigned to SET 312 at M1100+170
    section!(t, course: "BIOL 1015-03",
                rooms and times:
                    "SET 312",
                    "M1100+170",
    );

    // BIOL 1015-04: General Biology Lab (LAB)
    // assigned to SET 312 at T1100+170
    section!(t, course: "BIOL 1015-04",
                rooms and times:
                    "SET 312",
                    "T1100+170",
    );

    // BIOL 1015-05: General Biology Lab (LAB)
    // assigned to SET 312 at W1100+170
    section!(t, course: "BIOL 1015-05",
                rooms and times:
                    "SET 312",
                    "W1100+170",
    );

    // BIOL 1015-07: General Biology Lab (LAB)
    // assigned to SET 312 at T1400+170
    section!(t, course: "BIOL 1015-07",
                rooms and times:
                    "SET 312",
                    "T1400+170",
    );

    // BIOL 1015-51: General Biology Lab (LAB)
    // assigned to SET 312 at T1700+170
    section!(t, course: "BIOL 1015-51",
                rooms and times:
                    "SET 312",
                    "T1700+170",
    );

    // BIOL 1200-01: Human Biology (LS)
    // assigned to BROWN 201 at TR1030+75
    section!(t, course: "BIOL 1200-01",
                instructor: "Amber Rose Mortensen",
                rooms and times:
                    "BROWN 201",
                    "3 credit bell schedule",
    );

    // BIOL 1200-02: Human Biology (LS)
    // assigned to SET 105 at TR1500+75
    section!(t, course: "BIOL 1200-02",
                instructor: "Karen L Bauer",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 1610-01: Principles of Biology I (LS)
    // assigned to SET 106 at MTWRF0800+50
    section!(t, course: "BIOL 1610-01",
                instructor: "Douglas J Sainsbury",
                rooms and times:
                    "Science large lecture",
                    "5 credit bell schedule",
    );

    // BIOL 1610-02: Principles of Biology I (LS)
    // assigned to SET 105 at MTWF1100+50
    section!(t, course: "BIOL 1610-02",
                instructor: "Karen L Bauer",
                rooms and times:
                    "Science medium lecture",
                    "4 credit bell schedule",
    );

    // BIOL 1615-01: Principles of Biology I Lab (LAB)
    // assigned to SET 309 at T0800+170
    section!(t, course: "BIOL 1615-01",
                rooms and times:
                    "SET 309",
                    "T0800+170",
    );

    // BIOL 1615-02: Principles of Biology I Lab (LAB)
    // assigned to SET 309 at W0800+170
    section!(t, course: "BIOL 1615-02",
                rooms and times:
                    "SET 309",
                    "W0800+170",
    );

    // BIOL 1615-03: Principles of Biology I Lab (LAB)
    // assigned to SET 309 at R0800+170
    section!(t, course: "BIOL 1615-03",
                rooms and times:
                    "SET 309",
                    "R0800+170",
    );

    // BIOL 1615-04: Principles of Biology I Lab (LAB)
    // assigned to SET 309 at F0800+170
    section!(t, course: "BIOL 1615-04",
                rooms and times:
                    "SET 309",
                    "F0800+170",
    );

    // BIOL 1615-05: Principles of Biology I Lab (LAB)
    // assigned to SET 309 at T1100+170
    section!(t, course: "BIOL 1615-05",
                rooms and times:
                    "SET 309",
                    "T1100+170",
    );

    // BIOL 1615-06: Principles of Biology I Lab (LAB)
    // assigned to SET 309 at W1100+170
    section!(t, course: "BIOL 1615-06",
                rooms and times:
                    "SET 309",
                    "W1100+170",
    );

    // BIOL 1615-07: Principles of Biology I Lab (LAB)
    // assigned to SET 309 at R1100+170
    section!(t, course: "BIOL 1615-07",
                rooms and times:
                    "SET 309",
                    "R1100+170",
    );

    // BIOL 1615-08: Principles of Biology I Lab (LAB)
    // assigned to SET 309 at F1100+170
    section!(t, course: "BIOL 1615-08",
                rooms and times:
                    "SET 309",
                    "F1100+170",
    );

    // BIOL 1615-09: Principles of Biology I Lab (LAB)
    // assigned to SET 309 at T1400+170
    section!(t, course: "BIOL 1615-09",
                rooms and times:
                    "SET 309",
                    "T1400+170",
    );

    // BIOL 1615-10: Principles of Biology I Lab (LAB)
    // assigned to SET 309 at W1400+170
    section!(t, course: "BIOL 1615-10",
                rooms and times:
                    "SET 309",
                    "W1400+170",
    );

    // BIOL 1615-11: Principles of Biology I Lab (LAB)
    // assigned to SET 309 at R1400+170
    section!(t, course: "BIOL 1615-11",
                rooms and times:
                    "SET 309",
                    "R1400+170",
    );

    // BIOL 1615-12: Principles of Biology I Lab (LAB)
    // assigned to SET 309 at F1400+170
    section!(t, course: "BIOL 1615-12",
                rooms and times:
                    "SET 309",
                    "F1400+170",
    );

    // BIOL 1615-50: Principles of Biology I Lab (LAB)
    // assigned to SET 309 at T1700+170
    section!(t, course: "BIOL 1615-50",
                rooms and times:
                    "SET 309",
                    "T1700+170",
    );

    // BIOL 1615-51: Principles of Biology I Lab (LAB)
    // assigned to SET 309 at W1700+170
    section!(t, course: "BIOL 1615-51",
                rooms and times:
                    "SET 309",
                    "W1700+170",
    );

    // BIOL 1615-52: Principles of Biology I Lab (LAB)
    // assigned to SET 309 at R1700+170
    section!(t, course: "BIOL 1615-52",
                rooms and times:
                    "SET 309",
                    "R1700+170",
    );

    // BIOL 1620-01: Principles of Biology II
    // assigned to SET 105 at MTWF1000+50
    section!(t, course: "BIOL 1620-01",
                instructor: "Karen L Bauer",
                rooms and times:
                    "Science medium lecture",
                    "4 credit bell schedule",
    );

    // BIOL 1620-02: Principles of Biology II
    // assigned to SET 106 at MTRF1200+50
    section!(t, course: "BIOL 1620-02",
                instructor: "Jennifer L Ciaccio",
                rooms and times:
                    "Science large lecture",
                    "4 credit bell schedule",
    );

    // BIOL 1620-03: Principles of Biology II (HONORS)
    // assigned to SET 216 at MTWR1100+50
    section!(t, course: "BIOL 1620-03",
                instructor: "Geoffrey Smith",
                rooms and times:
                    "SET 216",
                    "4 credit bell schedule",
    );

    // BIOL 1625-01: Principles of Biology II Lab
    // assigned to SET 318 at R0800+170
    section!(t, course: "BIOL 1625-01",
                rooms and times:
                    "SET 318",
                    "R0800+170",
    );

    // BIOL 1625-02: Principles of Biology II Lab
    // assigned to SET 318 at R1100+170
    section!(t, course: "BIOL 1625-02",
                instructor: "Joseph B Platt",
                rooms and times:
                    "SET 318",
                    "R1100+170",
    );

    // BIOL 1625-03: Principles of Biology II Lab
    // assigned to SET 318 at W1200+170
    section!(t, course: "BIOL 1625-03",
                instructor: "Jennifer L Ciaccio",
                rooms and times:
                    "SET 318",
                    "W1200+170",
    );

    // BIOL 1625-04: Principles of Biology II Lab
    // assigned to SET 318 at R1400+170
    section!(t, course: "BIOL 1625-04",
                instructor: "David W Bean",
                rooms and times:
                    "SET 318",
                    "R1400+170",
    );

    // BIOL 1625-05: Principles of Biology II Lab
    // assigned to SET 318 at F1100+170
    section!(t, course: "BIOL 1625-05",
                instructor: "David W Bean",
                rooms and times:
                    "SET 318",
                    "F1100+170",
    );

    // BIOL 1625-06: Principles of Biology II Lab
    // assigned to SET 318 at W1500+170
    section!(t, course: "BIOL 1625-06",
                instructor: "David W Bean",
                rooms and times:
                    "SET 318",
                    "W1500+170",
    );

    // BIOL 1625-50: Principles of Biology II Lab
    // assigned to SET 318 at R1700+170
    section!(t, course: "BIOL 1625-50",
                instructor: "James David Meidell",
                rooms and times:
                    "SET 318",
                    "R1700+170",
    );

    // BIOL 2060-01: Principles of Microbiology
    // assigned to SET 105 at MW1500+75
    section!(t, course: "BIOL 2060-01",
                instructor: "Jeremy W Bakelar",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 2065-01: Principles of Microbiology Lab
    // assigned to SET 304 at MW1300+110
    section!(t, course: "BIOL 2065-01",
                instructor: "Kim C Jolley",
                rooms and times:
                    "SET 304",
                    "4 hour lab",
    );

    // BIOL 2065-02: Principles of Microbiology Lab
    // assigned to SET 304 at MW1700+110
    section!(t, course: "BIOL 2065-02",
                instructor: "Kim C Jolley",
                rooms and times:
                    "SET 304",
                    "4 hour lab",
    );

    // BIOL 2065-03: Principles of Microbiology Lab
    // assigned to SET 304 at S1000+300
    //section!(t, course: "BIOL 2065-03",
    //            instructor: "Kim C Jolley",
    //            rooms and times:
    //                "SET 304",
    //                "S1000+300",
    //);

    // BIOL 2300-01: Fundamentals of Bioinformatics
    // assigned to SET 216 at MW1330+50
    section!(t, course: "BIOL 2300-01",
                instructor: "Randy Klabacka",
                rooms and times:
                    "SET 216",
                    "2 credit lecture",
    );

    // BIOL 2320-01: Human Anatomy
    // assigned to BROWN 201 at MWF1000+50
    section!(t, course: "BIOL 2320-01",
                rooms and times:
                    "BROWN 201",
                    "3 credit bell schedule",
    );

    // BIOL 2320-02: Human Anatomy
    // assigned to SET 301 at MW1200+75
    section!(t, course: "BIOL 2320-02",
                instructor: "Scott B Griffin",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 2320-04: Human Anatomy: Supplemental Instruction
    // assigned to SET 301 at MW1330+75
    section!(t, course: "BIOL 2320-04",
                instructor: "Curtis B Walker",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 2320-04-alt: Human Anatomy: Supplemental Instruction
    // assigned to SET 105 at T1330+75
    section!(t, course: "BIOL 2320-04-SI",
                //instructor: "Curtis B Walker",
                rooms and times:
                    "Science medium lecture",
                    "T1330+75",
    );
    conflict!(t, set hard,
            clique: "BIOL 2320-04", "BIOL 2320-04-SI",
    );

    // BIOL 2320-05: Human Anatomy
    // assigned to SET 301 at TR1030+75
    section!(t, course: "BIOL 2320-05",
                instructor: "Glorimar L Aponte-Kline",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 2320-06: Human Anatomy
    // assigned to SET 201 at TR1500+75
    section!(t, course: "BIOL 2320-06",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 2320-07: Human Anatomy
    // assigned to SET 301 at TR1330+75
    section!(t, course: "BIOL 2320-07",
                instructor: "Glorimar L Aponte-Kline",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 2320-08: Human Anatomy: Supplemental Instruction
    // assigned to SET 301 at MW1330+75
    section!(t, course: "BIOL 2320-08",
                instructor: "Curtis B Walker",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 2320-08-alt: Human Anatomy: Supplemental Instruction
    // assigned to SET 105 at R1330+75
    section!(t, course: "BIOL 2320-08-SI",
                //instructor: "Curtis B Walker",
                rooms and times:
                    "Science medium lecture",
                    "R1330+75",
    );
    conflict!(t, set hard,
            clique: "BIOL 2320-08", "BIOL 2320-08-SI",
    );

    // BIOL 2325-01: Human Anatomy Lab
    // assigned to SET 213 at MW0600+110
    section!(t, course: "BIOL 2325-01",
                rooms and times:
                    "SET 213",
                    "4 hour lab",
    );

    // BIOL 2325-02: Human Anatomy Lab
    // assigned to SET 215 at TR0600+110
    section!(t, course: "BIOL 2325-02",
                rooms and times:
                    "SET 215",
                    "4 hour lab",
    );

    // BIOL 2325-03: Human Anatomy Lab
    // assigned to SET 213 at MW0800+110
    section!(t, course: "BIOL 2325-03",
                rooms and times:
                    "SET 213",
                    "4 hour lab",
    );

    // BIOL 2325-04: Human Anatomy Lab
    // assigned to SET 215 at MW0800+110
    section!(t, course: "BIOL 2325-04",
                rooms and times:
                    "SET 215",
                    "4 hour lab",
    );

    // BIOL 2325-05: Human Anatomy Lab
    // assigned to SET 213 at TR0800+110
    section!(t, course: "BIOL 2325-05",
                rooms and times:
                    "SET 213",
                    "4 hour lab",
    );

    // BIOL 2325-06: Human Anatomy Lab
    // assigned to SET 215 at TR0800+110
    section!(t, course: "BIOL 2325-06",
                rooms and times:
                    "SET 215",
                    "4 hour lab",
    );

    // BIOL 2325-07: Human Anatomy Lab
    // assigned to SET 213 at MW1000+110
    section!(t, course: "BIOL 2325-07",
                rooms and times:
                    "SET 213",
                    "4 hour lab",
    );

    // BIOL 2325-08: Human Anatomy Lab
    // assigned to SET 215 at MW1000+110
    section!(t, course: "BIOL 2325-08",
                rooms and times:
                    "SET 215",
                    "4 hour lab",
    );

    // BIOL 2325-09: Human Anatomy Lab
    // assigned to SET 213 at TR1000+110
    section!(t, course: "BIOL 2325-09",
                rooms and times:
                    "SET 213",
                    "4 hour lab",
    );

    // BIOL 2325-10: Human Anatomy Lab
    // assigned to SET 215 at TR1000+110
    section!(t, course: "BIOL 2325-10",
                rooms and times:
                    "SET 215",
                    "4 hour lab",
    );

    // BIOL 2325-11: Human Anatomy Lab
    // assigned to SET 213 at MW1200+110
    section!(t, course: "BIOL 2325-11",
                rooms and times:
                    "SET 213",
                    "4 hour lab",
    );

    // BIOL 2325-12: Human Anatomy Lab
    // assigned to SET 215 at MW1200+110
    section!(t, course: "BIOL 2325-12",
                rooms and times:
                    "SET 215",
                    "4 hour lab",
    );

    // BIOL 2325-13: Human Anatomy Lab
    // assigned to SET 213 at TR1200+110
    section!(t, course: "BIOL 2325-13",
                rooms and times:
                    "SET 213",
                    "4 hour lab",
    );

    // BIOL 2325-14: Human Anatomy Lab
    // assigned to SET 215 at TR1200+110
    section!(t, course: "BIOL 2325-14",
                rooms and times:
                    "SET 215",
                    "4 hour lab",
    );

    // BIOL 2325-15: Human Anatomy Lab
    // assigned to SET 213 at MW1400+110
    section!(t, course: "BIOL 2325-15",
                rooms and times:
                    "SET 213",
                    "4 hour lab",
    );

    // BIOL 2325-16: Human Anatomy Lab
    // assigned to SET 215 at MW1400+110
    section!(t, course: "BIOL 2325-16",
                rooms and times:
                    "SET 215",
                    "4 hour lab",
    );

    // BIOL 2325-17: Human Anatomy Lab
    // assigned to SET 213 at TR1400+110
    section!(t, course: "BIOL 2325-17",
                rooms and times:
                    "SET 213",
                    "4 hour lab",
    );

    // BIOL 2325-18: Human Anatomy Lab
    // assigned to SET 215 at TR1400+110
    section!(t, course: "BIOL 2325-18",
                rooms and times:
                    "SET 215",
                    "4 hour lab",
    );

    // BIOL 2325-19: Human Anatomy Lab
    // assigned to SET 213 at MW1600+110
    section!(t, course: "BIOL 2325-19",
                rooms and times:
                    "SET 213",
                    "4 hour lab",
    );

    // BIOL 2325-20: Human Anatomy Lab
    // assigned to SET 215 at MW1600+110
    section!(t, course: "BIOL 2325-20",
                rooms and times:
                    "SET 215",
                    "4 hour lab",
    );

    // BIOL 2325-21: Human Anatomy Lab
    // assigned to SET 213 at TR1600+110
    section!(t, course: "BIOL 2325-21",
                rooms and times:
                    "SET 213",
                    "4 hour lab",
    );

    // BIOL 2325-22: Human Anatomy Lab
    // assigned to SET 215 at TR1600+110
    section!(t, course: "BIOL 2325-22",
                rooms and times:
                    "SET 215",
                    "4 hour lab",
    );

    // BIOL 2325-50: Human Anatomy Lab
    // assigned to SET 213 at MW1800+110
    section!(t, course: "BIOL 2325-50",
                rooms and times:
                    "SET 213",
                    "4 hour lab",
    );

    // BIOL 2325-51: Human Anatomy Lab
    // assigned to SET 215 at MW1800+110
    section!(t, course: "BIOL 2325-51",
                rooms and times:
                    "SET 215",
                    "4 hour lab",
    );

    // BIOL 2325-52: Human Anatomy Lab
    // assigned to SET 213 at TR1800+110
    section!(t, course: "BIOL 2325-52",
                rooms and times:
                    "SET 213",
                    "4 hour lab",
    );

    // BIOL 2325-53: Human Anatomy Lab
    // assigned to SET 215 at TR1800+110
    section!(t, course: "BIOL 2325-53",
                rooms and times:
                    "SET 215",
                    "4 hour lab",
    );

    // BIOL 2420-01: Human Physiology
    // assigned to SET 106 at MWF0900+50
    section!(t, course: "BIOL 2420-01",
                instructor: "Amber Rose Mortensen",
                rooms and times:
                    "Science large lecture",
                    "3 credit bell schedule",
    );

    // BIOL 2420-02: Human Physiology
    // assigned to SET 106 at MWF1000+50
    section!(t, course: "BIOL 2420-02",
                instructor: "Amber Rose Mortensen",
                rooms and times:
                    "Science large lecture",
                    "3 credit bell schedule",
    );

    // BIOL 2420-03: Human Physiology
    // assigned to SET 106 at MWF1100+50
    section!(t, course: "BIOL 2420-03",
                instructor: "Amber Rose Mortensen",
                rooms and times:
                    "Science large lecture",
                    "3 credit bell schedule",
    );

    // BIOL 2420-04: Human Physiology
    // assigned to SET 301 at MW1500+75
    section!(t, course: "BIOL 2420-04",
                instructor: "Megen E Kepas",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 2420-05: Human Physiology
    // assigned to SET 301 at TR1500+75
    section!(t, course: "BIOL 2420-05",
                instructor: "Geoffrey Smith",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 2425-01: Human Physiology Lab
    // assigned to SET 214 at T0900+110
    section!(t, course: "BIOL 2425-01",
                rooms and times:
                    "SET 214",
                    "2 hour lab",
    );

    // BIOL 2425-02: Human Physiology Lab
    // assigned to SET 214 at W0900+110
    section!(t, course: "BIOL 2425-02",
                rooms and times:
                    "SET 214",
                    "2 hour lab",
    );

    // BIOL 2425-03: Human Physiology Lab
    // assigned to SET 214 at R0900+110
    section!(t, course: "BIOL 2425-03",
                rooms and times:
                    "SET 214",
                    "2 hour lab",
    );

    // BIOL 2425-04: Human Physiology Lab
    // assigned to SET 214 at F0900+110
    section!(t, course: "BIOL 2425-04",
                rooms and times:
                    "SET 214",
                    "2 hour lab",
    );

    // BIOL 2425-05: Human Physiology Lab
    // assigned to SET 214 at T1100+110
    section!(t, course: "BIOL 2425-05",
                rooms and times:
                    "SET 214",
                    "2 hour lab",
    );

    // BIOL 2425-06: Human Physiology Lab
    // assigned to SET 214 at W1100+110
    section!(t, course: "BIOL 2425-06",
                rooms and times:
                    "SET 214",
                    "2 hour lab",
    );

    // BIOL 2425-07: Human Physiology Lab
    // assigned to SET 214 at R1100+110
    section!(t, course: "BIOL 2425-07",
                rooms and times:
                    "SET 214",
                    "2 hour lab",
    );

    // BIOL 2425-08: Human Physiology Lab
    // assigned to SET 214 at F1100+110
    section!(t, course: "BIOL 2425-08",
                rooms and times:
                    "SET 214",
                    "2 hour lab",
    );

    // BIOL 2425-09: Human Physiology Lab
    // assigned to SET 214 at T1300+110
    section!(t, course: "BIOL 2425-09",
                rooms and times:
                    "SET 214",
                    "2 hour lab",
    );

    // BIOL 2425-10: Human Physiology Lab
    // assigned to SET 214 at W1300+110
    section!(t, course: "BIOL 2425-10",
                rooms and times:
                    "SET 214",
                    "2 hour lab",
    );

    // BIOL 2425-11: Human Physiology Lab
    // assigned to SET 214 at R1300+110
    section!(t, course: "BIOL 2425-11",
                rooms and times:
                    "SET 214",
                    "2 hour lab",
    );

    // BIOL 2425-12: Human Physiology Lab
    // assigned to SET 214 at F1300+110
    section!(t, course: "BIOL 2425-12",
                rooms and times:
                    "SET 214",
                    "2 hour lab",
    );

    // BIOL 2425-13: Human Physiology Lab
    // assigned to SET 214 at T1500+110
    section!(t, course: "BIOL 2425-13",
                rooms and times:
                    "SET 214",
                    "2 hour lab",
    );

    // BIOL 2425-14: Human Physiology Lab
    // assigned to SET 214 at W1500+110
    section!(t, course: "BIOL 2425-14",
                rooms and times:
                    "SET 214",
                    "2 hour lab",
    );

    // BIOL 2425-15: Human Physiology Lab
    // assigned to SET 214 at R1500+110
    section!(t, course: "BIOL 2425-15",
                rooms and times:
                    "SET 214",
                    "2 hour lab",
    );

    // BIOL 2425-50: Human Physiology Lab
    // assigned to SET 214 at T1700+110
    section!(t, course: "BIOL 2425-50",
                rooms and times:
                    "SET 214",
                    "2 hour lab",
    );

    // BIOL 2425-51: Human Physiology Lab
    // assigned to SET 214 at W1700+110
    section!(t, course: "BIOL 2425-51",
                rooms and times:
                    "SET 214",
                    "2 hour lab",
    );

    // BIOL 2991R-01A: Careers in Biology
    // assigned to SET 501 at W1200+50
    section!(t, course: "BIOL 2991R-01A",
                instructor: "Douglas J Sainsbury",
                rooms and times:
                    "SET 501",
                    "1 credit extended bell schedule",
    );

    // BIOL 3000R-09A: Advanced Utah Health Scholars Students
    // xlist entry: HO04
    // assigned to SET 105 at M0800+50
    section!(t, course: "BIOL 3000R-09A",
                instructor: "Rita Rae Osborn",
                rooms and times:
                    "Science medium lecture",
                    "1 credit bell schedule",
    );

    // BIOL 3010-01: Evolution
    // assigned to SET 301 at MWF1100+50
    section!(t, course: "BIOL 3010-01",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 3010-01-alt: Evolution
    // assigned to SET 301 at T1200+50
    section!(t, course: "BIOL 3010-01-SI",
                rooms and times:
                    "Science medium lecture",
                    "1 credit extended bell schedule",
    );
    conflict!(t, set hard,
            clique: "BIOL 3010-01", "BIOL 3010-01-SI",
    );

    // BIOL 3010-02: Evolution
    // assigned to SET 301 at MWF1100+50
    section!(t, course: "BIOL 3010-02",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 3010-02-alt: Evolution
    // assigned to SET 301 at R1200+50
    section!(t, course: "BIOL 3010-02-SI",
                rooms and times:
                    "Science medium lecture",
                    "1 credit extended bell schedule",
    );
    conflict!(t, set hard,
            clique: "BIOL 3010-02", "BIOL 3010-02-SI",
    );

    // BIOL 3030-01: Principles of Genetics: Supplemental Instruction
    // assigned to SET 301 at MWF0900+50
    section!(t, course: "BIOL 3030-01",
                instructor: "Randy Klabacka",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 3030-01-alt: Principles of Genetics: Supplemental Instruction
    // assigned to SET 301 at T0900+50
    section!(t, course: "BIOL 3030-01-SI",
                //instructor: "Randy Klabacka",
                rooms and times:
                    "Science medium lecture",
                    "1 credit bell schedule",
    );
    conflict!(t, set hard,
            clique: "BIOL 3030-01", "BIOL 3030-01-SI",
    );

    // BIOL 3030-02: Genetics
    // assigned to SET 301 at MWF0900+50
    section!(t, course: "BIOL 3030-02",
                instructor: "Randy Klabacka",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 3030-02-alt: Genetics
    // assigned to SET 301 at R0900+50
    section!(t, course: "BIOL 3030-02-SI",
                //instructor: "Randy Klabacka",
                rooms and times:
                    "Science medium lecture",
                    "1 credit bell schedule",
    );
    conflict!(t, set hard,
            clique: "BIOL 3030-02", "BIOL 3030-02-SI",
    );

    // BIOL 3040-01: General Ecology
    // assigned to SET 301 at MWF1000+50
    section!(t, course: "BIOL 3040-01",
                instructor: "Marius Van der Merwe",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 3045-01: General Ecology Lab
    // assigned to SET 216 at T1200+170
    section!(t, course: "BIOL 3045-01",
                instructor: "Marius Van der Merwe",
                rooms and times:
                    "SET 216",
                    "T1200+170",
    );

    // BIOL 3100-01: Bioethics
    // xlist entry: SC0B
    // assigned to HCC 476 at MWF1100+50
    section!(t, course: "BIOL 3100-01",
                instructor: "John E Wolfe",
                rooms and times:
                    "HCC 476",
                    "3 credit bell schedule",
    );

    // BIOL 3110-01: Scientific Writing
    // assigned to SET 408 at R0900+75
    section!(t, course: "BIOL 3110-01",
                instructor: "Jennifer L Ciaccio",
                rooms and times:
                    "SET 408",
                    "R0900+75",
    );

    // BIOL 3150-01: Biostatistics & the Sci Method
    // assigned to SET 106 at MW1330+75
    section!(t, course: "BIOL 3150-01",
                instructor: "Megen E Kepas",
                rooms and times:
                    "Science large lecture",
                    "3 credit bell schedule",
    );

    // BIOL 3155-01: Scientific Method and Experimental Design
    // assigned to SET 216 at R1200+135
    section!(t, course: "BIOL 3155-01",
                instructor: "Megen E Kepas",
                rooms and times:
                    "SET 216",
                    "R1200+135",
    );

    // BIOL 3155-02: Scientific Method and Experimental Design
    // assigned to SET 216 at T1500+170
    section!(t, course: "BIOL 3155-02",
                instructor: "Erin E O'Brien",
                rooms and times:
                    "SET 216",
                    "T1500+170",
    );

    // BIOL 3230R-01: Cadaver Practicum
    // assigned to SET 213 at F1330+170
    section!(t, course: "BIOL 3230R-01",
                instructor: "Scott B Griffin",
                rooms and times:
                    "SET 213",
                    "F1330+170",
    );

    // BIOL 3230R-02: Cadaver Practicum
    // assigned to SET 215 at F1330+170
    section!(t, course: "BIOL 3230R-02",
                instructor: "Kerby Robinson",
                rooms and times:
                    "SET 215",
                    "F1330+170",
    );

    // BIOL 3250-01: Cancer Biology
    // assigned to SET 319 at MW1330+75
    section!(t, course: "BIOL 3250-01",
                instructor: "Martina Gaspari",
                rooms and times:
                    "SET 319",
                    "3 credit bell schedule",
    );

    // BIOL 3300-01: Introduction to Bioinformatics
    // assigned to SET 501 at TR1500+75
    section!(t, course: "BIOL 3300-01",
                instructor: "Jesse William Breinholt",
                rooms and times:
                    "SET 501",
                    "3 credit bell schedule",
    );

    // BIOL 3420-01: Advanced Human Physiology
    // assigned to SNOW 128 at TR0900+75
    section!(t, course: "BIOL 3420-01",
                instructor: "Glorimar L Aponte-Kline",
                rooms and times:
                    "Science small lecture",
                    "3 credit bell schedule",
    );

    // BIOL 3450-01: General Microbiology
    // assigned to SET 524 at MWF1100+50
    section!(t, course: "BIOL 3450-01",
                instructor: "Jeremy W Bakelar",
                rooms and times:
                    "Science small lecture",
                    "3 credit bell schedule",
    );

    // BIOL 3455-01: General Microbiology Lab
    // assigned to SET 304 at T0900+170
    section!(t, course: "BIOL 3455-01",
                instructor: "Jeremy W Bakelar",
                rooms and times:
                    "SET 304",
                    "T0900+170",
    );

    // BIOL 3455-02: General Microbiology Lab
    // assigned to SET 304 at T1500+170
    section!(t, course: "BIOL 3455-02",
                instructor: "Jeremy W Bakelar",
                rooms and times:
                    "SET 304",
                    "T1500+170",
    );

    // BIOL 3460-01: Biology of Infectious Disease
    // assigned to SET 201 at MW1500+75
    section!(t, course: "BIOL 3460-01",
                instructor: "Donald H Warner",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // BIOL 4040-01: Medical Ecology
    // assigned to SET 501 at W0900+50
    section!(t, course: "BIOL 4040-01",
                instructor: "Marius Van der Merwe",
                rooms and times:
                    "SET 501",
                    "1 credit bell schedule",
    );

    // BIOL 4200-01: Plant Taxonomy (ALPP)
    // assigned to SNOW 208 at TR1500+50
    section!(t, course: "BIOL 4200-01",
                instructor: "Del William Smith",
                rooms and times:
                    "Science small lecture",
                    "2 credit lecture",
    );

    // BIOL 4205-01: Plant Taxonomy Lab (ALPP)
    // assigned to SNOW 208 at TR1600+170
    section!(t, course: "BIOL 4205-01",
                instructor: "Del William Smith",
                rooms and times:
                    "Science small lecture",
                    "TR1600+170",
    );

    // BIOL 4280-01: Marine Biology
    // assigned to SET 318 at MWF0900+50
    section!(t, course: "BIOL 4280-01",
                instructor: "Jennifer L Ciaccio",
                rooms and times:
                    "SET 318",
                    "3 credit bell schedule",
    );

    // BIOL 4300-01: Molecular Biology
    // assigned to SET 216 at MWF0900+50
    section!(t, course: "BIOL 4300-01",
                instructor: "Martina Gaspari",
                rooms and times:
                    "SET 216",
                    "3 credit bell schedule",
    );

    // BIOL 4305-01: Molecular Biology Laboratory
    // assigned to SET 308 at R0800+170
    section!(t, course: "BIOL 4305-01",
                instructor: "Martina Gaspari",
                rooms and times:
                    "SET 308",
                    "R0800+170",
    );

    // BIOL 4310-01: Advanced Bioinformatics
    // assigned to SET 501 at TR1330+75
    section!(t, course: "BIOL 4310-01",
                instructor: "Randy Klabacka",
                rooms and times:
                    "SET 501",
                    "3 credit bell schedule",
    );

    // BIOL 4350-01: Animal Behavior
    // assigned to SET 319 at TR1200+75
    section!(t, course: "BIOL 4350-01",
                instructor: "Curtis B Walker",
                rooms and times:
                    "SET 319",
                    "3 credit bell schedule",
    );

    // BIOL 4355-01: Animal Behavior Lab
    // assigned to SET 319 at T1400+170
    section!(t, course: "BIOL 4355-01",
                instructor: "Curtis B Walker",
                rooms and times:
                    "SET 319",
                    "T1400+170",
    );

    // BIOL 4440-01: General Entomology
    // assigned to SNOW 208 at TR1030+75
    section!(t, course: "BIOL 4440-01",
                instructor: "Bryan K Stevens",
                rooms and times:
                    "Science small lecture",
                    "3 credit bell schedule",
    );

    // BIOL 4600-01: Plant Physiology
    // assigned to SET 216 at MW1200+75
    section!(t, course: "BIOL 4600-01",
                instructor: "Erin E O'Brien",
                rooms and times:
                    "SET 216",
                    "3 credit bell schedule",
    );

    // BIOL 4605-01: Plant Physiology Lab
    // assigned to SET 216 at W1500+170
    section!(t, course: "BIOL 4605-01",
                instructor: "Erin E O'Brien",
                rooms and times:
                    "SET 216",
                    "W1500+170",
    );

    // BIOL 4810R-01B: Independent Research
    // assigned to SET 303 at M1400+180
    section!(t, course: "BIOL 4810R-01B",
                rooms and times:
                    "SET 303",
                    "M1400+180",
    );

    // BIOL 4890R-50: Life Science Internship
    // assigned to SET 501 at W1715+110
    section!(t, course: "BIOL 4890R-50",
                rooms and times:
                    "SET 501",
                    "W1715+110",
    );

    // BIOL 4890R-51: Life Science Internship
    // assigned to SET 501 at R1715+110
    section!(t, course: "BIOL 4890R-51",
                rooms and times:
                    "SET 501",
                    "R1715+110",
    );

    // BIOL 4910-01: Senior Seminar
    // assigned to SET 501 at M0800+50
    section!(t, course: "BIOL 4910-01",
                rooms and times:
                    "SET 501",
                    "1 credit bell schedule",
    );

    // BIOL 4910-02: Senior Seminar
    // assigned to SET 501 at R1100+50
    section!(t, course: "BIOL 4910-02",
                rooms and times:
                    "SET 501",
                    "1 credit bell schedule",
    );

    // BIOL 4910-03: Senior Seminar
    // assigned to SET 501 at T1030+50
    section!(t, course: "BIOL 4910-03",
                rooms and times:
                    "SET 501",
                    "1 credit extended bell schedule",
    );

    // BIOL 4990R-02: Seminar in Biology: Dental
    // assigned to SET 303 at R1600+170
    section!(t, course: "BIOL 4990R-02",
                instructor: "Scott E Bulloch",
                rooms and times:
                    "SET 303",
                    "R1600+170",
    );

    // BIOL 4990R-50: Seminar in Biology
    // assigned to SET 216 at W1800+50
    section!(t, course: "BIOL 4990R-50",
                rooms and times:
                    "SET 216",
                    "1 credit evening",
    );

    // BTEC 1010-01: Fundamentals of Biotechnology
    // assigned to SET 310 at TR1200+75
    section!(t, course: "BTEC 1010-01",
                instructor: "Douglas J Sainsbury",
                rooms and times:
                    "SET 310",
                    "3 credit bell schedule",
    );

    // BTEC 2020-01: Protein Purification and Analysis
    // assigned to SET 304 at TR1300+110
    section!(t, course: "BTEC 2020-01",
                instructor: "Jeremy W Bakelar",
                rooms and times:
                    "SET 304",
                    "4 hour lab",
    );

    // BTEC 2030-01: Cell Culture Techniques
    // assigned to SET 308 at MR1100+110
    section!(t, course: "BTEC 2030-01",
                instructor: "Martina Gaspari",
                rooms and times:
                    "SET 308",
                    "MR1100+110",
    );

    // BTEC 2050-01: Zebrafish Maintenance & Method
    // assigned to SET 303 at T1300+110
    section!(t, course: "BTEC 2050-01",
                instructor: "Hung Yu Shih",
                rooms and times:
                    "SET 303",
                    "2 hour lab",
    );

    // BTEC 2050-01-alt: Zebrafish Maintenance & Method
    // assigned to SET 303 at T1500+50
    section!(t, course: "BTEC 2050-01-lab",
                instructor: "Hung Yu Shih",
                rooms and times:
                    "SET 303",
                    "1 credit extended bell schedule",
    );

    // BTEC 2050-02: Zebrafish Maintenance & Method
    // assigned to SET 303 at T1300+110
    section!(t, course: "BTEC 2050-02",
                instructor: "Hung Yu Shih",
                rooms and times:
                    "SET 303",
                    "2 hour lab",
    );

    // BTEC 2050-02-alt: Zebrafish Maintenance & Method
    // assigned to SET 303 at T1600+50
    section!(t, course: "BTEC 2050-02-lab",
                instructor: "Hung Yu Shih",
                rooms and times:
                    "SET 303",
                    "1 credit extended bell schedule",
    );

    // BTEC 3010-01: Sequencing Methods & Technique
    // assigned to SET 312 at MW1530+75
    section!(t, course: "BTEC 3010-01",
                rooms and times:
                    "SET 312",
                    "MW1530+75",
    );

    // BTEC 4050-01A: In Situ Hybridization
    // assigned to SET 303 at W1330+170
    section!(t, course: "BTEC 4050-01A",
                instructor: "Hung Yu Shih",
                rooms and times:
                    "SET 303",
                    "W1330+170",
    );

    // CHEM 1010-01: Introduction to Chemistry (PS)
    // assigned to SNOW 113 at TR1030+75
    section!(t, course: "CHEM 1010-01",
                instructor: "Sarah Morgan Black",
                rooms and times:
                    "SNOW 113",
                    "3 credit bell schedule",
    );

    // CHEM 1010-02: Introduction to Chemistry (PS)
    // assigned to SNOW 113 at TR1330+75
    section!(t, course: "CHEM 1010-02",
                instructor: "Sarah Morgan Black",
                rooms and times:
                    "SNOW 113",
                    "3 credit bell schedule",
    );

    // CHEM 1015-01: Introduction to Chemistry Lab (LAB)
    // assigned to SET 405 at M0900+110
    section!(t, course: "CHEM 1015-01",
                rooms and times:
                    "SET 405",
                    "2 hour lab",
    );

    // CHEM 1015-02: Introduction to Chemistry Lab (LAB)
    // assigned to SET 405 at M1100+110
    section!(t, course: "CHEM 1015-02",
                rooms and times:
                    "SET 405",
                    "2 hour lab",
    );

    // CHEM 1015-03: Introduction to Chemistry Lab (LAB)
    // assigned to SET 405 at M1300+110
    section!(t, course: "CHEM 1015-03",
                rooms and times:
                    "SET 405",
                    "2 hour lab",
    );

    // CHEM 1120-01: Elem Organic / Bio Chemistry
    // assigned to SNOW 216 at MTWR0900+50
    section!(t, course: "CHEM 1120-01",
                instructor: "Jared M Hancock",
                rooms and times:
                    "Science small lecture",
                    "4 credit bell schedule",
    );

    // CHEM 1125-01: Elem Organic/Bio Chemistry Lab
    // assigned to SET 404 at M1100+110
    section!(t, course: "CHEM 1125-01",
                instructor: "Jared M Hancock",
                rooms and times:
                    "SET 404",
                    "2 hour lab",
    );

    // CHEM 1125-02: Elem Organic/Bio Chemistry Lab
    // assigned to SET 404 at M1300+110
    section!(t, course: "CHEM 1125-02",
                rooms and times:
                    "SET 404",
                    "2 hour lab",
    );

    // CHEM 1150-01: Integrated Chemistry for Health Sciences (PS)
    // assigned to SET 201 at MTWR0800+50
    section!(t, course: "CHEM 1150-01",
                instructor: "Jared M Hancock",
                rooms and times:
                    "Science medium lecture",
                    "4 credit bell schedule",
    );

    // CHEM 1150-02: Integrated Chemistry for Health Sciences (PS)
    // assigned to SET 201 at MTWR1400+50
    section!(t, course: "CHEM 1150-02",
                instructor: "Jared M Hancock",
                rooms and times:
                    "Science medium lecture",
                    "4 credit 4×50 extended bell schedule",
    );

    // CHEM 1150-03: Integrated Chemistry for Health Sciences (PS)
    // assigned to SNOW 216 at MTWR1200+50
    section!(t, course: "CHEM 1150-03",
                rooms and times:
                    "Science small lecture",
                    "4 credit bell schedule",
    );

    // CHEM 1155-01: Integrated Chemistry for Health Sciences Laboratory (LAB)
    // assigned to SET 405 at T1000+170
    section!(t, course: "CHEM 1155-01",
                instructor: "Christina M Quinn",
                rooms and times:
                    "SET 405",
                    "T1000+170",
    );

    // CHEM 1155-02: Integrated Chemistry for Health Sciences Laboratory (LAB)
    // assigned to SET 407 at W1000+170
    section!(t, course: "CHEM 1155-02",
                instructor: "Jared M Hancock",
                rooms and times:
                    "SET 407",
                    "W1000+170",
    );

    // CHEM 1155-03: Integrated Chemistry for Health Sciences Laboratory (LAB)
    // assigned to SET 407 at W1300+170
    section!(t, course: "CHEM 1155-03",
                instructor: "Christina M Quinn",
                rooms and times:
                    "SET 407",
                    "W1300+170",
    );

    // CHEM 1155-05: Integrated Chemistry for Health Sciences Laboratory (LAB)
    // assigned to SET 405 at T1600+170
    section!(t, course: "CHEM 1155-05",
                instructor: "Paul H Shirley",
                rooms and times:
                    "SET 405",
                    "T1600+170",
    );

    // CHEM 1155-06: Integrated Chemistry for Health Sciences Laboratory (LAB)
    // assigned to SET 405 at W0900+170
    section!(t, course: "CHEM 1155-06",
                instructor: "Teisha Richan",
                rooms and times:
                    "SET 405",
                    "W0900+170",
    );

    // CHEM 1155-50: Integrated Chemistry for Health Sciences Laboratory (LAB)
    // assigned to SET 405 at T1900+170
    section!(t, course: "CHEM 1155-50",
                instructor: "Paul H Shirley",
                rooms and times:
                    "SET 405",
                    "T1900+170",
    );

    // CHEM 1210-01: Principles of Chemistry I (PS)
    // assigned to SET 201 at MTWR0900+50
    section!(t, course: "CHEM 1210-01",
                instructor: "Diana L Reese",
                rooms and times:
                    "Science medium lecture",
                    "4 credit bell schedule",
    );

    // CHEM 1210-02: Principles of Chemistry I (PS)
    // assigned to SET 201 at MTWR1000+50
    section!(t, course: "CHEM 1210-02",
                instructor: "Diana L Reese",
                rooms and times:
                    "Science medium lecture",
                    "4 credit bell schedule",
    );

    // CHEM 1210-03: Principles of Chemistry I (PS)
    // assigned to SNOW 216 at MTWR1300+50
    section!(t, course: "CHEM 1210-03",
                rooms and times:
                    "Science small lecture",
                    "4 credit 4×50 extended bell schedule",
    );

    // CHEM 1215-01: Principles of Chemistry I Lab (LAB)
    // assigned to SET 407 at T0700+170
    section!(t, course: "CHEM 1215-01",
                instructor: "Christina M Quinn",
                rooms and times:
                    "SET 407",
                    "T0700+170",
    );

    // CHEM 1215-02: Principles of Chemistry I Lab (LAB)
    // assigned to SET 409 at R1000+170
    section!(t, course: "CHEM 1215-02",
                instructor: "Christina M Quinn",
                rooms and times:
                    "SET 409",
                    "R1000+170",
    );

    // CHEM 1215-03: Principles of Chemistry I Lab (LAB)
    // assigned to SET 407 at R1000+170
    section!(t, course: "CHEM 1215-03",
                rooms and times:
                    "SET 407",
                    "R1000+170",
    );

    // CHEM 1215-04: Principles of Chemistry I Lab (LAB)
    // assigned to SET 409 at R1300+170
    section!(t, course: "CHEM 1215-04",
                instructor: "Christina M Quinn",
                rooms and times:
                    "SET 409",
                    "R1300+170",
    );

    // CHEM 1215-05: Principles of Chemistry I Lab (LAB)
    // assigned to SET 407 at R1600+170
    section!(t, course: "CHEM 1215-05",
                instructor: "Megan R Liljenquist",
                rooms and times:
                    "SET 407",
                    "R1600+170",
    );

    // CHEM 1215-06: Principles of Chemistry I Lab (LAB)
    // assigned to SET 409 at R1600+170
    section!(t, course: "CHEM 1215-06",
                instructor: "Jacson Parker",
                rooms and times:
                    "SET 409",
                    "R1600+170",
    );

    // CHEM 1215-50: Principles of Chemistry I Lab (LAB)
    // assigned to SET 409 at R1900+170
    section!(t, course: "CHEM 1215-50",
                instructor: "David J Burr",
                rooms and times:
                    "SET 409",
                    "R1900+170",
    );

    // CHEM 1220-01: Principles of Chemistry II
    // assigned to SET 420 at MTWR0800+50
    section!(t, course: "CHEM 1220-01",
                instructor: "Gabriela Chilom",
                rooms and times:
                    "Science small lecture",
                    "4 credit bell schedule",
    );

    // CHEM 1220-02: Principles of Chemistry II
    // assigned to SNOW 216 at MTWR1400+50
    section!(t, course: "CHEM 1220-02",
                instructor: "Gabriela Chilom",
                rooms and times:
                    "Science small lecture",
                    "4 credit 4×50 extended bell schedule",
    );

    // CHEM 1220-03: Principles of Chemistry II
    // assigned to SET 420 at MTWR1000+50
    section!(t, course: "CHEM 1220-03",
                instructor: "Wendy E Schatzberg",
                rooms and times:
                    "Science small lecture",
                    "4 credit bell schedule",
    );

    // CHEM 1225-01: Principles of Chemistry II Lab
    // assigned to SET 409 at T0700+170
    section!(t, course: "CHEM 1225-01",
                rooms and times:
                    "SET 409",
                    "T0700+170",
    );

    // CHEM 1225-02: Principles of Chemistry II Lab
    // assigned to SET 409 at T1000+170
    section!(t, course: "CHEM 1225-02",
                rooms and times:
                    "SET 409",
                    "T1000+170",
    );

    // CHEM 1225-03: Principles of Chemistry II Lab
    // assigned to SET 409 at T1300+170
    section!(t, course: "CHEM 1225-03",
                instructor: "Christina M Quinn",
                rooms and times:
                    "SET 409",
                    "T1300+170",
    );

    // CHEM 1225-04: Principles of Chemistry II Lab
    // assigned to SET 407 at T1600+170
    section!(t, course: "CHEM 1225-04",
                instructor: "David J Burr",
                rooms and times:
                    "SET 407",
                    "T1600+170",
    );

    // CHEM 1225-05: Principles of Chemistry II Lab
    // assigned to SET 409 at T1600+170
    section!(t, course: "CHEM 1225-05",
                instructor: "Jacson Parker",
                rooms and times:
                    "SET 409",
                    "T1600+170",
    );

    // CHEM 1225-50: Principles of Chemistry II Lab
    // assigned to SET 407 at T1900+170
    section!(t, course: "CHEM 1225-50",
                instructor: "David J Burr",
                rooms and times:
                    "SET 407",
                    "T1900+170",
    );

    // CHEM 2310-01: Organic Chemistry I
    // assigned to SET 420 at MTWRF0900+50
    section!(t, course: "CHEM 2310-01",
                instructor: "Rico Del Sesto",
                rooms and times:
                    "Science small lecture",
                    "5 credit bell schedule",
    );

    // CHEM 2310-02: Organic Chemistry I
    // assigned to SNOW 216 at MTWRF1100+50
    section!(t, course: "CHEM 2310-02",
                rooms and times:
                    "Science small lecture",
                    "5 credit bell schedule",
    );

    // CHEM 2315-01: Organic Chemistry I Lab
    // assigned to SET 404 at R1000+170
    section!(t, course: "CHEM 2315-01",
                instructor: "Teisha Richan",
                rooms and times:
                    "SET 404",
                    "R1000+170",
    );

    // CHEM 2315-02: Organic Chemistry I Lab
    // assigned to SET 404 at R1300+170
    section!(t, course: "CHEM 2315-02",
                instructor: "Teisha Richan",
                rooms and times:
                    "SET 404",
                    "R1300+170",
    );

    // CHEM 2320-01: Organic Chemistry II
    // assigned to SET 201 at MTWRF1100+50
    section!(t, course: "CHEM 2320-01",
                instructor: "Rico Del Sesto",
                rooms and times:
                    "Science medium lecture",
                    "5 credit bell schedule",
    );

    // CHEM 2320-02: Organic Chemistry II
    // assigned to SET 420 at MTWRF1200+50
    section!(t, course: "CHEM 2320-02",
                instructor: "Diana L Reese",
                rooms and times:
                    "Science small lecture",
                    "5 credit bell schedule",
    );

    // CHEM 2325-01: Organic Chemistry II Lab
    // assigned to SET 404 at T0900+170
    section!(t, course: "CHEM 2325-01",
                instructor: "Teisha Richan",
                rooms and times:
                    "SET 404",
                    "T0900+170",
    );

    // CHEM 2325-02: Organic Chemistry II Lab
    // assigned to SET 404 at T1200+170
    section!(t, course: "CHEM 2325-02",
                instructor: "Teisha Richan",
                rooms and times:
                    "SET 404",
                    "T1200+170",
    );

    // CHEM 2325-03: Organic Chemistry II Lab
    // assigned to SET 404 at T1500+170
    section!(t, course: "CHEM 2325-03",
                rooms and times:
                    "SET 404",
                    "T1500+170",
    );

    // CHEM 2325-04: Organic Chemistry II Lab
    // assigned to SET 404 at W0900+170
    section!(t, course: "CHEM 2325-04",
                rooms and times:
                    "SET 404",
                    "W0900+170",
    );

    // CHEM 2325-05: Organic Chemistry II Lab
    // assigned to SET 404 at W1200+170
    section!(t, course: "CHEM 2325-05",
                instructor: "Teisha Richan",
                rooms and times:
                    "SET 404",
                    "W1200+170",
    );

    // CHEM 2325-06: Organic Chemistry II Lab
    // assigned to SET 404 at W1500+170
    section!(t, course: "CHEM 2325-06",
                instructor: "Megan R Liljenquist",
                rooms and times:
                    "SET 404",
                    "W1500+170",
    );

    // CHEM 2325-50: Organic Chemistry II Lab
    // assigned to SET 404 at T1800+170
    section!(t, course: "CHEM 2325-50",
                rooms and times:
                    "SET 404",
                    "T1800+170",
    );

    // CHEM 3070-01: Physical Chemistry II
    // assigned to SET 420 at MTWR1100+50
    section!(t, course: "CHEM 3070-01",
                instructor: "Wendy E Schatzberg",
                rooms and times:
                    "Science small lecture",
                    "4 credit bell schedule",
    );

    // CHEM 3075-01: Physical Chemistry II Lab
    // assigned to SNOW 103 at T1600+170
    section!(t, course: "CHEM 3075-01",
                instructor: "Wendy E Schatzberg",
                rooms and times:
                    "SNOW 103",
                    "T1600+170",
    );

    // CHEM 3300-01: Instrumental Analysis
    // assigned to SNOW 216 at MWF1000+50
    section!(t, course: "CHEM 3300-01",
                instructor: "Gabriela Chilom",
                rooms and times:
                    "Science small lecture",
                    "3 credit bell schedule",
    );

    // CHEM 3300-01-alt: Instrumental Analysis
    // assigned to SNOW 103 at R1500+170
    section!(t, course: "CHEM 3300-01-alt",
                instructor: "Gabriela Chilom",
                rooms and times:
                    "SNOW 103",
                    "R1500+170",
    );

    // CHEM 3510-01: Biochemistry I
    // assigned to SET 420 at MW1330+75
    section!(t, course: "CHEM 3510-01",
                instructor: "Jennifer A Meyer",
                rooms and times:
                    "Science small lecture",
                    "3 credit bell schedule",
    );

    // CHEM 3515-01: Biochemistry I Lab
    // assigned to SET 308 at R1300+170
    section!(t, course: "CHEM 3515-01",
                instructor: "Jennifer A Meyer",
                rooms and times:
                    "SET 308",
                    "R1300+170",
    );

    // CHEM 3515-02: Biochemistry I Lab
    // assigned to SET 308 at R1600+170
    section!(t, course: "CHEM 3515-02",
                instructor: "Cutler Cowdin",
                rooms and times:
                    "SET 308",
                    "R1600+170",
    );

    // CHEM 3520-01: Biochemistry II
    // assigned to SET 201 at MW1200+75
    section!(t, course: "CHEM 3520-01",
                instructor: "Jennifer A Meyer",
                rooms and times:
                    "Science medium lecture",
                    "3 credit bell schedule",
    );

    // CHEM 3525-01: Biochemistry II Lab
    // assigned to SET 308 at T1000+170
    section!(t, course: "CHEM 3525-01",
                rooms and times:
                    "SET 308",
                    "T1000+170",
    );

    // CHEM 3525-02: Biochemistry II Lab
    // assigned to SET 308 at T1300+170
    section!(t, course: "CHEM 3525-02",
                instructor: "Jennifer A Meyer",
                rooms and times:
                    "SET 308",
                    "T1300+170",
    );

    // CHEM 3525-03: Biochemistry II Lab
    // assigned to SET 308 at T1600+170
    section!(t, course: "CHEM 3525-03",
                instructor: "Cutler Cowdin",
                rooms and times:
                    "SET 308",
                    "T1600+170",
    );

    // CHEM 4800R-01: Independent Research
    // assigned to SNOW 204 at MTWRF1000+50
    section!(t, course: "CHEM 4800R-01",
                instructor: "Rico Del Sesto",
                rooms and times:
                    "SNOW 204",
                    "5 credit bell schedule",
    );

    // CHEM 4800R-02: Independent Research
    // assigned to SNOW 204 at MTWRF1200+50
    section!(t, course: "CHEM 4800R-02",
                instructor: "Wendy E Schatzberg",
                rooms and times:
                    "SNOW 204",
                    "5 credit bell schedule",
    );

    // CHEM 4800R-03: Independent Research
    // assigned to SNOW 204 at MTWRF1100+50
    section!(t, course: "CHEM 4800R-03",
                rooms and times:
                    "SNOW 204",
                    "5 credit bell schedule",
    );

    // CHEM 4800R-04: Independent Research
    // assigned to SNOW 204 at MTWRF1500+50
    section!(t, course: "CHEM 4800R-04",
                instructor: "Gabriela Chilom",
                rooms and times:
                    "SNOW 204",
                    "5 credit extended bell schedule",
    );

    // CHEM 4800R-06: Independent Research
    // assigned to SNOW 204 at MTWRF1600+50
    section!(t, course: "CHEM 4800R-06",
                instructor: "Diana L Reese",
                rooms and times:
                    "SNOW 204",
                    "5 credit extended bell schedule",
    );

    // CHEM 4910-01: Chemistry Senior Seminar
    // assigned to SET 201 at F1200+50
    section!(t, course: "CHEM 4910-01",
                instructor: "Wendy E Schatzberg",
                rooms and times:
                    "Science medium lecture",
                    "1 credit extended bell schedule",
    );

    // ECE 2100-01: Semiconductor Devices
    // assigned to SET 102 at MW1200+75
    section!(t, course: "ECE 2100-01",
                instructor: "Andrew Gregory Toth",
                rooms and times:
                    "SET 102",
                    "3 credit bell schedule",
    );

    // ECE 2280-01: Microelectronics
    // assigned to SET 102 at MWF1100+50
    section!(t, course: "ECE 2280-01",
                instructor: "Sai C Radavaram",
                rooms and times:
                    "SET 102",
                    "3 credit bell schedule",
    );

    // ECE 2285-01: Microelectronics Lab
    // assigned to SET 101 at T0800+110
    section!(t, course: "ECE 2285-01",
                instructor: "Sai C Radavaram",
                rooms and times:
                    "SET 101",
                    "2 hour lab",
    );

    // ECE 3500-01: Signals and Systems
    // assigned to SET 523 at MW1500+75
    section!(t, course: "ECE 3500-01",
                instructor: "Kameron J Eves",
                rooms and times:
                    "SET 523",
                    "3 credit bell schedule",
    );

    // ECE 3600-01: Power Electronics
    // assigned to SET 523 at MW1330+75
    section!(t, course: "ECE 3600-01",
                instructor: "Sai C Radavaram",
                rooms and times:
                    "SET 523",
                    "3 credit bell schedule",
    );

    // ECE 3605-01: Power Electronics Lab
    // assigned to SET 101 at T1200+110
    section!(t, course: "ECE 3605-01",
                instructor: "David Brent Christensen",
                rooms and times:
                    "SET 101",
                    "2 hour lab",
    );

    // ECE 4010-01: EE Product Design II
    // assigned to SET 219 at MWF1330+180
    section!(t, course: "ECE 4010-01",
                instructor: "Brant A Ross",
                rooms and times:
                    "SET 219",
                    "MWF1330+180",
    );

    // ECE 4510-01: Image Processing
    // assigned to SET 523 at TR0900+75
    section!(t, course: "ECE 4510-01",
                instructor: "Jeffrey Anderson",
                rooms and times:
                    "SET 523",
                    "3 credit bell schedule",
    );

    // ECE 4730-01: Embedded Systems II
    // assigned to SET 523 at MW1630+75
    section!(t, course: "ECE 4730-01",
                instructor: "Jeffrey Anderson",
                rooms and times:
                    "SET 523",
                    "3 credit bell schedule",
    );

    // ECE 4735-01: Embedded Systems II Lab
    // assigned to SET 101 at T1400+110
    section!(t, course: "ECE 4735-01",
                instructor: "Jeffrey Anderson",
                rooms and times:
                    "SET 101",
                    "2 hour lab",
    );

    // ECE 4990-01: Special Topics: Human-Machine Interfacing
    // assigned to SET 101 at F1000+110
    section!(t, course: "ECE 4990-01-lab",
                instructor: "Bing Jiang",
                rooms and times:
                    "SET 101",
                    "2 hour lab",
    );

    // ECE 4990-01-alt: Special Topics: Human-Machine Interfacing
    // assigned to SET 523 at MW1200+75
    section!(t, course: "ECE 4990-01",
                instructor: "Bing Jiang",
                rooms and times:
                    "SET 523",
                    "3 credit bell schedule",
    );

    // ECE 4990-02: Special Topics: Autopilot
    // assigned to SET 523 at TR1030+75
    section!(t, course: "ECE 4990-02",
                instructor: "Kameron J Eves",
                rooms and times:
                    "SET 523",
                    "3 credit bell schedule",
    );

    // ECE 4990-03: Special Topics: Antenna Engineering
    // assigned to SET 101 at F0800+115
    section!(t, course: "ECE 4990-03-lab",
                instructor: "Sai C Radavaram",
                rooms and times:
                    "SET 101",
                    "F0800+115",
    );

    // ECE 4990-03-alt: Special Topics: Antenna Engineering
    // assigned to SET 523 at TR1630+75
    section!(t, course: "ECE 4990-03",
                instructor: "Sai C Radavaram",
                rooms and times:
                    "SET 523",
                    "3 credit bell schedule",
    );

    // ENVS 1010-01: Intro to Environmental Science (PS)
    // assigned to SET 524 at TR1200+75
    section!(t, course: "ENVS 1010-01",
                rooms and times:
                    "Science small lecture",
                    "3 credit bell schedule",
    );

    // ENVS 1010-03: Intro to Environmental Science (PS)
    // assigned to SET 524 at TR1330+75
    section!(t, course: "ENVS 1010-03",
                rooms and times:
                    "Science small lecture",
                    "3 credit bell schedule",
    );

    // ENVS 1010-04: Intro to Environmental Science (PS)
    // assigned to SET 524 at MW1330+75
    section!(t, course: "ENVS 1010-04",
                instructor: "Greg L Melton",
                rooms and times:
                    "Science small lecture",
                    "3 credit bell schedule",
    );

    // ENVS 1010-05: Intro to Environmental Science (PS)
    // assigned to SNOW 113 at TR1500+75
    section!(t, course: "ENVS 1010-05",
                rooms and times:
                    "SNOW 113",
                    "3 credit bell schedule",
    );

    // ENVS 1010-06: Intro to Environmental Science (PS)
    // assigned to SNOW 113 at MW1330+75
    section!(t, course: "ENVS 1010-06",
                instructor: "Marshall Topham",
                rooms and times:
                    "SNOW 113",
                    "3 credit bell schedule",
    );

    // ENVS 1010-07: Intro to Environmental Science (PS)
    // assigned to SNOW 128 at TR1330+75
    section!(t, course: "ENVS 1010-07",
                rooms and times:
                    "Science small lecture",
                    "3 credit bell schedule",
    );

    // ENVS 1099-01: Recitation for Majors
    // assigned to SET 526 at F1000+50
    section!(t, course: "ENVS 1099-01",
                instructor: "Christina Pondell",
                rooms and times:
                    "SET 526",
                    "1 credit bell schedule",
    );

    // ENVS 1210-01: Introduction to Environmental Science
    // assigned to SNOW 113 at TR1200+75
    section!(t, course: "ENVS 1210-01",
                instructor: "Marzieh Ghasemi",
                rooms and times:
                    "SNOW 113",
                    "3 credit bell schedule",
    );

    // ENVS 1215-01: Introduction to Environmental Science Laboratory
    // assigned to SET 526 at M1300+170
    section!(t, course: "ENVS 1215-01",
                instructor: "Christina Pondell",
                rooms and times:
                    "SET 526",
                    "M1300+170",
    );

    // ENVS 1215-02: Introduction to Environmental Science Laboratory
    // assigned to SET 526 at R1330+165
    section!(t, course: "ENVS 1215-02",
                instructor: "Christina Pondell",
                rooms and times:
                    "SET 526",
                    "R1330+165",
    );

    // ENVS 2099R-50: Special Topics in Environmental Science: The Geology of Foundation Engineering in Southern Utah
    // assigned to SET 526 at TR1800+75
    section!(t, course: "ENVS 2099R-50",
                instructor: "Hugo Elio Angeles",
                rooms and times:
                    "SET 526",
                    "TR1800+75",
    );

    // ENVS 2210-01: Environmental Pollution and Remediation Techniques
    // assigned to SNOW 128 at MW1200+75
    section!(t, course: "ENVS 2210-01",
                instructor: "Marzieh Ghasemi",
                rooms and times:
                    "Science small lecture",
                    "3 credit bell schedule",
    );

    // ENVS 2700R-01: Field Methods EnvSci
    // assigned to SET 527 at F1400+170
    section!(t, course: "ENVS 2700R-01",
                instructor: "Alexander R Tye",
                rooms and times:
                    "SET 527",
                    "F1400+170",
    );

    // ENVS 3110-01: Scientific Writing
    // assigned to SET 408 at MWF1100+50
    section!(t, course: "ENVS 3110-01",
                instructor: "Jerald D Harris",
                rooms and times:
                    "SET 408",
                    "3 credit bell schedule",
    );

    // ENVS 3210-01: Soil Science
    // assigned to SET 526 at TR0900+75
    section!(t, course: "ENVS 3210-01",
                instructor: "Christina Pondell",
                rooms and times:
                    "SET 526",
                    "3 credit bell schedule",
    );

    // ENVS 3280-50: Environmental Law
    // assigned to SNOW 128 at TR1800+110
    section!(t, course: "ENVS 3280-50",
                rooms and times:
                    "Science small lecture",
                    "4 hour lab",
    );

    // ENVS 3410-01: Air Quality and Control
    // assigned to SET 522 at MWF1000+50
    section!(t, course: "ENVS 3410-01",
                instructor: "Marzieh Ghasemi",
                rooms and times:
                    "SET 522",
                    "3 credit bell schedule",
    );

    // ENVS 3920-50: Peruvian Amazon Natural Histor
    // assigned to SNOW 113 at W1800+50
    section!(t, course: "ENVS 3920-50",
                instructor: "Marius Van der Merwe",
                rooms and times:
                    "SNOW 113",
                    "1 credit evening",
    );

    // ENVS 4910-01: Senior Seminar
    // assigned to SET 408 at F1200+50
    section!(t, course: "ENVS 4910-01",
                rooms and times:
                    "SET 408",
                    "1 credit extended bell schedule",
    );

    // GEO 1010-01: Introduction to Geology (PS)
    // assigned to SET 524 at TR0900+75
    section!(t, course: "GEO 1010-01",
                instructor: "Greg L Melton",
                rooms and times:
                    "Science small lecture",
                    "3 credit bell schedule",
    );

    // GEO 1010-50: Introduction to Geology (PS)
    // assigned to SNOW 128 at MW1800+75
    section!(t, course: "GEO 1010-50",
                rooms and times:
                    "Science small lecture",
                    "MW1800+75",
    );

    // GEO 1015-01: Introduction to Geology Lab (LAB)
    // assigned to SET 527 at W0900+110
    section!(t, course: "GEO 1015-01",
                instructor: "Greg L Melton",
                rooms and times:
                    "SET 527",
                    "2 hour lab",
    );

    // GEO 1015-03: Introduction to Geology Lab (LAB)
    // assigned to SET 527 at T1100+110
    section!(t, course: "GEO 1015-03",
                rooms and times:
                    "SET 527",
                    "2 hour lab",
    );

    // GEO 1015-04: Introduction to Geology Lab (LAB)
    // assigned to SET 527 at T1500+110
    section!(t, course: "GEO 1015-04",
                rooms and times:
                    "SET 527",
                    "2 hour lab",
    );

    // GEO 1015-50: Introduction to Geology Lab (LAB)
    // assigned to SET 527 at T1700+110
    section!(t, course: "GEO 1015-50",
                instructor: "David R Black",
                rooms and times:
                    "SET 527",
                    "2 hour lab",
    );

    // GEO 1015-51: Introduction to Geology Lab (LAB)
    // assigned to SET 527 at T1900+110
    section!(t, course: "GEO 1015-51",
                rooms and times:
                    "SET 527",
                    "2 hour lab late evening",
    );

    // GEO 1050-01: Geology of the National Parks (PS)
    // assigned to SET 527 at W1100+110
    section!(t, course: "GEO 1050-01",
                rooms and times:
                    "SET 527",
                    "2 hour lab",
    );

    // GEO 1110-01: Physical Geology (PS)
    // assigned to SET 522 at TR0900+75
    section!(t, course: "GEO 1110-01",
                instructor: "Janice M Hayden",
                rooms and times:
                    "SET 522",
                    "3 credit bell schedule",
    );

    // GEO 1115-01: Physical Geology Lab
    // assigned to SET 522 at W1100+170
    section!(t, course: "GEO 1115-01",
                instructor: "Janice M Hayden",
                rooms and times:
                    "SET 522",
                    "W1100+170",
    );

    // GEO 1220-01: Historical Geology
    // assigned to SET 522 at TR1030+75
    section!(t, course: "GEO 1220-01",
                instructor: "Jerald D Harris",
                rooms and times:
                    "SET 522",
                    "3 credit bell schedule",
    );

    // GEO 1225-01: Historical Geology Lab
    // assigned to SET 522 at R1630+170
    section!(t, course: "GEO 1225-01",
                instructor: "Jerald D Harris",
                rooms and times:
                    "SET 522",
                    "R1630+170",
    );

    // GEO 2700R-01: Field Methods in Geoscience Research
    // assigned to SET 527 at F1400+170
    section!(t, course: "GEO 2700R-01",
                instructor: "Alexander R Tye",
                rooms and times:
                    "SET 527",
                    "F1400+170",
    );

    // GEO 3110-01: Scientific Writing
    // assigned to SET 408 at MWF1100+50
    section!(t, course: "GEO 3110-01",
                instructor: "Jerald D Harris",
                rooms and times:
                    "SET 408",
                    "3 credit bell schedule",
    );

    // GEO 3500-01: Geomorphology
    // assigned to SET 408 at R1200+170
    section!(t, course: "GEO 3500-01-lab",
                instructor: "Alexander R Tye",
                rooms and times:
                    "SET 408",
                    "R1200+170",
    );

    // GEO 3500-01-alt: Geomorphology
    // assigned to SET 408 at TR1500+75
    section!(t, course: "GEO 3500-01",
                instructor: "Alexander R Tye",
                rooms and times:
                    "SET 408",
                    "3 credit bell schedule",
    );

    // GEO 3600-01: Ig/Met Petrology
    // assigned to SET 522 at MW1500+75
    section!(t, course: "GEO 3600-01",
                instructor: "Greg L Melton",
                rooms and times:
                    "SET 522",
                    "3 credit bell schedule",
    );

    // GEO 3600-01-alt: Ig/Met Petrology
    // assigned to SET 522 at T1200+170
    section!(t, course: "GEO 3600-01-lab",
                instructor: "Greg L Melton",
                rooms and times:
                    "SET 522",
                    "T1200+170",
    );

    // GEO 3710-01: Hydrology
    // assigned to SET 524 at TR1500+75
    section!(t, course: "GEO 3710-01",
                instructor: "Marzieh Ghasemi",
                rooms and times:
                    "Science small lecture",
                    "3 credit bell schedule",
    );

    // GEO 4000R-01: Selected Geology Field Excursions
    // assigned to SET 527 at F1100+50
    section!(t, course: "GEO 4000R-01",
                rooms and times:
                    "SET 527",
                    "1 credit bell schedule",
    );

    // GEO 4910-01: Senior Seminar
    // assigned to SNOW 216 at F1200+50
    section!(t, course: "GEO 4910-01",
                rooms and times:
                    "Science small lecture",
                    "1 credit extended bell schedule",
    );

    // GEOG 1000-01: Physical Geography: Supplemental Instruction (PS)
    // assigned to SET 524 at MWF1000+50
    section!(t, course: "GEOG 1000-01",
                instructor: "Jerald D Harris",
                rooms and times:
                    "Science small lecture",
                    "3 credit bell schedule",
    );

    // GEOG 1000-01-alt: Physical Geography: Supplemental Instruction (PS)
    // assigned to SNOW 216 at R1000+50
    section!(t, course: "GEOG 1000-01-SI",
                //instructor: "Jerald D Harris",
                rooms and times:
                    "Science small lecture",
                    "1 credit bell schedule",
    );
    conflict!(t, set hard,
            clique: "GEOG 1000-01", "GEOG 1000-01-SI",
    );

    // GEOG 1000-02: Physical Geography (PS)
    // assigned to SET 524 at MW1200+75
    section!(t, course: "GEOG 1000-02",
                instructor: "Zhenyu Jin",
                rooms and times:
                    "Science small lecture",
                    "3 credit bell schedule",
    );

    // GEOG 1000-03: Physical Geography (PS)
    // assigned to SNOW 113 at TR0900+75
    section!(t, course: "GEOG 1000-03",
                rooms and times:
                    "SNOW 113",
                    "3 credit bell schedule",
    );

    // GEOG 1005-01: Physical Geography Lab (LAB)
    // assigned to SET 526 at T1100+110
    section!(t, course: "GEOG 1005-01",
                instructor: "Christina Pondell",
                rooms and times:
                    "SET 526",
                    "2 hour lab",
    );

    // GEOG 1005-02: Physical Geography Lab (LAB)
    // assigned to SET 526 at T1300+110
    section!(t, course: "GEOG 1005-02",
                instructor: "Christina Pondell",
                rooms and times:
                    "SET 526",
                    "2 hour lab",
    );

    // GEOG 1005-03: Physical Geography Lab (LAB)
    // assigned to SET 526 at W0900+110
    section!(t, course: "GEOG 1005-03",
                instructor: "Zhenyu Jin",
                rooms and times:
                    "SET 526",
                    "2 hour lab",
    );

    // GEOG 1005-04: Physical Geography Lab (LAB)
    // assigned to SET 526 at W1100+110
    section!(t, course: "GEOG 1005-04",
                rooms and times:
                    "SET 526",
                    "2 hour lab",
    );

    // GEOG 1005-05: Physical Geography Lab (LAB)
    // assigned to SET 526 at R1100+110
    section!(t, course: "GEOG 1005-05",
                rooms and times:
                    "SET 526",
                    "2 hour lab",
    );

    // GEOG 3600-01: Introduction to Geographic Information Systems
    // assigned to SET 408 at TR1030+75
    section!(t, course: "GEOG 3600-01",
                instructor: "Zhenyu Jin",
                rooms and times:
                    "SET 408",
                    "3 credit bell schedule",
    );

    // GEOG 3605-01: Introduction to Geographic Information Systems Laboratory
    // assigned to SET 408 at T1200+170
    section!(t, course: "GEOG 3605-01",
                instructor: "Zhenyu Jin",
                rooms and times:
                    "SET 408",
                    "T1200+170",
    );

    // GEOG 4180-01: Geoprocessing with Python
    // assigned to SET 408 at MW1330+75
    section!(t, course: "GEOG 4180-01",
                instructor: "Zhenyu Jin",
                rooms and times:
                    "SET 408",
                    "3 credit bell schedule",
    );

    // MATH 1010-03: Intermediate Algebra
    // assigned to SNOW 3 at MTWR1100+50
    section!(t, course: "MATH 1010-03",
                instructor: "Violeta Adina Ionita",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 1010-04: Intermediate Algebra
    // assigned to SNOW 145 at MW1300+100
    section!(t, course: "MATH 1010-04",
                instructor: "Elizabeth Karen Ludlow",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 1010-05: Intermediate Algebra
    // assigned to SNOW 145 at TR1500+100
    section!(t, course: "MATH 1010-05",
                instructor: "Odean Bowler",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 1010-06: Intermediate Algebra
    // assigned to SNOW 145 at MW1500+100
    section!(t, course: "MATH 1010-06",
                instructor: "Odean Bowler",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 1010-07: Intermediate Algebra
    // assigned to SNOW 3 at MTWR1200+50
    section!(t, course: "MATH 1010-07",
                instructor: "Violeta Adina Ionita",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 1010-50: Intermediate Algebra
    // assigned to SNOW 147 at TR1800+100
    section!(t, course: "MATH 1010-50",
                rooms and times:
                    "Math lecture",
                    "TR1800+100",
    );

    // MATH 1030-01: Quantitative Reasoning (MA)
    // assigned to SNOW 125 at MW1500+75
    section!(t, course: "MATH 1030-01",
                instructor: "Elizabeth Karen Ludlow",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1030-02: Quantitative Reasoning (MA)
    // assigned to SNOW 124 at TR0730+75
    section!(t, course: "MATH 1030-02",
                instructor: "Craig D Seegmiller",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1030-03: Quantitative Reasoning (MA)
    // assigned to SNOW 124 at TR0900+75
    section!(t, course: "MATH 1030-03",
                instructor: "Craig D Seegmiller",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1030-04: Quantitative Reasoning (MA)
    // assigned to SNOW 125 at MW1330+75
    section!(t, course: "MATH 1030-04",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1030-05: Quantitative Reasoning (MA)
    // assigned to SNOW 150 at TR1200+75
    section!(t, course: "MATH 1030-05",
                instructor: "Jeffrey P Harrah",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1030-06: Quantitative Reasoning (MA)
    // assigned to SNOW 150 at TR1330+75
    section!(t, course: "MATH 1030-06",
                instructor: "Jeffrey P Harrah",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1040-01: Introduction to Statistics (MA)
    // assigned to SNOW 124 at MWF0800+50
    section!(t, course: "MATH 1040-01",
                instructor: "James P Fitzgerald",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1040-02: Introduction to Statistics (MA)
    // assigned to SNOW 124 at MWF0900+50
    section!(t, course: "MATH 1040-02",
                instructor: "James P Fitzgerald",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1040-03: Introduction to Statistics (MA)
    // assigned to SNOW 124 at MWF1000+50
    section!(t, course: "MATH 1040-03",
                instructor: "James P Fitzgerald",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1040-04: Introduction to Statistics (MA)
    // assigned to SNOW 124 at MWF1200+50
    section!(t, course: "MATH 1040-04",
                rooms and times:
                    "Math lecture",
                    "MWF1200+50",
    );

    // MATH 1040-05: Introduction to Statistics (MA)
    // assigned to SNOW 124 at MWF1100+50
    section!(t, course: "MATH 1040-05",
                instructor: "Tye K Rogers",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1040-06: Introduction to Statistics (MA)
    // assigned to SNOW 125 at TR1330+75
    section!(t, course: "MATH 1040-06",
                instructor: "Tye K Rogers",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1040-07: Introduction to Statistics (MA)
    // assigned to SNOW 151 at TR1200+75
    section!(t, course: "MATH 1040-07",
                instructor: "Jameson C Hardy",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1040-08: Introduction to Statistics (MA)
    // assigned to SNOW 124 at MW1500+75
    section!(t, course: "MATH 1040-08",
                instructor: "Paula Manuele Temple",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1040-09: Introduction to Statistics (MA)
    // assigned to SNOW 150 at MW1200+75
    section!(t, course: "MATH 1040-09",
                instructor: "Jameson C Hardy",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1040-10: Introduction to Statistics (MA)
    // assigned to SNOW 124 at TR1200+75
    section!(t, course: "MATH 1040-10",
                instructor: "Jie Liu",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1040-11: Introduction to Statistics (MA)
    // assigned to SNOW 124 at TR1630+75
    section!(t, course: "MATH 1040-11",
                instructor: "Ryan C McConnell",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1040-12: Introduction to Statistics (MA)
    // assigned to SNOW 125 at TR1630+75
    section!(t, course: "MATH 1040-12",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1040-14: Introduction to Statistics (MA)
    // assigned to SNOW 124 at MW1630+75
    section!(t, course: "MATH 1040-14",
                instructor: "Robert T Reimer",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1050-01: College Algebra / Pre-Calculus (MA)
    // assigned to SNOW 3 at MTWR0800+50
    section!(t, course: "MATH 1050-01",
                instructor: "Violeta Adina Ionita",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 1050-02: College Algebra / Pre-Calculus (MA)
    // assigned to SNOW 3 at MTWR0900+50
    section!(t, course: "MATH 1050-02",
                instructor: "Violeta Adina Ionita",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 1050-03: College Algebra / Pre-Calculus: Supplemental Instruction (MA)
    // assigned to SNOW 125 at F1100+50
    section!(t, course: "MATH 1050-03",
                //instructor: "Costel Ionita",
                rooms and times:
                    "Math lecture",
                    "F1100+50",
    );

    // MATH 1050-03-alt: College Algebra / Pre-Calculus: Supplemental Instruction (MA)
    // assigned to SNOW 125 at MTWR1100+50
    section!(t, course: "MATH 1050-03-alt",
                instructor: "Costel Ionita",
                rooms and times:
                    "Math lecture",
                    "MTWR1100+50",
    );

    // MATH 1050-04: College Algebra / Pre-Calculus (MA)
    // assigned to SNOW 147 at MTWR1200+50
    section!(t, course: "MATH 1050-04",
                instructor: "Clare C Banks",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 1050-05: College Algebra / Pre-Calculus (MA)
    // assigned to SNOW 145 at TR1300+100
    section!(t, course: "MATH 1050-05",
                instructor: "Dawn Lashell Kidd-Thomas",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 1050-06: College Algebra / Pre-Calculus (MA)
    // assigned to SNOW 112 at MTWR1200+50
    section!(t, course: "MATH 1050-06",
                instructor: "Craig D Seegmiller",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 1060-01: Trigonometry (MA)
    // assigned to SNOW 147 at TR0900+75
    section!(t, course: "MATH 1060-01",
                instructor: "Ross C Decker",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1060-02: Trigonometry (MA)
    // assigned to SNOW 147 at TR1030+75
    section!(t, course: "MATH 1060-02",
                instructor: "Ross C Decker",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1080-01: Pre-Calculus with Trigonometry (MA)
    // assigned to SNOW 145 at MTWRF1000+50
    section!(t, course: "MATH 1080-01",
                instructor: "Jameson C Hardy",
                rooms and times:
                    "Math lecture",
                    "5 credit bell schedule",
    );

    // MATH 1100-02: Business Calculus (MA)
    // assigned to SNOW 124 at MW1330+75
    section!(t, course: "MATH 1100-02",
                instructor: "Trevor K Johnson",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 1210-01: Calculus I (MA)
    // assigned to SNOW 145 at MTWR1200+50
    section!(t, course: "MATH 1210-01",
                instructor: "Trevor K Johnson",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 1210-02: Calculus I (MA)
    // assigned to SNOW 125 at MTWR0800+50
    section!(t, course: "MATH 1210-02",
                instructor: "Costel Ionita",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 1210-03: Calculus I (MA)
    // assigned to SNOW 145 at MTWR1100+50
    section!(t, course: "MATH 1210-03",
                instructor: "Bhuvaneswari Sambandham",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 1220-01: Calculus II (MA)
    // assigned to SNOW 147 at MTWR0800+50
    section!(t, course: "MATH 1220-01",
                instructor: "Clare C Banks",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 1220-02: Calculus II (MA)
    // assigned to SNOW 125 at MTWR0900+50
    section!(t, course: "MATH 1220-02",
                instructor: "Costel Ionita",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 2010-01: Math for Elementary Teachers I
    // assigned to SNOW 150 at T1630+150
    section!(t, course: "MATH 2010-01",
                instructor: "Jeffrey P Harrah",
                rooms and times:
                    "Math lecture",
                    "T1630+150",
    );

    // MATH 2020-01: Math for Elemen Teachers II
    // assigned to SNOW 150 at TR1030+75
    section!(t, course: "MATH 2020-01",
                instructor: "Jeffrey P Harrah",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 2020-02: Math for Elemen Teachers II
    // assigned to SNOW 150 at W1630+150
    section!(t, course: "MATH 2020-02",
                instructor: "Jeffrey P Harrah",
                rooms and times:
                    "Math lecture",
                    "W1630+150",
    );

    // MATH 2200-01: Discrete Mathematics
    // assigned to SNOW 112 at TR1030+75
    section!(t, course: "MATH 2200-01",
                instructor: "Steven McKay Sullivan",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 2210-01: Multivariable Calculus (MA)
    // assigned to SNOW 112 at MTWR0900+50
    section!(t, course: "MATH 2210-01",
                instructor: "Steven McKay Sullivan",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 2250-01: Differential Equations and Linear Algebra
    // assigned to SNOW 125 at MTWF1000+50
    section!(t, course: "MATH 2250-01",
                instructor: "Bhuvaneswari Sambandham",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 2270-01: Linear Algebra
    // assigned to SNOW 151 at TR0900+75
    section!(t, course: "MATH 2270-01",
                instructor: "Md Sazib Hasan",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 2280-01: Ordinary Differential Equations
    // assigned to SNOW 151 at MW1200+75
    section!(t, course: "MATH 2280-01",
                instructor: "Bhuvaneswari Sambandham",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 3050-01: Stochastic Modeling and Applications
    // assigned to SNOW 151 at TR1030+75
    section!(t, course: "MATH 3050-01",
                instructor: "Md Sazib Hasan",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 3200-01: Introduction to Analysis I
    // assigned to SNOW 125 at TR1200+75
    section!(t, course: "MATH 3200-01",
                instructor: "Costel Ionita",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 3450-01: Statistical Inference
    // assigned to SNOW 124 at TR1030+75
    section!(t, course: "MATH 3450-01",
                instructor: "Jie Liu",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 3900-01: Number Theory
    // assigned to SNOW 112 at MWF1000+50
    section!(t, course: "MATH 3900-01",
                instructor: "Steven McKay Sullivan",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 4250-01: Programming for Scientific Computation
    // assigned to SNOW 147 at MW1500+100
    section!(t, course: "MATH 4250-01",
                instructor: "Vinodh Kumar Chellamuthu",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 4400-01: Financial Mathematics
    // assigned to SNOW 124 at TR1330+75
    section!(t, course: "MATH 4400-01",
                instructor: "Jie Liu",
                rooms and times:
                    "Math lecture",
                    "3 credit bell schedule",
    );

    // MATH 4410-01: Actuarial Exam FM/ 2 Preparation
    // assigned to SNOW 124 at T1500+75
    section!(t, course: "MATH 4410-01",
                instructor: "Jie Liu",
                rooms and times:
                    "Math lecture",
                    "T1500+75",
    );

    // MATH 4800-01: Industrial Careers in Mathematics
    // assigned to SNOW 147 at MW1645+75
    section!(t, course: "MATH 4800-01",
                instructor: "Vinodh Kumar Chellamuthu",
                rooms and times:
                    "Math lecture",
                    "MW1645+75",
    );

    // MATH 900-01: Transitional Math I
    // assigned to SNOW 144 at MTWR1200+50
    section!(t, course: "MATH 900-01",
                instructor: "Paula Manuele Temple",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 900-02: Transitional Math I
    // assigned to SNOW 144 at MTWR0900+50
    section!(t, course: "MATH 900-02",
                instructor: "Jameson C Hardy",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 900-03: Transitional Math I
    // assigned to SNOW 144 at MW1300+100
    section!(t, course: "MATH 900-03",
                instructor: "Paula Manuele Temple",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 900-04: Transitional Math I
    // assigned to SNOW 144 at MW1600+100
    section!(t, course: "MATH 900-04",
                instructor: "Scott Patrick Hicks",
                rooms and times:
                    "Math lecture",
                    "MW1600+100",
    );

    // MATH 900-06: Transitional Math I
    // assigned to SNOW 3 at TR1630+100
    section!(t, course: "MATH 900-06",
                rooms and times:
                    "Math lecture",
                    "TR1630+100",
    );

    // MATH 900-07: Transitional Math I
    // assigned to SNOW 144 at TR1300+100
    section!(t, course: "MATH 900-07",
                instructor: "Paula Manuele Temple",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 900-51: Transitional Math I
    // assigned to SNOW 144 at MW1800+100
    section!(t, course: "MATH 900-51",
                instructor: "Scott Patrick Hicks",
                rooms and times:
                    "Math lecture",
                    "MW1800+100",
    );

    // MATH 980-03: Transitional Math IIB
    // assigned to SNOW 144 at MTWR1000+50
    section!(t, course: "MATH 980-03",
                instructor: "Tye K Rogers",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 980-05: Transitional Math IIB
    // assigned to SNOW 144 at TR1630+100
    section!(t, course: "MATH 980-05",
                instructor: "Michael N Paxman",
                rooms and times:
                    "Math lecture",
                    "TR1630+100",
    );

    // MATH 980-06: Transitional Math IIB
    // assigned to SNOW 144 at MTWR0800+50
    section!(t, course: "MATH 980-06",
                instructor: "Tye K Rogers",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 980-07: Transitional Math IIB
    // assigned to SNOW 3 at MW1300+100
    section!(t, course: "MATH 980-07",
                instructor: "Kathryn E Ott",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 980-08: Transitional Math IIB
    // assigned to SNOW 3 at TR1300+100
    section!(t, course: "MATH 980-08",
                instructor: "Amanda Fa'onelua",
                rooms and times:
                    "Math lecture",
                    "4 credit bell schedule",
    );

    // MATH 980-10: Transitional Math IIB
    // assigned to SNOW 3 at MW1630+100
    section!(t, course: "MATH 980-10",
                rooms and times:
                    "Math lecture",
                    "MW1630+100",
    );

    // MECH 1100-01: Manufacturing Processes
    // assigned to SET 226 at MW1200+75
    section!(t, course: "MECH 1100-01",
                instructor: "Andrew C Schiller",
                rooms and times:
                    "SET 226",
                    "3 credit bell schedule",
    );

    // MECH 1150-01: Prototyping Techniques
    // assigned to SET 225 at TR1500+170
    section!(t, course: "MECH 1150-01",
                instructor: "Andrew C Schiller",
                rooms and times:
                    "SET 225",
                    "TR1500+170",
    );

    // MECH 1150-02: Prototyping Techniques
    // assigned to SET 225 at MW1500+170
    section!(t, course: "MECH 1150-02",
                instructor: "Andrew C Schiller",
                rooms and times:
                    "SET 225",
                    "MW1500+170",
    );

    // MECH 1200-01: Coding
    // assigned to SET 226 at MWF0900+50
    section!(t, course: "MECH 1200-01",
                instructor: "Bing Jiang",
                rooms and times:
                    "SET 226",
                    "3 credit bell schedule",
    );

    // MECH 1200-02: Coding
    // assigned to SET 226 at MWF1000+50
    section!(t, course: "MECH 1200-02",
                instructor: "Scott A Skeen",
                rooms and times:
                    "SET 226",
                    "3 credit bell schedule",
    );

    // MECH 1205-01: Coding Lab
    // assigned to SET 226 at R0800+110
    section!(t, course: "MECH 1205-01",
                instructor: "David Brent Christensen",
                rooms and times:
                    "SET 226",
                    "2 hour lab",
    );

    // MECH 1205-02: Coding Lab
    // assigned to SET 226 at R1000+110
    section!(t, course: "MECH 1205-02",
                instructor: "David Brent Christensen",
                rooms and times:
                    "SET 226",
                    "2 hour lab",
    );

    // MECH 1205-03: Coding Lab
    // assigned to SET 226 at R1200+110
    section!(t, course: "MECH 1205-03",
                instructor: "Russell C Reid",
                rooms and times:
                    "SET 226",
                    "2 hour lab",
    );

    // MECH 1205-04: Coding Lab
    // assigned to SET 226 at R1400+110
    section!(t, course: "MECH 1205-04",
                instructor: "Bing Jiang",
                rooms and times:
                    "SET 226",
                    "2 hour lab",
    );

    // MECH 1205-05: Coding Lab
    // assigned to SET 226 at R1600+110
    section!(t, course: "MECH 1205-05",
                instructor: "Bing Jiang",
                rooms and times:
                    "SET 226",
                    "2 hour lab",
    );

    // MECH 2030-01: Dynamics
    // assigned to SET 104 at MWF1100+50
    section!(t, course: "MECH 2030-01",
                instructor: "Kameron J Eves",
                rooms and times:
                    "SET 104",
                    "3 credit bell schedule",
    );

    // MECH 2160-01: Materials Science
    // assigned to SET 226 at MW1500+75
    section!(t, course: "MECH 2160-01",
                instructor: "Divya Singh",
                rooms and times:
                    "SET 226",
                    "3 credit bell schedule",
    );

    // MECH 2250-01: Sensors & Actuators
    // assigned to SET 104 at MW1200+75
    section!(t, course: "MECH 2250-01",
                instructor: "Scott A Skeen",
                rooms and times:
                    "SET 104",
                    "3 credit bell schedule",
    );

    // MECH 2250-02: Sensors & Actuators
    // assigned to SET 104 at MW1330+75
    section!(t, course: "MECH 2250-02",
                instructor: "Scott A Skeen",
                rooms and times:
                    "SET 104",
                    "3 credit bell schedule",
    );

    // MECH 2255-01: Sensors & Actuators Lab
    // assigned to SET 101 at R0800+110
    section!(t, course: "MECH 2255-01",
                instructor: "Scott A Skeen",
                rooms and times:
                    "SET 101",
                    "2 hour lab",
    );

    // MECH 2255-02: Sensors & Actuators Lab
    // assigned to SET 101 at R1200+110
    section!(t, course: "MECH 2255-02",
                instructor: "Scott A Skeen",
                rooms and times:
                    "SET 101",
                    "2 hour lab",
    );

    // MECH 2255-03: Sensors & Actuators Lab
    // assigned to SET 101 at R1400+110
    section!(t, course: "MECH 2255-03",
                instructor: "David Brent Christensen",
                rooms and times:
                    "SET 101",
                    "2 hour lab",
    );

    // MECH 2255-04: Sensors & Actuators Lab
    // assigned to SET 101 at R1600+110
    section!(t, course: "MECH 2255-04",
                instructor: "Kameron J Eves",
                rooms and times:
                    "SET 101",
                    "2 hour lab",
    );

    // MECH 3250-01: Machinery
    // assigned to SET 104 at MW1630+75
    section!(t, course: "MECH 3250-01",
                instructor: "Divya Singh",
                rooms and times:
                    "SET 104",
                    "3 credit bell schedule",
    );

    // MECH 3255-01: Machinery Lab
    // assigned to SET 104 at T1200+110
    section!(t, course: "MECH 3255-01",
                instructor: "Divya Singh",
                rooms and times:
                    "SET 104",
                    "2 hour lab",
    );

    // MECH 3255-02: Machinery Lab
    // assigned to SET 226 at T1200+110
    section!(t, course: "MECH 3255-02",
                instructor: "Andrew C Schiller",
                rooms and times:
                    "SET 226",
                    "2 hour lab",
    );

    // MECH 3600-01: Thermodynamics
    // xlist entry: SC0A
    // assigned to SET 104 at MTWF0900+50
    section!(t, course: "MECH 3600-01",
                instructor: "Russell C Reid",
                rooms and times:
                    "SET 104",
                    "4 credit bell schedule",
    );

    // MECH 3602-01: Thermo II
    // xlist entry: SC0A
    // assigned to SET 104 at MTWF0900+50
    section!(t, course: "MECH 3602-01",
                instructor: "Russell C Reid",
                rooms and times:
                    "SET 104",
                    "4 credit bell schedule",
    );

    // MECH 3605-01: Thermodynamics Lab
    // assigned to SET 104 at R1400+110
    section!(t, course: "MECH 3605-01",
                instructor: "Russell C Reid",
                rooms and times:
                    "SET 104",
                    "2 hour lab",
    );

    // MECH 3605-02: Thermodynamics Lab
    // assigned to SET 104 at R1600+110
    section!(t, course: "MECH 3605-02",
                instructor: "Russell C Reid",
                rooms and times:
                    "SET 104",
                    "2 hour lab",
    );

    // MECH 3650-01: Heat Transfer
    // assigned to SET 104 at MW1500+75
    section!(t, course: "MECH 3650-01",
                instructor: "Russell C Reid",
                rooms and times:
                    "SET 104",
                    "3 credit bell schedule",
    );

    // MECH 3655-01: Heat Transfer Lab
    // assigned to SET 104 at R0800+110
    section!(t, course: "MECH 3655-01",
                instructor: "Russell C Reid",
                rooms and times:
                    "SET 104",
                    "2 hour lab",
    );

    // MECH 3655-02: Heat Transfer Lab
    // assigned to SET 104 at R1000+110
    section!(t, course: "MECH 3655-02",
                instructor: "Russell C Reid",
                rooms and times:
                    "SET 104",
                    "2 hour lab",
    );

    // MECH 4010-01: Product Design II
    // assigned to SET 219 at MWF1330+180
    section!(t, course: "MECH 4010-01",
                instructor: "Brant A Ross",
                rooms and times:
                    "SET 219",
                    "MWF1330+180",
    );

    // MECH 4500-01: Advanced Engineering Math
    // assigned to SET 523 at TR1500+75
    section!(t, course: "MECH 4500-01",
                instructor: "Scott A Skeen",
                rooms and times:
                    "SET 523",
                    "3 credit bell schedule",
    );

    // MECH 4860R-01: Design Practicum
    // assigned to SET 102 at M0800+50
    section!(t, course: "MECH 4860R-01",
                instructor: "Scott A Skeen",
                rooms and times:
                    "SET 102",
                    "1 credit bell schedule",
    );

    // MECH 4990-01: Special Topics: Finite Element Analysis
    // assigned to SET 523 at MW1000+110
    section!(t, course: "MECH 4990-01",
                instructor: "Divya Singh",
                rooms and times:
                    "SET 523",
                    "4 hour lab",
    );

    // MTRN 2350-01: Advanced PLC Programming
    // assigned to SET 102 at TR1000+50
    section!(t, course: "MTRN 2350-01",
                instructor: "Bruford P Reynolds",
                rooms and times:
                    "SET 102",
                    "TR1000+50",
    );

    // MTRN 2355-01: Advanced PLC Programming Lab
    // assigned to SET 102 at TR1400+110
    section!(t, course: "MTRN 2355-01",
                instructor: "Bruford P Reynolds",
                rooms and times:
                    "SET 102",
                    "4 hour lab",
    );

    // PHYS 1010-01: Elementary Physics (PS)
    // assigned to SET 418 at MW1630+75
    section!(t, course: "PHYS 1010-01",
                instructor: "David M Syndergaard",
                rooms and times:
                    "Science small lecture",
                    "3 credit bell schedule",
    );

    // PHYS 1015-01: Elementary Physics Lab (LAB)
    // assigned to SET 410 at M1300+110
    section!(t, course: "PHYS 1015-01",
                instructor: "David M Syndergaard",
                rooms and times:
                    "SET 410",
                    "2 hour lab",
    );

    // PHYS 1015-02: Elementary Physics Lab (LAB)
    // assigned to SET 410 at M1000+110
    section!(t, course: "PHYS 1015-02",
                rooms and times:
                    "SET 410",
                    "2 hour lab",
    );

    // PHYS 1040-50: Elementary Astronomy (PS)
    // assigned to SET 418 at MW1800+75
    section!(t, course: "PHYS 1040-50",
                instructor: "David M Syndergaard",
                rooms and times:
                    "Science small lecture",
                    "MW1800+75",
    );

    // PHYS 1045-50: Elementary Astronomy Lab (LAB)
    // assigned to SET 418 at M1930+170
    section!(t, course: "PHYS 1045-50",
                instructor: "Christopher Kirk DeMacedo",
                rooms and times:
                    "Science small lecture",
                    "M1930+170",
    );

    // PHYS 1045-51: Elementary Astronomy Lab (LAB)
    // assigned to SET 418 at T1930+170
    section!(t, course: "PHYS 1045-51",
                instructor: "Rick L Peirce",
                rooms and times:
                    "Science small lecture",
                    "T1930+170",
    );

    // PHYS 1045-52: Elementary Astronomy Lab (LAB)
    // assigned to SET 418 at W1930+170
    section!(t, course: "PHYS 1045-52",
                instructor: "Jose C Saraiva",
                rooms and times:
                    "Science small lecture",
                    "W1930+170",
    );

    // PHYS 2010-01: College Physics I (PS)
    // assigned to SET 418 at MWRF0800+50
    section!(t, course: "PHYS 2010-01",
                instructor: "Steven K Sullivan",
                rooms and times:
                    "Science small lecture",
                    "4 credit bell schedule",
    );

    // PHYS 2010-02: College Physics I (PS)
    // assigned to SET 418 at MWRF1500+50
    section!(t, course: "PHYS 2010-02",
                rooms and times:
                    "Science small lecture",
                    "4 credit 4×50 extended bell schedule",
    );

    // PHYS 2015-01: College Physics I Lab (LAB)
    // assigned to SET 410 at T1200+110
    section!(t, course: "PHYS 2015-01",
                instructor: "Christopher Kirk DeMacedo",
                rooms and times:
                    "SET 410",
                    "2 hour lab",
    );

    // PHYS 2015-02: College Physics I Lab (LAB)
    // assigned to SET 410 at T1400+110
    section!(t, course: "PHYS 2015-02",
                instructor: "Christopher Kirk DeMacedo",
                rooms and times:
                    "SET 410",
                    "2 hour lab",
    );

    // PHYS 2015-03: College Physics I Lab (LAB)
    // assigned to SET 410 at T1000+110
    section!(t, course: "PHYS 2015-03",
                rooms and times:
                    "SET 410",
                    "2 hour lab",
    );

    // PHYS 2020-01: College Physics II
    // assigned to SET 418 at MWRF1000+50
    section!(t, course: "PHYS 2020-01",
                instructor: "Steven K Sullivan",
                rooms and times:
                    "Science small lecture",
                    "4 credit bell schedule",
    );

    // PHYS 2020-02: College Physics II
    // assigned to SET 418 at MWRF1100+50
    section!(t, course: "PHYS 2020-02",
                instructor: "Steven K Sullivan",
                rooms and times:
                    "Science small lecture",
                    "4 credit bell schedule",
    );

    // PHYS 2025-01: College Physics II Lab
    // assigned to SET 412 at T1400+50
    section!(t, course: "PHYS 2025-01",
                rooms and times:
                    "SET 412",
                    "1 credit extended bell schedule",
    );

    // PHYS 2025-03: College Physics II Lab
    // assigned to SET 412 at T1600+110
    section!(t, course: "PHYS 2025-03",
                instructor: "Jose C Saraiva",
                rooms and times:
                    "SET 412",
                    "2 hour lab",
    );

    // PHYS 2025-04: College Physics II Lab
    // assigned to SET 412 at T1800+110
    section!(t, course: "PHYS 2025-04",
                rooms and times:
                    "SET 412",
                    "2 hour lab evening",
    );

    // PHYS 2210-01: Physics/Scientists Engineers I (PS)
    // assigned to SET 418 at MTWF1300+50
    section!(t, course: "PHYS 2210-01",
                instructor: "Samuel K Tobler",
                rooms and times:
                    "Science small lecture",
                    "4 credit 4×50 extended bell schedule",
    );

    // PHYS 2210-02: Physics/Scientists Engineers I (PS)
    // assigned to SET 418 at MTWF0900+50
    section!(t, course: "PHYS 2210-02",
                rooms and times:
                    "Science small lecture",
                    "4 credit bell schedule",
    );

    // PHYS 2215-01: Physics/Scientists Engineers I Lab (LAB)
    // assigned to SET 410 at R1400+110
    section!(t, course: "PHYS 2215-01",
                rooms and times:
                    "SET 410",
                    "2 hour lab",
    );

    // PHYS 2215-02: Physics/Scientists Engineers I Lab (LAB)
    // assigned to SET 410 at R1600+110
    section!(t, course: "PHYS 2215-02",
                rooms and times:
                    "SET 410",
                    "2 hour lab",
    );

    // PHYS 2215-50: Physics/Scientists Engineers I Lab (LAB)
    // assigned to SET 410 at R1800+110
    section!(t, course: "PHYS 2215-50",
                instructor: "Jose C Saraiva",
                rooms and times:
                    "SET 410",
                    "2 hour lab evening",
    );

    // PHYS 2220-01: Physics/Scientists Engineers II
    // assigned to SET 418 at MTWF1400+50
    section!(t, course: "PHYS 2220-01",
                instructor: "Samuel K Tobler",
                rooms and times:
                    "Science small lecture",
                    "4 credit 4×50 extended bell schedule",
    );

    // PHYS 2225-01: Physics/Scientists Engineers II Lab
    // assigned to SET 412 at R1400+110
    section!(t, course: "PHYS 2225-01",
                rooms and times:
                    "SET 412",
                    "2 hour lab",
    );

    // PHYS 2225-02: Physics/Scientists Engineers II Lab
    // assigned to SET 412 at R1600+110
    section!(t, course: "PHYS 2225-02",
                instructor: "Jose C Saraiva",
                rooms and times:
                    "SET 412",
                    "2 hour lab",
    );

    // PHYS 3600-01: Thermodynamics
    // assigned to SET 104 at MTWF0900+50
    section!(t, course: "PHYS 3600-01",
                rooms and times:
                    "SET 104",
                    "4 credit bell schedule",
    );

    // PHYS 3605-01: Thermodynamics Lab
    // assigned to SET 104 at R1400+110
    section!(t, course: "PHYS 3605-01",
                rooms and times:
                    "SET 104",
                    "2 hour lab",
    );

    // PHYS 3605-02: Thermodynamics Lab
    // assigned to SET 104 at R1600+110
    section!(t, course: "PHYS 3605-02",
                rooms and times:
                    "SET 104",
                    "2 hour lab",
    );

    // SCI 4700-01: Secondary Sci Teaching Methods
    // assigned to SET 216 at R1530+150
    section!(t, course: "SCI 4700-01",
                instructor: "Mark L Dickson",
                rooms and times:
                    "SET 216",
                    "R1530+150",
    );

    // SCI 4720-01: Innovative Solutions - Product Development
    // assigned to SET 501 at F1400+170
    section!(t, course: "SCI 4720-01",
                rooms and times:
                    "SET 501",
                    "F1400+170",
    );

    // envs envs emphasis
    conflict!(t, set penalty to 99,
            clique: "ENVS 1210", "ENVS 1215",
                    "ENVS 2210",
                    "GEO 1110", "GEO 1115",
                    "GEOG 3600", "GEOG 3605",
                    "CHEM 1210", "CHEM 1215",
                    "CHEM 1220", "CHEM 1225",
                    "BIOL 1610", "BIOL 1615",
                    "MATH 1060",
        
                    "ENVS 2700R",
                    "ENVS 4910",
                    "ENVS 3920");

    // envs geo emphasis
    conflict!(t, set penalty to 99,
            clique: "ENVS 1210", "ENVS 1215",
                    "ENVS 2210",
                    "GEO 1110", "GEO 1115",
                    "GEOG 3600", "GEOG 3605",
                    "CHEM 1210", "CHEM 1215",
                    "CHEM 1220", "CHEM 1225",
                    "BIOL 1610", "BIOL 1615",
                    "MATH 1060",

                    "GEO 1220", "GEO 1225",
                    "GEO 2700R");

    // remove penalty between classes and their prereqs
    add_prereqs!(t, course: "CHEM 1210", prereqs: "MATH 1050");
    add_prereqs!(t, course: "CHEM 1210", prereqs: "MATH 1050");
    add_prereqs!(t, course: "CHEM 1210", prereqs: "MATH 1050");
    add_prereqs!(t, course: "ENVS 2700R", prereqs: "ENVS 1210", "ENVS 1215");
    add_prereqs!(t, course: "GEO 1220", prereqs: "GEO 1110", "GEO 1115");
    add_prereqs!(t, course: "GEO 12250", prereqs: "GEO 1110", "GEO 1115");
    add_prereqs!(t, course: "MATH 1060", prereqs: "MATH 1050");

    ////reduce scores by section count + lil more
    //conflict!(t, two section reduction: "BIOL 1610");
    //conflict!(t, two section reduction: "ENVS 1215");
    //conflict!(t, two section reduction: "MATH 1060");
    //conflict!(t, three section reduction: "CHEM 1210");
    //conflict!(t, three section reduction: "CHEM 1215");
    //
    ////multiple section scheduling conflict with themselves
    //conflict!(t, set hard, clique: "CHEM 1210-01", "CHEM 1210-02");
    //conflict!(t, set hard, clique: "CHEM 1220-01", "CHEM 1220-02");
    //conflict!(t, set hard, clique: "BIOL 1610-01", "BIOL 1610-02");
    //
    ////class and coreq lab conflict
    //conflict!(t, set hard, clique: "ENVS 1210-01", "ENVS 1215-01", "ENVS 1215-02");
    //conflict!(t, set hard, clique: "GEO 1110", "GEO 1115");
    //conflict!(t, set hard, clique: "GEO 1220", "GEO 1225");
    //conflict!(t, set hard, clique: "GEOG 3600", "GEOG 3605");

    // envs envs emphasis
    conflict!(t, set penalty to 99,
            clique: "ENVS 1210", "ENVS 1215", "ENVS 2210",
                    "GEO 1110", "GEO 1115",
                    "GEOG 3600", "GEOG 3605",
                    "CHEM 1210", "CHEM 1220",
                    "BIOL 1610",
                    "MATH 1060",
                    "ENVS 2700R", "ENVS 4910", "ENVS 3920");

    // envs geo emphasis
    conflict!(t, set penalty to 99,
            clique: "ENVS 1210", "ENVS 1215", "ENVS 2210",
                    "GEO 1110", "GEO 1115",
                    "GEOG 3600", "GEOG 3605",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "BIOL 1610", "BIOL 1615",
                    "MATH 1060",
                    "GEO 1220", "GEO 1225", "GEO 2700R");

    // geological sciences
    conflict!(t, set penalty to 99,
            clique: "BIOL 3110",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "GEO 1110", "GEO 1115", "GEO 1220", "GEO 1225", "GEO 2700R", "GEO 2990R",
                    "GEO 3060", "GEO 3180", "GEO 3200", "GEO 3500", "GEO 3550",
                    "GEO 3600", "GEO 3700", "GEO 3710", "GEO 4600", "GEO 4800R",
                    "GEOG 3600", "GEOG 3605",
                    "MATH 1210",
                    "PHYS 2010", "PHYS 2015", "PHYS 2210", "PHYS 2215",
                    "PHYS 2020", "PHYS 2025", "PHYS 2220", "PHYS 2225",
                    "GEO 3000", "GEO 3910",
                    "ENVS 3910", "ENVS 3920", "ENVS 3930",
                    "GEOG 3930");

    // bioinformatics core
    conflict!(t, set penalty to 99,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625",
                    "BIOL 3010", "BIOL 3300", "BIOL 3030", "BIOL 4010", "BIOL 4300",
                    "BIOL 4305", "BIOL 4310", "BIOL 4320", "BIOL 4810R", "BIOL 4910",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "CS 1400", "CS 1410", "CS 2420", "CS 2450", "CS 3310",
                    "IT 1100", "IT 2300",
                    "MATH 1210", "MATH 3060");
    // bioinformatics pick one tech lab course
    conflict!(t, set penalty to 30,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625",
                    "BIOL 3010", "BIOL 3300", "BIOL 3030", "BIOL 4010", "BIOL 4300",
                    "BIOL 4305", "BIOL 4310", "BIOL 4320", "BIOL 4810R", "BIOL 4910",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "CS 1400", "CS 1410", "CS 2420", "CS 2450", "CS 3310",
                    "IT 1100", "IT 2300",
                    "MATH 1210", "MATH 3060",

                    "BTEC 2010", "BTEC 2020", "BTEC 2030", "BTEC 2050", "BIOL 2300");

    //bio_education emphasis
    conflict!(t, set penalty to 99,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "BIOL 3010", "BIOL 3030", "HIST 1700", "POLS 1100", "FSHD 1500", "PSY 1010", "PSY 1100",
                    "EDUC 1010", "EDUC 2010", "EDUC 2400", "EDUC 2500", "EDUC 3110", "EDUC 2700", "MATH 1050",
                    "BIOL 2320", "BIOL 2325", "BIOL 3140", "BIOL 3145", "BIOL 2420", "BIOL 2425", "BIOL 4500", "BIOL 4505",
                    "BIOL 3040", "BIOL 3045", "BIOL 2060", "BIOL 2065", "BIOL 3450", "BIOL 3455", "BIOL 3550", "BIOL 3555",
                    "BIOL 2400", "BIOL 2405", "BIOL 3200", "BIOL 3205", "BIOL 4260", "BIOL 4265", "BIOL 4270",
                    "BIOL 4275", "BIOL 4350", "BIOL 4355", "BIOL 4380", "BIOL 4385", "BIOL 4411", "BIOL 4415", "BIOL 4440",
                    "SCI 2600", "SCI 4700",
                    "SCED 3720", "SCED 4100", "SCED 4200", "SCED 4600", "SCED 4300", "SCED 4900", "SCED 4989");

    //bio bio-sciences
    conflict!(t, set penalty to 99,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 3010", "BIOL 3030",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315", "CHEM 2320", "CHEM 2325",
                    "MATH 1210",
                    "BIOL 3040", "BIOL 3045", "BIOL 3155",
                    "MATH 3060",
                    "BIOL 4910");
    conflict!(t, set penalty to 45,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 3010", "BIOL 3030",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315", "CHEM 2320", "CHEM 2325",
                    "MATH 1210",
                    "BIOL 3040", "BIOL 3045", "BIOL 3155",
                    "MATH 3060",
                    "BIOL 4910",

                    "PHYS 2010", "PHYS 2015", "PHYS 2020", "PHYS 2025", "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225",

                    "BTEC 2010", "BTEC 2020", "BTEC 2030", "BTEC 2050", "BIOL 2300",

                    "BIOL 3450", "BIOL 3455", "BIOL 3550", "BIOL 3555",

                    "BIOL 3420", "BIOL 4500", "BIOL 4505", "BIOL 4600", "BIOL 4605",

                    "BIOL 3200", "BIOL 3205", "BIOL 4260", "BIOL 4265", "BIOL 4270", "BIOL 4275",
                    "BIOL 4280", "BIOL 4350", "BIOL 4355", "BIOL 4380", "BIOL 4385", "BIOL 4411", "BIOL 4415", "BIOL 4440");
    conflict!(t, set penalty to 30,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 3010", "BIOL 3030",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315", "CHEM 2320", "CHEM 2325",
                    "MATH 1210",
                    "BIOL 3040", "BIOL 3045", "BIOL 3155",
                    "MATH 3060",
                    "BIOL 4910",

                    "PHYS 2010", "PHYS 2015", "PHYS 2020", "PHYS 2025", "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225",

                    "BTEC 2010", "BTEC 2020", "BTEC 2030", "BTEC 2050", "BIOL 2300",

                    "BIOL 3450", "BIOL 3455", "BIOL 3550", "BIOL 3555",

                    "BIOL 3420", "BIOL 4500", "BIOL 4505", "BIOL 4600", "BIOL 4605",

                    "BIOL 3200", "BIOL 3205", "BIOL 4260", "BIOL 4265", "BIOL 4270", "BIOL 4275",
                    "BIOL 4280", "BIOL 4350", "BIOL 4355", "BIOL 4380", "BIOL 4385", "BIOL 4411", "BIOL 4415", "BIOL 4440",

                    "BTEC 3020", "CHEM 3510", "CHEM 3515", "CHEM 3520", "CHEM 3525",
                    "BTEC 3010", "BTEC 3040", "BTEC 3050", "BTEC 4020", "BTEC 4040", "BTEC 4050", "BTEC 4060");

    //bio biomed
    conflict!(t, set penalty to 99,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 3010", "BIOL 3030", "BIOL 3040",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315",
                    "CHEM 2320", "CHEM 2325", "CHEM 3510", "CHEM 3515",
                    "PHYS 2010", "PHYS 2015", "PHYS 2020", "PHYS 2025",
                    "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225",
                    "BIOL 2320", "BIOL 2325", "BIOL 3420",
                    "MATH 3060",
                    "BIOL 3155", "BIOL 3450", "BIOL 3455", "BIOL 3550", "BIOL 3555", "BIOL 4910",
                    "BTEC 2010", "BTEC 2020", "BTEC 2030", "BTEC 2050",
                    "BIOL 2300",
                    "PSY 2400", "PSY 3460", "PSY 3710");
    conflict!(t, set penalty to 30,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 3010", "BIOL 3030", "BIOL 3040",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315",
                    "CHEM 2320", "CHEM 2325", "CHEM 3510", "CHEM 3515",
                    "PHYS 2010", "PHYS 2015", "PHYS 2020", "PHYS 2025",
                    "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225",
                    "BIOL 2320", "BIOL 2325", "BIOL 3420",
                    "MATH 3060",
                    "BIOL 3155", "BIOL 3450", "BIOL 3455", "BIOL 3550", "BIOL 3555", "BIOL 4910",
                    "BTEC 2010", "BTEC 2020", "BTEC 2030", "BTEC 2050",
                    "BIOL 2300",
                    "PSY 2400", "PSY 3460", "PSY 3710",

                    "BIOL 3000R", "BIOL 3100", "BIOL 3110", "BIOL 3120", "BIOL 3140", "BIOL 3145",
                    "BIOL 3230R", "BIOL 3250", "BIOL 3360", "BIOL 3460", "BIOL 3470",
                    "BIOL 4300", "BIOL 4305", "BIOL 4440", "BIOL 4930R",
                    "CHEM 3520", "CHEM 3525",
                    "MATH 1210");

    //bio natural sciences
    conflict!(t, set penalty to 99,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 2400", "BIOL 2405",
                    "BIOL 3010", "BIOL 3030", "BIOL 3040", "BIOL 3045", "BIOL 3110", "BIOL 3120", "BIOL 4910",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "ENVS 1210", "ENVS 1215",
                    "GEO 1110", "GEO 1115",
                    "GEOG 3600", "GEOG 3605",
                    "MATH 1040", "MATH 1050",
                    "PHYS 1010", "PHYS 1015", "PHYS 2010", "PHYS 2015");
    conflict!(t, set penalty to 45,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 2400", "BIOL 2405",
                    "BIOL 3010", "BIOL 3030", "BIOL 3040", "BIOL 3045", "BIOL 3110", "BIOL 3120", "BIOL 4910",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "ENVS 1210", "ENVS 1215",
                    "GEO 1110", "GEO 1115",
                    "GEOG 3600", "GEOG 3605",
                    "MATH 1040", "MATH 1050",
                    "PHYS 1010", "PHYS 1015", "PHYS 2010", "PHYS 2015",
                    "BIOL 3200", "BIOL 3340", "BIOL 3345", "BIOL 4200", "BIOL 4205", "BIOL 4260",
                    "BIOL 4265", "BIOL 4270", "BIOL 4275", "BIOL 4280", "BIOL 4350", "BIOL 4355",
                    "BIOL 4380", "BIOL 4385", "BIOL 4411", "BIOL 4415", "BIOL 4440", "BIOL 4600", "BIOL 4605");
    conflict!(t, set penalty to 30,
            clique: "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 2400", "BIOL 2405",
                    "BIOL 3010", "BIOL 3030", "BIOL 3040", "BIOL 3045", "BIOL 3110", "BIOL 3120", "BIOL 4910",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "ENVS 1210", "ENVS 1215",
                    "GEO 1110", "GEO 1115",
                    "GEOG 3600", "GEOG 3605",
                    "MATH 1040", "MATH 1050",
                    "PHYS 1010", "PHYS 1015", "PHYS 2010", "PHYS 2015",
                    "BIOL 3200", "BIOL 3340", "BIOL 3345", "BIOL 4200", "BIOL 4205", "BIOL 4260",
                    "BIOL 4265", "BIOL 4270", "BIOL 4275", "BIOL 4280", "BIOL 4350", "BIOL 4355",
                    "BIOL 4380", "BIOL 4385", "BIOL 4411", "BIOL 4415", "BIOL 4440", "BIOL 4600", "BIOL 4605",
                    "BIOL 3100", "BIOL 3140", "BIOL 3145", "BIOL 3250", "BIOL 3360", "BIOL 3450", "BIOL 3455",
                    "BIOL 3550", "BIOL 3555", "BIOL 4300", "BIOL 4305", "BIOL 4500", "BIOL 4505",
                    "BIOL 4810R", "BIOL 4930R",
                    "GEOG 4140", "GEOG 4180",
                    "MATH 1210", "MATH 3060",
                    "BIOL 3155");

    //bio integrated edu sciences
    conflict!(t, set penalty to 99,
            clique: "HIST 1700", "POLS 1100", "FSHD 1500", "PSY 1010", "PSY 1100",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "PHYS 2010", "PHYS 2015",
                    "MATH 1050", "MATH 1060", "MATH 1080",
                    "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625", "BIOL 2320", "BIOL 2325",
                    "BIOL 3140", "BIOL 3145", "BIOL 2420", "BIOL 2425", "BIOL 4500", "BIOL 4505",
                    "BIOL 3010", "BIOL 3030", "BIOL 3040", "BIOL 3045", "BIOL 2060", "BIOL 2065",
                    "BIOL 3450", "BIOL 3455", "BIOL 3550", "BIOL 3555",
                    "BIOL 2400", "BIOL 2405", "BIOL 3200", "BIOL 3205",
                    "BIOL 4260", "BIOL 4265", "BIOL 4270", "BIOL 4275", "BIOL 4350", "BIOL 4355",
                    "BIOL 4380", "BIOL 4385", "BIOL 4411", "BIOL 4415", "BIOL 4440",
                    "GEO 1110", "GEO 1115",
                    "PHYS 1040", "PHYS 1045",
                    "SCI 2600",
                    "EDUC 1010", "EDUC 2010", "EDUC 2400", "EDUC 2500", "EDUC 3110",
                    "SCI 4700",
                    "SCED 3720", "SCED 4100", "SCED 4200", "SCED 4600", "SCED 4300", "SCED 4900", "SCED 4989");

    //chemistry chemistry major
    conflict!(t, set penalty to 99,
            clique: "MATH 1210", "MATH 1220",
                    "BIOL 1610", "BIOL 1615",
                    "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "CHEM 2310", "CHEM 2315", "CHEM 2320", "CHEM 2325", "CHEM 2600", "CHEM 2990R",
                    "CHEM 3000", "CHEM 3005", "CHEM 3060", "CHEM 3065", "CHEM 3070", "CHEM 3075",
                    "CHEM 3100", "CHEM 3300", "CHEM 3510", "CHEM 3515", "CHEM 3520", "CHEM 3525",
                    "CHEM 4100", "CHEM 4800R", "CHEM 4910", "CHEM 4200", "CHEM 4310", "CHEM 4510", "CHEM 4610");
    conflict!(t, set penalty to 30,
            clique: "MATH 1210", "MATH 1220",
                    "BIOL 1610", "BIOL 1615",
                    "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225",
                    "MATH 2210", "MATH 2250", "MATH 2270", "MATH 2280", "MATH 3060",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225",
                    "CHEM 2310", "CHEM 2315", "CHEM 2320", "CHEM 2325", "CHEM 2600", "CHEM 2990R",
                    "CHEM 3000", "CHEM 3005", "CHEM 3060", "CHEM 3065", "CHEM 3070", "CHEM 3075",
                    "CHEM 3100", "CHEM 3300", "CHEM 3510", "CHEM 3515", "CHEM 3520", "CHEM 3525",
                    "CHEM 4100", "CHEM 4800R", "CHEM 4910", "CHEM 4200", "CHEM 4310", "CHEM 4510", "CHEM 4610");

    //chem molecular biology
    conflict!(t, set penalty to 99,
            clique: "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315",
                    "CHEM 2320", "CHEM 2325", "CHEM 2600", "CHEM 2990R",
                    "CHEM 3000", "CHEM 3005", "CHEM 3060", "CHEM 3065", "CHEM 3070", "CHEM 3075", "CHEM 3100",
                    "CHEM 3300", "CHEM 3510", "CHEM 3515", "CHEM 3520", "CHEM 3525", "CHEM 4910",
                    "BIOL 1610", "BIOL 1615", "BIOL 3030", "BIOL 3550", "BIOL 3555", "BIOL 4300", "BIOL 4305",
                    "MATH 1210", "MATH 1220",
                    "PHYS 2010", "PHYS 2015", "PHYS 2020", "PHYS 2025",
                    "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225",
                    "CHEM 4800R",
                    "BIOL 4810R", "BIOL 4890R");
    conflict!(t, set penalty to 30,
            clique: "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315",
                    "CHEM 2320", "CHEM 2325", "CHEM 2600", "CHEM 2990R",
                    "CHEM 3000", "CHEM 3005", "CHEM 3060", "CHEM 3065", "CHEM 3070", "CHEM 3075", "CHEM 3100",
                    "CHEM 3300", "CHEM 3510", "CHEM 3515", "CHEM 3520", "CHEM 3525", "CHEM 4910",
                    "BIOL 1610", "BIOL 1615", "BIOL 3030", "BIOL 3550", "BIOL 3555", "BIOL 4300", "BIOL 4305",
                    "MATH 1210", "MATH 1220",
                    "PHYS 2010", "PHYS 2015", "PHYS 2020", "PHYS 2025",
                    "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225",
                    "CHEM 4800R",
                    "BIOL 4810R", "BIOL 4890R",

                    "CHEM 4100", "CHEM 4610",
                    "BIOL 3010", "BIOL 3250", "BIOL 3360", "BIOL 3420",
                    "BIOL 3450", "BIOL 3455", "BIOL 3470", "BIOL 3460", "BIOL 4400");

    //chem physical sciences
    conflict!(t, set penalty to 99,
            clique: "SCI 4700",
                    "SCED 3720", "SCED 4100", "SCED 4200", "SCED 4600", "SCED 4300", "SCED 4900", "SCED 4989",
                    "HIST 1700", "POLS 1100", "FSHD 1500", "PSY 1010", "PSY 1100",
                    "EDUC 1010", "EDUC 2010", "EDUC 2400", "EDUC 2500", "EDUC 3110",
                    "CHEM 1210", "CHEM 1215", "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315", "CHEM 3000",
                    "GEO 1110", "GEO 1115", "GEO 1220", "GEO 1225", "GEO 3060",
                    "PHYS 1040", "PHYS 1045", "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225", "PHYS 3710",
                    "BIOL 1610", "BIOL 1615",
                    "MATH 1210", "MATH 1220",
                    "SCI 2600", "SCI 4800R",
                    "CHEM 3510",
                    "PHYS 3400");

    // complete one of the following sets, etc.
    // note: conflicts between coreqs will be reinstituted later
    conflict!(t, remove penalty,
            clique: "PHYS 2010", "PHYS 2015", "PHYS 2020", "PHYS 2025",
                    "PHYS 2210", "PHYS 2215", "PHYS 2220", "PHYS 2225");

    // complete one technical lab course
    conflict!(t, remove penalty,
            clique: "BTEC 2010", "BTEC 2020", "BTEC 2030", "BTEC 2050", "BIOL 2300");

    // complete one of the following sets, etc.
    // note: conflicts between coreqs will be reinstituted later
    conflict!(t, remove penalty,
            clique: "BIOL 3450", "BIOL 3455", "BIOL 3550", "BIOL 3555");

    // complete one of the following sets, etc.
    // note: conflicts between coreqs will be reinstituted later
    conflict!(t, remove penalty,
            clique: "BIOL 3420", "BIOL 4500", "BIOL 4505", "BIOL 4600", "BIOL 4605");

    // complete one of the following sets, etc.
    // note: conflicts between coreqs will be reinstituted later
    conflict!(t, remove penalty,
            clique: "BIOL 3200", "BIOL 3205", "BIOL 4260", "BIOL 4265", "BIOL 4270", "BIOL 4275",
                    "BIOL 4280", "BIOL 4350", "BIOL 4355", "BIOL 4380", "BIOL 4385", "BIOL 4411", "BIOL 4415", "BIOL 4440",
    );
    conflict!(t, remove penalty, clique: "MATH 1050", "MATH 1080");
    conflict!(t, remove penalty, clique: "MATH 1050", "MATH 1080");

    // complete one of the following sets, etc.
    // note: conflicts between coreqs will be reinstituted later
    conflict!(t, remove penalty, clique: "BIOL 2320", "BIOL 2325", "BIOL 3140", "BIOL 3145");
    conflict!(t, remove penalty, clique: "BIOL 2420", "BIOL 2425", "BIOL 4500", "BIOL 4505");
    conflict!(t, remove penalty, clique: "BIOL 2060", "BIOL 2065", "BIOL 3450", "BIOL 3455", "BIOL 3550", "BIOL 3555");
    //complete one of the following
    conflict!(t,remove penalty, clique: "CHEM 2310", "CHEM 2315", "CHEM 3000");
    conflict!(t,remove penalty, clique: "CHEM 3510", "PHYS 3400");

    Ok(())
}

pub fn input_prereqs(t: &mut Input) -> Result<(), String> {
    add_prereqs!(t, course: "CS 1400", prereqs: "CS 1030", "MATH 1010");
    add_prereqs!(t, course: "CS 1410", prereqs: "CS 1400");
    add_prereqs!(t, course: "CS 2420", prereqs: "CS 1410");
    add_prereqs!(t, course: "CS 2450", prereqs: "CS 1410");
    add_prereqs!(t, course: "CS 2500", prereqs: "CS 1410");
    add_prereqs!(t, course: "CS 2810", prereqs: "CS 1410");
    add_prereqs!(t, course: "CS 3005", prereqs: "CS 1410");
    add_prereqs!(t, course: "CS 3150", prereqs: "CS 2420", "CS 2810");
    add_prereqs!(t, course: "CS 3310", prereqs: "CS 1410", "MATH 1100", "MATH 1210");
    add_prereqs!(t, course: "CS 3400", prereqs: "CS 2420", "CS 2810", "CS 3005");
    add_prereqs!(t, course: "CS 3410", prereqs: "CS 2420", "CS 2810");
    add_prereqs!(t, course: "CS 3500", prereqs: "CS 3005");
    add_prereqs!(t, course: "CS 3510", prereqs: "CS 2420", "CS 2810", "CS 3310");
    add_prereqs!(t, course: "CS 3520", prereqs: "CS 2420", "CS 2810");
    add_prereqs!(t, course: "CS 3530", coreqs: "CS 3310", prereqs: "CS 2420", "CS 2810", "CS 3310");
    add_prereqs!(t, course: "CS 3600", prereqs: "CS 2420", "CS 3005");
    add_prereqs!(t, course: "CS 4300", prereqs: "CS 2420", "CS 2810", "CS 3005");
    add_prereqs!(t, course: "CS 4307", prereqs: "CS 2420", "CS 2810");
    add_prereqs!(t, course: "CS 4310", prereqs: "CS 4307", "IT 2300");
    add_prereqs!(t, course: "CS 4320", prereqs: "CS 2420", "CS 2810", "CS 3005");
    add_prereqs!(t, course: "CS 4400", prereqs: "CS 2420", "CS 2810");
    add_prereqs!(t, course: "CS 4410", prereqs: "CS 2420", "CS 2810");
    add_prereqs!(t, course: "CS 4550", prereqs: "CS 2420", "CS 2810", "CS 3005");
    add_prereqs!(t, course: "CS 4600", prereqs: "CS 2420", "CS 2810", "CS 3005"); // sorta
    add_prereqs!(t, course: "CS 4991R", prereqs: "CS 1400");
    add_prereqs!(t, course: "CS 4992R", prereqs: "CS 2420", "CS 2810");

    add_prereqs!(t, course: "SE 3010", prereqs: "CS 2420", "CS 3005");
    add_prereqs!(t, course: "SE 3020", prereqs: "CS 2420", "CS 3005");
    add_prereqs!(t, course: "SE 3100", prereqs: "CS 2450");
    add_prereqs!(t, course: "SE 3150", prereqs: "CS 2450");
    add_prereqs!(t, course: "SE 3200", prereqs: "CS 1410", "SE 1400", "CS 2810");
    add_prereqs!(t, course: "SE 3400", prereqs: "SE 1400");
    add_prereqs!(t, course: "SE 3450", prereqs: "SE 1400");
    add_prereqs!(t, course: "SE 4600", prereqs: "CS 2420", "CS 2810", "CS 3005", "SE 1400", "SE 3200"); // sorta
    add_prereqs!(t, course: "SE 4200", prereqs: "SE 3200");

    add_prereqs!(t, course: "IT 2300", prereqs: "CS 1400", "IT 1100", "CS 1410");
    add_prereqs!(t, course: "IT 2400", coreqs: "IT 1100", "IT 1500", prereqs: "IT 1100", "IT 1500");
    add_prereqs!(t, course: "IT 2500", prereqs: "IT 2400");
    add_prereqs!(t, course: "IT 2700", prereqs: "CS 1400", "IT 2400");
    add_prereqs!(t, course: "IT 3100", prereqs: "CS 1400", "IT 1100", "IT 2400", "CS 3150");
    add_prereqs!(t, course: "IT 3110", prereqs: "CS 1410", "IT 3100");
    add_prereqs!(t, course: "IT 3150", prereqs: "IT 2400");
    add_prereqs!(t, course: "IT 3300", prereqs: "IT 2400", "IT 1100", "CS 3150");
    add_prereqs!(t, course: "IT 3400", prereqs: "IT 2400");
    add_prereqs!(t, course: "IT 4100", prereqs: "IT 3100");
    add_prereqs!(t, course: "IT 4200", prereqs: "CS 1400", "IT 2400", "CS 2810");
    add_prereqs!(t, course: "IT 4310", prereqs: "IT 2300");
    add_prereqs!(t, course: "IT 4400", prereqs: "IT 3400");
    add_prereqs!(t, course: "IT 4600", prereqs: "CS 1410", "IT 2400"); // sorta
    add_prereqs!(t, course: "IT 4510", prereqs: "CS 1410", "IT 3100");

    // scraped data
    add_prereqs!(t, course: "BIOL 1610", coreqs: "BIOL 1615");
    add_prereqs!(t, course: "BIOL 1615", coreqs: "BIOL 1610");
    add_prereqs!(t, course: "BIOL 1620", coreqs: "BIOL 1625", prereqs: "BIOL 1610");
    add_prereqs!(t, course: "BIOL 1625", coreqs: "BIOL 1620", prereqs: "BIOL 1615", "BIOL 1615A");
    add_prereqs!(t, course: "BIOL 2060", coreqs: "BIOL 2065", prereqs: "BIOL 1010", "BIOL 1200", "BIOL 1610");
    add_prereqs!(t, course: "BIOL 2065", coreqs: "BIOL 2060");
    add_prereqs!(t, course: "BIOL 2320", coreqs: "BIOL 2325");
    add_prereqs!(t, course: "BIOL 2325", coreqs: "BIOL 2320");
    add_prereqs!(t, course: "BIOL 2400", coreqs: "BIOL 2405");
    add_prereqs!(t, course: "BIOL 2405", coreqs: "BIOL 2400");
    add_prereqs!(t, course: "BIOL 2420", coreqs: "BIOL 2425");
    add_prereqs!(t, course: "BIOL 2425", coreqs: "BIOL 2420");
    add_prereqs!(t, course: "BIOL 3000R", prereqs: "HLOC 2000");
    add_prereqs!(t, course: "BIOL 3010", prereqs: "BIOL 1620");
    add_prereqs!(t, course: "BIOL 3030", prereqs: "BIOL 1610");
    add_prereqs!(t, course: "BIOL 3040", prereqs: "BIOL 1620");
    add_prereqs!(t, course: "BIOL 3045", coreqs: "BIOL 3040", prereqs: "BIOL 1620");
    add_prereqs!(t, course: "BIOL 3100", prereqs: "BIOL 3010", "BIOL 3030", "BIOL 3040");
    add_prereqs!(t, course: "BIOL 3110", prereqs: "ENGL 2010", "BIOL 3010", "BIOL 3030", "BIOL 3040");
    add_prereqs!(t, course: "BIOL 3120", prereqs: "ENGL 2010", "BIOL 3010", "3030, 3040");
    add_prereqs!(t, course: "BIOL 3140", coreqs: "BIOL 3145", prereqs: "BIOL 3010");
    add_prereqs!(t, course: "BIOL 3145", coreqs: "BIOL 3140");
    add_prereqs!(t, course: "BIOL 3155", prereqs: "BIOL 3010", "BIOL 3030", "MATH 3060");
    add_prereqs!(t, course: "BIOL 3200", prereqs: "BIOL 3010", "BIOL 3030");
    add_prereqs!(t, course: "BIOL 3205", coreqs: "BIOL 3200", prereqs: "BIOL 3010", "BIOL 3030");
    add_prereqs!(t, course: "BIOL 3230R", prereqs: "BIOL 2320", "BIOL 2325");
    add_prereqs!(t, course: "BIOL 3250", prereqs: "BIOL 3030");
    add_prereqs!(t, course: "BIOL 3300", prereqs: "CS 1400", "IT 1100");
    add_prereqs!(t, course: "BIOL 3340", coreqs: "BIOL 3345", prereqs: "MATH 1010");
    add_prereqs!(t, course: "BIOL 3345", coreqs: "BIOL 3340", prereqs: "BIOL 1625", "BIOL 1625A", "BIOL 2405");
    add_prereqs!(t, course: "BIOL 3360", prereqs: "BIOL 3010", "BIOL 3030");
    add_prereqs!(t, course: "BIOL 3420", prereqs: "BIOL 1610");
    add_prereqs!(t, course: "BIOL 3450", coreqs: "BIOL 3455", prereqs: "BIOL 3030", "CHEM 1220");
    add_prereqs!(t, course: "BIOL 3455", coreqs: "BIOL 3450");
    add_prereqs!(t, course: "BIOL 3460", prereqs: "BIOL 3010", "BIOL 3030");
    add_prereqs!(t, course: "BIOL 3470", prereqs: "BIOL 3010", "CHEM 3510");
    add_prereqs!(t, course: "BIOL 3550", coreqs: "BIOL 3555", prereqs: "BIOL 3030", "CHEM 2310");
    add_prereqs!(t, course: "BIOL 3555", coreqs: "BIOL 3550", prereqs: "CHEM 2315");
    add_prereqs!(t, course: "BIOL 4010", prereqs: "BIOL 3030");
    add_prereqs!(t, course: "BIOL 4200", coreqs: "BIOL 4205", prereqs: "BIOL 1620");
    add_prereqs!(t, course: "BIOL 4205", coreqs: "BIOL 4200");
    add_prereqs!(t, course: "BIOL 4260", coreqs: "BIOL 4265", prereqs: "BIOL 3040", "BIOL 3045");
    add_prereqs!(t, course: "BIOL 4265", coreqs: "BIOL 4260", prereqs: "BIOL 3040", "BIOL 3045");
    add_prereqs!(t, course: "BIOL 4270", coreqs: "BIOL 4275", prereqs: "BIOL 3040", "BIOL 3045");
    add_prereqs!(t, course: "BIOL 4275", coreqs: "BIOL 4270", prereqs: "BIOL 3040", "BIOL 3045");
    add_prereqs!(t, course: "BIOL 4280", prereqs: "BIOL 3040");
    add_prereqs!(t, course: "BIOL 4300", coreqs: "BIOL 4305", prereqs: "BIOL 3030", "CHEM 1220");
    add_prereqs!(t, course: "BIOL 4305", coreqs: "BIOL 4300");
    add_prereqs!(t, course: "BIOL 4310", prereqs: "BIOL 3300");
    add_prereqs!(t, course: "BIOL 4320", prereqs: "BIOL 3300");
    add_prereqs!(t, course: "BIOL 4350", coreqs: "BIOL 4355", prereqs: "BIOL 3010", "BIOL 3030");
    add_prereqs!(t, course: "BIOL 4355", coreqs: "BIOL 4350", prereqs: "BIOL 3010", "BIOL 3030");
    add_prereqs!(t, course: "BIOL 4380", coreqs: "BIOL 4385", prereqs: "BIOL 3040", "BIOL 3010");
    add_prereqs!(t, course: "BIOL 4385", coreqs: "BIOL 4380");
    add_prereqs!(t, course: "BIOL 4400", prereqs: "BIOL 2320", "BIOL 2325", "BIOL 2420", "BIOL 2425");
    add_prereqs!(t, course: "BIOL 4411", coreqs: "BIOL 4415");
    add_prereqs!(t, course: "BIOL 4415", coreqs: "BIOL 4411", prereqs: "BIOL 3045");
    add_prereqs!(t, course: "BIOL 4440", prereqs: "BIOL 1620");
    add_prereqs!(t, course: "BIOL 4500", coreqs: "BIOL 4505", prereqs: "BIOL 3010", "BIOL 3030", "CHEM 1220");
    add_prereqs!(t, course: "BIOL 4505", coreqs: "BIOL 4500", prereqs: "CHEM 1225");
    add_prereqs!(t, course: "BIOL 4600", coreqs: "BIOL 4605", prereqs: "BIOL 3010", "BIOL 3030", "CHEM 1220");
    add_prereqs!(t, course: "BIOL 4605", coreqs: "BIOL 4600", prereqs: "CHEM 1225");
    add_prereqs!(t, course: "BIOL 4910", prereqs: "ENGL 2010");
    add_prereqs!(t, course: "BIOL 4930R", prereqs: "BIOL 3110");
    add_prereqs!(t, course: "BTEC 2010", prereqs: "BTEC 1010", "BIOL 1610", "BIOL 1620", "BIOL 1620");
    add_prereqs!(t, course: "BTEC 2020", prereqs: "BTEC 1010", "BIOL 1610", "BIOL 1620", "BIOL 1620");
    add_prereqs!(t, course: "BTEC 2030", prereqs: "BTEC 1010", "BIOL 1610", "BIOL 1620", "BIOL 1620");
    add_prereqs!(t, course: "BTEC 2050", prereqs: "BIOL 1610", "BIOL 1620");
    add_prereqs!(t, course: "BTEC 3010", prereqs: "BIOL 3030");
    add_prereqs!(t, course: "BTEC 3020", prereqs: "BTEC 2020");
    add_prereqs!(t, course: "BTEC 3050", prereqs: "BTEC 2050");
    add_prereqs!(t, course: "BTEC 4020", prereqs: "BTEC 3020");
    add_prereqs!(t, course: "BTEC 4040", prereqs: "BTEC 3050");
    add_prereqs!(t, course: "BTEC 4050", prereqs: "BTEC 3050");
    add_prereqs!(t, course: "BTEC 4060", prereqs: "BTEC 4050");
    add_prereqs!(t, course: "CHEM 1210", coreqs: "CHEM 1215", prereqs: "MATH 1050");
    add_prereqs!(t, course: "CHEM 1215", coreqs: "CHEM 1210");
    add_prereqs!(t, course: "CHEM 1220", coreqs: "CHEM 1225", prereqs: "CHEM 1210");
    add_prereqs!(t, course: "CHEM 1225", coreqs: "CHEM 1220", prereqs: "CHEM 1215");
    add_prereqs!(t, course: "CHEM 2310", coreqs: "CHEM 2315", prereqs: "CHEM 1220");
    add_prereqs!(t, course: "CHEM 2315", coreqs: "CHEM 2310", prereqs: "CHEM 1225");
    add_prereqs!(t, course: "CHEM 2320", coreqs: "CHEM 2325", prereqs: "CHEM 2310");
    add_prereqs!(t, course: "CHEM 2325", coreqs: "CHEM 2320", prereqs: "CHEM 2315");
    add_prereqs!(t, course: "CHEM 2600", prereqs: "CHEM 1220", "CHEM 1225");
    add_prereqs!(t, course: "CHEM 3000", coreqs: "CHEM 3005", prereqs: "CHEM 1220");
    add_prereqs!(t, course: "CHEM 3005", coreqs: "CHEM 3000", prereqs: "CHEM 1225");
    add_prereqs!(t, course: "CHEM 3060", prereqs: "BIOL 3110");
    add_prereqs!(t, course: "CHEM 3065", coreqs: "CHEM 3060", prereqs: "PHYS 2015", "PHYS 2215", "CHEM 2325");
    add_prereqs!(t, course: "CHEM 3070", prereqs: "PHYS 2010", "PHYS 2210", "CHEM 2320", "MATH 1220");
    add_prereqs!(t, course: "CHEM 3075", coreqs: "CHEM 3070", prereqs: "PHYS 2015", "PHYS 2215", "CHEM 2325");
    add_prereqs!(t, course: "CHEM 3100", prereqs: "CHEM 2320");
    add_prereqs!(t, course: "CHEM 3300", prereqs: "CHEM 3000", "CHEM 3005");
    add_prereqs!(t, course: "CHEM 3510", coreqs: "CHEM 3515", prereqs: "BIOL 1610", "CHEM 2320");
    add_prereqs!(t, course: "CHEM 3515", coreqs: "CHEM 3510", prereqs: "BIOL 1615", "CHEM 2325");
    add_prereqs!(t, course: "CHEM 3520", coreqs: "CHEM 3525", prereqs: "CHEM 3510");
    add_prereqs!(t, course: "CHEM 3525", coreqs: "CHEM 3520", prereqs: "CHEM 3515");
    add_prereqs!(t, course: "CHEM 4100", prereqs: "CHEM 3100");
    add_prereqs!(t, course: "CHEM 4200", prereqs: "CHEM 2320");
    add_prereqs!(t, course: "CHEM 4310", prereqs: "CHEM 2320", "CHEM 2325");
    add_prereqs!(t, course: "CHEM 4510", prereqs: "CHEM 2320", "CHEM 2325", "CHEM 3000", "CHEM 3005");
    add_prereqs!(t, course: "CHEM 4610", prereqs: "CHEM 3520");
    add_prereqs!(t, course: "CHEM 4800R", prereqs: "CHEM 2320", "CHEM 2325", "ENGL 2010", "ENGL 2010A");
    add_prereqs!(t, course: "CHEM 4910", prereqs: "CHEM 2320", "CHEM 2325", "ENGL 2010");
    add_prereqs!(t, course: "EDUC 3110", prereqs: "FSHD 1500", "PSY 1010", "PSY 1100");
    add_prereqs!(t, course: "ENER 3310", prereqs: "MATH 1050", "CHEM 1210");
    add_prereqs!(t, course: "ENER 4310", prereqs: "ENER 2310", "GEO 2050");
    add_prereqs!(t, course: "ENVS 1210", coreqs: "ENVS 1215");
    add_prereqs!(t, course: "ENVS 1215", coreqs: "ENVS 1210");
    add_prereqs!(t, course: "ENVS 2210", prereqs: "ENVS 1210", "ENVS 1215", "MATH 1050", "CHEM 1210", "CHEM 1215");
    add_prereqs!(t, course: "ENVS 2700R", prereqs: "ENVS 1210", "ENVS 1215");
    add_prereqs!(t, course: "ENVS 3280", prereqs: "ENVS 2210");
    add_prereqs!(t, course: "ENVS 3410", prereqs: "ENVS 2210", "CHEM 1210");
    add_prereqs!(t, course: "ENVS 3510", prereqs: "ENVS 2210", "GEO 2050");
    add_prereqs!(t, course: "ENVS 4080", prereqs: "ENVS 3410", "ENVS 2700R");
    add_prereqs!(t, course: "GEO 1110", coreqs: "GEO 1115");
    add_prereqs!(t, course: "GEO 1115", coreqs: "GEO 1110");
    add_prereqs!(t, course: "GEO 1220", coreqs: "GEO 1225", prereqs: "GEO 1110");
    add_prereqs!(t, course: "GEO 1225", coreqs: "GEO 1220", prereqs: "GEO 1115");
    add_prereqs!(t, course: "GEO 2050", prereqs: "GEO 1110", "GEO 1115");
    add_prereqs!(t, course: "GEO 2700R", prereqs: "GEO 1110", "GEO 1115");
    add_prereqs!(t, course: "GEO 3000", prereqs: "GEO 1110", "GEO 1115");
    add_prereqs!(t, course: "GEO 3060", prereqs: "GEO 1110", "GEO 1115");
    add_prereqs!(t, course: "GEO 3180", prereqs: "GEO 1110", "GEO 1115");
    add_prereqs!(t, course: "GEO 3200", prereqs: "GEO 1110", "GEO 1115", "MATH 1050", "CHEM 1210", "CHEM 1215");
    add_prereqs!(t, course: "GEO 3400", prereqs: "GEO 1110", "GEO 1115", "CHEM 1210", "CHEM 1215");
    add_prereqs!(t, course: "GEO 3500", coreqs: "GEOG 3600", "GEOG 3605", prereqs: "GEO 1110", "GEO 1115", "MATH 1060", "CHEM 1210", "CHEM 1215");
    add_prereqs!(t, course: "GEO 3550", prereqs: "GEO 1220", "GEO 1225");
    add_prereqs!(t, course: "GEO 3600", prereqs: "GEO 1110", "GEO 1115", "Math 1050", "CHEM 1210", "CHEM 1215", "GEO 3200");
    add_prereqs!(t, course: "GEO 3700", prereqs: "GEO 1110", "GEO 1115", "MATH 1060", "MATH 1080");
    add_prereqs!(t, course: "GEO 4600", prereqs: "GEO 2700R", "GEO 3550", "GEO 3700");
    add_prereqs!(t, course: "GEO 4800R", prereqs: "GEO 2700R");
    add_prereqs!(t, course: "GEOG 2410", prereqs: "ENVS 1210", "BIOL 1610");
    add_prereqs!(t, course: "GEOG 3600", coreqs: "GEOG 3605");
    add_prereqs!(t, course: "GEOG 3605", coreqs: "GEOG 3600");
    add_prereqs!(t, course: "GEOG 4140", prereqs: "GEOG 3600", "GEOG 3605");
    add_prereqs!(t, course: "GEOG 4180", prereqs: "GEOG 3600", "GEOG 3605");
    add_prereqs!(t, course: "MATH 1040", prereqs: "MATH 0980");
    add_prereqs!(t, course: "MATH 1050", prereqs: "MATH 1010", "MATH 1000");
    add_prereqs!(t, course: "MATH 1060", prereqs: "MATH 1050");
    add_prereqs!(t, course: "MATH 1080", prereqs: "MATH 1010", "MATH 1000");
    add_prereqs!(t, course: "MATH 1210", prereqs: "MATH 1050", "MATH 1060", "MATH 1080");
    add_prereqs!(t, course: "MATH 1220", prereqs: "MATH 1210");
    add_prereqs!(t, course: "MATH 2210", prereqs: "MATH 1220");
    add_prereqs!(t, course: "MATH 2250", prereqs: "Math 1220");
    add_prereqs!(t, course: "MATH 2270", prereqs: "MATH 1210");
    add_prereqs!(t, course: "MATH 2280", prereqs: "MATH 1220");
    add_prereqs!(t, course: "MATH 3060", prereqs: "MATH 1210");
    add_prereqs!(t, course: "MATH 3400", prereqs: "MATH 1220");
    add_prereqs!(t, course: "MATH 3500", prereqs: "MATH 2270", "MATH 2280", "MATH 2250");
    add_prereqs!(t, course: "PHYS 1010", prereqs: "MATH 1010");
    add_prereqs!(t, course: "PHYS 1015", coreqs: "PHYS 1010");
    add_prereqs!(t, course: "PHYS 1040", coreqs: "PHYS 1045");
    add_prereqs!(t, course: "PHYS 1045", coreqs: "PHYS 1040");
    add_prereqs!(t, course: "PHYS 2010", coreqs: "PHYS 2015", prereqs: "MATH 1060", "MATH 1080");
    add_prereqs!(t, course: "PHYS 2015", coreqs: "PHYS 2010");
    add_prereqs!(t, course: "PHYS 2020", coreqs: "PHYS 2025", prereqs: "PHYS 2010");
    add_prereqs!(t, course: "PHYS 2025", coreqs: "PHYS 2020", prereqs: "PHYS 2015");
    add_prereqs!(t, course: "PHYS 2210", coreqs: "PHYS 2215", prereqs: "MATH 1210", "MATH 1220");
    add_prereqs!(t, course: "PHYS 2215", coreqs: "PHYS 2210");
    add_prereqs!(t, course: "PHYS 2220", coreqs: "PHYS 2225", prereqs: "MATH 1220", "PHYS 2210");
    add_prereqs!(t, course: "PHYS 2225", coreqs: "PHYS 2220", prereqs: "PHYS 2215");
    add_prereqs!(t, course: "PHYS 3400", prereqs: "PHYS 2220");
    add_prereqs!(t, course: "PHYS 3710", prereqs: "MATH 1220", "PHYS 2220");
    add_prereqs!(t, course: "POLS 1100", prereqs: "ENGL 1010", "ENGL 1010D");
    add_prereqs!(t, course: "PSY 2400", prereqs: "PSY 1010");
    add_prereqs!(t, course: "PSY 3460", prereqs: "PSY 1010");
    add_prereqs!(t, course: "PSY 3710", prereqs: "BIOL 1010", "BIOL 1610", "PSY 1010");
    add_prereqs!(t, course: "SCED 4900", coreqs: "SCED 4989");
    add_prereqs!(t, course: "SE 3100", prereqs: "SE 2450", "CS 2450", "WEB 3450");

    Ok(())
}

pub fn input_multiples(t: &mut Input) -> Result<(), String> {
    multiple_sections_reduce_penalties!(t,
            courses:
                "BIOL 1010", "BIOL 1015", "BIOL 1200", "BIOL 1610", "BIOL 1615", "BIOL 1620", "BIOL 1625",
                "BIOL 2065", "BIOL 2320", "BIOL 2325", "BIOL 2420", "BIOL 2425", "BIOL 3010", "BIOL 3030",
                "BIOL 3155", "BIOL 3230R", "BIOL 3455", "BIOL 4890R", "BIOL 4910", "BIOL 4990R",
                "BTEC 2050",
                "CHEM 1010", "CHEM 1015", "CHEM 1125", "CHEM 1150", "CHEM 1155", "CHEM 1210", "CHEM 1215",
                "CHEM 1220", "CHEM 1225", "CHEM 2310", "CHEM 2315", "CHEM 2320", "CHEM 2325", "CHEM 3300",
                "CHEM 3515", "CHEM 3525", "CHEM 4800R",
                "CS 1400", "CS 1410", "CS 2450", "CS 2810", "CS 4600",
                "ECE 4990",
                "ENVS 1010", "ENVS 1215",
                "GEO 1010", "GEO 1015", "GEO 3500", "GEO 3600",
                "GEOG 1000", "GEOG 1005",
                "IT 1100",
                "MATH 900", "MATH 980", "MATH 1010", "MATH 1030", "MATH 1040", "MATH 1050", "MATH 1060",
                "MATH 1210", "MATH 1220", "MATH 2020",
                "MECH 1150", "MECH 1200", "MECH 1205", "MECH 2250", "MECH 2255", "MECH 3255", "MECH 3605", "MECH 3655",
                "PHYS 1015", "PHYS 1045", "PHYS 2010", "PHYS 2015", "PHYS 2020", "PHYS 2025",
                "PHYS 2210", "PHYS 2215", "PHYS 2225", "PHYS 3605",
                "SE 1400");

    // multiple-section courses must be taught at different times
    // TODO:
    //multiple_sections_spread_out!(t, days: "mt", times: "0800-1200", "1200-1630",
    //        courses: "CS 1400", "CS 1410", "CS 2450", "CS 2810", "IT 1100", "SE 1400");
    conflict!(t, set hard, clique: "CS 1400-01", "CS 1400-02", "CS 1400-03", "CS 1400-04");
    conflict!(t, set hard, clique: "CS 1410-01", "CS 1410-02");
    conflict!(t, set hard, clique: "CS 2450-01", "CS 2450-02");
    conflict!(t, set hard, clique: "CS 2810-01", "CS 2810-02");
    conflict!(t, set hard, clique: "IT 1100-01", "IT 1100-02");
    conflict!(t, set hard, clique: "SE 1400-01", "SE 1400-02");

    Ok(())
}
