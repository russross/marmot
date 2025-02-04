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
./edit make-conflict 'Computer Science' '3rd/4th semester bottleneck classes' 100 maximize \
    'CS 2420' 'CS 2450' 'CS 2810' 'CS 3005'
./edit make-conflict 'Computer Science' 'core requirements' 99 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2420' 'CS 2450' 'CS 2810' 'CS 3005' 'CS 3530' 'CS 3510' 'CS 4600' \
    'CS 3150' 'CS 3400' 'CS 3410' 'CS 3520' 'CS 3600' 'CS 4300' 'CS 4307' 'CS 4320' 'CS 4550' 'SE 3200' \
    'MATH 1210' 'MATH 3400' 'CS 2100'
./edit make-conflict 'Computer Science' 'electives' 30 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2420' 'CS 2450' 'CS 2810' 'CS 3005' 'CS 3530' 'CS 3510' 'CS 4600' \
    'CS 3150' 'CS 3400' 'CS 3410' 'CS 3520' 'CS 3600' 'CS 4300' 'CS 4307' 'CS 4320' 'CS 4550' 'SE 3200' \
    'MATH 1210' 'MATH 3400' 'CS 2100' \
    'SE 1400' 'SE 3010' 'SE 3020' 'SE 3100' 'SE 3400' 'SE 4200' 'SE 3150' 'SE 3450' \
    'IT 1100' 'IT 2700' 'IT 3100' 'IT 3110' 'IT 4200'
./edit make-conflict 'Computer Science' 'math choices' 35 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2420' 'CS 2450' 'CS 2810' 'CS 3005' 'CS 3530' 'CS 3510' 'CS 4600' \
    'CS 3150' 'CS 3400' 'CS 3410' 'CS 3520' 'CS 3600' 'CS 4300' 'CS 4307' 'CS 4320' 'CS 4550' 'SE 3200' \
    'MATH 1210' 'MATH 3400' 'CS 2100' \
    'MATH 1220' 'MATH 2210' 'MATH 2250' 'MATH 2270' 'MATH 2280' \
    'MATH 3050' 'MATH 3450' 'MATH 3605' 'MATH 3905' 'MATH 4005'

./edit make-program 'Data Science' Computing
./edit make-conflict 'Data Science' 'core requirements' 99 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2100' 'CS 2420' 'CS 2450' 'CS 2500' 'CS 2810' \
    'CS 3005' 'CS 3410' 'CS 3510' 'CS 4300' 'CS 4307' 'CS 4320' 'CS 4400' 'CS 4410' 'CS 4600' \
    'MATH 1210' 'MATH 1220' 'MATH 2270' 'MATH 3400' 'IT 1500'

./edit make-program 'Software Engineering' Computing
./edit make-conflict 'Software Engineering' 'core requirements' 99 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2100' 'CS 2420' 'CS 2810' 'CS 3005' \
    'CS 3150' 'CS 3510' 'CS 4307' 'IT 2300' 'IT 1100' \
    'SE 1400' 'CS 2450' 'SE 3010' 'SE 3020' 'SE 3100' 'SE 3150' 'SE 3200' 'SE 3400' \
    'SE 4200' 'SE 4600' 'MATH 1100' 'MATH 1210' 'MATH 2050'
./edit make-conflict 'Software Engineering' 'Entrepreneurial and marketing track' 45 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2100' 'CS 2420' 'CS 2810' 'CS 3005' \
    'CS 3150' 'CS 3510' 'CS 4307' 'IT 2300' 'IT 1100' \
    'SE 1400' 'CS 2450' 'SE 3010' 'SE 3020' 'SE 3100' 'SE 3150' 'SE 3200' 'SE 3400' \
    'SE 4200' 'SE 4600' 'MATH 1100' 'MATH 1210' 'MATH 2050' \
    'SE 3500' 'SE 3550'
./edit make-conflict 'Software Engineering' 'DevOps track' 45 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2100' 'CS 2420' 'CS 2810' 'CS 3005' \
    'CS 3150' 'CS 3510' 'CS 4307' 'IT 2300' 'IT 1100' \
    'SE 1400' 'CS 2450' 'SE 3010' 'SE 3020' 'SE 3100' 'SE 3150' 'SE 3200' 'SE 3400' \
    'SE 4200' 'SE 4600' 'MATH 1100' 'MATH 1210' 'MATH 2050' \
    'IT 3110' 'IT 3300' 'IT 4200'
