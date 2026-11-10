from queries import DB


def build_programs(db: DB) -> None:
    # Computer Engineering BS, 2026–2027 worksheet:
    # https://catalog.utahtech.edu/programs/engineering/computer-engineering-bs/
    # ENGL 3010 is outside the college catalog used in this dry run.
    db.make_program('Computer Engineering', 'Engineering')
    db.make_conflict('Computer Engineering', 'required courses', 1, 'boost', [
        'MATH 1210', 'MATH 1220', 'MATH 2250',
        'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225',
        'CS 1400', 'CS 1410', 'CS 2100', 'CS 2420', 'CS 2450',
        'CS 2810', 'CS 3400', 'CS 3410',
        'MECH 1000', 'MECH 1005', 'MECH 1205', 'MECH 2250',
        'MECH 2255', 'MECH 3200', 'MECH 3205',
        'ECE 2210', 'ECE 2215', 'ECE 2220', 'ECE 2225',
        'ECE 2700', 'ECE 2705', 'ECE 3230', 'ECE 3235',
        'ECE 3500', 'ECE 3730', 'ECE 3735', 'ECE 4730',
        'ECE 4735', 'ECE 4005', 'ECE 4015',
    ])

    # The engineering degrees have broad upper-division elective pools with
    # differing credit and prefix requirements. Those opportunistic choices
    # contribute no conflict pressure; each mandatory core remains separate.
    # Non-college ENGL 3010 is not modeled.
    # https://catalog.utahtech.edu/programs/engineering/electrical-engineering-bs/
    # https://catalog.utahtech.edu/programs/engineering/mechanical-engineering-bs/
    # https://catalog.utahtech.edu/programs/engineering/mechatronics-bs/
    programs = [
        ('Electrical Engineering', [
            'CHEM 1210', 'CHEM 1215', 'ECE 2210', 'ECE 2215',
            'ECE 2220', 'ECE 2225', 'ECE 2700', 'ECE 2705', 'ECE 3100',
            'ECE 3230', 'ECE 3235', 'ECE 3300', 'ECE 3305', 'ECE 3500',
            'ECE 3600', 'ECE 3605', 'ECE 3730', 'ECE 3735', 'ECE 4000',
            'ECE 4010', 'MATH 1210', 'MATH 1220', 'MATH 2210', 'MATH 2250',
            'ENGR 3450', 'MECH 1000', 'MECH 1005', 'MECH 1200', 'MECH 1205',
            'MECH 2250', 'MECH 2255', 'MECH 3200', 'MECH 3205',
            'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225',
        ]),
        ('Mechanical Engineering', [
            'MATH 1210', 'MATH 1220', 'MATH 2210', 'MATH 2250', 'ENGR 3450',
            'CHEM 1210', 'CHEM 1215', 'PHYS 2210', 'PHYS 2215', 'PHYS 2220',
            'PHYS 2225', 'MECH 1000', 'MECH 1005', 'MECH 1100', 'MECH 1150',
            'MECH 1200', 'MECH 1205', 'MECH 2010', 'MECH 2030', 'MECH 2160',
            'MECH 2165', 'ECE 2210', 'ECE 2215', 'MECH 2250', 'MECH 2255',
            'MECH 3200', 'MECH 3205', 'MECH 3250', 'MECH 3255', 'MECH 3300',
            'MECH 3305', 'MECH 3600', 'MECH 3605', 'MECH 3650', 'MECH 3655',
            'MECH 3700', 'MECH 3705', 'MECH 4000', 'MECH 4010',
        ]),
        ('Mechatronics', [
            'ENGR 2050', 'MECH 1150', 'MECH 1205', 'MECH 2010',
            'ECE 2210', 'ECE 2215', 'MECH 2250', 'MECH 2255',
            'MTRN 2200', 'MTRN 2205', 'MTRN 2300', 'MTRN 2305',
            'MTRN 2350', 'MTRN 2355', 'MTRN 2800', 'MTRN 2805',
            'MTRN 3400', 'MTRN 3405', 'MTRN 3410', 'MTRN 3415',
            'MTRN 3450', 'MTRN 3455', 'MTRN 3500', 'MTRN 3505',
            'MTRN 3560', 'MTRN 3565', 'MTRN 3600', 'MTRN 3605',
            'MTRN 4000', 'MTRN 4010', 'PHYS 2010', 'PHYS 2015',
            'PHYS 2020', 'PHYS 2025',
        ]),
    ]
    for program, required in programs:
        db.make_program(program, 'Engineering')
        db.make_conflict(program, 'required courses', 1, 'boost', required)
