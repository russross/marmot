import queries
from queries import *


def build_pre(db: DB) -> None:
    pass


def build_post(db: DB) -> None:
    print('building math conflicts')

    #
    # Mathematics degree
    #
    db.make_program('Mathematics', 'Mathematics')
    db.make_conflict('Mathematics', 'core requirements', 1, 'boost', [
        # core requirements
        'CS 1400', 'CS 2100',
        'MATH 1210', 'MATH 1220',
        'MATH 2200', 'MATH 2210', 'MATH 2270', 'MATH 2280',
        'MATH 3120', 'MATH 3200', 'MATH 3400', 'MATH 3500',
        'MATH 4000', 'MATH 4200', 'MATH 4100', 'MATH 4900',
        # note: one discrete math class: CS 2100 or MATH 2200

    ])
    db.make_conflict('Mathematics', 'math electives', 3, 'boost', [
        # core requirements
        'CS 1400', 'CS 2100',
        'MATH 1210', 'MATH 1220',
        'MATH 2200', 'MATH 2210', 'MATH 2270', 'MATH 2280',
        'MATH 3120', 'MATH 3200', 'MATH 3400', 'MATH 3500',
        'MATH 4000', 'MATH 4200', 'MATH 4100', 'MATH 4900',
        # note: one discrete math class: CS 2100 or MATH 2200

        # math electives
        'MATH 3000', 'MATH 3100', 'MATH 3150', 'MATH 3210', 'MATH 3450', 'MATH 3700', 'MATH 3900',
        'MATH 4010', 'MATH 4250', 'MATH 4800', 'MATH 4890R',
    ])
    db.make_conflict('Mathematics', 'only need one discrete math class', None, 'reduce',
        ['MATH 2200', 'CS 2100'])

    #
    # Mathematics Education degree
    #
    db.make_program('Mathematics Education', 'Mathematics')
    db.make_conflict('Mathematics Education', 'core requirements', 1, 'boost', [
        # core requirements
        'MATH 1210', 'MATH 1220',
        'MATH 2200', 'MATH 2210', 'MATH 2270', 'MATH 2280',
        'MATH 3000', 'MATH 3010', 'MATH 3020', 'MATH 3100', 'MATH 3120', 'MATH 3200', 'MATH 3400',
        'MATH 4000',
        'CS 1400',
    ])

    #
    # Applied and Computational Mathematics degree
    #
    db.make_program('Applied and Computational Mathematics', 'Mathematics')
    db.make_conflict('Applied and Computational Mathematics', 'core requirements', 1, 'boost', [
        # core requirements
        'CS 1400', 'CS 1410',
        'CS 2100',
        'MATH 1210', 'MATH 1220',
        'MATH 2200', 'MATH 2210', 'MATH 2270', 'MATH 2280',
        'MATH 3400', 'MATH 3500', 'MATH 3700',
        'MATH 4800', 'MATH 4890R', 'MATH 4900',
        # note: one discrete math class: CS 2100 or MATH 2200
    ])
    db.make_conflict('Applied and Computational Mathematics', 'actuarial science emphasis', 3, 'boost', [
        # core requirements
        'CS 1400', 'CS 1410',
        'CS 2100',
        'MATH 1210', 'MATH 1220',
        'MATH 2200', 'MATH 2210', 'MATH 2270', 'MATH 2280',
        'MATH 3400', 'MATH 3500', 'MATH 3700',
        'MATH 4800', 'MATH 4890R', 'MATH 4900',
        # note: one discrete math class: CS 2100 or MATH 2200

        # actuarial science emphasis requirements
        'CS 2420',
        'MATH 3410', 'MATH 3450',
        'MATH 4400', 'MATH 4410',
    ])
    db.make_conflict('Applied and Computational Mathematics', 'actuarial science emphasis electives', 7, 'boost', [
        # core requirements
        'CS 1400', 'CS 1410',
        'CS 2100',
        'MATH 1210', 'MATH 1220',
        'MATH 2200', 'MATH 2210', 'MATH 2270', 'MATH 2280',
        'MATH 3400', 'MATH 3500', 'MATH 3700',
        'MATH 4800', 'MATH 4890R', 'MATH 4900',
        # note: one discrete math class: CS 2100 or MATH 2200

        # actuarial science emphasis requirements
        'CS 2420',
        'MATH 3410', 'MATH 3450',
        'MATH 4400', 'MATH 4410',

        # actuarial science emphasis electives
        'MATH 3050', 'MATH 3100', 'MATH 3120', 'MATH 3150', 'MATH 3200', 'MATH 3900',
        'MATH 4000', 'MATH 4010', 'MATH 4100', 'MATH 4200', 'MATH 4330',
    ])
    db.make_conflict('Applied and Computational Mathematics', 'data analytics emphasis', 3, 'boost', [
        # core requirements
        'CS 1400', 'CS 1410',
        'CS 2100',
        'MATH 1210', 'MATH 1220',
        'MATH 2200', 'MATH 2210', 'MATH 2270', 'MATH 2280',
        'MATH 3400', 'MATH 3500', 'MATH 3700',
        'MATH 4800', 'MATH 4890R', 'MATH 4900',
        # note: one discrete math class: CS 2100 or MATH 2200

        # data analytics emphasis requirements
        'IT 1100',
        'IT 2300', 'IT 2400',
        'IT 4310',
        'MATH 2050',
        'MATH 3050', 'MATH 3450',
    ])
    db.make_conflict('Applied and Computational Mathematics', 'data analytics emphasis electives', 7, 'boost', [
        # core requirements
        'CS 1400', 'CS 1410',
        'CS 2100',
        'MATH 1210', 'MATH 1220',
        'MATH 2200', 'MATH 2210', 'MATH 2270', 'MATH 2280',
        'MATH 3400', 'MATH 3500', 'MATH 3700',
        'MATH 4800', 'MATH 4890R', 'MATH 4900',
        # note: one discrete math class: CS 2100 or MATH 2200

        # data analytics emphasis requirements
        'IT 1100',
        'IT 2300', 'IT 2400',
        'IT 4310',
        'MATH 2050',
        'MATH 3050', 'MATH 3450',

        # data analytics emphasis electives
        'CS 3005', 
        'IT 4510',
        'MATH 3100', 'MATH 3120', 'MATH 3150', 'MATH 3200', 'MATH 3900',
        'MATH 4000', 'MATH 4010', 'MATH 4100', 'MATH 4200', 'MATH 4330',
    ])
    db.make_conflict('Applied and Computational Mathematics', 'scientific computing emphasis', 3, 'boost', [
        # core requirements
        'CS 1400', 'CS 1410',
        'CS 2100',
        'MATH 1210', 'MATH 1220',
        'MATH 2200', 'MATH 2210', 'MATH 2270', 'MATH 2280',
        'MATH 3400', 'MATH 3500', 'MATH 3700',
        'MATH 4800', 'MATH 4890R', 'MATH 4900',
        # note: one discrete math class: CS 2100 or MATH 2200

        # scientific computing emphasis requirements
        'CS 2420',
        'CS 3005',
        'MATH 2050',
        'MATH 3150',
        'MATH 4250', 'MATH 4550',
    ])
    db.make_conflict('Applied and Computational Mathematics', 'scientific computing emphasis electives', 7, 'boost', [
        # core requirements
        'CS 1400', 'CS 1410',
        'CS 2100',
        'MATH 1210', 'MATH 1220',
        'MATH 2200', 'MATH 2210', 'MATH 2270', 'MATH 2280',
        'MATH 3400', 'MATH 3500', 'MATH 3700',
        'MATH 4800', 'MATH 4890R', 'MATH 4900',
        # note: one discrete math class: CS 2100 or MATH 2200

        # scientific computing emphasis requirements
        'CS 2420',
        'CS 3005',
        'MATH 2050',
        'MATH 3150',
        'MATH 4250', 'MATH 4550',

        # scientific computing emphasis electives
        'MATH 3050', 'MATH 3100', 'MATH 3120', 'MATH 3200', 'MATH 3450', 'MATH 3900', 'MATH 3905',
        'MATH 4000', 'MATH 4005', 'MATH 4010', 'MATH 4100', 'MATH 4200', 'MATH 4330',
    ])
    db.make_conflict('Applied and Computational Mathematics', 'only need one discrete math class', None, 'reduce',
        ['MATH 2200', 'CS 2100'])
