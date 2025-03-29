# encoders/room_pref.py
"""
Room preference constraint encoders for the Marmot timetabling system.

This module provides encoders to implement constraints for sections to avoid
certain rooms with specified priorities.
"""
from pysat.formula import CNF, IDPool # type: ignore

from data import TimetableData, RoomPreference
from registry import SectionTimeVars, SectionRoomVars, ConstraintEncoder, register_encoder


class RoomPreferenceEncoder(ConstraintEncoder):
    """Encoder for room preference constraints."""
    
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
        Encode room preference constraints: sections should avoid specific rooms
        if possible, according to their defined preferences.
        
        For each room preference constraint, a criterion variable is created.
        The encoding enforces that if a section is assigned to a room it should
        avoid, the corresponding criterion variable must be true (indicating a
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
        # Get all room preferences at this priority level
        preferences_at_level = [p for p in timetable.room_preferences 
                               if p.priority == priority]
        criterion_vars = []
        
        for preference in preferences_at_level:
            section = preference.section
            room = preference.room
            
            # Verify section and room exist
            assert section in timetable.sections, f"Section {section} in room preference not found"
            assert room in timetable.rooms, f"Room {room} in room preference not found"
            
            # Verify section could be assigned this room
            assert room in timetable.sections[section].available_rooms, \
                   f"Room {room} is not available for section {section}"
            
            # The section-room variable must exist if we've initialized correctly
            assert (section, room) in section_room_vars, \
                   f"Missing variable for {section}, {room}"
            
            room_var = section_room_vars[(section, room)]
            
            # Create a criterion variable for this preference
            criterion_var = pool.id(("room_pref", section, room))
            criterion_vars.append(criterion_var)
            
            # Encode: room_var -> criterion_var
            # Equivalent to: (!room_var OR criterion_var)
            cnf.append([-room_var, criterion_var])
        
        return criterion_vars


# Register encoder with the registry
register_encoder("RoomPreference", RoomPreferenceEncoder)
