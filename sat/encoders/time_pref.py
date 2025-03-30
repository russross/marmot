# encoders/time_pref.py
"""
Time slot preference constraint encoder for the Marmot timetabling system.

This module provides a function to encode a time slot preference constraint:
sections should avoid specific time slots if possible.
"""

from data import TimetableData, TimeSlotPreference
from encoding import Encoding


def encode_time_slot_preference(
    timetable: TimetableData,
    encoding: Encoding,
    hallpass: int,
    preference: TimeSlotPreference
) -> None:
    """
    Encode a single time slot preference constraint.
    
    A time slot preference specifies that a section should avoid a specific time slot
    if possible. This function creates a hallpass variable and adds clauses to
    enforce that if the section is assigned to the time slot it should avoid,
    the hallpass variable must be true (indicating a violation that is allowed).
    """
    section = preference.section
    time_slot = preference.time_slot
    
    # Verify section and time slot exist
    assert section in timetable.sections, f"Section {section} in time preference not found"
    assert time_slot in timetable.time_slots, f"Time slot {time_slot} in time preference not found"
    
    # Verify section could be assigned this time slot
    assert time_slot in timetable.sections[section].available_time_slots, \
           f"Time slot {time_slot} is not available for section {section}"
    
    # The section-time variable must exist if we've initialized correctly
    assert (section, time_slot) in encoding.section_time_vars, \
           f"Missing variable for {section}, {time_slot}"
    
    time_var = encoding.section_time_vars[(section, time_slot)]
    
    # Encode: time_var -> hallpass
    # Equivalent to: (!time_var OR hallpass)
    encoding.add_clause([-time_var, hallpass])
