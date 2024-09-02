set -e

echo building math building and classrooms
./edit make-building Snow
./edit make-room 'Snow 103' 16
./edit make-room 'Snow 112' 42 'Math lecture'
./edit make-room 'Snow 113' 36
./edit make-room 'Snow 124' 42 'Math lecture'
./edit make-room 'Snow 125' 42 'Math lecture'
./edit make-room 'Snow 144' 42 'Math lecture'
./edit make-room 'Snow 145' 42 'Math lecture'
./edit make-room 'Snow 147' 42 'Math lecture'
./edit make-room 'Snow 150' 42 'Math lecture'
./edit make-room 'Snow 151' 42 'Math lecture'
./edit make-room 'Snow 204' 10
./edit make-room 'Snow 3' 42 'Math lecture'


echo building math time slots
./edit make-time-slot 'MW0730+75' '3 credit bell schedule' '2×75 bell schedule' 'MW 2×75 bell schedule'
./edit make-time-slot 'TR0730+75' '3 credit bell schedule' '2×75 bell schedule' 'TR 2×75 bell schedule'
./edit make-time-slot 'MTRF0800+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTRF0900+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTRF1000+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTRF1100+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTRF1200+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTRF1300+50' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTRF1400+50' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTRF1500+50' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTWF0800+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTWF0900+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTWF1000+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTWF1100+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTWF1200+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTWF1300+50' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTWF1400+50' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTWF1500+50' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTWR0800+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTWR0900+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTWR1000+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTWR1100+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTWR1200+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTWR1300+50' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTWR1400+50' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTWR1500+50' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MWRF0800+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MWRF0900+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MWRF1000+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MWRF1100+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MWRF1200+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MWRF1300+50' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MWRF1400+50' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MWRF1500+50' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'TWRF0800+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'TWRF0900+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'TWRF1000+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'TWRF1100+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'TWRF1200+50' '4 credit bell schedule' '4 credit 4×50 bell schedule' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'TWRF1300+50' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'TWRF1400+50' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'TWRF1500+50' '4 credit 4×50 extended bell schedule'
./edit make-time-slot 'MTWRF0800+50' '5 credit bell schedule' '5 credit extended bell schedule'
./edit make-time-slot 'MTWRF0900+50' '5 credit bell schedule' '5 credit extended bell schedule'
./edit make-time-slot 'MTWRF1000+50' '5 credit bell schedule' '5 credit extended bell schedule'
./edit make-time-slot 'MTWRF1100+50' '5 credit bell schedule' '5 credit extended bell schedule'
./edit make-time-slot 'MTWRF1200+50' '5 credit bell schedule' '5 credit extended bell schedule'
./edit make-time-slot 'MTWRF1300+50' '5 credit extended bell schedule'
./edit make-time-slot 'MTWRF1400+50' '5 credit extended bell schedule'
./edit make-time-slot 'MTWRF1500+50' '5 credit extended bell schedule'
./edit make-time-slot 'MTWRF1600+50' '5 credit extended bell schedule'
./edit make-time-slot 'MW1300+100' '4 credit bell schedule' '4 credit 2×100 bell schedule'
./edit make-time-slot 'MW1500+100' '4 credit bell schedule' '4 credit 2×100 bell schedule'
./edit make-time-slot 'TR1300+100' '4 credit bell schedule' '4 credit 2×100 bell schedule'
./edit make-time-slot 'TR1500+100' '4 credit bell schedule' '4 credit 2×100 bell schedule'
./edit make-time-slot 'MWF1200+50'
./edit make-time-slot 'MW1600+100'
./edit make-time-slot 'MW1630+100'
./edit make-time-slot 'MW1645+75'
./edit make-time-slot 'MW1800+100'
./edit make-time-slot 'TR1630+100'
./edit make-time-slot 'TR1800+100'
./edit make-time-slot 'T1500+75'
./edit make-time-slot 'T1630+150'
./edit make-time-slot 'W1630+150'
./edit make-time-slot 'F1100+50'

echo building math conflicts

./edit make-program Math Math
./edit make-conflict Math 'core requirements' 99 maximize \
    'MATH 1210' 'MATH 1220' 'MATH 2200' 'MATH 2210' 'MATH 2270' 'MATH 2280' \
    'MATH 3200' 'MATH 3400' 'MATH 3900' 'MATH 4000' 'MATH 4900' \
    'CS 1400' 'CS 2100'
