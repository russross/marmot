from queries import DB


def cancel_alternatives(db: DB, program: str, label: str, left: list[str], right: list[str]) -> None:
    # Cancel only pairs from different alternatives, retaining conflicts
    # between courses that students take together within one alternative.
    for a in left:
        for b in right:
            db.make_conflict(program, f'{label}: {a} or {b}', None, 'reduce', [a, b])


def build_programs(db: DB) -> None:
    # 2026–2027 worksheets:
    # https://catalog.utahtech.edu/programs/chemistry-biochemistry/chemistry-bs/
    # https://catalog.utahtech.edu/programs/chemistry-biochemistry/biochemistry-molecular-biology-bs/
    chemistry_core = [
        'CHEM 1210', 'CHEM 1215', 'CHEM 1220', 'CHEM 1225',
        'CHEM 2310', 'CHEM 2315', 'CHEM 2320', 'CHEM 2325',
        'CHEM 2600', 'CHEM 2990R', 'CHEM 3000', 'CHEM 3005',
        'CHEM 3100', 'CHEM 3300', 'CHEM 3510', 'CHEM 3515', 'CHEM 4910',
        'BIOL 1610', 'BIOL 1615', 'MATH 1210', 'MATH 1220',
    ]
    physical_chemistry = ['CHEM 3060', 'CHEM 3065', 'CHEM 3070', 'CHEM 3075']
    db.make_program('Chemistry', 'Chemistry and Biochemistry')
    db.make_conflict('Chemistry', 'required courses', 1, 'boost', chemistry_core + physical_chemistry + [
        'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225', 'CHEM 4100',
    ])
    # Exactly one additional mathematics course is required.
    math_options = ['MATH 2210', 'MATH 2250', 'MATH 2270', 'MATH 2280', 'MATH 3060']
    db.make_conflict('Chemistry', 'mathematics choice', 1, 'boost', math_options,
                     include=['required courses'])
    db.make_conflict('Chemistry', 'only one mathematics choice', None, 'reduce', math_options)
    db.make_conflict('Chemistry', 'chemistry electives', 3, 'boost', [
        'CHEM 3520', 'CHEM 3525', 'CHEM 4200', 'CHEM 4310',
        'CHEM 4510', 'CHEM 4610', 'CHEM 4900', 'CHEM 4905',
    ], include=['required courses'])

    # Biochemistry chooses one physical chemistry pair and one complete
    # physics sequence. Research and internship choices are individualized.
    program = 'Biochemistry - Molecular Biology'
    db.make_program(program, 'Chemistry and Biochemistry')
    db.make_conflict(program, 'required courses', 1, 'boost', chemistry_core + physical_chemistry + [
        'CHEM 3520', 'CHEM 3525', 'BIOL 3030', 'BIOL 3550', 'BIOL 3555',
        'BIOL 4300', 'BIOL 4305', 'PHYS 2010', 'PHYS 2015', 'PHYS 2020',
        'PHYS 2025', 'PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225',
    ])
    cancel_alternatives(db, program, 'physical chemistry alternatives',
                        ['CHEM 3060', 'CHEM 3065'], ['CHEM 3070', 'CHEM 3075'])
    cancel_alternatives(db, program, 'physics sequence alternatives',
                        ['PHYS 2010', 'PHYS 2015', 'PHYS 2020', 'PHYS 2025'],
                        ['PHYS 2210', 'PHYS 2215', 'PHYS 2220', 'PHYS 2225'])
    # Three electives from a broad chemistry/biology pool are opportunistic
    # choices and contribute no conflict pressure.
