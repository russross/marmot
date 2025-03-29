# conflicts.py
from pysat.formula import CNF, IDPool # type: ignore

from data import TimetableData, Conflict, AntiConflict
from registry import SectionTimeVars, SectionRoomVars, ConstraintEncoder, register_encoder

class ConflictEncoder(ConstraintEncoder):
    """Encoder for conflict constraints."""
    
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
        Encode conflict constraints at a specific priority level.
        
        Each conflict specifies that two sections cannot be scheduled at conflicting times.
        This encoder creates a criterion variable for each conflict and adds clauses to
        enforce that if both sections are scheduled at conflicting times, the criterion
        variable must be true (indicating a violation).
        
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
        # Get all conflicts at this priority level
        conflicts_at_level = [c for c in timetable.conflicts if c.priority == priority]
        criterion_vars = []

        for conflict in conflicts_at_level:
            section_a, section_b = conflict.sections
            
            # Verify sections exist
            assert section_a in timetable.sections, f"Section {section_a} in conflict not found"
            assert section_b in timetable.sections, f"Section {section_b} in conflict not found"
            
            # Create a criterion variable for this conflict
            criterion_var = pool.id(("conflict", section_a, section_b))
            criterion_vars.append(criterion_var)
            
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
                    
                    # Encode: (var_a AND var_b) -> criterion_var
                    # Equivalent to: (!var_a OR !var_b OR criterion_var)
                    cnf.append([-var_a, -var_b, criterion_var])
        
        return criterion_vars

class AntiConflictEncoder(ConstraintEncoder):
    """Encoder for anti-conflict constraints."""
    
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
        Encode anti-conflict constraints at a specific priority level.
        
        An anti-conflict specifies that a single section must be scheduled at the same time
        as at least one section from a specified group. This encoder creates a criterion 
        variable for each anti-conflict and adds clauses to enforce that if the single section
        is scheduled at a time when no group section is scheduled, the criterion variable 
        must be true (indicating a violation).
        
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
        # Get all anti-conflicts at this priority level
        anti_conflicts_at_level = [c for c in timetable.anti_conflicts if c.priority == priority]
        criterion_vars = []
        
        for anti_conflict in anti_conflicts_at_level:
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
            
            # Create a criterion variable for this anti-conflict constraint
            criterion_var = pool.id(("anti_conflict", single, tuple(sorted(group))))
            criterion_vars.append(criterion_var)
            
            # For each time slot of the single section
            for single_time in timetable.sections[single].available_time_slots:
                # The variable must exist if we've initialized correctly
                assert (single, single_time) in section_time_vars, f"Missing variable for {single}, {single_time}"
                single_var = section_time_vars[(single, single_time)]
                    
                # Find group sections that share this exact time slot
                group_vars = []
                for group_section in group:
                    if single_time in timetable.sections[group_section].available_time_slots:
                        # This variable must exist if we've initialized correctly
                        assert (group_section, single_time) in section_time_vars, \
                               f"Missing variable for {group_section}, {single_time}"
                        group_vars.append(section_time_vars[(group_section, single_time)])
                
                # If no group sections share this time slot
                if not group_vars:
                    # Encode: single_time_var -> criterion_var
                    # Equivalent to: (!single_time_var | criterion_var)
                    cnf.append([-single_var, criterion_var])
                else:
                    # There are some group sections that share this time slot
                    # Encode: single_time_var -> (group_var_1 | group_var_2 | ... | criterion_var)
                    # Equivalent to: (!single_time_var | group_var_1 | group_var_2 | ... | criterion_var)
                    clause = [-single_var] + group_vars + [criterion_var]
                    cnf.append(clause)
        
        return criterion_vars

# Register encoders
register_encoder("Conflict", ConflictEncoder)
register_encoder("AntiConflict", AntiConflictEncoder)