./edit make-conflict Math electives 30 maximize \
    'MATH 1210' 'MATH 1220' 'MATH 2200' 'MATH 2210' 'MATH 2270' 'MATH 2280' \
    'MATH 3200' 'MATH 3400' 'MATH 3900' 'MATH 4000' 'MATH 4900' \
    'CS 1400' 'CS 2100' \
    'MATH 3000' 'MATH 3100' 'MATH 3150' 'MATH 3210' 'MATH 3450' \
    'MATH 3500' 'MATH 3605' 'MATH 3700' \
    'MATH 4010' 'MATH 4100' 'MATH 4200' 'MATH 4250' 'MATH 4550' \
    'MATH 4800' 'MATH 4890R'
./edit make-program 'Math ACM Data Analytics' Math
./edit make-conflict 'Math ACM Data Analytics' 'core requirements' 99 maximize \
    'CS 1400' 'CS 1410' \
    'MATH 1210' 'MATH 1220' 'MATH 2200' 'CS 2100' \
    'MATH 2210' 'MATH 2270' 'MATH 2280' 'MATH 3400' 'MATH 3700' \
    'MATH 4250' 'MATH 4800' 'MATH 4890R' 'MATH 4900' \
    'MATH 2050' 'MATH 3050' 'MATH 3450' \
    'IT 1100' 'IT 2300' 'IT 2400' 'IT 4310'
./edit make-conflict 'Math ACM Data Analytics' electives 30 maximize \
    'CS 1400' 'CS 1410' \
    'MATH 1210' 'MATH 1220' 'MATH 2200' 'CS 2100' \
    'MATH 2210' 'MATH 2270' 'MATH 2280' 'MATH 3400' 'MATH 3700' \
    'MATH 4250' 'MATH 4800' 'MATH 4890R' 'MATH 4900' \
    'MATH 2050' 'MATH 3050' 'MATH 3450' \
    'IT 1100' 'IT 2300' 'IT 2400' 'IT 4310' \
    'CS 3005' 'IT 4510' \
    'MATH 3100' 'MATH 3150' 'MATH 3120' 'MATH 3200' 'MATH 3500' \
    'MATH 3900' 'MATH 3905' 'MATH 4000' 'MATH 4005' 'MATH 4010' \
    'MATH 4100' 'MATH 4200' 'MATH 4330' 'MATH 4550'
./edit make-conflict 'Math ACM Data Analytics' 'discrete math' 0 minimize \
    'MATH 2200' 'CS 2100'

./edit make-program 'Math Education' Math
./edit make-conflict 'Math Education' 'core requirements' 99 maximize \
    'MATH 1040' 'MATH 1210' 'MATH 1220' 'MATH 2200' 'MATH 2210' \
    'MATH 2270' 'MATH 2280' 'MATH 3000' 'MATH 3010' 'MATH 3020' \
    'MATH 3100' 'MATH 3120' 'MATH 3200' 'MATH 3400' 'MATH 4000' \
    'CS 1400'

./edit make-program 'Math ACM Scientific Computing' Math
./edit make-conflict 'Math ACM Scientific Computing' 'core requirements' 99 maximize \
    'CS 1400' 'CS 1410' \
    'MATH 1210' 'MATH 1220' 'MATH 2200' 'CS 2100' \
    'MATH 2210' 'MATH 2270' 'MATH 2280' 'MATH 3400' 'MATH 3700' \
    'MATH 4250' 'MATH 4800' 'MATH 4890R' 'MATH 4900' \
    'CS 2420' 'CS 3005' \
    'MATH 2050' 'MATH 3150' 'MATH 3500' 'MATH 4550'
./edit make-conflict 'Math ACM Scientific Computing' electives 30 maximize \
    'CS 1400' 'CS 1410' \
    'MATH 1210' 'MATH 1220' 'MATH 2200' 'CS 2100' \
    'MATH 2210' 'MATH 2270' 'MATH 2280' 'MATH 3400' 'MATH 3700' \
    'MATH 4250' 'MATH 4800' 'MATH 4890R' 'MATH 4900' \
    'CS 2420' 'CS 3005' \
    'MATH 2050' 'MATH 3150' 'MATH 3500' 'MATH 4550' \
    'MATH 3050' 'MATH 3450' 'MATH 3120' 'MATH 3100' 'MATH 3900' 'MATH 3905' \
    'MATH 4000' 'MATH 4005' 'MATH 4010' 'MATH 4100' 'MATH 4330' \
    'MATH 3200' 'MATH 4200'
