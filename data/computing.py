from queries import DB

def build_pre(db: DB) -> None:
    # Computing graduate sections use these evening groups in addition to
    # individually specified evening time slots.
    db.make_time_slot('M1630+150', ['3 credit early evening'])
    db.make_time_slot('T1630+150', ['3 credit early evening'])
    db.make_time_slot('W1630+150', ['3 credit early evening'])
    db.make_time_slot('R1630+150', ['3 credit early evening'])
    db.make_time_slot('M1800+150', ['3 credit evening'])
    db.make_time_slot('T1800+150', ['3 credit evening'])
    db.make_time_slot('R1800+150', ['3 credit evening'])
    db.make_time_slot('W1800+150', ['3 credit evening'])

    print('adding special case computing course')
    db.make_course('Computing', 'SA 1410', 'Success CS 1410', 3, 3)
    db.make_course('Computing', 'SA 1100', 'Success IT 1100', 3, 3)

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
        # core electives
        'CS 3150', 'CS 3400', 'CS 3410', 'CS 3520', 'CS 3600',
        'CS 4300', 'CS 4307', 'CS 4320', 'CS 4550',
        'SE 3200',
        'CS 4990',
    ], include=['core requirements'])
    db.make_conflict('Computer Science', 'math electives and supported science option', 6, 'boost', [
        # math electives
        'MATH 1220',
        'MATH 2210', 'MATH 2250', 'MATH 2270', 'MATH 2280',
        'MATH 3050', 'MATH 3450',

        # science core option available in the course catalog
        'PHYS 2210', 'PHYS 2215',
    ], include=['core electives'])

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
        # ai/ml track
        'CS 3005',
        'CS 4300', 'CS 4320',
    ], include=['core requirements'])
    db.make_conflict('Data Science', 'Data engineering track', 5, 'boost', [
        # data engineering track
        'CS 3150',
        'CS 3410', 'CS 4307',
    ], include=['core requirements'])
    db.make_conflict('Data Science', 'Software engineering track', 5, 'boost', [
        # software engineering track
        'CS 2450',
        'SE 3100', 'SE 3150',
    ], include=['core requirements'])
    db.make_conflict('Data Science', 'all tracks combined', 7, 'boost', [
        # all track courses (students complete two tracks)
        'CS 2450',
        'CS 3005', 'CS 3150', 'CS 3410',
        'CS 4307', 'CS 4300', 'CS 4320',
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
        # entrepreneurial and marketing track
        'DES 2100',
        'SE 3500', 'SE 3550',
    ], include=['core requirements'])
    db.make_conflict('Software Engineering', 'DevOps track', 5, 'boost', [
        # devops track
        'IT 3110', 'IT 3300',
        'IT 4200',
    ], include=['core requirements'])
    db.make_conflict('Software Engineering', 'Application track', 5, 'boost', [
        # application track
        'SE 3010', 'SE 3250', 'SE 3450',
    ], include=['core requirements'])
    db.make_conflict('Software Engineering', 'Data science track', 5, 'boost', [
        # data science track
        'CS 4300', 'CS 4400', # ai or data mining
        'CS 4320', 'CS 4410',
    ], include=['core requirements'])
    db.make_conflict('Software Engineering', 'Game development track', 5, 'boost', [
        # game development track
        'CS 3500', 'CS 3600',
        'CS 4995',
    ], include=['core requirements'])
    db.make_conflict('Software Engineering', 'CS 4995 special case', 9, 'boost', [
        # CS core electives
        'CS 3150', 'CS 3400', 'CS 3410', 'CS 3520', 'CS 3600',
        'CS 4300', 'CS 4307', 'CS 4320', 'CS 4550',
        'SE 3200',

        # give 4995 a better chance to make
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
        'CS 1030', 'CS 1400', 'CS 1410',
        'IT 1100', 'IT 1200', 'IT 1500',
        'IT 2150', 'IT 2300', 'IT 2400', 'IT 2700',
        'IT 3100', 'IT 3400', 'IT 3510',
        'IT 4600',
        'MATH 1040', 'MATH 1050',
    ])
    db.make_conflict('Information Technology', 'core electives', 2, 'boost', [
        # core electives
        'IT 3110', 'IT 3300', 'IT 3710',
        'IT 4100', 'IT 4200', 'IT 4400', 'IT 4510', 'IT 4920R',
    ], include=['core requirements'])

    # DevOps is a separate published emphasis. Its four emphasis courses are
    # mandatory rather than choices within the regular IT degree.
    # https://catalog.utahtech.edu/programs/computing/information-technology-devops-bs/
    db.make_program('Information Technology - DevOps', 'Computing')
    db.make_conflict('Information Technology - DevOps', 'core requirements', 1, 'boost', [
        'CS 1030', 'CS 1400', 'CS 1410', 'IT 1100', 'IT 1200', 'IT 1500',
        'IT 2150', 'IT 2300', 'IT 2400', 'IT 2700', 'IT 3100', 'IT 3400',
        'IT 3510', 'IT 4600', 'MATH 1040', 'MATH 1050',
        'CS 2450', 'IT 3110', 'IT 3300', 'IT 4200',
    ])
    db.make_conflict('Information Technology - DevOps', 'core electives', 2, 'boost', [
        'IT 4100', 'IT 4400', 'IT 4510', 'IT 4920R',
    ], include=['core requirements'])
    # A second nine-credit elective list is much broader and is omitted.
    db.make_conflict('Information Technology - DevOps', 'only need one math class', None, 'reduce',
        ['MATH 1040', 'MATH 1050'])

    # IT BAP
    db.make_program('Information Technology BAP', 'Computing')
    db.make_conflict('Information Technology BAP', 'core requirements', 1, 'boost', [
        'CS 1030', 'CS 1400', 'CS 1410',
        'IT 1100', 'IT 1200', 'IT 1500',
        'IT 2150', 'IT 2300', 'IT 2400', 'IT 2700',
        'IT 3100', 'IT 3400', 'IT 3510',
        'IT 4600',
    ])
    db.make_conflict('Information Technology BAP', 'electives', 4, 'boost', [
        # electives
        'CS 2450', 'CS 3005',
        'IT 3110', 'IT 3300', 'IT 3700', 'IT 3710',
        'IT 4100', 'IT 4200', 'IT 4310', 'IT 4400',
        'IT 4510', 'IT 4700', 'IT 4920R', 'IT 4990',
    ], include=['core requirements'])

    # only need one math class
    db.make_conflict('Information Technology', 'only need one math class', None, 'reduce',
        ['MATH 1040', 'MATH 1050'])

    # Cybersecurity
    db.make_program('Cybersecurity', 'Computing')
    db.make_conflict('Cybersecurity', 'core requirements', 1, 'boost', [
        'CS 1030', 'CS 1400', 'CS 1410', 'CS 2420',
        'IT 1100', 'IT 1500', 'IT 2150',
        'IT 2400', 'IT 2600', 'IT 2700', 'IT 2750',
        'IT 3100', 'IT 3110', 'IT 3400', 'IT 3700', 'IT 3710',
        'IT 4510', 'IT 4600', 'IT 4700',
        'MATH 1040', 'MATH 1050',
    ])
    db.make_conflict('Cybersecurity', 'only need one math class', None, 'reduce',
        ['MATH 1040', 'MATH 1050'])

    # The master's worksheets list required courses and one mobile elective.
    # https://catalog.utahtech.edu/programs/computing/software-development-msd/
    # https://catalog.utahtech.edu/programs/computing/artifical-intelligence-ms/
    db.make_program('Software Development MS', 'Computing')
    db.make_conflict('Software Development MS', 'required courses', 1, 'boost', [
        'SD 6100', 'SD 6110', 'SD 6200', 'SD 6210', 'SD 6220',
        'SD 6300', 'SD 6310', 'SD 6400', 'SD 6450',
    ])
    db.make_conflict('Software Development MS', 'mobile elective', 1, 'boost',
        ['SD 6330', 'SD 6340'], include=['required courses'])
    db.make_conflict('Software Development MS', 'only one mobile elective', None, 'reduce',
        ['SD 6330', 'SD 6340'])

    # Block A and B sections still use the solver's full-term overlap model.
    db.make_program('Artificial Intelligence MS', 'Computing')
    db.make_conflict('Artificial Intelligence MS', 'required courses', 1, 'boost', [
        'CS 6300', 'CS 6310', 'CS 6320', 'CS 6321', 'CS 6322', 'CS 6323',
        'CS 6350', 'CS 6351', 'CS 6352', 'CS 6353',
    ])


    # Misc

    # all CS/SE senior project sections should meet at the same time
    db.add_anti_conflict(5, 'CS 4600-01', ['CS 4600-02'])
    db.add_anti_conflict(5, 'CS 4600-02', ['CS 4600-03'])
    db.add_anti_conflict(5, 'CS 4600-03', ['SE 4600-04'])
    db.add_anti_conflict(5, 'SE 4600-04', ['CS 4600-01'])

    db.add_anti_conflict(5, 'CS 1030-01', ['CS 1400'])
    db.add_multiple_section_override('CS 4600', 1)
    db.add_multiple_section_override('SE 4600', 1)

    db.make_conflict('Computer Science', 'spread out CS 1400', 5, 'boost',
        ['CS 1400-01', 'CS 1400-02'])
    db.make_conflict('Computer Science', 'spread out CS 3510', 5, 'boost',
        ['CS 3510-01', 'CS 3510-02'])
    db.make_conflict('Computer Science', 'spread out CS 1410', 5, 'boost',
        ['CS 1410-01', 'CS 1410-02'])
    db.make_conflict('Information Technology', 'spread out IT 1100', 5, 'boost',
        ['IT 1100-01', 'IT 1100-02'])
