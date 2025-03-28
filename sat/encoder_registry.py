# encoders.py
from typing import Type, Protocol

from data import TimetableData
from encoder_types import ConstraintEncoder

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
