# encoders/faculty_no_room_switch.py
"""
Faculty no room switch constraint encoder for the Marmot timetabling system.

This module provides a function to encode a faculty no room switch constraint:
ensuring faculty don't have to switch rooms between back-to-back classes.
"""

from itertools import combinations

from data import TimetableData, Priority
from data import FacultyNoRoomSwitch, RoomName, TimeSlotName, SectionName, FacultyName
from data import Duration, Time, Day, Days
from encoding import Encoding


def make_faculty_room_time_vars(
    timetable: TimetableData,
    encoding: Encoding,
    faculty: FacultyName,
    days_to_check: Days
) -> dict[tuple[RoomName, TimeSlotName], int]:
    """
    Create variables that represent if a faculty is teaching in a specific room at a specific time.
    
    These variables are true if and only if at least one section taught by this faculty
    is scheduled in the given room at the given time slot.
    
    Args:
        timetable: The timetable data
        encoding: The SAT encoding instance
        faculty: The faculty name to create variables for
        days_to_check: The set of days to consider
        
    Returns:
        A dictionary mapping (room, time_slot) pairs to their corresponding SAT variables
    """
    # Create the set of variables we'll return
    room_time_to_var: dict[tuple[RoomName, TimeSlotName], int] = {}
    
    # Create mappings to help with encoding
    var_to_section_room_time_pairs: dict[int, set[tuple[int, int]]] = {}
    
    # For each section taught by this faculty
    for section_name in timetable.faculty[faculty].sections:
        section = timetable.sections[section_name]
        
        # For each room available to this section
        for room_name in section.available_rooms:
            # Get the room variable
            if (section_name, room_name) not in encoding.section_room_vars:
                continue
            room_var = encoding.section_room_vars[(section_name, room_name)]
            
            # For each time slot available to this section
            for time_slot_name in section.available_time_slots:
                time_slot = timetable.time_slots[time_slot_name]
                
                # Only consider time slots that meet on at least one day in days_to_check
                if not any(day in time_slot.days for day in days_to_check):
                    continue
                
                # Get the time slot variable
                if (section_name, time_slot_name) not in encoding.section_time_vars:
                    continue
                time_var = encoding.section_time_vars[(section_name, time_slot_name)]
                
                # Get or create a variable for this (room, time_slot) combination
                if (room_name, time_slot_name) not in room_time_to_var:
                    var = encoding.new_var()
                    room_time_to_var[(room_name, time_slot_name)] = var
                    var_to_section_room_time_pairs[var] = set()
                
                # Record this (section_room, section_time) pair for encoding
                var_to_section_room_time_pairs[room_time_to_var[(room_name, time_slot_name)]].add((room_var, time_var))
    
    # Now add the clauses that define these variables
    for frt_var, section_pairs in var_to_section_room_time_pairs.items():
        # For each section's (room_var, time_var) pair:
        # (section_room AND section_time) → faculty_room_time
        # Equivalent to: (!section_room OR !section_time OR faculty_room_time)
        for room_var, time_var in section_pairs:
            encoding.add_clause({-room_var, -time_var, frt_var})
        
        # faculty_room_time → OR(section_room AND section_time)
        # Equivalent to: !faculty_room_time OR OR(section_room AND section_time)
        # This would require a complex clause which we'll break down using auxiliary variables
        # For our purposes, we only need the first implication direction, so we'll skip this
    
    return room_time_to_var


