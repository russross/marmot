from queries import DB


def add_choice(db: DB, program: str, label: str, alternatives: list[list[str]],
               previous: str = 'required courses') -> None:
    # Courses within an alternative remain compatible requirements; only
    # pairs from different alternatives have their conflict canceled.
    choices = [course for alternative in alternatives for course in alternative]
    db.make_conflict(program, label, 1, 'boost', choices, include=[previous])
    for i, left in enumerate(alternatives):
        for right in alternatives[i + 1:]:
            for a in left:
                for b in right:
                    db.make_conflict(program, f'{label}: {a} or {b}', None, 'reduce', [a, b])


def build_programs(db: DB) -> None:
    # Biotechnology BS, 2026–2027 worksheet:
    # https://catalog.utahtech.edu/programs/biological-sciences/biotechnology-bs/
    # PHIL 3555 is outside the college catalog. The six-credit capstone and
    # broad upper-division elective requirement are omitted pending review of
    # their individualized offerings and senior-standing assumptions.
    program = 'Biotechnology'
    db.make_program(program, 'Biological Sciences')
    db.make_conflict(program, 'required courses', 1, 'boost', [
        'BTEC 1010', 'BTEC 1015R', 'BTEC 2030', 'BTEC 3015R',
        'BIOL 1610', 'BIOL 1615', 'BIOL 1620', 'BIOL 1625',
        'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225',
        'MATH 1050', 'BIOL 3010', 'BIOL 4010', 'BIOL 3030',
        'BIOL 4300', 'BIOL 4305', 'BIOL 3550', 'BIOL 3555',
        'BIOL 3450', 'BIOL 3455', 'PHYS 1010', 'PHYS 1015',
        'PHYS 2010', 'PHYS 2015', 'CHEM 2310', 'CHEM 2315',
        'CHEM 2320', 'CHEM 2325',
    ])
    db.make_conflict(program, 'evolution alternatives', None, 'reduce', ['BIOL 3010', 'BIOL 4010'])

    # Each alternative consists of a lecture and its lab. Cancel only
    # cross-alternative pairs; each lecture still conflicts with its own lab.
    for label, left, right in [
        ('biology alternatives', ['BIOL 3550', 'BIOL 3555'], ['BIOL 3450', 'BIOL 3455']),
        ('physics alternatives', ['PHYS 1010', 'PHYS 1015'], ['PHYS 2010', 'PHYS 2015']),
    ]:
        for a in left:
            for b in right:
                db.make_conflict(program, f'{label}: {a} or {b}', None, 'reduce', [a, b])

    # Students complete two tracks. Preserve compatibility within each track
    # and with the core, but omit pressure between optional track combinations.
    # BIOL 3030/4300/4305 also appear in the mandatory core.
    for label, track in [
        ('functional genomics', ['BTEC 2050', 'BTEC 3050', 'BTEC 4040', 'BTEC 4050']),
        ('protein characterization', ['BTEC 2020', 'BIOL 4300', 'BIOL 4305', 'BTEC 4020']),
        ('genetic sequencing', ['BTEC 2010', 'BIOL 3030', 'BTEC 3010']),
        ('bioinformatics', ['BIOL 2300', 'CS 1400', 'BIOL 3300']),
        ('machine learning', ['BIOL 2070', 'CS 1400', 'BIOL 3070']),
    ]:
        db.make_conflict(program, f'{label} track', 3, 'boost', track,
                         include=['required courses'])

    # Bioinformatics BS, 2026–2027 worksheet:
    # https://catalog.utahtech.edu/programs/biological-sciences/bioinformatics-bs/
    # Research is individualized. Senior Seminar is omitted until its
    # de-facto prerequisite assumptions can be checked across biology degrees.
    program = 'Bioinformatics'
    db.make_program(program, 'Biological Sciences')
    db.make_conflict(program, 'required courses', 1, 'boost', [
        'BIOL 1610', 'BIOL 1615', 'BIOL 1620', 'BIOL 1625',
        'BIOL 3300', 'BIOL 3030', 'BIOL 3150', 'BIOL 3155',
        'BIOL 4010', 'BIOL 4300', 'BIOL 4305', 'BIOL 4310', 'BIOL 4320',
        'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225',
        'CS 1400', 'CS 1410', 'CS 2450', 'CS 2100', 'IT 1100',
        'IT 2300', 'MATH 1210',
    ])
    lab_choices = ['BTEC 2010', 'BTEC 2020', 'BTEC 2030', 'BTEC 2050', 'BIOL 2300']
    db.make_conflict(program, 'technical lab choices', 1, 'boost', lab_choices,
                     include=['required courses'])
    db.make_conflict(program, 'only one technical lab', None, 'reduce', lab_choices)

    # The fifteen-credit elective requirement spans many departments and
    # allows opportunistic choices, so it contributes no conflict pressure.

    # Genomics MS worksheets list 5000/6000 paired numbers. Use the 6000
    # graduate numbers here; prerequisite undergraduate courses are admission
    # requirements, not concurrent degree requirements. Capstone is included
    # because the posted schedule has timed capstone meetings.
    genomics_core = [
        'BIOL 6100', 'BIOL 6320', 'BIOL 6330', 'BIOL 6910', 'BIOL 6920',
        'BIOL 6930', 'BIOL 6940', 'BIOL 6900R',
    ]
    # https://catalog.utahtech.edu/programs/biological-sciences/genomics-functional-genomics-ms/
    # https://catalog.utahtech.edu/programs/biological-sciences/genomics-bioinformatics-ms/
    for name, emphasis in [
        ('Genomics - Functional Genomics', [
            'BIOL 6050', 'BIOL 6550', 'BIOL 6555', 'BIOL 6630', 'BIOL 6430', 'BIOL 6500',
        ]),
        ('Genomics - Bioinformatics', [
            'BIOL 6070', 'BIOL 6010', 'BIOL 6040', 'BIOL 6045', 'BIOL 6130', 'BIOL 6310',
        ]),
    ]:
        db.make_program(name, 'Biological Sciences')
        db.make_conflict(name, 'required courses', 1, 'boost', genomics_core + emphasis)

    # Biology emphasis worksheets share core introductory courses but differ
    # in science requirements and alternatives. Senior Seminar is deferred
    # pending a college-wide decision about its de-facto prerequisites.
    # https://catalog.utahtech.edu/programs/biological-sciences/biological-sciences-bs/
    # https://catalog.utahtech.edu/programs/biological-sciences/biology-biomedical-bs/
    # https://catalog.utahtech.edu/programs/biological-sciences/biology-natural-sciences-bs/
    biology_core = [
        'BIOL 1610', 'BIOL 1615', 'BIOL 1620', 'BIOL 1625',
        'BIOL 3010', 'BIOL 3030', 'BIOL 3040',
        'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225',
    ]
    physics_series = [
        ['PHYS 2010', 'PHYS 2015', 'PHYS 2020', 'PHYS 2025'],
        ['PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225'],
    ]
    microbes = [['BIOL 3450', 'BIOL 3455'], ['BIOL 3550', 'BIOL 3555']]
    labs = [[course] for course in ['BTEC 2010', 'BTEC 2020', 'BTEC 2030', 'BTEC 2050', 'BIOL 2300']]
    organic = ['CHEM 2310', 'CHEM 2315', 'CHEM 2320', 'CHEM 2325']
    for name, extra in [
        ('Biology - Biological Sciences', ['BIOL 3045', 'BIOL 3150', 'BIOL 3155'] + organic),
        ('Biology - Biomedical Sciences', [
            'BIOL 2320', 'BIOL 2325', 'BIOL 3420', 'BIOL 3150', 'BIOL 3155',
            'CHEM 3510', 'CHEM 3515',
        ] + organic),
    ]:
        db.make_program(name, 'Biological Sciences')
        db.make_conflict(name, 'required courses', 1, 'boost', biology_core + extra)
        add_choice(db, name, 'physics series', physics_series)
        add_choice(db, name, 'microbiology or cell biology', microbes, 'physics series')
        add_choice(db, name, 'technical laboratory', labs, 'microbiology or cell biology')
    add_choice(db, 'Biology - Biological Sciences', 'physiology', [
        ['BIOL 3420'], ['BIOL 4500', 'BIOL 4505'], ['BIOL 4600', 'BIOL 4605'],
    ], 'technical laboratory')
    # Biomedical ethics can alternatively be fulfilled by non-college PHIL
    # courses; BIOL 3100 therefore is a choice, not a universal requirement.
    db.make_conflict('Biology - Biomedical Sciences', 'biology ethics option', 3, 'boost',
                     ['BIOL 3100'], include=['technical laboratory'])

    name = 'Biology - Natural Sciences'
    db.make_program(name, 'Biological Sciences')
    db.make_conflict(name, 'required courses', 1, 'boost', biology_core + [
        'BIOL 2400', 'BIOL 2405', 'BIOL 3045', 'ENVS 1210', 'ENVS 1215',
        'GEO 1110', 'GEO 1115', 'GEOG 3600', 'GEOG 3605',
    ])
    add_choice(db, name, 'science communication', [['BIOL 3110'], ['BIOL 3120']])
    add_choice(db, name, 'mathematics', [['MATH 1040'], ['MATH 1050']], 'science communication')
    add_choice(db, name, 'physics', [['PHYS 1010', 'PHYS 1015'], ['PHYS 2010', 'PHYS 2015']], 'mathematics')
    db.make_conflict(name, 'organismal biology options', 3, 'boost', [
        'BIOL 3140', 'BIOL 3145', 'BIOL 3200', 'BIOL 4200', 'BIOL 4205',
        'BIOL 4260', 'BIOL 4265', 'BIOL 4270', 'BIOL 4275', 'BIOL 4280',
        'BIOL 4350', 'BIOL 4355', 'BIOL 4380', 'BIOL 4385', 'BIOL 4411',
        'BIOL 4415', 'BIOL 4440', 'BIOL 4500', 'BIOL 4505', 'BIOL 4600', 'BIOL 4605',
    ], include=['physics'])

    # Biology Education worksheet: the secondary-education courses are
    # outside the current college course catalog.
    # https://catalog.utahtech.edu/programs/biological-sciences/biology-secondary-education-bs/
    name = 'Biology - Education'
    db.make_program(name, 'Biological Sciences')
    db.make_conflict(name, 'required courses', 1, 'boost', biology_core + [
        'BIOL 3045', 'BIOL 2400', 'BIOL 2405', 'MATH 1050',
        'SCI 2600', 'SCI 4700', 'SCI 4750',
    ])
    add_choice(db, name, 'anatomy', [
        ['BIOL 2320', 'BIOL 2325'], ['BIOL 3140', 'BIOL 3145'],
    ])
    add_choice(db, name, 'physiology', [
        ['BIOL 2420', 'BIOL 2425'], ['BIOL 4500', 'BIOL 4505'],
    ], 'anatomy')
    add_choice(db, name, 'cell biology', [
        ['BIOL 2060', 'BIOL 2065'], ['BIOL 3450', 'BIOL 3455'], ['BIOL 3550', 'BIOL 3555'],
    ], 'physiology')
    add_choice(db, name, 'zoology', [
        ['BIOL 3200', 'BIOL 3205'], ['BIOL 4260', 'BIOL 4265'],
        ['BIOL 4270', 'BIOL 4275'], ['BIOL 4350', 'BIOL 4355'],
        ['BIOL 4380', 'BIOL 4385'], ['BIOL 4411', 'BIOL 4415'], ['BIOL 4440'],
    ], 'cell biology')
