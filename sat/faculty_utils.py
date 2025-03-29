"""
Utility functions for faculty-related SAT encodings.

This module provides helper functions to generate and manage SAT variables
related to faculty scheduling constraints.
"""
from pysat.formula import CNF, IDPool  # type: ignore

from typing import FrozenSet
from data import TimetableData, Days
from registry import SectionTimeVars, SectionRoomVars

# Type aliases
SectionName = str
TimeSlotName = str
RoomName = str
FacultyName = str
DayNum = int

# cache
_faculty_section_day_vars_cache: dict[FacultyName, dict[tuple[SectionName, DayNum], int]] = {}

def get_faculty_section_day_vars(
    timetable: TimetableData,
    cnf: CNF,
    pool: IDPool,
    section_time_vars: SectionTimeVars,
    faculty: FacultyName,
    days_to_check: FrozenSet[DayNum]
) -> dict[tuple[SectionName, DayNum], int]:
    """
    Get or create variables that represent when a faculty member's sections are scheduled on specific days.
    
    Args:
        timetable: The timetable data
        cnf: The CNF formula to add clauses to
        pool: The ID pool for variable creation
        section_time_vars: Mapping from (section, time_slot) to variable IDs
        faculty: The faculty member's name
        days_to_check: The days to consider
        
    Returns:
        A dictionary mapping (section, day) pairs to variable IDs
    """
    # Return cached variables if they exist
    if faculty in _faculty_section_day_vars_cache:
        return _faculty_section_day_vars_cache[faculty]
    
    # create the set of vars we return
    section_day_to_var: dict[tuple[SectionName, DayNum], int] = {}

    # create mappings to help with encoding
    var_to_time_slot_vars: dict[int, set[int]] = {}

    # for each section
    for section_name in timetable.faculty[faculty].sections:
        section = timetable.sections[section_name]

        # for each day of interest
        for day in days_to_check:

            # for each time slot of the section
            for time_slot_name in section.available_time_slots:
                time_slot = timetable.time_slots[time_slot_name]

                # but only the ones that cover this day
                if day not in time_slot.days.days:
                    continue

                # get a variable for this (section, day)
                if (section_name, day) not in section_day_to_var:
                    var = pool.id((faculty, section_name, day, 'faculty_section_day'))
                    section_day_to_var[(section_name, day)] = var
                    var_to_time_slot_vars[var] = set()

                # record this for encoding
                time_var = section_time_vars[(section_name, time_slot_name)]
                var_to_time_slot_vars[var].add(time_var)

    for (var, time_slot_vars) in var_to_time_slot_vars.items():
        # encode var -> (time_slot_1 OR time_slot_2 OR ...)
        # i.e. !var OR time_slot_1 OR time_slot_2 OR ...
        cnf.append([-var] + list(time_slot_vars))

        # encode: (any of the time slots) -> var
        # i.e.: (!time_slot_1 AND !time_slot_2 AND ...) OR section_day_var
        # i.e.: (!time_slot_1 OR var) AND (!time_slot_2 OR var) AND ...
        for time_var in time_slot_vars:
            cnf.append([-time_var, var])
    
    # Cache the variables for future use
    _faculty_section_day_vars_cache[faculty] = section_day_to_var
    
    return section_day_to_var
