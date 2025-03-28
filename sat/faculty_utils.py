"""
Utility functions for faculty-related SAT encodings.

This module provides helper functions to generate and manage SAT variables
related to faculty scheduling constraints.
"""
from typing import Optional, FrozenSet, NamedTuple
from pysat.formula import CNF, IDPool  # type: ignore

from data import TimetableData, Days
from encoder_types import SectionTimeVars, SectionRoomVars

# Type aliases
SectionName = str
TimeSlotName = str
RoomName = str
FacultyName = str
DayNum = int

class FacultyDayVariable(NamedTuple):
    """Represents a faculty-section-day variable in the SAT encoding."""
    faculty: FacultyName
    section: SectionName
    day: DayNum
    purpose: str = "faculty_section_day"

# Cache for faculty day variables
_faculty_day_vars_cache: dict[tuple[FacultyName, FrozenSet[DayNum]], dict[tuple[SectionName, DayNum], int]] = {}

def get_faculty_day_vars(
    timetable: TimetableData,
    cnf: CNF,
    pool: IDPool,
    section_time_vars: SectionTimeVars,
    faculty: FacultyName,
    days_to_check: Days
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
    # Create a cache key from faculty and days
    cache_key = (faculty, frozenset(days_to_check.days))
    
    # Return cached variables if they exist
    if cache_key in _faculty_day_vars_cache:
        return _faculty_day_vars_cache[cache_key]
    
    # Get this faculty's sections
    faculty_sections = timetable.faculty[faculty].sections
    
    # Create variables and constraints for each section and day
    faculty_day_vars: dict[tuple[SectionName, DayNum], int] = {}
    
    for section_name in faculty_sections:
        section = timetable.sections[section_name]
        
        # For each day to check
        for day in days_to_check.days:
            # Create a variable for (section, day)
            var = pool.id((faculty, section_name, day, "faculty_section_day"))
            faculty_day_vars[(section_name, day)] = var
            
            # Find all time slots for this section that include this day
            day_time_slots = []
            for time_slot_name in section.available_time_slots:
                time_slot = timetable.time_slots[time_slot_name]
                if day in time_slot.days.days:
                    # Only consider time slots that are actually available to this section
                    if (section_name, time_slot_name) in section_time_vars:
                        day_time_slots.append(time_slot_name)
            
            # If no time slots include this day for this section, the variable must be false
            if not day_time_slots:
                cnf.append([-var])
                continue
                
            # Encode: section_day_var <-> (time_slot_1 OR time_slot_2 OR ...)
            
            # First part: section_day_var -> (time_slot_1 OR time_slot_2 OR ...)
            # Equivalent to: !section_day_var OR time_slot_1 OR time_slot_2 OR ...
            clause = [-var]
            for time_slot_name in day_time_slots:
                time_var = section_time_vars[(section_name, time_slot_name)]
                clause.append(time_var)
            cnf.append(clause)
            
            # Second part: (time_slot_1 OR time_slot_2 OR ...) -> section_day_var
            # Equivalent to: (!time_slot_1 AND !time_slot_2 AND ...) OR section_day_var
            # Which transforms to: (!time_slot_1 OR section_day_var) AND (!time_slot_2 OR section_day_var) AND ...
            for time_slot_name in day_time_slots:
                time_var = section_time_vars[(section_name, time_slot_name)]
                cnf.append([-time_var, var])
    
    # Cache the variables for future use
    _faculty_day_vars_cache[cache_key] = faculty_day_vars
    
    return faculty_day_vars
