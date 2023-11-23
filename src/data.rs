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

    input_computing(&mut t)?;
    input_set(&mut t)?;

    Ok(t)
}

pub fn input_computing(t: &mut Input) -> Result<(), String> {
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

    section!(t, course: "CS 2420-01",
                instructor: "Bart Stander",
                rooms and times: "stadium", "flex" with penalty 10, "mwf");

    section!(t, course: "CS 3310-01",
                instructor: "Bart Stander",
                rooms and times: "stadium", "pcs");
    section!(t, course: "CS 3600-01",
                instructor: "Bart Stander",
                rooms and times: "pcs", "stadium" with penalty 10);
    section!(t, course: "CS 4550-01",
                instructor: "Bart Stander",
                rooms and times: "pcs");

    section!(t, course: "CS 1030-01",
                instructor: "Carol Stander",
                rooms and times: "flex");
    section!(t, course: "CS 1410-02",
                instructor: "Carol Stander",
                rooms and times: "flex");
    //course: CS1410 online
    //course: IT1100 online

    section!(t, course: "CS 3005-01",
                instructor: "Curtis Larsen",
                rooms and times: "Smith 116", "mwf");
    section!(t, course: "CS 3510-01",
                instructor: "Curtis Larsen",
                rooms and times: "flex" with penalty 1, "Smith 116", "mw", "mwf", "tr" with penalty 10);
    section!(t, course: "CS 4320-01",
                instructor: "Curtis Larsen",
                rooms and times: "flex" with penalty 1, "Smith 116", "mw", "mwf" with penalty 10, "tr");
    section!(t, course: "CS 4600-01",
                instructor: "Curtis Larsen",
                rooms and times: "flex" with penalty 1, "Smith 116", "mw", "mwf", "tr" with penalty 10);
    // all senior projects at same time
    // course: CS 4920R not scheduled

    section!(t, course: "SE 3010-01",
                instructor: "DJ Holt",
                rooms and times: "flex", "macs", "MW1500+75"); // same day as SE4200
    section!(t, course: "SE 4200-01",
                instructor: "DJ Holt",
                rooms and times: "flex", "macs", "MW1330+75");
    // with cross-listed CS4600-02 section; all senior projects at same time
    section!(t, course: "SE 4600-01",
                instructor: "DJ Holt",
                rooms and times: "flex");
    section!(t, course: "CS 4600-02",
                instructor: "DJ Holt",
                rooms and times: "flex");
    crosslist!(t, "SE 4600-01" cross-list with "CS 4600-02");
    // schedule SE4600 same time as CS4600-01: Curtis and I will each have a section of CS4600, and my section will be cross-listed with SE4600
    // note all CS cross-listings above

    section!(t, course: "SE 3500-01",
                instructor: "Eric Pedersen",
                rooms and times: "flex");

    section!(t, course: "IT 1200-01",
                instructor: "Jay Sneddon",
                rooms and times: "Smith 107", "tr");
    section!(t, course: "IT 2300-01",
                instructor: "Jay Sneddon",
                rooms and times: "Smith 107");
    section!(t, course: "IT 2700-01",
                instructor: "Jay Sneddon",
                rooms and times: "Smith 107", "tr");
    section!(t, course: "IT 3150-01",
                instructor: "Jay Sneddon",
                rooms and times: "Smith 107", "mw", "mwf" with penalty 5);
    section!(t, course: "IT 3400-01",
                instructor: "Jay Sneddon",
                rooms and times: "Smith 107");

    section!(t, course: "CS 1400-03",
                instructor: "Jeff Compas",
                rooms and times: "stadium");
    section!(t, course: "CS 1400-04",
                instructor: "Jeff Compas",
                rooms and times: "stadium");
    section!(t, course: "CS 2450-02",
                instructor: "Jeff Compas",
                rooms and times: "flex");
    section!(t, course: "SE 3100-01",
                instructor: "Jeff Compas",
                rooms and times: "flex");
    //he needs two morning courses and two evening courses
    //3 credit release for first semester

    section!(t, course: "IT 3110-01",
                instructor: "Joe Francom",
                rooms and times: "flex");
    section!(t, course: "IT 4510-01",
                instructor: "Joe Francom",
                rooms and times: "flex");
    //online section of IT1500 1 credit
    //online section of IT4600 3 credits

    section!(t, course: "SE 3200-01",
                instructor: "Lora Klein",
                rooms and times: "Smith 107" with penalty 5, "flex");
    //course: CS1410 ACE MW 9:30-10:45am, INV 112
    //course: CS1410 ACE MW 12:00-1:15pm, INV 112
    // workload release for program development

    section!(t, course: "SE 3450-01",
                instructor: "Matt Kearl",
                rooms and times: "flex", "macs");
    section!(t, course: "SE 3550-01",
                instructor: "Matt Kearl",
                rooms and times: "flex", "macs");
    section!(t, course: "SE 1400-01",
                instructor: "Matt Kearl",
                rooms and times: "macs");
    //course: SE1400 online
    //course: SE4920 not scheduled

    section!(t, course: "IT 1100-01",
                instructor: "Phil Daley",
                rooms and times: "pcs");
    section!(t, course: "IT 1100-02",
                instructor: "Phil Daley",
                rooms and times: "pcs");
    section!(t, course: "IT 2400-01",
                instructor: "Phil Daley",
                rooms and times: "Smith 107");
    section!(t, course: "IT 3100-01",
                instructor: "Phil Daley",
                rooms and times: "Smith 107");
    // avoid IT 4510 so Phil can shadow Joe
    // 3 credits release for network admin

    section!(t, course: "CS 1400-01",
                instructor: "Ren Quinn",
                rooms and times: "flex");
    section!(t, course: "CS 1400-02",
                instructor: "Ren Quinn",
                rooms and times: "flex");
    section!(t, course: "CS 1410-01",
                instructor: "Ren Quinn",
                rooms and times: "flex");
    section!(t, course: "CS 2450-01",
                instructor: "Ren Quinn",
                rooms and times: "flex");
    section!(t, course: "CS 3150-01",
                instructor: "Ren Quinn",
                rooms and times: "flex");
    section!(t, course: "CS 4991-01",
                instructor: "Ren Quinn",
                rooms and times: "Smith 116", "R1900+50");
    // overload acm
    // course: CS4992 flex F1300 // overload seminar
    // course: CS4800R not scheduled
    // CS4991R is actually R1900-1950

    section!(t, course: "CS 2810-01",
                instructor: "Russ Ross",
                rooms and times: "Smith 109");
    section!(t, course: "CS 2810-02",
                instructor: "Russ Ross",
                rooms and times: "Smith 109");
    section!(t, course: "CS 3410-01",
                instructor: "Russ Ross",
                rooms and times: "Smith 109");
    section!(t, course: "CS 4307-01",
                instructor: "Russ Ross",
                rooms and times: "Smith 109");

    section!(t, course: "SE 1400-02",
                instructor: "Rex Frisbey",
                rooms and times: "macs");

    section!(t, course: "IT 4990-01",
                instructor: "Jamie Bennion",
                rooms and times: "flex");

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
    conflict!(t, set hard, clique: "CS 1400-01", "CS 1400-02", "CS 1400-03", "CS 1400-04");
    conflict!(t, set hard, clique: "CS 1410-01", "CS 1410-02");
    conflict!(t, set hard, clique: "CS 2450-01", "CS 2450-02");
    conflict!(t, set hard, clique: "CS 2810-01", "CS 2810-02");
    conflict!(t, set hard, clique: "IT 1100-01", "IT 1100-02");
    conflict!(t, set hard, clique: "SE 1400-01", "SE 1400-02");

    // courses that must be scheduled at the same time
    anticonflict!(t, set penalty to 50, single: "CS 1030-01", group: "CS 1400");
    //anticonflict!(t, set penalty to 50, clique: "SE 1400", "IT 1100"); // temporarily removed because of new hire planning
    anticonflict!(t, set penalty to 50, single: "CS 4600-01", group: "CS 4600-02");

    Ok(())
}

