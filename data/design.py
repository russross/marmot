from queries import DB


def build_programs(db: DB) -> None:
    # 2026–2027 worksheets share the design core and define separate emphasis
    # requirements. ART, ARTH, ENGL, and MKTG courses are outside the
    # college course catalog in this dry run.
    # https://catalog.utahtech.edu/programs/design/design-digital-design-bs/
    # https://catalog.utahtech.edu/programs/design/design-graphic-design-bs/
    # https://catalog.utahtech.edu/programs/design/design-interaction-design-bs/
    core = [
        'DES 1100', 'DES 1101', 'DES 1102', 'DES 1103', 'DES 1111',
        'DES 1200', 'DES 1300', 'DES 1600', 'DES 2100', 'DES 2200',
        'DES 2300', 'DES 2710', 'DES 2780', 'DES 3000', 'DES 3610',
        'DES 4600', 'DES 4610', 'IXD 1200', 'IXD 1300', 'SET 1000',
    ]
    elective_pool = [
        'DES 1610', 'DES 3300', 'DES 3400', 'DES 3600', 'DES 3710',
        'DES 3780', 'DES 3800', 'DES 4300', 'DES 4650', 'DES 4750',
        'DES 4990', 'IXD 2000', 'IXD 3500', 'IXD 4100', 'IXD 4200',
        'SE 3500', 'SE 3550',
    ]
    emphases = [
        ('Design - Digital Design', ['DES 3300', 'DES 3400', 'DES 3600', 'IXD 3500']),
        ('Design - Graphic Design', ['DES 3300', 'DES 3780', 'DES 3800', 'DES 4650', 'DES 4750']),
        ('Design - Interaction Design', ['IXD 2000', 'IXD 3500', 'IXD 4100', 'IXD 4200']),
    ]
    for program, emphasis in emphases:
        db.make_program(program, 'Design')
        db.make_conflict(program, 'required courses', 1, 'boost', core + emphasis)
        # Students take DES 1100, the three one-credit courses, or a skills
        # exam. Cancel conflicts only across these alternative routes.
        for single in ('DES 1100', 'DES 1111'):
            for part in ('DES 1101', 'DES 1102', 'DES 1103'):
                db.make_conflict(program, f'intro alternative: {single} or {part}', None, 'reduce', [single, part])
        db.make_conflict(program, 'intro single-course alternatives', None, 'reduce', ['DES 1100', 'DES 1111'])
        # Fifteen elective credits allow substantial choice. Research and
        # internship are individualized and are omitted from this group.
        db.make_conflict(program, 'design electives', 4, 'boost',
                         [course for course in elective_pool if course not in emphasis],
                         include=['required courses'])

    # https://catalog.utahtech.edu/programs/design/user-experience-design-muxd/
    db.make_program('User Experience Design', 'Design')
    db.make_conflict('User Experience Design', 'required courses', 1, 'boost', [
        'UXD 6100', 'UXD 6140', 'UXD 6180', 'UXD 6200', 'UXD 6240',
        'UXD 6280', 'UXD 6300', 'UXD 6340', 'UXD 6600', 'UXD 6650',
    ])
