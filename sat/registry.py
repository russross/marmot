# encoder_types.py
"""
Type definitions for SAT encoders in the Marmot timetabling system.
"""
from typing import Protocol, Type
from pysat.formula import CNF, IDPool # type: ignore
from data import TimetableData

# Common type aliases
SectionName = str
TimeSlotName = str
RoomName = str

# Type for section-time variables mapping
SectionTimeVars = dict[tuple[str, str], int]

# Type for section-room variables mapping
SectionRoomVars = dict[tuple[str, str], int]

class ConstraintEncoder(Protocol):
    """Protocol defining the interface for constraint encoders."""
    
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
        Encode constraints at a specific priority level.
        
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


# Registry of constraint encoders by constraint type
_ENCODER_REGISTRY: dict[str, Type[ConstraintEncoder]] = {}

def register_encoder(constraint_type: str, encoder_class: Type[ConstraintEncoder]) -> None:
    """Register a constraint encoder for a specific constraint type."""
    _ENCODER_REGISTRY[constraint_type] = encoder_class

def get_encoder(constraint_type: str) -> Type[ConstraintEncoder]:
    """Get the encoder class for a specific constraint type."""
    if constraint_type not in _ENCODER_REGISTRY:
        raise ValueError(f"No encoder registered for constraint type: {constraint_type}")
    return _ENCODER_REGISTRY[constraint_type]

def get_all_encoders() -> dict[str, Type[ConstraintEncoder]]:
    """Get all registered encoders."""
    return _ENCODER_REGISTRY.copy()