./edit make-conflict 'Math ACM Scientific Computing' 'discrete math' 0 minimize \
    'MATH 2200' 'CS 2100'

./edit make-program 'Math ACM Actuarial Sciences' Math
./edit make-conflict 'Math ACM Actuarial Sciences' 'core requirements' 99 maximize \
    'CS 1400' 'CS 1410' \
    'MATH 1210' 'MATH 1220' 'MATH 2200' 'CS 2100' \
    'MATH 2210' 'MATH 2270' 'MATH 2280' 'MATH 3400' 'MATH 3700' \
    'MATH 4250' 'MATH 4800' 'MATH 4890R' 'MATH 4900' \
    'CS 2420' \
    'MATH 3410' 'MATH 3450' 'MATH 4400' 'MATH 4410'
./edit make-conflict 'Math ACM Actuarial Sciences' electives 30 maximize \
    'CS 1400' 'CS 1410' \
    'MATH 1210' 'MATH 1220' 'MATH 2200' 'CS 2100' \
    'MATH 2210' 'MATH 2270' 'MATH 2280' 'MATH 3400' 'MATH 3700' \
    'MATH 4250' 'MATH 4800' 'MATH 4890R' 'MATH 4900' \
    'CS 2420' \
    'MATH 3410' 'MATH 3450' 'MATH 4400' 'MATH 4410' \
    'MATH 3050' 'MATH 3120' 'MATH 3150' 'MATH 3200' 'MATH 3100' \
    'MATH 3500' 'MATH 3900' 'MATH 3905' 'MATH 4000' 'MATH 4005' \
    'MATH 4200' 'MATH 4010' 'MATH 4100' 'MATH 4330' 'MATH 4550'
./edit make-conflict 'Math ACM Actuarial Sciences' 'discrete math' 0 minimize \
    'MATH 2200' 'CS 2100'


echo building math faculty

./edit make-faculty "Amanda Fa'onelua" Math 'TR 1300-1500'
# TR1300+100

./edit make-faculty 'Bhuvaneswari Sambandham' Math 'MTWRF 0900-1700'
# MTWF1000+50, MTWR1100+50, MW1200+75

./edit make-faculty 'Clare C Banks' Math 'MTWRF 0800-1700'
# MTWR0800+50, MTWR1200+50

./edit make-faculty 'Costel Ionita' Math 'MTWRF 0800-1700'
# F1100+50, MTWR0800+50, MTWR0900+50, MTWR1100+50, TR1200+75

./edit make-faculty 'Craig D Seegmiller' Math 'MWF 0900-1700, TR 0700-1700'
# MTWR1200+50, TR0730+75, TR0900+75

./edit make-faculty 'Dawn Lashell Kidd-Thomas' Math 'TR 1200-1700'
# TR1300+100

./edit make-faculty 'Elizabeth Karen Ludlow' Math 'MW 1300-1700'
# MW1300+100, MW1500+75

./edit make-faculty 'James P Fitzgerald' Math 'MTWRF 0800-1200'
# MWF0800+50, MWF0900+50, MWF1000+50

./edit make-faculty 'Jameson C Hardy' Math 'MTWRF 0900-1700'
# MTWR0900+50, MTWRF1000+50, MW1200+75, TR1200+75

./edit make-faculty 'Jeffrey P Harrah' Math 'MRF 0800-1700, TW 0800-1900'
# T1630+150, TR1030+75, TR1200+75, TR1330+75, W1630+150

./edit make-faculty 'Jie Liu' Math 'MTWRF 0900-1700'
# T1500+75, TR1030+75, TR1200+75, TR1330+75

./edit make-faculty 'Kathryn E Ott' Math 'MW 1300-1700'
# MW1300+100

./edit make-faculty 'Md Sazib Hasan' Math 'TR 0900-1200'
# TR0900+75, TR1030+75

./edit make-faculty 'Michael N Paxman' Math 'TR 1630-1900'
# TR1630+100