def get_back_to_back_section_pairs(
    timetable: TimetableData,
    faculty: FacultyName,
    days_to_check: Days,
    max_gap: Duration
) -> list[tuple[Day, tuple[SectionName, SectionName], set[RoomName]]]:
    """
    Find all pairs of sections that are back-to-back on the same day for this faculty, 
    along with their common valid rooms.
    
    Args:
        timetable: The timetable data
        faculty: The faculty name
        days_to_check: The set of days to consider
        max_gap: Maximum gap allowed between back-to-back time slots
        
    Returns:
        A list of (day, (section1, section2), common_rooms) tuples where:
        - day is a specific day where both time slots meet
        - section1 ends before section2 starts
        - the gap between them is <= max_gap
        - common_rooms is the set of rooms both sections can use without preference conflicts
    """
    # Validate inputs
    assert faculty in timetable.faculty, f"Faculty {faculty} not found in timetable"
    assert days_to_check, f"Empty days_to_check for faculty {faculty}"
    assert max_gap.minutes >= 0, f"Negative max_gap for faculty {faculty}"
    
    # Get all sections for this faculty
    faculty_sections = list(timetable.faculty[faculty].sections)
    
    # Skip if faculty only has 0 or 1 section
    if len(faculty_sections) <= 1:
        return []
    
    result = []
    
    # For each pair of sections
    for i, section_a in enumerate(faculty_sections):
        for j in range(i+1, len(faculty_sections)):
            section_b = faculty_sections[j]
            
            # Get section objects
            section_a_obj = timetable.sections[section_a]
            section_b_obj = timetable.sections[section_b]
            
            # Find rooms that both sections can use without preferences
            common_rooms = set()
            for room in section_a_obj.available_rooms & section_b_obj.available_rooms:
                # Check if either section has a preference to avoid this room
                has_preference = (
                    room in section_a_obj.room_preferences or 
                    room in section_b_obj.room_preferences
                )
                if not has_preference:
                    common_rooms.add(room)
            
            # Skip if no common valid rooms
            # note: this is legitimate because sections are not required to have rooms
            if not common_rooms:
                continue
            
            # Get all time slots for each section
            a_time_slots = [(ts_name, timetable.time_slots[ts_name]) 
                           for ts_name in section_a_obj.available_time_slots]
            b_time_slots = [(ts_name, timetable.time_slots[ts_name]) 
                           for ts_name in section_b_obj.available_time_slots]
            
            # Check each day in days_to_check
            for day in days_to_check:
                # Get time slots that meet on this day
                a_day_slots = [(ts_name, ts) for ts_name, ts in a_time_slots if day in ts.days]
                b_day_slots = [(ts_name, ts) for ts_name, ts in b_time_slots if day in ts.days]
                
                # Skip if either section has no time slots on this day
                if not a_day_slots or not b_day_slots:
                    continue
                
                # Try all combinations for back-to-back schedulable pairs
                for (ts_a_name, ts_a) in a_day_slots:
                    a_end = ts_a.end_time
                    
                    for (ts_b_name, ts_b) in b_day_slots:
                        b_start = ts_b.start_time
                        
                        # Check if a ends before b starts
                        if a_end <= b_start:
                            # Calculate the gap
                            gap = b_start - a_end
                            assert(type(gap) == Duration)
                            
                            # If the gap is within our max_gap, this pair is back-to-back
                            if gap <= max_gap:
                                result.append((day, (section_a, section_b), common_rooms))
                                # Break after finding first valid pair for these sections on this day
                                break
                    
                    # If we found a pair, no need to check other time slots for section a
                    else:
                        continue
                    break
                
                # Check the reverse order (b before a)
                for (ts_b_name, ts_b) in b_day_slots:
                    b_end = ts_b.end_time
                    
                    for (ts_a_name, ts_a) in a_day_slots:
                        a_start = ts_a.start_time
                        
                        # Check if b ends before a starts
                        if b_end <= a_start:
                            # Calculate the gap
                            gap = a_start - b_end
                            assert(type(gap) == Duration)
                            
                            # If the gap is within our max_gap, this pair is back-to-back
                            if gap <= max_gap:
                                result.append((day, (section_b, section_a), common_rooms))
                                # Break after finding first valid pair for these sections on this day
                                break
                    
                    # If we found a pair, no need to check other time slots for section b
                    else:
                        continue
                    break
    
    return result


def get_back_to_back_time_slot_pairs(
    timetable: TimetableData,
    faculty: FacultyName,
    days_to_check: Days,
    max_gap: Duration
) -> list[tuple[Day, tuple[TimeSlotName, TimeSlotName]]]:
    """
    Find all pairs of time slots that are back-to-back on the same day for this faculty.
    
    Args:
        timetable: The timetable data
        faculty: The faculty name
        days_to_check: The set of days to consider
        max_gap: Maximum gap allowed between back-to-back time slots
        
    Returns:
        A list of (day, (time_slot1, time_slot2)) tuples where:
        - day is a specific day where both time slots meet
        - time_slot1 ends before time_slot2 starts
        - the gap between them is <= max_gap
    """
    # Validate inputs
    assert faculty in timetable.faculty, f"Faculty {faculty} not found in timetable"
    assert days_to_check, f"Empty days_to_check for faculty {faculty}"
    assert max_gap.minutes >= 0, f"Negative max_gap for faculty {faculty}"
    
    # Get all sections for this faculty and their available time slots
    faculty_sections = timetable.faculty[faculty].sections
    
    # Collect all time slots potentially usable by this faculty
    all_time_slots: set[TimeSlotName] = set()
    for section_name in faculty_sections:
        section = timetable.sections[section_name]
        all_time_slots.update(section.available_time_slots)
    
    # Process each day in days_to_check
    result = []
    for day in days_to_check:
        # Collect time slots that meet on this day
        day_time_slots = []
        for ts_name in all_time_slots:
            time_slot = timetable.time_slots[ts_name]
            if day in time_slot.days:
                day_time_slots.append((ts_name, time_slot))
        
        # Sort by start time
        day_time_slots.sort(key=lambda x: x[1].start_time.minutes)
        
        # Find pairs of time slots that are back-to-back
        for i in range(len(day_time_slots) - 1):
            ts1_name, ts1 = day_time_slots[i]
            
            # Check all later time slots to find those that start after ts1 ends
            # but within max_gap
            for j in range(i + 1, len(day_time_slots)):
                ts2_name, ts2 = day_time_slots[j]
                
                # Calculate the gap between ts1 end time and ts2 start time
                gap = ts2.start_time - ts1.end_time
                assert(type(gap) == Duration)
                
                # If the gap is within our max_gap, this pair is back-to-back
                if gap <= max_gap:
                    result.append((day, (ts1_name, ts2_name)))
                
                # Once the gap exceeds max_gap, no need to check further time slots
                # since they're sorted by start time
                else:
                    break
    
    return result


