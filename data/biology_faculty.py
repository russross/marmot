import csv
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path

from posted_schedule import PostedMeeting, SupplementalOffering, room_name, supplemental_offerings
from queries import DB, TimeInterval
from rooms import science_lecture_tag


@dataclass(frozen=True)
class Offering:
    section: str
    credits: float
    instructors: tuple[str, ...]
    meeting: PostedMeeting
    dates: str
    title: str
    supplemental: SupplementalOffering | None


def build_faculty(db: DB) -> None:
    path = Path(__file__).parent / 'postedschedule' / 'biol.csv'
    with path.open(encoding='utf-8-sig', newline='') as file:
        rows = [row for row in csv.DictReader(file) if int(row['Cap']) > 0]
    si = {offering.crn: offering for offering in supplemental_offerings(path.parent)}
    by_crn: dict[str, list[dict[str, str]]] = defaultdict(list)
    for row in rows:
        by_crn[row['CRN']].append(row)

    # SI rows share a CRN with the main meeting. Any other multi-row biology
    # offering needs an explicit interpretation before it can be imported.
    offerings: list[Offering] = []
    for crn, records in by_crn.items():
        if len(records) != 1 and crn not in si:
            raise ValueError(f'BIOL CRN {crn}: unsupported multiple meetings')
        row = records[0]
        course = row['Course'].removeprefix('="').removesuffix('"')
        number = row['Section'].removeprefix('="').removesuffix('"')
        meeting = si[crn].primary if crn in si else PostedMeeting(
            row['Days'], row['Time'], row['Location'], row['Instructor'],
        )
        instructors = tuple(
            name for part in row['Instructor'].split(',')
            if (name := part.removesuffix(' (P)').strip()) not in ('', 'TBA')
        )
        offerings.append(Offering(
            f"BIOL {course}-{number}", float(row['Credits']),
            instructors, meeting,
            row['Dates'], row['Title'], si.get(crn),
        ))

    availability = [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')]
    for name in sorted({name for offering in offerings for name in offering.instructors}):
        if db.db.execute('SELECT 1 FROM faculty WHERE faculty = ?', (name,)).fetchone() is None:
            db.make_faculty(name, 'Biological Sciences', availability)

    # An identical room, meeting, and date range indicate a shared lecture
    # or a 5000/6000 listing, even if one instructor is TBA. SI relationships
    # continue to reference each original section.
    shared: dict[tuple[str, str, str, str], list[Offering]] = defaultdict(list)
    individual: list[Offering] = []
    for offering in offerings:
        meeting = offering.meeting
        if not meeting.days or meeting.time == 'TBA':
            individual.append(offering)
        else:
            key = (meeting.days, meeting.time, meeting.location, offering.dates)
            shared[key].append(offering)

    standard_three_credit = {
        'MWF0900+50', 'MWF1000+50', 'MWF1100+50',
        'MW1200+75', 'MW1330+75', 'MW1500+75',
        'TR0900+75', 'TR1030+75', 'TR1200+75', 'TR1330+75', 'TR1500+75',
    }
    for group in list(shared.values()) + [[offering] for offering in individual]:
        group.sort(key=lambda offering: not offering.instructors)
        primary = group[0]
        meeting = primary.meeting
        tags: list[str] = []
        if meeting.days and meeting.time != 'TBA':
            slot = meeting.slot_name()
            # Lectures use compatible science halls; labs retain their room
            # while ordinary single-day labs use the staggered time grid.
            if primary.credits == 3 and slot in standard_three_credit and 'lab' not in primary.title.lower():
                tags.append('3 credit bell schedule')
            elif primary.credits == 2 and meeting.days in ('MW', 'TR') and meeting.weekly_minutes() == 100 and 'lab' not in primary.title.lower():
                tags.extend([slot, '2 credit 2×50 bell schedule'])
            elif primary.credits == 4 and meeting.days in ('MTWR', 'MTWF', 'MWRF', 'MTRF', 'TWRF') and meeting.weekly_minutes() == 200 and 'lab' not in primary.title.lower():
                tags.extend([slot, '4 credit four-day daytime'])
            elif 'lab' in primary.title.lower():
                tags.extend(meeting.lab_time_tags())
            else:
                tags.append(slot)
            room = room_name(meeting.location)
            tags.append((science_lecture_tag(room) if 'lab' not in primary.title.lower() else None) or room)
        db.make_section_with_no_faculty(primary.section, *tags, credit_hours=primary.credits)
        for name in sorted({name for offering in group for name in offering.instructors}):
            db.assign_faculty_to_existing_section(name, primary.section)

        # Aliases do not own room, time, or faculty rows; they refer to the
        # same scheduled lecture while retaining their own enrollment record.
        for alias in group[1:]:
            db.make_section_with_no_faculty(alias.section, credit_hours=alias.credits)
        if len(group) > 1:
            db.add_cross_listing(primary.section, [alias.section for alias in group[1:]])
        for offering in group:
            if offering.supplemental is None:
                continue
            extra = offering.supplemental.supplemental
            section = offering.section + 'si'
            room = room_name(extra.location)
            db.make_section_with_no_faculty(
                section, *extra.si_time_tags(), science_lecture_tag(room) or room, credit_hours=offering.credits,
            )
            db.add_supplemental_instruction(offering.section, section)