./edit make-faculty 'Odean Bowler' Math 'MTWRF 1500-1700'
# MW1500+100, TR1500+100

./edit make-faculty 'Paula Manuele Temple' Math 'MTWRF 0900-1700'
# MTWR1200+50, MW1300+100, MW1500+75, TR1300+100

./edit make-faculty 'Robert T Reimer' Math 'MW 1630-1800'
# MW1630+75

./edit make-faculty 'Ross C Decker' Math 'TR 0900-1200'
# TR0900+75, TR1030+75

./edit make-faculty 'Ryan C McConnell' Math 'TR 1630-1800'
# TR1630+75

./edit make-faculty 'Scott Patrick Hicks' Math 'MW 1600-2000'
# MW1600+100, MW1800+100

./edit make-faculty 'Steven McKay Sullivan' Math 'MTWRF 0900-1700'
# MTWR0900+50, MWF1000+50, TR1030+75

./edit make-faculty 'Trevor K Johnson' Math 'MTWRF 0900-1700'
# MTWR1200+50, MW1330+75

./edit make-faculty 'Tye K Rogers' Math 'MTWRF 0800-1700'
# MTWR0800+50, MTWR1000+50, MWF1100+50, TR1330+75

./edit make-faculty 'Vinodh Kumar Chellamuthu' Math 'MW 1500-1800'
# MW1500+100, MW1645+75

./edit make-faculty 'Violeta Adina Ionita' Math 'MTWRF 0800-1700'
# MTWR0800+50, MTWR0900+50, MTWR1100+50, MTWR1200+50

#
# math sections
#

echo building math sections

# MATH 0900-01: Transitional Math I
# assigned to SNOW 144 at MTWR1200+50
./edit make-section 'MATH 0900-01' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Paula Manuele Temple' 'MATH 0900-01'

# MATH 0900-02: Transitional Math I
# assigned to SNOW 144 at MTWR0900+50
./edit make-section 'MATH 0900-02' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Jameson C Hardy' 'MATH 0900-02'

# MATH 0900-03: Transitional Math I
# assigned to SNOW 144 at MW1300+100
./edit make-section 'MATH 0900-03' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Paula Manuele Temple' 'MATH 0900-03'

# MATH 0900-04: Transitional Math I
# assigned to SNOW 144 at MW1600+100
./edit make-section 'MATH 0900-04' 'Math lecture' 'MW1600+100'
./edit assign-faculty-sections 'Scott Patrick Hicks' 'MATH 0900-04'

# MATH 0900-06: Transitional Math I
# assigned to SNOW 3 at TR1630+100
./edit make-section 'MATH 0900-06' 'Math lecture' 'TR1630+100'

# MATH 0900-07: Transitional Math I
# assigned to SNOW 144 at TR1300+100
./edit make-section 'MATH 0900-07' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Paula Manuele Temple' 'MATH 0900-07'

# MATH 0900-51: Transitional Math I
# assigned to SNOW 144 at MW1800+100
./edit make-section 'MATH 0900-51' 'Math lecture' 'MW1800+100'
./edit assign-faculty-sections 'Scott Patrick Hicks' 'MATH 0900-51'

# MATH 0980-03: Transitional Math IIB
# assigned to SNOW 144 at MTWR1000+50
./edit make-section 'MATH 0980-03' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Tye K Rogers' 'MATH 0980-03'

# MATH 0980-05: Transitional Math IIB
# assigned to SNOW 144 at TR1630+100
./edit make-section 'MATH 0980-05' 'Math lecture' 'TR1630+100'
./edit assign-faculty-sections 'Michael N Paxman' 'MATH 0980-05'

# MATH 0980-06: Transitional Math IIB
# assigned to SNOW 144 at MTWR0800+50
./edit make-section 'MATH 0980-06' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Tye K Rogers' 'MATH 0980-06'

# MATH 0980-07: Transitional Math IIB
# assigned to SNOW 3 at MW1300+100
./edit make-section 'MATH 0980-07' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Kathryn E Ott' 'MATH 0980-07'

# MATH 0980-08: Transitional Math IIB
# assigned to SNOW 3 at TR1300+100
./edit make-section 'MATH 0980-08' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections "Amanda Fa'onelua" 'MATH 0980-08'

# MATH 0980-10: Transitional Math IIB
# assigned to SNOW 3 at MW1630+100
./edit make-section 'MATH 0980-10' 'Math lecture' 'MW1630+100'

