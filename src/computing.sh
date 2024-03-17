set -e

echo building smith building and classrooms
./edit make-building Smith
./edit make-room 'Smith 107' 32 flex
./edit make-room 'Smith 108' 32 flex
./edit make-room 'Smith 109' 32 flex
./edit make-room 'Smith 112' 24 macs
./edit make-room 'Smith 113' 24 pcs
./edit make-room 'Smith 116' 38 stadium
./edit make-room 'Smith 117' 38 stadium


echo building computing time slots
./edit make-time-slot 'MWF0800+50' '3 credit bell schedule' 'MWF 3×50 bell schedule'
./edit make-time-slot 'MWF0900+50' '3 credit bell schedule' 'MWF 3×50 bell schedule'
./edit make-time-slot 'MWF1000+50' '3 credit bell schedule' 'MWF 3×50 bell schedule'
./edit make-time-slot 'MWF1100+50' '3 credit bell schedule' 'MWF 3×50 bell schedule'
./edit make-time-slot 'MW1200+75' '3 credit bell schedule' '2×75 bell schedule' 'MW 2×75 bell schedule'
./edit make-time-slot 'MW1330+75' '3 credit bell schedule' '2×75 bell schedule' 'MW 2×75 bell schedule'
./edit make-time-slot 'MW1500+75' '3 credit bell schedule' '2×75 bell schedule' 'MW 2×75 bell schedule'
./edit make-time-slot 'MW1630+75' '3 credit bell schedule' '2×75 bell schedule' 'MW 2×75 bell schedule'
./edit make-time-slot 'TR0900+75' '3 credit bell schedule' '2×75 bell schedule' 'TR 2×75 bell schedule'
./edit make-time-slot 'TR1030+75' '3 credit bell schedule' '2×75 bell schedule' 'TR 2×75 bell schedule'
./edit make-time-slot 'TR1200+75' '3 credit bell schedule' '2×75 bell schedule' 'TR 2×75 bell schedule'
./edit make-time-slot 'TR1330+75' '3 credit bell schedule' '2×75 bell schedule' 'TR 2×75 bell schedule'
./edit make-time-slot 'TR1500+75' '3 credit bell schedule' '2×75 bell schedule' 'TR 2×75 bell schedule'
./edit make-time-slot 'TR1630+75' '3 credit bell schedule' '2×75 bell schedule' 'TR 2×75 bell schedule'
./edit make-time-slot 'R1800+150' '1×150 evening'
./edit make-time-slot 'T1800+150' '1×150 evening'
./edit make-time-slot 'W1800+150' '1×150 evening'
./edit make-time-slot 'R1900+50'
./edit make-time-slot 'F1300+50'

echo building computing conflicts
./edit make-program 'Computer Science' Computing
./edit make-conflict 'Computer Science' '3rd/4th semester classes' 100 maximize \
    'CS 2420' 'CS 2450' 'CS 2810' 'CS 3005'
./edit make-conflict 'Computer Science' 'grad plan: 2nd year fall' 100 maximize \
    'CS 2420' 'CS 2450' 'CS 2810'
./edit make-conflict 'Computer Science' 'grad plan: 2nd year spring' 100 maximize \
    'CS 3005' 'CS 3520' 'SE 3200'
./edit make-conflict 'Computer Science' 'grad plan: 3rd year fall' 100 maximize \
    'CS 3310' 'CS 3400' 'CS 3530'
./edit make-conflict 'Computer Science' 'grad plan: 3rd year spring' 100 maximize \
    'CS 3510' 'CS 4307' 'CS 4550'
./edit make-conflict 'Computer Science' 'grad plan: 4th year fall' 100 maximize \
    'CS 4300'
./edit make-conflict 'Computer Science' 'grad plan: 4th year spring' 100 maximize \
    'CS 3600' 'CS 4600'
./edit make-conflict 'Computer Science' 'core requirements' 99 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2420' 'CS 2450' 'CS 2810' 'CS 3005' \
    'CS 3150' 'CS 3310' 'CS 3400' 'CS 3410' 'CS 3510' 'CS 3520' 'CS 3530' 'CS 3600' \
    'CS 4300' 'CS 4307' 'CS 4320' 'CS 4550' 'CS 4600' \
    'SE 3200'
