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

    print('building core time slots')

    db.make_time_slot('MWF0800+50', [])
    db.make_time_slot('MWF0900+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MWF1000+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MWF1100+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MW1200+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])
    db.make_time_slot('MW1330+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])
    db.make_time_slot('MW1500+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])
    db.make_time_slot('MW1630+75', [])

    db.make_time_slot('TR0900+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1030+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1200+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1330+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1500+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1630+75', [])

    db.make_time_slot('M1630+150', ['3 credit early evening'])
    db.make_time_slot('T1630+150', ['3 credit early evening'])
    db.make_time_slot('W1630+150', ['3 credit early evening'])
    db.make_time_slot('R1630+150', ['3 credit early evening'])
    db.make_time_slot('M1800+150', ['3 credit evening'])
    db.make_time_slot('T1800+150', ['3 credit evening'])
    db.make_time_slot('R1800+150', ['3 credit evening'])
    db.make_time_slot('W1800+150', ['3 credit evening'])

    print('adding special case courses and sections')
    db.make_course('Computing', 'SA 1400', 'Success CS 1400')
    db.make_course('Computing', 'CS 6300', 'Principles of Artificial Intelligence')    
    db.make_course('Computing', 'CS 6310', 'Foundations of Machine Learning')    
    db.make_course('Computing', 'CS 6320', 'Foundations of Deep Learning')
    db.make_course('Computing', 'CS 6330', 'Programming for Machine Learning in Life Sciences')
    db.make_course('Computing', 'CS 6331', 'Machine Learning for Life Sciences')
    db.make_course('Computing', 'CS 6350', 'Artificial Intelligence and Machine Learning Project 1')    
    db.make_section_with_no_faculty('MATH 3050-01', 'MW1330+75')
    db.make_section_with_no_faculty('MATH 3400-01', 'TR1030+75')

def build_post(db: DB) -> None:
    print('building computing conflicts')

    #
    # CS degree
    #
    db.make_program('Computer Science', 'Computing')
    db.make_conflict('Computer Science', 'core requirements', 1, 'boost', [
        # core requirements
        'CS 1030', 'CS 1400', 'CS 1410',
        'CS 2420', 'CS 2450', 'CS 2810',
        'CS 3005', 'CS 3530', 'CS 3510',
        'CS 4600',

        # math core requirements
        'MATH 1210', 'MATH 3400',
        'CS 2100',
    ])
    db.make_conflict('Computer Science', 'core electives', 2, 'boost', [
        # core requirements
        'CS 1030', 'CS 1400', 'CS 1410',
        'CS 2420', 'CS 2450', 'CS 2810',
        'CS 3005', 'CS 3530', 'CS 3510',
        'CS 4600',

        # math core requirements
        'MATH 1210', 'MATH 3400',
        'CS 2100',

        # core electives
        'CS 3150', 'CS 3400', 'CS 3410', 'CS 3520', 'CS 3600',
        'CS 4300', 'CS 4307', 'CS 4320', 'CS 4550',
        'SE 3200',
    ])
    db.make_conflict('Computer Science', 'math electives', 6, 'boost', [
        # core requirements
        'CS 1030', 'CS 1400', 'CS 1410',
        'CS 2420', 'CS 2450', 'CS 2810',
        'CS 3005', 'CS 3530', 'CS 3510',
        'CS 4600',

        # math core requirements
        'MATH 1210', 'MATH 3400',
        'CS 2100',

        # core electives
        'CS 3150', 'CS 3400', 'CS 3410', 'CS 3520', 'CS 3600',
        'CS 4300', 'CS 4307', 'CS 4320', 'CS 4550',
        'SE 3200',

        # math electives
        'MATH 1220',
        'MATH 2210', 'MATH 2250', 'MATH 2270', 'MATH 2280',
        'MATH 3050', 'MATH 3450',
    ])

    #
    # Data science degree
    #
    db.make_program('Data Science', 'Computing')
    db.make_conflict('Data Science', 'core requirements', 1, 'boost', [
        # computing core requirements
        'CS 1030', 'CS 1400', 'CS 1410',
        'CS 2100', 'CS 2420', 'CS 2810',
        'CS 3510',
        'IT 1500',

        # data science core requirements
        'CS 2500',
        'CS 4400', 'CS 4410', 'CS 4420', 'CS 4480R', 'CS 4490R',

        # math core requirements
        'MATH 1210', 'MATH 1220',
        'MATH 2270',
        'MATH 3400',
    ])
    db.make_conflict('Data Science', 'AI/ML track', 5, 'boost', [
        # computing core requirements
        'CS 1030', 'CS 1400', 'CS 1410',
        'CS 2100', 'CS 2420', 'CS 2810',
        'CS 3510',
        'IT 1500',

        # data science core requirements
        'CS 2500',
        'CS 4400', 'CS 4410', 'CS 4420', 'CS 4480R', 'CS 4490R',

        # math core requirements
        'MATH 1210', 'MATH 1220',
        'MATH 2270',
        'MATH 3400',

        # ai/ml track
        'CS 3005',
        'CS 4300', 'CS 4320',
    ])
    db.make_conflict('Data Science', 'Data engineering track', 5, 'boost', [
        # computing core requirements
        'CS 1030', 'CS 1400', 'CS 1410',
        'CS 2100', 'CS 2420', 'CS 2810',
        'CS 3510',
        'IT 1500',

        # data science core requirements
        'CS 2500',
        'CS 4400', 'CS 4410', 'CS 4420', 'CS 4480R', 'CS 4490R',

        # math core requirements
        'MATH 1210', 'MATH 1220',
        'MATH 2270',
        'MATH 3400',

        # data engineering track
        'CS 3150',
        'CS 3410', 'CS 4307',
    ])
    db.make_conflict('Data Science', 'Software engineering track', 5, 'boost', [
        # computing core requirements
        'CS 1030', 'CS 1400', 'CS 1410',
        'CS 2100', 'CS 2420', 'CS 2810',
        'CS 3510',
        'IT 1500',

        # data science core requirements
        'CS 2500',
        'CS 4400', 'CS 4410', 'CS 4420', 'CS 4480R', 'CS 4490R',

        # math core requirements
        'MATH 1210', 'MATH 1220',
        'MATH 2270',
        'MATH 3400',

        # software engineering track
        'CS 2450',
        'SE 3100', 'SE 3150',
    ])

    #
    # SE degree
    #
    db.make_program('Software Engineering', 'Computing')
    db.make_conflict('Software Engineering', 'core requirements', 1, 'boost', [
        # core requirements
        'CS 1030', 'CS 1400', 'CS 1410',
        'CS 2100', 'CS 2420', 'CS 2450', 'CS 2810',
        'CS 3005', 'CS 3150', 'CS 3510',
        'CS 4307', 'IT 2300', # one database class
        'IT 1100',
        'SE 1400',
        'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600',
        'MATH 1100', 'MATH 1210', # one calc class
        'MATH 2050',
    ])
    db.make_conflict('Software Engineering', 'Entrepreneurial and marketing track', 5, 'boost', [
        # core requirements
        'CS 1030', 'CS 1400', 'CS 1410',
        'CS 2100', 'CS 2420', 'CS 2450', 'CS 2810',
        'CS 3005', 'CS 3150', 'CS 3510',
        'CS 4307', 'IT 2300', # one database class
        'IT 1100',
        'SE 1400',
        'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600',
        'MATH 1100', 'MATH 1210', # one calc class
        'MATH 2050',

        # entrepreneurial and marketing track
        #'DES 2100',
        'SE 3500', 'SE 3550',
    ])
    db.make_conflict('Software Engineering', 'DevOps track', 5, 'boost', [
        # core requirements
        'CS 1030', 'CS 1400', 'CS 1410',
        'CS 2100', 'CS 2420', 'CS 2450', 'CS 2810',
        'CS 3005', 'CS 3150', 'CS 3510',
        'CS 4307', 'IT 2300', # one database class
        'IT 1100',
        'SE 1400',
        'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600',
        'MATH 1100', 'MATH 1210', # one calc class
        'MATH 2050',

        # devops track
        'IT 3110', 'IT 3300',
        'IT 4200',
    ])
    db.make_conflict('Software Engineering', 'Application track', 5, 'boost', [
        # core requirements
        'CS 1030', 'CS 1400', 'CS 1410',
        'CS 2100', 'CS 2420', 'CS 2450', 'CS 2810',
        'CS 3005', 'CS 3150', 'CS 3510',
        'CS 4307', 'IT 2300', # one database class
        'IT 1100',
        'SE 1400',
        'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600',
        'MATH 1100', 'MATH 1210', # one calc class
        'MATH 2050',

        # application track
        'SE 3010', 'SE 3250', 'SE 3450',
    ])
    db.make_conflict('Software Engineering', 'Data science track', 5, 'boost', [
        # core requirements
        'CS 1030', 'CS 1400', 'CS 1410',
        'CS 2100', 'CS 2420', 'CS 2450', 'CS 2810',
        'CS 3005', 'CS 3150', 'CS 3510',
        'CS 4307', 'IT 2300', # one database class
        'IT 1100',
        'SE 1400',
        'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600',
        'MATH 1100', 'MATH 1210', # one calc class
        'MATH 2050',
        'SE 3010', # mobile app dev?

        # data science track
        'CS 4300', 'CS 4400', # ai or data mining
        'CS 4320', 'CS 4410',
    ])
    db.make_conflict('Software Engineering', 'Game development track', 5, 'boost', [
        # core requirements
        'CS 1030', 'CS 1400', 'CS 1410',
        'CS 2100', 'CS 2420', 'CS 2450', 'CS 2810',
        'CS 3005', 'CS 3150', 'CS 3510',
        'CS 4307', 'IT 2300', # one database class
        'IT 1100',
        'SE 1400',
        'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600',
        'MATH 1100', 'MATH 1210', # one calc class
        'MATH 2050',
        'SE 3010', # mobile app dev?

        # game development track
        'CS 3500', 'CS 3600',
        'CS 4995',
    ])

    db.make_conflict('Software Engineering', 'only need one of AI/data mining', None, 'reduce',
        ['CS 4300', 'CS 4400'])
    db.make_conflict('Software Engineering', 'only need one database class', None, 'reduce',
        ['CS 4307', 'IT 2300'])
    db.make_conflict('Software Engineering', 'only need one calculus class', None, 'reduce',
        ['MATH 1100', 'MATH 1210'])

    #
    # IT degrees
    #
    db.make_program('Information Technology', 'Computing')

    # IT
    db.make_conflict('Information Technology', 'core requirements', 1, 'boost', [
        # core requirements
        'CS 1400', 'CS 1410',
        'IT 1100', 'IT 1200', 'IT 1500',
        'IT 2150', 'IT 2300', 'IT 2400', 'IT 2700',
        'IT 3100', 'IT 3400', 'IT 3500',
        'IT 4600',
        'MATH 1040', 'MATH 1050',
    ])
    db.make_conflict('Information Technology', 'core electives', 2, 'boost', [
        # core requirements
        'CS 1400', 'CS 1410',
        'IT 1100', 'IT 1200', 'IT 1500',
        'IT 2150', 'IT 2300', 'IT 2400', 'IT 2700',
        'IT 3100', 'IT 3400', 'IT 3500',
        'IT 4600',
        'MATH 1040', 'MATH 1050',

        # core electives
        'IT 3110', 'IT 3300', 'IT 3710',
        'IT 4100', 'IT 4200', 'IT 4310', 'IT 4400', 'IT 4510', 'IT 4920R',
    ])

    # DevOps
    db.make_conflict('Information Technology', 'DevOps requirements', 3, 'boost', [
        # core requirements
        'CS 1400', 'CS 1410',
        'IT 1100', 'IT 1200', 'IT 1500',
        'IT 2150', 'IT 2300', 'IT 2400', 'IT 2700',
        'IT 3100', 'IT 3400', 'IT 3500',
        'IT 4600',
        'MATH 1040', 'MATH 1050',

        # core electives
        'CS 2450',
        'IT 3110', 'IT 3300', 'IT 4200',
    ])
    db.make_conflict('Information Technology', 'DevOps requirements vs core electives', 5, 'boost', [
        # core requirements
        'CS 1400', 'CS 1410',
        'IT 1100', 'IT 1200', 'IT 1500',
        'IT 2150', 'IT 2300', 'IT 2400', 'IT 2700',
        'IT 3100', 'IT 3400', 'IT 3500',
        'IT 4600',
        'MATH 1040', 'MATH 1050',

        # core electives
        'IT 3110', 'IT 3300', 'IT 3710',
        'IT 4100', 'IT 4200', 'IT 4310', 'IT 4400', 'IT 4510', 'IT 4920R',
        'CS 2450',
    ])

    # Cybersecurity
    db.make_conflict('Information Technology', 'Cybersecurity requirements', 3, 'boost', [
        'CS 1400', 'CS 1410', 'CS 2420',
        'IT 1100', 'IT 1500',
        'IT 2400', 'IT 2600', 'IT 2700', 'IT 2750',
        'IT 3100', 'IT 3110', 'IT 3400', 'IT 3700', 'IT 3710',
        'IT 4510', 'IT 4600', 'IT 4700',
        'MATH 1040', 'MATH 1050',
    ])

    # only need one math class
    db.make_conflict('Information Technology', 'only need one math class', None, 'reduce',
        ['MATH 1040', 'MATH 1050'])


    # Misc

    #db.add_anti_conflict(5, 'CS 4600-01', ['CS 4600-02', 'CS 4600-03'])
    #db.add_anti_conflict(5, 'CS 4600-02', ['CS 4600-01', 'CS 4600-03'])
    #db.add_anti_conflict(5, 'CS 4600-03', ['CS 4600-01', 'CS 4600-02'])
    db.add_anti_conflict(5, 'CS 1030-01', ['CS 1400'])
    #db.add_multiple_section_override('CS 4600', 1)
    #db.add_multiple_section_override('SE 4600', 1)

    db.make_conflict('Computer Science', 'spread out CS 1400', 5, 'boost',
        ['CS 1400-01', 'CS 1400-02', 'CS 1400-03', 'CS 1400-04'])
    db.make_conflict('Computer Science', 'spread out CS 1410', 5, 'boost',
        ['CS 1410-01', 'CS 1410-02'])
    db.make_conflict('Information Technology', 'spread out IT 1100', 5, 'boost',
        ['IT 1100-01', 'IT 1100-02', 'IT 1100-03'])
