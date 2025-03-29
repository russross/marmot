# encoders/room_pref.py
"""
Room preference constraint encoder for the Marmot timetabling system.

This module provides a function to encode a room preference constraint:
sections should avoid specific rooms if possible.
"""
from pysat.formula import CNF, IDPool # type: ignore

from data import TimetableData, RoomPreference, SectionRoomVars


def encode_room_preference(
    timetable: TimetableData, 
    cnf: CNF,
    pool: IDPool,
    section_room_vars: SectionRoomVars,
    preference: RoomPreference
) -> int:
    """
    Encode a single room preference constraint.
    
    A room preference specifies that a section should avoid a specific room
    if possible. This function creates a hallpass variable and adds clauses to
    enforce that if the section is assigned to the room it should avoid,
    the hallpass variable must be true (indicating a violation that is allowed).
    
    Args:
        timetable: The timetable data
        cnf: The CNF formula to add clauses to
        pool: The ID pool for variable creation
        section_room_vars: Mapping from (section, room) to variable IDs
        preference: The specific room preference to encode
        
    Returns:
        The hallpass variable that can be set to true to allow a violation
    """
    section = preference.section
    room = preference.room
    
    # Verify section and room exist
    assert section in timetable.sections, f"Section {section} in room preference not found"
    assert room in timetable.rooms, f"Room {room} in room preference not found"
    
    # Verify section could be assigned this room
    assert room in timetable.sections[section].available_rooms, \
           f"Room {room} is not available for section {section}"
    
    # The section-room variable must exist if we've initialized correctly
    assert (section, room) in section_room_vars, \
           f"Missing variable for {section}, {room}"
    
    room_var = section_room_vars[(section, room)]
    
    # Create a hallpass variable for this preference
    hallpass_var: int = pool.id(("room_pref", section, room))
    
    # Encode: room_var -> hallpass_var
    # Equivalent to: (!room_var OR hallpass_var)
    cnf.append([-room_var, hallpass_var])
    
    return hallpass_var
