from collections.abc import Iterator, Sequence
from io import BytesIO
from pathlib import Path
from zipfile import ZIP_DEFLATED, ZipFile

import pytest

from timetable_chat.assignments import (
    AssignmentWorkbook,
    AssignmentWorkbookClient,
    parse_assignment_workbook,
)
from timetable_chat.models import AssignmentSourceDetails
from timetable_chat.semester import SemesterRepository

APP_ROOT = Path(__file__).resolve().parents[1]


@pytest.fixture(scope="session")
def repository() -> SemesterRepository:
    return SemesterRepository.load(APP_ROOT / "data" / "fall-2026.json")


@pytest.fixture(scope="session")
def spring_repository() -> SemesterRepository:
    return SemesterRepository.load(APP_ROOT / "data" / "spring-2027.json")


class StaticAssignmentClient(AssignmentWorkbookClient):
    def __init__(self, workbook: AssignmentWorkbook) -> None:
        self.workbook = workbook

    async def fetch(self) -> AssignmentWorkbook:
        return self.workbook

    async def close(self) -> None:
        return None


@pytest.fixture(scope="session")
def assignment_workbook() -> AssignmentWorkbook:
    rows = [
        ("Lname", "Fname", "Subject", "Course", "Section", "Title", "Notes"),
        ("Stander", "Barton", "CS", "2100", "01", "Discrete Structures", ""),
        ("Stander", "Barton", "CS", "2420", "01", "Data Structures", ""),
        ("Stander", "Barton", "CS", "3600", "01", "Graphics Programming", ""),
        ("Stander", "Barton", "CS", "3600", "02", "Graphics Programming", "Cancelled"),
        ("Stander", "Barton", "CS", "4550", "01", "Compilers", ""),
        ("Klein", "Lora", "CS", "1410", "1SJ", "Object Oriented Programming", ""),
        ("Klein", "Lora", "IT", "1100", "1SJ", "Introduction to Unix/Linux", ""),
        ("Klein", "Lora", "SE", "4930R", "01", "Sandbox", "Externally scheduled"),
        ("Francom", "Joseph", "IT", "1500", "40A", "Cloud Fundamentals", ""),
        ("Francom", "Joseph", "IT", "3110", "01", "System Automation", ""),
        ("Johnston", "Kevin", "CS", "2320", "01", "Machine Learning", ""),
        ("Daley", "Philip", "IT", "1100", "", "Introduction to Unix/Linux", ""),
        ("Daley", "Philip", "IT", "1100", "", "Introduction to Unix/Linux", ""),
        ("", "", "CS", "1030", "01", "Problem Solving with Computers", ""),
    ]
    content = _xlsx_bytes(rows, struck_rows=frozenset({5}))
    return AssignmentWorkbook(
        rows=parse_assignment_workbook(content),
        source=AssignmentSourceDetails(
            url="https://assignments.test/current.xlsx",
            revision="fixture-revision-1",
            last_modified="Tue, 25 Aug 2026 16:10:11 GMT",
            etag='"fixture-1"',
        ),
    )


@pytest.fixture
def assignment_client(assignment_workbook: AssignmentWorkbook) -> AssignmentWorkbookClient:
    return StaticAssignmentClient(assignment_workbook)


@pytest.fixture
def runtime_directory(tmp_path: Path) -> Iterator[Path]:
    runtime = tmp_path / "runtime"
    yield runtime


def _xlsx_bytes(
    rows: Sequence[tuple[str, ...]],
    *,
    struck_rows: frozenset[int] = frozenset(),
) -> bytes:
    row_xml = []
    for row_number, row in enumerate(rows, start=1):
        cell_xml = []
        for column, value in zip("ABCDEFG", row, strict=True):
            if not value:
                continue
            style = ' s="1"' if row_number in struck_rows and column != "G" else ""
            cell_xml.append(
                f'<c r="{column}{row_number}" t="inlineStr"{style}><is><t>{value}</t></is></c>'
            )
        row_xml.append(f'<row r="{row_number}">{"".join(cell_xml)}</row>')
    worksheet = (
        '<?xml version="1.0" encoding="UTF-8"?>'
        '<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">'
        f"<sheetData>{''.join(row_xml)}</sheetData></worksheet>"
    )
    workbook = (
        '<?xml version="1.0" encoding="UTF-8"?>'
        '<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" '
        'xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">'
        '<sheets><sheet name="Assignments" sheetId="1" r:id="rId1"/></sheets></workbook>'
    )
    relationships = (
        '<?xml version="1.0" encoding="UTF-8"?>'
        "<Relationships "
        'xmlns="http://schemas.openxmlformats.org/package/2006/relationships">'
        '<Relationship Id="rId1" '
        'Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" '
        'Target="worksheets/sheet1.xml"/></Relationships>'
    )
    styles = (
        '<?xml version="1.0" encoding="UTF-8"?>'
        '<styleSheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">'
        '<fonts count="2"><font/><font><strike/></font></fonts>'
        '<cellXfs count="2"><xf fontId="0"/><xf fontId="1" applyFont="1"/></cellXfs>'
        "</styleSheet>"
    )
    output = BytesIO()
    with ZipFile(output, "w", ZIP_DEFLATED) as archive:
        archive.writestr("xl/workbook.xml", workbook)
        archive.writestr("xl/_rels/workbook.xml.rels", relationships)
        archive.writestr("xl/worksheets/sheet1.xml", worksheet)
        archive.writestr("xl/styles.xml", styles)
    return output.getvalue()
