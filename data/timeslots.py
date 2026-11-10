import queries


def build_time_slots(db: queries.DB) -> None:
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

    # The posted schedule suggests two 50-minute meetings for some two-credit
    # lectures. Their starts follow the established three-credit bell times.
    for days in ('MW', 'TR'):
        for start in ('0900', '1000', '1030', '1100', '1200', '1330', '1500'):
            db.make_time_slot(f'{days}{start}+50', ['2 credit 2×50 bell schedule'])

    # Four-credit lectures commonly use four 50-minute meetings or two
    # 100-minute meetings. Keep observed 13:00 starts and add 13:30 bell starts.
    for start in ('0900', '1000', '1100', '1200'):
        db.make_time_slot(f'MTWR{start}+50', ['4 credit bell schedule', '4 credit four-day daytime'])
    for days in ('MTWF', 'MWRF', 'MTRF', 'TWRF'):
        for start in ('0900', '1000', '1100'):
            db.make_time_slot(f'{days}{start}+50', ['4 credit bell schedule', '4 credit four-day daytime'])
    for days in ('MW', 'TR'):
        for start in ('1300', '1330', '1500'):
            db.make_time_slot(f'{days}{start}+100', ['4 credit bell schedule'])

    # The posted five-credit mathematics lecture meets five days per week.
    for start in ('0900', '1000', '1100'):
        db.make_time_slot(f'MTWRF{start}+50', ['5 credit 5×50 bell schedule'])

    # SI is led separately from the instructor. Weekday daytime 50-minute
    # sessions may move across the week, retaining any explicit posted time.
    for day in 'MTWRF':
        starts = ('0900', '1000', '1100', '1200') if day == 'F' else (
            '0900', '1000', '1030', '1100', '1200', '1330', '1500',
        )
        for start in starts:
            db.make_time_slot(f'{day}{start}+50', ['SI 50 minute daytime'])

    # The earlier lab grids used same-day blocks staggered at two- or
    # three-hour intervals. Keep alternatives inside daytime teaching hours.
    for day in ('M', 'T', 'W', 'R', 'MW', 'TR'):
        durations = ((110, range(9, 15)),) if len(day) > 1 else ((110, range(9, 15)), (170, range(9, 14)))
        for duration, starts in durations:
            for hour in starts:
                phase = hour % (2 if duration == 110 else 3)
                db.make_time_slot(f'{day}{hour:02}00+{duration}', [f'lab {duration} {day} phase {phase}'])

