import csv
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path

from posted_schedule import PostedMeeting, room_name
from queries import DB, TimeInterval


@dataclass(frozen=True)
class Offering:
    section: str
    credits: float
    instructor: str
    meeting: PostedMeeting
    dates: str
    campus: str


def build_faculty(db: DB) -> None:
    directory = Path(__file__).parent / 'postedschedule'
    offerings: list[Offering] = []
    for prefix in ('des', 'ixd', 'uxd'):
        with (directory / f'{prefix}.csv').open(encoding='utf-8-sig', newline='') as file:
            rows = [row for row in csv.DictReader(file) if int(row['Cap']) > 0]
        seen: set[str] = set()
        for row in rows:
            if row['CRN'] in seen:
                raise ValueError(f"{prefix.upper()} CRN {row['CRN']}: unsupported multiple meetings")
            seen.add(row['CRN'])
            course = row['Course'].removeprefix('="').removesuffix('"')
            number = row['Section'].removeprefix('="').removesuffix('"')
            offerings.append(Offering(
                f"{row['Subject']} {course}-{number}", float(row['Credits']),
                row['Instructor'].removesuffix(' (P)').strip(),
                PostedMeeting(row['Days'], row['Time'], row['Location'], row['Instructor']),
                row['Dates'], row['Campus'],
            ))

    availability = [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')]
    for name in sorted({offering.instructor for offering in offerings} - {'', 'TBA'}):
        if db.db.execute('SELECT 1 FROM faculty WHERE faculty = ?', (name,)).fetchone() is None:
            db.make_faculty(name, 'Design', availability)

    # Only physical rooms identify shared meetings. VIRT is a delivery mode,
    # not a single shared room; C02 has no room in the posted data.
    shared: dict[tuple[str, str, str, str], list[Offering]] = defaultdict(list)
    individual: list[Offering] = []
    for offering in offerings:
        m = offering.meeting
        if not m.days or m.time == 'TBA' or m.location in ('', 'VIRT', 'ONLINE'):
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
            tags.append('3 credit bell schedule' if primary.credits == 3 and slot in standard_three else slot)
            if m.location not in ('', 'VIRT', 'ONLINE'):
                tags.append(room_name(m.location))
        db.make_section_with_no_faculty(primary.section, *tags, credit_hours=primary.credits)
        for name in sorted({offering.instructor for offering in group} - {'', 'TBA'}):
            db.assign_faculty_to_existing_section(name, primary.section)
        for alias in group[1:]:
            db.make_section_with_no_faculty(alias.section, credit_hours=alias.credits)
        if len(group) > 1:
            db.add_cross_listing(primary.section, [alias.section for alias in group[1:]])
