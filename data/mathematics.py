import queries


def build_programs(db: queries.DB) -> None:
    # 2026–2027 Mathematics BA/BS worksheet:
    # https://catalog.utahtech.edu/programs/mathematics/mathematics-ba-bs/
    # The discrete mathematics requirement has two alternatives. Physics and
    # programming are required program courses alongside the mathematics core.
    db.make_program('Mathematics', 'Math')
    db.make_conflict('Mathematics', 'core requirements', 1, 'boost', [
        'MATH 1210', 'MATH 1220', 'MATH 2200', 'CS 2100',
        'MATH 2210', 'MATH 2270', 'MATH 2280', 'MATH 3120',
        'MATH 3200', 'MATH 3400', 'MATH 3500', 'MATH 4000',
        'MATH 4100', 'MATH 4200', 'MATH 4900', 'CS 1400',
        'PHYS 2210', 'PHYS 2215',
    ])
    db.make_conflict('Mathematics', 'discrete mathematics alternatives', None, 'reduce', [
        'MATH 2200', 'CS 2100',
    ])

    # Students select 12 credits from this list. MATH 4100 is already in
    # the core. The worksheet also lists MATH 4250, which has no course
    # description in the current catalog, so it is omitted pending review.
    db.make_conflict('Mathematics', 'mathematics electives', 3, 'boost', [
        'MATH 3000', 'MATH 3100', 'MATH 3150', 'MATH 3210',
        'MATH 3450', 'MATH 3700', 'MATH 3900', 'MATH 4010',
        'MATH 4800', 'MATH 4890R',
    ], include=['core requirements'])

    # Mathematics Education BA/BS worksheet:
    # https://catalog.utahtech.edu/programs/mathematics/mathematics-education-ba-bs/
    # Education courses are outside the current scheduling scope. MATH 4500
    # belongs to the professional semester after 95% of the major coursework,
    # so it is not grouped with the earlier mathematics requirements.
    db.make_program('Mathematics Education', 'Math')
    db.make_conflict('Mathematics Education', 'core requirements', 1, 'boost', [
        'MATH 1210', 'MATH 1220', 'MATH 2200', 'MATH 2210',
        'MATH 2270', 'MATH 2280', 'MATH 3000', 'MATH 3010',
        'MATH 3020', 'MATH 3100', 'MATH 3120', 'MATH 3200',
        'MATH 3400', 'MATH 4000', 'CS 1400', 'PHYS 2210', 'PHYS 2215',
    ])

    # The two current applied mathematics worksheets share a core. Each
    # emphasis has its own required courses and a choice of four electives.
    # Non-college courses on the worksheets are outside the current catalog.
    # https://catalog.utahtech.edu/programs/mathematics/applied-computational-math-actuarial-science-bs/
    # https://catalog.utahtech.edu/programs/mathematics/applied-computational-math-data-analytics-bs/
    core = [
        'CS 1400', 'CS 1410', 'MATH 1210', 'MATH 1220',
        'MATH 2200', 'CS 2100', 'MATH 2210', 'MATH 2270',
        'MATH 2280', 'MATH 3400', 'MATH 3500', 'MATH 3700',
        'MATH 4800', 'MATH 4890R', 'MATH 4900',
    ]
    common_electives = [
        'MATH 3100', 'MATH 3120', 'MATH 3150', 'MATH 3200',
        'MATH 3900', 'MATH 4000', 'MATH 4010', 'MATH 4100',
        'MATH 4200', 'MATH 4330',
    ]
    emphases = [
        ('Applied Mathematics - Actuarial Science',
         ['CS 2420', 'MATH 3410', 'MATH 3450', 'MATH 4400', 'MATH 4410'],
         ['MATH 3050']),
        ('Applied Mathematics - Data Analytics',
         ['MATH 2050', 'MATH 3050', 'MATH 3450', 'IT 1100', 'IT 2300', 'IT 2400'],
         ['CS 3005', 'IT 4510']),
    ]
    for program, required, electives in emphases:
        db.make_program(program, 'Math')
        db.make_conflict(program, 'required courses', 1, 'boost', core + required)
        db.make_conflict(program, 'discrete mathematics alternatives', None, 'reduce', [
            'MATH 2200', 'CS 2100',
        ])
        db.make_conflict(program, 'electives', 3, 'boost', common_electives + electives,
                         include=['required courses'])