./edit make-conflict 'Software Engineering' 'Application track' 45 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2100' 'CS 2420' 'CS 2810' 'CS 3005' \
    'CS 3150' 'CS 3510' 'CS 4307' 'IT 2300' 'IT 1100' \
    'SE 1400' 'CS 2450' 'SE 3010' 'SE 3020' 'SE 3100' 'SE 3150' 'SE 3200' 'SE 3400' \
    'SE 4200' 'SE 4600' 'MATH 1100' 'MATH 1210' 'MATH 2050' \
    'SE 3250' 'SE 3450'
./edit make-conflict 'Software Engineering' 'Data science track' 45 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2100' 'CS 2420' 'CS 2810' 'CS 3005' \
    'CS 3150' 'CS 3510' 'CS 4307' 'IT 2300' 'IT 1100' \
    'SE 1400' 'CS 2450' 'SE 3010' 'SE 3020' 'SE 3100' 'SE 3150' 'SE 3200' 'SE 3400' \
    'SE 4200' 'SE 4600' 'MATH 1100' 'MATH 1210' 'MATH 2050' \
    'CS 4300' 'CS 4400' 'CS 4320' 'CS 4410'
./edit make-conflict 'Software Engineering' 'only need one of AI/data mining' 0 minimize \
    'CS 4300' 'CS 4400'
./edit make-conflict 'Software Engineering' 'Virtual reality track' 45 maximize \
    'CS 1030' 'CS 1400' 'CS 1410' 'CS 2100' 'CS 2420' 'CS 2810' 'CS 3005' \
    'CS 3150' 'CS 3510' 'CS 4307' 'IT 2300' 'IT 1100' \
    'SE 1400' 'CS 2450' 'SE 3010' 'SE 3020' 'SE 3100' 'SE 3150' 'SE 3200' 'SE 3400' \
    'SE 4200' 'SE 4600' 'MATH 1100' 'MATH 1210' 'MATH 2050' \
    'CS 3500' 'CS 4995' 'CS 4996'
./edit make-conflict 'Software Engineering' 'only need one database class' 0 minimize \
    'CS 4307' 'IT 2300'
./edit make-conflict 'Software Engineering' 'only need one calculus class' 0 minimize \
    'MATH 1100' 'MATH 1210'

./edit make-program 'Information Technology' Computing
./edit make-conflict 'Information Technology' 'core requirements' 99 maximize \
    'IT 1100' 'IT 1200' 'IT 1500' 'IT 2300' 'IT 2400' 'IT 2500' 'IT 2700' \
    'IT 3100' 'IT 3150' 'IT 3400' 'IT 4600' 'MATH 1040' 'MATH 1050'
./edit make-conflict 'Information Technology' 'core electives' 60 maximize \
    'IT 1100' 'IT 1200' 'IT 1500' 'IT 2300' 'IT 2400' 'IT 2500' 'IT 2700' \
    'IT 3100' 'IT 3150' 'IT 3400' 'IT 4600' 'MATH 1040' 'MATH 1050' \
    'IT 3110' 'IT 3300' 'IT 3710' 'IT 4100' 'IT 4200' 'IT 4310' 'IT 4400' 'IT 4510' 'IT 4920R'
./edit make-conflict 'Information Technology' 'electives' 30 maximize \
    'IT 1100' 'IT 1200' 'IT 1500' 'IT 2300' 'IT 2400' 'IT 2500' 'IT 2700' \
    'IT 3100' 'IT 3150' 'IT 3400' 'IT 4600' 'MATH 1040' 'MATH 1050' \
    'IT 3110' 'IT 3300' 'IT 3710' 'IT 4060' 'IT 4070' 'IT 4100' 'IT 4200' 'IT 4310' 'IT 4400' 'IT 4510' 'IT 4920R'
./edit make-conflict 'Information Technology' 'DevOps requirements' 99 maximize \
    'IT 1100' 'IT 1200' 'IT 1500' 'IT 2300' 'IT 2400' 'IT 2500' 'IT 2700' \
    'IT 3100' 'IT 3150' 'IT 3400' 'IT 4600' 'MATH 1040' 'MATH 1050' \
    'IT 3110' 'IT 3300' 'IT 4200' 'CS 2450'
./edit make-conflict 'Information Technology' 'Cybersecurity requirements' 99 maximize \
    'IT 1100' 'IT 1200' 'IT 1500' 'IT 2300' 'IT 2400' 'IT 2500' 'IT 2700' \
    'IT 3100' 'IT 3150' 'IT 3400' 'IT 4600' 'MATH 1040' 'MATH 1050' \
    'IT 3710' 'IT 4400' 'IT 4510'