# MATH 1010-03: Intermediate Algebra
# assigned to SNOW 3 at MTWR1100+50
./edit make-section 'MATH 1010-03' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Violeta Adina Ionita' 'MATH 1010-03'

# MATH 1010-04: Intermediate Algebra
# assigned to SNOW 145 at MW1300+100
./edit make-section 'MATH 1010-04' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Elizabeth Karen Ludlow' 'MATH 1010-04'

# MATH 1010-05: Intermediate Algebra
# assigned to SNOW 145 at TR1500+100
./edit make-section 'MATH 1010-05' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Odean Bowler' 'MATH 1010-05'

# MATH 1010-06: Intermediate Algebra
# assigned to SNOW 145 at MW1500+100
./edit make-section 'MATH 1010-06' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Odean Bowler' 'MATH 1010-06'

# MATH 1010-07: Intermediate Algebra
# assigned to SNOW 3 at MTWR1200+50
./edit make-section 'MATH 1010-07' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Violeta Adina Ionita' 'MATH 1010-07'

# MATH 1010-50: Intermediate Algebra
# assigned to SNOW 147 at TR1800+100
./edit make-section 'MATH 1010-50' 'Math lecture' 'TR1800+100'

# MATH 1030-01: Quantitative Reasoning (MA)
# assigned to SNOW 125 at MW1500+75
./edit make-section 'MATH 1030-01' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Elizabeth Karen Ludlow' 'MATH 1030-01'

# MATH 1030-02: Quantitative Reasoning (MA)
# assigned to SNOW 124 at TR0730+75
./edit make-section 'MATH 1030-02' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Craig D Seegmiller' 'MATH 1030-02'

# MATH 1030-03: Quantitative Reasoning (MA)
# assigned to SNOW 124 at TR0900+75
./edit make-section 'MATH 1030-03' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Craig D Seegmiller' 'MATH 1030-03'

# MATH 1030-04: Quantitative Reasoning (MA)
# assigned to SNOW 125 at MW1330+75
./edit make-section 'MATH 1030-04' 'Math lecture' '3 credit bell schedule'

# MATH 1030-05: Quantitative Reasoning (MA)
# assigned to SNOW 150 at TR1200+75
./edit make-section 'MATH 1030-05' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Jeffrey P Harrah' 'MATH 1030-05'

# MATH 1030-06: Quantitative Reasoning (MA)
# assigned to SNOW 150 at TR1330+75
./edit make-section 'MATH 1030-06' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Jeffrey P Harrah' 'MATH 1030-06'

# MATH 1040-01: Introduction to Statistics (MA)
# assigned to SNOW 124 at MWF0800+50
./edit make-section 'MATH 1040-01' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'James P Fitzgerald' 'MATH 1040-01'

# MATH 1040-02: Introduction to Statistics (MA)
# assigned to SNOW 124 at MWF0900+50
./edit make-section 'MATH 1040-02' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'James P Fitzgerald' 'MATH 1040-02'

# MATH 1040-03: Introduction to Statistics (MA)
# assigned to SNOW 124 at MWF1000+50
./edit make-section 'MATH 1040-03' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'James P Fitzgerald' 'MATH 1040-03'

# MATH 1040-04: Introduction to Statistics (MA)
# assigned to SNOW 124 at MWF1200+50
./edit make-section 'MATH 1040-04' 'Math lecture' 'MWF1200+50'

# MATH 1040-05: Introduction to Statistics (MA)
# assigned to SNOW 124 at MWF1100+50
./edit make-section 'MATH 1040-05' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Tye K Rogers' 'MATH 1040-05'

# MATH 1040-06: Introduction to Statistics (MA)
# assigned to SNOW 125 at TR1330+75
./edit make-section 'MATH 1040-06' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Tye K Rogers' 'MATH 1040-06'

# MATH 1040-07: Introduction to Statistics (MA)
# assigned to SNOW 151 at TR1200+75
./edit make-section 'MATH 1040-07' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Jameson C Hardy' 'MATH 1040-07'

# MATH 1040-08: Introduction to Statistics (MA)
# assigned to SNOW 124 at MW1500+75
./edit make-section 'MATH 1040-08' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Paula Manuele Temple' 'MATH 1040-08'

