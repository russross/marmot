from data import TimetableData, Priority
from data import FacultyEvenlySpread
from encoding import Encoding
from encoders.helpers import *

def encode_faculty_evenly_spread(
    timetable: TimetableData,
    encoding: Encoding,
    priority: Priority,
    constraint: FacultyEvenlySpread
) -> None:
    """
    Encode a single faculty evenly spread constraint.
    
    A faculty evenly spread constraint specifies that a faculty member's classes
    should be evenly distributed across days with classes. This function creates
    a hallpass variable and adds clauses to enforce that if the faculty member's
    schedule isn't evenly spread, the hallpass variable must be true.
    
    The evenly spread constraint is satisfied if (max - min) <= 1 where
    - max is the maximum sections scheduled on a single day
    - min is the minimum (non-zero) sections scheduled on a single day
    
    The encoding uses a truth table approach, enumerating all possible section-day
    assignments and adding CNF clauses to forbid configurations that violate the
    constraint.
    """
    faculty = constraint.faculty
    days = constraint.days_to_check

    hallpass = encoding.new_var()
    encoding.hallpass.add(hallpass)
    encoding.problems[hallpass] = f'{priority}: {faculty} wants sections evenly spread across days'

    # Validate inputs
    assert faculty in timetable.faculty, f"Faculty {faculty} not found in timetable"
    assert days, f"Empty days_to_check for faculty {faculty}"
    assert len(days) > 1, f"Need at least two days to spread out classes for faculty {faculty}"

    # get faculty sections and auxiliary variables
    #   (section_name, day) -> variable
    section_day_to_var = make_faculty_section_day_vars(timetable, encoding, faculty, days)
    section_day_list = list(section_day_to_var.keys())

    # iterate through a truth table of section_day combinations that are actually possible
    for combo in get_unique_section_day_patterns(timetable, faculty, section_day_list, days):
        # figure out which days are scheduled for this combo
        scheduled_days = {}
        for (is_scheduled, (_, day)) in zip(combo, section_day_list):
            if is_scheduled:
                if day not in scheduled_days:
                    scheduled_days[day] = 0
                scheduled_days[day] += 1

        # is this combo a violation?
        if not scheduled_days:  # Only check if there are scheduled days
            continue

        min_sections = float('inf')
        max_sections = 0
        for n in scheduled_days.values():
            min_sections = min(min_sections, n)
            max_sections = max(max_sections, n)
        
        # this is a good spread
        if max_sections - min_sections <= 1:
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
