from itertools import product

from typing import Callable

from data import TimetableData, FacultyName, TimeSlotName, SectionName, Priority
from data import Time, Duration, Days, Day
from encoding import Encoding

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
    assert faculty in timetable.faculty, f'faculty {faculty} not found in input data'

    faculty_obj = timetable.faculty[faculty]

    # Get all sections assigned to this faculty
    faculty_sections = list(faculty_obj.sections)

    # If no sections, return an empty set
    assert len(faculty_sections) > 0, f'no sections found for {faculty}'

    # Get time slots for each section
    section_time_slots = []
    for section_name in faculty_sections:
        assert section_name in timetable.sections

        section = timetable.sections[section_name]
        assert section.available_time_slots

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

    # this constraint should not exist if there is nothing to do
    assert section_time_slots

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

def get_faculty_day_patterns(
    timetable: TimetableData,
    faculty: FacultyName,
    days_to_check: Days,
    max_gap_within_cluster: Duration
) -> list[tuple[Day, set[tuple[TimeSlotName, bool]], list[tuple[Time, Time]]]]:
    """
    Generate all possible faculty teaching patterns with day-specific time clusters.
    
    For each day in days_to_check, this function identifies all unique combinations
    of time slots that could be assigned to the faculty on that day, along with
    the resulting time clusters for each combination.
    
    A time cluster is a continuous block of teaching time where gaps between adjacent
    time slots are <= max_gap_within_cluster.
    
    Args:
        timetable: The timetable data containing sections, faculty, and time slots
        faculty: The name of the faculty member
        days_to_check: The set of days to consider
        max_gap_within_cluster: The maximum allowable gap between time slots in a cluster
        
    Returns:
        A list of tuples where each tuple contains:
        - Day: A specific day from days_to_check
        - set[tuple[TimeSlotName, bool]]: Set of (time_slot, is_used) pairs for all relevant time slots
        - list[tuple[Time, Time]]: A list of (start_time, end_time) tuples representing
          teaching clusters for this pattern
    """
    # Validate inputs
    assert faculty in timetable.faculty, f"Faculty {faculty} not found in timetable"
    assert days_to_check, f"Empty days_to_check for faculty {faculty}"
    assert max_gap_within_cluster.minutes >= 0, f"Negative max_gap_within_cluster for faculty {faculty}"
    
    # Generate all valid time slot combinations for this faculty
    all_combos = get_faculty_time_slot_combos(timetable, faculty, days_to_check)
    
    # If no valid combinations, return an empty list
    if not all_combos:
        return []
    
    # Get all potential time slots for this faculty for each day
    day_to_potential_time_slots: dict[Day, set[TimeSlotName]] = {}
    for day in days_to_check:
        day_to_potential_time_slots[day] = set()
        
    for section_name in timetable.faculty[faculty].sections:
        section = timetable.sections[section_name]
        for time_slot_name in section.available_time_slots:
            time_slot = timetable.time_slots[time_slot_name]
            # Add to each day this time slot covers
            for day in days_to_check:
                if day in time_slot.days:
                    day_to_potential_time_slots[day].add(time_slot_name)
    
    # Process each day to find unique time slot patterns and their clusters
    result = []
    
    for day in sorted(days_to_check):
        # Skip days with no potential time slots
        if not day_to_potential_time_slots[day]:
            continue
            
        # Track patterns we've already processed for this day (using frozensets for hashability)
        seen_patterns = set()
        
        # Process each possible faculty time slot combination
        for combo in all_combos:
            # Get the set of time slots used in this combo
            combo_time_slots = {ts for _, ts in combo}
            
            # Create a set of (time_slot, is_used) pairs for all potential time slots on this day
            day_pattern = set()
            for ts_name in day_to_potential_time_slots[day]:
                is_used = ts_name in combo_time_slots
                day_pattern.add((ts_name, is_used))
            
            # Create a hashable representation of this pattern
            pattern_key = frozenset(day_pattern)
            
            # Skip if we've already processed this pattern
            if pattern_key in seen_patterns:
                continue
                
            # Add to seen patterns
            seen_patterns.add(pattern_key)
            
            # Get the set of time slots actually used on this day in this pattern
            used_time_slots = {ts for ts, used in day_pattern if used}
            
            # Skip if no time slots are used on this day
            if not used_time_slots:
                continue
            
            # Get the clusters for this day's time slots
            clusters = get_time_slot_clusters(timetable, used_time_slots, day, max_gap_within_cluster)
            
            # Add this entry to the result
            result.append((day, day_pattern, clusters))
    
    return result