# MATH 1040-09: Introduction to Statistics (MA)
# assigned to SNOW 150 at MW1200+75
./edit make-section 'MATH 1040-09' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Jameson C Hardy' 'MATH 1040-09'

# MATH 1040-10: Introduction to Statistics (MA)
# assigned to SNOW 124 at TR1200+75
./edit make-section 'MATH 1040-10' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Jie Liu' 'MATH 1040-10'

# MATH 1040-11: Introduction to Statistics (MA)
# assigned to SNOW 124 at TR1630+75
./edit make-section 'MATH 1040-11' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Ryan C McConnell' 'MATH 1040-11'

# MATH 1040-12: Introduction to Statistics (MA)
# assigned to SNOW 125 at TR1630+75
./edit make-section 'MATH 1040-12' 'Math lecture' '3 credit bell schedule'

# MATH 1040-14: Introduction to Statistics (MA)
# assigned to SNOW 124 at MW1630+75
./edit make-section 'MATH 1040-14' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Robert T Reimer' 'MATH 1040-14'

# MATH 1050-01: College Algebra / Pre-Calculus (MA)
# assigned to SNOW 3 at MTWR0800+50
./edit make-section 'MATH 1050-01' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Violeta Adina Ionita' 'MATH 1050-01'

# MATH 1050-02: College Algebra / Pre-Calculus (MA)
# assigned to SNOW 3 at MTWR0900+50
./edit make-section 'MATH 1050-02' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Violeta Adina Ionita' 'MATH 1050-02'

# MATH 1050-03: College Algebra / Pre-Calculus: Supplemental Instruction (MA)
# assigned to SNOW 125 at F1100+50
./edit make-section 'MATH 1050-03' 'Math lecture' 'F1100+50'

# MATH 1050-03-alt: College Algebra / Pre-Calculus: Supplemental Instruction (MA)
# assigned to SNOW 125 at MTWR1100+50
./edit make-section 'MATH 1050-03-alt' 'Math lecture' 'MTWR1100+50'
./edit assign-faculty-sections 'Costel Ionita' 'MATH 1050-03-alt'

# MATH 1050-04: College Algebra / Pre-Calculus (MA)
# assigned to SNOW 147 at MTWR1200+50
./edit make-section 'MATH 1050-04' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Clare C Banks' 'MATH 1050-04'

# MATH 1050-05: College Algebra / Pre-Calculus (MA)
# assigned to SNOW 145 at TR1300+100
./edit make-section 'MATH 1050-05' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Dawn Lashell Kidd-Thomas' 'MATH 1050-05'

# MATH 1050-06: College Algebra / Pre-Calculus (MA)
# assigned to SNOW 112 at MTWR1200+50
./edit make-section 'MATH 1050-06' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Craig D Seegmiller' 'MATH 1050-06'

# MATH 1060-01: Trigonometry (MA)
# assigned to SNOW 147 at TR0900+75
./edit make-section 'MATH 1060-01' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Ross C Decker' 'MATH 1060-01'

# MATH 1060-02: Trigonometry (MA)
# assigned to SNOW 147 at TR1030+75
./edit make-section 'MATH 1060-02' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Ross C Decker' 'MATH 1060-02'

# MATH 1080-01: Pre-Calculus with Trigonometry (MA)
# assigned to SNOW 145 at MTWRF1000+50
./edit make-section 'MATH 1080-01' 'Math lecture' '5 credit bell schedule'
./edit assign-faculty-sections 'Jameson C Hardy' 'MATH 1080-01'

# MATH 1100-02: Business Calculus (MA)
# assigned to SNOW 124 at MW1330+75
./edit make-section 'MATH 1100-02' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Trevor K Johnson' 'MATH 1100-02'

# MATH 1210-01: Calculus I (MA)
# assigned to SNOW 145 at MTWR1200+50
./edit make-section 'MATH 1210-01' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Trevor K Johnson' 'MATH 1210-01'

# MATH 1210-02: Calculus I (MA)
# assigned to SNOW 125 at MTWR0800+50
./edit make-section 'MATH 1210-02' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Costel Ionita' 'MATH 1210-02'

# MATH 1210-03: Calculus I (MA)
# assigned to SNOW 145 at MTWR1100+50
./edit make-section 'MATH 1210-03' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Bhuvaneswari Sambandham' 'MATH 1210-03'

