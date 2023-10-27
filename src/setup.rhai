let t = term("Spring 2024", "2024-01-08", "2024-04-25");

t.holiday("2024-01-15");
t.holiday("2024-02-19");
t.holiday("2024-03-11");
t.holiday("2024-03-12");
t.holiday("2024-03-13");
t.holiday("2024-03-14");
t.holiday("2024-03-15");

t.room("Smith 107", 32, ["flex"]);
t.room("Smith 108", 32, ["flex"]);
t.room("Smith 109", 32, ["flex"]);
t.room("Smith 112", 24, ["macs"]);
t.room("Smith 113", 24, ["pcs"]);
t.room("Smith 116", 38, ["stadium"]);
t.room("Smith 117", 38, ["stadium"]);

t.time("MWF0800+50", ["mwf"]);
t.time("MWF0900+50", ["mwf", "morning"]);
t.time("MWF1000+50", ["mwf", "morning"]);
t.time("MWF1100+50", ["mwf", "morning"]);
t.time("MW1200+75", ["mw", "afternoon"]);
t.time("MW1330+75", ["mw", "afternoon"]);
t.time("MW1500+75", ["mw", "afternoon"]);
t.time("MW1630+75", ["mw"]);
t.time("TR0900+75", ["tr", "morning"]);
t.time("TR1030+75", ["tr", "morning"]);
t.time("TR1200+75", ["tr", "afternoon"]);
t.time("TR1330+75", ["tr", "afternoon"]);
t.time("TR1500+75", ["tr", "afternoon"]);
t.time("TR1630+75", ["tr"]);
t.time("T1800+150", ["evening"]);
t.time("W1800+150", ["evening"]);
t.time("R1800+150", ["evening"]);
t.time("R1900+50", []);

t.instructor("Bart Stander",
    ["MWF0900+50", "MWF1000+50", "MWF1100+50",
     ["MW1200+75",10], "MW1330+75", "MW1500+75",
     "TR1030+75", "TR1330+75", ["TR1500+75",10]]);
//t.preference("oneday");
t.instructor("Carol Stander",
    ["MWF1000+50", "MWF1100+50",
     ["MW1200+75", 10], "MW1330+75",
     ["TR1330+75", 5]]);
t.instructor("Curtis Larsen",
    ["MWF0900+50", "MWF1000+50", ["MWF1100+50", 10],
     ["MW1200+75", 10], "MW1330+75", "MW1500+75",
     "TR0900+75", ["TR1030+75", 10], ["TR1200+75", 10], "TR1330+75", "TR1500+75"]);
//t.preference("twodays");
t.instructor("DJ Holt",
    ["MW1200+75", "MW1330+75", ["MW1500+75", 10],
    "TR0900+75", "TR1030+75", "TR1200+75", "TR1330+75", ["TR1500+75", 10]]);
//t.preference("twodays");
t.instructor("Eric Pedersen",
    ["TR1200+75"]);
t.instructor("Jay Sneddon",
    [["MWF0800+50", 15], ["MWF0900+50", 10], ["MWF1000+50", 10], ["MWF1100+50", 10], "MW1200+75", "MW1330+75", "MW1500+75", "TR0900+75", "TR1030+75", "TR1200+75", "TR1330+75", ["TR1500+75", 5]]);
//t.preference("twodays");
t.instructor("Jeff Compas",
    ["MWF0800+50", "MW1630+75", "TR1630+75", "T1800+150"]);
t.instructor("Joe Francom",
    ["MWF0900+50", "MWF1000+50", "MWF1100+50", "MW1330+75"]);
//t.preference("oneday");
t.instructor("Lora Klein",
    ["TR0900+75", "TR1030+75", "TR1200+75", "TR1330+75", ["MW1500+75", 15]]);
t.instructor("Matt Kearl",
    ["MW1200+75", "TR0900+75", "TR1030+75", "TR1200+75"]);
