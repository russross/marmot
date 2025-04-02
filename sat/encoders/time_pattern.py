# encoders/time_pattern.py
"""
Time pattern constraint encoder for the Marmot timetabling system.

This module provides a function to encode a time pattern matching constraint:
groups of sections should use the same time pattern (number of days, duration).
"""
from typing import NewType

from data import TimetableData, TimePatternMatch, Duration, Priority
from data import TimeSlotName, SectionName
from encoding import Encoding

# Define a Pattern type for clarity
Pattern = NewType('Pattern', tuple[int, Duration])

def encode_time_pattern_match(
    timetable: TimetableData,
    encoding: Encoding,
    priority: Priority,
    constraint: TimePatternMatch
) -> None:
    """
    Encode a single time pattern match constraint.
    
    A time pattern match constraint specifies that all sections in a group should
    have the same time pattern (number of days per week, duration). This function
    creates a hallpass variable and adds clauses to enforce that if sections in the 
    constraint group are assigned different time patterns, the hallpass variable must
    be true (indicating a violation that is allowed).
    """
    # Skip if fewer than 2 sections (constraint is trivially satisfied)
    if len(constraint.sections) < 2:
        return

    hallpass = encoding.new_var()
    encoding.hallpass.add(hallpass)
    encoding.problems[hallpass] = (priority, f'{" and ".join(constraint.sections)} should have the same time pattern')
        
    # Get all possible time patterns for these sections
    # A time pattern is a tuple of (number of days, duration)
    pattern_to_time_slots: dict[Pattern, set[TimeSlotName]] = {}
    section_to_patterns: dict[SectionName, dict[Pattern, list[tuple[TimeSlotName, int]]]] = {}
    
    # For each section, collect all possible time slots and group by pattern
    for section_name in constraint.sections:
        section = timetable.sections[section_name]
        section_patterns: dict[Pattern, list[tuple[TimeSlotName, int]]] = {}
        
        for time_slot_name in section.available_time_slots:
            time_slot = timetable.time_slots[time_slot_name]
            pattern = Pattern((len(time_slot.days), time_slot.duration))
            
            # Add to the global pattern map
            if pattern not in pattern_to_time_slots:
                pattern_to_time_slots[pattern] = set()
            pattern_to_time_slots[pattern].add(time_slot_name)
            
            # Add to this section's pattern map if we have a variable for it
            if (section_name, time_slot_name) in encoding.section_time_vars:
                if pattern not in section_patterns:
                    section_patterns[pattern] = []
                section_patterns[pattern].append((time_slot_name, encoding.section_time_vars[(section_name, time_slot_name)]))
        
        section_to_patterns[section_name] = section_patterns
    
    # Only one pattern is possible, constraint is trivially satisfied
    if len(pattern_to_time_slots) <= 1:
        return
    
    # Create a variable for each pattern representing "all sections use this pattern"
    pattern_vars: dict[Pattern, int] = {}
    
    for pattern in pattern_to_time_slots:
        pattern_var = encoding.new_var()
        pattern_vars[pattern] = pattern_var
        
        # For each section, if this pattern is selected, the section must use this pattern
        for section_name in constraint.sections:
            if section_name in section_to_patterns and pattern in section_to_patterns[section_name]:
                # Get all time slot variables for this section with this pattern
                pattern_time_vars = {var for _, var in section_to_patterns[section_name][pattern]}
                
                # If this pattern is selected, the section must use one of these time slots
                # pattern_var -> (time_var_1 OR time_var_2 OR ... OR hallpass)
                # Equivalent to: !pattern_var OR time_var_1 OR time_var_2 OR ... OR hallpass
                clause = {-pattern_var} | pattern_time_vars | {hallpass}
                encoding.add_clause(clause)
                
                # For each time slot variable with this pattern:
                # time_var -> (pattern_var OR hallpass)
                # Equivalent to: !time_var OR pattern_var OR hallpass
                for var in pattern_time_vars:
                    encoding.add_clause({-var, pattern_var, hallpass})
            else:
                # This section cannot use this pattern, so this pattern cannot be selected
                # unless hallpass is true
                encoding.add_clause({-pattern_var, hallpass})
    
    # At least one pattern must be selected, or hallpass must be true
    clause = set(pattern_vars.values()) | {hallpass}
    if clause:  # Only add if we have patterns
        encoding.add_clause(clause)
        
        # At most one pattern can be selected, or hallpass must be true
        for i, var1 in enumerate(pattern_vars.values()):
            for var2 in list(pattern_vars.values())[i+1:]:
                encoding.add_clause({-var1, -var2, hallpass})
    
    # For each pair of sections, encode: if they use different patterns, hallpass is true
    for i, section_a in enumerate(constraint.sections):
        for section_b in list(constraint.sections)[i+1:]:
            # For each pattern
            for pattern in pattern_to_time_slots:
                # Skip if either section can't use this pattern
                if (section_a not in section_to_patterns or 
                    pattern not in section_to_patterns[section_a] or
                    section_b not in section_to_patterns or
                    pattern not in section_to_patterns[section_b]):
                    continue
                
                vars_a = [var for _, var in section_to_patterns[section_a][pattern]]
                
                # For each var_a, check against all vars_b of different patterns
                for var_a in vars_a:
                    for other_pattern in pattern_to_time_slots:
                        if pattern == other_pattern:
                            continue
                            
                        if (section_b in section_to_patterns and 
                            other_pattern in section_to_patterns[section_b]):
                            vars_b_other = [var for _, var in section_to_patterns[section_b][other_pattern]]
                            
                            for var_b in vars_b_other:
                                # If section_a uses pattern1 and section_b uses pattern2, hallpass must be true
                                # (var_a AND var_b) -> hallpass
                                # Equivalent to: !var_a OR !var_b OR hallpass
                                encoding.add_clause({-var_a, -var_b, hallpass})
