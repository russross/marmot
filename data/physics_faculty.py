import csv
from pathlib import Path

from posted_schedule import PostedMeeting
from queries import DB, TimeInterval
from rooms import science_lecture_tag


def build_faculty(db: DB) -> None:
    # Physics is scheduled by Engineering. The posted four-credit lectures
    # use MTWF or MWRF; infer both as possible daytime patterns. The posted
    # meeting remains available even when it lies outside core hours.
    # Retain the posted room and lab times pending information about equipment
    # and interchangeability. Online and TBA offerings remain unscheduled.
    path = Path(__file__).parent / 'postedschedule' / 'phys.csv'
    with path.open(encoding='utf-8-sig', newline='') as file:
        rows = [row for row in csv.DictReader(file) if int(row['Cap']) > 0]
    availability = [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')]
    names = {row['Instructor'].removesuffix(' (P)').strip() for row in rows} - {'TBA', ''}
    for name in sorted(names):
        db.make_faculty(name, 'Engineering', availability)

    for row in rows:
        course = row['Course'].removeprefix('="').removesuffix('"')
        number = row['Section'].removeprefix('="').removesuffix('"')
        section = f"{row['Subject']} {course}-{number}"
        name = row['Instructor'].removesuffix(' (P)').strip()
        tags: list[str] = []
        if row['Days'] and row['Time'] != 'TBA':
            meeting = PostedMeeting(row['Days'], row['Time'], row['Location'], row['Instructor'])
            slot = meeting.slot_name()
            is_lab = 'lab' in row['Title'].lower()
            tags = meeting.lab_time_tags() if is_lab else [slot]
            tags.append((science_lecture_tag(meeting.location) if not is_lab else None) or meeting.location)
            if float(row['Credits']) == 4 and meeting.days in ('MTWF', 'MWRF') and slot.endswith('+50'):
                tags.append('4 credit four-day daytime')
            elif slot == 'MW1500+75' and float(row['Credits']) == 3:
                tags.append('3 credit bell schedule')
        db.make_section_with_no_faculty(section, *tags, credit_hours=float(row['Credits']))
        if name and name != 'TBA':
            db.assign_faculty_to_existing_section(name, section)
