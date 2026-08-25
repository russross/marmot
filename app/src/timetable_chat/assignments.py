from __future__ import annotations

import re
from collections import Counter, defaultdict
from collections.abc import Iterable
from dataclasses import dataclass
from hashlib import sha256
from io import BytesIO
from xml.etree import ElementTree
from zipfile import BadZipFile, ZipFile

import httpx
from defusedxml import ElementTree as DefusedElementTree

from timetable_chat.models import (
    AssignmentSourceDetails,
    AvailabilityInterval,
    CurrentAssignment,
    Faculty,
    FacultyContext,
    FacultySection,
    PreferenceHistory,
    SectionSetup,
    SectionSetupMethod,
)
from timetable_chat.semester import SemesterRepository

WORKSHEET_NAMESPACE = "http://schemas.openxmlformats.org/spreadsheetml/2006/main"
RELATIONSHIP_NAMESPACE = "http://schemas.openxmlformats.org/officeDocument/2006/relationships"
PACKAGE_RELATIONSHIP_NAMESPACE = "http://schemas.openxmlformats.org/package/2006/relationships"
EXPECTED_HEADERS = ("Lname", "Fname", "Subject", "Course", "Section", "Title", "Notes")
MAX_WORKBOOK_BYTES = 5 * 1024 * 1024
MAX_XML_BYTES = 10 * 1024 * 1024
CELL_REFERENCE = re.compile(r"^(?P<column>[A-Z]+)[1-9][0-9]*$")


@dataclass(frozen=True)
class SpreadsheetAssignment:
    row: int
    last_name: str | None
    first_name: str | None
    subject: str
    course: str
    section: str | None
    title: str
    notes: str | None

    @property
    def faculty_name(self) -> str | None:
        if self.first_name is None or self.last_name is None:
            return None
        return f"{self.first_name} {self.last_name}"

    @property
    def course_code(self) -> str:
        return f"{self.subject} {self.course}"


@dataclass(frozen=True)
class AssignmentWorkbook:
    rows: list[SpreadsheetAssignment]
    source: AssignmentSourceDetails


