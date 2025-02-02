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

    db.make_course('Computing', 'CS 6300', 'Principles of Artificial Intelligence')
    db.make_course('Computing', 'CS 6310', 'Foundations of Machine Learning')
    db.make_course('Computing', 'CS 6350', 'Artificial Intelligence and Machine Learning Project 1')
    db.make_course('Computing', 'IT 3700', 'Cybersecurity Analytics')
    db.make_course('Computing', 'SA 1400', 'Fundamentals of Programming (Success)')
    db.make_section_with_no_faculty('MATH 2050-01', 'MW1330+75')
    db.make_section_with_no_faculty('MATH 3400-01', 'TR1030+75')

    print('building computing conflicts')
    db.make_program('Computer Science', 'Computing')
    db.make_conflict('Computer Science', '3rd/4th semester bottleneck classes', 0, 'boost',
        ['CS 2420', 'CS 2450', 'CS 2810', 'CS 3005'])
    db.make_conflict('Computer Science', 'core requirements', 1, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2420', 'CS 2450', 'CS 2810', 'CS 3005', 'CS 3530', 'CS 3510', 'CS 4600',
        'MATH 1210', 'MATH 3400', 'CS 2100'])
    db.make_conflict('Computer Science', 'core electives', 2, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2420', 'CS 2450', 'CS 2810', 'CS 3005', 'CS 3530', 'CS 3510', 'CS 4600',
        'MATH 1210', 'MATH 3400', 'CS 2100',
        'CS 3150', 'CS 3400', 'CS 3410', 'CS 3520', 'CS 3600', 'CS 4300', 'CS 4307', 'CS 4320', 'CS 4550', 'SE 3200'])
    db.make_conflict('Computer Science', 'math electives', 6, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2420', 'CS 2450', 'CS 2810', 'CS 3005', 'CS 3530', 'CS 3510', 'CS 4600',
        'MATH 1210', 'MATH 3400', 'CS 2100',
        'CS 3150', 'CS 3400', 'CS 3410', 'CS 3520', 'CS 3600', 'CS 4300', 'CS 4307', 'CS 4320', 'CS 4550', 'SE 3200',
        'MATH 1220', 'MATH 2210', 'MATH 2250', 'MATH 2270', 'MATH 2280',
        'MATH 3050', 'MATH 3450', 'MATH 3605', 'MATH 3905', 'MATH 4005'])

    db.make_program('Data Science', 'Computing')
    db.make_conflict('Data Science', 'core requirements', 1, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2100', 'CS 2420', 'CS 2450', 'CS 2500', 'CS 2810',
        'CS 3005', 'CS 3410', 'CS 3510', 'CS 4300', 'CS 4307', 'CS 4320', 'CS 4400', 'CS 4410', 'CS 4600',
        'MATH 1210', 'MATH 1220', 'MATH 2270', 'MATH 3400', 'IT 1500'])

    db.make_program('Software Engineering', 'Computing')
    db.make_conflict('Software Engineering', 'core requirements', 1, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2100', 'CS 2420', 'CS 2810', 'CS 3005',
        'CS 3150', 'CS 3510', 'CS 4307', 'IT 2300', 'IT 1100',
        'SE 1400', 'CS 2450', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600', 'MATH 1100', 'MATH 1210', 'MATH 2050'])
    db.make_conflict('Software Engineering', 'Entrepreneurial and marketing track', 5, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2100', 'CS 2420', 'CS 2810',
        'CS 3150', 'CS 3510', 'CS 4307', 'IT 2300', 'IT 1100',
        'SE 1400', 'CS 2450', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600', 'MATH 1100', 'MATH 1210', 'MATH 2050',
        'SE 3500', 'SE 3550'])
    db.make_conflict('Software Engineering', 'DevOps track', 5, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2100', 'CS 2420', 'CS 2810', 'CS 3005',
        'CS 3150', 'CS 3510', 'CS 4307', 'IT 2300', 'IT 1100',
        'SE 1400', 'CS 2450', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600', 'MATH 1100', 'MATH 1210', 'MATH 2050',
        'IT 3110', 'IT 3300', 'IT 4200'])
    db.make_conflict('Software Engineering', 'Application track', 5, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2100', 'CS 2420', 'CS 2810', 'CS 3005',
        'CS 3150', 'CS 3510', 'CS 4307', 'IT 2300', 'IT 1100',
        'SE 1400', 'CS 2450', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600', 'MATH 1100', 'MATH 1210', 'MATH 2050',
        'SE 3250', 'SE 3450'])
    db.make_conflict('Software Engineering', 'Data science track', 5, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2100', 'CS 2420', 'CS 2810', 'CS 3005',
        'CS 3150', 'CS 3510', 'CS 4307', 'IT 2300', 'IT 1100',
        'SE 1400', 'CS 2450', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600', 'MATH 1100', 'MATH 1210', 'MATH 2050',
        'CS 4300', 'CS 4400', 'CS 4320', 'CS 4410'])
    db.make_conflict('Software Engineering', 'Virtual reality track', 5, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2100', 'CS 2420', 'CS 2810', 'CS 3005',
        'CS 3150', 'CS 3510', 'CS 4307', 'IT 2300', 'IT 1100',
        'SE 1400', 'CS 2450', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600', 'MATH 1100', 'MATH 1210', 'MATH 2050',
        'CS 3500', 'CS 4995', 'CS 4996'])
    db.make_conflict('Software Engineering', 'only need one of AI/data mining', None, 'reduce',
        ['CS 4300', 'CS 4400'])
    db.make_conflict('Software Engineering', 'only need one database class', None, 'reduce',
        ['CS 4307', 'IT 2300'])
    db.make_conflict('Software Engineering', 'only need one mobile app class', None, 'reduce',
        ['SE 3010', 'SE 3020'])
    db.make_conflict('Software Engineering', 'only need one calculus class', None, 'reduce',
        ['MATH 1100', 'MATH 1210'])

    db.make_program('Information Technology', 'Computing')
    db.make_conflict('Information Technology', 'core requirements', 1, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 2500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050'])
    db.make_conflict('Information Technology', 'core electives', 2, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 2500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3110', 'IT 3300', 'IT 3710', 'IT 4100', 'IT 4200', 'IT 4310', 'IT 4400', 'IT 4510', 'IT 4920R'])
    db.make_conflict('Information Technology', 'DevOps requirements', 3, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 2500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3110', 'IT 3300', 'IT 4200', 'CS 2450'])
    db.make_conflict('Information Technology', 'DevOps requirements vs core electives', 5, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 2500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3110', 'IT 3300', 'IT 3710', 'IT 4100', 'IT 4200', 'IT 4310', 'IT 4400', 'IT 4510', 'IT 4920R',
        'CS 2450'])
    db.make_conflict('Information Technology', 'Cybersecurity requirements', 3, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 2500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3710', 'IT 4400', 'IT 4510'])
    # Cybersecurity requirements are a subset of core electives
    db.make_conflict('Information Technology', 'Cybersecurity choose 2', 4, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 2500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3710', 'IT 4400', 'IT 4510',
        'IT 4310', 'IT 4990', 'CS 2420', 'CS 2810'])
    db.make_conflict('Information Technology', 'Cybersecurity choose 2 vs core electives', 6, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 2500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3110', 'IT 3300', 'IT 3710', 'IT 4100', 'IT 4200', 'IT 4310', 'IT 4400', 'IT 4510', 'IT 4920R',
        'IT 4990', 'CS 2420', 'CS 2810'])
    db.make_conflict('Information Technology', 'only need one math class', None, 'reduce',
        ['MATH 1040', 'MATH 1050'])

    print('building computing faculty and sections')
    default_availability = [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')]
    default_prefs_twoday = [
        DoNotWantADayOff(),
        WantClassesEvenlySpreadAcrossDays(),
        AvoidClassClusterLongerThan('2h45m'),
        AvoidGapBetweenClassClustersLongerThan('3h15m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
        WantBackToBackClassesInTheSameRoom(),
        WantClassesPackedIntoAsFewRoomsAsPossible(),
    ]
    default_prefs_oneday = [
        WantADayOff(),
        WantClassesEvenlySpreadAcrossDays(),
        AvoidClassClusterLongerThan('2h45m'),
        AvoidGapBetweenClassClustersLongerThan('3h15m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
        WantBackToBackClassesInTheSameRoom(),
        WantClassesPackedIntoAsFewRoomsAsPossible(),
    ]

    db.make_faculty('Andrew Wilson', 'Computing', default_availability)
    db.make_faculty_section('Andrew Wilson', 'SD 6110-01', 'M1630+150', 'Smith 117')


    db.make_faculty('Bart Stander', 'Computing', default_availability)
    db.make_faculty_section('Bart Stander', 'CS 2100-01', '3 credit bell schedule', 'Smith 116')
    db.make_faculty_section('Bart Stander', 'CS 2420-01', '3 credit bell schedule', 'Smith 116')
    db.make_faculty_section('Bart Stander', 'CS 2420-02', '3 credit bell schedule', 'Smith 116')
    db.make_faculty_section('Bart Stander', 'CS 3500-01', '3 credit bell schedule', 'pcs')
    db.make_faculty_section('Bart Stander', 'CS 4995-01', 'TR1200+75', 'TR1330+75', 'TR1500+75', 'pcs')
    db.faculty_preferences('Bart Stander', 'MT',
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        AvoidTimeSlot('MW1200+75'),
        AvoidClassClusterShorterThan('1h50m'),
    )

    db.make_faculty('Brayden Connole', 'Computing', default_availability)
    db.make_faculty_section('Brayden Connole', 'IT 4200-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Brayden Connole', 'SE 1400-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Brayden Connole', 'SE 1400-02', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Brayden Connole', 'SE 3020-01', '3 credit bell schedule', 'macs')
    db.faculty_preferences('Brayden Connole', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('TR1030+75'),
        WantADayOff(),
    )

    db.make_faculty('Carol Stander', 'Computing', default_availability)
    db.make_faculty_section('Carol Stander', 'CS 1030-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Carol Stander', 'CS 1400-40')
    db.make_faculty_section('Carol Stander', 'IT 1100-01', '3 credit bell schedule', 'pcs', 'flex', 'Smith 116')
    db.make_faculty_section('Carol Stander', 'IT 2300-01', '3 credit bell schedule', 'flex', 'pcs', 'Smith 116')
    db.make_faculty_section('Carol Stander', 'IT 2300-40')
    db.faculty_preferences('Carol Stander', 'MT',
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        AvoidTimeSlot('MWF0900+50'),
        AvoidSectionInRooms('IT 1100-01', ['flex', 'Smith 116']),
        AvoidSectionInRooms('IT 2300-01', ['pcs', 'Smith 116']),
        AvoidTimeSlot('MW1200+75'),
        AvoidTimeSlot('TR1200+75'),
        AvoidGapBetweenClassClustersLongerThan('1h50m'),
        AvoidSectionInTimeSlots('IT 1100-01', ['MWF 3×50 bell schedule']),
        AvoidSectionInTimeSlots('IT 2300-01', ['MWF 3×50 bell schedule']),
    )

    # adjunct CS 1030 section
    db.make_section_with_no_faculty('CS 1030-02', 'MW1200+75', 'TR1200+75', 'flex')

    db.make_faculty('Curtis Larsen', 'Computing', default_availability)
    db.make_faculty_section('Curtis Larsen', 'CS 3530-01', '3 credit bell schedule', 'Smith 116')
    db.make_faculty_section('Curtis Larsen', 'CS 4300-01', '3 credit bell schedule', 'Smith 116')
    db.make_faculty_section('Curtis Larsen', 'CS 4920R-01')
    db.make_faculty_section('Curtis Larsen', 'CS 6300-01', 'R1630+150', 'Smith 116')
    db.make_faculty_section('Curtis Larsen', 'CS 6350-01')
    db.faculty_preferences('Curtis Larsen', 'MT',
        *default_prefs_oneday,
    )

    db.make_faculty('DJ Holt', 'Computing', default_availability)
    db.make_faculty_section('DJ Holt', 'CS 4410-01', 'TR1500+75', 'Smith 117')
    db.make_faculty_section('DJ Holt', 'SD 6100-01', 'T1630+150', 'Smith 117')
    db.make_faculty_section('DJ Holt', 'SD 6400-01', 'W1630+150', 'Smith 117')
    db.make_faculty_section('DJ Holt', 'SD 6450-01', 'R1630+150', 'Smith 117')


    db.make_faculty('Eric Pedersen', 'Computing', default_availability)
    db.make_faculty_section('Eric Pedersen', 'SE 3500-01', 'TR1030+75', 'TR1200+75', 'flex')
    db.make_faculty_section('Eric Pedersen', 'SE 4990-01', 'T1600+75')
    db.make_faculty_section('Eric Pedersen', 'SE 4990-02', 'W1200+50', 'flex', 'Smith 116')
    db.assign_faculty_to_existing_section('Eric Pedersen', 'SD 6450-01') # co-taught with DJ


    db.make_faculty('Jay Sneddon', 'Computing', default_availability)
    db.make_faculty_section('Jay Sneddon', 'IT 1200-01', '3 credit bell schedule', 'Smith 107')
    db.make_faculty_section('Jay Sneddon', 'IT 2700-01', '3 credit bell schedule', 'Smith 107')
    db.make_faculty_section('Jay Sneddon', 'IT 3700-40')
    db.make_faculty_section('Jay Sneddon', 'IT 4310-01', '3 credit bell schedule', 'Smith 107')
    db.make_faculty_section('Jay Sneddon', 'IT 4990-01', 'T1630+100', 'Smith 107')
    db.faculty_preferences('Jay Sneddon', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        DoNotWantADayOff(),
        WantClassesEvenlySpreadAcrossDays(),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
    )

    db.make_faculty('Jeff Compas', 'Computing', default_availability)
    db.make_faculty_section('Jeff Compas', 'CS 1400-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Jeff Compas', 'CS 2450-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Jeff Compas', 'CS 2450-02', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Jeff Compas', 'CS 3005-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Jeff Compas', 'SE 3150-01', '3 credit bell schedule', 'flex')
    db.faculty_preferences('Jeff Compas', 'MT',
        UseSameTimePattern(['CS 2450-01', 'CS 2450-02']),
        AvoidTimeSlot('MW1500+75'),
        AvoidTimeSlot('TR1500+75'),
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidClassClusterLongerThan('2h45m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
        AvoidTimeSlot('TR1030+75'),
        WantClassesEvenlySpreadAcrossDays(),
        AvoidClassClusterShorterThan('2h45m'),
    )

    db.make_faculty('Joe Francom', 'Computing', default_availability)
    db.make_faculty_section('Joe Francom', 'IT 1500-40')
    db.make_faculty_section('Joe Francom', 'IT 3300-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Joe Francom', 'SE 3200-01', '3 credit bell schedule', 'flex')
    db.faculty_preferences('Joe Francom', 'MT',
        UnavailableTimeSlot('MW1200+75'),
        UnavailableTimeSlot('MW1500+75'),
        UnavailableTimeSlot('TR1500+75'),
        AvoidClassClusterShorterThan('1h50m'),
    )

    db.make_faculty('Lora Klein', 'Computing', default_availability)
    db.make_faculty_section('Lora Klein', 'CS 1400-02', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Lora Klein', 'SA 1400-01', 'TR0930+80')
    db.make_faculty_section('Lora Klein', 'SA 1400-02', 'TR1200+80')
    db.faculty_preferences('Lora Klein', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        WantClassesEvenlySpreadAcrossDays(),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('TR1330+75'),
        AvoidTimeSlot('TR1500+75'),
    )

    db.make_faculty('Matt Kearl', 'Computing', default_availability)
    db.make_faculty_section('Matt Kearl', 'SE 1400-03', '3 credit bell schedule', 'macs')
    db.make_faculty_section('Matt Kearl', 'SE 1400-40')
    db.make_faculty_section('Matt Kearl', 'SE 3400-40')
    db.make_faculty_section('Matt Kearl', 'SE 3550-01', '3 credit bell schedule', 'macs', 'pcs', 'flex')
    db.make_faculty_section('Matt Kearl', 'SE 4920-01')
    db.faculty_preferences('Matt Kearl', 'MT',
        AvoidSectionInTimeSlots('SE 1400-03', ['MWF 3×50 bell schedule']),
        AvoidSectionInTimeSlots('SE 3550-01', ['MWF 3×50 bell schedule']),
        WantADayOff(),
        AvoidTimeSlot('MW1500+75'),
        AvoidTimeSlot('TR1500+75'),
        AvoidTimeSlot('MW1330+75'),
        AvoidTimeSlot('TR1330+75'),
        AvoidClassClusterShorterThan('2h45m'),
        AvoidTimeSlot('MW1200+75'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
        AvoidTimeSlot('MWF0900+50'),
    )

    db.make_faculty('Nicole Dang', 'Computing', [])
    db.make_faculty_section('Nicole Dang', 'SET 1000-40')

    db.make_faculty('Phil Daley', 'Computing', default_availability)
    db.make_faculty_section('Phil Daley', 'IT 1100-02', '3 credit bell schedule', 'pcs')
    db.make_faculty_section('Phil Daley', 'IT 1100-40')
    db.make_faculty_section('Phil Daley', 'IT 2400-01', '3 credit bell schedule', 'Smith 107')
    db.make_faculty_section('Phil Daley', 'IT 3100-01', '3 credit bell schedule', 'Smith 107')
    db.make_faculty_section('Phil Daley', 'IT 4400-01', '3 credit bell schedule', 'Smith 107')
    db.faculty_preferences('Phil Daley', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
    )

    db.make_faculty('Ren Quinn', 'Computing', default_availability)
    db.make_faculty_section('Ren Quinn', 'CS 1400-03', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Ren Quinn', 'CS 1410-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Ren Quinn', 'CS 2500-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Ren Quinn', 'CS 3150-01', '3 credit bell schedule', 'flex', 'Smith 116')
    db.make_faculty_section('Ren Quinn', 'CS 4800R-01')
    db.make_faculty_section('Ren Quinn', 'CS 4991R-50', 'R1900+50', 'Smith 116')
    db.make_faculty_section('Ren Quinn', 'CS 4992R-01', 'F1300+50', 'Smith 109')
    db.faculty_preferences('Ren Quinn', 'MT',
        AvoidSectionInRooms('CS 3150-01', ['Smith 116']),
        DoNotWantADayOff(),
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        AvoidTimeSlot('TR1200+75'),
        AvoidTimeSlot('MW1200+75'),
        WantBackToBackClassesInTheSameRoom(),
    )

    db.make_faculty('Russ Ross', 'Computing', default_availability)
    db.make_faculty_section('Russ Ross', 'CS 2810-01', 'Smith 109', '3 credit bell schedule')
    db.make_faculty_section('Russ Ross', 'CS 2810-02', 'Smith 109', '3 credit bell schedule')
    db.make_faculty_section('Russ Ross', 'CS 3400-01', 'Smith 109', '3 credit bell schedule')
    db.make_faculty_section('Russ Ross', 'CS 3520-01', 'Smith 109', '3 credit bell schedule')
    db.make_faculty_section('Russ Ross', 'CS 4800R-02')
    db.faculty_preferences('Russ Ross', 'MT',
        UnavailableTimeSlot('MWF0900+50'),
        UnavailableTimeSlot('MWF1000+50'),
        UnavailableTimeSlot('MWF1100+50'),
        UnavailableTimeSlot('TR0900+75'),
        UnavailableTimeSlot('TR1030+75'),
        WantClassesEvenlySpreadAcrossDays(10),
        AvoidClassClusterShorterThan('2h45m', 10),
    )

    db.make_faculty('Syed Ali', 'Computing', default_availability)
    db.make_faculty_section('Syed Ali', 'IT 1100-03', '3 credit bell schedule', 'pcs')
    db.make_faculty_section('Syed Ali', 'IT 2500-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Syed Ali', 'IT 4510-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Syed Ali', 'IT 4990-02', '3 credit bell schedule', 'flex')
    db.faculty_preferences('Syed Ali', 'MT',
        *default_prefs_twoday,
    )

    db.make_faculty('Yuanfei Sun', 'Computing', default_availability)
    db.make_faculty_section('Yuanfei Sun', 'CS 1410-02', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Yuanfei Sun', 'CS 6310-40')
    db.make_faculty_section('Yuanfei Sun', 'CS 6350-40')
    db.faculty_preferences('Yuanfei Sun', 'MT',
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        AvoidTimeSlot('TR1200+75'),
        AvoidTimeSlot('TR1330+75'),
        AvoidTimeSlot('TR1500+75'),
        WantADayOff(),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidClassClusterLongerThan('2h45m'),
    )

    db.add_anti_conflict(5, 'CS 1030-01', ['CS 1400'])

    db.make_conflict('Computer Science', 'spread out CS 1400', 5, 'boost',
        ['CS 1400-01', 'CS 1400-02', 'CS 1400-03'])
    db.make_conflict('Computer Science', 'spread out CS 1410', 5, 'boost',
        ['CS 1410-01', 'CS 1410-02'])
    db.make_conflict('Information Technology', 'spread out IT 1100', 5, 'boost',
        ['IT 1100-01', 'IT 1100-02', 'IT 1100-03'])
    db.make_conflict('Software Engineering', 'spread out SE 1400', 5, 'boost',
        ['SE 1400-01', 'SE 1400-02', 'SE 1400-03'])
