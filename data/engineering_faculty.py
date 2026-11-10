import csv
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path

from posted_schedule import PostedMeeting
from queries import DB, TimeInterval


@dataclass(frozen=True)
class Offering:
    section: str
    credits: float
    instructor: str
    meeting: PostedMeeting
    dates: str
    title: str


def build_faculty(db: DB) -> None:
    # MTRN 3455-01 has two fixed Tuesday meetings, both instructor-led.
    directory = Path(__file__).parent / 'postedschedule'
    offerings: list[Offering] = []
    for prefix in ('ece', 'mech', 'mtrn'):
        with (directory / f'{prefix}.csv').open(encoding='utf-8-sig', newline='') as file:
            rows = [row for row in csv.DictReader(file) if int(row['Cap']) > 0]
        groups: dict[str, list[dict[str, str]]] = defaultdict(list)
        for row in rows:
            groups[row['CRN']].append(row)
        for crn, records in groups.items():
            is_split_lab = records[0]['Subject'] == 'MTRN' and records[0]['Course'] == '="3455"' and records[0]['Section'] == '="01"'
            if len(records) != 1 and not (is_split_lab and len(records) == 2):
                raise ValueError(f'{prefix.upper()} CRN {crn}: unsupported multiple meetings')
            for index, row in enumerate(records):
                course = row['Course'].removeprefix('="').removesuffix('"')
                number = row['Section'].removeprefix('="').removesuffix('"')
                suffix = 'extra' if index else ''
                offerings.append(Offering(
                    f"{row['Subject']} {course}-{number}{suffix}", float(row['Credits']),
                    row['Instructor'].removesuffix(' (P)').strip(),
                    PostedMeeting(row['Days'], row['Time'], row['Location'], row['Instructor']),
                    row['Dates'], row['Title'],
                ))

    availability = [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')]
    for name in sorted({o.instructor for o in offerings} - {'TBA', ''}):
        if db.db.execute('SELECT 1 FROM faculty WHERE faculty = ?', (name,)).fetchone() is None:
            db.make_faculty(name, 'Engineering', availability)

    # Identical room, time, and dates identify a common meeting. This includes
    # product-design listings with two instructors. Online and TBA offerings
    # remain separate when they have no meeting time.
    shared: dict[tuple[str, str, str, str], list[Offering]] = defaultdict(list)
    individual: list[Offering] = []
    for offering in offerings:
        m = offering.meeting
        if not m.days or m.time == 'TBA':
            individual.append(offering)
        else:
            shared[(m.days, m.time, m.location, offering.dates)].append(offering)

    standard_three = {
        'MWF0900+50', 'MWF1000+50', 'MWF1100+50',
        'MW1200+75', 'MW1330+75', 'MW1500+75',
        'TR0900+75', 'TR1030+75', 'TR1200+75', 'TR1330+75', 'TR1500+75',
    }
    standard_two = {
        f'{days}{start}+50' for days in ('MW', 'TR')
        for start in ('0900', '1000', '1030', '1100', '1200', '1330', '1500')
    }
    for group in list(shared.values()) + [[offering] for offering in individual]:
        group.sort(key=lambda offering: offering.instructor in ('', 'TBA'))
        primary = group[0]
        m = primary.meeting
        tags: list[str] = []
        # MECH 4860R-02 is an individualized practicum posted during the
        # instructor's shared product-design meeting; retain workload only.
        if m.days and m.time != 'TBA' and primary.section != 'MECH 4860R-02':
            slot = m.slot_name()
            if 'lab' not in primary.title.lower() and primary.credits == 3 and slot in standard_three:
                tags.append('3 credit bell schedule')
            elif 'lab' not in primary.title.lower() and primary.credits == 2 and slot in standard_two:
                tags.append('2 credit 2×50 bell schedule')
            elif 'lab' in primary.title.lower() and not primary.section.startswith('MTRN 3455-01'):
                tags.extend(m.lab_time_tags())
            else:
                tags.append(slot)
            tags.append(m.location)
        db.make_section_with_no_faculty(primary.section, *tags, credit_hours=primary.credits)
        for name in sorted({offering.instructor for offering in group} - {'', 'TBA'}):
            db.assign_faculty_to_existing_section(name, primary.section)
        for alias in group[1:]:
            db.make_section_with_no_faculty(alias.section, credit_hours=alias.credits)
        if len(group) > 1:
            db.add_cross_listing(primary.section, [alias.section for alias in group[1:]])

    # The two MTRN 3455 meetings are one student offering, not two choices.
    if any(offering.section == 'MTRN 3455-01extra' for offering in offerings):
        count = sum(1 for offering in offerings if offering.section.startswith('MTRN 3455-') and not offering.section.endswith('extra'))
        db.add_multiple_section_override('MTRN 3455', count)
