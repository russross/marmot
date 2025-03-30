# conflicts.py
"""
AntiConflict constraint encoders for the Marmot timetabling system.

This module provides functions to encode conflict and anti-conflict constraints.
Each function focuses on encoding a single constraint instance and returns
exactly one hallpass variable.
"""
from data import TimetableData, Conflict, AntiConflict
from encoding import Encoding

def encode_anti_conflict(
    timetable: TimetableData,
    encoding: Encoding,
    hallpass: int,
    anti_conflict: AntiConflict
) -> None:
    """
    Encode a single anti-conflict constraint.
    
    An anti-conflict specifies that a single section must be scheduled at the same time
    as at least one section from a specified group. This function creates a hallpass 
    variable and adds clauses to enforce that if the single section is scheduled at a 
    time when no group section is scheduled, the hallpass variable must be true 
    (indicating a violation that is allowed).
    """
    single = anti_conflict.single
    group = anti_conflict.group
    
    # Verify sections exist and have time slots
    assert single in timetable.sections, f"Single section {single} in anti-conflict not found"
    assert timetable.sections[single].available_time_slots, f"Single section {single} has no available time slots"
    
    for group_section in group:
        assert group_section in timetable.sections, f"Group section {group_section} in anti-conflict not found"
    
    # Verify at least one group section shares a time slot with the single section
    has_shared_time_slot = False
    for single_time in timetable.sections[single].available_time_slots:
        for group_section in group:
            if single_time in timetable.sections[group_section].available_time_slots:
                has_shared_time_slot = True
                break
        if has_shared_time_slot:
            break
    
    assert has_shared_time_slot, f"Anti-conflict for section {single} has no shared time slots with any group section"
    
    # For each time slot of the single section
    for single_time in timetable.sections[single].available_time_slots:
        # The variable must exist if we've initialized correctly
        assert (single, single_time) in encoding.section_time_vars, f"Missing variable for {single}, {single_time}"
        single_var = encoding.section_time_vars[(single, single_time)]
            
        # Find group sections that share this exact time slot
        group_vars = []
        for group_section in group:
            if single_time in timetable.sections[group_section].available_time_slots:
                # This variable must exist if we've initialized correctly
                assert (group_section, single_time) in encoding.section_time_vars, \
                       f"Missing variable for {group_section}, {single_time}"
                group_vars.append(encoding.section_time_vars[(group_section, single_time)])
        
        # If no group sections share this time slot
        if not group_vars:
            # Encode: single_time_var -> hallpass_var
            # Equivalent to: (!single_time_var | hallpass_var)
            encoding.add_clause([-single_var, hallpass])
        else:
            # There are some group sections that share this time slot
            # Encode: single_time_var -> (group_var_1 | group_var_2 | ... | hallpass_var)
            # Equivalent to: (!single_time_var | group_var_1 | group_var_2 | ... | hallpass_var)
            clause = [-single_var] + group_vars + [hallpass]
            encoding.add_clause(clause)
