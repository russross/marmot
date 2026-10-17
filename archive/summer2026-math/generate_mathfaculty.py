#!/usr/bin/env python3
import csv
import re
import sys
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import NoReturn


CSV_PATH = Path("math-fall-2026.csv")
OUT_PATH = Path("mathfaculty.py")

TIME_RE = re.compile(r"^(\d{1,2}):(\d{2})\s*([ap]m)-(\d{1,2}):(\d{2})\s*([ap]m)$", re.IGNORECASE)
DAYS_RE = re.compile(r"^[MTWRF]+$")
COURSE_RE = re.compile(r"^\d{4}[A-Z]?$")
SECTION_RE = re.compile(r"^\d{2}[A-Z]?$")


@dataclass(frozen=True)
class SectionRow:
    row_num: int
    subject: str
    course: str
    section: str
    campus: str
    credits: int
    days: str
    time: str
    location: str
    instructor_raw: str
    instructors: list[str]
    attributes: str
    has_si_attr: bool


@dataclass(frozen=True)
class SectionOutput:
    section_name: str
    tags: list[str]
    comment: str | None
    instructors: list[str]
    is_scheduleable: bool


def fail(msg: str) -> NoReturn:
    print(f"ERROR: {msg}", file=sys.stderr)
    raise SystemExit(1)


def clean_excel_text(value: str) -> str:
    value = value.strip()
    if value.startswith('="') and value.endswith('"'):
        return value[2:-1]
    return value


def parse_ampm(hh: str, mm: str, ampm: str) -> int:
    h = int(hh)
    m = int(mm)
    a = ampm.lower()
    if a == "pm" and h != 12:
        h += 12
    if a == "am" and h == 12:
        h = 0
    return h * 60 + m


def parse_time_range(time_text: str) -> tuple[int, int, int, str]:
    m = TIME_RE.match(time_text.strip())
    if not m:
        fail(f"invalid time format: {time_text!r}")
    sh, sm, sa, eh, em, ea = m.groups()
    start = parse_ampm(sh, sm, sa)
    end = parse_ampm(eh, em, ea)
    if end <= start:
        fail(f"end time must be after start time: {time_text!r}")
    duration = end - start
    hhmm = f"{start // 60:02d}{start % 60:02d}"
    return start, end, duration, hhmm


def parse_instructors(text: str) -> list[str]:
    parts = [p.strip() for p in text.split(",") if p.strip()]
    cleaned = []
    for part in parts:
        if part == "TBA":
            return []
        cleaned_name = re.sub(r"\s*\(P\)\s*$", "", part).strip()
        if cleaned_name and cleaned_name != "TBA":
            cleaned.append(cleaned_name)
    return cleaned


def section_number(section: str) -> int:
    if len(section) < 2 or not section[:2].isdigit():
        fail(f"section does not begin with 2 digits: {section!r}")
    return int(section[:2])


def bell_schedule_tag(credits: int, days: str, start: int, duration: int, hhmm: str) -> str:
    if credits == 3:
        if days == "MWF" and hhmm in {"0900", "1000", "1100"} and duration == 50:
            return "3 credit bell schedule"
        if days == "MW" and hhmm in {"1200", "1330", "1500"} and duration == 75:
            return "3 credit bell schedule"
        if days == "TR" and hhmm in {"0900", "1030", "1200", "1330", "1500"} and duration == 75:
            return "3 credit bell schedule"
    if credits == 4:
        if days == "MTWR" and hhmm in {"0900", "1000", "1100", "1200"} and duration == 50:
            return "4 credit bell schedule"
        if days in {"MW", "TR"} and hhmm in {"1300", "1500"} and duration == 100:
            return "4 credit bell schedule"
    if credits == 1:
        if len(days) == 1 and duration == 50 and 9 * 60 <= start <= 13 * 60:
            return f"{days}{hhmm}+{duration}"
    return f"{days}{hhmm}+{duration}"


def parse_section_row(row: dict[str, str], row_num: int) -> SectionRow:
    subject = clean_excel_text(row["Subject"])
    course = clean_excel_text(row["Course"])
    section = clean_excel_text(row["Section"])
    campus = row["Campus"].strip()
    credits_text = row["Credits"].strip()
    days = row["Days"].strip()
    time_text = row["Time"].strip()
    location = row["Location"].strip()
    instructor_raw = row["Instructor"].strip()
    attributes = row["Attributes"].strip()

    if subject != "MATH":
        fail(f"row {row_num}: expected Subject MATH, found {subject!r}")
    if not COURSE_RE.fullmatch(course):
        fail(f"row {row_num}: unexpected course format {course!r}")
    if not SECTION_RE.fullmatch(section):
        fail(f"row {row_num}: unexpected section format {section!r}")
    try:
        credits = int(credits_text)
    except ValueError:
        fail(f"row {row_num}: credits is not an integer: {credits_text!r}")

    return SectionRow(
        row_num=row_num,
        subject=subject,
        course=course,
        section=section,
        campus=campus,
        credits=credits,
        days=days,
        time=time_text,
        location=location,
        instructor_raw=instructor_raw,
        instructors=parse_instructors(instructor_raw),
        attributes=attributes,
        has_si_attr="SI" in {a.strip() for a in attributes.split(",") if a.strip()},
    )