//t.preference("oneday");
t.instructor("Phil Daley",
    ["MWF0900+50", "MWF1000+50", "MWF1100+50",
    "MW1200+75", "MW1330+75", ["MW1500+75", 10],
    "TR0900+75", "TR1030+75", "TR1200+75", "TR1330+75", ["TR1500+75", 10]]);
//t.preference("twodays");
t.instructor("Ren Quinn",
    ["MWF0900+50", "MWF1000+50", "MWF1100+50",
     ["TR1200+75", 5], "TR1330+75", "TR1500+75",
     "R1900+50"]);
//t.preference("twodays");
t.instructor("Russ Ross",
    ["MW1200+75", "MW1330+75", ["MW1500+75", 10],
     "TR1200+75", "TR1330+75", ["TR1500+75", 10]]);
t.instructor("Rex Frisbey",
    ["MWF1100+50"]);
t.instructor("Jamie Bennion",
    ["W1800+150"]);

t.section("CS 2420", "01", "Bart Stander", ["stadium", ["flex", 10], "mwf"]);
t.section("CS 3310", "01", "Bart Stander", ["stadium", "pcs"]);
t.section("CS 3600", "01", "Bart Stander", ["pcs", ["stadium", 10]]);
t.section("CS 4550", "01", "Bart Stander", ["pcs"]);

t.section("CS 1030", "01", "Carol Stander", ["flex"]);
t.section("CS 1410", "02", "Carol Stander", ["flex"]);
//course: CS1410 online
//course: IT1100 online

t.section("CS 3005", "01", "Curtis Larsen", ["Smith 116", "mwf"]);
t.section("CS 3510", "01", "Curtis Larsen", [["flex", 1], "Smith 116", "mw", "mwf", ["tr", 10]]);
t.section("CS 4320", "01", "Curtis Larsen", [["flex", 1], "Smith 116", "mw", ["mwf", 10], "tr"]);
t.section("CS 4600", "01", "Curtis Larsen", [["flex", 1], "Smith 116", "mw", "mwf", ["tr", 10]]);
// all senior projects at same time
// course: CS 4920R not scheduled

t.section("SE 3010", "01", "DJ Holt", ["flex", "macs", "MW1500+75"]); // same day as SE4200
t.section("SE 4200", "01", "DJ Holt", ["flex", "macs", "MW1330+75"]);
t.section("SE 4600", "01", "DJ Holt", ["flex"]); // with cross-listed CS4600-02 section; all senior projects at same time
// schedule SE4600 same time as CS4600-01: Curtis and I will each have a section of CS4600, and my section will be cross-listed with SE4600
// note all CS cross-listings above

t.section("SE 3500", "01", "Eric Pedersen", ["flex"]);

t.section("IT 1200", "01", "Jay Sneddon", ["Smith 107", "tr"]);
t.section("IT 2300", "01", "Jay Sneddon", ["Smith 107"]);
t.section("IT 2700", "01", "Jay Sneddon", ["Smith 107", "tr"]);
t.section("IT 3150", "01", "Jay Sneddon", ["Smith 107", "mw", ["mwf", 5]]);
t.section("IT 3400", "01", "Jay Sneddon", ["Smith 107"]);

t.section("CS 1400", "03", "Jeff Compas", ["stadium"]);
t.section("CS 1400", "04", "Jeff Compas", ["stadium"]);
t.section("CS 2450", "02", "Jeff Compas", ["flex"]);
t.section("SE 3100", "01", "Jeff Compas", ["flex"]);
//he needs two morning courses and two evening courses
//3 credit release for first semester

t.section("IT 3110", "01", "Joe Francom", ["flex"]);
t.section("IT 4510", "01", "Joe Francom", ["flex"]);
//online section of IT1500 1 credit
//online section of IT4600 3 credits

t.section("SE 3200", "01", "Lora Klein", [["Smith 107", 5], "flex"]);
//course: CS1410 ACE MW 9:30-10:45am, INV 112
//course: CS1410 ACE MW 12:00-1:15pm, INV 112
// workload release for program development