./edit make-conflict 'Computer Science' 'electives' 30 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2420' 'CS 2450' 'CS 2810' 'CS 3005' \
    'CS 3150' 'CS 3310' 'CS 3400' 'CS 3410' 'CS 3510' 'CS 3520' 'CS 3530' 'CS 3600' \
    'CS 4300' 'CS 4307' 'CS 4320' 'CS 4550' 'CS 4600' \
    'SE 3200' \
    'SE 3010' 'SE 3020' 'SE 3100' 'SE 3400' 'SE 4200' \
    'IT 2700' 'IT 3100' 'IT 3110' 'IT 4200'
./edit make-conflict 'Computer Science' 'math and science' 50 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2420' 'CS 2450' 'CS 2810' 'CS 3005' \
    'CS 3150' 'CS 3310' 'CS 3400' 'CS 3410' 'CS 3510' 'CS 3520' 'CS 3530' 'CS 3600' \
    'CS 4300' 'CS 4307' 'CS 4320' 'CS 4550' 'CS 4600' \
    'SE 3200' \
    'MATH 1210' 'MATH 1220' 'BIOL 1610' 'BIOL 1615' 'PHYS 2210' 'PHYS 2215'

./edit make-program 'Data Science' Computing
./edit make-conflict 'Data Science' 'third semester' 45 maximize \
    'CS 2500' 'CS 2810' 'CS 3005'

./edit make-program 'Software Engineering' Computing
./edit make-conflict 'Software Engineering' 'core requirements' 99 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2420' 'CS 2450' 'CS 2810' \
    'CS 3150' 'CS 3310' 'CS 3510' 'CS 4307' \
    'IT 1100' 'IT 2300' \
    'SE 1400' 'SE 3010' 'SE 3020' 'SE 3100' 'SE 3150' 'SE 3200' 'SE 3400' \
    'SE 4200' 'SE 4600'
./edit make-conflict 'Software Engineering' 'Entrepreneurial and marketing track' 45 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2420' 'CS 2450' 'CS 2810' \
    'CS 3150' 'CS 3310' 'CS 3510' 'CS 4307' \
    'IT 1100' 'IT 2300' \
    'SE 1400' 'SE 3010' 'SE 3020' 'SE 3100' 'SE 3150' 'SE 3200' 'SE 3400' 'SE 3500' 'SE 3550' \
    'SE 4200' 'SE 4600'
./edit make-conflict 'Software Engineering' 'DevOps track' 45 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2420' 'CS 2450' 'CS 2810' \
    'CS 3150' 'CS 3310' 'CS 3510' 'CS 4307' \
    'IT 1100' 'IT 2300' 'IT 3110' 'IT 3300' 'IT 4200' \
    'SE 1400' 'SE 3010' 'SE 3020' 'SE 3100' 'SE 3150' 'SE 3200' 'SE 3400' \
    'SE 4200' 'SE 4600'
./edit make-conflict 'Software Engineering' 'Application track' 45 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2420' 'CS 2450' 'CS 2810' \
    'CS 3150' 'CS 3310' 'CS 3500' 'CS 3510' 'CS 4307' \
    'IT 1100' 'IT 2300' \
    'SE 1400' 'SE 3010' 'SE 3020' 'SE 3100' 'SE 3150' 'SE 3200' 'SE 3400' 'SE 3450' \
    'SE 4200' 'SE 4600'
./edit make-conflict 'Software Engineering' 'Data science track' 45 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2420' 'CS 2450' 'CS 2810' \
    'CS 3150' 'CS 3310' 'CS 3510' 'CS 4300' 'CS 4307' 'CS 4320' \
    'IT 1100' 'IT 2300' \
    'SE 1400' 'SE 3010' 'SE 3020' 'SE 3100' 'SE 3150' 'SE 3200' 'SE 3400' \
    'SE 4200' 'SE 4600'
