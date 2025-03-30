# encoders/room_pref.py
"""
Room preference constraint encoder for the Marmot timetabling system.

This module provides a function to encode a room preference constraint:
sections should avoid specific rooms if possible.
"""

from data import TimetableData, RoomPreference
from encoding import Encoding


def encode_room_preference(
    timetable: TimetableData, 
    encoding: Encoding,
    hallpass: int,
    preference: RoomPreference
) -> None:
    """
    Encode a single room preference constraint.
    
    A room preference specifies that a section should avoid a specific room
    if possible. This function creates a hallpass variable and adds clauses to
    enforce that if the section is assigned to the room it should avoid,
    the hallpass variable must be true (indicating a violation that is allowed).
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
    assert (section, room) in encoding.section_room_vars, \
           f"Missing variable for {section}, {room}"
    
    room_var = encoding.section_room_vars[(section, room)]
    
    # Encode: room_var -> hallpass
    # Equivalent to: (!room_var OR hallpass)
    encoding.add_clause([-room_var, hallpass])