t.section("SE 3450", "01", "Matt Kearl", ["flex", "macs"]);
t.section("SE 3550", "01", "Matt Kearl", ["flex", "macs"]);
t.section("SE 1400", "01", "Matt Kearl", ["macs"]);
//course: SE1400 online
//course: SE1400 online
//course: SE4920 not scheduled

t.section("IT 1100", "01", "Phil Daley", ["pcs"]);
t.section("IT 1100", "02", "Phil Daley", ["pcs"]);
t.section("IT 2400", "01", "Phil Daley", ["Smith 107"]);
t.section("IT 3100", "01", "Phil Daley", ["Smith 107"]);
// avoid IT 4510 so Phil can shadow Joe
// 3 credits release for network admin

t.section("CS 1400", "01", "Ren Quinn", ["flex"]);
t.section("CS 1400", "02", "Ren Quinn", ["flex"]);
t.section("CS 1410", "01", "Ren Quinn", ["flex"]);
t.section("CS 2450", "01", "Ren Quinn", ["flex"]);
t.section("CS 3150", "01", "Ren Quinn", ["flex"]);
t.section("CS 4991", "01", "Ren Quinn", ["Smith 116", "R1900+50"]); // overload acm
// course: CS4992 flex F1300 // overload seminar
// course: CS4800R not scheduled
// CS4991R is actually R1900-1950

t.section("CS 2810", "01", "Russ Ross", [["Smith 107", 5], ["Smith 108", 5], "Smith 109"]);
t.section("CS 2810", "02", "Russ Ross", [["Smith 107", 5], ["Smith 108", 5], "Smith 109"]);
t.section("CS 3410", "01", "Russ Ross", [["Smith 107", 5], ["Smith 108", 5], "Smith 109"]);
t.section("CS 4307", "01", "Russ Ross", [["Smith 107", 5], ["Smith 108", 5], "Smith 109"]);

t.section("SE 1400", "02", "Rex Frisbey", ["macs"]);

t.section("IT 4990", "01", "Jamie Bennion", ["flex"]);

t.hard_conflict_clique(["CS 2420", "CS 2450", "CS 2810", "CS 3005"]);   // 3rd/4th semester classes
t.hard_conflict_clique(["CS 2420", "CS 2450", "CS 2810"]);               // grad plan: 2nd year fall
t.hard_conflict_clique(["CS 3005", "CS 3520", "SE 3200"]);               // grad plan: 2nd year spring
t.hard_conflict_clique(["CS 3310", "CS 3400", "SE 3530"]);               // grad plan: 3nd year fall
t.hard_conflict_clique(["CS 3510", "CS 4307", "SE 4550"]);               // grad plan: 3nd year spring
t.hard_conflict_clique(["CS 4300"]);                                    // grad plan: 4th year fall
t.hard_conflict_clique(["CS 3600", "CS 4600"]);                         // grad plan: 4th year spring

// CS upper division core
t.conflict_clique(90, [
    "CS 2450",
    "CS 3150", "CS 3310", "CS 3400", "CS 3410", "CS 3510", "CS 3520", "CS 3530", "CS 3600",
    "CS 4300", "CS 4307", "CS 4320", "CS 4550", "CS 4600",
    "SE 3200"]);

// CS electives
t.conflict_clique(30, [
    "CS 2450",
    "CS 3150", "CS 3310", "CS 3400", "CS 3410", "CS 3500", "CS 3510", "CS 3520", "CS 3530", "CS 3600",
    "CS 4300", "CS 4307", "CS 4320", "CS 4550", "CS 4600", "CS 4990",
    "SE 3010", "SE 3020", "SE 3100", "SE 3200", "SE 3400", "SE 4200",
    "IT 2700", "IT 3100", "IT 3110", "IT 4200"]);

// CS classes that do not require CS2810 so they can be taken concurrently
t.conflict_clique(45, ["CS 2810", "SE 3020", "SE 3200", "CS 3500"]);

