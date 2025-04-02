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
    """
    faculty = constraint.faculty
    days = constraint.days_to_check
    max_gap = constraint.max_gap_within_cluster
    
    # Skip if faculty has only one section (can't have room switches)
    if len(timetable.faculty[faculty].sections) <= 1:
        return
    
    # Create a hallpass variable for this constraint
    hallpass = encoding.new_var()
    encoding.hallpass.add(hallpass)
    encoding.problems[hallpass] = (priority, f'{faculty} should not switch rooms between back-to-back classes')
    
    # Create faculty_room_time variables
    faculty_room_time_vars = make_faculty_room_time_vars(timetable, encoding, faculty, days)
    
    # If no faculty_room_time variables were created, skip this constraint
    # (this happens if faculty has no sections, or no available rooms or time slots)
    if not faculty_room_time_vars:
        return
    
    # Get all back-to-back time slot pairs
    back_to_back_pairs = get_back_to_back_time_slot_pairs(timetable, faculty, days, max_gap)
    
    # For each back-to-back time slot pair:
    for day, (time_slot1, time_slot2) in back_to_back_pairs:
        # Get all possible rooms for these time slots
        rooms1 = {room for (room, ts) in faculty_room_time_vars if ts == time_slot1}
        rooms2 = {room for (room, ts) in faculty_room_time_vars if ts == time_slot2}
        
        # For each pair of different rooms:
        for room1 in rooms1:
            for room2 in rooms2:
                # Skip if it's the same room (no switch)
                if room1 == room2:
                    continue
                
                # Get the corresponding variables
                rt1_var = faculty_room_time_vars.get((room1, time_slot1))
                rt2_var = faculty_room_time_vars.get((room2, time_slot2))
                
                # Skip if either variable doesn't exist (shouldn't happen but to be safe)
                if rt1_var is None or rt2_var is None:
                    continue
                
                # Encode: (faculty_room_time[room1, time1] AND faculty_room_time[room2, time2]) → hallpass
                # Equivalent to: (!faculty_room_time[room1, time1] OR !faculty_room_time[room2, time2] OR hallpass)
                encoding.add_clause({-rt1_var, -rt2_var, hallpass})
