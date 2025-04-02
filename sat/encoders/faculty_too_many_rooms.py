# encoders/faculty_too_many_rooms.py
"""
Faculty too many rooms constraint encoder for the Marmot timetabling system.

This module provides a function to encode a faculty too many rooms constraint:
ensuring faculty don't teach in more rooms than necessary.
"""

from data import TimetableData, Priority
from data import FacultyTooManyRooms, FacultyName, RoomName
from encoding import Encoding


def encode_faculty_too_many_rooms(
    timetable: TimetableData,
    encoding: Encoding,
    priority: Priority,
    constraint: FacultyTooManyRooms
) -> None:
    """
    Encode a single faculty too many rooms constraint.
    
    A faculty too many rooms constraint specifies that a faculty member should not
    be scheduled in more than a minimum necessary number of rooms. This function 
    creates room usage variables and adds constraints to enforce that the number of 
    rooms used does not exceed the desired maximum, or the hallpass variable must be true.
    
    The encoding works by:
    1. Creating variables representing "this faculty uses this room"
    2. Connecting these variables to the actual section-room assignments
    3. Using an at-most-k constraint to limit the number of room variables that can be true
    """
    faculty = constraint.faculty
    desired_max_rooms = constraint.desired_max_rooms
    
    # Create a hallpass variable for this constraint
    hallpass = encoding.new_var()
    encoding.hallpass.add(hallpass)
    encoding.problems[hallpass] = (priority, f'{faculty} should use at most {desired_max_rooms} room{"" if desired_max_rooms == 1 else "s"}')
    
    # Validate inputs
    assert faculty in timetable.faculty, f"Faculty {faculty} not found in timetable"
    assert desired_max_rooms > 0, f"Non-positive desired_max_rooms for faculty {faculty}"
    
    # Get the faculty's sections
    faculty_sections = timetable.faculty[faculty].sections
    
    # Skip if faculty has no sections
    if not faculty_sections:
        return
    
    # Get all potential rooms this faculty could be assigned
    potential_rooms: set[RoomName] = set()
    for section_name in faculty_sections:
        section = timetable.sections[section_name]
        potential_rooms.update(section.available_rooms)
    
    # Skip if no available rooms or just one room (constraint is trivially satisfied)
    if len(potential_rooms) <= 1:
        return
    
    # Skip if the desired max is greater than or equal to the number of potential rooms
    # (constraint is trivially satisfied)
    if desired_max_rooms >= len(potential_rooms):
        return
    
    # Create variables for "faculty uses this room"
    faculty_room_vars: dict[RoomName, int] = {}
    for room in potential_rooms:
        faculty_room_vars[room] = encoding.new_var()
    
    # For each section, connect the section-room variables to the faculty-room variables
    for section_name in faculty_sections:
        section = timetable.sections[section_name]
        
        for room in section.available_rooms:
            # Get the section-room variable
            section_room_var = encoding.section_room_vars[(section_name, room)]
            
            # Connect: section_room_var -> faculty_room_var
            # If this section is assigned to this room, then the faculty uses this room
            # Equivalent to: !section_room_var OR faculty_room_var
            encoding.add_clause({-section_room_var, faculty_room_vars[room]})
    
    # For each room, connect the faculty-room variable to at least one section-room variable
    for room in potential_rooms:
        # Collect all section-room variables for this room and faculty
        section_room_vars = []
        for section_name in faculty_sections:
            if room in timetable.sections[section_name].available_rooms:
                section_room_vars.append(encoding.section_room_vars[(section_name, room)])
        
        # If no sections can use this room, the faculty-room variable must be false
        if not section_room_vars:
            encoding.add_clause({-faculty_room_vars[room]})
            continue
        
        # faculty_room_var -> (section1_room OR section2_room OR ...)
        # If faculty uses this room, at least one section must be assigned to it
        # Equivalent to: !faculty_room_var OR section1_room OR section2_room OR ...
        encoding.add_clause({-faculty_room_vars[room]} | set(section_room_vars))
    
    # If there are more potential rooms than the desired maximum, apply the constraint
    if len(faculty_room_vars) > desired_max_rooms:
        # Apply the at-most-k constraint using the totalizer encoding
        # We give it our hallpass variable
        encoding.totalizer_at_most_k(set(faculty_room_vars.values()), desired_max_rooms, hallpass)