./edit make-conflict 'Software Engineering' 'only need one database class' 0 minimize \
    'CS 4307' 'IT 2300'

./edit make-program 'Information Technology' Computing
./edit make-conflict 'Information Technology' 'core requirements' 99 maximize \
    'IT 1100' 'IT 1200' 'IT 2300' 'IT 2400' 'IT 2500' 'IT 2700' \
    'IT 3100' 'IT 3110' 'IT 3150' 'IT 3300' 'IT 3400' \
    'IT 4100' 'IT 4200' 'IT 4310' 'IT 4400' 'IT 4510' 'IT 4600'
./edit make-conflict 'Information Technology' 'choose two section' 60 maximize \
    'CS 3005' \
    'IT 1100' 'IT 1200' 'IT 2300' 'IT 2400' 'IT 2500' 'IT 2700' \
    'IT 3100' 'IT 3110' 'IT 3150' 'IT 3300' 'IT 3400' \
    'IT 4100' 'IT 4200' 'IT 4310' 'IT 4400' 'IT 4510' 'IT 4600' \
    'SE 3200' 'SE 3400'


echo building computing faculty and sections
./edit make-faculty 'Bart Stander' Computing \
    'MWF 0900-1200,
     MW  1200-1330 with penalty 10,
     MW  1330-1630,
     TR  1030-1200,
     TR  1330-1500,
     TR  1500-1630 with penalty 10'
./edit faculty-default-clustering 'Bart Stander' 'MT' 1
./edit make-section 'CS 2420-01' 'stadium' 'flex:10' 'MWF 3×50 bell schedule'
./edit make-section 'CS 3310-01' 'stadium' 'pcs' '3 credit bell schedule'
./edit make-section 'CS 3600-01' 'pcs' 'stadium:10' '3 credit bell schedule'
./edit make-section 'CS 4550-01' 'pcs' '3 credit bell schedule'
./edit assign-faculty-sections 'Bart Stander' 'CS 2420-01' 'CS 3310-01' 'CS 3600-01' 'CS 4550-01'

./edit make-faculty 'Carol Stander' Computing \
    'MWF 1000-1200,
     MW  1200-1330 with penalty 10,
     MW  1330-1500,
     TR  1330-1500 with penalty 5'
./edit faculty-default-clustering 'Carol Stander' 'MT' no_preference
./edit make-section 'CS 1030-01' 'flex' '3 credit bell schedule'
./edit make-section 'CS 1410-01' 'flex' '3 credit bell schedule'
./edit make-section 'IT 2300-02' 'Smith 113' '3 credit bell schedule'
./edit assign-faculty-sections 'Carol Stander' 'CS 1030-01' 'CS 1410-01' 'IT 2300-02'

./edit make-faculty 'Curtis Larsen' Computing \
    'MWF 0900-1100,
     MWF 1100-1200 with penalty 10,
     MW  1200-1330 with penalty 10,
     MW  1330-1630,
     TR  0900-1030,
     TR  1030-1330 with penalty 10,
     TR  1330-1630'
./edit faculty-default-clustering 'Curtis Larsen' 'MT' 0
./edit make-section 'CS 3005-01' 'Smith 116' 'MWF 3×50 bell schedule'
./edit make-section 'CS 3510-01' 'Smith 116' 'flex:1' '3 credit bell schedule' 'TR 2×75 bell schedule:10'
./edit make-section 'CS 4320-01' 'Smith 116' 'flex:1' 'MWF 3×50 bell schedule:10' '2×75 bell schedule'
./edit make-section 'CS 4600-01' 'Smith 116' 'flex:1' '3 credit bell schedule' 'TR 2×75 bell schedule:10'
./edit assign-faculty-sections 'Curtis Larsen' 'CS 3005-01' 'CS 3510-01' 'CS 4320-01' 'CS 4600-01'

./edit make-faculty 'DJ Holt' Computing \
    'MW 1200-1500,
     MW 1500-1630 with penalty 10,
     TR 0900-1500,
     TR 1500-1630 with penalty 10'
