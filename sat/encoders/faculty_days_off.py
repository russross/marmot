# encoders/faculty_days_off.py
"""
Faculty days off constraint encoder for the Marmot timetabling system.

This module provides a function to encode a faculty days off constraint:
ensuring faculty members have a specific number of days without classes.
"""

from data import TimetableData, Priority, FacultyName, Day, Days
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
    assert len(timetable.faculty[faculty].sections) > 1, f'Faculty {faculty} must have multiple sections with a days off constraint'
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

    # Generate a truth table of all possible day combinations
    # For n days, there are 2^n possible combinations
    num_days = len(days_list)
    num_combinations = 2**num_days
    
    # Iterate through all possible combinations
    for combo_idx in range(num_combinations):
        # Convert the index to a binary pattern of days
        # For example, with 3 days, combo_idx=5 (binary 101) means
        # days[0] is scheduled, days[1] is not scheduled, days[2] is scheduled
        day_pattern = [(combo_idx >> i) & 1 == 1 for i in range(num_days)]
        
        # Count days off in this pattern
        days_off = day_pattern.count(False)
        
        # If this pattern has the correct number of days off, we're good
        if days_off == desired_days_off:
            continue
            
        # Encode that this pattern should not happen without a hallpass
        clause = {hallpass}
        for i, is_scheduled in enumerate(day_pattern):
            day = days_list[i]
            var = day_to_var[day]
            
            # If the day is scheduled in this pattern, add -var to forbid it
            # If the day is not scheduled in this pattern, add var to forbid it
            if is_scheduled:
                clause.add(-var)
            else:
                clause.add(var)
                
        encoding.add_clause(clause)

def make_faculty_day_vars(
    timetable: TimetableData,
    encoding: Encoding,
    faculty: FacultyName,
    days_to_check: Days
) -> dict[Day, int]:
    """
    Create variables that represent whether a faculty member teaches on specific days.
    
    For each day in days_to_check, creates a variable that will be true if and only if
    at least one of the faculty member's sections is scheduled on that day.
    
    Args:
        timetable: The timetable data
        encoding: The SAT encoding instance
        faculty: The faculty name to create variables for
        days_to_check: The set of days to consider
        
    Returns:
        A dictionary mapping days to their corresponding SAT variables
    """
    # Create the set of day variables we'll return
    day_to_var: dict[Day, int] = {}
    
    # Create mappings to help with encoding
    var_to_section_time_vars: dict[int, set[int]] = {}
    
    # Initialize day variables
    for day in days_to_check:
        var = encoding.new_var()
        day_to_var[day] = var
        var_to_section_time_vars[var] = set()
    
    # For each section taught by this faculty
    for section_name in timetable.faculty[faculty].sections:
        section = timetable.sections[section_name]
        
        # For each day of interest
        for day in days_to_check:
            
            # For each time slot available to this section
            for time_slot_name in section.available_time_slots:
                time_slot = timetable.time_slots[time_slot_name]
                
                # But only the ones that cover this day
                if day not in time_slot.days:
                    continue
                
                # Record this for encoding
                time_var = encoding.section_time_vars[(section_name, time_slot_name)]
                var_to_section_time_vars[day_to_var[day]].add(time_var)
    
    # Add the clauses for each day variable
    for (day_var, section_time_vars) in var_to_section_time_vars.items():
        if not section_time_vars:
            # If there are no possible section-time assignments for this day,
            # this variable must be false
            encoding.add_clause({-day_var})
            continue
        
        # Encode day_var -> (time_slot_1 OR time_slot_2 OR ...)
        # i.e. !day_var OR time_slot_1 OR time_slot_2 OR ...
        encoding.add_clause({-day_var} | section_time_vars)
        
        # Encode: (any of the time slots) -> day_var
        # i.e.: (!time_slot_1 AND !time_slot_2 AND ...) OR day_var
        # i.e.: (!time_slot_1 OR day_var) AND (!time_slot_2 OR day_var) AND ...
        for time_var in section_time_vars:
            encoding.add_clause({-time_var, day_var})
    
    return day_to_var

