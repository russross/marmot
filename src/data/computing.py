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

    db.make_time_slot('MW0800+50', ['2 credit lecture'])
    db.make_time_slot('MW0900+50', ['2 credit lecture'])
    db.make_time_slot('MW1000+50', ['2 credit lecture'])
    db.make_time_slot('MW1100+50', ['2 credit lecture'])
    db.make_time_slot('MW1200+50', ['2 credit lecture'])
    db.make_time_slot('MW1330+50', ['2 credit lecture'])
    db.make_time_slot('MW1500+50', ['2 credit lecture'])
    db.make_time_slot('MW1630+50', ['2 credit lecture'])
    db.make_time_slot('TR0900+50', ['2 credit lecture'])
    db.make_time_slot('TR1030+50', ['2 credit lecture'])
    db.make_time_slot('TR1200+50', ['2 credit lecture'])
    db.make_time_slot('TR1330+50', ['2 credit lecture'])
    db.make_time_slot('TR1500+50', ['2 credit lecture'])
    db.make_time_slot('TR1630+50', ['2 credit lecture'])
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
    db.make_time_slot('R1900+50', [])
    db.make_time_slot('F1300+50', [])

    db.make_course('Computing', 'BTEC 1010', 'Fundamentals of Biotechnology')
    db.make_course('Computing', 'CS 6300', 'Principles of Artificial Intelligence')
    db.make_course('Computing', 'CS 6310', 'Foundations of Machine Learning')
    db.make_course('Computing', 'CS 6350', 'Artificial Intelligence and Machine Learning Project 1')
    db.make_course('Computing', 'IT 3750', 'Industrial Control System Security')
    db.make_course('Computing', 'SA 1400', 'Fundamentals of Programming (Success)')
    db.make_course('Computing', 'SE 6400', 'Advanced Topics in App Development')
    db.make_course('Computing', 'SE 6450', 'Graduate Capstone')
    db.make_section('MATH 2050-01')
    db.make_section('MATH 3400-01')

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
    default_availability = [Availability('MTWR', '0900', '1630'), Availability('F', '0900', '1200')]

    db.make_faculty('Andrew Wilson', 'Computing', default_availability)
    db.make_section('SD 6110-01', '3 credit bell schedule', 'Smith 117')
    db.assign_faculty_sections('Andrew Wilson', 'SD 6110-01')

    db.make_faculty('Bart Stander', 'Computing', default_availability)
    db.make_section('CS 2100-01', '3 credit bell schedule', 'Smith 116')
    db.make_section('CS 2420-01', '3 credit bell schedule', 'Smith 116')
    db.make_section('CS 2420-02', '3 credit bell schedule', 'Smith 116')
    db.make_section('CS 3500-01', '3 credit bell schedule', 'pcs')
    db.assign_faculty_sections('Bart Stander', 'CS 2100-01', 'CS 2420-01', 'CS 2420-02', 'CS 3500-01')

    db.make_faculty('Brayden Connole', 'Computing', default_availability)
    db.make_section('IT 4200-01', '3 credit bell schedule', 'flex')
    db.make_section('SE 1400-01', '3 credit bell schedule', 'macs')
    db.make_section('SE 1400-02', '3 credit bell schedule', 'macs')
    db.make_section('SE 3020-01', '3 credit bell schedule', 'macs')
    db.assign_faculty_sections('Brayden Connole', 'IT 4200-01', 'SE 1400-01', 'SE 1400-02', 'SE 3020-01')

    db.make_faculty('Carol Stander', 'Computing', default_availability)
    db.make_section('CS 1030-01', '3 credit bell schedule', 'flex')
    db.make_section('CS 1400-40')
    db.make_section('IT 1100-40')
    db.make_section('IT 2300-01', '3 credit bell schedule', 'flex')
    db.make_section('IT 2300-02', '3 credit bell schedule', 'flex')
    db.assign_faculty_sections('Carol Stander', 'CS 1030-01', 'CS 1400-40', 'IT 1100-40', 'IT 2300-01', 'IT 2300-02')

    db.make_faculty('Curtis Larsen', 'Computing', default_availability)
    db.make_section('CS 3530-01', '3 credit bell schedule', 'Smith 116')
    db.make_section('CS 4300-01', '3 credit bell schedule', 'Smith 116')
    db.make_section('CS 4920R-01')
    db.make_section('CS 6300-01','3 credit bell schedule', 'Smith 117')
    db.make_section('CS 6350-01','3 credit bell schedule', 'Smith 117')
    db.assign_faculty_sections('Curtis Larsen', 'CS 3530-01', 'CS 4300-01', 'CS 4920R-01', 'CS 6300-01', 'CS 6350-01')

    db.make_faculty('DJ Holt', 'Computing', default_availability)
    db.make_section('CS 4410-01', '3 credit bell schedule', 'macs')
    db.make_section('SD 6100-01', '3 credit bell schedule', 'Smith 117')
    db.make_section('SE 6400-01', '3 credit bell schedule', 'Smith 117')
    db.assign_faculty_sections('DJ Holt', 'CS 4410-01', 'SD 6100-01', 'SE 6400-01')

    db.make_faculty('Eric Pedersen', 'Computing', default_availability)
    db.make_section('SE 3500-01', '3 credit bell schedule', 'flex')
    db.make_section('SE 4990-01', '3 credit bell schedule', 'flex')
    db.make_section('SE 6450-01', '3 credit bell schedule', 'flex')
    db.assign_faculty_sections('Eric Pedersen', 'SE 3500-01', 'SE 4990-01', 'SE 6450-01')

    db.make_faculty('Jay Sneddon', 'Computing', default_availability)
    db.make_section('IT 1200-01', '3 credit bell schedule', 'Smith 107')
    db.make_section('IT 2700-01', '3 credit bell schedule', 'Smith 107')
    db.make_section('IT 3710-01', '3 credit bell schedule', 'Smith 107')
    db.make_section('IT 3750-01', '3 credit bell schedule', 'Smith 107')
    db.make_section('IT 4310-01', '3 credit bell schedule', 'Smith 107')
    db.make_section('IT 4990-01', '2 credit lecture', 'Smith 107')
    db.assign_faculty_sections('Jay Sneddon', 'IT 1200-01', 'IT 2700-01', 'IT 3710-01', 'IT 3750-01', 'IT 4310-01', 'IT 4990-01')

    db.make_faculty('Jeff Compas', 'Computing', default_availability)
    db.make_section('CS 1400-01', '3 credit bell schedule', 'flex', 'Smith 116')
    db.make_section('CS 2450-01', '3 credit bell schedule', 'flex')
    db.make_section('CS 2450-02', '3 credit bell schedule', 'flex')
    db.make_section('CS 3005-01', '3 credit bell schedule', 'Smith 116')
    db.make_section('SE 3150-01', '3 credit bell schedule', 'pcs')
    db.assign_faculty_sections('Jeff Compas', 'CS 1400-01', 'CS 2450-01', 'CS 2450-02', 'CS 3005-01', 'SE 3150-01')

    db.make_faculty('Joe Francom', 'Computing', default_availability)
    db.make_section('IT 1500-40A')
    db.make_section('IT 1500-41B')
    db.make_section('IT 3300-01', '3 credit bell schedule', 'flex')
    db.assign_faculty_sections('Joe Francom', 'IT 1500-40A', 'IT 1500-41B', 'IT 3300-01')

    db.make_faculty('Lora Klein', 'Computing', default_availability)
    db.make_section('CS 1400-02', '3 credit bell schedule', 'flex')
    db.make_section('SA 1400-01')
    db.make_section('SA 1400-02')
    db.make_section('SE 3200-01', '3 credit bell schedule', 'flex')
    db.assign_faculty_sections('Lora Klein', 'CS 1400-02', 'SA 1400-01', 'SA 1400-02', 'SE 3200-01')

    db.make_faculty('Matt Kearl', 'Computing', default_availability)
    db.make_section('SE 1400-03', '3 credit bell schedule', 'macs')
    db.make_section('SE 1400-40')
    db.make_section('SE 3400-01', '3 credit bell schedule', 'macs', 'flex')
    db.make_section('SE 3550-01', '3 credit bell schedule', 'macs', 'flex')
    db.assign_faculty_sections('Matt Kearl', 'SE 1400-03', 'SE 1400-40', 'SE 3400-01', 'SE 3550-01')

    db.make_faculty('Nicole Dang', 'Computing', default_availability)
    db.make_section('SET 1000-40')
    db.assign_faculty_sections('Nicole Dang', 'SET 1000-40')

    db.make_faculty('Phil Daley', 'Computing', default_availability)
    db.make_section('IT 1100-02', '3 credit bell schedule', 'pcs')
    db.make_section('IT 1100-03', '3 credit bell schedule', 'pcs')
    db.make_section('IT 2400-01', '3 credit bell schedule', 'Smith 107')
    db.make_section('IT 3100-01', '3 credit bell schedule', 'Smith 107')
    db.assign_faculty_sections('Phil Daley', 'IT 1100-02', 'IT 1100-03', 'IT 2400-01', 'IT 3100-01')

    db.make_faculty('Ren Quinn', 'Computing', [
        Availability('MTWF', '0900', '1630'),
        Availability('F', '0900', '1200'),
        Availability('R', '1900', '2000'),
        Availability('F', '1300', '1400'),
    ])
    db.make_section('CS 1400-03', '3 credit bell schedule', 'flex')
    db.make_section('CS 1410-01', '3 credit bell schedule', 'flex')
    db.make_section('CS 2500-01', '3 credit bell schedule', 'flex')
    db.make_section('CS 4800R-01')
    db.make_section('CS 4991R-50', 'R1900+50', 'Smith 116')
    db.make_section('CS 4992R-01', 'F1300+50', 'Smith 109')
    db.assign_faculty_sections('Ren Quinn', 'CS 1400-03', 'CS 1410-01', 'CS 2500-01', 'CS 4800R-01', 'CS 4991R-50', 'CS 4992R-01')

    db.make_faculty('Russ Ross', 'Computing', [Availability('MTWR', '1200', '1500')])
    db.faculty_preferences('Russ Ross', 'MT',
        DaysOff(0, 11), EvenlySpread(11), NoRoomSwitch(12), TooManyRooms(12),
        GapTooLong(105, 15), GapTooLong(195, 11),
        ClusterTooShort(110, 15), ClusterTooLong(165, 11))
    db.make_section('CS 2810-01', 'Smith 109', '3 credit bell schedule')
    db.make_section('CS 2810-02', 'Smith 109', '3 credit bell schedule')
    db.make_section('CS 3410-01', 'Smith 109', '3 credit bell schedule')
    db.make_section('CS 4307-01', 'Smith 109', '3 credit bell schedule')
    db.make_section('CS 4800R-02')
    db.assign_faculty_sections('Russ Ross', 'CS 2810-01', 'CS 2810-02', 'CS 3410-01', 'CS 4307-01', 'CS 4800R-02')

    db.make_faculty('Syed Ali', 'Computing', default_availability)
    db.make_section('IT 1100-01', '3 credit bell schedule', 'pcs')
    db.make_section('IT 2500-01', '3 credit bell schedule', 'flex')
    db.make_section('IT 4510-01', '3 credit bell schedule', 'flex')
    db.make_section('IT 4990-02', '3 credit bell schedule', 'flex')
    db.assign_faculty_sections('Syed Ali', 'IT 1100-01', 'IT 2500-01', 'IT 4510-01', 'IT 4990-02')

    db.make_faculty('Yuanfei Sun', 'Computing', default_availability)
    db.make_section('CS 1410-02', '3 credit bell schedule', 'flex')
    db.make_section('CS 6310-01', '3 credit bell schedule', 'Smith 117')
    db.make_section('CS 6350-02', '3 credit bell schedule', 'Smith 117')
    db.make_section('BTEC 1010-01', '3 credit bell schedule', 'flex')
    db.assign_faculty_sections('Yuanfei Sun', 'CS 1410-02', 'CS 6310-01', 'CS 6350-02', 'BTEC 1010-01')

    db.add_anti_conflict(5, 'CS 1030-01', ['CS 1400'])
    db.add_anti_conflict(5, 'SE 1400-01', ['IT 1100'])

    db.make_conflict('Computer Science', 'spread out CS 1400', 5, 'boost',
        ['CS 1400-01', 'CS 1400-02', 'CS 1400-03'])
    db.make_conflict('Computer Science', 'spread out CS 1410', 5, 'boost',
        ['CS 1410-01', 'CS 1410-02'])
    db.make_conflict('Computer Science', 'spread out CS 6350', 5, 'boost',
        ['CS 6350-01', 'CS 6350-02'])
    db.make_conflict('Information Technology', 'spread out IT 1100', 5, 'boost',
        ['IT 1100-01', 'IT 1100-02', 'IT 1100-03'])
    db.make_conflict('Information Technology', 'spread out IT 2300', 5, 'boost',
        ['IT 2300-01', 'IT 2300-02'])
    db.make_conflict('Software Engineering', 'spread out SE 1400', 5, 'boost',
        ['SE 1400-01', 'SE 1400-02', 'SE 1400-03'])
