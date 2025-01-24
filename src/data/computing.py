import queries
from queries import *

def build(db: DB) -> None:
    print('building smith building and classrooms')
    db.make_building('Smith')
    db.make_room('Smith 107', 32, ['flex'])
    db.make_room('Smith 108', 32, ['flex'])
    db.make_room('Smith 109', 32, ['flex'])
    db.make_room('Smith 112', 24, ['macs'])
    db.make_room('Smith 113', 24, ['pcs'])
    db.make_room('Smith 116', 38, [])
    db.make_room('Smith 117', 38, [])

    print('building computing time slots')

    db.make_time_slot('MWF0800+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MWF0900+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MWF1000+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MWF1100+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MW1200+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])
    db.make_time_slot('MW1330+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])
    db.make_time_slot('MW1500+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])
    db.make_time_slot('MW1630+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])
    db.make_time_slot('TR0900+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1030+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1200+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1330+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1500+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1630+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('R1800+150', ['1×150 evening'])
    db.make_time_slot('T1800+150', ['1×150 evening'])
    db.make_time_slot('W1800+150', ['1×150 evening'])
    db.make_time_slot('T1630+100', ['2 credit early evening'])
    db.make_time_slot('M1630+150', ['3 credit early evening'])
    db.make_time_slot('T1630+150', ['3 credit early evening'])
    db.make_time_slot('W1630+150', ['3 credit early evening'])
    db.make_time_slot('R1630+150', ['3 credit early evening'])
    db.make_time_slot('R1900+50', [])
    db.make_time_slot('F1300+50', [])

    db.make_course('Computing', 'CS 6300', 'Principles of Artificial Intelligence')
    db.make_course('Computing', 'CS 6310', 'Foundations of Machine Learning')
    db.make_course('Computing', 'CS 6350', 'Artificial Intelligence and Machine Learning Project 1')
    db.make_course('Computing', 'IT 3700', 'Cybersecurity Analytics')
    db.make_course('Computing', 'SA 1400', 'Fundamentals of Programming (Success)')
    db.make_time_slot('TR0930+80', ['Success'])
    db.make_time_slot('TR1200+80', ['Success'])
    db.make_time_slot('T1600+75', [])
    db.make_time_slot('W1200+50', [])
    db.make_section('MATH 2050-01', 'MW1330+75')
    db.make_section('MATH 3400-01', 'TR1030+75')

    print('building computing conflicts')
    db.make_program('Computer Science', 'Computing')
    db.make_conflict('Computer Science', '3rd/4th semester bottleneck classes', 1, 'boost',
        ['CS 2420', 'CS 2450', 'CS 2810', 'CS 3005'])
    db.make_conflict('Computer Science', 'core requirements', 3, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2420', 'CS 2450', 'CS 2810', 'CS 3005', 'CS 3530', 'CS 3510', 'CS 4600',
        'MATH 1210', 'MATH 3400', 'CS 2100'])
    db.make_conflict('Computer Science', 'core electives', 4, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2420', 'CS 2450', 'CS 2810', 'CS 3005', 'CS 3530', 'CS 3510', 'CS 4600',
        'MATH 1210', 'MATH 3400', 'CS 2100',
        'CS 3150', 'CS 3400', 'CS 3410', 'CS 3520', 'CS 3600', 'CS 4300', 'CS 4307', 'CS 4320', 'CS 4550', 'SE 3200'])
    db.make_conflict('Computer Science', 'math electives', 8, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2420', 'CS 2450', 'CS 2810', 'CS 3005', 'CS 3530', 'CS 3510', 'CS 4600',
        'MATH 1210', 'MATH 3400', 'CS 2100',
        'CS 3150', 'CS 3400', 'CS 3410', 'CS 3520', 'CS 3600', 'CS 4300', 'CS 4307', 'CS 4320', 'CS 4550', 'SE 3200',
        'MATH 1220', 'MATH 2210', 'MATH 2250', 'MATH 2270', 'MATH 2280',
        'MATH 3050', 'MATH 3450', 'MATH 3605', 'MATH 3905', 'MATH 4005'])

    db.make_program('Data Science', 'Computing')
    db.make_conflict('Data Science', 'core requirements', 3, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2100', 'CS 2420', 'CS 2450', 'CS 2500', 'CS 2810',
        'CS 3005', 'CS 3410', 'CS 3510', 'CS 4300', 'CS 4307', 'CS 4320', 'CS 4400', 'CS 4410', 'CS 4600',
        'MATH 1210', 'MATH 1220', 'MATH 2270', 'MATH 3400', 'IT 1500'])

    db.make_program('Software Engineering', 'Computing')
    db.make_conflict('Software Engineering', 'core requirements', 3, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2100', 'CS 2420', 'CS 2810', 'CS 3005',
        'CS 3150', 'CS 3510', 'CS 4307', 'IT 2300', 'IT 1100',
        'SE 1400', 'CS 2450', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600', 'MATH 1100', 'MATH 1210', 'MATH 2050'])
    db.make_conflict('Software Engineering', 'Entrepreneurial and marketing track', 7, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2100', 'CS 2420', 'CS 2810',
        'CS 3150', 'CS 3510', 'CS 4307', 'IT 2300', 'IT 1100',
        'SE 1400', 'CS 2450', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600', 'MATH 1100', 'MATH 1210', 'MATH 2050',
        'SE 3500', 'SE 3550'])
    db.make_conflict('Software Engineering', 'DevOps track', 7, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2100', 'CS 2420', 'CS 2810', 'CS 3005',
        'CS 3150', 'CS 3510', 'CS 4307', 'IT 2300', 'IT 1100',
        'SE 1400', 'CS 2450', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600', 'MATH 1100', 'MATH 1210', 'MATH 2050',
        'IT 3110', 'IT 3300', 'IT 4200'])
    db.make_conflict('Software Engineering', 'Application track', 7, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2100', 'CS 2420', 'CS 2810', 'CS 3005',
        'CS 3150', 'CS 3510', 'CS 4307', 'IT 2300', 'IT 1100',
        'SE 1400', 'CS 2450', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600', 'MATH 1100', 'MATH 1210', 'MATH 2050',
        'SE 3250', 'SE 3450'])
    db.make_conflict('Software Engineering', 'Data science track', 7, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2100', 'CS 2420', 'CS 2810', 'CS 3005',
        'CS 3150', 'CS 3510', 'CS 4307', 'IT 2300', 'IT 1100',
        'SE 1400', 'CS 2450', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600', 'MATH 1100', 'MATH 1210', 'MATH 2050',
        'CS 4300', 'CS 4400', 'CS 4320', 'CS 4410'])
    db.make_conflict('Software Engineering', 'Virtual reality track', 7, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2100', 'CS 2420', 'CS 2810', 'CS 3005',
        'CS 3150', 'CS 3510', 'CS 4307', 'IT 2300', 'IT 1100',
        'SE 1400', 'CS 2450', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600', 'MATH 1100', 'MATH 1210', 'MATH 2050',
        'CS 3500', 'CS 4995', 'CS 4996'])
    db.make_conflict('Software Engineering', 'only need one of AI/data mining', 0, 'reduce',
        ['CS 4300', 'CS 4400'])
    db.make_conflict('Software Engineering', 'only need one database class', 0, 'reduce',
        ['CS 4307', 'IT 2300'])
    db.make_conflict('Software Engineering', 'only need one mobile app class', 0, 'reduce',
        ['SE 3010', 'SE 3020'])
    db.make_conflict('Software Engineering', 'only need one calculus class', 0, 'reduce',
        ['MATH 1100', 'MATH 1210'])

    db.make_program('Information Technology', 'Computing')
    db.make_conflict('Information Technology', 'core requirements', 3, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 2500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050'])
    db.make_conflict('Information Technology', 'core electives', 4, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 2500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3110', 'IT 3300', 'IT 3710', 'IT 4100', 'IT 4200', 'IT 4310', 'IT 4400', 'IT 4510', 'IT 4920R'])
    db.make_conflict('Information Technology', 'DevOps requirements', 5, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 2500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3110', 'IT 3300', 'IT 4200', 'CS 2450'])
    db.make_conflict('Information Technology', 'DevOps requirements vs core electives', 7, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 2500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3110', 'IT 3300', 'IT 3710', 'IT 4100', 'IT 4200', 'IT 4310', 'IT 4400', 'IT 4510', 'IT 4920R',
        'CS 2450'])
    db.make_conflict('Information Technology', 'Cybersecurity requirements', 5, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 2500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3710', 'IT 4400', 'IT 4510'])
    # Cybersecurity requirements are a subset of core electives
    db.make_conflict('Information Technology', 'Cybersecurity choose 2', 6, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 2500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3710', 'IT 4400', 'IT 4510',
        'IT 4310', 'IT 4990', 'CS 2420', 'CS 2810'])
    db.make_conflict('Information Technology', 'Cybersecurity choose 2 vs core electives', 8, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 2500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3110', 'IT 3300', 'IT 3710', 'IT 4100', 'IT 4200', 'IT 4310', 'IT 4400', 'IT 4510', 'IT 4920R',
        'IT 4990', 'CS 2420', 'CS 2810'])
    db.make_conflict('Information Technology', 'only need one math class', 0, 'reduce',
        ['MATH 1040', 'MATH 1050'])

    print('building computing faculty and sections')
    default_availability = [Available('MTWR', '0900', '1630'), Available('F', '0900', '1200')]
    default_prefs_twoday = [
        DaysOff(0, 10), EvenlySpread(11), NoRoomSwitch(12), TooManyRooms(19),
        GapTooLong(105, 15), GapTooLong(195, 13),
        ClusterTooShort(110, 13), ClusterTooLong(165, 12),
    ]
    default_prefs_oneday = [
        DaysOff(1, 10), EvenlySpread(11), NoRoomSwitch(12), TooManyRooms(19),
        GapTooLong(105, 15), GapTooLong(195, 13),
        ClusterTooShort(110, 13), ClusterTooLong(165, 12),
    ]

    db.make_faculty('Andrew Wilson', 'Computing', default_availability + [
        Available('M', '1630', '1900'),
    ])
    db.make_section('SD 6110-01', 'M1630+150', 'Smith 117')
    db.assign_faculty_sections('Andrew Wilson', 'SD 6110-01')

    db.make_faculty('Bart Stander', 'Computing', default_availability + [
        Available('TR', '0900', '1030', 10),
        Available('TR', '1030', '1200', 10),
        Available('MW', '1200', '1330', 11),
    ])
    db.faculty_preferences('Bart Stander', 'MT',
        ClusterTooShort(110, 12),
    )
    db.make_section('CS 2100-01', '3 credit bell schedule', 'Smith 116')
    db.make_section('CS 2420-01', '3 credit bell schedule', 'Smith 116')
    db.make_section('CS 2420-02', '3 credit bell schedule', 'Smith 116')
    db.make_section('CS 3500-01', '3 credit bell schedule', 'pcs')
    db.make_section('CS 4995-01', 'TR1200+75', 'TR1330+75', 'TR1500+75', 'pcs')
    db.assign_faculty_sections('Bart Stander', 'CS 2100-01', 'CS 2420-01', 'CS 2420-02', 'CS 3500-01', 'CS 4995-01')

    db.make_faculty('Brayden Connole', 'Computing', default_availability + [
        Available('MWF', '0900', '1000', 10),
        Available('TR', '0900', '1030', 10),
        Available('MWF', '1000', '1100', 11),
        Available('MWF', '1100', '1200', 11),
        Available('TR', '1030', '1200', 12),
    ])
    db.faculty_preferences('Brayden Connole', 'MT',
        DaysOff(1, 13),
    )
    db.make_section('IT 4200-01', '3 credit bell schedule', 'flex')
    db.make_section('SE 1400-01', '3 credit bell schedule', 'flex')
    db.make_section('SE 1400-02', '3 credit bell schedule', 'flex')
    db.make_section('SE 3020-01', '3 credit bell schedule', 'macs')
    db.assign_faculty_sections('Brayden Connole', 'IT 4200-01', 'SE 1400-01', 'SE 1400-02', 'SE 3020-01')

    db.make_faculty('Carol Stander', 'Computing', default_availability + [
        Available('TR',  '0900', '1030', 10),
        Available('TR',  '1030', '1200', 10),
        Available('MWF', '0900', '1000', 11),
        Available('MW',  '1200', '1330', 11),
        Available('TR',  '1200', '1330', 12),
    ])
    db.faculty_preferences('Carol Stander', 'MT',
        ClusterTooLong(110, 12),
    )
    db.make_section('CS 1030-01', '3 credit bell schedule', 'flex')
    db.make_section('CS 1400-40')
    db.make_section('IT 1100-01', '3 credit bell schedule', 'MWF 3×50 bell schedule:13', 'pcs')
    db.make_section('IT 2300-01', '3 credit bell schedule', 'MWF 3×50 bell schedule:13', 'flex')
    db.make_section('IT 2300-40')
    db.assign_faculty_sections('Carol Stander', 'CS 1030-01', 'CS 1400-40', 'IT 1100-01', 'IT 2300-01', 'IT 2300-40')

    # adjunct CS 1030 section
    db.make_section('CS 1030-02', 'MW1200+75', 'TR1200+75', 'flex')

    db.make_faculty('Curtis Larsen', 'Computing', default_availability + [
        Available('R', '1630', '1900'),
    ])
    db.faculty_preferences('Curtis Larsen', 'MT', *default_prefs_oneday)
    db.make_section('CS 3530-01', '3 credit bell schedule', 'Smith 116')
    db.make_section('CS 4300-01', '3 credit bell schedule', 'Smith 116')
    db.make_section('CS 4920R-01')
    db.make_section('CS 6300-01', 'R1630+150', 'Smith 116')
    db.make_section('CS 6350-01')
    db.assign_faculty_sections('Curtis Larsen', 'CS 3530-01', 'CS 4300-01', 'CS 4920R-01', 'CS 6300-01', 'CS 6350-01')

    db.make_faculty('DJ Holt', 'Computing', default_availability + [
        Available('T', '1630', '1900'),
        Available('W', '1630', '1900'),
        Available('R', '1630', '1900'),
    ])
    db.faculty_preferences('DJ Holt', 'MT')
    db.make_section('CS 4410-01', 'TR1500+75', 'Smith 117')
    db.make_section('SD 6100-01', 'T1630+150', 'Smith 117')
    db.make_section('SD 6400-01', 'W1630+150', 'Smith 117')
    db.make_section('SD 6450-01', 'R1630+150', 'Smith 117')
    db.assign_faculty_sections('DJ Holt', 'CS 4410-01', 'SD 6100-01', 'SD 6400-01', 'SD 6450-01')

    db.make_faculty('Eric Pedersen', 'Computing', default_availability + [
        Available('T', '1600', '1715'),
        Available('R', '1630', '1900'),
    ])
    db.faculty_preferences('Eric Pedersen', 'MT')
    db.make_section('SE 3500-01', 'TR1030+75', 'TR1200+75', 'flex')
    db.make_section('SE 4990-01', 'T1600+75')
    db.make_section('SE 4990-02', 'W1200+50', 'flex', 'Smith 116')
    # SD 6450-01 co-taught with DJ
    db.assign_faculty_sections('Eric Pedersen', 'SE 3500-01', 'SE 4990-01', 'SE 4990-02', 'SD 6450-01')

    db.make_faculty('Jay Sneddon', 'Computing', default_availability + [
        Available('MWF', '0900', '1000', 10),
        Available('MWF', '1000', '1100', 10),
        Available('MWF', '1100', '1200', 11),
        Available('T', '1630', '1810'),
    ])
    db.faculty_preferences('Jay Sneddon', 'MT',
        DaysOff(0, 11),
        EvenlySpread(12),
        ClusterTooShort(110, 13),
        GapTooLong(105, 13),
    )
    db.make_section('IT 1200-01', '3 credit bell schedule', 'Smith 107')
    db.make_section('IT 2700-01', '3 credit bell schedule', 'Smith 107')
    db.make_section('IT 3700-40')
    db.make_section('IT 4310-01', '3 credit bell schedule', 'Smith 107')
    db.make_section('IT 4990-01', 'T1630+100', 'Smith 107')
    db.assign_faculty_sections('Jay Sneddon', 'IT 1200-01', 'IT 2700-01', 'IT 3700-40', 'IT 4310-01', 'IT 4990-01')

    db.make_faculty('Jeff Compas', 'Computing', default_availability + [
        Available('MWF', '0900', '1000', 11),
        Available('MWF', '1000', '1100', 12),
        Available('MWF', '1100', '1200', 12),
        Available('TR',  '0900', '1030', 11),
        Available('TR',  '1030', '1200', 14),
        Available('MW',  '1500', '1630', 10),
        Available('TR',  '1500', '1630', 10),

    ])
    db.faculty_preferences('Jeff Compas', 'MT',
        ClusterTooShort(165, 15),
        ClusterTooLong(165, 13),
        GapTooLong(105, 13),
        EvenlySpread(14),
    )
    db.make_section('CS 1400-01', '3 credit bell schedule', 'flex')
    db.make_section('CS 2450-01', '3 credit bell schedule', 'flex')
    db.make_section('CS 2450-02', '3 credit bell schedule', 'flex')
    db.make_section('CS 3005-01', '3 credit bell schedule', 'flex')
    db.make_section('SE 3150-01', '3 credit bell schedule', 'flex')
    db.assign_faculty_sections('Jeff Compas', 'CS 1400-01', 'CS 2450-01', 'CS 2450-02', 'CS 3005-01', 'SE 3150-01')

    db.make_faculty('Joe Francom', 'Computing', default_availability + [
        Available('MW', '1200', '1330', -1),
        Available('MW', '1500', '1630', -1),
        Available('TR', '1500', '1630', -1),
    ])
    db.faculty_preferences('Joe Francom', 'MT',
        ClusterTooShort(110, 15))
    db.make_section('IT 1500-40')
    db.make_section('IT 3300-01', '3 credit bell schedule', 'flex')
    db.make_section('SE 3200-01', '3 credit bell schedule', 'flex')
    db.assign_faculty_sections('Joe Francom', 'IT 1500-40', 'IT 3300-01', 'SE 3200-01')

    db.make_faculty('Lora Klein', 'Computing', default_availability + [
        Available('MWF', '0900', '1000', 10),
        Available('MWF', '1000', '1100', 10),
        Available('MWF', '1100', '1200', 11),
        Available('TR',  '1330', '1500', 12),
        Available('TR',  '1500', '1630', 12),
    ])
    db.faculty_preferences('Lora Klein', 'MT',
        EvenlySpread(11),
    )
    db.make_section('CS 1400-02', '3 credit bell schedule', 'flex')
    db.make_section('SA 1400-01', 'TR0930+80')
    db.make_section('SA 1400-02', 'TR1200+80')
    db.assign_faculty_sections('Lora Klein', 'CS 1400-02', 'SA 1400-01', 'SA 1400-02')

    db.make_faculty('Matt Kearl', 'Computing', default_availability + [
        Available('MWF', '0900', '1000', 16),
        Available('MWF', '1000', '1100', 15),
        Available('MWF', '1100', '1200', 15),
        Available('MW',  '1200', '1330', 14),
        Available('MW',  '1330', '1500', 13),
        Available('MW',  '1500', '1630', 12),
        Available('TR',  '1330', '1500', 13),
        Available('TR',  '1500', '1630', 12),
    ])
    db.faculty_preferences('Matt Kearl', 'MT',
        ClusterTooShort(165, 14),
        GapTooLong(105, 16),
        DaysOff(1, 11))
    db.make_section('SE 1400-03', '3 credit bell schedule', 'MWF 3×50 bell schedule:10', 'macs')
    db.make_section('SE 1400-40')
    db.make_section('SE 3400-40')
    db.make_section('SE 3550-01', '3 credit bell schedule', 'MWF 3×50 bell schedule:10', 'macs', 'pcs', 'flex')
    db.make_section('SE 4920-01')
    db.assign_faculty_sections('Matt Kearl', 'SE 1400-03', 'SE 1400-40', 'SE 3400-40', 'SE 3550-01', 'SE 4920-01')

    db.make_faculty('Nicole Dang', 'Computing', default_availability)
    db.make_section('SET 1000-40')
    db.assign_faculty_sections('Nicole Dang', 'SET 1000-40')

    db.make_faculty('Phil Daley', 'Computing', default_availability + [
        Available('MWF', '0900', '1000', 10),
        Available('MWF', '1000', '1100', 10),
        Available('MWF', '1100', '1200', 11),
    ])
    db.faculty_preferences('Phil Daley', 'MT')
    db.make_section('IT 1100-02', '3 credit bell schedule', 'pcs')
    db.make_section('IT 1100-40')
    db.make_section('IT 2400-01', '3 credit bell schedule', 'Smith 107')
    db.make_section('IT 3100-01', '3 credit bell schedule', 'Smith 107')
    db.make_section('IT 4400-01', '3 credit bell schedule', 'Smith 107')
    db.assign_faculty_sections('Phil Daley', 'IT 1100-02', 'IT 1100-40', 'IT 2400-01', 'IT 3100-01', 'IT 4400-01')

    db.make_faculty('Ren Quinn', 'Computing', default_availability + [
        Available('R',  '1900', '2000'),
        Available('F',  '1300', '1400'),
        Available('TR', '0900', '1030', 10),
        Available('TR', '1030', '1200', 10),
        Available('TR', '1200', '1330', 11),
        Available('MW', '1200', '1330', 11),
    ])
    db.faculty_preferences('Ren Quinn', 'MT',
        DaysOff(0, 12),
        NoRoomSwitch(12),
    )
    db.make_section('CS 1400-03', '3 credit bell schedule', 'flex')
    db.make_section('CS 1410-01', '3 credit bell schedule', 'flex')
    db.make_section('CS 2500-01', '3 credit bell schedule', 'flex')
    db.make_section('CS 3150-01', '3 credit bell schedule', 'flex', 'Smith 116:10')
    db.make_section('CS 4800R-01')
    db.make_section('CS 4991R-50', 'R1900+50', 'Smith 116')
    db.make_section('CS 4992R-01', 'F1300+50', 'Smith 109')
    db.assign_faculty_sections('Ren Quinn', 'CS 1400-03', 'CS 1410-01', 'CS 2500-01', 'CS 3150-01', 'CS 4800R-01', 'CS 4991R-50', 'CS 4992R-01')

    db.make_faculty('Russ Ross', 'Computing', default_availability + [
        Available('MWF', '0900', '1000', 10),
        Available('MWF', '1000', '1100', 10),
        Available('MWF', '1100', '1200', 10),
        Available('MW',  '1500', '1630', 12),
        Available('TR',  '0900', '1030', 10),
        Available('TR',  '1030', '1200', 10),
        Available('TR',  '1500', '1630', 12),
    ])
    db.faculty_preferences('Russ Ross', 'MT',
        DaysOff(0, 11),
        EvenlySpread(11),
        ClusterTooLong(165, 13),
        ClusterTooShort(110, 13),
    )
    db.make_section('CS 2810-01', 'Smith 109', '3 credit bell schedule')
    db.make_section('CS 2810-02', 'Smith 109', '3 credit bell schedule')
    db.make_section('CS 3400-01', 'Smith 109', '3 credit bell schedule')
    db.make_section('CS 3520-01', 'Smith 109', '3 credit bell schedule')
    db.make_section('CS 4800R-02')
    db.assign_faculty_sections('Russ Ross', 'CS 2810-01', 'CS 2810-02', 'CS 3400-01', 'CS 3520-01', 'CS 4800R-02')

    db.make_faculty('Syed Ali', 'Computing', default_availability)
    db.faculty_preferences('Syed Ali', 'MT', *default_prefs_twoday)
    db.make_section('IT 1100-03', '3 credit bell schedule', 'pcs')
    db.make_section('IT 2500-01', '3 credit bell schedule', 'flex')
    db.make_section('IT 4510-01', '3 credit bell schedule', 'flex')
    db.make_section('IT 4990-02', '3 credit bell schedule', 'flex')
    db.assign_faculty_sections('Syed Ali', 'IT 1100-03', 'IT 2500-01', 'IT 4510-01', 'IT 4990-02')

    db.make_faculty('Yuanfei Sun', 'Computing', default_availability + [
        Available('TR', '0900', '1030', 10),
        Available('TR', '1030', '1200', 10),
        Available('TR', '1200', '1330', 11),
        Available('TR', '1330', '1500', 11),
        Available('TR', '1500', '1630', 12),
    ])
    db.faculty_preferences('Yuanfei Sun', 'MT',
        ClusterTooShort(110, 15),
        ClusterTooLong(165, 15),
        GapTooLong(105, 14),
        DaysOff(1, 13),
    )
    db.make_section('CS 1410-02', '3 credit bell schedule', 'flex')
    db.make_section('CS 6310-40')
    db.make_section('CS 6350-40')
    db.assign_faculty_sections('Yuanfei Sun', 'CS 1410-02', 'CS 6310-40', 'CS 6350-40')

    db.add_anti_conflict(5, 'CS 1030-01', ['CS 1400'])

    db.make_conflict('Computer Science', 'spread out CS 1400', 5, 'boost',
        ['CS 1400-01', 'CS 1400-02', 'CS 1400-03'])
    db.make_conflict('Computer Science', 'spread out CS 1410', 5, 'boost',
        ['CS 1410-01', 'CS 1410-02'])
    db.make_conflict('Information Technology', 'spread out IT 1100', 5, 'boost',
        ['IT 1100-01', 'IT 1100-02', 'IT 1100-03'])
    db.make_conflict('Software Engineering', 'spread out SE 1400', 5, 'boost',
        ['SE 1400-01', 'SE 1400-02', 'SE 1400-03'])
