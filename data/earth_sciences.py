from queries import DB


def build_programs(db: DB) -> None:
    # 2026–2027 earth, energy, and environmental science worksheets:
    # https://catalog.utahtech.edu/programs/earth-environmental-sciences/earth-energy-environmental-science-envsci-bs/
    # https://catalog.utahtech.edu/programs/earth-environmental-sciences/earth-energy-environmental-science-geoscience-bs/
    # Broad electives, travel choices, individualized research, and ENVS 4910
    # Senior Seminar are deferred. ENGL 2201 is outside the college catalog.
    department = 'Earth, Energy and Environmental Science'
    core = [
        'ENVS 1210', 'ENVS 1215', 'ENVS 2210', 'ENER 3310', 'ENER 4310',
        'GEO 1110', 'GEO 1115', 'GEO 2050', 'GEO 3400', 'GEOG 3600',
        'GEOG 3605', 'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225',
        'BIOL 1610', 'BIOL 1615',
    ]
    name = 'Earth, Energy and Environmental Science - Environmental Science'
    db.make_program(name, department)
    db.make_conflict(name, 'required courses', 1, 'boost', core + [
        'ENVS 3110', 'BIOL 3110', 'GEOG 3410', 'MATH 1040', 'MATH 1050',
        'ENVS 2700R', 'ENVS 2990R', 'ENVS 3510', 'ENVS 4080',
    ])
    db.make_conflict(name, 'scientific writing alternatives', None, 'reduce', ['ENVS 3110', 'BIOL 3110'])

    name = 'Earth, Energy and Environmental Science - Geoscience'
    db.make_program(name, department)
    db.make_conflict(name, 'required courses', 1, 'boost', core + [
        'BIOL 3110', 'GEO 1220', 'GEO 1225', 'GEO 2700R', 'GEO 2990R', 'GEO 3700',
    ])
    # The worksheet requires 22 credits from a broad elective list. The
    # in-person discipline courses get a lower priority; travel and independent
    # study options are excluded until their scheduling is understood. The
    # worksheet's ENVS 4085 has no course description in the current catalog.
    db.make_conflict(name, 'geoscience electives', 4, 'boost', [
        'GEO 3060', 'GEO 3180', 'GEO 3200', 'GEO 3500', 'GEO 3550',
        'GEO 3600', 'GEO 3710', 'GEO 4600', 'ENVS 3280', 'ENVS 3410',
        'ENVS 3510', 'ENVS 4080', 'GEOG 4140', 'GEOG 3410',
        'GEOG 4180', 'MATH 1210',
    ], include=['required courses'])

    # https://catalog.utahtech.edu/programs/earth-environmental-sciences/geological-sciences-bs/
    name = 'Geological Sciences'
    db.make_program(name, department)
    db.make_conflict(name, 'required courses', 1, 'boost', [
        'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225',
        'GEO 1110', 'GEO 1115', 'GEO 1220', 'GEO 1225', 'GEO 2700R',
        'GEO 2990R', 'GEO 3110', 'GEO 3200', 'GEO 3500', 'GEO 3550',
        'GEO 3600', 'GEO 3700', 'GEO 4600', 'GEOG 3600', 'GEOG 3605',
        'MATH 1210', 'PHYS 2010', 'PHYS 2015', 'PHYS 2020', 'PHYS 2025',
        'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225',
    ])
    for a, b in [('PHYS 2010', 'PHYS 2210'), ('PHYS 2015', 'PHYS 2215'),
                 ('PHYS 2020', 'PHYS 2220'), ('PHYS 2025', 'PHYS 2225')]:
        db.make_conflict(name, f'physics alternatives: {a} or {b}', None, 'reduce', [a, b])
    db.make_conflict(name, 'geology electives', 3, 'boost', [
        'GEO 3000', 'GEO 3060', 'GEO 3180', 'GEO 3710', 'GEO 4000R',
        'GEOG 4140', 'GEOG 4180', 'ENVS 3210',
    ], include=['required courses'])
