let data = term("Spring 2024", "2024-01-08", "2024-04-25");

data.holiday("2024-01-15");
data.holiday("2024-02-19");
data.holiday("2024-03-11");
data.holiday("2024-03-12");
data.holiday("2024-03-13");
data.holiday("2024-03-14");
data.holiday("2024-03-15");

data.building("Smith", ["oncampus"]);
data.room("107", 32, ["flex"]);
data.room("108", 32, ["flex"]);
data.room("109", 32, ["flex"]);
data.room("112", 24, ["macs"]);
data.room("113", 24, ["pcs"]);
data.room("116", 38, ["stadium"]);
data.room("117", 38, ["stadium"]);

data.time("MWF0800+50", ["mwf"]);
data.time("MWF0900+50", ["mwf", "morning"]);
data.time("MWF1000+50", ["mwf", "morning"]);
data.time("MWF1100+50", ["mwf", "morning"]);
data.time("MW1200+75", ["mw", "afternoon"]);
data.time("MW1330+75", ["mw", "afternoon"]);
data.time("MW1500+75", ["mw", "afternoon"]);
data.time("MW1630+75", ["mw"]);
data.time("TR0900+75", ["tr", "morning"]);
data.time("TR1030+75", ["tr", "morning"]);
data.time("TR1200+75", ["tr", "afternoon"]);
data.time("TR1330+75", ["tr", "afternoon"]);
data.time("TR1500+75", ["tr", "afternoon"]);
data.time("TR1630+75", ["tr"]);
data.time("T1800+150", ["evening"]);
data.time("W1800+150", ["evening"]);
data.time("R1800+150", ["evening"]);
data.time("R1900+50", []);

data
