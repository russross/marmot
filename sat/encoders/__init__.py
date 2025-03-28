# encoders/__init__.py
"""
Encoders package for the Marmot timetabling system.

This package contains encoders for various constraints in the timetabling problem.
All encoders are automatically loaded when this package is imported.
"""

# Import all encoder modules to ensure they register with the encoder registry
from . import conflicts
from . import time_pref
# Add other encoder imports here as they are created