./edit make-conflict 'Information Technology' 'Cybersecurity choose 2' 99 maximize \
    'IT 1100' 'IT 1200' 'IT 1500' 'IT 2300' 'IT 2400' 'IT 2500' 'IT 2700' \
    'IT 3100' 'IT 3150' 'IT 3400' 'IT 4600' 'MATH 1040' 'MATH 1050' \
    'IT 3710' 'IT 4400' 'IT 4510' \
    'IT 4310' 'IT 4990' 'CS 2420' 'CS 2810'
./edit make-conflict 'Information Technology' 'only need one math class' 0 minimize \
    'MATH 1040' 'MATH 1050'


echo building computing faculty and sections
# updated
./edit make-faculty 'Bart Stander' Computing \
    'MWF 0900-1200,
     MW  1200-1330 with penalty 4,
     MW  1330-1630,
     TR  0900-1630 with penalty 9'
./edit faculty-custom-clustering 'Bart Stander' 'MT' 1 \
     'cluster,too_short,110,4' \
     'cluster,too_long,255,9' \
     'gap,too_short,60,9' \
     'gap,too_long,105,4' \
     'gap,too_long,195,9'
./edit make-section 'CS 2100-01' 'stadium' '3 credit bell schedule'
./edit make-section 'CS 2420-01' 'stadium' '3 credit bell schedule'
./edit make-section 'CS 3600-01' 'pcs' 'stadium:9' '3 credit bell schedule'
./edit make-section 'CS 4550-01' 'pcs' '3 credit bell schedule'
./edit assign-faculty-sections 'Bart Stander' 'CS 2100-01' 'CS 2420-01' 'CS 3600-01' 'CS 4550-01'

# updated
./edit make-faculty 'Carol Stander' Computing \
    'MWF 0900-1000 with penalty 9,
     MWF 1000-1200,
     MW  1200-1330 with penalty 9,
     MW  1330-1630,
     TR  0900-1030 with penalty 9,
     TR  1030-1500 with penalty 4'
./edit faculty-custom-clustering 'Carol Stander' 'MT' 1 \
    'cluster,too_long,120,19' \
    'gap,too_short,60,9' \
    'gap,too_long,105,4' \
    'gap,too_long,195,9'
./edit make-section 'CS 1030-01' 'flex' 'pcs' 'MW1200+75' 'MW1330+75' 'MWF 3×50 bell schedule'
./edit make-section 'CS 1410-02' 'flex' 'MWF 3×50 bell schedule' '2×75 bell schedule:9'
./edit make-section 'IT 2300-01' 'flex' '2×75 bell schedule' 'MWF 3×50 bell schedule:9'
./edit make-section 'CS 1410-40'
./edit make-section 'IT 1100-40'
./edit assign-faculty-sections 'Carol Stander' 'CS 1030-01' 'CS 1410-02' 'CS 1410-40' 'IT 1100-40' 'IT 2300-01'

# updated
./edit make-faculty 'Curtis Larsen' Computing \
    'MTWRF 0800-0900 with penalty 9,
     MTWR 0900-1630,
     F    0900-1030,
     F    1330-1630'
./edit faculty-default-clustering 'Curtis Larsen' 'MT' 0
./edit make-section 'CS 3005-01' 'Smith 116' 'MWF 3×50 bell schedule'
./edit make-section 'CS 3005-02' 'Smith 116' 'MWF 3×50 bell schedule'
./edit make-section 'CS 4320-01' 'Smith 116' '3 credit bell schedule'
./edit make-section 'CS 4600-01' 'Smith 116' '3 credit bell schedule'
./edit make-section 'CS 4920R-01'
./edit assign-faculty-sections 'Curtis Larsen' 'CS 3005-01' 'CS 3005-02' 'CS 4320-01' 'CS 4600-01' 'CS 4920R-01'

# updated
./edit make-faculty 'DJ Holt' Computing \
    'MW 1200-1500,
     TR 1200-1630'
./edit faculty-default-clustering 'DJ Holt' 'MT' 0
./edit make-section 'SE 3010-01' 'macs' '3 credit bell schedule'
./edit make-section 'SE 4200-01' 'macs' '3 credit bell schedule'
./edit make-section 'CS 4600-02' 'stadium' 'flex:4' 'macs:9' '3 credit bell schedule'
./edit assign-faculty-sections 'DJ Holt' 'SE 3010-01' 'SE 4200-01' 'CS 4600-02'