def encode_faculty_cluster_helper(
    timetable: TimetableData,
    encoding: Encoding,
    faculty: FacultyName,
    days: Days,
    max_gap: Duration,
    priority: Priority,
    violation_counter: Callable[[list[tuple[Time, Time]], Day], int],
    description_generator: Callable[[int, Day], str]
) -> None:
    """
    Helper function for encoding faculty cluster constraints.
    
    This function handles the common structure of faculty cluster constraints,
    using callback functions to determine violations based on specific criteria
    and generate appropriate descriptions.
    
    Args:
        timetable: The timetable data
        encoding: The SAT encoding instance
        faculty: The faculty member's name
        days: The set of days to check
        max_gap: Maximum gap duration to consider within a cluster
        priority: The priority level of this constraint
        violation_counter: A callback function that takes a list of clusters (each a
                          tuple of (start_time, end_time)) and a day, and returns the 
                          number of violations detected according to the specific constraint
        description_generator: A callback function that takes a violation number and a day,
                              and returns a descriptive string for the hallpass variable
    """
    # Validate inputs
    assert faculty in timetable.faculty, f"Faculty {faculty} not found in timetable"
    assert days, f"Empty days_to_check for faculty {faculty}"
    assert max_gap.minutes >= 0, f"Negative max gap within cluster for faculty {faculty}"
    assert len(timetable.faculty[faculty].sections) > 1, f'faculty {faculty} must have multiple sections to be concerned about clusters and gaps'
    
    # Create faculty time slot variables for each day
    faculty_time_slot_vars = make_faculty_time_slot_vars(timetable, encoding, faculty, days)
    
    # Skip if no time slots are available for this faculty
    if not faculty_time_slot_vars:
        return
    
    # Dictionary to store hallpass variables: (day, violation_number) -> hallpass_var
    hallpass_vars: dict[tuple[Day, int], int] = {}
    
    # Local helper function to get or create hallpass variables
    def get_hallpass(day: Day, n: int) -> int:
        """Get or create a hallpass variable for a specific day and violation number."""
        if (day, n) in hallpass_vars:
            return hallpass_vars[(day, n)]
        
        # Create a new hallpass variable
        hallpass = encoding.new_var()
        encoding.hallpass.add(hallpass)
        
        # Generate description using the provided callback
        description = description_generator(n, day)
        encoding.problems[hallpass] = (priority, f'{faculty} {description}')
        
        hallpass_vars[(day, n)] = hallpass
        return hallpass
    
    # Get all potential day patterns with clusters
    day_patterns = get_faculty_day_patterns(timetable, faculty, days, max_gap)
    
    # Process each day pattern
    for day, pattern, clusters in day_patterns:
        # Use the provided callback to determine violations
        violation_count = violation_counter(clusters, day)
        
        # Skip if no violations in this pattern
        if violation_count <= 0:
            continue
        
        # Build the base clause (the pattern literals)
        base_clause = set()
        
        # For each (time_slot, is_used) pair in the pattern
        for time_slot_name, is_used in pattern:
            # Skip if this time slot isn't relevant to this faculty on this day
            if time_slot_name not in faculty_time_slot_vars:
                continue
                
            var = faculty_time_slot_vars[time_slot_name]
            
            # Add the appropriate literal to the base clause
            if is_used:
                # If the time slot is used in this pattern, add !var to clause
                base_clause.add(-var)
            else:
                # If the time slot is not used in this pattern, add var to clause
                base_clause.add(var)
        
        # For each violation in this pattern, create a clause
        for i in range(1, violation_count + 1):
            # Get the appropriate hallpass variable
            hallpass = get_hallpass(day, i)
            
            # Create a clause with the base literals plus this hallpass
            clause = base_clause | {hallpass}
            
            # Add the clause to the encoding
            encoding.add_clause(clause)
