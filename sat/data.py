"""
Data structures for the Marmot SAT-based timetabling system.

This module defines the core data structures that represent the timetabling problem
for encoding into SAT.
"""
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Optional, Set, Tuple, FrozenSet, Any, TypeVar, Union


@dataclass(frozen=True)
class Days:
    """
    Represents a set of days of the week.
    Uses a frozenset for immutability and to allow it to be used as a dictionary key.
    Days are represented as integers 0-6 (Monday-Sunday).
    """
    days: FrozenSet[int]
    
    @staticmethod
    def from_string(day_str: str) -> 'Days':
        """Create Days from a string like 'MWF' or 'TR'"""
        day_map = {'M': 0, 'T': 1, 'W': 2, 'R': 3, 'F': 4, 'S': 5, 'U': 6,
                   'm': 0, 't': 1, 'w': 2, 'r': 3, 'f': 4, 's': 5, 'u': 6}
        return Days(frozenset(day_map[d] for d in day_str if d in day_map))
    
    def __str__(self) -> str:
        day_chars = ['M', 'T', 'W', 'R', 'F', 'S', 'U']
        return ''.join(day_chars[d] for d in sorted(self.days))


@dataclass(frozen=True)
class Time:
    """
    Represents a time of day in minutes from midnight.
    """
    minutes: int
    
    def __str__(self) -> str:
        hours, mins = divmod(self.minutes, 60)
        return f"{hours:02d}:{mins:02d}"
    
    def __add__(self, duration: 'Duration') -> 'Time':
        """Add a duration to this time."""
        return Time(self.minutes + duration.minutes)
    
    def __sub__(self, other: 'Time') -> 'Duration':
        """Subtract another time from this time to get a duration."""
        return Duration(self.minutes - other.minutes)


@dataclass(frozen=True)
class Duration:
    """
    Represents a duration in minutes.
    """
    minutes: int
    
    def __str__(self) -> str:
        if self.minutes == 0:
            return "0m"
        
        result = []
        mins = self.minutes
        
        if mins >= 60:
            hours, mins = divmod(mins, 60)
            result.append(f"{hours}h")
        
        if mins > 0:
            result.append(f"{mins}m")
            
        return "".join(result)


@dataclass(frozen=True)
class Room:
    """
    Represents a room.
    """
    name: str


@dataclass(frozen=True)
class TimeSlot:
    """
    Represents a time slot.
    """
    name: str
    days: Days
    start_time: Time
    duration: Duration
    
    @property
    def end_time(self) -> Time:
        """Calculate the end time of this time slot."""
        return self.start_time + self.duration
    
    @property
    def time_pattern(self) -> Tuple[int, Duration]:
        """Return the time pattern as (number of days, duration)."""
        return (len(self.days.days), self.duration)


@dataclass(frozen=True)
class Section:
    """
    Represents a course section.
    """
    name: str
    
    # Available rooms and time slots for this section
    available_rooms: Set[str] = field(default_factory=set)
    available_time_slots: Set[str] = field(default_factory=set)
    
    # Room preferences (room name -> priority), higher priority is worse
    room_preferences: Dict[str, int] = field(default_factory=dict)
    
    # Time slot preferences (time slot name -> priority), higher priority is worse
    time_slot_preferences: Dict[str, int] = field(default_factory=dict)
    
    # Faculty assigned to this section
    faculty: Set[str] = field(default_factory=set)


@dataclass
class Faculty:
    """
    Represents a faculty member.
    """
    name: str
    
    # Sections assigned to this faculty member
    sections: Set[str] = field(default_factory=set)


class DistributionIntervalType(Enum):
    """Types of distribution intervals for faculty preferences."""
    GAP_TOO_SHORT = auto()
    GAP_TOO_LONG = auto()
    CLUSTER_TOO_SHORT = auto()
    CLUSTER_TOO_LONG = auto()


@dataclass
class Conflict:
    """
    Represents a conflict between two sections.
    """
    sections: Tuple[str, str]  # Ordered pair of section names
    priority: int


@dataclass
class AntiConflict:
    """
    Represents an anti-conflict: single section must be at the same time as at least one
    section from the group.
    """
    single: str
    group: Set[str]
    priority: int


@dataclass
class RoomPreference:
    """
    Represents a preference for a section to avoid a specific room.
    """
    section: str
    room: str
    priority: int


@dataclass
class TimeSlotPreference:
    """
    Represents a preference for a section to avoid a specific time slot.
    """
    section: str
    time_slot: str
    priority: int


