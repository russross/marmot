# encoders/faculty_days_off.py
"""
Faculty days off constraint encoder for the Marmot timetabling system.

This module provides a function to encode a faculty days off constraint:
ensuring faculty members have a specific number of days without classes.
"""

from data import TimetableData, Priority
from data import FacultyDaysOff
from encoding import Encoding
from encoders.helpers import *

def encode_faculty_days_off(
    timetable: TimetableData,
    encoding: Encoding,
    priority: Priority,
    constraint: FacultyDaysOff
) -> None:
    """
    Encode a single faculty days off constraint.
    
    A faculty days off constraint specifies that a faculty member should have a 
    specific number of days without classes. This function creates a hallpass variable
    and adds clauses to enforce that if the faculty member's schedule doesn't have
    the desired number of days off, the hallpass variable must be true.
    
    The encoding uses a truth table approach, enumerating all possible day
    patterns and adding CNF clauses to forbid patterns where the number
    of days off differs from the desired amount.
    """
    faculty = constraint.faculty
    days = constraint.days_to_check
    desired_days_off = constraint.desired_days_off

    hallpass = encoding.new_var()
    encoding.hallpass.add(hallpass)
    encoding.problems[hallpass] = (priority, f'{faculty} wants {desired_days_off} day{"" if desired_days_off == 1 else "s"} off')

    # Validate inputs
    assert faculty in timetable.faculty, f"Faculty {faculty} not found in timetable"
    assert days, f"Empty days_to_check for faculty {faculty}"
    assert desired_days_off >= 0, f"Negative desired_days_off for faculty {faculty}"
    assert desired_days_off <= len(days), (
        f"Desired days off {desired_days_off} exceeds possible days "
        f"{len(days)} for faculty {faculty}"
    )

    # Get day auxiliary variables
    #   day -> variable
    day_to_var = make_faculty_day_vars(timetable, encoding, faculty, days)
    days_list = sorted(days)

    # Iterate through a truth table of day combinations that are actually possible
    for combo in get_unique_day_patterns(timetable, faculty, days):
        # Count days off in this pattern
        days_off = combo.count(False)

        # If this pattern has the correct number of days off, we're good
        if days_off == desired_days_off:
            continue

        # Encode that this should not happen without a hallpass
        clause = {hallpass}
        for day, is_scheduled in zip(days_list, combo):
            var = day_to_var[day]
            if is_scheduled:
                clause.add(-var)
            else:
                clause.add(var)

        encoding.add_clause(clause)