def encode_faculty_no_room_switch(
    timetable: TimetableData,
    encoding: Encoding,
    priority: Priority,
    constraint: FacultyNoRoomSwitch
) -> None:
    """
    Encode a single faculty no room switch constraint.
    
    A faculty no room switch constraint specifies that a faculty member should not
    have to switch rooms between back-to-back classes, where back-to-back means the
    gap between classes is <= max_gap_within_cluster. This function creates a hallpass
    variable and adds clauses to enforce that if the faculty member teaches in different
    rooms in back-to-back time slots, the hallpass variable must be true.
    
    The encoding considers whether the two sections actually have a common room they
    can both use without violating room preferences.
    """
    faculty = constraint.faculty
    days = constraint.days_to_check
    max_gap = constraint.max_gap_within_cluster
    
    # Skip if faculty has only one section (can't have room switches)
    assert len(timetable.faculty[faculty].sections) > 1, f'faculty {faculty} should not have no room switch criterion with < 2 sections'
    
    # Create a hallpass variable for this constraint
    hallpass = encoding.new_var()
    encoding.hallpass.add(hallpass)
    encoding.problems[hallpass] = (priority, f'{faculty} should not switch rooms between back-to-back classes')
    
    # Create faculty_room_time variables
    faculty_room_time_vars = make_faculty_room_time_vars(timetable, encoding, faculty, days)
    
    # If no faculty_room_time variables were created, complain
    # (this happens if faculty has no sections, or no available rooms or time slots)
    assert faculty_room_time_vars, f'{faculty} no room switch constraint found no rooms/time slots to consider'
    
    # Get all back-to-back section pairs with their common valid rooms
    back_to_back_section_pairs = get_back_to_back_section_pairs(timetable, faculty, days, max_gap)
    
    # For each back-to-back section pair:
    for day, (section1, section2), common_rooms in back_to_back_section_pairs:
        # Skip if no common rooms (shouldn't happen due to filtering in get_back_to_back_section_pairs)
        if not common_rooms:
            continue
        
        # Get available time slots for these sections on this day
        section1_time_slots = [ts for ts in timetable.sections[section1].available_time_slots 
                              if day in timetable.time_slots[ts].days]
        section2_time_slots = [ts for ts in timetable.sections[section2].available_time_slots 
                              if day in timetable.time_slots[ts].days]
        
        # For each possible time slot assignment to these sections
        for ts1 in section1_time_slots:
            for ts2 in section2_time_slots:
                # Skip if time slots don't form a back-to-back pair
                if not is_back_to_back(timetable, ts1, ts2, max_gap):
                    continue
                
                # For each possible room assignment, check if using different rooms
                for room1 in timetable.sections[section1].available_rooms:
                    for room2 in timetable.sections[section2].available_rooms:
                        # Skip if it's the same room (no switch)
                        if room1 == room2:
                            continue
                        
                        # Skip if no common valid room exists
                        if not common_rooms:
                            continue
                        
                        # Get section-room and section-time variables
                        sr1_var = encoding.section_room_vars.get((section1, room1))
                        st1_var = encoding.section_time_vars.get((section1, ts1))
                        sr2_var = encoding.section_room_vars.get((section2, room2))
                        st2_var = encoding.section_time_vars.get((section2, ts2))
                        
                        # Skip if any variable doesn't exist
                        if sr1_var is None or st1_var is None or sr2_var is None or st2_var is None:
                            continue
                        
                        # Encode: (sr1 AND st1 AND sr2 AND st2) → hallpass
                        # This means: if section1 is assigned room1 at time1 and section2 is assigned room2 at time2, 
                        # then there must be a hallpass for this constraint
                        # Equivalent to: (!sr1 OR !st1 OR !sr2 OR !st2 OR hallpass)
                        encoding.add_clause({-sr1_var, -st1_var, -sr2_var, -st2_var, hallpass})


def is_back_to_back(
    timetable: TimetableData,
    ts1: TimeSlotName,
    ts2: TimeSlotName,
    max_gap: Duration
) -> bool:
    """
    Check if two time slots are back-to-back (within max_gap of each other).
    
    Args:
        timetable: The timetable data
        ts1: First time slot name
        ts2: Second time slot name
        max_gap: Maximum gap allowed between time slots
        
    Returns:
        True if the time slots are back-to-back, False otherwise
    """
    # Get time slot objects
    time_slot1 = timetable.time_slots[ts1]
    time_slot2 = timetable.time_slots[ts2]
    
    # Get start and end times
    start1 = time_slot1.start_time
    end1 = time_slot1.end_time
    start2 = time_slot2.start_time
    end2 = time_slot2.end_time
    
    # Check if they share any days
    common_days = time_slot1.days & time_slot2.days
    if not common_days:
        return False
    
    # Check if they are back-to-back
    if end1 <= start2:
        gap = start2 - end1
        assert(type(gap) == Duration)
        return gap <= max_gap
    elif end2 <= start1:
        gap = start1 - end2
        assert(type(gap) == Duration)
        return gap <= max_gap
    
    # Overlapping time slots are not back-to-back
    return False
