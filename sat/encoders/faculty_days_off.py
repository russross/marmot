# encoders/faculty_days_off.py
"""
Faculty days off constraint encoder for the Marmot timetabling system.

This module provides a function to encode a faculty days off constraint:
ensuring faculty members have a specific number of days without classes.
"""

from data import TimetableData, FacultyDaysOff
from encoding import Encoding
from faculty_utils import get_faculty_section_day_vars


def encode_faculty_days_off(
    timetable: TimetableData,
    encoding: Encoding,
    hallpass: int,
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
    section_day_to_var = get_faculty_section_day_vars(timetable, encoding, faculty, days)
    section_day_list = list(section_day_to_var.keys())
    
    # iterate through a truth table of all 2**n possible section_day combinations
    # note: this could be refined by filtering out the impossible combinations
    for combo in range(2**len(section_day_list)):
        # figure out what days are scheduled for this combo
        scheduled_days = set()
        for (i, (_, day)) in enumerate(section_day_list):
            # is this section_day var true in this combo?
            if combo & (1<<i) != 0:
                scheduled_days.add(day)

        # is this combo a violation?
        if len(days) - len(scheduled_days) != desired_days_off:
            # encode that this should not happen
            clause = []
            for (i, key) in enumerate(section_day_list):
                var = section_day_to_var[key]
                if combo & (1<<i) == 0:
                    clause.append(var)
                else:
                    clause.append(-var)

            # create the hallpass variable lazily
            clause.append(hallpass)
            encoding.add_clause(clause)
