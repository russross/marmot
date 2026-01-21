import queries
from queries import *

def build_pre(db: DB) -> None:
    print('building smith building and classrooms')
    db.make_building('Smith')
    db.make_room('Smith 107', 32, ['flex'])
    db.make_room('Smith 108', 32, ['flex'])
    db.make_room('Smith 109', 32, ['flex'])
    db.make_room('Smith 112', 24, ['macs'])
    db.make_room('Smith 113', 24, ['pcs'])
    db.make_room('Smith 116', 38, ['stadium'])
    db.make_room('Smith 117', 38, ['stadium'])
    #db.make_building('SET')
    #db.make_room('SET 105a', 30, [])
    #db.make_room('SET 105b', 30, [])
    #db.make_room('SET 105c', 30, [])
    #db.make_building('SUCCESS')
    #db.make_room('SUCCESS 100', 30, [])
    #db.make_building('Snow')
    #db.make_room('Snow 3050', 30, [])
    #db.make_room('Snow 3400', 30, [])

    print('building core time slots')

    db.make_time_slot('MWF0800+50', [])

    db.make_time_slot('MWF0900+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MWF1000+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MWF1100+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MW1200+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])
    db.make_time_slot('MW1330+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])
    db.make_time_slot('MW1500+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])

    db.make_time_slot('MW0900+50', ['2 credit bell schedule', 'MW 2×50 bell schedule'])
    db.make_time_slot('MW1000+50', ['2 credit bell schedule', 'MW 2×50 bell schedule'])
    db.make_time_slot('MW1100+50', ['2 credit bell schedule', 'MW 2×50 bell schedule'])
    db.make_time_slot('MW1200+50', ['2 credit bell schedule', 'MW 2×50 bell schedule'])
    db.make_time_slot('MW1330+50', ['2 credit bell schedule', 'MW 2×50 bell schedule'])
    db.make_time_slot('MW1500+50', ['2 credit bell schedule', 'MW 2×50 bell schedule'])

    db.make_time_slot('MW1630+75', [])

    db.make_time_slot('TR0900+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1030+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1200+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1330+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1500+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])

    db.make_time_slot('TR0900+50', ['2 credit bell schedule', 'TR 2×50 bell schedule'])
    db.make_time_slot('TR1030+50', ['2 credit bell schedule', 'TR 2×50 bell schedule'])
    db.make_time_slot('TR1200+50', ['2 credit bell schedule', 'TR 2×50 bell schedule'])
    db.make_time_slot('TR1330+50', ['2 credit bell schedule', 'TR 2×50 bell schedule'])
    db.make_time_slot('TR1500+50', ['2 credit bell schedule', 'TR 2×50 bell schedule'])

    db.make_time_slot('TR1630+75', [])

    db.make_time_slot('M1800+150', ['3 credit evening'])
    db.make_time_slot('T1800+150', ['3 credit evening'])
    db.make_time_slot('R1800+150', ['3 credit evening'])
    db.make_time_slot('W1800+150', ['3 credit evening'])
    db.make_time_slot('M1630+150', ['3 credit early evening'])
    db.make_time_slot('T1630+150', ['3 credit early evening'])
    db.make_time_slot('W1630+150', ['3 credit early evening'])
    db.make_time_slot('R1630+150', ['3 credit early evening'])

    print('adding special case courses and sections')
    db.make_course('Computing', 'SA 1400', 'Success CS 1400')
    db.make_course('Computing', 'CS 4420', 'Data Privacy, Security, and Ethics')    
    db.make_course('Computing', 'CS 4480R', 'Data Science Practicum')    
    db.make_course('Computing', 'CS 6300', 'Principles of Artificial Intelligence')    
    db.make_course('Computing', 'CS 6310', 'Foundations of Machine Learning')    
    db.make_course('Computing', 'CS 6320', 'Foundations of Deep Learning')
    db.make_course('Computing', 'CS 6330', 'Programming for Machine Learning in Life Sciences')
    db.make_course('Computing', 'CS 6331', 'Machine Learning for Life Sciences')
    db.make_course('Computing', 'CS 6350', 'Artificial Intelligence and Machine Learning Project 1')    
    db.make_course('Computing', 'IT 2750', 'Industrial Networking Essentials')
    #db.make_section_with_no_faculty('MATH 3050-01', 'MW1330+75', 'Snow 3050')
    #db.make_section_with_no_faculty('MATH 3400-01', 'TR1030+75', 'Snow 3400')

def build_post(db: DB) -> None:
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
    db.make_conflict('Software Engineering', 'Game development track', 5, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2100', 'CS 2420', 'CS 2810', 'CS 3005',
        'CS 3150', 'CS 3510', 'CS 4307', 'IT 2300', 'IT 1100',
        'SE 1400', 'CS 2450', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600', 'MATH 1100', 'MATH 1210', 'MATH 2050',
        'CS 3500', 'CS 3600', 'CS 4995'])
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
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 3500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050'])
    db.make_conflict('Information Technology', 'core electives', 2, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 3500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3110', 'IT 3300', 'IT 3710', 'IT 4100', 'IT 4200', 'IT 4310', 'IT 4400', 'IT 4510', 'IT 4920R'])
    db.make_conflict('Information Technology', 'DevOps requirements', 3, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 3500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3110', 'IT 3300', 'IT 4200', 'CS 2450'])
    db.make_conflict('Information Technology', 'DevOps requirements vs core electives', 5, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 3500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3110', 'IT 3300', 'IT 3710', 'IT 4100', 'IT 4200', 'IT 4310', 'IT 4400', 'IT 4510', 'IT 4920R',
        'CS 2450'])
    db.make_conflict('Information Technology', 'Cybersecurity requirements', 3, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 3500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3710', 'IT 4400', 'IT 4510'])
    # Cybersecurity requirements are a subset of core electives
    db.make_conflict('Information Technology', 'Cybersecurity choose 2', 4, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 3500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3710', 'IT 4400', 'IT 4510',
        'IT 4310', 'IT 4990', 'CS 2420', 'CS 2810'])
    db.make_conflict('Information Technology', 'Cybersecurity choose 2 vs core electives', 6, 'boost',
        ['CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500', 'IT 2300', 'IT 2400', 'IT 3500', 'IT 2700',
        'IT 3100', 'IT 3150', 'IT 3400', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'IT 3110', 'IT 3300', 'IT 3710', 'IT 4100', 'IT 4200', 'IT 4310', 'IT 4400', 'IT 4510', 'IT 4920R',
        'IT 4990', 'CS 2420', 'CS 2810'])
    db.make_conflict('Information Technology', 'only need one math class', None, 'reduce',
        ['MATH 1040', 'MATH 1050'])


    # TODO

    #db.add_anti_conflict(5, 'CS 4600-01', ['CS 4600-02', 'CS 4600-03'])
    #db.add_anti_conflict(5, 'CS 4600-02', ['CS 4600-01', 'CS 4600-03'])
    #db.add_anti_conflict(5, 'CS 4600-03', ['CS 4600-01', 'CS 4600-02'])
    #db.add_anti_conflict(5, 'CS 1030-01', ['CS 1400'])
    #db.add_multiple_section_override('CS 4600', 1)
    #db.add_multiple_section_override('SE 4600', 1)

    #db.make_conflict('Computer Science', 'spread out CS 1400', 5, 'boost',
    #    ['CS 1400-01', 'CS 1400-02', 'CS 1400-03'])
    #db.make_conflict('Computer Science', 'spread out CS 1410', 5, 'boost',
    #    ['CS 1410-01', 'CS 1410-02'])
    #db.make_conflict('Information Technology', 'spread out IT 1100', 5, 'boost',
    #    ['IT 1100-01', 'IT 1100-02'])
