# encoders/faculty_evenly_spread.py
"""
Faculty evenly spread constraint encoder for the Marmot timetabling system.

This module provides a function to encode a faculty evenly spread constraint:
ensuring faculty classes are evenly distributed across days with classes.
"""
from pysat.formula import CNF, IDPool  # type: ignore
from typing import Optional

from data import TimetableData, FacultyEvenlySpread, SectionTimeVars
from faculty_utils import get_faculty_section_day_vars


def encode_faculty_evenly_spread(
    timetable: TimetableData,
    cnf: CNF,
    pool: IDPool,
    section_time_vars: SectionTimeVars,
    constraint: FacultyEvenlySpread
) -> int:
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
    
    Args:
        timetable: The timetable data
        cnf: The CNF formula to add clauses to
        pool: The ID pool for variable creation
        section_time_vars: Mapping from (section, time_slot) to variable IDs
        constraint: The specific faculty evenly spread constraint to encode
        
    Returns:
        The hallpass variable that can be set to true to allow a violation
    """
    faculty = constraint.faculty
    days = constraint.days_to_check.days

    # Validate inputs
    assert faculty in timetable.faculty, f"Faculty {faculty} not found in timetable"
    assert days, f"Empty days_to_check for faculty {faculty}"
    assert len(days) > 1, f"Need at least two days to spread out classes for faculty {faculty}"

    # get faculty sections and auxiliary variables
    #   (section_name, day) -> variable
    section_day_to_var = get_faculty_section_day_vars(timetable, cnf, pool, section_time_vars, faculty, days)
    section_day_list = list(section_day_to_var.keys())

    hallpass_var: int = pool.id((faculty, "evenly_spread", days))

    # iterate through a truth table of all 2**n possible section_day combinations
    # note: this could be refined by filtering out the impossible combinations
    for combo in range(2**len(section_day_list)):
        # count the sections on each day for this combo
        scheduled_days = {}
        for (i, (_, day)) in enumerate(section_day_list):
            # is this section_day var true in this combo?
            if combo & (1<<i) != 0:
                if day not in scheduled_days:
                    scheduled_days[day] = 0
                scheduled_days[day] += 1

        # is this combo a violation?
        if scheduled_days:  # Only check if there are scheduled days
            min_sections = float('inf')
            max_sections = 0
            for n in scheduled_days.values():
                min_sections = min(min_sections, n)
                max_sections = max(max_sections, n)
            
            if max_sections - min_sections > 1:
                # encode that this should not happen
                clause = []
                for (i, key) in enumerate(section_day_list):
                    var = section_day_to_var[key]
                    if combo & (1<<i) == 0:
                        clause.append(var)
                    else:
                        clause.append(-var)

                # create the hallpass variable lazily
                clause.append(hallpass_var)
                cnf.append(clause)

    return hallpass_var
