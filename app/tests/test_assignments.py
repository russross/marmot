from conftest import _xlsx_bytes

from timetable_chat.assignments import parse_assignment_workbook


def test_parser_recovers_assignments_from_a_noisy_planning_sheet() -> None:
    content = _xlsx_bytes(
        [
            ("Spring planning notes", "", "", "", "", "", ""),
            ("Lname", "Fname", "Subject", "Course", "Section", "Title", "Notes"),
            ("", "", "Please resolve the rows below", "", "", "", ""),
            ("Stander", "Barton", "CS", "2420", "01", "Data Structures", ""),
            ("Lname", "Fname", "Subject", "Course", "Section", "Title", "Notes"),
            ("Klein", "Lora", "CS", "", "", "Possible reassignment", ""),
            ("", "", "", "", "", "Last edited Tuesday", ""),
            ("Daley", "Philip", "IT", "1100", "", "Introduction to Unix/Linux", ""),
        ]
    )

    assignments = parse_assignment_workbook(content)

    assert [(row.row, row.course_code, row.faculty_name) for row in assignments] == [
        (4, "CS 2420", "Barton Stander"),
        (8, "IT 1100", "Philip Daley"),
    ]
