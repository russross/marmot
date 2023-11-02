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

    time!(t, name: "MWF0800+50", tags: "mwf");
    time!(t, name: "MWF0900+50", tags: "mwf", "morning");
    time!(t, name: "MWF1000+50", tags: "mwf", "morning");
    time!(t, name: "MWF1100+50", tags: "mwf", "morning");
    time!(t, name: "MW1200+75", tags: "mw", "afternoon");
    time!(t, name: "MW1330+75", tags: "mw", "afternoon");
    time!(t, name: "MW1500+75", tags: "mw", "afternoon");
    time!(t, name: "MW1630+75", tags: "mw");
    time!(t, name: "TR0900+75", tags: "tr", "morning");
    time!(t, name: "TR1030+75", tags: "tr", "morning");
    time!(t, name: "TR1200+75", tags: "tr", "afternoon");
    time!(t, name: "TR1330+75", tags: "tr", "afternoon");
    time!(t, name: "TR1500+75", tags: "tr", "afternoon");
    time!(t, name: "TR1630+75", tags: "tr");
    time!(t, name: "T1800+150", tags: "evening");
    time!(t, name: "W1800+150", tags: "evening");
    time!(t, name: "R1800+150", tags: "evening");
    time!(t, name: "R1900+50");

    instructor!(t,
        name:
            "Bart Stander",
        available:
            "MWF0900+50",
            "MWF1000+50",
            "MWF1100+50",
            "MW1200+75" with penalty 10,
            "MW1330+75",
            "MW1500+75",
            "TR1030+75",
            "TR1330+75",
            "TR1500+75" with penalty 10,
    );
    //t.preference("oneday");

    instructor!(t,
        name:
            "Carol Stander",
        available:
            "MWF1000+50",
            "MWF1100+50",
            "MW1200+75" with penalty 10,
            "MW1330+75",
            "TR1330+75" with penalty 5,
    );
    instructor!(t,
        name:
            "Curtis Larsen",
        available:
            "MWF0900+50",
            "MWF1000+50",
            "MWF1100+50" with penalty 10,
            "MW1200+75" with penalty 10,
            "MW1330+75",
            "MW1500+75",
            "TR0900+75",
            "TR1030+75" with penalty 10,
            "TR1200+75" with penalty 10,
            "TR1330+75",
            "TR1500+75",
    );
    //t.preference("twodays");

    instructor!(t,
        name:
            "DJ Holt",
        available:
            "MW1200+75",
            "MW1330+75",
            "MW1500+75" with penalty 10,
            "TR0900+75",
            "TR1030+75",
            "TR1200+75",
            "TR1330+75",
            "TR1500+75" with penalty 10,
    );
    //t.preference("twodays");

    instructor!(t,
        name:
            "Eric Pedersen",
        available:
            "TR1200+75",
    );

    instructor!(t,
        name:
            "Jay Sneddon",
        available:
            "MWF0800+50" with penalty 15,
            "MWF0900+50" with penalty 10,
            "MWF1000+50" with penalty 10,
            "MWF1100+50" with penalty 10,
            "MW1200+75",
            "MW1330+75",
            "MW1500+75",
            "TR0900+75",
            "TR1030+75",
            "TR1200+75",
            "TR1330+75",
            "TR1500+75" with penalty 5,
    );
    //t.preference("twodays");

    instructor!(t,
        name:
            "Jeff Compas",
        available:
            "MWF0800+50",
            "MW1630+75",
            "TR1630+75",
            "T1800+150",
    );

    instructor!(t,
        name:
            "Joe Francom",
        available:
            "MWF0900+50",
            "MWF1000+50",
            "MWF1100+50",
            "MW1330+75",
    );
    //t.preference("oneday");

    instructor!(t,
        name:
            "Lora Klein",
        available:
            "TR0900+75",
            "TR1030+75",
            "TR1200+75",
            "TR1330+75",
            "MW1500+75" with penalty 15,
    );

    instructor!(t,
        name:
            "Matt Kearl",
        available:
            "MW1200+75",
            "TR0900+75",
            "TR1030+75",
            "TR1200+75",
    );
    //t.preference("oneday");

    instructor!(t,
        name:
            "Phil Daley",
        available:
            "MWF0900+50",
            "MWF1000+50",
            "MWF1100+50",
            "MW1200+75",
            "MW1330+75",
            "MW1500+75" with penalty 10,
            "TR0900+75",
            "TR1030+75",
            "TR1200+75",
            "TR1330+75",
            "TR1500+75" with penalty 10,
    );
    //t.preference("twodays");

    instructor!(t,
        name:
            "Ren Quinn",
        available:
            "MWF0900+50",
            "MWF1000+50",
            "MWF1100+50",
            "TR1200+75" with penalty 5,
            "TR1330+75",
            "TR1500+75",
            "R1900+50",
     );
    //t.preference("twodays");

    instructor!(t,
        name:
            "Russ Ross",
        available:
            "MW1200+75",
            "MW1330+75",
            "MW1500+75" with penalty 10,
            "TR1200+75",
            "TR1330+75",
            "TR1500+75" with penalty 10,
    );

    instructor!(t,
        name:
            "Rex Frisbey",
        available:
            "MWF1100+50",
    );

    instructor!(t,
        name:
            "Jamie Bennion",
        available:
            "W1800+150",
    );

    section!(t, course: "CS 2420"-"01",
                instructor: "Bart Stander",
                rooms and times: "stadium", "flex" with penalty 10, "mwf");

    section!(t, course: "CS 3310"-"01",
                instructor: "Bart Stander",
                rooms and times: "stadium", "pcs");
    section!(t, course: "CS 3600"-"01",
                instructor: "Bart Stander",
                rooms and times: "pcs", "stadium" with penalty 10);
    section!(t, course: "CS 4550"-"01",
                instructor: "Bart Stander",
                rooms and times: "pcs");

    section!(t, course: "CS 1030"-"01",
                instructor: "Carol Stander",
                rooms and times: "flex");
    section!(t, course: "CS 1410"-"02",
                instructor: "Carol Stander",
                rooms and times: "flex");
    //course: CS1410 online
    //course: IT1100 online

    section!(t, course: "CS 3005"-"01",
                instructor: "Curtis Larsen",
                rooms and times: "Smith 116", "mwf");
    section!(t, course: "CS 3510"-"01",
                instructor: "Curtis Larsen",
                rooms and times: "flex" with penalty 1, "Smith 116", "mw", "mwf", "tr" with penalty 10);
    section!(t, course: "CS 4320"-"01",
                instructor: "Curtis Larsen",
                rooms and times: "flex" with penalty 1, "Smith 116", "mw", "mwf" with penalty 10, "tr");
    section!(t, course: "CS 4600"-"01",
                instructor: "Curtis Larsen",
                rooms and times: "flex" with penalty 1, "Smith 116", "mw", "mwf", "tr" with penalty 10);
    // all senior projects at same time
    // course: CS 4920R not scheduled

    section!(t, course: "SE 3010"-"01",
                instructor: "DJ Holt",
                rooms and times: "flex", "macs", "MW1500+75"); // same day as SE4200
    section!(t, course: "SE 4200"-"01",
                instructor: "DJ Holt",
                rooms and times: "flex", "macs", "MW1330+75");
    // with cross-listed CS4600-02 section; all senior projects at same time
    section!(t, course: "SE 4600"-"01",
                instructor: "DJ Holt",
                rooms and times: "flex");
    section!(t, course: "CS 4600"-"02",
                instructor: "DJ Holt",
                rooms and times: "flex");
    crosslist!(t, "SE 4600"-"01" cross-list with "CS 4600"-"02");
    // schedule SE4600 same time as CS4600-01: Curtis and I will each have a section of CS4600, and my section will be cross-listed with SE4600
    // note all CS cross-listings above

    section!(t, course: "SE 3500"-"01",
                instructor: "Eric Pedersen",
                rooms and times: "flex");

    section!(t, course: "IT 1200"-"01",
                instructor: "Jay Sneddon",
                rooms and times: "Smith 107", "tr");
    section!(t, course: "IT 2300"-"01",
                instructor: "Jay Sneddon",
                rooms and times: "Smith 107");
    section!(t, course: "IT 2700"-"01",
                instructor: "Jay Sneddon",
                rooms and times: "Smith 107", "tr");
    section!(t, course: "IT 3150"-"01",
                instructor: "Jay Sneddon",
                rooms and times: "Smith 107", "mw", "mwf" with penalty 5);
    section!(t, course: "IT 3400"-"01",
                instructor: "Jay Sneddon",
                rooms and times: "Smith 107");

    section!(t, course: "CS 1400"-"03",
                instructor: "Jeff Compas",
                rooms and times: "stadium");
    section!(t, course: "CS 1400"-"04",
                instructor: "Jeff Compas",
                rooms and times: "stadium");
    section!(t, course: "CS 2450"-"02",
                instructor: "Jeff Compas",
                rooms and times: "flex");
    section!(t, course: "SE 3100"-"01",
                instructor: "Jeff Compas",
                rooms and times: "flex");
    //he needs two morning courses and two evening courses
    //3 credit release for first semester

    section!(t, course: "IT 3110"-"01",
                instructor: "Joe Francom",
                rooms and times: "flex");
    section!(t, course: "IT 4510"-"01",
                instructor: "Joe Francom",
                rooms and times: "flex");
    //online section of IT1500 1 credit
    //online section of IT4600 3 credits

    section!(t, course: "SE 3200"-"01",
                instructor: "Lora Klein",
                rooms and times: "Smith 107" with penalty 5, "flex");
    //course: CS1410 ACE MW 9:30-10:45am, INV 112
    //course: CS1410 ACE MW 12:00-1:15pm, INV 112
    // workload release for program development

    section!(t, course: "SE 3450"-"01",
                instructor: "Matt Kearl",
                rooms and times: "flex", "macs");
    section!(t, course: "SE 3550"-"01",
                instructor: "Matt Kearl",
                rooms and times: "flex", "macs");
    section!(t, course: "SE 1400"-"01",
                instructor: "Matt Kearl",
                rooms and times: "macs");
    //course: SE1400 online
    //course: SE1400 online
    //course: SE4920 not scheduled

    section!(t, course: "IT 1100"-"01",
                instructor: "Phil Daley",
                rooms and times: "pcs");
    section!(t, course: "IT 1100"-"02",
                instructor: "Phil Daley",
                rooms and times: "pcs");
    section!(t, course: "IT 2400"-"01",
                instructor: "Phil Daley",
                rooms and times: "Smith 107");
    section!(t, course: "IT 3100"-"01",
                instructor: "Phil Daley",
                rooms and times: "Smith 107");
    // avoid IT 4510 so Phil can shadow Joe
    // 3 credits release for network admin

    section!(t, course: "CS 1400"-"01",
                instructor: "Ren Quinn",
                rooms and times: "flex");
    section!(t, course: "CS 1400"-"02",
                instructor: "Ren Quinn",
                rooms and times: "flex");
    section!(t, course: "CS 1410"-"01",
                instructor: "Ren Quinn",
                rooms and times: "flex");
    section!(t, course: "CS 2450"-"01",
                instructor: "Ren Quinn",
                rooms and times: "flex");
    section!(t, course: "CS 3150"-"01",
                instructor: "Ren Quinn",
                rooms and times: "flex");
    section!(t, course: "CS 4991"-"01",
                instructor: "Ren Quinn",
                rooms and times: "Smith 116", "R1900+50"); // overload acm
    // course: CS4992 flex F1300 // overload seminar
    // course: CS4800R not scheduled
    // CS4991R is actually R1900-1950

    section!(t, course: "CS 2810"-"01",
                instructor: "Russ Ross",
                rooms and times: "Smith 107" with penalty 5, "Smith 108" with penalty 5, "Smith 109");
    section!(t, course: "CS 2810"-"02",
                instructor: "Russ Ross",
                rooms and times: "Smith 107" with penalty 5, "Smith 108" with penalty 5, "Smith 109");
    section!(t, course: "CS 3410"-"01",
                instructor: "Russ Ross",
                rooms and times: "Smith 107" with penalty 5, "Smith 108" with penalty 5, "Smith 109");
    section!(t, course: "CS 4307"-"01",
                instructor: "Russ Ross",
                rooms and times: "Smith 107" with penalty 5, "Smith 108" with penalty 5, "Smith 109");

    section!(t, course: "SE 1400"-"02",
                instructor: "Rex Frisbey",
                rooms and times: "macs");

    section!(t, course: "IT 4990"-"01",
                instructor: "Jamie Bennion",
                rooms and times: "flex");


    conflict!(t, set hard,
                clique: "CS 2420", "CS 2450", "CS 2810", "CS 3005");    // 3rd/4th semester classes
    conflict!(t, set hard,
                clique: "CS 2420", "CS 2450", "CS 2810");               // grad plan: 2nd year fall
    conflict!(t, set hard,
                clique: "CS 3005", "CS 3520", "SE 3200");               // grad plan: 2nd year spring
    conflict!(t, set hard,
                clique: "CS 3310", "CS 3400", "SE 3530");               // grad plan: 3nd year fall
    conflict!(t, set hard,
                clique: "CS 3510", "CS 4307", "SE 4550");               // grad plan: 3nd year spring
    conflict!(t, set hard,
                clique: "CS 4300");                                     // grad plan: 4th year fall
    conflict!(t, set hard,
                clique: "CS 3600", "CS 4600");                          // grad plan: 4th year spring

    // CS upper division core
    conflict!(t, set penalty to 90,
                clique: "CS 2450",
                        "CS 3150", "CS 3310", "CS 3400", "CS 3410", "CS 3510", "CS 3520", "CS 3530", "CS 3600",
                        "CS 4300", "CS 4307", "CS 4320", "CS 4550", "CS 4600",
                        "SE 3200");

    // CS electives
    conflict!(t, set penalty to 30,
                clique: "CS 2450",
                        "CS 3150", "CS 3310", "CS 3400", "CS 3410", "CS 3500", "CS 3510", "CS 3520", "CS 3530", "CS 3600",
                        "CS 4300", "CS 4307", "CS 4320", "CS 4550", "CS 4600", "CS 4990",
                        "SE 3010", "SE 3020", "SE 3100", "SE 3200", "SE 3400", "SE 4200",
                        "IT 2700", "IT 3100", "IT 3110", "IT 4200");

    // CS classes that do not require CS2810 so they can be taken concurrently
    conflict!(t, set penalty to 45,
                clique: "CS 2810", "SE 3020", "SE 3200", "CS 3500");

    // CS classes that do not require CS3005 so they can be taken concurrently
    conflict!(t, set penalty to 45,
                clique: "CS 3005", "CS 3150", "CS 3310", "CS 3410", "CS 3510", "CS 3520", "CS 3530");

    // DS: TODO
    conflict!(t, set penalty to 45,
                clique: "CS 2500", "CS 2810", "CS 3005");

    // SE upper division core
    conflict!(t, set penalty to 90,
                clique: "CS 2450", "CS 3150", "CS 3310", "CS 3510", "CS 4307",
                        "IT 2300",
                        "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400",
                        "SE 4200", "SE 4600"); // IT 1100, SE 1400

    // Entrepreneurial and marketing track
    conflict!(t, set penalty to 90,
                clique: "CS 2450", "CS 3150", "CS 3310", "CS 3510", "CS 4307",
                        "IT 2300",
                        "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400", "SE 3500", "SE 3550",
                        "SE 4200", "SE 4600"); // IT 1100 SE 1400

    // DevOps track
    conflict!(t, set penalty to 90,
                clique: "CS 2450", "CS 3150", "CS 3310", "CS 3510", "CS 4307",
                        "IT 2300", "IT 3110", "IT 3300", "IT 4200",
                        "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400",
                        "SE 4200", "SE 4600"); // IT 1100 SE 1400

    // Application track
    conflict!(t, set penalty to 90,
                clique: "CS 2450", "CS 3150", "CS 3310", "CS 3500", "CS 3510", "CS 4307",
                        "IT 2300",
                        "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400", "SE 3450",
                        "SE 4200", "SE 4600"); // IT 1100 SE 1400

    // Data science track
    conflict!(t, set penalty to 90,
                clique: "CS 2450", "CS 3150", "CS 3310", "CS 3510", "CS 4300", "CS 4307", "CS 4320",
                        "IT 2300",
                        "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400",
                        "SE 4200", "SE 4600"); // IT 1100 SE 1400

    // IT conflicts
    //conflict!(t, set penalty to 50, clique: "IT 1100", "IT 1200"); // when there is only one in-person section of each
    conflict!(t, set penalty to 90,
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

    conflict!(t, remove penalty, clique: "CS 2450", "SE 3100"); // CS2450 is a prereq for SE3100 so no conflict
    conflict!(t, remove penalty, clique: "CS 2450", "SE 3150"); // CS2450 is a prereq for SE3150 so no conflict
    conflict!(t, remove penalty, clique: "SE 3200", "SE 4200"); // SE3200 is a prereq for SE4200 so no conflict
    conflict!(t, remove penalty, clique: "CS 4307", "IT 2300"); // students take either CS4307 or IT2300 but not both so no conflict

    // multiple-section courses must be taught at different times
    conflict!(t, set hard, clique: "CS 1400");
    conflict!(t, set hard, clique: "CS 1410");
    conflict!(t, set hard, clique: "CS 2450");
    conflict!(t, set hard, clique: "CS 2810");
    conflict!(t, set hard, clique: "IT 1100");
    conflict!(t, set hard, clique: "SE 1400");

    // courses that must be scheduled at the same time
    //anticonflict!(t, set penalty to 50, clique: "CS 1400", "CS 1030");
    //anticonflict!(t, set penalty to 50, clique: "SE 1400", "IT 1100"); // temporarily removed because of new hire planning
    //anticonflict!(t, set penalty to 50, clique: "CS 4600", "SE 4600");

    Ok(t)
}
