import queries
from queries import *


def build_pre(db: DB) -> None:
    print('building shared buildings and classrooms')
    db.make_building('Smith')
    db.make_room('Smith 107', 32, ['flex'])
    db.make_room('Smith 108', 32, ['flex'])
    db.make_room('Smith 109', 32, ['flex'])
    db.make_room('Smith 112', 24, ['macs'])
    db.make_room('Smith 113', 24, ['pcs'])
    db.make_room('Smith 116', 38, ['stadium'])
    db.make_room('Smith 117', 38, ['stadium'])

    db.make_building('Snow')
    db.make_room('Snow 003', 40, ['snow math rooms'])
    db.make_room('Snow 112', 40, ['snow math rooms'])
    db.make_room('Snow 124', 40, ['snow math rooms'])
    db.make_room('Snow 125', 40, ['snow math rooms'])
    db.make_room('Snow 144', 40, ['snow math rooms'])
    db.make_room('Snow 145', 40, ['snow math rooms'])
    db.make_room('Snow 147', 40, ['snow math rooms'])
    db.make_room('Snow 150', 40, ['snow math rooms'])
    db.make_room('Snow 151', 40, ['snow math rooms'])

    print('building shared time slots')

    db.make_time_slot('MWF0800+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MWF0900+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MWF1000+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MWF1100+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MW1200+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])
    db.make_time_slot('MW1330+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])
    db.make_time_slot('MW1500+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])
    db.make_time_slot('MW1630+75', [])

    db.make_time_slot('TR0730+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
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

    db.make_time_slot('MTWR0800+50', ['4 credit bell schedule', '4×50 bell schedule', 'MTWR 4×50 bell schedule'])
    db.make_time_slot('MTWR0900+50', ['4 credit bell schedule', '4×50 bell schedule', 'MTWR 4×50 bell schedule'])
    db.make_time_slot('MTWR1000+50', ['4 credit bell schedule', '4×50 bell schedule', 'MTWR 4×50 bell schedule'])
    db.make_time_slot('MTWR1100+50', ['4 credit bell schedule', '4×50 bell schedule', 'MTWR 4×50 bell schedule'])
    db.make_time_slot('MTWR1200+50', ['4 credit bell schedule', '4×50 bell schedule', 'MTWR 4×50 bell schedule'])

    db.make_time_slot('MW1300+100', ['4 credit bell schedule', '2×100 bell schedule', 'MW 2×100 bell schedule'])
    db.make_time_slot('MW1500+100', ['4 credit bell schedule', '2×100 bell schedule', 'MW 2×100 bell schedule'])
    db.make_time_slot('TR1300+100', ['4 credit bell schedule', '2×100 bell schedule', 'TR 2×100 bell schedule'])
    db.make_time_slot('TR1500+100', ['4 credit bell schedule', '2×100 bell schedule', 'TR 2×100 bell schedule'])

    db.make_time_slot('MW1630+100', ['4 credit late afternoon'])
    db.make_time_slot('TR1630+100', ['4 credit late afternoon'])
    db.make_time_slot('MW1800+100', ['4 credit evening'])

    db.make_time_slot('W1200+50', [])
    db.make_time_slot('MTWRF0800+50', [])