class AssignmentWorkbookClient:
    def __init__(self, url: str, client: httpx.AsyncClient | None = None) -> None:
        self.url = url
        self.client = client or httpx.AsyncClient(
            follow_redirects=True,
            timeout=httpx.Timeout(20.0, connect=10.0),
        )
        self._owns_client = client is None

    async def close(self) -> None:
        if self._owns_client:
            await self.client.aclose()

    async def fetch(self) -> AssignmentWorkbook:
        try:
            async with self.client.stream("GET", self.url, follow_redirects=True) as response:
                response.raise_for_status()
                content_type = response.headers.get("content-type", "").partition(";")[0]
                if (
                    content_type
                    != "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                ):
                    raise ValueError(
                        "current assignment download did not return an Excel workbook "
                        f"(content type {content_type or 'missing'})"
                    )
                content = bytearray()
                async for chunk in response.aiter_bytes():
                    content.extend(chunk)
                    if len(content) > MAX_WORKBOOK_BYTES:
                        raise ValueError("current assignment workbook exceeds the 5 MiB size limit")
                last_modified = response.headers.get("last-modified")
                etag = response.headers.get("etag")
        except httpx.HTTPError as error:
            raise ValueError(f"could not download current assignments: {error}") from error
        return AssignmentWorkbook(
            rows=parse_assignment_workbook(bytes(content)),
            source=AssignmentSourceDetails(
                url=self.url,
                revision=sha256(content).hexdigest(),
                last_modified=last_modified,
                etag=etag,
            ),
        )


def parse_assignment_workbook(content: bytes) -> list[SpreadsheetAssignment]:
    try:
        with ZipFile(BytesIO(content)) as archive:
            shared_strings = _shared_strings(archive)
            struck_style_indexes = _struck_style_indexes(archive)
            worksheet_path = _first_worksheet_path(archive)
            worksheet = _read_xml(archive, worksheet_path)
    except (BadZipFile, KeyError, ElementTree.ParseError) as error:
        raise ValueError("current assignment workbook is not a valid XLSX file") from error

    namespace = {"x": WORKSHEET_NAMESPACE}
    table: list[tuple[int, dict[str, str], bool]] = []
    for row in worksheet.findall(".//x:sheetData/x:row", namespace):
        row_number = int(row.attrib["r"])
        values: dict[str, str] = {}
        populated_cell_strike_states: list[bool] = []
        row_style_index = _row_style_index(row)
        for cell in row.findall("x:c", namespace):
            reference = cell.attrib.get("r", "")
            match = CELL_REFERENCE.fullmatch(reference)
            if match is None:
                raise ValueError(f"invalid cell reference {reference!r} in assignment workbook")
            column = match.group("column")
            value = _cell_value(cell, shared_strings)
            values[column] = value
            if column in "ABCDEF" and value.strip():
                style_index = _cell_style_index(cell, row_style_index)
                populated_cell_strike_states.append(style_index in struck_style_indexes)
        row_is_struck = bool(populated_cell_strike_states) and all(populated_cell_strike_states)
        table.append((row_number, values, row_is_struck))
    if not table:
        raise ValueError("current assignment workbook has no rows")

    header_row, header_values, _ = table[0]
    headers = tuple(header_values.get(column, "") for column in "ABCDEFG")
    if header_row != 1 or headers != EXPECTED_HEADERS:
        raise ValueError(
            "current assignment workbook headers must be " + ", ".join(EXPECTED_HEADERS)
        )

    assignments: list[SpreadsheetAssignment] = []
    for row_number, values, row_is_struck in table[1:]:
        if row_is_struck:
            continue
        if not any(value.strip() for value in values.values()):
            continue
        subject = values.get("C", "").strip()
        course = values.get("D", "").strip()
        title = values.get("F", "").strip()
        if not subject or not course or not title:
            raise ValueError(
                f"assignment workbook row {row_number} requires Subject, Course, and Title"
            )
        assignments.append(
            SpreadsheetAssignment(
                row=row_number,
                last_name=_optional_cell(values.get("A")),
                first_name=_optional_cell(values.get("B")),
                subject=subject,
                course=course,
                section=_optional_cell(values.get("E")),
                title=title,
                notes=_optional_cell(values.get("G")),
            )
        )
    return assignments


def resolve_faculty_contexts(
    repository: SemesterRepository,
    workbook: AssignmentWorkbook,
) -> tuple[dict[str, FacultyContext], list[CurrentAssignment]]:
    canonical_names = repository.faculty_names()
    grouped_rows: dict[str, list[SpreadsheetAssignment]] = defaultdict(list)
    display_names: dict[str, str] = {}
    unassigned_rows: list[SpreadsheetAssignment] = []
    for row in workbook.rows:
        if row.faculty_name is None:
            unassigned_rows.append(row)
            continue
        canonical_name = _canonical_faculty_name(row, canonical_names)
        key = canonical_name.casefold()
        display_names[key] = canonical_name
        grouped_rows[key].append(row)

    contexts: dict[str, FacultyContext] = {}
    for key, rows in grouped_rows.items():
        name = display_names[key]
        contexts[key] = _faculty_context(repository, workbook.source, name, rows)

    unassigned = _resolved_assignments(repository, None, unassigned_rows)
    return contexts, unassigned


def empty_faculty_context(
    source: AssignmentSourceDetails,
    faculty: Faculty,
) -> FacultyContext:
    return FacultyContext(
        faculty=faculty.model_copy(
            update={"sections": [], "section_setup": [], "current_preferences": None}
        ),
        assignments=[],
        assignment_source=source,
        issues=["No current assignment rows were found for this faculty member."],
    )


def _faculty_context(
    repository: SemesterRepository,
    source: AssignmentSourceDetails,
    name: str,
    rows: list[SpreadsheetAssignment],
) -> FacultyContext:
    identity_issues: list[str] = []
    try:
        base = repository.faculty(name)
    except ValueError:
        same_first_name = [
            historical_name
            for historical_name in repository.faculty_names()
            if historical_name.partition(" ")[0].casefold() == name.partition(" ")[0].casefold()
        ]
        if same_first_name:
            identity_issues.append(
                f"Spreadsheet faculty {name} did not match historical identity; possible "
                f"match: {', '.join(same_first_name)}."
            )
        else:
            identity_issues.append(
                f"Spreadsheet faculty {name} has no matching historical identity."
            )
        base = Faculty(
            name=name,
            department=_department_for_rows(repository, rows),
            availability=[
                AvailabilityInterval(days="MTWR", start="0900", end="1630"),
                AvailabilityInterval(days="F", start="0900", end="1200"),
            ],
            sections=[],
            section_setup=[],
            approved_unavailable_time_slots=[],
            current_preferences=None,
            preference_history=[
                PreferenceHistory(term=term, faculty_present=False, preferences=None)
                for term in repository.semester.historical_terms
            ],
        )
    spreadsheet_names = {
        row.faculty_name
        for row in rows
        if row.faculty_name is not None and row.faculty_name.casefold() != base.name.casefold()
    }
    identity_issues.extend(
        f"Spreadsheet faculty name {spreadsheet_name} was matched to historical identity "
        f"{base.name}."
        for spreadsheet_name in sorted(spreadsheet_names)
    )

    assignments = _resolved_assignments(repository, base, rows)
    sections = [
        FacultySection(
            name=assignment.section,
            room_tags=assignment.room_tags,
            time_slot_tags=assignment.time_slot_tags,
        )
        for assignment in assignments
    ]
    setup = [
        SectionSetup(
            method=assignment.setup_method,
            name=assignment.section,
            tags=[*assignment.time_slot_tags, *assignment.room_tags],
        )
        for assignment in assignments
    ]
    issues = [
        *identity_issues,
        *(issue for assignment in assignments for issue in assignment.issues),
    ]
    return FacultyContext(
        faculty=base.model_copy(
            update={"sections": sections, "section_setup": setup, "current_preferences": None}
        ),
        assignments=assignments,
        assignment_source=source,
        issues=issues,
    )


def _resolved_assignments(
    repository: SemesterRepository,
    faculty: Faculty | None,
    rows: list[SpreadsheetAssignment],
) -> list[CurrentAssignment]:
    missing_section_counts = Counter(row.course_code for row in rows if row.section is None)
    used_sections: dict[str, set[str]] = defaultdict(set)
    for row in rows:
        if row.section is not None:
            used_sections[row.course_code].add(_normalized_section(row.section))

    next_section: dict[str, int] = defaultdict(lambda: 1)
    result: list[CurrentAssignment] = []
    course_codes = {course.code for course in repository.semester.courses}
    for row in rows:
        issues: list[str] = []
        if row.course_code not in course_codes:
            issues.append(
                f"Spreadsheet row {row.row} uses unknown catalog course {row.course_code}."
            )
        if row.notes is not None:
            issues.append(f"Spreadsheet row {row.row} note: {row.notes}")

        section_number_inferred = row.section is None
        if row.section is None:
            section_number = _next_section_number(
                row.course_code,
                used_sections,
                next_section,
            )
            if missing_section_counts[row.course_code] == 1:
                issues.append(
                    f"Spreadsheet row {row.row} has no section; inferred {section_number} for "
                    "isolated faculty input."
                )
            else:
                issues.append(
                    f"Spreadsheet row {row.row} has no section; inferred {section_number} to "
                    "distinguish repeated course rows in isolated faculty input."
                )
        else:
            section_number = _normalized_section(row.section)
            if section_number != row.section:
                issues.append(
                    f"Spreadsheet row {row.row} section {row.section} was normalized to "
                    f"{section_number}."
                )
        section = f"{row.course_code}-{section_number}"
        room_tags, time_slot_tags, setup_method, inferred_from = _inferred_constraints(
            repository,
            faculty,
            row.course_code,
            section_number,
        )
        if not room_tags and not time_slot_tags:
            issues.append(
                f"No room or time limits could be inferred for {section}; confirm whether it "
                "is scheduleable, online, externally scheduled, or workload-only."
            )
        result.append(
            CurrentAssignment(
                spreadsheet_row=row.row,
                spreadsheet_faculty_name=row.faculty_name,
                course_code=row.course_code,
                section=section,
                title=row.title,
                notes=row.notes,
                section_number_inferred=section_number_inferred,
                constraints_inferred_from=inferred_from,
                setup_method=setup_method,
                room_tags=room_tags,
                time_slot_tags=time_slot_tags,
                issues=issues,
            )
        )
    return result


def _canonical_faculty_name(row: SpreadsheetAssignment, names: list[str]) -> str:
    spreadsheet_name = row.faculty_name
    if spreadsheet_name is None or row.last_name is None or row.first_name is None:
        raise ValueError(f"spreadsheet row {row.row} has an incomplete faculty name")
    exact = next((name for name in names if name.casefold() == spreadsheet_name.casefold()), None)
    if exact is not None:
        return exact
    surname_matches = [
        name for name in names if name.rpartition(" ")[2].casefold() == row.last_name.casefold()
    ]
    if len(surname_matches) == 1:
        return surname_matches[0]
    first_name = row.first_name.casefold()
    compatible = [
        name
        for name in surname_matches
        if (historical_first := name.partition(" ")[0].casefold()).startswith(first_name)
        or first_name.startswith(historical_first)
    ]
    if len(compatible) == 1:
        return compatible[0]
    return spreadsheet_name


def _department_for_rows(
    repository: SemesterRepository,
    rows: Iterable[SpreadsheetAssignment],
) -> str:
    departments = {
        course.department
        for row in rows
        for course in repository.semester.courses
        if course.code == row.course_code
    }
    if len(departments) != 1:
        raise ValueError("new faculty assignments do not identify one installed department")
    return departments.pop()


def _inferred_constraints(
    repository: SemesterRepository,
    faculty: Faculty | None,
    course_code: str,
    section_number: str,
) -> tuple[list[str], list[str], SectionSetupMethod, str | None]:
    if faculty is None:
        return [], [], SectionSetupMethod.MAKE, None
    active_season = repository.semester.term.partition(" ")[0]
    histories = sorted(
        faculty.preference_history,
        key=lambda history: history.term.partition(" ")[0] != active_season,
    )
    for history in histories:
        candidates = [
            setup for setup in history.section_setup if setup.name.rpartition("-")[0] == course_code
        ]
        if not candidates:
            continue
        exact = next(
            (setup for setup in candidates if setup.name.rpartition("-")[2] == section_number),
            None,
        )
        setup = exact or _common_setup(candidates)
        if setup is None:
            continue
        room_tags, time_slot_tags = _partition_tags(repository, setup.tags)
        return room_tags, time_slot_tags, setup.method, history.term
    return [], [], SectionSetupMethod.MAKE, None


def _common_setup(candidates: list[SectionSetup]) -> SectionSetup | None:
    first = candidates[0]
    if all(
        candidate.method == first.method and candidate.tags == first.tags
        for candidate in candidates
    ):
        return first
    return None


def _partition_tags(
    repository: SemesterRepository,
    tags: list[str],
) -> tuple[list[str], list[str]]:
    room_names = {room.name for room in repository.semester.rooms}
    time_slot_names = {time_slot.name for time_slot in repository.semester.time_slots}
    room_vocabulary = set(repository.semester.room_tags) | room_names
    time_vocabulary = set(repository.semester.time_slot_tags) | time_slot_names
    room_tags = [tag for tag in tags if tag in room_vocabulary]
    time_slot_tags = [tag for tag in tags if tag in time_vocabulary]
    return room_tags, time_slot_tags


def _next_section_number(
    course_code: str,
    used_sections: dict[str, set[str]],
    next_section: dict[str, int],
) -> str:
    while True:
        section_number = f"{next_section[course_code]:02d}"
        next_section[course_code] += 1
        if section_number in used_sections[course_code]:
            continue
        used_sections[course_code].add(section_number)
        return section_number


def _normalized_section(section: str) -> str:
    return section.zfill(2) if section.isdecimal() else section


def _optional_cell(value: str | None) -> str | None:
    if value is None or not (stripped := value.strip()):
        return None
    return stripped


def _read_xml(archive: ZipFile, path: str) -> ElementTree.Element:
    info = archive.getinfo(path)
    if info.file_size > MAX_XML_BYTES:
        raise ValueError(f"assignment workbook XML part {path} is too large")
    return DefusedElementTree.fromstring(
        archive.read(info),
        forbid_dtd=True,
        forbid_entities=True,
        forbid_external=True,
    )


def _shared_strings(archive: ZipFile) -> list[str]:
    try:
        root = _read_xml(archive, "xl/sharedStrings.xml")
    except KeyError:
        return []
    namespace = {"x": WORKSHEET_NAMESPACE}
    return ["".join(item.itertext()) for item in root.findall("x:si", namespace)]


def _struck_style_indexes(archive: ZipFile) -> frozenset[int]:
    try:
        styles = _read_xml(archive, "xl/styles.xml")
    except KeyError:
        return frozenset()

    namespace = {"x": WORKSHEET_NAMESPACE}
    fonts = styles.find("x:fonts", namespace)
    cell_formats = styles.find("x:cellXfs", namespace)
    if fonts is None or cell_formats is None:
        return frozenset()

    struck_font_indexes = {
        index
        for index, font in enumerate(fonts.findall("x:font", namespace))
        if _xml_flag_is_enabled(font.find("x:strike", namespace))
    }
    struck_style_indexes: set[int] = set()
    for index, cell_format in enumerate(cell_formats.findall("x:xf", namespace)):
        font_id = cell_format.attrib.get("fontId", "0")
        try:
            font_index = int(font_id)
        except ValueError as error:
            raise ValueError(
                f"assignment workbook contains invalid font index {font_id!r}"
            ) from error
        if font_index in struck_font_indexes:
            struck_style_indexes.add(index)
    return frozenset(struck_style_indexes)


def _xml_flag_is_enabled(element: ElementTree.Element | None) -> bool:
    if element is None:
        return False
    return element.attrib.get("val", "1").casefold() not in {"0", "false", "off"}


def _row_style_index(row: ElementTree.Element) -> int:
    if not _xml_flag_is_enabled_from_attribute(row, "customFormat"):
        return 0
    return _style_index(row, "row")


def _cell_style_index(cell: ElementTree.Element, row_style_index: int) -> int:
    if "s" not in cell.attrib:
        return row_style_index
    return _style_index(cell, "cell")


def _style_index(element: ElementTree.Element, element_name: str) -> int:
    value = element.attrib.get("s", "0")
    try:
        return int(value)
    except ValueError as error:
        raise ValueError(
            f"assignment workbook contains invalid {element_name} style index {value!r}"
        ) from error


def _xml_flag_is_enabled_from_attribute(element: ElementTree.Element, name: str) -> bool:
    return element.attrib.get(name, "0").casefold() not in {"0", "false", "off"}


def _first_worksheet_path(archive: ZipFile) -> str:
    workbook = _read_xml(archive, "xl/workbook.xml")
    relationships = _read_xml(archive, "xl/_rels/workbook.xml.rels")
    namespace = {"x": WORKSHEET_NAMESPACE}
    sheet = workbook.find("x:sheets/x:sheet", namespace)
    if sheet is None:
        raise ValueError("current assignment workbook has no worksheet")
    relationship_id = sheet.attrib[f"{{{RELATIONSHIP_NAMESPACE}}}id"]
    relationship = next(
        (
            item
            for item in relationships
            if item.attrib.get("Id") == relationship_id
            and item.tag == f"{{{PACKAGE_RELATIONSHIP_NAMESPACE}}}Relationship"
        ),
        None,
    )
    if relationship is None:
        raise ValueError("current assignment worksheet relationship is missing")
    target = relationship.attrib["Target"].lstrip("/")
    return target if target.startswith("xl/") else f"xl/{target}"


def _cell_value(cell: ElementTree.Element, shared_strings: list[str]) -> str:
    namespace = {"x": WORKSHEET_NAMESPACE}
    cell_type = cell.attrib.get("t")
    if cell_type == "inlineStr":
        inline = cell.find("x:is", namespace)
        return "" if inline is None else "".join(inline.itertext())
    value = cell.findtext("x:v", default="", namespaces=namespace)
    if cell_type != "s" or not value:
        return value
    try:
        return shared_strings[int(value)]
    except (IndexError, ValueError) as error:
        raise ValueError("assignment workbook contains an invalid shared string") from error
