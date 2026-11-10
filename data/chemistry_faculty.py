import csv
from collections import defaultdict
from pathlib import Path

from posted_schedule import PostedMeeting, room_name, supplemental_offerings
from queries import DB, TimeInterval
from rooms import science_lecture_tag


def build_faculty(db: DB) -> None:
    path = Path(__file__).parent / 'postedschedule' / 'chem.csv'
    with path.open(encoding='utf-8-sig', newline='') as file:
        rows = [row for row in csv.DictReader(file) if int(row['Cap']) > 0]
    si = {offering.crn: offering for offering in supplemental_offerings(path.parent)}
    groups: dict[str, list[dict[str, str]]] = defaultdict(list)
    for row in rows:
        groups[row['CRN']].append(row)

    availability = [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')]
    for name in sorted({row['Instructor'].removesuffix(' (P)').strip() for row in rows} - {'', 'TBA'}):
        if db.db.execute('SELECT 1 FROM faculty WHERE faculty = ?', (name,)).fetchone() is None:
            db.make_faculty(name, 'Chemistry and Biochemistry', availability)

    standard_three = {
        'MWF0900+50', 'MWF1000+50', 'MWF1100+50',
        'MW1200+75', 'MW1330+75', 'MW1500+75',
        'TR0900+75', 'TR1030+75', 'TR1200+75', 'TR1330+75', 'TR1500+75',
    }
    for crn, records in groups.items():
        row = records[0]
        course = row['Course'].removeprefix('="').removesuffix('"')
        number = row['Section'].removeprefix('="').removesuffix('"')
        section = f'CHEM {course}-{number}'
        name = row['Instructor'].removesuffix(' (P)').strip()
        split_lab = (course, number) == ('4510', '01')
        if len(records) > 1 and crn not in si and not (split_lab and len(records) == 2):
            raise ValueError(f'CHEM CRN {crn}: unsupported multiple meetings')
        primary = si[crn].primary if crn in si else PostedMeeting(
            row['Days'], row['Time'], row['Location'], row['Instructor'],
        )
        meetings = [primary]
        if split_lab:
            meetings.append(PostedMeeting(
                records[1]['Days'], records[1]['Time'], records[1]['Location'], records[1]['Instructor'],
            ))
        for index, meeting in enumerate(meetings):
            current = section + ('extra' if index else '')
            # CHEM 1210-02 and -04 share the same posted lecture in SET 201;
            # the latter's instructor is TBA and each has a different SI.
            if section == 'CHEM 1210-04':
                db.make_section_with_no_faculty(current, credit_hours=float(row['Credits']))
                db.add_cross_listing('CHEM 1210-02', [current])
                continue
            tags: list[str] = []
            # The posted independent research placeholders overlap their own
            # instructors' classes; record workload without scheduling them.
            if meeting.days and meeting.time != 'TBA' and course != '4800R':
                slot = meeting.slot_name()
                is_lab = split_lab or 'lab' in row['Title'].lower()
                tags = meeting.lab_time_tags() if is_lab and not split_lab else [slot]
                room = room_name(meeting.location)
                tags.append((science_lecture_tag(room) if not is_lab else None) or room)
                # Four-day lectures may move to other standard four-credit
                # patterns. Keep the exact posted slot for out-of-hours cases.
                if not split_lab and 'lab' not in row['Title'].lower():
                    if float(row['Credits']) == 4 and meeting.days == 'MTWR' and slot.endswith('+50'):
                        tags.append('4 credit bell schedule')
                    elif float(row['Credits']) == 3 and slot in standard_three:
                        tags.append('3 credit bell schedule')
            db.make_section_with_no_faculty(current, *tags, credit_hours=float(row['Credits']))
            if name not in ('', 'TBA'):
                db.assign_faculty_to_existing_section(name, current)

        # SI has its own room and time, and no faculty. The CHEM 3300
        # Thursday meeting is a documented inference despite no SI attribute.
        if crn in si:
            extra = si[crn].supplemental
            si_section = section + 'si'
            room = room_name(extra.location)
            db.make_section_with_no_faculty(
                si_section, *extra.si_time_tags(), science_lecture_tag(room) or room, credit_hours=float(row['Credits']),
            )
            db.add_supplemental_instruction(section, si_section)
        if split_lab:
            # Both instructor-led meetings are one student offering.
            db.add_multiple_section_override('CHEM 4510', 1)
