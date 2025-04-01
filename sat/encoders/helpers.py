from itertools import product

from data import TimetableData, FacultyName, TimeSlotName, SectionName, Priority
from data import Time, Duration, Days, Day
from encoding import Encoding

def make_faculty_section_day_vars(
    timetable: TimetableData,
    encoding: Encoding,
    faculty: FacultyName,
    days_to_check: Days
) -> dict[tuple[SectionName, Day], int]:
    """
    Get or create variables that represent when a faculty member's sections are scheduled on specific days.
    """
    
    # create the set of vars we return
    section_day_to_var: dict[tuple[SectionName, Day], int] = {}

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
                if day not in time_slot.days:
                    continue

                # get a variable for this (section, day)
                if (section_name, day) not in section_day_to_var:
                    var = encoding.new_var()
                    section_day_to_var[(section_name, day)] = var
                    var_to_time_slot_vars[var] = set()

                # record this for encoding
                time_var = encoding.section_time_vars[(section_name, time_slot_name)]
                var_to_time_slot_vars[var].add(time_var)

    for (var, time_slot_vars) in var_to_time_slot_vars.items():
        # encode var -> (time_slot_1 OR time_slot_2 OR ...)
        # i.e. !var OR time_slot_1 OR time_slot_2 OR ...
        encoding.add_clause({-var} | time_slot_vars)

        # encode: (any of the time slots) -> var
        # i.e.: (!time_slot_1 AND !time_slot_2 AND ...) OR section_day_var
        # i.e.: (!time_slot_1 OR var) AND (!time_slot_2 OR var) AND ...
        for time_var in time_slot_vars:
            encoding.add_clause({-time_var, var})
    
    return section_day_to_var

def make_faculty_time_slot_vars(
    timetable: TimetableData,
    encoding: Encoding,
    faculty: FacultyName,
    days_to_check: Days
) -> dict[TimeSlotName, int]:
    """
    Get or create variables that represent when a faculty member is scheduled in specific time slots.
    
    Creates variables for each unique time slot that could be assigned to any of the faculty member's
    sections, but only for time slots that meet on at least one of the days in days_to_check.
    
    Args:
        timetable: The timetable data
        encoding: The SAT encoding instance
        faculty: The faculty name to create variables for
        days_to_check: The set of days to consider
        
    Returns:
        A dictionary mapping time slot names to their corresponding SAT variables
    """
    
    # create the set of vars we return
    time_slot_to_var: dict[TimeSlotName, int] = {}
    
    # create mappings to help with encoding
    var_to_section_time_vars: dict[int, set[int]] = {}
    
    # for each section taught by this faculty
    for section_name in timetable.faculty[faculty].sections:
        section = timetable.sections[section_name]
        
        # for each time slot available to this section
        for time_slot_name in section.available_time_slots:
            time_slot = timetable.time_slots[time_slot_name]
            
            # only consider time slots that meet on at least one day in days_to_check
            if not any(day in time_slot.days for day in days_to_check):
                continue
                
            # get or create a variable for this time slot
            if time_slot_name not in time_slot_to_var:
                var = encoding.new_var()
                time_slot_to_var[time_slot_name] = var
                var_to_section_time_vars[var] = set()
            
            # record section-time variable for encoding
            time_var = encoding.section_time_vars[(section_name, time_slot_name)]
            var_to_section_time_vars[time_slot_to_var[time_slot_name]].add(time_var)
    
    # encode the constraints
    for (var, section_time_vars) in var_to_section_time_vars.items():
        # encode var -> (section1_time OR section2_time OR ...)
        # i.e., !var OR section1_time OR section2_time OR ...
        encoding.add_clause({-var} | section_time_vars)
        
        # encode: (any of the section-time assignments) -> var
        # i.e., (!section1_time AND !section2_time AND ...) OR faculty_time_var
        # i.e., (!section1_time OR var) AND (!section2_time OR var) AND ...
        for section_time_var in section_time_vars:
            encoding.add_clause({-section_time_var, var})
    
    return time_slot_to_var

