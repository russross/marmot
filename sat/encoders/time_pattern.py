# time_pattern.py
"""
Time pattern constraint encoder for the Marmot timetabling system.

This module provides encoders to implement constraints for groups of sections
to use the same time pattern (number of days, duration).
"""
from typing import Dict, List, Set, Tuple, FrozenSet
from pysat.formula import CNF, IDPool  # type: ignore

from data import TimetableData, TimePatternMatch, Duration
from registry import SectionTimeVars, SectionRoomVars, ConstraintEncoder, register_encoder


class TimePatternMatchEncoder(ConstraintEncoder):
    """Encoder for time pattern matching constraints."""
    
    def encode(
        self,
        timetable: TimetableData,
        cnf: CNF,
        pool: IDPool,
        section_time_vars: SectionTimeVars,
        section_room_vars: SectionRoomVars,
        priority: int
    ) -> list[int]:
        """
        Encode time pattern match constraints: All sections in the group should
        have the same time pattern (number of days per week, duration).
        
        For each time pattern match constraint, a criterion variable is created.
        The encoding enforces that if sections in the constraint group are assigned
        different time patterns, the criterion variable must be true (indicating a
        violation).
        
        Args:
            timetable: The timetable data
            cnf: The CNF formula to add clauses to
            pool: The ID pool for variable creation
            section_time_vars: Mapping from (section, time_slot) to variable IDs
            section_room_vars: Mapping from (section, room) to variable IDs
            priority: The priority level to encode
            
        Returns:
            List of criterion variables that can be set to true to allow a violation
        """
        # Get all time pattern match constraints at this priority level
        time_pattern_matches = [c for c in timetable.time_pattern_matches if c.priority == priority]
        criterion_vars = []
        
        for constraint in time_pattern_matches:
            # Skip if fewer than 2 sections (constraint is trivially satisfied)
            if len(constraint.sections) < 2:
                continue
                
            # Get all possible time patterns for these sections
            # A time pattern is a tuple of (number of days, duration)
            pattern_to_time_slots: Dict[Tuple[int, Duration], Set[str]] = {}
            section_to_patterns: Dict[str, Dict[Tuple[int, Duration], List[Tuple[str, int]]]] = {}
            
            # For each section, collect all possible time slots and group by pattern
            for section_name in constraint.sections:
                section = timetable.sections[section_name]
                section_patterns: Dict[Tuple[int, Duration], List[Tuple[str, int]]] = {}
                
                for time_slot_name in section.available_time_slots:
                    time_slot = timetable.time_slots[time_slot_name]
                    pattern = time_slot.time_pattern
                    
                    # Add to the global pattern map
                    if pattern not in pattern_to_time_slots:
                        pattern_to_time_slots[pattern] = set()
                    pattern_to_time_slots[pattern].add(time_slot_name)
                    
                    # Add to this section's pattern map if we have a variable for it
                    if (section_name, time_slot_name) in section_time_vars:
                        if pattern not in section_patterns:
                            section_patterns[pattern] = []
                        section_patterns[pattern].append((time_slot_name, section_time_vars[(section_name, time_slot_name)]))
                
                section_to_patterns[section_name] = section_patterns
            
            # Only one pattern is possible, constraint is trivially satisfied
            if len(pattern_to_time_slots) <= 1:
                continue
            
            # Encode the constraint
            criterion_var = self._encode_pattern_matching(
                timetable, cnf, pool, constraint, 
                pattern_to_time_slots, section_to_patterns
            )
            
            criterion_vars.append(criterion_var)
        
        return criterion_vars
    
    def _encode_pattern_matching(
        self,
        timetable: TimetableData,
        cnf: CNF,
        pool: IDPool,
        constraint: TimePatternMatch,
        pattern_to_time_slots: Dict[Tuple[int, Duration], Set[str]],
        section_to_patterns: Dict[str, Dict[Tuple[int, Duration], List[Tuple[str, int]]]]
    ) -> int:
        """
        Encode a specific time pattern matching constraint.
        
        This method creates pattern selection variables and encodes the constraint
        that if a pattern is selected, all sections must use that pattern. If sections
        use different patterns, the criterion variable must be true.
        
        Args:
            timetable: The timetable data
            cnf: The CNF formula to add clauses to
            pool: The ID pool for variable creation
            constraint: The time pattern match constraint
            pattern_to_time_slots: Mapping from pattern to time slots with that pattern
            section_to_patterns: Mapping from section to time patterns and their variables
            
        Returns:
            Criterion variable for this constraint
        """
        # Create a criterion variable for this constraint
        criterion_var: int = pool.id(("time_pattern_match", frozenset(constraint.sections)))
        
        # Create a variable for each pattern representing "all sections use this pattern"
        pattern_vars: Dict[Tuple[int, Duration], int] = {}
        
        for pattern in pattern_to_time_slots:
            # Use frozenset for hashing
            pattern_var = pool.id(("pattern", frozenset(constraint.sections), pattern))
            pattern_vars[pattern] = pattern_var
            
            # For each section, if this pattern is selected, the section must use this pattern
            for section_name in constraint.sections:
                if section_name in section_to_patterns and pattern in section_to_patterns[section_name]:
                    # Get all time slot variables for this section with this pattern
                    pattern_time_vars = [var for _, var in section_to_patterns[section_name][pattern]]
                    
                    # If this pattern is selected, the section must use one of these time slots
                    # pattern_var -> (time_var_1 OR time_var_2 OR ... OR criterion_var)
                    # Equivalent to: !pattern_var OR time_var_1 OR time_var_2 OR ... OR criterion_var
                    clause = [-pattern_var]
                    clause.extend(pattern_time_vars)
                    clause.append(criterion_var)  # Allow violation with criterion_var
                    cnf.append(clause)
                    
                    # For each time slot variable with this pattern:
                    # time_var -> (pattern_var OR criterion_var)
                    # Equivalent to: !time_var OR pattern_var OR criterion_var
                    for var in pattern_time_vars:
                        cnf.append([-var, pattern_var, criterion_var])
                else:
                    # This section cannot use this pattern, so this pattern cannot be selected
                    # unless criterion_var is true
                    cnf.append([-pattern_var, criterion_var])
        
        # At least one pattern must be selected, or criterion_var must be true
        clause = list(pattern_vars.values()) + [criterion_var]
        if clause:  # Only add if we have patterns
            cnf.append(clause)
            
            # At most one pattern can be selected, or criterion_var must be true
            for i, var1 in enumerate(pattern_vars.values()):
                for var2 in list(pattern_vars.values())[i+1:]:
                    cnf.append([-var1, -var2, criterion_var])
        
        # For each pair of sections, encode: if they use different patterns, criterion_var is true
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
                                    # If section_a uses pattern1 and section_b uses pattern2, criterion_var must be true
                                    # (var_a AND var_b) -> criterion_var
                                    # Equivalent to: !var_a OR !var_b OR criterion_var
                                    cnf.append([-var_a, -var_b, criterion_var])
        
        return criterion_var


# Register encoder with the registry
register_encoder("TimePatternMatch", TimePatternMatchEncoder)