// CS classes that do not require CS3005 so they can be taken concurrently
t.conflict_clique(45, ["CS 3005", "CS 3150", "CS 3310", "CS 3410", "CS 3510", "CS 3520", "CS 3530"]);

// DS: TODO
t.conflict_clique(45, ["CS 2500", "CS 2810", "CS 3005"]);

// SE upper division core
t.conflict_clique(90, [
    "CS 2450", "CS 3150", "CS 3310", "CS 3510", "CS 4307",
    "IT 2300",
    "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400",
    "SE 4200", "SE 4600"]); // IT 1100, SE 1400

// Entrepreneurial and marketing track
t.conflict_clique(90, [
    "CS 2450", "CS 3150", "CS 3310", "CS 3510", "CS 4307",
    "IT 2300",
    "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400", "SE 3500", "SE 3550",
    "SE 4200", "SE 4600"]); // IT 1100 SE 1400

// DevOps track
t.conflict_clique(90, [
    "CS 2450", "CS 3150", "CS 3310", "CS 3510", "CS 4307",
    "IT 2300", "IT 3110", "IT 3300", "IT 4200",
    "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400",
    "SE 4200", "SE 4600"]); // IT 1100 SE 1400

// Application track
t.conflict_clique(90, [
    "CS 2450", "CS 3150", "CS 3310", "CS 3500", "CS 3510", "CS 4307",
    "IT 2300",
    "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400", "SE 3450",
    "SE 4200", "SE 4600"]); // IT 1100 SE 1400

// Data science track
t.conflict_clique(90, [
    "CS 2450", "CS 3150", "CS 3310", "CS 3510", "CS 4300", "CS 4307", "CS 4320",
    "IT 2300",
    "SE 3010", "SE 3020", "SE 3100", "SE 3150", "SE 3200", "SE 3400",
    "SE 4200", "SE 4600"]); // IT 1100 SE 1400

// IT conflicts
//t.conflict_clique(50, ["IT 1100", "IT 1200"]); // when there is only one in-person section of each
t.conflict_clique(90, [
    "IT 2300", "IT 2400", "IT 2500", "IT 2700",
    "IT 3100", "IT 3110", "IT 3150", "IT 3300", "IT 3400",
    "IT 4100", "IT 4200", "IT 4310", "IT 4400", "IT 4510", "IT 4600"]);

// IT choose 2 section
t.conflict_clique(60, [
    "CS 3005",
    "IT 2300", "IT 2400", "IT 2500", "IT 2700",
    "IT 3100", "IT 3110", "IT 3150", "IT 3300", "IT 3400",
    "IT 4100", "IT 4200", "IT 4310", "IT 4400", "IT 4510", "IT 4600",
    "SE 3200", "SE 3400"]);

// CS2450 is a prereq for SE3100 so they can be scheduled at the same time
t.clear_conflict_clique(["CS 2450", "SE 3100"]);
// CS2450 is a prereq for SE3150 so they can be scheduled at the same time
t.clear_conflict_clique(["CS 2450", "SE 3150"]);
// SE3200 is a prereq for SE4200 so they can be scheduled at the same time
t.clear_conflict_clique(["SE 3200", "SE 4200"]);
// students take either CS4307 or IT2300 but not both
t.clear_conflict_clique(["CS 4307", "IT 2300"]);

// multiple-section courses must be taught at different times
t.hard_conflict_clique(["CS 1400"]);
t.hard_conflict_clique(["CS 1410"]);
t.hard_conflict_clique(["CS 2450"]);
t.hard_conflict_clique(["CS 2810"]);
t.hard_conflict_clique(["IT 1100"]);
t.hard_conflict_clique(["SE 1400"]);


// courses that must be scheduled at the same time
//anticonflict: 50 CS1400 CS1030
//anticonflict: 50 SE1400 IT1100 // temporarily removed because of new hire planning
//anticonflict: 50 CS4600 SE4600

t