# updated
./edit make-faculty 'Eric Pedersen' Computing \
    'TR  1200-1330'
./edit make-section 'SE 3500-01' 'flex' 'TR1200+75'
./edit assign-faculty-sections 'Eric Pedersen' 'SE 3500-01'

# updated
./edit make-faculty 'Jay Sneddon' Computing \
    'MWF 0900-1200 with penalty 9,
     MW  1200-1500,
     TR  0900-1200,
     TR  1200-1500 with penalty 4'
./edit faculty-default-clustering 'Jay Sneddon' 'MT' 0
./edit make-section 'IT 1200-01' 'Smith 107' '3 credit bell schedule'
./edit make-section 'IT 2700-01' 'Smith 107' '3 credit bell schedule'
./edit make-section 'IT 3150-01' 'Smith 107' '3 credit bell schedule'
./edit make-section 'IT 3400-01' 'Smith 107' '3 credit bell schedule'
./edit make-section 'IT 3710-40'
./edit assign-faculty-sections 'Jay Sneddon' 'IT 1200-01' 'IT 2700-01' 'IT 3150-01' 'IT 3400-01' 'IT 3710-40'

./edit make-faculty 'Jeff Compas' Computing \
    'MWF 0800-0900 with penalty 9,
     MWF 0900-1200,
     MW  1200-1630,
     TR  0900-1630'
./edit faculty-default-clustering 'Jeff Compas' 'MT' 0
./edit make-section 'CS 1400-03' 'stadium' '3 credit bell schedule'
./edit make-section 'CS 1400-04' 'stadium' '3 credit bell schedule'
./edit make-section 'CS 2450-01' 'flex' '3 credit bell schedule'
./edit make-section 'SE 3100-01' 'flex' '3 credit bell schedule'
./edit assign-faculty-sections 'Jeff Compas' 'CS 1400-03' 'CS 1400-04' 'CS 2450-01' 'SE 3100-01'

# updated
./edit make-faculty 'Joe Francom' Computing \
    'MWF 0800-1200,
     MW  1200-1500,
     TR  0900-1200 with penalty 9'
./edit faculty-default-clustering 'Joe Francom' 'MT' 1
./edit make-section 'IT 1500-40'
./edit make-section 'IT 3110-01' 'flex' '3 credit bell schedule'
./edit make-section 'IT 4600-01' 'flex' 'stadium' 'MWF0800+50'
./edit make-section 'SE 3200-01' 'flex' '3 credit bell schedule'
./edit assign-faculty-sections 'Joe Francom' 'IT 1500-40' 'IT 3110-01' 'IT 4600-01' 'SE 3200-01'

#./edit make-faculty 'Lora Klein' Computing \
#    'TR 0900-1500,
#     MW 1500-1630 with penalty 15'
#./edit faculty-default-clustering 'Lora Klein' 'MT' no_preference
#./edit make-section 'SE 3200-01' 'Smith 107:4' 'flex' '3 credit bell schedule'
#./edit assign-faculty-sections 'Lora Klein' 'SE 3200-01'

# updated
./edit make-faculty 'Matt Kearl' Computing \
    'MW 1200-1330 with penalty 9,
     TR 0900-1330,
     TR 1330-1500 with penalty 9'
./edit faculty-custom-clustering 'Matt Kearl' 'MT' 1 \
    'cluster,too_short,165,9'
./edit make-section 'SE 1400-01' 'macs' 'flex:4' '3 credit bell schedule'
./edit make-section 'SE 1400-40'
./edit make-section 'SE 3450-01' 'flex:4' 'macs' '3 credit bell schedule'
./edit make-section 'SE 3550-01' 'flex:4' 'macs' '3 credit bell schedule'
./edit make-section 'SE 4920-01'
./edit assign-faculty-sections 'Matt Kearl' 'SE 1400-01' 'SE 1400-40' 'SE 3450-01' 'SE 3550-01' 'SE 4920-01'

# updated
./edit make-faculty 'Phil Daley' Computing \
    'MWF 0900-1200 with penalty 9,
     MW  1200-1630,
     TR  0900-1630'
