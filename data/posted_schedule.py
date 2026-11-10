import csv
from collections import defaultdict
from dataclasses import dataclass
from datetime import datetime
from decimal import Decimal
from pathlib import Path


def room_name(posted: str) -> str:
    # Brown and Snow are stored with title case; other building codes match
    # the schedule export.
    building, number = posted.split(' ', 1)
    if building in ('BROWN', 'SNOW', 'SMITH'):
        building = building.title()
    return f'{building} {number}'


@dataclass(frozen=True)
class PostedMeeting:
    days: str
    time: str
    location: str
    instructor: str

    def weekly_minutes(self) -> int:
        start, end = self.time.split('-', 1)
        start_time = datetime.strptime(start.strip(), '%I:%M %p')
        end_time = datetime.strptime(end.strip(), '%I:%M %p')
        minutes = int((end_time - start_time).total_seconds() // 60)
        if minutes <= 0 or not self.days or any(day not in 'MTWRFSU' for day in self.days):
            raise ValueError(f'invalid posted meeting: {self.days} {self.time}')
        return minutes * len(self.days)

    def slot_name(self) -> str:
        start, end = self.time.split('-', 1)
        start_time = datetime.strptime(start.strip(), '%I:%M %p')
        end_time = datetime.strptime(end.strip(), '%I:%M %p')
        self.weekly_minutes()
        minutes = int((end_time - start_time).total_seconds() // 60)
        return f'{self.days}{start_time:%H%M}+{minutes}'

    def lab_time_tags(self) -> list[str]:
        slot = self.slot_name()
        start_text = self.time.split('-', 1)[0]
        start = datetime.strptime(start_text.strip(), '%I:%M %p')
        duration = self.weekly_minutes() // len(self.days)
        # Restrict ordinary same-day lab grids to posted starts already inside
        # core hours. Special and outside-hours labs remain explicitly fixed.
        latest = 15 if duration == 110 else 14
        days_allowed = self.days in ('M', 'T', 'W', 'R') or (self.days in ('MW', 'TR') and duration == 110)
        if days_allowed and duration in (110, 170) and start.minute == 0 and 9 <= start.hour <= latest:
            phase = start.hour % (2 if duration == 110 else 3)
            return [slot, f'lab {duration} {self.days} phase {phase}']
        return [slot]

    def si_time_tags(self) -> list[str]:
        slot = self.slot_name()
        if len(self.days) == 1 and self.weekly_minutes() == 50:
            return [slot, 'SI 50 minute daytime']
        return [slot]


@dataclass(frozen=True)
class SupplementalOffering:
    crn: str
    section: str
    credits: Decimal
    primary: PostedMeeting
    supplemental: PostedMeeting


def supplemental_offerings(directory: Path) -> list[SupplementalOffering]:
    # Group SI rows by CRN; attributes and credits are repeated on each row.
    # These posted offerings have unmarked supplemental meetings. Their
    # MWF lectures are primary; the other meeting is supplemental.
    primary_day_overrides = {'21557': 'MWF', '20543': 'MWF', '22611': 'MWF'}
    groups: dict[str, list[dict[str, str]]] = defaultdict(list)
    for path in sorted(directory.glob('*.csv')):
        with path.open(encoding='utf-8-sig', newline='') as file:
            for row in csv.DictReader(file):
                if int(row['Cap']) == 0:
                    continue
                if (row['CRN'] in primary_day_overrides or
                        'SI' in {attribute.strip().upper() for attribute in row['Attributes'].split(',')}):
                    groups[row['CRN']].append(row)

    offerings = []
    for crn, rows in groups.items():
        if len(rows) != 2:
            raise ValueError(f'SI CRN {crn}: expected two meetings, found {len(rows)}')
        first = rows[0]
        if any(row[key] != first[key] for row in rows for key in ('Subject', 'Course', 'Section', 'Credits')):
            raise ValueError(f'SI CRN {crn}: inconsistent section or credit information')
        credits = Decimal(first['Credits'])
        meetings = [PostedMeeting(row['Days'], row['Time'], row['Location'], row['Instructor']) for row in rows]
        if crn in primary_day_overrides:
            matching = [meeting.days == primary_day_overrides[crn] for meeting in meetings]
        else:
            matching = [meeting.weekly_minutes() == credits * 50 for meeting in meetings]
        if matching.count(True) != 1:
            raise ValueError(f'SI CRN {crn}: cannot identify one primary meeting')

        # Exported course and section cells use Excel formula syntax to retain
        # leading zeroes; ordinary CSV values are accepted as well.
        course = first['Course'].removeprefix('="').removesuffix('"')
        section = first['Section'].removeprefix('="').removesuffix('"')
        primary_index = matching.index(True)
        offerings.append(SupplementalOffering(
            crn, f"{first['Subject']} {course}-{section}", credits,
            meetings[primary_index], meetings[1 - primary_index],
        ))
    return offerings
