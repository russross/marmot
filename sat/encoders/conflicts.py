# conflicts.py
"""
Conflict constraint encoder.
"""

from data import TimetableData, Conflict, AntiConflict, Priority
from encoding import Encoding


def encode_conflict(
    timetable: TimetableData,
    encoding: Encoding,
    priority: Priority,
    conflict: Conflict
) -> None:
    """
    Encode a single conflict constraint.
    
    A conflict specifies that two sections cannot be scheduled at conflicting times.
    This function creates a hallpass variable and adds clauses to enforce that if 
    both sections are scheduled at conflicting times, the hallpass variable must be 
    true (indicating a violation that is allowed).
    """
    section_a, section_b = conflict.sections
    
    hallpass = encoding.new_var()
    encoding.hallpass.add(hallpass)
    encoding.problems[hallpass] = (priority, f'{section_a} and {section_b} conflict')

    # Verify sections exist
    assert section_a in timetable.sections, f"Section {section_a} in conflict not found"
    assert section_b in timetable.sections, f"Section {section_b} in conflict not found"
    assert section_a != section_b, f'section {section_a} cannot conflict with itself'
    
    # Check each pair of potentially conflicting time slots
    for time_a in timetable.sections[section_a].available_time_slots:
        for time_b in timetable.sections[section_b].available_time_slots:
            # Skip if the time slots don't conflict
            if not timetable.do_time_slots_conflict(time_a, time_b):
                continue
            
            # The variables must exist if we've initialized correctly
            assert (section_a, time_a) in encoding.section_time_vars, f"Missing variable for {section_a}, {time_a}"
            assert (section_b, time_b) in encoding.section_time_vars, f"Missing variable for {section_b}, {time_b}"
            
            var_a = encoding.section_time_vars[(section_a, time_a)]
            var_b = encoding.section_time_vars[(section_b, time_b)]
            
            # Encode: (var_a AND var_b) -> hallpass
            # Equivalent to: (!var_a OR !var_b OR hallpass)
            encoding.add_clause({-var_a, -var_b, hallpass})
