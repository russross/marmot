import csv
from collections import defaultdict
from pathlib import Path

from posted_schedule import PostedMeeting
from queries import DB, TimeInterval


def build_faculty(db: DB) -> None:
    path = Path(__file__).parent / 'postedschedule' / 'btec.csv'
    with path.open(encoding='utf-8-sig', newline='') as file:
        rows = [row for row in csv.DictReader(file) if int(row['Cap']) > 0]
    groups: dict[str, list[dict[str, str]]] = defaultdict(list)
    for row in rows:
        groups[row['CRN']].append(row)
    availability = [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')]
    for name in sorted({row['Instructor'].removesuffix(' (P)').strip() for row in rows} - {'', 'TBA'}):
        if db.db.execute('SELECT 1 FROM faculty WHERE faculty = ?', (name,)).fetchone() is None:
            db.make_faculty(name, 'Biological Sciences', availability)

    split_courses: set[str] = set()
    for crn, records in groups.items():
        first = records[0]
        course = first['Course'].removeprefix('="').removesuffix('"')
        number = first['Section'].removeprefix('="').removesuffix('"')
        section = f'BTEC {course}-{number}'
        name = first['Instructor'].removesuffix(' (P)').strip()
        if len(records) > 1:
            if (course, number) not in {('2050', '01'), ('2050', '02'), ('4050', '01A'), ('4805', '01')} or len(records) != 2:
                raise ValueError(f'BTEC CRN {crn}: unsupported multiple meetings')
            split_courses.add(course)
        for index, row in enumerate(records):
            current = section + ('extra' if index else '')
            meeting = PostedMeeting(row['Days'], row['Time'], row['Location'], row['Instructor'])
            tags: list[str] = []
            if meeting.days and meeting.time != 'TBA':
                slot = meeting.slot_name()
                # BTEC 1010 is a standard lecture; the remaining hands-on
                # meetings retain their posted times and specialized rooms.
                tags = ['3 credit bell schedule' if course == '1010' and slot == 'TR1200+75' else slot,
                        meeting.location]

            # Both 2050 sections share the Tuesday 11:00 meeting. Reserve
            # the longer posted 11:00–12:50 interval for the shared meeting.
            if (course, number, index) == ('2050', '02', 1):
                db.make_section_with_no_faculty(current, credit_hours=float(row['Credits']))
                continue
            db.make_section_with_no_faculty(current, *tags, credit_hours=float(row['Credits']))
            if name not in ('', 'TBA'):
                db.assign_faculty_to_existing_section(name, current)

    # The two 2050 aliases represent one shared instructor and room meeting.
    if {'21796', '21838'} <= groups.keys():
        db.add_cross_listing('BTEC 2050-01extra', ['BTEC 2050-02extra'])
    for course in split_courses:
        # Count posted CRNs, not the extra sections created for meetings.
        count = sum(1 for records in groups.values() if records[0]['Course'] == f'="{course}"')
        db.add_multiple_section_override(f'BTEC {course}', count)
