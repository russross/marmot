# encoders/faculty_days_off.py
"""
Faculty days off constraint encoder for the Marmot timetabling system.

This module provides a function to encode a faculty days off constraint:
ensuring faculty members have a specific number of days without classes.
"""

from itertools import product

from data import TimetableData, FacultyName, TimeSlotName, SectionName, Priority, Day
from data import FacultyDaysOff, FacultyEvenlySpread
from encoding import Encoding

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
    section_day_to_var = encoding.make_faculty_section_day_vars(timetable, faculty, days)
    section_day_list = list(section_day_to_var.keys())

    # iterate through a truth table of section_day combinations that are actually possible
    for combo in get_unique_section_day_patterns(timetable, faculty, section_day_list):
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
    section_day_to_var = encoding.make_faculty_section_day_vars(timetable, faculty, days)
    section_day_list = list(section_day_to_var.keys())

    # iterate through a truth table of section_day combinations that are actually possible
    for combo in get_unique_section_day_patterns(timetable, faculty, section_day_list):
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


def get_faculty_time_slot_combos(timetable: TimetableData, faculty: FacultyName) -> set[frozenset[tuple[SectionName, TimeSlotName]]]:
    """
    Generate all possible valid time slot combinations for a faculty member.
    
    This function uses itertools.product to generate all possible ways the 
    faculty's sections could be scheduled, then filters to only include
    combinations where no time slots conflict with each other.
    
    Args:
        timetable: The timetable data containing sections, faculty, and time slot information
        faculty: The name of the faculty member
    
    Returns:
        A set of frozensets, where each frozenset contains (section, time_slot) pairs
        representing a valid non-conflicting assignment
    """
    # Get the faculty object
    if faculty not in timetable.faculty:
        return set()
    
    faculty_obj = timetable.faculty[faculty]
    
    # Get all sections assigned to this faculty
    faculty_sections = list(faculty_obj.sections)
    
    # If no sections, return an empty set
    if not faculty_sections:
        return set()
    
    # Get time slots for each section
    section_time_slots = []
    for section_name in faculty_sections:
        if section_name not in timetable.sections:
            continue
        
        section = timetable.sections[section_name]
        if section.available_time_slots:
            # Create list of (section_name, time_slot) pairs for this section
            pairs = [(section_name, ts) for ts in section.available_time_slots]
            section_time_slots.append(pairs)
    
    # If any section has no available time slots, return an empty set
    if len(section_time_slots) < len(faculty_sections):
        return set()
    
    # Generate all possible combinations using itertools.product
    valid_combinations = set()
    
    # Generate all combinations
    for combo in product(*section_time_slots):
        # Check if this combination has any conflicts
        has_conflict = False
        
        # Check each pair of time slots for conflicts
        for i in range(len(combo)):
            for j in range(i + 1, len(combo)):
                # Extract the time slot names from the pairs
                _, time_slot_i = combo[i]
                _, time_slot_j = combo[j]
                
                if timetable.do_time_slots_conflict(time_slot_i, time_slot_j):
                    has_conflict = True
                    break
                    
            if has_conflict:
                break
        
        # If no conflicts, add this combination as a frozenset
        if not has_conflict:
            valid_combinations.add(frozenset(combo))
    
    return valid_combinations

def is_section_scheduled_on_day(
    timetable: TimetableData,
    combo: frozenset[tuple[SectionName, TimeSlotName]],
    section_day_pairs: list[tuple[SectionName, Day]]
) -> list[bool]:
    """
    Determine if sections are scheduled on specific days based on a time slot combination.
    
    Args:
        timetable: The timetable data containing time slot information
        combo: A frozenset of (section, time_slot) pairs representing a schedule
        section_day_pairs: A list of (section, day) pairs to check
        
    Returns:
        A list of boolean values in the same order as section_day_pairs,
        where each value indicates if the section is scheduled on that day
        
    Raises:
        AssertionError: If a section in section_day_pairs is not found in the combo,
                       or if a time slot from the combo is not found in timetable.time_slots
    """
    # Create a dictionary mapping from section to its assigned time slot
    section_to_timeslot = {section: timeslot for section, timeslot in combo}
    
    # Check each section-day pair
    results = []
    for section_name, day in section_day_pairs:
        # Assert that the section is in our combination
        assert section_name in section_to_timeslot, f"Section {section_name} not found in the provided combination"
            
        # Get the assigned time slot for this section
        time_slot_name = section_to_timeslot[section_name]
        
        # Assert that the time slot exists in the timetable data
        assert time_slot_name in timetable.time_slots, f"Time slot {time_slot_name} not found in timetable data"
            
        time_slot = timetable.time_slots[time_slot_name]
        
        # Check if the day is in the time slot's days
        results.append(day in time_slot.days)
    
    return results

def get_unique_section_day_patterns(
    timetable: TimetableData,
    faculty: FacultyName,
    section_day_pairs: list[tuple[SectionName, Day]]
) -> list[list[bool]]:
    """
    Generate a list of unique day scheduling patterns for a faculty member.
    
    This function finds all possible valid time slot combinations for a faculty member,
    converts each to a pattern of which sections are scheduled on which days,
    and returns the unique patterns.
    
    Args:
        timetable: The timetable data containing sections, faculty, and time slot information
        faculty: The name of the faculty member
        section_day_pairs: A list of (section, day) pairs to check for scheduling
        
    Returns:
        A list of unique boolean lists, where each list represents a possible
        scheduling pattern for the specified section-day pairs
    """
    # Generate all valid time slot combinations for this faculty
    all_combos = get_faculty_time_slot_combos(timetable, faculty)
    
    # If no valid combinations, return an empty list
    if not all_combos:
        return []
    
    # Convert each combination to a boolean pattern and track unique patterns
    unique_patterns = set()
    
    for combo in all_combos:
        # Get the boolean pattern for this combination
        try:
            bool_pattern = is_section_scheduled_on_day(timetable, combo, section_day_pairs)
            # Convert to tuple for hashing (sets can't contain lists)
            unique_patterns.add(tuple(bool_pattern))
        except AssertionError:
            # Skip combinations that don't include all required sections
            continue
    
    # Convert back to lists for the return value
    return [list(pattern) for pattern in unique_patterns]

