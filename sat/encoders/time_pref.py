# encoders/time_pref.py
"""
Time slot preference constraint encoder for the Marmot timetabling system.

This module provides a function to encode a time slot preference constraint:
sections should avoid specific time slots if possible.
"""
from pysat.formula import CNF, IDPool # type: ignore

from data import TimetableData, TimeSlotPreference, SectionTimeVars


def encode_time_slot_preference(
    timetable: TimetableData,
    cnf: CNF,
    pool: IDPool,
    section_time_vars: SectionTimeVars,
    preference: TimeSlotPreference
) -> int:
    """
    Encode a single time slot preference constraint.
    
    A time slot preference specifies that a section should avoid a specific time slot
    if possible. This function creates a hallpass variable and adds clauses to
    enforce that if the section is assigned to the time slot it should avoid,
    the hallpass variable must be true (indicating a violation that is allowed).
    
    Args:
        timetable: The timetable data
        cnf: The CNF formula to add clauses to
        pool: The ID pool for variable creation
        section_time_vars: Mapping from (section, time_slot) to variable IDs
        preference: The specific time slot preference to encode
        
    Returns:
        The hallpass variable that can be set to true to allow a violation
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
    assert (section, time_slot) in section_time_vars, \
           f"Missing variable for {section}, {time_slot}"
    
    time_var = section_time_vars[(section, time_slot)]
    
    # Create a hallpass variable for this preference
    hallpass_var: int = pool.id(("time_pref", section, time_slot))
    
    # Encode: time_var -> hallpass_var
    # Equivalent to: (!time_var OR hallpass_var)
    cnf.append([-time_var, hallpass_var])
    
    return hallpass_var