def get_faculty_time_slot_combos(timetable: TimetableData, faculty: FacultyName, days_to_check: Days) -> set[frozenset[tuple[SectionName, TimeSlotName]]]:
    """
    Generate all possible valid time slot combinations for a faculty member,
    filtered by days.

    This function uses itertools.product to generate all possible ways the
    faculty's sections could be scheduled, then filters to only include
    combinations where:
    1. No time slots conflict with each other
    2. Each time slot meets on at least one of the days in days_to_check

    Args:
        timetable: The timetable data containing sections, faculty, and time slot information
        faculty: The name of the faculty member
        days_to_check: The set of days to consider

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
            # Filter for time slots that include at least one day from days_to_check
            valid_time_slots = []
            for ts_name in section.available_time_slots:
                time_slot = timetable.time_slots[ts_name]
                # Check if this time slot meets on any of the days we're checking
                if any(day in time_slot.days for day in days_to_check):
                    valid_time_slots.append((section_name, ts_name))

            # Only add to section_time_slots if there are valid time slots
            if valid_time_slots:
                section_time_slots.append(valid_time_slots)

    # If any section has no available time slots after filtering, return an empty set
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

def get_unique_section_day_patterns(
    timetable: TimetableData,
    faculty: FacultyName,
    section_day_pairs: list[tuple[SectionName, Day]],
    days_to_check: Days
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
    all_combos = get_faculty_time_slot_combos(timetable, faculty, days_to_check)

    # If no valid combinations, return an empty list
    if not all_combos:
        return []

    # Group the section-day pairs by section for faster lookup
    section_days: dict[SectionName, list[tuple[int, Day]]] = {}
    for idx, (section, day) in enumerate(section_day_pairs):
        if section not in section_days:
            section_days[section] = []
        section_days[section].append((idx, day))

    # Track unique patterns (using tuples for hashability)
    unique_patterns = set()

    # Process each combination
    for combo in all_combos:
        # Create a result array filled with False initially
        result = [False] * len(section_day_pairs)

        # Create section to timeslot mapping for this combo
        section_to_timeslot = {section: timeslot for section, timeslot in combo}

        # Check if all required sections are in this combo
        if not all(section in section_to_timeslot for section in section_days.keys()):
            continue

        # Process each section in the combination
        for section, section_day_indices in section_days.items():
            time_slot_name = section_to_timeslot[section]

            # Get the time slot's days once for each section
            if time_slot_name not in timetable.time_slots:
                # Skip this combo if a time slot is missing
                break

            time_slot_days = timetable.time_slots[time_slot_name].days

            # Check each day for this section and update the result
            for idx, day in section_day_indices:
                result[idx] = day in time_slot_days
        else:  # Only executes if the inner loop didn't break
            # Add this pattern to the unique set (as a tuple for hashability)
            unique_patterns.add(tuple(result))

    # Convert back to lists for the return value
    return [list(pattern) for pattern in unique_patterns]

def get_time_slot_clusters(
    timetable: TimetableData, 
    time_slots: set[TimeSlotName], 
    day: Day, 
    max_gap_within_cluster: Duration
) -> list[tuple[Time, Time]]:
    """
    Find clusters of time slots on a specific day.
    
    A cluster is a group of time slots that are close together in time (the gap between
    adjacent time slots is <= max_gap_within_cluster). This is useful for identifying
    blocks of continuous teaching time for faculty scheduling constraints.
    
    Args:
        timetable: The timetable data containing time slot information
        time_slots: A set of time slot names to consider
        day: The specific day to check for
        max_gap_within_cluster: The maximum allowable gap between time slots in a cluster
        
    Returns:
        A list of (start_time, end_time) tuples representing time clusters, sorted by start time
    """
    # Filter time slots to only those that include the specified day
    day_intervals = []
    
    for ts_name in time_slots:
        # Assert that the time slot exists in the timetable
        assert ts_name in timetable.time_slots, f"Time slot {ts_name} not found in timetable"
            
        time_slot = timetable.time_slots[ts_name]
        
        # Skip if this time slot doesn't include the specified day
        if day not in time_slot.days:
            continue
            
        start_time = time_slot.start_time
        end_time = time_slot.end_time
        
        day_intervals.append((start_time, end_time))
    
    # Sort intervals by start time
    day_intervals.sort()
    
    # Check for overlapping intervals
    for i in range(1, len(day_intervals)):
        prev_end = day_intervals[i-1][1]
        curr_start = day_intervals[i][0]
        
        # Assert directly that there's no overlap
        assert curr_start >= prev_end, f"Overlapping time slots detected on day {day}: {day_intervals[i-1]} and {day_intervals[i]}"
    
    # If no intervals, return empty list
    if not day_intervals:
        return []
    
    # Merge intervals that are close together
    clusters = []
    current_cluster_start, current_cluster_end = day_intervals[0]
    
    for i in range(1, len(day_intervals)):
        current_start, current_end = day_intervals[i]
        
        # Calculate gap as a Duration
        gap = current_start - current_cluster_end
        assert(type(gap) == Duration)
        
        # Compare directly 
        if gap <= max_gap_within_cluster:
            # If gap is small enough, extend the current cluster
            current_cluster_end = current_end
        else:
            # Gap is too large, start a new cluster
            clusters.append((current_cluster_start, current_cluster_end))
            current_cluster_start, current_cluster_end = current_start, current_end
    
    # Add the final cluster
    clusters.append((current_cluster_start, current_cluster_end))
    
    return clusters
