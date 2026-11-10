import csv
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path

from posted_schedule import PostedMeeting, room_name
from queries import DB, TimeInterval
from rooms import science_lecture_tag


@dataclass(frozen=True)
class Offering:
    section: str
    credits: float
    instructor: str
    meeting: PostedMeeting
    dates: str
    title: str
    split: bool


def build_faculty(db: DB) -> None:
    directory = Path(__file__).parent / 'postedschedule'
    offerings: list[Offering] = []
    split_counts: dict[str, int] = {}
    for prefix in ('envs', 'geo', 'geog', 'ener', 'sci'):
        with (directory / f'{prefix}.csv').open(encoding='utf-8-sig', newline='') as file:
            rows = [row for row in csv.DictReader(file) if int(row['Cap']) > 0]
        groups: dict[str, list[dict[str, str]]] = defaultdict(list)
        for row in rows:
            groups[row['CRN']].append(row)
        for crn, records in groups.items():
            first = records[0]
            course = first['Course'].removeprefix('="').removesuffix('"')
            number = first['Section'].removeprefix('="').removesuffix('"')
            course_name = f"{first['Subject']} {course}"
            split = len(records) > 1
            # These two posted courses have lecture and lab meetings under
            # one CRN. Both are fixed and taught by the posted instructor.
            if split and (course_name, number) not in {('ENVS 4080', '01'), ('GEO 3700', '01')}:
                raise ValueError(f'{course_name}-{number}: unsupported multiple meetings')
            if split:
                split_counts[course_name] = sum(
                    1 for group in groups.values() if group[0]['Course'] == first['Course']
                )
            for index, row in enumerate(records):
                offerings.append(Offering(
                    f'{course_name}-{number}' + ('extra' if index else ''), float(row['Credits']),
                    row['Instructor'].removesuffix(' (P)').strip(),
                    PostedMeeting(row['Days'], row['Time'], row['Location'], row['Instructor']),
                    row['Dates'], row['Title'], split,
                ))

    availability = [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')]
    for name in sorted({offering.instructor for offering in offerings} - {'', 'TBA'}):
        if db.db.execute('SELECT 1 FROM faculty WHERE faculty = ?', (name,)).fetchone() is None:
            db.make_faculty(name, 'Earth, Energy and Environmental Science', availability)

    # Matching room, time, and dates are treated as one shared meeting; the
    # GEO and ENVS 4910 sections are a posted example. Date ranges themselves
    # are not yet used by the solver when testing overlapping placements.
    shared: dict[tuple[str, str, str, str], list[Offering]] = defaultdict(list)
    individual: list[Offering] = []
    for offering in offerings:
        m = offering.meeting
        if not m.days or m.time == 'TBA':
            individual.append(offering)
        else:
            shared[(m.days, m.time, m.location, offering.dates)].append(offering)

    standard_three = {
        'MWF0900+50', 'MWF1000+50', 'MWF1100+50', 'MW1200+75', 'MW1330+75', 'MW1500+75',
        'TR0900+75', 'TR1030+75', 'TR1200+75', 'TR1330+75', 'TR1500+75',
    }
    for group in list(shared.values()) + [[offering] for offering in individual]:
        group.sort(key=lambda offering: offering.instructor in ('', 'TBA'))
        primary = group[0]
        m = primary.meeting
        tags: list[str] = []
        if m.days and m.time != 'TBA':
            slot = m.slot_name()
            if not primary.split and primary.credits == 3 and 'lab' not in primary.title.lower() and slot in standard_three:
                tags.append('3 credit bell schedule')
            elif not primary.split and 'lab' in primary.title.lower():
                tags.extend(m.lab_time_tags())
            else:
                tags.append(slot)
            room = room_name(m.location)
            tags.append((science_lecture_tag(room) if not primary.split and 'lab' not in primary.title.lower() else None) or room)
        db.make_section_with_no_faculty(primary.section, *tags, credit_hours=primary.credits)
        for name in sorted({offering.instructor for offering in group} - {'', 'TBA'}):
            db.assign_faculty_to_existing_section(name, primary.section)
        for alias in group[1:]:
            db.make_section_with_no_faculty(alias.section, credit_hours=alias.credits)
        if len(group) > 1:
            db.add_cross_listing(primary.section, [alias.section for alias in group[1:]])
    for course, count in split_counts.items():
        db.add_multiple_section_override(course, count)
