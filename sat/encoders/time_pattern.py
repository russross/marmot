# encoders/time_pattern.py
"""
Time pattern constraint encoder for the Marmot timetabling system.

This module provides a function to encode a time pattern matching constraint:
groups of sections should use the same time pattern (number of days, duration).
"""
from typing import NewType
from itertools import product
import sys

from data import TimetableData, TimePatternMatch, Duration, Priority
from data import TimeSlotName, SectionName
from encoding import Encoding

# Define a Pattern type for clarity
Pattern = NewType('Pattern', tuple[int, Duration])

def make_section_pattern_vars(
    timetable: TimetableData,
    encoding: Encoding,
    sections: frozenset[SectionName]
) -> dict[Pattern, int]:
    """
    Create pattern variables representing "at least one section uses this pattern".
    
    For each distinct time pattern found across all sections, creates a variable that
    will be true if and only if at least one of the specified sections is assigned
    to a time slot with that pattern. Adds clauses connecting section-time variables
    to pattern variables.
    
    Args:
        timetable: The timetable data
        encoding: The SAT encoding instance
        sections: The sections to analyze for patterns
        
    Returns:
        A dictionary mapping patterns to their corresponding SAT variables
    """
    # Maps patterns to their variables
    pattern_to_var: dict[Pattern, int] = {}
    
    # Maps patterns to all section time variables with that pattern
    pattern_to_time_vars: dict[Pattern, list[int]] = {}
    
    # Scan all sections and their time slots
    for section_name in sections:
        section = timetable.sections[section_name]
        
        for time_slot_name in section.available_time_slots:
            # Verify the section-time slot variable exists
            assert (section_name, time_slot_name) in encoding.section_time_vars, \
                   f"Missing variable for {section_name}, {time_slot_name}"
                
            time_slot = timetable.time_slots[time_slot_name]
            pattern = Pattern((len(time_slot.days), time_slot.duration))
            
            # Get or create a variable for this pattern
            if pattern not in pattern_to_var:
                pattern_to_var[pattern] = encoding.new_var()
                pattern_to_time_vars[pattern] = []
            
            # Add this section-time variable to the pattern's list
            time_var = encoding.section_time_vars[(section_name, time_slot_name)]
            pattern_to_time_vars[pattern].append(time_var)
    
    # Add clauses connecting time slot variables to pattern variables
    for pattern, pattern_var in pattern_to_var.items():
        time_vars = pattern_to_time_vars[pattern]
        
        # For each section-time variable with this pattern:
        # time_var → pattern_var
        # Equivalent to: !time_var OR pattern_var
        for time_var in time_vars:
            encoding.add_clause({-time_var, pattern_var})
        
        # pattern_var → (time_var_1 OR time_var_2 OR ...)
        # Equivalent to: !pattern_var OR time_var_1 OR time_var_2 OR ...
        if time_vars:  # Only add if there are time variables
            encoding.add_clause({-pattern_var} | set(time_vars))
    
    return pattern_to_var


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
    
    The encoding uses a direct truth table approach to enforce that exactly one
    pattern can be used across all sections.
    """
    # Skip if fewer than 2 sections (constraint is trivially satisfied)
    if len(constraint.sections) < 2:
        print(f"Warning: TimePatternMatch constraint has fewer than 2 sections: {constraint.sections}", 
              file=sys.stderr)
        return

    # Create the hallpass variable
    hallpass = encoding.new_var()
    encoding.hallpass.add(hallpass)
    encoding.problems[hallpass] = (priority, f'{" and ".join(constraint.sections)} should have the same time pattern')
    
    # Create pattern variables (no hallpass involvement at this stage)
    pattern_to_var = make_section_pattern_vars(
        timetable, encoding, constraint.sections
    )
    
    # Check for potential data issues
    if len(pattern_to_var) < 2:
        print(f"Warning: TimePatternMatch constraint has fewer than 2 patterns for sections: {constraint.sections}", 
              file=sys.stderr)
        return  # If no patterns or only one pattern, constraint is trivially satisfied
    
    # Get the list of pattern variables
    pattern_vars = list(pattern_to_var.values())
    
    # Add constraints enforcing that exactly one pattern is used,
    # or the hallpass is true
    
    # Generate all possible combinations of pattern variable values
    for values in product([True, False], repeat=len(pattern_vars)):
        # Count how many pattern variables are true in this combination
        true_count = sum(values)
        
        # If exactly one pattern variable is true, this is a valid assignment
        if true_count == 1:
            continue
            
        # Otherwise, this combination is invalid without hallpass
        # Add a clause forbidding this combination unless hallpass is true
        clause = {hallpass}
        
        for var, is_true in zip(pattern_vars, values):
            # For each variable, add the appropriate literal to forbid this assignment
            if is_true:
                # If var should be true in this forbidden assignment, add !var to forbid it
                clause.add(-var)
            else:
                # If var should be false in this forbidden assignment, add var to forbid it
                clause.add(var)
        
        encoding.add_clause(clause)