./edit faculty-default-clustering 'Phil Daley' 'MT' 0
./edit make-section 'IT 1100-01' 'pcs' '3 credit bell schedule'
./edit make-section 'IT 1100-02' 'pcs' '3 credit bell schedule'
./edit make-section 'IT 2400-01' 'Smith 107' '3 credit bell schedule'
./edit make-section 'IT 2400-40'
./edit make-section 'IT 3100-01' 'Smith 107' '3 credit bell schedule'
./edit assign-faculty-sections 'Phil Daley' 'IT 1100-01' 'IT 1100-02' 'IT 2400-01' 'IT 2400-40' 'IT 3100-01'

./edit make-faculty 'Ren Quinn' Computing \
    'MWF 0900-1200,
     MW  1200-1630,
     TR  1330-1630,
     R   1900-2000,
     F   1300-1400'
./edit faculty-default-clustering 'Ren Quinn' 'MT' 0
./edit make-section 'CS 1400-01' 'flex' 'MWF 3×50 bell schedule' 'MW 2×75 bell schedule'
./edit make-section 'CS 1400-02' 'flex' 'TR 2×75 bell schedule'
./edit make-section 'CS 1410-01' 'flex' '3 credit bell schedule'
./edit make-section 'CS 3150-01' 'flex' '3 credit bell schedule'
./edit make-section 'CS 4800R-01'
./edit make-section 'CS 4991R-50' 'Smith 116' 'R1900+50'
./edit make-section 'CS 4992R-01' 'Smith 109' 'F1300+50'
./edit assign-faculty-sections 'Ren Quinn' 'CS 1400-01' 'CS 1400-02' 'CS 1410-01' 'CS 3150-01' 'CS 4800R-01' 'CS 4991R-50' 'CS 4992R-01'

# updated
./edit make-faculty 'Russ Ross' Computing \
    'MW 1200-1500,
     MW 1500-1630 with penalty 9,
     TR 1030-1200 with penalty 9,
     TR 1200-1500,
     TR 1500-1630 with penalty 9'
./edit faculty-default-clustering 'Russ Ross' 'MT' 0
./edit make-section 'CS 2810-01' 'Smith 109' '3 credit bell schedule'
./edit make-section 'CS 2810-02' 'Smith 109' '3 credit bell schedule'
./edit make-section 'CS 3410-01' 'Smith 109' '3 credit bell schedule'
./edit make-section 'CS 4307-01' 'Smith 109' '3 credit bell schedule'
./edit make-section 'CS 4800R-02'
./edit assign-faculty-sections 'Russ Ross' 'CS 2810-01' 'CS 2810-02' 'CS 3410-01' 'CS 4307-01' 'CS 4800R-02'

# updated
./edit make-faculty 'Yuanfei Sun' Computing \
    'MWF 0900-1200,
     MW  1200-1745'
./edit faculty-default-clustering 'Yuanfei Sun' 'MT' 1
./edit make-section 'CS 3510-01' 'flex' '3 credit bell schedule'
./edit make-section 'CS 3510-02' 'flex' '3 credit bell schedule'
./edit make-section 'CS 2320-01' 'flex' '3 credit bell schedule'
./edit assign-faculty-sections 'Yuanfei Sun' 'CS 3510-01' 'CS 3510-02' 'CS 2320-01'

# updated
./edit make-faculty 'unknown cyber' Computing \
    'MWF 0800-1200,
     MW  1200-1500,
     TR  0900-1500'
./edit make-section 'IT 4510-01' 'flex' '3 credit bell schedule'
./edit make-section 'IT 4990-01' 'flex' 'stadium' '3 credit bell schedule'
./edit assign-faculty-sections 'unknown cyber' 'IT 4510-01' 'IT 4990-01'

./edit add-cross-listing 'CS 4600-02' 'SE 4600-01'
./edit add-anti-conflict 99 'CS 4600-01' 'CS 4600-02'
./edit add-anti-conflict 50 'CS 1030-01' 'CS 1400'

./edit make-conflict 'Computer Science' 'spread out CS 1400' 100 maximize \
    'CS 1400-01' 'CS 1400-02' 'CS 1400-03' 'CS 1400-04'
./edit make-conflict 'Computer Science' 'spread out CS 1410' 100 maximize \
    'CS 1410-01' 'CS 1410-02'
./edit make-conflict 'Computer Science' 'spread out CS 2810' 100 maximize \
    'CS 2810-01' 'CS 2810-02'
./edit make-conflict 'Information Technology' 'spread out IT 1100' 100 maximize \
    'IT 1100-01' 'IT 1100-02'
