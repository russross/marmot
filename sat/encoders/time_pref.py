# encoders/time_pref.py
"""
Time slot preference constraint encoders for the Marmot timetabling system.

This module provides encoders to implement constraints for sections to avoid
certain time slots with specified priorities.
"""
from pysat.formula import CNF, IDPool # type: ignore

from data import TimetableData, TimeSlotPreference
from encoder_types import SectionTimeVars, SectionRoomVars, ConstraintEncoder
from encoder_registry import register_encoder


class TimeSlotPreferenceEncoder(ConstraintEncoder):
    """Encoder for time slot preference constraints."""
    
    def encode(
        self,
        timetable: TimetableData,
        cnf: CNF,
        pool: IDPool,
        section_time_vars: SectionTimeVars,
        section_room_vars: SectionRoomVars,
        priority: int,
        allow_violations: bool = False
    ) -> list[int]:
        """
        Encode time slot preference constraints: sections should avoid specific time slots
        if possible, according to their defined preferences.
        
        Args:
            timetable: The timetable data
            cnf: The CNF formula to add clauses to
            pool: The ID pool for variable creation
            section_time_vars: Mapping from (section, time_slot) to variable IDs
            section_room_vars: Mapping from (section, room) to variable IDs
            priority: The priority level to encode
            allow_violations: Whether to allow violations of these constraints
            
        Returns:
            List of criterion variables if violations are allowed, empty list otherwise
        """
        # Get all time slot preferences at this priority level
        preferences_at_level = [p for p in timetable.time_slot_preferences 
                               if p.priority == priority]
        criterion_vars = []
        
        for preference in preferences_at_level:
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
            
            if allow_violations:
                # Create a criterion variable for this preference
                criterion_var = pool.id(("time_pref", section, time_slot))
                criterion_vars.append(criterion_var)
                
                # Encode: time_var -> criterion_var
                # Equivalent to: (!time_var OR criterion_var)
                cnf.append([-time_var, criterion_var])
            else:
                # Hard constraint: section cannot be scheduled at this time slot
                cnf.append([-time_var])
        
        return criterion_vars


# Register encoder with the registry
register_encoder("TimeSlotPreference", TimeSlotPreferenceEncoder)