# MATH 1220-01: Calculus II (MA)
# assigned to SNOW 147 at MTWR0800+50
./edit make-section 'MATH 1220-01' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Clare C Banks' 'MATH 1220-01'

# MATH 1220-02: Calculus II (MA)
# assigned to SNOW 125 at MTWR0900+50
./edit make-section 'MATH 1220-02' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Costel Ionita' 'MATH 1220-02'

# MATH 2010-01: Math for Elementary Teachers I
# assigned to SNOW 150 at T1630+150
./edit make-section 'MATH 2010-01' 'Math lecture' 'T1630+150'
./edit assign-faculty-sections 'Jeffrey P Harrah' 'MATH 2010-01'

# MATH 2020-01: Math for Elemen Teachers II
# assigned to SNOW 150 at TR1030+75
./edit make-section 'MATH 2020-01' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Jeffrey P Harrah' 'MATH 2020-01'

# MATH 2020-02: Math for Elemen Teachers II
# assigned to SNOW 150 at W1630+150
./edit make-section 'MATH 2020-02' 'Math lecture' 'W1630+150'
./edit assign-faculty-sections 'Jeffrey P Harrah' 'MATH 2020-02'

# MATH 2200-01: Discrete Mathematics
# assigned to SNOW 112 at TR1030+75
./edit make-section 'MATH 2200-01' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Steven McKay Sullivan' 'MATH 2200-01'

# MATH 2210-01: Multivariable Calculus (MA)
# assigned to SNOW 112 at MTWR0900+50
./edit make-section 'MATH 2210-01' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Steven McKay Sullivan' 'MATH 2210-01'

# MATH 2250-01: Differential Equations and Linear Algebra
# assigned to SNOW 125 at MTWF1000+50
./edit make-section 'MATH 2250-01' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Bhuvaneswari Sambandham' 'MATH 2250-01'

# MATH 2270-01: Linear Algebra
# assigned to SNOW 151 at TR0900+75
./edit make-section 'MATH 2270-01' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Md Sazib Hasan' 'MATH 2270-01'

# MATH 2280-01: Ordinary Differential Equations
# assigned to SNOW 151 at MW1200+75
./edit make-section 'MATH 2280-01' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Bhuvaneswari Sambandham' 'MATH 2280-01'

# MATH 3050-01: Stochastic Modeling and Applications
# assigned to SNOW 151 at TR1030+75
./edit make-section 'MATH 3050-01' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Md Sazib Hasan' 'MATH 3050-01'

# MATH 3200-01: Introduction to Analysis I
# assigned to SNOW 125 at TR1200+75
./edit make-section 'MATH 3200-01' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Costel Ionita' 'MATH 3200-01'

# MATH 3450-01: Statistical Inference
# assigned to SNOW 124 at TR1030+75
./edit make-section 'MATH 3450-01' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Jie Liu' 'MATH 3450-01'

# MATH 3900-01: Number Theory
# assigned to SNOW 112 at MWF1000+50
./edit make-section 'MATH 3900-01' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Steven McKay Sullivan' 'MATH 3900-01'

# MATH 4250-01: Programming for Scientific Computation
# assigned to SNOW 147 at MW1500+100
./edit make-section 'MATH 4250-01' 'Math lecture' '4 credit bell schedule'
./edit assign-faculty-sections 'Vinodh Kumar Chellamuthu' 'MATH 4250-01'

# MATH 4400-01: Financial Mathematics
# assigned to SNOW 124 at TR1330+75
./edit make-section 'MATH 4400-01' 'Math lecture' '3 credit bell schedule'
./edit assign-faculty-sections 'Jie Liu' 'MATH 4400-01'

# MATH 4410-01: Actuarial Exam FM/ 2 Preparation
# assigned to SNOW 124 at T1500+75
./edit make-section 'MATH 4410-01' 'Math lecture' 'T1500+75'
./edit assign-faculty-sections 'Jie Liu' 'MATH 4410-01'

# MATH 4800-01: Industrial Careers in Mathematics
# assigned to SNOW 147 at MW1645+75
./edit make-section 'MATH 4800-01' 'Math lecture' 'MW1645+75'
./edit assign-faculty-sections 'Vinodh Kumar Chellamuthu' 'MATH 4800-01'