./edit faculty-default-clustering 'DJ Holt' 'MT' 0
# SE 3010-01 same day as SE 4200-01
./edit make-section 'SE 3010-01' 'flex' 'macs' 'MW1500+75'
./edit make-section 'SE 4200-01' 'flex' 'macs' 'MW1330+75'
./edit make-section 'SE 4600-01' 'flex' '3 credit bell schedule'
./edit make-section 'CS 4600-02' 'flex' '3 credit bell schedule'
./edit assign-faculty-sections 'DJ Holt' 'SE 3010-01' 'SE 4200-01' 'SE 4600-01' 'CS 4600-02'
# crosslist!(t, "SE 4600-01" cross-list with "CS 4600-02");
# anticonflict!(t, set penalty to 50, single: "CS 4600-01", group: "CS 4600-02");

./edit make-faculty 'Eric Pedersen' Computing \
    'TR  1200-1330'
./edit make-section 'SE 3500-01' 'flex' 'TR1200+75'
./edit assign-faculty-sections 'Eric Pedersen' 'SE 3500-01'

./edit make-faculty 'Jay Sneddon' Computing \
    'MWF 0800-0900 with penalty 15,
     MWF 0900-1200 with penalty 10,
     MW  1200-1630,
     TR  0900-1500,
     TR  1500-1630 with penalty 5'
./edit faculty-default-clustering 'Jay Sneddon' 'MT' 0
./edit make-section 'IT 1200-01' 'Smith 107' 'TR 2×75 bell schedule'
./edit make-section 'IT 2300-01' 'Smith 107' 'Smith 113' '3 credit bell schedule'
./edit make-section 'IT 2700-01' 'Smith 107' 'TR 2×75 bell schedule'
./edit make-section 'IT 3150-01' 'Smith 107' 'MW 2×75 bell schedule' 'MWF 3×50 bell schedule:5'
./edit make-section 'IT 3400-01' 'Smith 107' '3 credit bell schedule'
./edit assign-faculty-sections 'Jay Sneddon' 'IT 1200-01' 'IT 2300-01' 'IT 2700-01' 'IT 3150-01' 'IT 3400-01'

./edit make-faculty 'Jeff Compas' Computing \
    'MWF 0800-0900,
     MW  1630-1800,
     TR  1630-1800,
     T   1800-2030'
./edit make-section 'CS 1400-01' 'stadium' '3 credit bell schedule' '1×150 evening'
./edit make-section 'CS 1400-50' 'stadium' '3 credit bell schedule' '1×150 evening'
./edit make-section 'CS 2450-02' 'flex' '3 credit bell schedule' '1×150 evening'
./edit make-section 'SE 3100-01' 'flex' '3 credit bell schedule' '1×150 evening'
./edit assign-faculty-sections 'Jeff Compas' 'CS 1400-01' 'CS 1400-50' 'CS 2450-02' 'SE 3100-01'

./edit make-faculty 'Joe Francom' Computing \
    'MWF 0800-1200,
     MW  1330-1500'
./edit faculty-default-clustering 'Joe Francom' 'MT' 1
./edit make-section 'IT 3110-01' 'flex' '3 credit bell schedule'
./edit make-section 'IT 4600-01' 'flex' '3 credit bell schedule'
./edit assign-faculty-sections 'Joe Francom' 'IT 3110-01' 'IT 4600-01'

./edit make-faculty 'Lora Klein' Computing \
    'TR 0900-1500,
     MW 1500-1630 with penalty 15'
./edit faculty-default-clustering 'Lora Klein' 'MT' no_preference
./edit make-section 'SE 3200-01' 'Smith 107:5' 'flex' '3 credit bell schedule'
./edit assign-faculty-sections 'Lora Klein' 'SE 3200-01'

./edit make-faculty 'Matt Kearl' Computing \
    'MW 1200-1330,
     TR 0900-1330'
