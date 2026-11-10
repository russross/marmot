import csv
from pathlib import Path

from posted_schedule import PostedMeeting, supplemental_offerings
from queries import DB, TimeInterval


def build_faculty(db: DB) -> None:
    # The downloaded CSV is the source for names, credits, and posted meetings.
    # Generalize only ordinary 3- and 4-credit daytime patterns; preserve
    # unusual and outside-hours meetings as explicit section choices.
    path = Path(__file__).parent / 'postedschedule' / 'math.csv'
    with path.open(encoding='utf-8-sig', newline='') as file:
        rows = [row for row in csv.DictReader(file) if int(row['Cap']) > 0]
    si = {offering.crn: offering for offering in supplemental_offerings(path.parent)}
    # Several posted four-credit meetings end at 16:40. Extend the common
    # mathematics weekday availability by ten minutes to admit that pattern.
    default_availability = [TimeInterval('MTWR', '0900', '1640'), TimeInterval('F', '0900', '1200')]

    faculty = {row['Instructor'].removesuffix(' (P)').strip() for row in rows} - {'TBA', ''}
    for name in sorted(faculty):
        db.make_faculty(name, 'Math', default_availability)

    # The posting repeats SI rows under one CRN. The classified primary is
    # processed once, and the supplemental meeting is created separately.
    seen: set[str] = set()
    for row in rows:
        crn = row['CRN']
        if crn in seen:
            continue
        seen.add(crn)
        course = row['Course'].removeprefix('="').removesuffix('"')
        number = row['Section'].removeprefix('="').removesuffix('"')
        section = f"{row['Subject']} {course}-{number}"
        credits = float(row['Credits'])
        instructor = row['Instructor'].removesuffix(' (P)').strip()
        meeting = si[crn].primary if crn in si else PostedMeeting(
            row['Days'], row['Time'], row['Location'], row['Instructor'],
        )

        tags: list[str] = []
        if meeting.days and meeting.time != 'TBA':
            slot = meeting.slot_name()
            three_credit = {
                'MWF0900+50', 'MWF1000+50', 'MWF1100+50',
                'MW1200+75', 'MW1330+75', 'MW1500+75',
                'TR0900+75', 'TR1030+75', 'TR1200+75', 'TR1330+75', 'TR1500+75',
            }
            four_credit = {
                *(f'MTWR{start}+50' for start in ('0900', '1000', '1100', '1200')),
                *(f'{days}{start}+100' for days in ('MW', 'TR') for start in ('1300', '1500')),
            }
            if credits == 3 and slot in three_credit:
                tags.append('3 credit bell schedule')
            elif credits == 4 and slot in four_credit:
                tags.append('4 credit bell schedule')
            elif credits == 5 and slot in {'MTWRF0900+50', 'MTWRF1000+50', 'MTWRF1100+50'}:
                tags.append('5 credit 5×50 bell schedule')
            else:
                tags.append(slot)
            if meeting.location.startswith('SNOW '):
                tags.append('snow math rooms')
            elif meeting.location.startswith('GCB '):
                tags.append(meeting.location)
            else:
                raise ValueError(f'{section}: unrecognized posted room {meeting.location!r}')

        db.make_section_with_no_faculty(section, *tags, credit_hours=credits)
        if instructor and instructor != 'TBA':
            db.assign_faculty_to_existing_section(instructor, section)

        # SI keeps the posted meeting and room for now. The room, time, and
        # instructor are uncertain; no instructor is assigned to this meeting.
        if crn in si:
            supplemental = si[crn].supplemental
            si_section = section + 'si'
            room = supplemental.location.replace('SNOW ', 'Snow ')
            # SI posted in Snow or GCB may also use the standard math rooms.
            db.make_section_with_no_faculty(si_section, *supplemental.si_time_tags(), room, 'snow math rooms', credit_hours=credits)
            db.add_supplemental_instruction(section, si_section)