def build_make_call(faculty: str, section_name: str, tags: list[str], comment: str | None) -> str:
    args = [repr(faculty), repr(section_name), *[repr(t) for t in tags]]
    line = f"    db.make_faculty_section({', '.join(args)})"
    if comment:
        line += f"  # {comment}"
    return line


def build_make_no_faculty_call(section_name: str, tags: list[str], comment: str | None) -> str:
    args = [repr(section_name), *[repr(t) for t in tags]]
    line = f"    db.make_section_with_no_faculty({', '.join(args)})"
    if comment:
        line += f"  # {comment}"
    return line


def build_assign_call(faculty: str, section_name: str) -> str:
    return f"    db.assign_faculty_to_existing_section({repr(faculty)}, {repr(section_name)})"


def main() -> None:
    if not CSV_PATH.exists():
        fail(f"missing CSV file: {CSV_PATH}")

    rows: list[SectionRow] = []
    with CSV_PATH.open(newline="", encoding="utf-8-sig") as f:
        reader = csv.DictReader(f)
        required = {
            "Subject", "Course", "Section", "Campus", "Credits", "Days", "Time", "Instructor", "Location", "Attributes"
        }
        if not required.issubset(set(reader.fieldnames or [])):
            fail(f"CSV is missing required columns. Found: {reader.fieldnames}")
        for i, row in enumerate(reader, start=2):
            rows.append(parse_section_row(row, i))

    grouped: dict[tuple[str, str, str], list[SectionRow]] = defaultdict(list)
    order: list[tuple[str, str, str]] = []
    for row in rows:
        key = (row.subject, row.course, row.section)
        if key not in grouped:
            order.append(key)
        grouped[key].append(row)

    faculty_order: list[str] = []
    faculty_seen: set[str] = set()
    faculty_actions: dict[str, list[str]] = defaultdict(list)
    faculty_section_counts: dict[str, int] = defaultdict(int)
    unassigned_actions: list[str] = []
    section_outputs: list[SectionOutput] = []

    def track_faculty(name: str) -> None:
        if name not in faculty_seen:
            faculty_seen.add(name)
            faculty_order.append(name)

    for key in order:
        section_rows = grouped[key]
        si_rows = []
        if len(section_rows) > 1:
            # Supplemental-instruction add-ons in this feed are duplicate rows with
            # SI attribute and a single meeting day. Omit them from timetabling input.
            si_rows = [r for r in section_rows if r.has_si_attr and len(r.days) == 1]
        normal_rows = [r for r in section_rows if r not in si_rows]

        if not normal_rows:
            fail(f"section {key[0]} {key[1]}-{key[2]} has only SI rows; no schedulable row found")
        if len(normal_rows) > 1:
            locs = [(r.days, r.time, r.location) for r in normal_rows]
            fail(f"section {key[0]} {key[1]}-{key[2]} has multiple non-SI rows: {locs}")

        row = normal_rows[0]
        subject = row.subject
        course = row.course
        section = row.section
        section_name = f"{subject} {course}-{section}"
        sec_num = section_number(section)
        campus = row.campus
        days = row.days
        time_text = row.time
        location = row.location
        credits = row.credits
        instructors = row.instructors

        # Skip individualized classes by explicit rule.
        if campus == "A01" and time_text.upper() == "TBA" and location == "":
            continue

        online_candidate = sec_num in range(40, 50) or campus == "O01" or location.upper() == "ONLINE"

        tags: list[str] = []
        is_scheduleable = False
        if online_candidate:
            if sec_num not in range(40, 50):
                fail(f"{section_name}: online section must be numbered 40-49")
            if campus != "O01":
                fail(f"{section_name}: online section must have campus O01, found {campus!r}")
            if time_text.upper() != "TBA":
                fail(f"{section_name}: online section must have time TBA, found {time_text!r}")
            if location.upper() != "ONLINE":
                fail(f"{section_name}: online section must have location ONLINE, found {location!r}")
            if days:
                fail(f"{section_name}: online section should have blank days, found {days!r}")
            if instructors and len(instructors) > 2:
                fail(f"{section_name}: expected at most 2 instructors, found {instructors!r}")
        else:
            is_scheduleable = True
            if campus != "A01":
                fail(f"{section_name}: non-online section must have campus A01, found {campus!r}")
            if not location.startswith("SNOW"):
                fail(f"{section_name}: non-online section must be in SNOW building, found {location!r}")
            if not days or not DAYS_RE.fullmatch(days):
                fail(f"{section_name}: invalid day pattern {days!r}")
            if time_text.upper() == "TBA":
                fail(f"{section_name}: non-online section cannot be TBA")

            start, _end, duration, hhmm = parse_time_range(time_text)

            if start >= 18 * 60:
                if sec_num < 50:
                    fail(f"{section_name}: evening section starts at/after 6:00 PM but section is not 50+")
            else:
                if sec_num >= 50:
                    fail(f"{section_name}: section is 50+ but starts before 6:00 PM")
                if sec_num in range(40, 50):
                    fail(f"{section_name}: section 40-49 reserved for online sections")

            weekly_minutes = duration * len(days)
            expected_minutes = 50 * int(credits)
            if weekly_minutes != expected_minutes:
                fail(
                    f"{section_name}: weekly minutes mismatch ({weekly_minutes}) != 50*credits ({expected_minutes})"
                )

            tags.append(bell_schedule_tag(credits, days, start, duration, hhmm))
            tags.append("snow math rooms" if location.startswith("SNOW") else location)

            if instructors and len(instructors) > 2:
                fail(f"{section_name}: expected at most 2 instructors, found {instructors!r}")

        si_comment = None
        comment_parts: list[str] = []
        if si_rows:
            si_parts = []
            for si in si_rows:
                if si.time.upper() == "TBA" or not si.days:
                    si_parts.append("SI omitted: TBA")
                else:
                    s_start, _s_end, s_dur, s_hhmm = parse_time_range(si.time)
                    _ = s_start
                    si_parts.append(f"SI omitted: {si.days} {s_hhmm}+{s_dur} {si.location}")
            comment_parts.append("; ".join(si_parts))

        if not instructors:
            comment_parts.append("no instructor assigned; created with room/time constraints only")

        if comment_parts:
            si_comment = "; ".join(comment_parts)

        for instructor in instructors:
            track_faculty(instructor)
            if is_scheduleable:
                faculty_section_counts[instructor] += 1
        section_outputs.append(
            SectionOutput(
                section_name=section_name,
                tags=tags,
                comment=si_comment,
                instructors=instructors,
                is_scheduleable=is_scheduleable,
            )
        )

    faculty_index = {name: idx for idx, name in enumerate(faculty_order)}
    for entry in section_outputs:
        instructors = entry.instructors
        section_name = entry.section_name
        tags = entry.tags
        si_comment = entry.comment
        if not instructors:
            unassigned_actions.append(build_make_no_faculty_call(section_name, tags, si_comment))
            continue
        if len(instructors) == 1:
            creator = instructors[0]
            faculty_actions[creator].append(build_make_call(creator, section_name, tags, si_comment))
            continue
        creator = min(instructors, key=lambda name: faculty_index[name])
        faculty_actions[creator].append(build_make_call(creator, section_name, tags, si_comment))
        for instructor in instructors:
            if instructor != creator:
                faculty_actions[instructor].append(build_assign_call(instructor, section_name))

    lines: list[str] = []
    lines.append("import queries")
    lines.append("from queries import *")
    lines.append("")
    lines.append("")
    lines.append("def build_faculty(db: DB) -> None:")
    lines.append("    print('building math faculty and sections')")
    lines.append("    default_availability = [TimeInterval('MTWR', '0900', '1640'), TimeInterval('F', '0900', '1200')]")
    lines.append("    default_prefs = [")
    lines.append("        WantClassesEvenlySpreadAcrossDays(),")
    lines.append("        AvoidClassClusterLongerThan('2h45m'),")
    lines.append("        AvoidClassClusterShorterThan('1h50m'),")
    lines.append("        AvoidGapBetweenClassClustersLongerThan('1h45m'),")
    lines.append("    ]")
    lines.append("    reduced_default_prefs = [")
    lines.append("        AvoidClassClusterLongerThan('2h45m'),")
    lines.append("        AvoidClassClusterShorterThan('1h50m'),")
    lines.append("        AvoidGapBetweenClassClustersLongerThan('1h45m'),")
    lines.append("    ]")
    lines.append("")

    for action in unassigned_actions:
        lines.append(action)
    if unassigned_actions:
        lines.append("")

    for faculty in faculty_order:
        lines.append(f"    db.make_faculty({faculty!r}, 'Mathematics', default_availability)")
        actions = faculty_actions[faculty]
        for action in actions:
            lines.append(action)
        scheduleable_count = faculty_section_counts[faculty]
        if scheduleable_count > 3:
            pref_list_name = "default_prefs"
        elif scheduleable_count > 1:
            pref_list_name = "reduced_default_prefs"
        else:
            pref_list_name = None

        if pref_list_name is not None:
            lines.append(f"    db.faculty_preferences({faculty!r}, 'MT',")
            lines.append(f"        *{pref_list_name},")
            lines.append("    )")
        lines.append("")

    OUT_PATH.write_text("\n".join(lines).rstrip() + "\n", encoding="utf-8")
    print(f"wrote {OUT_PATH}")


if __name__ == "__main__":
    main()
