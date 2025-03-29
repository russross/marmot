# conflicts.py
"""
Conflict constraint encoders for the Marmot timetabling system.

This module provides functions to encode conflict and anti-conflict constraints.
Each function focuses on encoding a single constraint instance and returns
exactly one hallpass variable.
"""
from pysat.formula import CNF, IDPool # type: ignore

from data import TimetableData, Conflict, AntiConflict, SectionTimeVars


def encode_conflict(
    timetable: TimetableData,
    cnf: CNF,
    pool: IDPool,
    section_time_vars: SectionTimeVars,
    conflict: Conflict
) -> int:
    """
    Encode a single conflict constraint.
    
    A conflict specifies that two sections cannot be scheduled at conflicting times.
    This function creates a hallpass variable and adds clauses to enforce that if 
    both sections are scheduled at conflicting times, the hallpass variable must be 
    true (indicating a violation that is allowed).
    
    Args:
        timetable: The timetable data
        cnf: The CNF formula to add clauses to
        pool: The ID pool for variable creation
        section_time_vars: Mapping from (section, time_slot) to variable IDs
        conflict: The specific conflict to encode
        
    Returns:
        The hallpass variable that can be set to true to allow a violation
    """
    section_a, section_b = conflict.sections
    
    # Verify sections exist
    assert section_a in timetable.sections, f"Section {section_a} in conflict not found"
    assert section_b in timetable.sections, f"Section {section_b} in conflict not found"
    
    # Create a hallpass variable for this conflict
    hallpass_var: int = pool.id(("conflict", section_a, section_b))
    
    # Check each pair of potentially conflicting time slots
    for time_a in timetable.sections[section_a].available_time_slots:
        for time_b in timetable.sections[section_b].available_time_slots:
            # Skip if the time slots don't conflict
            if not timetable.do_time_slots_conflict(time_a, time_b):
                continue
            
            # The variables must exist if we've initialized correctly
            assert (section_a, time_a) in section_time_vars, f"Missing variable for {section_a}, {time_a}"
            assert (section_b, time_b) in section_time_vars, f"Missing variable for {section_b}, {time_b}"
            
            var_a = section_time_vars[(section_a, time_a)]
            var_b = section_time_vars[(section_b, time_b)]
            
            # Encode: (var_a AND var_b) -> hallpass_var
            # Equivalent to: (!var_a OR !var_b OR hallpass_var)
            cnf.append([-var_a, -var_b, hallpass_var])
    
    return hallpass_var