./edit faculty-default-clustering 'Matt Kearl' 'MT' 1
./edit make-section 'SE 3450-01' 'flex' 'macs' '3 credit bell schedule'
./edit make-section 'SE 3550-01' 'flex' 'macs' '3 credit bell schedule'
./edit make-section 'SE 1400-02' 'macs' '3 credit bell schedule'
./edit assign-faculty-sections 'Matt Kearl' 'SE 3450-01' 'SE 3550-01' 'SE 1400-02'

./edit make-faculty 'Phil Daley' Computing \
    'MWF 0900-1200,
     MW  1200-1500,
     MW  1500-1630 with penalty 10,
     TR  0900-1500,
     TR  1500-1630 with penalty 10'
./edit faculty-default-clustering 'Phil Daley' 'MT' 0
./edit make-section 'IT 1100-01' 'pcs' '3 credit bell schedule'
./edit make-section 'IT 1100-02' 'pcs' '3 credit bell schedule'
./edit make-section 'IT 2400-01' 'Smith 107' '3 credit bell schedule'
./edit make-section 'IT 3100-01' 'Smith 107' '3 credit bell schedule'
./edit assign-faculty-sections 'Phil Daley' 'IT 1100-01' 'IT 1100-02' 'IT 2400-01' 'IT 3100-01'

./edit make-faculty 'Derek Sneddon' Computing \
    'R 1800-2230'
./edit make-section 'IT 4510-01' 'flex' 'R1800+150'
./edit assign-faculty-sections 'Derek Sneddon' 'IT 4510-01'

./edit make-faculty 'Ren Quinn' Computing \
    'MWF 0900-1200,
     TR  1200-1330 with penalty 5,
     TR  1330-1630,
     R   1900-2000,
     F   1300-1400'
./edit faculty-default-clustering 'Ren Quinn' 'MT' 0
./edit make-section 'CS 1400-02' 'flex' '3 credit bell schedule'
./edit make-section 'CS 1400-03' 'flex' '3 credit bell schedule'
./edit make-section 'CS 1410-02' 'flex' '3 credit bell schedule'
./edit make-section 'CS 2450-01' 'flex' '3 credit bell schedule'
./edit make-section 'CS 3150-01' 'flex' '3 credit bell schedule'
./edit make-section 'CS 4991R-50' 'Smith 116' 'R1900+50'
./edit make-section 'CS 4992R-01' 'Smith 109' 'F1300+50'
./edit assign-faculty-sections 'Ren Quinn' 'CS 1400-02' 'CS 1400-03' 'CS 1410-02' 'CS 2450-01' 'CS 3150-01' 'CS 4991R-50' 'CS 4992R-01'

./edit make-faculty 'Russ Ross' Computing \
    'MTWR 1200-1500'
./edit faculty-default-clustering 'Russ Ross' 'MT' 0
./edit make-section 'CS 2810-01' 'Smith 109' '3 credit bell schedule'
./edit make-section 'CS 2810-02' 'Smith 109' '3 credit bell schedule'
./edit make-section 'CS 3400-01' 'Smith 109' '3 credit bell schedule'
./edit make-section 'CS 4307-01' 'Smith 109' '3 credit bell schedule'
./edit assign-faculty-sections 'Russ Ross' 'CS 2810-01' 'CS 2810-02' 'CS 3400-01' 'CS 4307-01'

./edit make-faculty 'Rex Frisbey' Computing \
    'MWF 1100-1200'
./edit make-section 'SE 1400-01' 'macs' '3 credit bell schedule'
./edit assign-faculty-sections 'Rex Frisbey' 'SE 1400-01'

./edit make-faculty 'Jamie Bennion' Computing \
    'W 1800-2030'
./edit make-section 'IT 4990-01' 'flex' '1×150 evening'
./edit assign-faculty-sections 'Jamie Bennion' 'IT 4990-01'

./edit add-cross-listing cs-se-senior-project 'SE 4600-01' 'CS 4600-02'
./edit add-anti-conflict 50 'CS 4600-01' 'CS 4600-02'
./edit add-anti-conflict 50 'CS 1030-01' 'CS 1400'
#./edit add-anti-conflict 50 'SE 1400' 'IT 1100' # temporarily removed because of new hire planning