@dataclass
class FacultyDaysOff:
    """
    Represents a preference for a faculty member to have a specific number of days off.
    """
    faculty: str
    days_to_check: Days
    desired_days_off: int
    priority: int


@dataclass
class FacultyEvenlySpread:
    """
    Represents a preference for a faculty member's classes to be evenly spread across days.
    """
    faculty: str
    days_to_check: Days
    priority: int


@dataclass
class FacultyNoRoomSwitch:
    """
    Represents a preference for a faculty member to not have to switch rooms between back-to-back classes.
    """
    faculty: str
    days_to_check: Days
    max_gap_within_cluster: Duration
    priority: int


@dataclass
class FacultyTooManyRooms:
    """
    Represents a preference for a faculty member to not have classes in too many different rooms.
    """
    faculty: str
    desired_max_rooms: int
    priority: int


@dataclass
class FacultyDistributionInterval:
    """
    Represents a constraint on time gaps or clusters in a faculty member's schedule.
    """
    faculty: str
    days_to_check: Days
    interval_type: DistributionIntervalType
    duration: Duration
    max_gap_within_cluster: Duration
    priority: int


@dataclass
class TimePatternMatch:
    """
    Represents a constraint that all sections in the group should have the same time pattern.
    """
    sections: Set[str]
    priority: int


# Type for constraints that can be grouped by priority
ConstraintType = Union[
    Conflict, 
    AntiConflict, 
    RoomPreference, 
    TimeSlotPreference,
    FacultyDaysOff, 
    FacultyEvenlySpread, 
    FacultyNoRoomSwitch, 
    FacultyTooManyRooms,
    FacultyDistributionInterval, 
    TimePatternMatch
]


@dataclass
class TimetableData:
    """
    Main container for all timetabling data.
    """
    term_name: str
    
    # Basic entities
    rooms: Dict[str, Room] = field(default_factory=dict)
    time_slots: Dict[str, TimeSlot] = field(default_factory=dict)
    sections: Dict[str, Section] = field(default_factory=dict)
    faculty: Dict[str, Faculty] = field(default_factory=dict)
    
    # Time slot conflicts - maps pairs of time slot names to whether they conflict
    time_slot_conflicts: Dict[Tuple[str, str], bool] = field(default_factory=dict)
    
    # Constraint collections organized by type
    conflicts: List[Conflict] = field(default_factory=list)
    anti_conflicts: List[AntiConflict] = field(default_factory=list)
    room_preferences: List[RoomPreference] = field(default_factory=list)
    time_slot_preferences: List[TimeSlotPreference] = field(default_factory=list)
    faculty_days_off: List[FacultyDaysOff] = field(default_factory=list)
    faculty_evenly_spread: List[FacultyEvenlySpread] = field(default_factory=list)
    faculty_no_room_switch: List[FacultyNoRoomSwitch] = field(default_factory=list)
    faculty_too_many_rooms: List[FacultyTooManyRooms] = field(default_factory=list)
    faculty_distribution_intervals: List[FacultyDistributionInterval] = field(default_factory=list)
    time_pattern_matches: List[TimePatternMatch] = field(default_factory=list)
    
    def do_time_slots_conflict(self, time_slot1: str, time_slot2: str) -> bool:
        """Check if two time slots conflict."""
        key = (time_slot1, time_slot2)
        return self.time_slot_conflicts.get(key, False)
    
    def get_constraints_by_priority(self) -> Dict[int, List[ConstraintType]]:
        """
        Group all constraints by priority level.
        Returns a dictionary mapping priority levels to lists of constraints.
        """
        result: Dict[int, List[ConstraintType]] = {}
        
        # Collect all constraints
        all_constraints: List[ConstraintType] = (
            self.conflicts +
            self.anti_conflicts +
            self.room_preferences +
            self.time_slot_preferences +
            self.faculty_days_off +
            self.faculty_evenly_spread +
            self.faculty_no_room_switch +
            self.faculty_too_many_rooms +
            self.faculty_distribution_intervals +
            self.time_pattern_matches
        )
        
        # Group by priority
        for constraint in all_constraints:
            priority = constraint.priority
            if priority not in result:
                result[priority] = []
            result[priority].append(constraint)
            
        return result
            
    def get_all_constraints(self) -> List[ConstraintType]:
        """Return all constraints as a flat list."""
        return (
            self.conflicts +
            self.anti_conflicts +
            self.room_preferences +
            self.time_slot_preferences +
            self.faculty_days_off +
            self.faculty_evenly_spread +
            self.faculty_no_room_switch +
            self.faculty_too_many_rooms +
            self.faculty_distribution_intervals +
            self.time_pattern_matches
        )