pub fn input_set(t: &mut Input) -> Result<(), String> {
    room!(t, name: "BROWN 201", capacity: 65);
    room!(t, name: "COE 121", capacity: 50);
    room!(t, name: "HCC 476", capacity: 20);
    room!(t, name: "HURCTR 110", capacity: 20);
    room!(t, name: "INNOV 110", capacity: 30);
    room!(t, name: "INNOV 111", capacity: 30);
    room!(t, name: "INNOV 119", capacity: 30);
    room!(t, name: "INNOV 121", capacity: 30);
    room!(t, name: "SET 101", capacity: 18);
    room!(t, name: "SET 102", capacity: 18);
    room!(t, name: "SET 104", capacity: 40);
    room!(t, name: "SET 105", capacity: 60);
    room!(t, name: "SET 106", capacity: 60);
    room!(t, name: "SET 201", capacity: 65);
    room!(t, name: "SET 213", capacity: 20);
    room!(t, name: "SET 214", capacity: 20);
    room!(t, name: "SET 215", capacity: 20);
    room!(t, name: "SET 216", capacity: 24);
    room!(t, name: "SET 219", capacity: 24);
    room!(t, name: "SET 225", capacity: 20);
    room!(t, name: "SET 226", capacity: 40);
    room!(t, name: "SET 301", capacity: 65);
    room!(t, name: "SET 303", capacity: 6);
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
    room!(t, name: "SET 418", capacity: 48);
    room!(t, name: "SET 420", capacity: 48);
    room!(t, name: "SET 501", capacity: 20);
    room!(t, name: "SET 522", capacity: 24);
    room!(t, name: "SET 523", capacity: 24);
    room!(t, name: "SET 524", capacity: 45);
    room!(t, name: "SET 526", capacity: 24);
    room!(t, name: "SET 527", capacity: 8);
    room!(t, name: "SNOW 103", capacity: 16);
    room!(t, name: "SNOW 112", capacity: 42);
    room!(t, name: "SNOW 113", capacity: 36);
    room!(t, name: "SNOW 124", capacity: 42);
    room!(t, name: "SNOW 125", capacity: 42);
    room!(t, name: "SNOW 128", capacity: 40);
    room!(t, name: "SNOW 144", capacity: 42);
    room!(t, name: "SNOW 145", capacity: 42);
    room!(t, name: "SNOW 147", capacity: 42);
    room!(t, name: "SNOW 150", capacity: 42);
    room!(t, name: "SNOW 151", capacity: 42);
    room!(t, name: "SNOW 204", capacity: 10);
    room!(t, name: "SNOW 208", capacity: 24);
    room!(t, name: "SNOW 216", capacity: 45);
    room!(t, name: "SNOW 3", capacity: 42);
    room!(t, name: "TECH 110", capacity: 30);

    time!(t, name: "F0800+115"); // was 119 minutes
    time!(t, name: "F0800+170");
    time!(t, name: "F0900+110");
    time!(t, name: "F0930+80");
    time!(t, name: "F1000+110");
    time!(t, name: "F1000+50");
    time!(t, name: "F1100+110");
    time!(t, name: "F1100+170");
    time!(t, name: "F1100+50");
    time!(t, name: "F1200+120");
    time!(t, name: "F1200+50");
    time!(t, name: "F1300+110");
    time!(t, name: "F1330+120");
    time!(t, name: "F1330+170");
    time!(t, name: "F1400+170");
    time!(t, name: "M0800+50");
    time!(t, name: "M0900+110");
    time!(t, name: "M1000+110");
    time!(t, name: "M1030+75");
    time!(t, name: "M1100+110");
    time!(t, name: "M1100+170");
    time!(t, name: "M1300+110");
    time!(t, name: "M1300+170");
    time!(t, name: "M1400+180");
    time!(t, name: "M1930+170");
    time!(t, name: "MR1100+110");
    time!(t, name: "MTR0800+80");
    time!(t, name: "MTR1200+80");
    time!(t, name: "MTRF1200+50");
    time!(t, name: "MTWF0900+50");
    time!(t, name: "MTWF1000+50");
    time!(t, name: "MTWF1100+50");
    time!(t, name: "MTWF1300+50");
    time!(t, name: "MTWF1400+50");
    time!(t, name: "MTWR0800+50");
    time!(t, name: "MTWR0900+50");
    time!(t, name: "MTWR1000+50");
    time!(t, name: "MTWR1100+50");
    time!(t, name: "MTWR1200+50");
    time!(t, name: "MTWR1300+50");
    time!(t, name: "MTWR1400+50");
    time!(t, name: "MTWRF0800+50");
    time!(t, name: "MTWRF0900+50");
    time!(t, name: "MTWRF1000+50");
    time!(t, name: "MTWRF1100+50");
    time!(t, name: "MTWRF1200+50");
    time!(t, name: "MTWRF1500+50");
    time!(t, name: "MTWRF1600+50");
    time!(t, name: "MW0600+110");
    time!(t, name: "MW0800+110");
    time!(t, name: "MW0800+80");
    time!(t, name: "MW0930+80");
    time!(t, name: "MW1000+110");
    time!(t, name: "MW1200+110");
    //time!(t, name: "MW1200+75");
    time!(t, name: "MW1200+80");
    time!(t, name: "MW1300+100");
    time!(t, name: "MW1300+110");
    time!(t, name: "MW1330+50");
    //time!(t, name: "MW1330+75");
    time!(t, name: "MW1330+80");
    time!(t, name: "MW1400+110");
    time!(t, name: "MW1500+100");
    time!(t, name: "MW1500+170");
    //time!(t, name: "MW1500+75");
    time!(t, name: "MW1530+75");
    time!(t, name: "MW1600+100");
    time!(t, name: "MW1600+110");
    time!(t, name: "MW1630+100");
    //time!(t, name: "MW1630+75");
    time!(t, name: "MW1645+75");
    time!(t, name: "MW1700+110");
    time!(t, name: "MW1800+100");
    time!(t, name: "MW1800+110");
    time!(t, name: "MW1800+75");
    //time!(t, name: "MWF0800+50");
    time!(t, name: "MWF0800+80");
    //time!(t, name: "MWF0900+50");
    //time!(t, name: "MWF1000+50");
    //time!(t, name: "MWF1100+50");
    time!(t, name: "MWF1200+50");
    time!(t, name: "MWF1330+180");
    time!(t, name: "MWF1330+80");
    time!(t, name: "MWRF0800+50");
    time!(t, name: "MWRF1000+50");
    time!(t, name: "MWRF1100+50");
    time!(t, name: "MWRF1500+50");
    time!(t, name: "R0800+110");
    time!(t, name: "R0800+170");
    time!(t, name: "R0900+110");
    time!(t, name: "R0900+50");
    time!(t, name: "R0900+75");
    time!(t, name: "R0930+120");
    time!(t, name: "R1000+110");
    time!(t, name: "R1000+170");
    time!(t, name: "R1000+50");
    time!(t, name: "R1100+110");
    time!(t, name: "R1100+170");
    time!(t, name: "R1100+50");
    time!(t, name: "R1200+110");
    time!(t, name: "R1200+135");
    time!(t, name: "R1200+170");
    time!(t, name: "R1200+50");
    time!(t, name: "R1300+110");
    time!(t, name: "R1300+170");
    time!(t, name: "R1330+120");
    time!(t, name: "R1330+165");
    time!(t, name: "R1330+75");
    time!(t, name: "R1400+110");
    time!(t, name: "R1400+170");
    time!(t, name: "R1500+110");
    time!(t, name: "R1500+170");
    time!(t, name: "R1530+150");
    time!(t, name: "R1600+110");
    time!(t, name: "R1600+170");
    time!(t, name: "R1630+170");
    time!(t, name: "R1700+170");
    time!(t, name: "R1715+110");
    time!(t, name: "R1800+110");
    time!(t, name: "R1900+170");
    time!(t, name: "S1000+300");
    time!(t, name: "T0700+170");
    time!(t, name: "T0800+110");
    time!(t, name: "T0800+120");
    time!(t, name: "T0800+170");
    time!(t, name: "T0900+110");
    time!(t, name: "T0900+170");
    time!(t, name: "T0900+50");
    time!(t, name: "T1000+110");
    time!(t, name: "T1000+170");
    time!(t, name: "T1030+50");
    time!(t, name: "T1100+110");
    time!(t, name: "T1100+170");
    time!(t, name: "T1200+110");
    time!(t, name: "T1200+170");
    time!(t, name: "T1200+50");
    time!(t, name: "T1300+110");
    time!(t, name: "T1300+170");
    time!(t, name: "T1330+75");
    time!(t, name: "T1400+110");
    time!(t, name: "T1400+170");
    time!(t, name: "T1400+50");
    time!(t, name: "T1500+110");
    time!(t, name: "T1500+170");
    time!(t, name: "T1500+50");
    time!(t, name: "T1500+75");
    time!(t, name: "T1600+110");
    time!(t, name: "T1600+170");
    time!(t, name: "T1600+50");
    time!(t, name: "T1630+150");
    time!(t, name: "T1700+110");
    time!(t, name: "T1700+170");
    time!(t, name: "T1800+110");
    time!(t, name: "T1800+170");
    time!(t, name: "T1900+110");
    time!(t, name: "T1900+170");
    time!(t, name: "T1930+170");
    time!(t, name: "TR0600+110");
    time!(t, name: "TR0730+75");
    time!(t, name: "TR0800+110");
    time!(t, name: "TR0800+80");
    time!(t, name: "TR0815+75");
    //time!(t, name: "TR0900+75");
    time!(t, name: "TR0930+80");
    time!(t, name: "TR0945+75");
    time!(t, name: "TR1000+110");
    time!(t, name: "TR1000+50");
    //time!(t, name: "TR1030+75");
    time!(t, name: "TR1200+110");
    //time!(t, name: "TR1200+75");
    time!(t, name: "TR1200+80");
    time!(t, name: "TR1300+100");
    time!(t, name: "TR1300+110");
    //time!(t, name: "TR1330+75");
    time!(t, name: "TR1400+110");
    time!(t, name: "TR1500+100");
    time!(t, name: "TR1500+170");
    time!(t, name: "TR1500+50");
    //time!(t, name: "TR1500+75");
    time!(t, name: "TR1600+110");
    time!(t, name: "TR1600+170");
    time!(t, name: "TR1630+100");
    //time!(t, name: "TR1630+75");
    time!(t, name: "TR1800+100");
    time!(t, name: "TR1800+110");
    time!(t, name: "TR1800+75");
    time!(t, name: "TW0800+80");
    time!(t, name: "TW0930+80");
    time!(t, name: "TW1200+80");
    time!(t, name: "TW1330+80");
    time!(t, name: "W0800+170");
    time!(t, name: "W0900+110");
    time!(t, name: "W0900+170");
    time!(t, name: "W0900+50");
    time!(t, name: "W1000+170");
    time!(t, name: "W1030+75");
    time!(t, name: "W1100+110");
    time!(t, name: "W1100+170");
    time!(t, name: "W1200+170");
    time!(t, name: "W1200+50");
    time!(t, name: "W1300+110");
    time!(t, name: "W1300+170");
    time!(t, name: "W1330+170");
    time!(t, name: "W1400+170");
    time!(t, name: "W1500+110");
    time!(t, name: "W1500+170");
    time!(t, name: "W1630+150");
    time!(t, name: "W1700+110");
    time!(t, name: "W1700+170");
    time!(t, name: "W1715+110");
    time!(t, name: "W1800+50");
    time!(t, name: "W1930+170");

    instructor!(t,
        name:
            "Alexander R Tye",
        available:
            "F1400+170",
            "R1200+170",
            "TR1500+75",
    );
    instructor!(t,
        name:
            "Amanda Fa'onelua",
        available:
            "TR1300+100",
    );
    instructor!(t,
        name:
            "Amber Rose Mortensen",
        available:
            "MWF0900+50",
            "MWF1000+50",
            "MWF1100+50",
            "TR1030+75",
    );
    instructor!(t,
        name:
            "Andrew C Schiller",
        available:
            "MW1200+75",
            "MW1500+170",
            "T1200+110",
            "TR1500+170",
    );
    instructor!(t,
        name:
            "Andrew Gregory Toth",
        available:
            "MW1200+75",
    );
    instructor!(t,
        name:
            "Bhuvaneswari Sambandham",
        available:
            "MTWF1000+50",
            "MTWR1100+50",
            "MW1200+75",
    );
    instructor!(t,
        name:
            "Bing Jiang",
        available:
            "F1000+110",
            "MW1200+75",
            "MWF0900+50",
            "R1400+110",
            "R1600+110",
    );
    instructor!(t,
        name:
            "Brant A Ross",
        available:
            "MWF1330+180",
    );
    instructor!(t,
        name:
            "Bruford P Reynolds",
        available:
            "TR1000+50",
            "TR1400+110",
    );
    instructor!(t,
        name:
            "Bryan K Stevens",
        available:
            "TR0730+75",
            "TR0900+75",
            "TR1030+75",
    );
    instructor!(t,
        name:
            "Bryce A Clay",
        available:
            "F1200+120",
            "F1330+120",
            "MW0800+80",
            "MW0930+80",
            "MW1200+80",
            "MW1330+80",
            "R0930+120",
            "T0800+120",
    );
    instructor!(t,
        name:
            "Christina M Quinn",
        available:
            "R1000+170",
            "R1300+170",
            "T0700+170",
            "T1000+170",
            "T1300+170",
            "W1300+170",
    );
    instructor!(t,
        name:
            "Christina Pondell",
        available:
            "F1000+50",
            "M1300+170",
            "R1330+165",
            "T1100+110",
            "T1300+110",
            "TR0900+75",
    );
    instructor!(t,
        name:
            "Christopher Kirk DeMacedo",
        available:
            "M1930+170",
            "T1200+110",
            "T1400+110",
    );
    instructor!(t,
        name:
            "Clare C Banks",
        available:
            "MTWR0800+50",
            "MTWR1200+50",
    );
    instructor!(t,
        name:
            "Costel Ionita",
        available:
            "F1100+50",
            "MTWR0800+50",
            "MTWR0900+50",
            "MTWR1100+50",
            "TR1200+75",
    );
    instructor!(t,
        name:
            "Craig D Seegmiller",
        available:
            "MTWR1200+50",
            "TR0730+75",
            "TR0900+75",
    );
    instructor!(t,
        name:
            "Curtis B Walker",
        available:
            "MW1330+75",
            "R1330+75",
            "T1330+75",
            "T1400+170",
            "TR1200+75",
    );
    instructor!(t,
        name:
            "Cutler Cowdin",
        available:
            "R1600+170",
            "T1600+170",
    );
    instructor!(t,
        name:
            "David Brent Christensen",
        available:
            "R0800+110",
            "R1000+110",
            "R1400+110",
            "T1200+110",
    );
    instructor!(t,
        name:
            "David J Burr",
        available:
            "R1900+170",
            "T1600+170",
            "T1900+170",
    );
    instructor!(t,
        name:
            "David M Syndergaard",
        available:
            "M1300+110",
            "MW1630+75",
            "MW1800+75",
    );
    instructor!(t,
        name:
            "David R Black",
        available:
            "T1700+110",
    );
    instructor!(t,
        name:
            "David W Bean",
        available:
            "F1100+170",
            "R1400+170",
            "W1500+170",
    );
    instructor!(t,
        name:
            "Dawn Lashell Kidd-Thomas",
        available:
            "TR1300+100",
    );
    instructor!(t,
        name:
            "Del William Smith",
        available:
            "TR0815+75",
            "TR0945+75",
            "TR1330+75",
            "TR1500+50",
            "TR1600+170",
    );
    instructor!(t,
        name:
            "Diana L Reese",
        available:
            "MTWR0900+50",
            "MTWR1000+50",
            "MTWRF1200+50",
            "MTWRF1600+50",
    );
    instructor!(t,
        name:
            "Divya Singh",
        available:
            "MW1000+110",
            "MW1500+75",
            "MW1630+75",
            "T1200+110",
    );
    instructor!(t,
        name:
            "Donald H Warner",
        available:
            "MW1500+75",
    );
    instructor!(t,
        name:
            "Douglas J Sainsbury",
        available:
            "MTWRF0800+50",
            "TR1200+75",
            "W1200+50",
    );
    instructor!(t,
        name:
            "Elizabeth Karen Ludlow",
        available:
            "MW1300+100",
            "MW1500+75",
    );
    instructor!(t,
        name:
            "Erin E O'Brien",
        available:
            "MW1200+75",
            "T1500+170",
            "W1500+170",
    );
    instructor!(t,
        name:
            "Gabriela Chilom",
        available:
            "MTWR0800+50",
            "MTWR1400+50",
            "MTWRF1500+50",
            "MWF1000+50",
            "R1500+170",
    );
    instructor!(t,
        name:
            "Geoffrey Smith",
        available:
            "MTWR1100+50",
            "TR1500+75",
    );
    instructor!(t,
        name:
            "Glorimar L Aponte-Kline",
        available:
            "TR0900+75",
            "TR1030+75",
            "TR1330+75",
    );
    instructor!(t,
        name:
            "Greg L Melton",
        available:
            "MW1330+75",
            "MW1500+75",
            "T1200+170",
            "TR0900+75",
            "W0900+110",
    );
    instructor!(t,
        name:
            "Hugo Elio Angeles",
        available:
            "TR1800+75",
    );
    instructor!(t,
        name:
            "Hung Yu Shih",
        available:
            "T1300+110",
            "T1500+50",
            "T1600+50",
            "W1330+170",
    );
    instructor!(t,
        name:
            "Jacson Parker",
        available:
            "R1600+170",
            "T1600+170",
    );
    instructor!(t,
        name:
            "James David Meidell",
        available:
            "MW1630+75",
            "R1700+170",
    );
    instructor!(t,
        name:
            "James P Fitzgerald",
        available:
            "MWF0800+50",
            "MWF0900+50",
            "MWF1000+50",
    );
    instructor!(t,
        name:
            "Jameson C Hardy",
        available:
            "MTWR0900+50",
            "MTWRF1000+50",
            "MW1200+75",
            "TR1200+75",
    );
    instructor!(t,
        name:
            "Janice M Hayden",
        available:
            "TR0900+75",
            "W1100+170",
    );
    instructor!(t,
        name:
            "Jared M Hancock",
        available:
            "M1100+110",
            "MTWR0800+50",
            "MTWR0900+50",
            "MTWR1400+50",
            "W1000+170",
    );
    instructor!(t,
        name:
            "Jeffrey Anderson",
        available:
            "MW1630+75",
            "T1400+110",
            "TR0900+75",
    );
    instructor!(t,
        name:
            "Jeffrey P Harrah",
        available:
            "T1630+150",
            "TR1030+75",
            "TR1200+75",
            "TR1330+75",
            "W1630+150",
    );
    instructor!(t,
        name:
            "Jeffrey V Yule",
        available:
            "M1030+75",
            "MWF1100+50",
            "TR1030+75",
            "W1030+75",
    );
    instructor!(t,
        name:
            "Jennifer A Meyer",
        available:
            "MW1200+75",
            "MW1330+75",
            "R1300+170",
            "T1300+170",
    );
    instructor!(t,
        name:
            "Jennifer L Ciaccio",
        available:
            "MTRF1200+50",
            "MWF0900+50",
            "R0900+75",
            "W1200+170",
    );
    instructor!(t,
        name:
            "Jerald D Harris",
        available:
            "MWF1000+50",
            "MWF1100+50",
            "R1000+50",
            "R1630+170",
            "TR1030+75",
    );
    instructor!(t,
        name:
            "Jeremy W Bakelar",
        available:
            "MW1500+75",
            "MWF1100+50",
            "T0900+170",
            "T1500+170",
            "TR1300+110",
    );
    instructor!(t,
        name:
            "Jesse William Breinholt",
        available:
            "TR1500+75",
    );
    instructor!(t,
        name:
            "Jie Liu",
        available:
            "T1500+75",
            "TR1030+75",
            "TR1200+75",
            "TR1330+75",
    );
    instructor!(t,
        name:
            "John E Wolfe",
        available:
            "MWF1100+50",
    );
    instructor!(t,
        name:
            "Jose C Saraiva",
        available:
            "R1600+110",
            "R1800+110",
            "T1600+110",
            "W1930+170",
    );
    instructor!(t,
        name:
            "Joseph B Platt",
        available:
            "R1100+170",
    );
    instructor!(t,
        name:
            "Kameron J Eves",
        available:
            "MW1500+75",
            "MWF1100+50",
            "R1600+110",
            "TR1030+75",
    );
    instructor!(t,
        name:
            "Karen L Bauer",
        available:
            "MTWF1000+50",
            "MTWF1100+50",
            "MWF0800+50",
            "TR1500+75",
    );
    instructor!(t,
        name:
            "Kathryn E Ott",
        available:
            "MW1300+100",
    );
    instructor!(t,
        name:
            "Kerby Robinson",
        available:
            "F1330+170",
    );
    instructor!(t,
        name:
            "Kim C Jolley",
        available:
            "MW1300+110",
            "MW1700+110",
            "S1000+300",
    );
    instructor!(t,
        name:
            "Marius Van der Merwe",
        available:
            "MWF1000+50",
            "T1200+170",
            "W0900+50",
            "W1800+50",
    );
    instructor!(t,
        name:
            "Mark L Dickson",
        available:
            "R1530+150",
    );
    instructor!(t,
        name:
            "Marshall Topham",
        available:
            "MW1330+75",
    );
    instructor!(t,
        name:
            "Martina Gaspari",
        available:
            "MR1100+110",
            "MW1330+75",
            "MWF0900+50",
            "MWF1000+50",
            "R0800+170",
    );
    instructor!(t,
        name:
            "Marzieh Ghasemi",
        available:
            "MW1200+75",
            "MWF1000+50",
            "TR1200+75",
            "TR1500+75",
    );
    instructor!(t,
        name:
            "Matthew S Smith",
        available:
            "MTR0800+80",
            "MTR1200+80",
            "MW1330+80",
            "TR0930+80",
    );
    instructor!(t,
        name:
            "Md Sazib Hasan",
        available:
            "TR0900+75",
            "TR1030+75",
    );
    instructor!(t,
        name:
            "Megan R Liljenquist",
        available:
            "R0930+120",
            "R1330+120",
            "R1600+170",
            "TW0800+80",
            "TW0930+80",
            "TW1200+80",
            "TW1330+80",
            "W1500+170",
    );
    instructor!(t,
        name:
            "Megen E Kepas",
        available:
            "MW1330+75",
            "MW1500+75",
            "R1200+135",
    );
    instructor!(t,
        name:
            "Michael N Paxman",
        available:
            "TR1630+100",
    );
    instructor!(t,
        name:
            "Nathan St Andre",
        available:
            "TR1200+75",
    );
    instructor!(t,
        name:
            "Nikell Dodge",
        available:
            "TR1630+75",
    );
    instructor!(t,
        name:
            "Odean Bowler",
        available:
            "F0930+80",
            "MW1500+100",
            "MWF0800+80",
            "MWF1330+80",
            "TR0800+80",
            "TR1500+100",
    );
    instructor!(t,
        name:
            "Paul H Shirley",
        available:
            "T1600+170",
            "T1900+170",
    );
    instructor!(t,
        name:
            "Paula Manuele Temple",
        available:
            "MTWR1200+50",
            "MW1300+100",
            "MW1500+75",
            "TR1300+100",
    );
    instructor!(t,
        name:
            "Randy Klabacka",
        available:
            "MW1330+50",
            "MWF0900+50",
            "R0900+50",
            "T0900+50",
            "TR1330+75",
    );
    instructor!(t,
        name:
            "Rick L Peirce",
        available:
            "T1930+170",
    );
    instructor!(t,
        name:
            "Rico Del Sesto",
        available:
            "MTWRF0900+50",
            "MTWRF1000+50",
            "MTWRF1100+50",
    );
    instructor!(t,
        name:
            "Rita Rae Osborn",
        available:
            "M0800+50",
    );
    instructor!(t,
        name:
            "Robert T Reimer",
        available:
            "MW1630+75",
    );
    instructor!(t,
        name:
            "Ross C Decker",
        available:
            "TR0900+75",
            "TR1030+75",
    );
    instructor!(t,
        name:
            "Russell C Reid",
        available:
            "MTWF0900+50",
            "MW1500+75",
            "R0800+110",
            "R1000+110",
            "R1200+110",
            "R1400+110",
            "R1600+110",
    );
    instructor!(t,
        name:
            "Ryan C McConnell",
        available:
            "TR1630+75",
    );
    instructor!(t,
        name:
            "Sai C Radavaram",
        available:
            "F0800+115",
            "MW1330+75",
            "MWF1100+50",
            "T0800+110",
            "TR1630+75",
    );
    instructor!(t,
        name:
            "Samuel K Tobler",
        available:
            "MTWF1300+50",
            "MTWF1400+50",
    );
    instructor!(t,
        name:
            "Sarah Morgan Black",
        available:
            "TR1030+75",
            "TR1330+75",
    );
    instructor!(t,
        name:
            "Scott A Skeen",
        available:
            "M0800+50",
            "MW1200+75",
            "MW1330+75",
            "MWF1000+50",
            "R0800+110",
            "R1200+110",
            "TR1500+75",
    );
    instructor!(t,
        name:
            "Scott B Griffin",
        available:
            "F1330+170",
            "MW1200+75",
    );
    instructor!(t,
        name:
            "Scott E Bulloch",
        available:
            "R1600+170",
    );
    instructor!(t,
        name:
            "Scott Patrick Hicks",
        available:
            "MW1600+100",
            "MW1800+100",
    );
    instructor!(t,
        name:
            "Steven K Sullivan",
        available:
            "MWRF0800+50",
            "MWRF1000+50",
            "MWRF1100+50",
    );
    instructor!(t,
        name:
            "Steven McKay Sullivan",
        available:
            "MTWR0900+50",
            "MWF1000+50",
            "TR1030+75",
    );
    instructor!(t,
        name:
            "Teisha Richan",
        available:
            "R1000+170",
            "R1300+170",
            "T0900+170",
            "T1200+170",
            "W0900+170",
            "W1200+170",
    );
    instructor!(t,
        name:
            "Trevor K Johnson",
        available:
            "MTWR1200+50",
            "MW1330+75",
    );
    instructor!(t,
        name:
            "Tye K Rogers",
        available:
            "MTWR0800+50",
            "MTWR1000+50",
            "MWF1100+50",
            "TR1330+75",
    );
    instructor!(t,
        name:
            "Vinodh Kumar Chellamuthu",
        available:
            "MW1500+100",
            "MW1645+75",
    );
    instructor!(t,
        name:
            "Violeta Adina Ionita",
        available:
            "MTWR0800+50",
            "MTWR0900+50",
            "MTWR1100+50",
            "MTWR1200+50",
    );
    instructor!(t,
        name:
            "Wendy E Schatzberg",
        available:
            "F1200+50",
            "MTWR1000+50",
            "MTWR1100+50",
            "MTWRF1200+50",
            "T1600+170",
    );
    instructor!(t,
        name:
            "Zhenyu Jin",
        available:
            "MW1200+75",
            "MW1330+75",
            "T1200+170",
            "TR1030+75",
            "W0900+110",
    );

    // BIOL 1010-01: General Biology (LS)
    section!(t, course: "BIOL 1010-01",
                instructor: "Bryan K Stevens",
                rooms and times: "BROWN 201", "TR0730+75");

    // BIOL 1010-01H: General Biology (LS)
    section!(t, course: "BIOL 1010-01H",
                instructor: "Del William Smith",
                rooms and times: "HURCTR 110", "TR0945+75");

    // BIOL 1010-02: General Biology (LS)
    section!(t, course: "BIOL 1010-02",
                instructor: "Bryan K Stevens",
                rooms and times: "BROWN 201", "TR0900+75");

    // BIOL 1010-03: General Biology (LS)
    section!(t, course: "BIOL 1010-03",
                instructor: "Karen L Bauer",
                rooms and times: "SET 301", "MWF0800+50");

    // BIOL 1010-04: General Biology (LS)
    section!(t, course: "BIOL 1010-04",
                instructor: "Martina Gaspari",
                rooms and times: "COE 121", "MWF1000+50");

    // BIOL 1010-05: General Biology: Supplemental Instruction (LS)
    section!(t, course: "BIOL 1010-05",
                instructor: "Jeffrey V Yule",
                rooms and times: "SET 106", "TR1030+75");

    // BIOL 1010-05-alt: General Biology: Supplemental Instruction (LS)
    section!(t, course: "BIOL 1010-05-alt",
                instructor: "Jeffrey V Yule",
                rooms and times: "SNOW 113", "W1030+75");

    // BIOL 1010-06: General Biology (LS)
    section!(t, course: "BIOL 1010-06",
                instructor: "Jeffrey V Yule",
                rooms and times: "BROWN 201", "MWF1100+50");

    // BIOL 1010-07: General Biology (LS)
    section!(t, course: "BIOL 1010-07",
                instructor: "Nathan St Andre",
                rooms and times: "SET 105", "TR1200+75");

    // BIOL 1010-08: General Biology (LS)
    section!(t, course: "BIOL 1010-08",
                instructor: "Del William Smith",
                rooms and times: "SNOW 151", "TR1330+75");

    // BIOL 1010-09: General Biology (LS)
    section!(t, course: "BIOL 1010-09",
                instructor: "James David Meidell",
                rooms and times: "SET 420", "MW1630+75");

    // BIOL 1010-10: General Biology (LS)
    section!(t, course: "BIOL 1010-10",
                instructor: "Nikell Dodge",
                rooms and times: "SET 301", "TR1630+75");

    // BIOL 1010-11: General Biology (LS)
    section!(t, course: "BIOL 1010-11",
                instructor: "Jeffrey V Yule",
                rooms and times: "SNOW 113", "M1030+75");

    // BIOL 1010-11-alt: General Biology (LS)
    section!(t, course: "BIOL 1010-11-alt",
                instructor: "Jeffrey V Yule",
                rooms and times: "SET 106", "TR1030+75");

    // BIOL 1010-1SJ: General Biology
    section!(t, course: "BIOL 1010-1SJ",
                instructor: "Megan R Liljenquist",
                rooms and times: "TECH 110", "TW0800+80");

    // BIOL 1010-2SJ: General Biology
    section!(t, course: "BIOL 1010-2SJ",
                instructor: "Megan R Liljenquist",
                rooms and times: "TECH 110", "TW1200+80");

    // BIOL 1010-50: General Biology (LS)
    section!(t, course: "BIOL 1010-50",
                rooms and times: "SNOW 112", "TR1800+75");

    // BIOL 1015-03: General Biology Lab (LAB)
    section!(t, course: "BIOL 1015-03",
                rooms and times: "SET 312", "M1100+170");

    // BIOL 1015-04: General Biology Lab (LAB)
    section!(t, course: "BIOL 1015-04",
                rooms and times: "SET 312", "T1100+170");

    // BIOL 1015-05: General Biology Lab (LAB)
    section!(t, course: "BIOL 1015-05",
                rooms and times: "SET 312", "W1100+170");

    // BIOL 1015-07: General Biology Lab (LAB)
    section!(t, course: "BIOL 1015-07",
                rooms and times: "SET 312", "T1400+170");

    // BIOL 1015-51: General Biology Lab (LAB)
    section!(t, course: "BIOL 1015-51",
                rooms and times: "SET 312", "T1700+170");

    // BIOL 1200-01: Human Biology (LS)
    section!(t, course: "BIOL 1200-01",
                instructor: "Amber Rose Mortensen",
                rooms and times: "BROWN 201", "TR1030+75");

    // BIOL 1200-02: Human Biology (LS)
    section!(t, course: "BIOL 1200-02",
                instructor: "Karen L Bauer",
                rooms and times: "SET 105", "TR1500+75");

    // BIOL 1610-01: Principles of Biology I (LS)
    section!(t, course: "BIOL 1610-01",
                instructor: "Douglas J Sainsbury",
                rooms and times: "SET 106", "MTWRF0800+50");

    // BIOL 1610-02: Principles of Biology I (LS)
    section!(t, course: "BIOL 1610-02",
                instructor: "Karen L Bauer",
                rooms and times: "SET 105", "MTWF1100+50");

    // BIOL 1615-01: Principles of Biology I Lab (LAB)
    section!(t, course: "BIOL 1615-01",
                rooms and times: "SET 309", "T0800+170");

    // BIOL 1615-02: Principles of Biology I Lab (LAB)
    section!(t, course: "BIOL 1615-02",
                rooms and times: "SET 309", "W0800+170");

    // BIOL 1615-03: Principles of Biology I Lab (LAB)
    section!(t, course: "BIOL 1615-03",
                rooms and times: "SET 309", "R0800+170");

    // BIOL 1615-04: Principles of Biology I Lab (LAB)
    section!(t, course: "BIOL 1615-04",
                rooms and times: "SET 309", "F0800+170");

    // BIOL 1615-05: Principles of Biology I Lab (LAB)
    section!(t, course: "BIOL 1615-05",
                rooms and times: "SET 309", "T1100+170");

    // BIOL 1615-06: Principles of Biology I Lab (LAB)
    section!(t, course: "BIOL 1615-06",
                rooms and times: "SET 309", "W1100+170");

    // BIOL 1615-07: Principles of Biology I Lab (LAB)
    section!(t, course: "BIOL 1615-07",
                rooms and times: "SET 309", "R1100+170");

    // BIOL 1615-08: Principles of Biology I Lab (LAB)
    section!(t, course: "BIOL 1615-08",
                rooms and times: "SET 309", "F1100+170");

    // BIOL 1615-09: Principles of Biology I Lab (LAB)
    section!(t, course: "BIOL 1615-09",
                rooms and times: "SET 309", "T1400+170");

    // BIOL 1615-10: Principles of Biology I Lab (LAB)
    section!(t, course: "BIOL 1615-10",
                rooms and times: "SET 309", "W1400+170");

    // BIOL 1615-11: Principles of Biology I Lab (LAB)
    section!(t, course: "BIOL 1615-11",
                rooms and times: "SET 309", "R1400+170");

    // BIOL 1615-12: Principles of Biology I Lab (LAB)
    section!(t, course: "BIOL 1615-12",
                rooms and times: "SET 309", "F1400+170");

    // BIOL 1615-50: Principles of Biology I Lab (LAB)
    section!(t, course: "BIOL 1615-50",
                rooms and times: "SET 309", "T1700+170");

    // BIOL 1615-51: Principles of Biology I Lab (LAB)
    section!(t, course: "BIOL 1615-51",
                rooms and times: "SET 309", "W1700+170");

    // BIOL 1615-52: Principles of Biology I Lab (LAB)
    section!(t, course: "BIOL 1615-52",
                rooms and times: "SET 309", "R1700+170");

    // BIOL 1620-01: Principles of Biology II
    section!(t, course: "BIOL 1620-01",
                instructor: "Karen L Bauer",
                rooms and times: "SET 105", "MTWF1000+50");

    // BIOL 1620-02: Principles of Biology II
    section!(t, course: "BIOL 1620-02",
                instructor: "Jennifer L Ciaccio",
                rooms and times: "SET 106", "MTRF1200+50");

    // BIOL 1620-03: Principles of Biology II (HONORS)
    section!(t, course: "BIOL 1620-03",
                instructor: "Geoffrey Smith",
                rooms and times: "SET 216", "MTWR1100+50");

    // BIOL 1625-01: Principles of Biology II Lab
    section!(t, course: "BIOL 1625-01",
                rooms and times: "SET 318", "R0800+170");

    // BIOL 1625-02: Principles of Biology II Lab
    section!(t, course: "BIOL 1625-02",
                instructor: "Joseph B Platt",
                rooms and times: "SET 318", "R1100+170");

    // BIOL 1625-03: Principles of Biology II Lab
    section!(t, course: "BIOL 1625-03",
                instructor: "Jennifer L Ciaccio",
                rooms and times: "SET 318", "W1200+170");

    // BIOL 1625-04: Principles of Biology II Lab
    section!(t, course: "BIOL 1625-04",
                instructor: "David W Bean",
                rooms and times: "SET 318", "R1400+170");

    // BIOL 1625-05: Principles of Biology II Lab
    section!(t, course: "BIOL 1625-05",
                instructor: "David W Bean",
                rooms and times: "SET 318", "F1100+170");

    // BIOL 1625-06: Principles of Biology II Lab
    section!(t, course: "BIOL 1625-06",
                instructor: "David W Bean",
                rooms and times: "SET 318", "W1500+170");

    // BIOL 1625-50: Principles of Biology II Lab
    section!(t, course: "BIOL 1625-50",
                instructor: "James David Meidell",
                rooms and times: "SET 318", "R1700+170");

    // BIOL 2060-01: Principles of Microbiology
    section!(t, course: "BIOL 2060-01",
                instructor: "Jeremy W Bakelar",
                rooms and times: "SET 105", "MW1500+75");

    // BIOL 2065-01: Principles of Microbiology Lab
    section!(t, course: "BIOL 2065-01",
                instructor: "Kim C Jolley",
                rooms and times: "SET 304", "MW1300+110");

    // BIOL 2065-02: Principles of Microbiology Lab
    section!(t, course: "BIOL 2065-02",
                instructor: "Kim C Jolley",
                rooms and times: "SET 304", "MW1700+110");

    // BIOL 2065-03: Principles of Microbiology Lab
    section!(t, course: "BIOL 2065-03",
                instructor: "Kim C Jolley",
                rooms and times: "SET 304", "S1000+300");

    // BIOL 2300-01: Fundamentals of Bioinformatics
    section!(t, course: "BIOL 2300-01",
                instructor: "Randy Klabacka",
                rooms and times: "SET 216", "MW1330+50");

    // BIOL 2320-01: Human Anatomy
    section!(t, course: "BIOL 2320-01",
                rooms and times: "BROWN 201", "MWF1000+50");

    // BIOL 2320-02: Human Anatomy
    section!(t, course: "BIOL 2320-02",
                instructor: "Scott B Griffin",
                rooms and times: "SET 301", "MW1200+75");

    // BIOL 2320-04: Human Anatomy: Supplemental Instruction
    section!(t, course: "BIOL 2320-04",
                instructor: "Curtis B Walker",
                rooms and times: "SET 301", "MW1330+75");

    // BIOL 2320-04-alt: Human Anatomy: Supplemental Instruction
    section!(t, course: "BIOL 2320-04-alt",
                instructor: "Curtis B Walker",
                rooms and times: "SET 105", "T1330+75");

    // BIOL 2320-05: Human Anatomy
    section!(t, course: "BIOL 2320-05",
                instructor: "Glorimar L Aponte-Kline",
                rooms and times: "SET 301", "TR1030+75");

    // BIOL 2320-06: Human Anatomy
    section!(t, course: "BIOL 2320-06",
                rooms and times: "SET 201", "TR1500+75");

    // BIOL 2320-07: Human Anatomy
    section!(t, course: "BIOL 2320-07",
                instructor: "Glorimar L Aponte-Kline",
                rooms and times: "SET 301", "TR1330+75");

    // BIOL 2320-08: Human Anatomy: Supplemental Instruction
    section!(t, course: "BIOL 2320-08",
                instructor: "Curtis B Walker",
                rooms and times: "SET 301", "MW1330+75");

    // BIOL 2320-08-alt: Human Anatomy: Supplemental Instruction
    section!(t, course: "BIOL 2320-08-alt",
                instructor: "Curtis B Walker",
                rooms and times: "SET 105", "R1330+75");

    // BIOL 2325-01: Human Anatomy Lab
    section!(t, course: "BIOL 2325-01",
                rooms and times: "SET 213", "MW0600+110");

    // BIOL 2325-02: Human Anatomy Lab
    section!(t, course: "BIOL 2325-02",
                rooms and times: "SET 215", "TR0600+110");

    // BIOL 2325-03: Human Anatomy Lab
    section!(t, course: "BIOL 2325-03",
                rooms and times: "SET 213", "MW0800+110");

    // BIOL 2325-04: Human Anatomy Lab
    section!(t, course: "BIOL 2325-04",
                rooms and times: "SET 215", "MW0800+110");

    // BIOL 2325-05: Human Anatomy Lab
    section!(t, course: "BIOL 2325-05",
                rooms and times: "SET 213", "TR0800+110");

    // BIOL 2325-06: Human Anatomy Lab
    section!(t, course: "BIOL 2325-06",
                rooms and times: "SET 215", "TR0800+110");

    // BIOL 2325-07: Human Anatomy Lab
    section!(t, course: "BIOL 2325-07",
                rooms and times: "SET 213", "MW1000+110");

    // BIOL 2325-08: Human Anatomy Lab
    section!(t, course: "BIOL 2325-08",
                rooms and times: "SET 215", "MW1000+110");

    // BIOL 2325-09: Human Anatomy Lab
    section!(t, course: "BIOL 2325-09",
                rooms and times: "SET 213", "TR1000+110");

    // BIOL 2325-10: Human Anatomy Lab
    section!(t, course: "BIOL 2325-10",
                rooms and times: "SET 215", "TR1000+110");

    // BIOL 2325-11: Human Anatomy Lab
    section!(t, course: "BIOL 2325-11",
                rooms and times: "SET 213", "MW1200+110");

    // BIOL 2325-12: Human Anatomy Lab
    section!(t, course: "BIOL 2325-12",
                rooms and times: "SET 215", "MW1200+110");

    // BIOL 2325-13: Human Anatomy Lab
    section!(t, course: "BIOL 2325-13",
                rooms and times: "SET 213", "TR1200+110");

    // BIOL 2325-14: Human Anatomy Lab
    section!(t, course: "BIOL 2325-14",
                rooms and times: "SET 215", "TR1200+110");

    // BIOL 2325-15: Human Anatomy Lab
    section!(t, course: "BIOL 2325-15",
                rooms and times: "SET 213", "MW1400+110");

    // BIOL 2325-16: Human Anatomy Lab
    section!(t, course: "BIOL 2325-16",
                rooms and times: "SET 215", "MW1400+110");

    // BIOL 2325-17: Human Anatomy Lab
    section!(t, course: "BIOL 2325-17",
                rooms and times: "SET 213", "TR1400+110");

    // BIOL 2325-18: Human Anatomy Lab
    section!(t, course: "BIOL 2325-18",
                rooms and times: "SET 215", "TR1400+110");

    // BIOL 2325-19: Human Anatomy Lab
    section!(t, course: "BIOL 2325-19",
                rooms and times: "SET 213", "MW1600+110");

    // BIOL 2325-20: Human Anatomy Lab
    section!(t, course: "BIOL 2325-20",
                rooms and times: "SET 215", "MW1600+110");

    // BIOL 2325-21: Human Anatomy Lab
    section!(t, course: "BIOL 2325-21",
                rooms and times: "SET 213", "TR1600+110");

    // BIOL 2325-22: Human Anatomy Lab
    section!(t, course: "BIOL 2325-22",
                rooms and times: "SET 215", "TR1600+110");

    // BIOL 2325-50: Human Anatomy Lab
    section!(t, course: "BIOL 2325-50",
                rooms and times: "SET 213", "MW1800+110");

    // BIOL 2325-51: Human Anatomy Lab
    section!(t, course: "BIOL 2325-51",
                rooms and times: "SET 215", "MW1800+110");

    // BIOL 2325-52: Human Anatomy Lab
    section!(t, course: "BIOL 2325-52",
                rooms and times: "SET 213", "TR1800+110");

    // BIOL 2325-53: Human Anatomy Lab
    section!(t, course: "BIOL 2325-53",
                rooms and times: "SET 215", "TR1800+110");

    // BIOL 2420-01: Human Physiology
    section!(t, course: "BIOL 2420-01",
                instructor: "Amber Rose Mortensen",
                rooms and times: "SET 106", "MWF0900+50");

    // BIOL 2420-02: Human Physiology
    section!(t, course: "BIOL 2420-02",
                instructor: "Amber Rose Mortensen",
                rooms and times: "SET 106", "MWF1000+50");

    // BIOL 2420-03: Human Physiology
    section!(t, course: "BIOL 2420-03",
                instructor: "Amber Rose Mortensen",
                rooms and times: "SET 106", "MWF1100+50");

    // BIOL 2420-04: Human Physiology
    section!(t, course: "BIOL 2420-04",
                instructor: "Megen E Kepas",
                rooms and times: "SET 301", "MW1500+75");

    // BIOL 2420-05: Human Physiology
    section!(t, course: "BIOL 2420-05",
                instructor: "Geoffrey Smith",
                rooms and times: "SET 301", "TR1500+75");

    // BIOL 2425-01: Human Physiology Lab
    section!(t, course: "BIOL 2425-01",
                rooms and times: "SET 214", "T0900+110");

    // BIOL 2425-02: Human Physiology Lab
    section!(t, course: "BIOL 2425-02",
                rooms and times: "SET 214", "W0900+110");

    // BIOL 2425-03: Human Physiology Lab
    section!(t, course: "BIOL 2425-03",
                rooms and times: "SET 214", "R0900+110");

    // BIOL 2425-04: Human Physiology Lab
    section!(t, course: "BIOL 2425-04",
                rooms and times: "SET 214", "F0900+110");

    // BIOL 2425-05: Human Physiology Lab
    section!(t, course: "BIOL 2425-05",
                rooms and times: "SET 214", "T1100+110");

    // BIOL 2425-06: Human Physiology Lab
    section!(t, course: "BIOL 2425-06",
                rooms and times: "SET 214", "W1100+110");

    // BIOL 2425-07: Human Physiology Lab
    section!(t, course: "BIOL 2425-07",
                rooms and times: "SET 214", "R1100+110");

    // BIOL 2425-08: Human Physiology Lab
    section!(t, course: "BIOL 2425-08",
                rooms and times: "SET 214", "F1100+110");

    // BIOL 2425-09: Human Physiology Lab
    section!(t, course: "BIOL 2425-09",
                rooms and times: "SET 214", "T1300+110");

    // BIOL 2425-10: Human Physiology Lab
    section!(t, course: "BIOL 2425-10",
                rooms and times: "SET 214", "W1300+110");

    // BIOL 2425-11: Human Physiology Lab
    section!(t, course: "BIOL 2425-11",
                rooms and times: "SET 214", "R1300+110");

    // BIOL 2425-12: Human Physiology Lab
    section!(t, course: "BIOL 2425-12",
                rooms and times: "SET 214", "F1300+110");

    // BIOL 2425-13: Human Physiology Lab
    section!(t, course: "BIOL 2425-13",
                rooms and times: "SET 214", "T1500+110");

    // BIOL 2425-14: Human Physiology Lab
    section!(t, course: "BIOL 2425-14",
                rooms and times: "SET 214", "W1500+110");

    // BIOL 2425-15: Human Physiology Lab
    section!(t, course: "BIOL 2425-15",
                rooms and times: "SET 214", "R1500+110");

    // BIOL 2425-50: Human Physiology Lab
    section!(t, course: "BIOL 2425-50",
                rooms and times: "SET 214", "T1700+110");

    // BIOL 2425-51: Human Physiology Lab
    section!(t, course: "BIOL 2425-51",
                rooms and times: "SET 214", "W1700+110");

    // BIOL 2991R-01A: Careers in Biology
    section!(t, course: "BIOL 2991R-01A",
                instructor: "Douglas J Sainsbury",
                rooms and times: "SET 501", "W1200+50");

    // BIOL 3000R-09A: Advanced Utah Health Scholars Students
    // xlist entry: HO04
    section!(t, course: "BIOL 3000R-09A",
                instructor: "Rita Rae Osborn",
                rooms and times: "SET 105", "M0800+50");

    // BIOL 3010-01: Evolution
    section!(t, course: "BIOL 3010-01",
                rooms and times: "SET 301", "MWF1100+50");

    // BIOL 3010-01-alt: Evolution
    section!(t, course: "BIOL 3010-01-alt",
                rooms and times: "SET 301", "T1200+50");

    // BIOL 3010-02: Evolution
    section!(t, course: "BIOL 3010-02",
                rooms and times: "SET 301", "MWF1100+50");

    // BIOL 3010-02-alt: Evolution
    section!(t, course: "BIOL 3010-02-alt",
                rooms and times: "SET 301", "R1200+50");

    // BIOL 3030-01: Principles of Genetics: Supplemental Instruction
    section!(t, course: "BIOL 3030-01",
                instructor: "Randy Klabacka",
                rooms and times: "SET 301", "MWF0900+50");

    // BIOL 3030-01-alt: Principles of Genetics: Supplemental Instruction
    section!(t, course: "BIOL 3030-01-alt",
                instructor: "Randy Klabacka",
                rooms and times: "SET 301", "T0900+50");

    // BIOL 3030-02: Genetics
    section!(t, course: "BIOL 3030-02",
                instructor: "Randy Klabacka",
                rooms and times: "SET 301", "MWF0900+50");

    // BIOL 3030-02-alt: Genetics
    section!(t, course: "BIOL 3030-02-alt",
                instructor: "Randy Klabacka",
                rooms and times: "SET 301", "R0900+50");

    // BIOL 3040-01: General Ecology
    section!(t, course: "BIOL 3040-01",
                instructor: "Marius Van der Merwe",
                rooms and times: "SET 301", "MWF1000+50");

    // BIOL 3045-01: General Ecology Lab
    section!(t, course: "BIOL 3045-01",
                instructor: "Marius Van der Merwe",
                rooms and times: "SET 216", "T1200+170");

    // BIOL 3100-01: Bioethics
    // xlist entry: SC0B
    section!(t, course: "BIOL 3100-01",
                instructor: "John E Wolfe",
                rooms and times: "HCC 476", "MWF1100+50");

    // BIOL 3110-01: Scientific Writing
    section!(t, course: "BIOL 3110-01",
                instructor: "Jennifer L Ciaccio",
                rooms and times: "SET 408", "R0900+75");

    // BIOL 3150-01: Biostatistics & the Sci Method
    section!(t, course: "BIOL 3150-01",
                instructor: "Megen E Kepas",
                rooms and times: "SET 106", "MW1330+75");

    // BIOL 3155-01: Scientific Method and Experimental Design
    section!(t, course: "BIOL 3155-01",
                instructor: "Megen E Kepas",
                rooms and times: "SET 216", "R1200+135");

    // BIOL 3155-02: Scientific Method and Experimental Design
    section!(t, course: "BIOL 3155-02",
                instructor: "Erin E O'Brien",
                rooms and times: "SET 216", "T1500+170");

    // BIOL 3230R-01: Cadaver Practicum
    section!(t, course: "BIOL 3230R-01",
                instructor: "Scott B Griffin",
                rooms and times: "SET 213", "F1330+170");

    // BIOL 3230R-02: Cadaver Practicum
    section!(t, course: "BIOL 3230R-02",
                instructor: "Kerby Robinson",
                rooms and times: "SET 215", "F1330+170");

    // BIOL 3250-01: Cancer Biology
    section!(t, course: "BIOL 3250-01",
                instructor: "Martina Gaspari",
                rooms and times: "SET 319", "MW1330+75");

    // BIOL 3300-01: Introduction to Bioinformatics
    section!(t, course: "BIOL 3300-01",
                instructor: "Jesse William Breinholt",
                rooms and times: "SET 501", "TR1500+75");

    // BIOL 3420-01: Advanced Human Physiology
    section!(t, course: "BIOL 3420-01",
                instructor: "Glorimar L Aponte-Kline",
                rooms and times: "SNOW 128", "TR0900+75");

    // BIOL 3450-01: General Microbiology
    section!(t, course: "BIOL 3450-01",
                instructor: "Jeremy W Bakelar",
                rooms and times: "SET 524", "MWF1100+50");

    // BIOL 3455-01: General Microbiology Lab
    section!(t, course: "BIOL 3455-01",
                instructor: "Jeremy W Bakelar",
                rooms and times: "SET 304", "T0900+170");

    // BIOL 3455-02: General Microbiology Lab
    section!(t, course: "BIOL 3455-02",
                instructor: "Jeremy W Bakelar",
                rooms and times: "SET 304", "T1500+170");

    // BIOL 3460-01: Biology of Infectious Disease
    section!(t, course: "BIOL 3460-01",
                instructor: "Donald H Warner",
                rooms and times: "SET 201", "MW1500+75");

    // BIOL 4040-01: Medical Ecology
    section!(t, course: "BIOL 4040-01",
                instructor: "Marius Van der Merwe",
                rooms and times: "SET 501", "W0900+50");

    // BIOL 4200-01: Plant Taxonomy (ALPP)
    section!(t, course: "BIOL 4200-01",
                instructor: "Del William Smith",
                rooms and times: "SNOW 208", "TR1500+50");

    // BIOL 4205-01: Plant Taxonomy Lab (ALPP)
    section!(t, course: "BIOL 4205-01",
                instructor: "Del William Smith",
                rooms and times: "SNOW 208", "TR1600+170");

    // BIOL 4280-01: Marine Biology
    section!(t, course: "BIOL 4280-01",
                instructor: "Jennifer L Ciaccio",
                rooms and times: "SET 318", "MWF0900+50");

    // BIOL 4300-01: Molecular Biology
    section!(t, course: "BIOL 4300-01",
                instructor: "Martina Gaspari",
                rooms and times: "SET 216", "MWF0900+50");

    // BIOL 4305-01: Molecular Biology Laboratory
    section!(t, course: "BIOL 4305-01",
                instructor: "Martina Gaspari",
                rooms and times: "SET 308", "R0800+170");

    // BIOL 4310-01: Advanced Bioinformatics
    section!(t, course: "BIOL 4310-01",
                instructor: "Randy Klabacka",
                rooms and times: "SET 501", "TR1330+75");

    // BIOL 4350-01: Animal Behavior
    section!(t, course: "BIOL 4350-01",
                instructor: "Curtis B Walker",
                rooms and times: "SET 319", "TR1200+75");

    // BIOL 4355-01: Animal Behavior Lab
    section!(t, course: "BIOL 4355-01",
                instructor: "Curtis B Walker",
                rooms and times: "SET 319", "T1400+170");

    // BIOL 4440-01: General Entomology
    section!(t, course: "BIOL 4440-01",
                instructor: "Bryan K Stevens",
                rooms and times: "SNOW 208", "TR1030+75");

    // BIOL 4600-01: Plant Physiology
    section!(t, course: "BIOL 4600-01",
                instructor: "Erin E O'Brien",
                rooms and times: "SET 216", "MW1200+75");

    // BIOL 4605-01: Plant Physiology Lab
    section!(t, course: "BIOL 4605-01",
                instructor: "Erin E O'Brien",
                rooms and times: "SET 216", "W1500+170");

    // BIOL 4810R-01B: Independent Research
    section!(t, course: "BIOL 4810R-01B",
                rooms and times: "SET 303", "M1400+180");

    // BIOL 4890R-50: Life Science Internship
    section!(t, course: "BIOL 4890R-50",
                rooms and times: "SET 501", "W1715+110");

    // BIOL 4890R-51: Life Science Internship
    section!(t, course: "BIOL 4890R-51",
                rooms and times: "SET 501", "R1715+110");

    // BIOL 4910-01: Senior Seminar
    section!(t, course: "BIOL 4910-01",
                rooms and times: "SET 501", "M0800+50");

    // BIOL 4910-02: Senior Seminar
    section!(t, course: "BIOL 4910-02",
                rooms and times: "SET 501", "R1100+50");

    // BIOL 4910-03: Senior Seminar
    section!(t, course: "BIOL 4910-03",
                rooms and times: "SET 501", "T1030+50");

    // BIOL 4990R-02: Seminar in Biology: Dental
    section!(t, course: "BIOL 4990R-02",
                instructor: "Scott E Bulloch",
                rooms and times: "SET 303", "R1600+170");

    // BIOL 4990R-50: Seminar in Biology
    section!(t, course: "BIOL 4990R-50",
                rooms and times: "SET 216", "W1800+50");

    // BTEC 1010-01: Fundamentals of Biotechnology
    section!(t, course: "BTEC 1010-01",
                instructor: "Douglas J Sainsbury",
                rooms and times: "SET 310", "TR1200+75");

    // BTEC 2020-01: Protein Purification and Analysis
    section!(t, course: "BTEC 2020-01",
                instructor: "Jeremy W Bakelar",
                rooms and times: "SET 304", "TR1300+110");

    // BTEC 2030-01: Cell Culture Techniques
    section!(t, course: "BTEC 2030-01",
                instructor: "Martina Gaspari",
                rooms and times: "SET 308", "MR1100+110");

    // BTEC 2050-01: Zebrafish Maintenance & Method
    section!(t, course: "BTEC 2050-01",
                instructor: "Hung Yu Shih",
                rooms and times: "SET 303", "T1300+110");

    // BTEC 2050-01-alt: Zebrafish Maintenance & Method
    section!(t, course: "BTEC 2050-01-alt",
                instructor: "Hung Yu Shih",
                rooms and times: "SET 303", "T1500+50");

    // BTEC 2050-02: Zebrafish Maintenance & Method
    section!(t, course: "BTEC 2050-02",
                instructor: "Hung Yu Shih",
                rooms and times: "SET 303", "T1300+110");

    // BTEC 2050-02-alt: Zebrafish Maintenance & Method
    section!(t, course: "BTEC 2050-02-alt",
                instructor: "Hung Yu Shih",
                rooms and times: "SET 303", "T1600+50");

    // BTEC 3010-01: Sequencing Methods & Technique
    section!(t, course: "BTEC 3010-01",
                rooms and times: "SET 312", "MW1530+75");

    // BTEC 4050-01A: In Situ Hybridization
    section!(t, course: "BTEC 4050-01A",
                instructor: "Hung Yu Shih",
                rooms and times: "SET 303", "W1330+170");

    // CHEM 1010-01: Introduction to Chemistry (PS)
    section!(t, course: "CHEM 1010-01",
                instructor: "Sarah Morgan Black",
                rooms and times: "SNOW 113", "TR1030+75");

    // CHEM 1010-02: Introduction to Chemistry (PS)
    section!(t, course: "CHEM 1010-02",
                instructor: "Sarah Morgan Black",
                rooms and times: "SNOW 113", "TR1330+75");

    // CHEM 1010-1SJ: Intro to Chemistry
    section!(t, course: "CHEM 1010-1SJ",
                instructor: "Megan R Liljenquist",
                rooms and times: "TECH 110", "TW0930+80");

    // CHEM 1010-2SJ: Intro to Chemistry
    section!(t, course: "CHEM 1010-2SJ",
                instructor: "Megan R Liljenquist",
                rooms and times: "TECH 110", "TW1330+80");

    // CHEM 1015-01: Introduction to Chemistry Lab (LAB)
    section!(t, course: "CHEM 1015-01",
                rooms and times: "SET 405", "M0900+110");

    // CHEM 1015-02: Introduction to Chemistry Lab (LAB)
    section!(t, course: "CHEM 1015-02",
                rooms and times: "SET 405", "M1100+110");

    // CHEM 1015-03: Introduction to Chemistry Lab (LAB)
    section!(t, course: "CHEM 1015-03",
                rooms and times: "SET 405", "M1300+110");

    // CHEM 1015-1SJ: Intro to Chemistry Lab
    section!(t, course: "CHEM 1015-1SJ",
                instructor: "Megan R Liljenquist",
                rooms and times: "TECH 110", "R0930+120");

    // CHEM 1015-2SJ: Intro to Chemistry Lab
    section!(t, course: "CHEM 1015-2SJ",
                instructor: "Megan R Liljenquist",
                rooms and times: "TECH 110", "R1330+120");

    // CHEM 1120-01: Elem Organic / Bio Chemistry
    section!(t, course: "CHEM 1120-01",
                instructor: "Jared M Hancock",
                rooms and times: "SNOW 216", "MTWR0900+50");

    // CHEM 1125-01: Elem Organic/Bio Chemistry Lab
    section!(t, course: "CHEM 1125-01",
                instructor: "Jared M Hancock",
                rooms and times: "SET 404", "M1100+110");

    // CHEM 1125-02: Elem Organic/Bio Chemistry Lab
    section!(t, course: "CHEM 1125-02",
                rooms and times: "SET 404", "M1300+110");

    // CHEM 1150-01: Integrated Chemistry for Health Sciences (PS)
    section!(t, course: "CHEM 1150-01",
                instructor: "Jared M Hancock",
                rooms and times: "SET 201", "MTWR0800+50");

    // CHEM 1150-02: Integrated Chemistry for Health Sciences (PS)
    section!(t, course: "CHEM 1150-02",
                instructor: "Jared M Hancock",
                rooms and times: "SET 201", "MTWR1400+50");

    // CHEM 1150-03: Integrated Chemistry for Health Sciences (PS)
    section!(t, course: "CHEM 1150-03",
                rooms and times: "SNOW 216", "MTWR1200+50");

    // CHEM 1155-01: Integrated Chemistry for Health Sciences Laboratory (LAB)
    section!(t, course: "CHEM 1155-01",
                instructor: "Christina M Quinn",
                rooms and times: "SET 405", "T1000+170");

    // CHEM 1155-02: Integrated Chemistry for Health Sciences Laboratory (LAB)
    section!(t, course: "CHEM 1155-02",
                instructor: "Jared M Hancock",
                rooms and times: "SET 407", "W1000+170");

    // CHEM 1155-03: Integrated Chemistry for Health Sciences Laboratory (LAB)
    section!(t, course: "CHEM 1155-03",
                instructor: "Christina M Quinn",
                rooms and times: "SET 407", "W1300+170");

    // CHEM 1155-05: Integrated Chemistry for Health Sciences Laboratory (LAB)
    section!(t, course: "CHEM 1155-05",
                instructor: "Paul H Shirley",
                rooms and times: "SET 405", "T1600+170");

    // CHEM 1155-06: Integrated Chemistry for Health Sciences Laboratory (LAB)
    section!(t, course: "CHEM 1155-06",
                instructor: "Teisha Richan",
                rooms and times: "SET 405", "W0900+170");

    // CHEM 1155-50: Integrated Chemistry for Health Sciences Laboratory (LAB)
    section!(t, course: "CHEM 1155-50",
                instructor: "Paul H Shirley",
                rooms and times: "SET 405", "T1900+170");

    // CHEM 1210-01: Principles of Chemistry I (PS)
    section!(t, course: "CHEM 1210-01",
                instructor: "Diana L Reese",
                rooms and times: "SET 201", "MTWR0900+50");

    // CHEM 1210-02: Principles of Chemistry I (PS)
    section!(t, course: "CHEM 1210-02",
                instructor: "Diana L Reese",
                rooms and times: "SET 201", "MTWR1000+50");

    // CHEM 1210-03: Principles of Chemistry I (PS)
    section!(t, course: "CHEM 1210-03",
                rooms and times: "SNOW 216", "MTWR1300+50");

    // CHEM 1215-01: Principles of Chemistry I Lab (LAB)
    section!(t, course: "CHEM 1215-01",
                instructor: "Christina M Quinn",
                rooms and times: "SET 407", "T0700+170");

    // CHEM 1215-02: Principles of Chemistry I Lab (LAB)
    section!(t, course: "CHEM 1215-02",
                instructor: "Christina M Quinn",
                rooms and times: "SET 409", "R1000+170");

    // CHEM 1215-03: Principles of Chemistry I Lab (LAB)
    section!(t, course: "CHEM 1215-03",
                rooms and times: "SET 407", "R1000+170");

    // CHEM 1215-04: Principles of Chemistry I Lab (LAB)
    section!(t, course: "CHEM 1215-04",
                instructor: "Christina M Quinn",
                rooms and times: "SET 409", "R1300+170");

    // CHEM 1215-05: Principles of Chemistry I Lab (LAB)
    section!(t, course: "CHEM 1215-05",
                instructor: "Megan R Liljenquist",
                rooms and times: "SET 407", "R1600+170");

    // CHEM 1215-06: Principles of Chemistry I Lab (LAB)
    section!(t, course: "CHEM 1215-06",
                instructor: "Jacson Parker",
                rooms and times: "SET 409", "R1600+170");

    // CHEM 1215-50: Principles of Chemistry I Lab (LAB)
    section!(t, course: "CHEM 1215-50",
                instructor: "David J Burr",
                rooms and times: "SET 409", "R1900+170");

    // CHEM 1220-01: Principles of Chemistry II
    section!(t, course: "CHEM 1220-01",
                instructor: "Gabriela Chilom",
                rooms and times: "SET 420", "MTWR0800+50");

    // CHEM 1220-02: Principles of Chemistry II
    section!(t, course: "CHEM 1220-02",
                instructor: "Gabriela Chilom",
                rooms and times: "SNOW 216", "MTWR1400+50");

    // CHEM 1220-03: Principles of Chemistry II
    section!(t, course: "CHEM 1220-03",
                instructor: "Wendy E Schatzberg",
                rooms and times: "SET 420", "MTWR1000+50");

    // CHEM 1225-01: Principles of Chemistry II Lab
    section!(t, course: "CHEM 1225-01",
                rooms and times: "SET 409", "T0700+170");

    // CHEM 1225-02: Principles of Chemistry II Lab
    section!(t, course: "CHEM 1225-02",
                rooms and times: "SET 409", "T1000+170");

    // CHEM 1225-03: Principles of Chemistry II Lab
    section!(t, course: "CHEM 1225-03",
                instructor: "Christina M Quinn",
                rooms and times: "SET 409", "T1300+170");

    // CHEM 1225-04: Principles of Chemistry II Lab
    section!(t, course: "CHEM 1225-04",
                instructor: "David J Burr",
                rooms and times: "SET 407", "T1600+170");

    // CHEM 1225-05: Principles of Chemistry II Lab
    section!(t, course: "CHEM 1225-05",
                instructor: "Jacson Parker",
                rooms and times: "SET 409", "T1600+170");

    // CHEM 1225-50: Principles of Chemistry II Lab
    section!(t, course: "CHEM 1225-50",
                instructor: "David J Burr",
                rooms and times: "SET 407", "T1900+170");

    // CHEM 2310-01: Organic Chemistry I
    section!(t, course: "CHEM 2310-01",
                instructor: "Rico Del Sesto",
                rooms and times: "SET 420", "MTWRF0900+50");

    // CHEM 2310-02: Organic Chemistry I
    section!(t, course: "CHEM 2310-02",
                rooms and times: "SNOW 216", "MTWRF1100+50");

    // CHEM 2315-01: Organic Chemistry I Lab
    section!(t, course: "CHEM 2315-01",
                instructor: "Teisha Richan",
                rooms and times: "SET 404", "R1000+170");

    // CHEM 2315-02: Organic Chemistry I Lab
    section!(t, course: "CHEM 2315-02",
                instructor: "Teisha Richan",
                rooms and times: "SET 404", "R1300+170");

    // CHEM 2320-01: Organic Chemistry II
    section!(t, course: "CHEM 2320-01",
                instructor: "Rico Del Sesto",
                rooms and times: "SET 201", "MTWRF1100+50");

    // CHEM 2320-02: Organic Chemistry II
    section!(t, course: "CHEM 2320-02",
                instructor: "Diana L Reese",
                rooms and times: "SET 420", "MTWRF1200+50");

    // CHEM 2325-01: Organic Chemistry II Lab
    section!(t, course: "CHEM 2325-01",
                instructor: "Teisha Richan",
                rooms and times: "SET 404", "T0900+170");

    // CHEM 2325-02: Organic Chemistry II Lab
    section!(t, course: "CHEM 2325-02",
                instructor: "Teisha Richan",
                rooms and times: "SET 404", "T1200+170");

    // CHEM 2325-03: Organic Chemistry II Lab
    section!(t, course: "CHEM 2325-03",
                rooms and times: "SET 404", "T1500+170");

    // CHEM 2325-04: Organic Chemistry II Lab
    section!(t, course: "CHEM 2325-04",
                rooms and times: "SET 404", "W0900+170");

    // CHEM 2325-05: Organic Chemistry II Lab
    section!(t, course: "CHEM 2325-05",
                instructor: "Teisha Richan",
                rooms and times: "SET 404", "W1200+170");

    // CHEM 2325-06: Organic Chemistry II Lab
    section!(t, course: "CHEM 2325-06",
                instructor: "Megan R Liljenquist",
                rooms and times: "SET 404", "W1500+170");

    // CHEM 2325-50: Organic Chemistry II Lab
    section!(t, course: "CHEM 2325-50",
                rooms and times: "SET 404", "T1800+170");

    // CHEM 3070-01: Physical Chemistry II
    section!(t, course: "CHEM 3070-01",
                instructor: "Wendy E Schatzberg",
                rooms and times: "SET 420", "MTWR1100+50");

    // CHEM 3075-01: Physical Chemistry II Lab
    section!(t, course: "CHEM 3075-01",
                instructor: "Wendy E Schatzberg",
                rooms and times: "SNOW 103", "T1600+170");

    // CHEM 3300-01: Instrumental Analysis
    section!(t, course: "CHEM 3300-01",
                instructor: "Gabriela Chilom",
                rooms and times: "SNOW 216", "MWF1000+50");

    // CHEM 3300-01-alt: Instrumental Analysis
    section!(t, course: "CHEM 3300-01-alt",
                instructor: "Gabriela Chilom",
                rooms and times: "SNOW 103", "R1500+170");

    // CHEM 3510-01: Biochemistry I
    section!(t, course: "CHEM 3510-01",
                instructor: "Jennifer A Meyer",
                rooms and times: "SET 420", "MW1330+75");

    // CHEM 3515-01: Biochemistry I Lab
    section!(t, course: "CHEM 3515-01",
                instructor: "Jennifer A Meyer",
                rooms and times: "SET 308", "R1300+170");

    // CHEM 3515-02: Biochemistry I Lab
    section!(t, course: "CHEM 3515-02",
                instructor: "Cutler Cowdin",
                rooms and times: "SET 308", "R1600+170");

    // CHEM 3520-01: Biochemistry II
    section!(t, course: "CHEM 3520-01",
                instructor: "Jennifer A Meyer",
                rooms and times: "SET 201", "MW1200+75");

    // CHEM 3525-01: Biochemistry II Lab
    section!(t, course: "CHEM 3525-01",
                rooms and times: "SET 308", "T1000+170");

    // CHEM 3525-02: Biochemistry II Lab
    section!(t, course: "CHEM 3525-02",
                instructor: "Jennifer A Meyer",
                rooms and times: "SET 308", "T1300+170");

    // CHEM 3525-03: Biochemistry II Lab
    section!(t, course: "CHEM 3525-03",
                instructor: "Cutler Cowdin",
                rooms and times: "SET 308", "T1600+170");

    // CHEM 4800R-01: Independent Research
    section!(t, course: "CHEM 4800R-01",
                instructor: "Rico Del Sesto",
                rooms and times: "SNOW 204", "MTWRF1000+50");

    // CHEM 4800R-02: Independent Research
    section!(t, course: "CHEM 4800R-02",
                instructor: "Wendy E Schatzberg",
                rooms and times: "SNOW 204", "MTWRF1200+50");

    // CHEM 4800R-03: Independent Research
    section!(t, course: "CHEM 4800R-03",
                rooms and times: "SNOW 204", "MTWRF1100+50");

    // CHEM 4800R-04: Independent Research
    section!(t, course: "CHEM 4800R-04",
                instructor: "Gabriela Chilom",
                rooms and times: "SNOW 204", "MTWRF1500+50");

    // CHEM 4800R-06: Independent Research
    section!(t, course: "CHEM 4800R-06",
                instructor: "Diana L Reese",
                rooms and times: "SNOW 204", "MTWRF1600+50");

    // CHEM 4910-01: Chemistry Senior Seminar
    section!(t, course: "CHEM 4910-01",
                instructor: "Wendy E Schatzberg",
                rooms and times: "SET 201", "F1200+50");

    // ECE 2100-01: Semiconductor Devices
    section!(t, course: "ECE 2100-01",
                instructor: "Andrew Gregory Toth",
                rooms and times: "SET 102", "MW1200+75");

    // ECE 2280-01: Microelectronics
    section!(t, course: "ECE 2280-01",
                instructor: "Sai C Radavaram",
                rooms and times: "SET 102", "MWF1100+50");

    // ECE 2285-01: Microelectronics Lab
    section!(t, course: "ECE 2285-01",
                instructor: "Sai C Radavaram",
                rooms and times: "SET 101", "T0800+110");

    // ECE 3500-01: Signals and Systems
    section!(t, course: "ECE 3500-01",
                instructor: "Kameron J Eves",
                rooms and times: "SET 523", "MW1500+75");

    // ECE 3600-01: Power Electronics
    section!(t, course: "ECE 3600-01",
                instructor: "Sai C Radavaram",
                rooms and times: "SET 523", "MW1330+75");

    // ECE 3605-01: Power Electronics Lab
    section!(t, course: "ECE 3605-01",
                instructor: "David Brent Christensen",
                rooms and times: "SET 101", "T1200+110");

    // ECE 4010-01: EE Product Design II
    section!(t, course: "ECE 4010-01",
                instructor: "Brant A Ross",
                rooms and times: "SET 219", "MWF1330+180");

    // ECE 4510-01: Image Processing
    section!(t, course: "ECE 4510-01",
                instructor: "Jeffrey Anderson",
                rooms and times: "SET 523", "TR0900+75");

    // ECE 4730-01: Embedded Systems II
    section!(t, course: "ECE 4730-01",
                instructor: "Jeffrey Anderson",
                rooms and times: "SET 523", "MW1630+75");

    // ECE 4735-01: Embedded Systems II Lab
    section!(t, course: "ECE 4735-01",
                instructor: "Jeffrey Anderson",
                rooms and times: "SET 101", "T1400+110");

    // ECE 4990-01: Special Topics: Human-Machine Interfacing
    section!(t, course: "ECE 4990-01",
                instructor: "Bing Jiang",
                rooms and times: "SET 101", "F1000+110");

    // ECE 4990-01-alt: Special Topics: Human-Machine Interfacing
    section!(t, course: "ECE 4990-01-alt",
                instructor: "Bing Jiang",
                rooms and times: "SET 523", "MW1200+75");

    // ECE 4990-02: Special Topics: Autopilot
    section!(t, course: "ECE 4990-02",
                instructor: "Kameron J Eves",
                rooms and times: "SET 523", "TR1030+75");

    // ECE 4990-03: Special Topics: Antenna Engineering
    section!(t, course: "ECE 4990-03",
                instructor: "Sai C Radavaram",
                rooms and times: "SET 101", "F0800+115");

    // ECE 4990-03-alt: Special Topics: Antenna Engineering
    section!(t, course: "ECE 4990-03-alt",
                instructor: "Sai C Radavaram",
                rooms and times: "SET 523", "TR1630+75");

    // ENVS 1010-01: Intro to Environmental Science (PS)
    section!(t, course: "ENVS 1010-01",
                rooms and times: "SET 524", "TR1200+75");

    // ENVS 1010-03: Intro to Environmental Science (PS)
    section!(t, course: "ENVS 1010-03",
                rooms and times: "SET 524", "TR1330+75");

    // ENVS 1010-04: Intro to Environmental Science (PS)
    section!(t, course: "ENVS 1010-04",
                instructor: "Greg L Melton",
                rooms and times: "SET 524", "MW1330+75");

    // ENVS 1010-05: Intro to Environmental Science (PS)
    section!(t, course: "ENVS 1010-05",
                rooms and times: "SNOW 113", "TR1500+75");

    // ENVS 1010-06: Intro to Environmental Science (PS)
    section!(t, course: "ENVS 1010-06",
                instructor: "Marshall Topham",
                rooms and times: "SNOW 113", "MW1330+75");

    // ENVS 1010-07: Intro to Environmental Science (PS)
    section!(t, course: "ENVS 1010-07",
                rooms and times: "SNOW 128", "TR1330+75");

    // ENVS 1099-01: Recitation for Majors
    section!(t, course: "ENVS 1099-01",
                instructor: "Christina Pondell",
                rooms and times: "SET 526", "F1000+50");

    // ENVS 1210-01: Introduction to Environmental Science
    section!(t, course: "ENVS 1210-01",
                instructor: "Marzieh Ghasemi",
                rooms and times: "SNOW 113", "TR1200+75");

    // ENVS 1215-01: Introduction to Environmental Science Laboratory
    section!(t, course: "ENVS 1215-01",
                instructor: "Christina Pondell",
                rooms and times: "SET 526", "M1300+170");

    // ENVS 1215-02: Introduction to Environmental Science Laboratory
    section!(t, course: "ENVS 1215-02",
                instructor: "Christina Pondell",
                rooms and times: "SET 526", "R1330+165");

    // ENVS 2099R-50: Special Topics in Environmental Science: The Geology of Foundation Engineering in Southern Utah
    section!(t, course: "ENVS 2099R-50",
                instructor: "Hugo Elio Angeles",
                rooms and times: "SET 526", "TR1800+75");

    // ENVS 2210-01: Environmental Pollution and Remediation Techniques
    section!(t, course: "ENVS 2210-01",
                instructor: "Marzieh Ghasemi",
                rooms and times: "SNOW 128", "MW1200+75");

    // ENVS 2700R-01: Field Methods EnvSci
    section!(t, course: "ENVS 2700R-01",
                instructor: "Alexander R Tye",
                rooms and times: "SET 527", "F1400+170");

    // ENVS 3110-01: Scientific Writing
    section!(t, course: "ENVS 3110-01",
                instructor: "Jerald D Harris",
                rooms and times: "SET 408", "MWF1100+50");

    // ENVS 3210-01: Soil Science
    section!(t, course: "ENVS 3210-01",
                instructor: "Christina Pondell",
                rooms and times: "SET 526", "TR0900+75");

    // ENVS 3280-50: Environmental Law
    section!(t, course: "ENVS 3280-50",
                rooms and times: "SNOW 128", "TR1800+110");

    // ENVS 3410-01: Air Quality and Control
    section!(t, course: "ENVS 3410-01",
                instructor: "Marzieh Ghasemi",
                rooms and times: "SET 522", "MWF1000+50");

    // ENVS 3920-50: Peruvian Amazon Natural Histor
    section!(t, course: "ENVS 3920-50",
                instructor: "Marius Van der Merwe",
                rooms and times: "SNOW 113", "W1800+50");

    // ENVS 4910-01: Senior Seminar
    section!(t, course: "ENVS 4910-01",
                rooms and times: "SET 408", "F1200+50");

    // GEO 1010-01: Introduction to Geology (PS)
    section!(t, course: "GEO 1010-01",
                instructor: "Greg L Melton",
                rooms and times: "SET 524", "TR0900+75");

    // GEO 1010-04H: Introduction to Geology (PS)
    section!(t, course: "GEO 1010-04H",
                instructor: "Del William Smith",
                rooms and times: "HURCTR 110", "TR0815+75");

    // GEO 1010-50: Introduction to Geology (PS)
    section!(t, course: "GEO 1010-50",
                rooms and times: "SNOW 128", "MW1800+75");

    // GEO 1015-01: Introduction to Geology Lab (LAB)
    section!(t, course: "GEO 1015-01",
                instructor: "Greg L Melton",
                rooms and times: "SET 527", "W0900+110");

    // GEO 1015-03: Introduction to Geology Lab (LAB)
    section!(t, course: "GEO 1015-03",
                rooms and times: "SET 527", "T1100+110");

    // GEO 1015-04: Introduction to Geology Lab (LAB)
    section!(t, course: "GEO 1015-04",
                rooms and times: "SET 527", "T1500+110");

    // GEO 1015-50: Introduction to Geology Lab (LAB)
    section!(t, course: "GEO 1015-50",
                instructor: "David R Black",
                rooms and times: "SET 527", "T1700+110");

    // GEO 1015-51: Introduction to Geology Lab (LAB)
    section!(t, course: "GEO 1015-51",
                rooms and times: "SET 527", "T1900+110");

    // GEO 1050-01: Geology of the National Parks (PS)
    section!(t, course: "GEO 1050-01",
                rooms and times: "SET 527", "W1100+110");

    // GEO 1110-01: Physical Geology (PS)
    section!(t, course: "GEO 1110-01",
                instructor: "Janice M Hayden",
                rooms and times: "SET 522", "TR0900+75");

    // GEO 1115-01: Physical Geology Lab
    section!(t, course: "GEO 1115-01",
                instructor: "Janice M Hayden",
                rooms and times: "SET 522", "W1100+170");

    // GEO 1220-01: Historical Geology
    section!(t, course: "GEO 1220-01",
                instructor: "Jerald D Harris",
                rooms and times: "SET 522", "TR1030+75");

    // GEO 1225-01: Historical Geology Lab
    section!(t, course: "GEO 1225-01",
                instructor: "Jerald D Harris",
                rooms and times: "SET 522", "R1630+170");

    // GEO 2700R-01: Field Methods in Geoscience Research
    section!(t, course: "GEO 2700R-01",
                instructor: "Alexander R Tye",
                rooms and times: "SET 527", "F1400+170");

    crosslist!(t, "GEO 2700R-01" cross-list with "ENVS 2700R-01");

    // GEO 3110-01: Scientific Writing
    section!(t, course: "GEO 3110-01",
                instructor: "Jerald D Harris",
                rooms and times: "SET 408", "MWF1100+50");

    crosslist!(t, "ENVS 3110-01" cross-list with "GEO 3110-01");

    // GEO 3500-01: Geomorphology
    section!(t, course: "GEO 3500-01",
                instructor: "Alexander R Tye",
                rooms and times: "SET 408", "R1200+170");

    // GEO 3500-01-alt: Geomorphology
    section!(t, course: "GEO 3500-01-alt",
                instructor: "Alexander R Tye",
                rooms and times: "SET 408", "TR1500+75");

    // GEO 3600-01: Ig/Met Petrology
    section!(t, course: "GEO 3600-01",
                instructor: "Greg L Melton",
                rooms and times: "SET 522", "MW1500+75");

    // GEO 3600-01-alt: Ig/Met Petrology
    section!(t, course: "GEO 3600-01-alt",
                instructor: "Greg L Melton",
                rooms and times: "SET 522", "T1200+170");

    // GEO 3710-01: Hydrology
    section!(t, course: "GEO 3710-01",
                instructor: "Marzieh Ghasemi",
                rooms and times: "SET 524", "TR1500+75");

    // GEO 4000R-01: Selected Geology Field Excursions
    section!(t, course: "GEO 4000R-01",
                rooms and times: "SET 527", "F1100+50");

    // GEO 4910-01: Senior Seminar
    section!(t, course: "GEO 4910-01",
                rooms and times: "SNOW 216", "F1200+50");

    // GEOG 1000-01: Physical Geography: Supplemental Instruction (PS)
    section!(t, course: "GEOG 1000-01",
                instructor: "Jerald D Harris",
                rooms and times: "SET 524", "MWF1000+50");

    // GEOG 1000-01-alt: Physical Geography: Supplemental Instruction (PS)
    section!(t, course: "GEOG 1000-01-alt",
                instructor: "Jerald D Harris",
                rooms and times: "SNOW 216", "R1000+50");

    // GEOG 1000-02: Physical Geography (PS)
    section!(t, course: "GEOG 1000-02",
                instructor: "Zhenyu Jin",
                rooms and times: "SET 524", "MW1200+75");

    // GEOG 1000-03: Physical Geography (PS)
    section!(t, course: "GEOG 1000-03",
                rooms and times: "SNOW 113", "TR0900+75");

    // GEOG 1005-01: Physical Geography Lab (LAB)
    section!(t, course: "GEOG 1005-01",
                instructor: "Christina Pondell",
                rooms and times: "SET 526", "T1100+110");

    // GEOG 1005-02: Physical Geography Lab (LAB)
    section!(t, course: "GEOG 1005-02",
                instructor: "Christina Pondell",
                rooms and times: "SET 526", "T1300+110");

    // GEOG 1005-03: Physical Geography Lab (LAB)
    section!(t, course: "GEOG 1005-03",
                instructor: "Zhenyu Jin",
                rooms and times: "SET 526", "W0900+110");

    // GEOG 1005-04: Physical Geography Lab (LAB)
    section!(t, course: "GEOG 1005-04",
                rooms and times: "SET 526", "W1100+110");

    // GEOG 1005-05: Physical Geography Lab (LAB)
    section!(t, course: "GEOG 1005-05",
                rooms and times: "SET 526", "R1100+110");

    // GEOG 3600-01: Introduction to Geographic Information Systems
    section!(t, course: "GEOG 3600-01",
                instructor: "Zhenyu Jin",
                rooms and times: "SET 408", "TR1030+75");

    // GEOG 3605-01: Introduction to Geographic Information Systems Laboratory
    section!(t, course: "GEOG 3605-01",
                instructor: "Zhenyu Jin",
                rooms and times: "SET 408", "T1200+170");

    // GEOG 4180-01: Geoprocessing with Python
    section!(t, course: "GEOG 4180-01",
                instructor: "Zhenyu Jin",
                rooms and times: "SET 408", "MW1330+75");

    // MATH 1010-03: Intermediate Algebra
    section!(t, course: "MATH 1010-03",
                instructor: "Violeta Adina Ionita",
                rooms and times: "SNOW 3", "MTWR1100+50");

    // MATH 1010-04: Intermediate Algebra
    section!(t, course: "MATH 1010-04",
                instructor: "Elizabeth Karen Ludlow",
                rooms and times: "SNOW 145", "MW1300+100");

    // MATH 1010-05: Intermediate Algebra
    section!(t, course: "MATH 1010-05",
                instructor: "Odean Bowler",
                rooms and times: "SNOW 145", "TR1500+100");

    // MATH 1010-06: Intermediate Algebra
    section!(t, course: "MATH 1010-06",
                instructor: "Odean Bowler",
                rooms and times: "SNOW 145", "MW1500+100");

    // MATH 1010-07: Intermediate Algebra
    section!(t, course: "MATH 1010-07",
                instructor: "Violeta Adina Ionita",
                rooms and times: "SNOW 3", "MTWR1200+50");

    // MATH 1010-1SJ: Intermediate Algebra
    section!(t, course: "MATH 1010-1SJ",
                instructor: "Odean Bowler",
                rooms and times: "INNOV 110", "F0930+80");

    // MATH 1010-1SJ-alt: Intermediate Algebra
    section!(t, course: "MATH 1010-1SJ-alt",
                instructor: "Odean Bowler",
                rooms and times: "INNOV 110", "TR0800+80");

    // MATH 1010-2SJ: Intermediate Algebra
    section!(t, course: "MATH 1010-2SJ",
                rooms and times: "INNOV 110", "TR1200+80");

    // MATH 1010-50: Intermediate Algebra
    section!(t, course: "MATH 1010-50",
                rooms and times: "SNOW 147", "TR1800+100");

    // MATH 1030-01: Quantitative Reasoning (MA)
    section!(t, course: "MATH 1030-01",
                instructor: "Elizabeth Karen Ludlow",
                rooms and times: "SNOW 125", "MW1500+75");

    // MATH 1030-02: Quantitative Reasoning (MA)
    section!(t, course: "MATH 1030-02",
                instructor: "Craig D Seegmiller",
                rooms and times: "SNOW 124", "TR0730+75");

    // MATH 1030-03: Quantitative Reasoning (MA)
    section!(t, course: "MATH 1030-03",
                instructor: "Craig D Seegmiller",
                rooms and times: "SNOW 124", "TR0900+75");

    // MATH 1030-04: Quantitative Reasoning (MA)
    section!(t, course: "MATH 1030-04",
                rooms and times: "SNOW 125", "MW1330+75");

    // MATH 1030-05: Quantitative Reasoning (MA)
    section!(t, course: "MATH 1030-05",
                instructor: "Jeffrey P Harrah",
                rooms and times: "SNOW 150", "TR1200+75");

    // MATH 1030-06: Quantitative Reasoning (MA)
    section!(t, course: "MATH 1030-06",
                instructor: "Jeffrey P Harrah",
                rooms and times: "SNOW 150", "TR1330+75");

    // MATH 1040-01: Introduction to Statistics (MA)
    section!(t, course: "MATH 1040-01",
                instructor: "James P Fitzgerald",
                rooms and times: "SNOW 124", "MWF0800+50");

    // MATH 1040-02: Introduction to Statistics (MA)
    section!(t, course: "MATH 1040-02",
                instructor: "James P Fitzgerald",
                rooms and times: "SNOW 124", "MWF0900+50");

    // MATH 1040-03: Introduction to Statistics (MA)
    section!(t, course: "MATH 1040-03",
                instructor: "James P Fitzgerald",
                rooms and times: "SNOW 124", "MWF1000+50");

    // MATH 1040-04: Introduction to Statistics (MA)
    section!(t, course: "MATH 1040-04",
                rooms and times: "SNOW 124", "MWF1200+50");

    // MATH 1040-05: Introduction to Statistics (MA)
    section!(t, course: "MATH 1040-05",
                instructor: "Tye K Rogers",
                rooms and times: "SNOW 124", "MWF1100+50");

    // MATH 1040-06: Introduction to Statistics (MA)
    section!(t, course: "MATH 1040-06",
                instructor: "Tye K Rogers",
                rooms and times: "SNOW 125", "TR1330+75");

    // MATH 1040-07: Introduction to Statistics (MA)
    section!(t, course: "MATH 1040-07",
                instructor: "Jameson C Hardy",
                rooms and times: "SNOW 151", "TR1200+75");

    // MATH 1040-08: Introduction to Statistics (MA)
    section!(t, course: "MATH 1040-08",
                instructor: "Paula Manuele Temple",
                rooms and times: "SNOW 124", "MW1500+75");

    // MATH 1040-09: Introduction to Statistics (MA)
    section!(t, course: "MATH 1040-09",
                instructor: "Jameson C Hardy",
                rooms and times: "SNOW 150", "MW1200+75");

    // MATH 1040-10: Introduction to Statistics (MA)
    section!(t, course: "MATH 1040-10",
                instructor: "Jie Liu",
                rooms and times: "SNOW 124", "TR1200+75");

    // MATH 1040-11: Introduction to Statistics (MA)
    section!(t, course: "MATH 1040-11",
                instructor: "Ryan C McConnell",
                rooms and times: "SNOW 124", "TR1630+75");

    // MATH 1040-12: Introduction to Statistics (MA)
    section!(t, course: "MATH 1040-12",
                rooms and times: "SNOW 125", "TR1630+75");

    // MATH 1040-14: Introduction to Statistics (MA)
    section!(t, course: "MATH 1040-14",
                instructor: "Robert T Reimer",
                rooms and times: "SNOW 124", "MW1630+75");

    // MATH 1050-01: College Algebra / Pre-Calculus (MA)
    section!(t, course: "MATH 1050-01",
                instructor: "Violeta Adina Ionita",
                rooms and times: "SNOW 3", "MTWR0800+50");

    // MATH 1050-02: College Algebra / Pre-Calculus (MA)
    section!(t, course: "MATH 1050-02",
                instructor: "Violeta Adina Ionita",
                rooms and times: "SNOW 3", "MTWR0900+50");

    // MATH 1050-03: College Algebra / Pre-Calculus: Supplemental Instruction (MA)
    section!(t, course: "MATH 1050-03",
                instructor: "Costel Ionita",
                rooms and times: "SNOW 125", "F1100+50");

    // MATH 1050-03-alt: College Algebra / Pre-Calculus: Supplemental Instruction (MA)
    section!(t, course: "MATH 1050-03-alt",
                instructor: "Costel Ionita",
                rooms and times: "SNOW 125", "MTWR1100+50");

    // MATH 1050-04: College Algebra / Pre-Calculus (MA)
    section!(t, course: "MATH 1050-04",
                instructor: "Clare C Banks",
                rooms and times: "SNOW 147", "MTWR1200+50");

    // MATH 1050-05: College Algebra / Pre-Calculus (MA)
    section!(t, course: "MATH 1050-05",
                instructor: "Dawn Lashell Kidd-Thomas",
                rooms and times: "SNOW 145", "TR1300+100");

    // MATH 1050-06: College Algebra / Pre-Calculus (MA)
    section!(t, course: "MATH 1050-06",
                instructor: "Craig D Seegmiller",
                rooms and times: "SNOW 112", "MTWR1200+50");

    // MATH 1050-1SJ: College Algebra / Pre-Calculus
    section!(t, course: "MATH 1050-1SJ",
                instructor: "Matthew S Smith",
                rooms and times: "INNOV 121", "MTR0800+80");

    // MATH 1050-2SJ: College Algebra / Pre-Calculus
    section!(t, course: "MATH 1050-2SJ",
                instructor: "Matthew S Smith",
                rooms and times: "INNOV 121", "MTR1200+80");

    // MATH 1050-3SJ: College Algebra / Pre-Calculus
    section!(t, course: "MATH 1050-3SJ",
                instructor: "Odean Bowler",
                rooms and times: "INNOV 110", "MWF0800+80");

    // MATH 1050-4SJ: College Algebra / Pre-Calculus
    section!(t, course: "MATH 1050-4SJ",
                instructor: "Odean Bowler",
                rooms and times: "INNOV 110", "MWF1330+80");

    // MATH 1060-01: Trigonometry (MA)
    section!(t, course: "MATH 1060-01",
                instructor: "Ross C Decker",
                rooms and times: "SNOW 147", "TR0900+75");

    // MATH 1060-02: Trigonometry (MA)
    section!(t, course: "MATH 1060-02",
                instructor: "Ross C Decker",
                rooms and times: "SNOW 147", "TR1030+75");

    // MATH 1060-1SJ: Trigonometry
    section!(t, course: "MATH 1060-1SJ",
                instructor: "Matthew S Smith",
                rooms and times: "INNOV 121", "TR0930+80");

    // MATH 1060-2SJ: Trigonometry
    section!(t, course: "MATH 1060-2SJ",
                instructor: "Matthew S Smith",
                rooms and times: "INNOV 121", "MW1330+80");

    // MATH 1080-01: Pre-Calculus with Trigonometry (MA)
    section!(t, course: "MATH 1080-01",
                instructor: "Jameson C Hardy",
                rooms and times: "SNOW 145", "MTWRF1000+50");

    // MATH 1100-02: Business Calculus (MA)
    section!(t, course: "MATH 1100-02",
                instructor: "Trevor K Johnson",
                rooms and times: "SNOW 124", "MW1330+75");

    // MATH 1210-01: Calculus I (MA)
    section!(t, course: "MATH 1210-01",
                instructor: "Trevor K Johnson",
                rooms and times: "SNOW 145", "MTWR1200+50");

    // MATH 1210-02: Calculus I (MA)
    section!(t, course: "MATH 1210-02",
                instructor: "Costel Ionita",
                rooms and times: "SNOW 125", "MTWR0800+50");

    // MATH 1210-03: Calculus I (MA)
    section!(t, course: "MATH 1210-03",
                instructor: "Bhuvaneswari Sambandham",
                rooms and times: "SNOW 145", "MTWR1100+50");

    // MATH 1220-01: Calculus II (MA)
    section!(t, course: "MATH 1220-01",
                instructor: "Clare C Banks",
                rooms and times: "SNOW 147", "MTWR0800+50");

    // MATH 1220-02: Calculus II (MA)
    section!(t, course: "MATH 1220-02",
                instructor: "Costel Ionita",
                rooms and times: "SNOW 125", "MTWR0900+50");

    // MATH 2010-01: Math for Elementary Teachers I
    section!(t, course: "MATH 2010-01",
                instructor: "Jeffrey P Harrah",
                rooms and times: "SNOW 150", "T1630+150");

    // MATH 2020-01: Math for Elemen Teachers II
    section!(t, course: "MATH 2020-01",
                instructor: "Jeffrey P Harrah",
                rooms and times: "SNOW 150", "TR1030+75");

    // MATH 2020-02: Math for Elemen Teachers II
    section!(t, course: "MATH 2020-02",
                instructor: "Jeffrey P Harrah",
                rooms and times: "SNOW 150", "W1630+150");

    // MATH 2200-01: Discrete Mathematics
    section!(t, course: "MATH 2200-01",
                instructor: "Steven McKay Sullivan",
                rooms and times: "SNOW 112", "TR1030+75");

    // MATH 2210-01: Multivariable Calculus (MA)
    section!(t, course: "MATH 2210-01",
                instructor: "Steven McKay Sullivan",
                rooms and times: "SNOW 112", "MTWR0900+50");

    // MATH 2250-01: Differential Equations and Linear Algebra
    section!(t, course: "MATH 2250-01",
                instructor: "Bhuvaneswari Sambandham",
                rooms and times: "SNOW 125", "MTWF1000+50");

    // MATH 2270-01: Linear Algebra
    section!(t, course: "MATH 2270-01",
                instructor: "Md Sazib Hasan",
                rooms and times: "SNOW 151", "TR0900+75");

    // MATH 2280-01: Ordinary Differential Equations
    section!(t, course: "MATH 2280-01",
                instructor: "Bhuvaneswari Sambandham",
                rooms and times: "SNOW 151", "MW1200+75");

    // MATH 3050-01: Stochastic Modeling and Applications
    section!(t, course: "MATH 3050-01",
                instructor: "Md Sazib Hasan",
                rooms and times: "SNOW 151", "TR1030+75");

    // MATH 3200-01: Introduction to Analysis I
    section!(t, course: "MATH 3200-01",
                instructor: "Costel Ionita",
                rooms and times: "SNOW 125", "TR1200+75");

    // MATH 3450-01: Statistical Inference
    section!(t, course: "MATH 3450-01",
                instructor: "Jie Liu",
                rooms and times: "SNOW 124", "TR1030+75");

    // MATH 3900-01: Number Theory
    section!(t, course: "MATH 3900-01",
                instructor: "Steven McKay Sullivan",
                rooms and times: "SNOW 112", "MWF1000+50");

    // MATH 4250-01: Programming for Scientific Computation
    section!(t, course: "MATH 4250-01",
                instructor: "Vinodh Kumar Chellamuthu",
                rooms and times: "SNOW 147", "MW1500+100");

    // MATH 4400-01: Financial Mathematics
    section!(t, course: "MATH 4400-01",
                instructor: "Jie Liu",
                rooms and times: "SNOW 124", "TR1330+75");

    // MATH 4410-01: Actuarial Exam FM/ 2 Preparation
    section!(t, course: "MATH 4410-01",
                instructor: "Jie Liu",
                rooms and times: "SNOW 124", "T1500+75");

    // MATH 4800-01: Industrial Careers in Mathematics
    section!(t, course: "MATH 4800-01",
                instructor: "Vinodh Kumar Chellamuthu",
                rooms and times: "SNOW 147", "MW1645+75");

    // MATH 900-01: Transitional Math I
    section!(t, course: "MATH 900-01",
                instructor: "Paula Manuele Temple",
                rooms and times: "SNOW 144", "MTWR1200+50");

    // MATH 900-02: Transitional Math I
    section!(t, course: "MATH 900-02",
                instructor: "Jameson C Hardy",
                rooms and times: "SNOW 144", "MTWR0900+50");

    // MATH 900-03: Transitional Math I
    section!(t, course: "MATH 900-03",
                instructor: "Paula Manuele Temple",
                rooms and times: "SNOW 144", "MW1300+100");

    // MATH 900-04: Transitional Math I
    section!(t, course: "MATH 900-04",
                instructor: "Scott Patrick Hicks",
                rooms and times: "SNOW 144", "MW1600+100");

    // MATH 900-06: Transitional Math I
    section!(t, course: "MATH 900-06",
                rooms and times: "SNOW 3", "TR1630+100");

    // MATH 900-07: Transitional Math I
    section!(t, course: "MATH 900-07",
                instructor: "Paula Manuele Temple",
                rooms and times: "SNOW 144", "TR1300+100");

    // MATH 900-51: Transitional Math I
    section!(t, course: "MATH 900-51",
                instructor: "Scott Patrick Hicks",
                rooms and times: "SNOW 144", "MW1800+100");

    // MATH 980-03: Transitional Math IIB
    section!(t, course: "MATH 980-03",
                instructor: "Tye K Rogers",
                rooms and times: "SNOW 144", "MTWR1000+50");

    // MATH 980-05: Transitional Math IIB
    section!(t, course: "MATH 980-05",
                instructor: "Michael N Paxman",
                rooms and times: "SNOW 144", "TR1630+100");

    // MATH 980-06: Transitional Math IIB
    section!(t, course: "MATH 980-06",
                instructor: "Tye K Rogers",
                rooms and times: "SNOW 144", "MTWR0800+50");

    // MATH 980-07: Transitional Math IIB
    section!(t, course: "MATH 980-07",
                instructor: "Kathryn E Ott",
                rooms and times: "SNOW 3", "MW1300+100");

    // MATH 980-08: Transitional Math IIB
    section!(t, course: "MATH 980-08",
                instructor: "Amanda Fa'onelua",
                rooms and times: "SNOW 3", "TR1300+100");

    // MATH 980-10: Transitional Math IIB
    section!(t, course: "MATH 980-10",
                rooms and times: "SNOW 3", "MW1630+100");

    // MECH 1100-01: Manufacturing Processes
    section!(t, course: "MECH 1100-01",
                instructor: "Andrew C Schiller",
                rooms and times: "SET 226", "MW1200+75");

    // MECH 1150-01: Prototyping Techniques
    section!(t, course: "MECH 1150-01",
                instructor: "Andrew C Schiller",
                rooms and times: "SET 225", "TR1500+170");

    // MECH 1150-02: Prototyping Techniques
    section!(t, course: "MECH 1150-02",
                instructor: "Andrew C Schiller",
                rooms and times: "SET 225", "MW1500+170");

    // MECH 1200-01: Coding
    section!(t, course: "MECH 1200-01",
                instructor: "Bing Jiang",
                rooms and times: "SET 226", "MWF0900+50");

    // MECH 1200-02: Coding
    section!(t, course: "MECH 1200-02",
                instructor: "Scott A Skeen",
                rooms and times: "SET 226", "MWF1000+50");

    // MECH 1205-01: Coding Lab
    section!(t, course: "MECH 1205-01",
                instructor: "David Brent Christensen",
                rooms and times: "SET 226", "R0800+110");

    // MECH 1205-02: Coding Lab
    section!(t, course: "MECH 1205-02",
                instructor: "David Brent Christensen",
                rooms and times: "SET 226", "R1000+110");

    // MECH 1205-03: Coding Lab
    section!(t, course: "MECH 1205-03",
                instructor: "Russell C Reid",
                rooms and times: "SET 226", "R1200+110");

    // MECH 1205-04: Coding Lab
    section!(t, course: "MECH 1205-04",
                instructor: "Bing Jiang",
                rooms and times: "SET 226", "R1400+110");

    // MECH 1205-05: Coding Lab
    section!(t, course: "MECH 1205-05",
                instructor: "Bing Jiang",
                rooms and times: "SET 226", "R1600+110");

    // MECH 2030-01: Dynamics
    section!(t, course: "MECH 2030-01",
                instructor: "Kameron J Eves",
                rooms and times: "SET 104", "MWF1100+50");

    // MECH 2160-01: Materials Science
    section!(t, course: "MECH 2160-01",
                instructor: "Divya Singh",
                rooms and times: "SET 226", "MW1500+75");

    // MECH 2250-01: Sensors & Actuators
    section!(t, course: "MECH 2250-01",
                instructor: "Scott A Skeen",
                rooms and times: "SET 104", "MW1200+75");

    // MECH 2250-02: Sensors & Actuators
    section!(t, course: "MECH 2250-02",
                instructor: "Scott A Skeen",
                rooms and times: "SET 104", "MW1330+75");

    // MECH 2255-01: Sensors & Actuators Lab
    section!(t, course: "MECH 2255-01",
                instructor: "Scott A Skeen",
                rooms and times: "SET 101", "R0800+110");

    // MECH 2255-02: Sensors & Actuators Lab
    section!(t, course: "MECH 2255-02",
                instructor: "Scott A Skeen",
                rooms and times: "SET 101", "R1200+110");

    // MECH 2255-03: Sensors & Actuators Lab
    section!(t, course: "MECH 2255-03",
                instructor: "David Brent Christensen",
                rooms and times: "SET 101", "R1400+110");

    // MECH 2255-04: Sensors & Actuators Lab
    section!(t, course: "MECH 2255-04",
                instructor: "Kameron J Eves",
                rooms and times: "SET 101", "R1600+110");

    // MECH 3250-01: Machinery
    section!(t, course: "MECH 3250-01",
                instructor: "Divya Singh",
                rooms and times: "SET 104", "MW1630+75");

    // MECH 3255-01: Machinery Lab
    section!(t, course: "MECH 3255-01",
                instructor: "Divya Singh",
                rooms and times: "SET 104", "T1200+110");

    // MECH 3255-02: Machinery Lab
    section!(t, course: "MECH 3255-02",
                instructor: "Andrew C Schiller",
                rooms and times: "SET 226", "T1200+110");

    // MECH 3600-01: Thermodynamics
    // xlist entry: SC0A
    section!(t, course: "MECH 3600-01",
                instructor: "Russell C Reid",
                rooms and times: "SET 104", "MTWF0900+50");

    // MECH 3602-01: Thermo II
    // xlist entry: SC0A
    section!(t, course: "MECH 3602-01",
                instructor: "Russell C Reid",
                rooms and times: "SET 104", "MTWF0900+50");

    // MECH 3605-01: Thermodynamics Lab
    section!(t, course: "MECH 3605-01",
                instructor: "Russell C Reid",
                rooms and times: "SET 104", "R1400+110");

    // MECH 3605-02: Thermodynamics Lab
    section!(t, course: "MECH 3605-02",
                instructor: "Russell C Reid",
                rooms and times: "SET 104", "R1600+110");

    // MECH 3650-01: Heat Transfer
    section!(t, course: "MECH 3650-01",
                instructor: "Russell C Reid",
                rooms and times: "SET 104", "MW1500+75");

    // MECH 3655-01: Heat Transfer Lab
    section!(t, course: "MECH 3655-01",
                instructor: "Russell C Reid",
                rooms and times: "SET 104", "R0800+110");

    // MECH 3655-02: Heat Transfer Lab
    section!(t, course: "MECH 3655-02",
                instructor: "Russell C Reid",
                rooms and times: "SET 104", "R1000+110");

    // MECH 4010-01: Product Design II
    section!(t, course: "MECH 4010-01",
                instructor: "Brant A Ross",
                rooms and times: "SET 219", "MWF1330+180");

    crosslist!(t, "ECE 4010-01" cross-list with "MECH 4010-01");

    // MECH 4500-01: Advanced Engineering Math
    section!(t, course: "MECH 4500-01",
                instructor: "Scott A Skeen",
                rooms and times: "SET 523", "TR1500+75");

    // MECH 4860R-01: Design Practicum
    section!(t, course: "MECH 4860R-01",
                instructor: "Scott A Skeen",
                rooms and times: "SET 102", "M0800+50");

    // MECH 4990-01: Special Topics: Finite Element Analysis
    section!(t, course: "MECH 4990-01",
                instructor: "Divya Singh",
                rooms and times: "SET 523", "MW1000+110");

    // MTRN 2350-01: Advanced PLC Programming
    section!(t, course: "MTRN 2350-01",
                instructor: "Bruford P Reynolds",
                rooms and times: "SET 102", "TR1000+50");

    // MTRN 2355-01: Advanced PLC Programming Lab
    section!(t, course: "MTRN 2355-01",
                instructor: "Bruford P Reynolds",
                rooms and times: "SET 102", "TR1400+110");

    // PHYS 1010-01: Elementary Physics (PS)
    section!(t, course: "PHYS 1010-01",
                instructor: "David M Syndergaard",
                rooms and times: "SET 418", "MW1630+75");

    // PHYS 1010-1SJ: Elementary Physics
    section!(t, course: "PHYS 1010-1SJ",
                instructor: "Bryce A Clay",
                rooms and times: "INNOV 111", "MW0930+80");

    // PHYS 1010-2SJ: Elementary Physics
    section!(t, course: "PHYS 1010-2SJ",
                instructor: "Bryce A Clay",
                rooms and times: "INNOV 111", "MW1330+80");

    // PHYS 1010-3SJ: Elementary Physics
    section!(t, course: "PHYS 1010-3SJ",
                instructor: "Bryce A Clay",
                rooms and times: "INNOV 111", "MW0800+80");

    // PHYS 1010-4SJ: Elementary Physics
    section!(t, course: "PHYS 1010-4SJ",
                instructor: "Bryce A Clay",
                rooms and times: "INNOV 111", "MW1200+80");

    // PHYS 1015-01: Elementary Physics Lab (LAB)
    section!(t, course: "PHYS 1015-01",
                instructor: "David M Syndergaard",
                rooms and times: "SET 410", "M1300+110");

    // PHYS 1015-02: Elementary Physics Lab (LAB)
    section!(t, course: "PHYS 1015-02",
                rooms and times: "SET 410", "M1000+110");

    // PHYS 1015-1SJ: Elementary Physics Lab
    section!(t, course: "PHYS 1015-1SJ",
                instructor: "Bryce A Clay",
                rooms and times: "INNOV 111", "T0800+120");

    // PHYS 1015-2SJ: Elementary Physics Lab
    section!(t, course: "PHYS 1015-2SJ",
                instructor: "Bryce A Clay",
                rooms and times: "INNOV 111", "F1330+120");

    // PHYS 1015-3SJ: Elementary Physics Lab
    section!(t, course: "PHYS 1015-3SJ",
                instructor: "Bryce A Clay",
                rooms and times: "INNOV 111", "R0930+120");

    // PHYS 1015-4SJ: Elementary Physics Lab
    section!(t, course: "PHYS 1015-4SJ",
                instructor: "Bryce A Clay",
                rooms and times: "INNOV 119", "F1200+120");

    // PHYS 1040-50: Elementary Astronomy (PS)
    section!(t, course: "PHYS 1040-50",
                instructor: "David M Syndergaard",
                rooms and times: "SET 418", "MW1800+75");

    // PHYS 1045-50: Elementary Astronomy Lab (LAB)
    section!(t, course: "PHYS 1045-50",
                instructor: "Christopher Kirk DeMacedo",
                rooms and times: "SET 418", "M1930+170");

    // PHYS 1045-51: Elementary Astronomy Lab (LAB)
    section!(t, course: "PHYS 1045-51",
                instructor: "Rick L Peirce",
                rooms and times: "SET 418", "T1930+170");

    // PHYS 1045-52: Elementary Astronomy Lab (LAB)
    section!(t, course: "PHYS 1045-52",
                instructor: "Jose C Saraiva",
                rooms and times: "SET 418", "W1930+170");

    // PHYS 2010-01: College Physics I (PS)
    section!(t, course: "PHYS 2010-01",
                instructor: "Steven K Sullivan",
                rooms and times: "SET 418", "MWRF0800+50");

    // PHYS 2010-02: College Physics I (PS)
    section!(t, course: "PHYS 2010-02",
                rooms and times: "SET 418", "MWRF1500+50");

    // PHYS 2015-01: College Physics I Lab (LAB)
    section!(t, course: "PHYS 2015-01",
                instructor: "Christopher Kirk DeMacedo",
                rooms and times: "SET 410", "T1200+110");

    // PHYS 2015-02: College Physics I Lab (LAB)
    section!(t, course: "PHYS 2015-02",
                instructor: "Christopher Kirk DeMacedo",
                rooms and times: "SET 410", "T1400+110");

    // PHYS 2015-03: College Physics I Lab (LAB)
    section!(t, course: "PHYS 2015-03",
                rooms and times: "SET 410", "T1000+110");

    // PHYS 2020-01: College Physics II
    section!(t, course: "PHYS 2020-01",
                instructor: "Steven K Sullivan",
                rooms and times: "SET 418", "MWRF1000+50");

    // PHYS 2020-02: College Physics II
    section!(t, course: "PHYS 2020-02",
                instructor: "Steven K Sullivan",
                rooms and times: "SET 418", "MWRF1100+50");

    // PHYS 2025-01: College Physics II Lab
    section!(t, course: "PHYS 2025-01",
                rooms and times: "SET 412", "T1400+50");

    // PHYS 2025-03: College Physics II Lab
    section!(t, course: "PHYS 2025-03",
                instructor: "Jose C Saraiva",
                rooms and times: "SET 412", "T1600+110");

    // PHYS 2025-04: College Physics II Lab
    section!(t, course: "PHYS 2025-04",
                rooms and times: "SET 412", "T1800+110");

    // PHYS 2210-01: Physics/Scientists Engineers I (PS)
    section!(t, course: "PHYS 2210-01",
                instructor: "Samuel K Tobler",
                rooms and times: "SET 418", "MTWF1300+50");

    // PHYS 2210-02: Physics/Scientists Engineers I (PS)
    section!(t, course: "PHYS 2210-02",
                rooms and times: "SET 418", "MTWF0900+50");

    // PHYS 2215-01: Physics/Scientists Engineers I Lab (LAB)
    section!(t, course: "PHYS 2215-01",
                rooms and times: "SET 410", "R1400+110");

    // PHYS 2215-02: Physics/Scientists Engineers I Lab (LAB)
    section!(t, course: "PHYS 2215-02",
                rooms and times: "SET 410", "R1600+110");

    // PHYS 2215-50: Physics/Scientists Engineers I Lab (LAB)
    section!(t, course: "PHYS 2215-50",
                instructor: "Jose C Saraiva",
                rooms and times: "SET 410", "R1800+110");

    // PHYS 2220-01: Physics/Scientists Engineers II
    section!(t, course: "PHYS 2220-01",
                instructor: "Samuel K Tobler",
                rooms and times: "SET 418", "MTWF1400+50");

    // PHYS 2225-01: Physics/Scientists Engineers II Lab
    section!(t, course: "PHYS 2225-01",
                rooms and times: "SET 412", "R1400+110");

    // PHYS 2225-02: Physics/Scientists Engineers II Lab
    section!(t, course: "PHYS 2225-02",
                instructor: "Jose C Saraiva",
                rooms and times: "SET 412", "R1600+110");

    // PHYS 3600-01: Thermodynamics
    section!(t, course: "PHYS 3600-01",
                rooms and times: "SET 104", "MTWF0900+50");

    crosslist!(t, "MECH 3600-01" cross-list with "MECH 3602-01" cross-list with "PHYS 3600-01");

    // PHYS 3605-01: Thermodynamics Lab
    section!(t, course: "PHYS 3605-01",
                rooms and times: "SET 104", "R1400+110");

    crosslist!(t, "MECH 3605-01" cross-list with "PHYS 3605-01");

    // PHYS 3605-02: Thermodynamics Lab
    section!(t, course: "PHYS 3605-02",
                rooms and times: "SET 104", "R1600+110");

    crosslist!(t, "MECH 3605-02" cross-list with "PHYS 3605-02");

    // SCI 4700-01: Secondary Sci Teaching Methods
    section!(t, course: "SCI 4700-01",
                instructor: "Mark L Dickson",
                rooms and times: "SET 216", "R1530+150");

    // SCI 4720-01: Innovative Solutions - Product Development
    section!(t, course: "SCI 4720-01",
                rooms and times: "SET 501", "F1400+170");

    Ok(())
}
