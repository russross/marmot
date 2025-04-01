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
    
    The encoding uses a truth table approach, enumerating all possible section-day
    assignments and adding CNF clauses to forbid configurations where the number
    of days off differs from the desired amount.
    """
    faculty = constraint.faculty
    days = constraint.days_to_check
    desired_days_off = constraint.desired_days_off

    hallpass = encoding.new_var()
    encoding.hallpass.add(hallpass)
    encoding.problems[hallpass] = f'{priority}: {faculty} wants {desired_days_off} day{"" if desired_days_off == 1 else "s"} off'

    # Validate inputs
    assert faculty in timetable.faculty, f"Faculty {faculty} not found in timetable"
    assert days, f"Empty days_to_check for faculty {faculty}"
    assert desired_days_off >= 0, f"Negative desired_days_off for faculty {faculty}"
    assert desired_days_off <= len(days), (
        f"Desired days off {desired_days_off} exceeds possible days "
        f"{len(days)} for faculty {faculty}"
    )

    # get faculty sections day auxiliary variables
    #   (section_name, day) -> variable
    section_day_to_var = make_faculty_section_day_vars(timetable, encoding, faculty, days)
    section_day_list = list(section_day_to_var.keys())

    # iterate through a truth table of section_day combinations that are actually possible
    for combo in get_unique_section_day_patterns(timetable, faculty, section_day_list, days):
        # figure out which days are scheduled for this combo
        scheduled_days = set()
        for (is_scheduled, (_, day)) in zip(combo, section_day_list):
            if is_scheduled:
                scheduled_days.add(day)

        # is this the right number of days off?
        if len(days) - len(scheduled_days) == desired_days_off:
            continue

        # encode that this should not happen without a hallpass
        clause = {hallpass}
        for (is_scheduled, key) in zip(combo, section_day_list):
            var = section_day_to_var[key]
            if is_scheduled:
                clause.add(-var)
            else:
                clause.add(var)

        encoding.add_clause(clause)
