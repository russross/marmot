# encoder_types.py
"""
Type definitions for SAT encoders in the Marmot timetabling system.
"""
from typing import Dict, List, Tuple, Protocol, Optional
from pysat.formula import CNF, IDPool # type: ignore
from data import TimetableData

# Common type aliases
SectionName = str
TimeSlotName = str
RoomName = str

# Type for section-time variables mapping
SectionTimeVars = Dict[Tuple[str, str], int]

# Type for section-room variables mapping
SectionRoomVars = Dict[Tuple[str, str], int]

class ConstraintEncoder(Protocol):
    """Protocol defining the interface for constraint encoders."""
    
    def encode(
        self,
        timetable: TimetableData,
        cnf: CNF,
        pool: IDPool,
        section_time_vars: SectionTimeVars,
        section_room_vars: SectionRoomVars,
        priority: int,
        allow_violations: bool = False
    ) -> List[int]:
        """
        Encode constraints at a specific priority level.
        
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
