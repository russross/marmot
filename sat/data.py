"""
Data structures for the Marmot SAT-based timetabling system.

This module defines the core data structures that represent the timetabling problem
for encoding into SAT.
"""
from dataclasses import dataclass
from typing import Optional, FrozenSet, Union, NewType, Mapping, Iterable

# make some new string types to prevent confusion
SectionName = NewType('SectionName', str)
TimeSlotName = NewType('TimeSlotName', str)
RoomName = NewType('RoomName', str)
FacultyName = NewType('FacultyName', str)
Day = NewType('Day', str)
Priority = NewType('Priority', int)

class Days(frozenset[Day]):
    """Represents a set of days of the week."""
    valid_days = "MTWRFSU"

    def __new__(cls, days: Iterable[str]) -> 'Days':
        return super().__new__(cls, (Day(d) for d in days if d in cls.valid_days))

    @staticmethod
    def from_string(day_str: str) -> 'Days':
        """Create Days from a string like 'MWF' or 'TR'"""
        return Days(day_str.upper())

    def __str__(self) -> str:
        return ''.join(sorted(self, key=lambda d: self.valid_days.index(d)))

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

    # Comparison operators
    def __lt__(self, other: 'Duration') -> bool:
        return self.minutes < other.minutes

    def __le__(self, other: 'Duration') -> bool:
        return self.minutes <= other.minutes

    def __gt__(self, other: 'Duration') -> bool:
        return self.minutes > other.minutes

    def __ge__(self, other: 'Duration') -> bool:
        return self.minutes >= other.minutes

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Duration):
            return NotImplemented
        return self.minutes == other.minutes

@dataclass(frozen=True)
class Time:
    """
    Represents a time of day in minutes from midnight.
    """
    minutes: int

    def __str__(self) -> str:
        hours, mins = divmod(self.minutes, 60)
        return f"{hours:02d}:{mins:02d}"

    def __add__(self, duration: Duration) -> 'Time':
        """Add a duration to this time."""
        return Time(self.minutes + duration.minutes)

    def __sub__(self, other: object) -> Union[Duration, 'Time']:
        """
        Subtract from this time.
        - If subtracting a Time, returns a Duration
        - If subtracting a Duration, returns a Time
        """
        if isinstance(other, Time):
            return Duration(self.minutes - other.minutes)
        elif isinstance(other, Duration):
            return Time(self.minutes - other.minutes)
        return NotImplemented

    # Comparison operators
    def __lt__(self, other: 'Time') -> bool:
        return self.minutes < other.minutes

    def __le__(self, other: 'Time') -> bool:
        return self.minutes <= other.minutes

    def __gt__(self, other: 'Time') -> bool:
        return self.minutes > other.minutes

    def __ge__(self, other: 'Time') -> bool:
        return self.minutes >= other.minutes

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Time):
            return NotImplemented
        return self.minutes == other.minutes


@dataclass(frozen=True)
class Room:
    """
    Represents a room.
    """
    name: RoomName


@dataclass(frozen=True)
class TimeSlot:
    """
    Represents a time slot.
    """
    name: TimeSlotName
    days: Days
    start_time: Time
    duration: Duration
    
    @property
    def end_time(self) -> Time:
        """Calculate the end time of this time slot."""
        return self.start_time + self.duration
    
    @property
    def time_pattern(self) -> tuple[int, Duration]:
        """Return the time pattern as (number of days, duration)."""
        return (len(self.days), self.duration)


@dataclass(frozen=True)
class Section:
    """
    Represents a course section.
    """
    name: SectionName
    available_rooms: FrozenSet[RoomName]
    available_time_slots: FrozenSet[TimeSlotName]
    room_preferences: Mapping[RoomName, Priority]
    time_slot_preferences: Mapping[TimeSlotName, Priority]
    faculty: FrozenSet[FacultyName]


@dataclass(frozen=True)
class Faculty:
    """
    Represents a faculty member.
    """
    name: FacultyName
    sections: FrozenSet[SectionName]


@dataclass(frozen=True)
class Conflict:
    """
    Represents a conflict between two sections.
    """
    sections: tuple[SectionName, SectionName]  # Ordered pair of section names
    priority: Priority


@dataclass(frozen=True)
class AntiConflict:
    """
    Represents an anti-conflict: single section must be at the same time as at least one
    section from the group.
    """
    single: SectionName
    group: FrozenSet[SectionName]
    priority: Priority


@dataclass(frozen=True)
class RoomPreference:
    """
    Represents a preference for a section to avoid a specific room.
    """
    section: SectionName
    room: RoomName
    priority: Priority


@dataclass(frozen=True)
class TimeSlotPreference:
    """
    Represents a preference for a section to avoid a specific time slot.
    """
    section: SectionName
    time_slot: TimeSlotName
    priority: Priority


@dataclass(frozen=True)
class FacultyDaysOff:
    """
    Represents a preference for a faculty member to have a specific number of days off.
    """
    faculty: FacultyName
    days_to_check: Days
    desired_days_off: int
    priority: Priority


@dataclass(frozen=True)
class FacultyEvenlySpread:
    """
    Represents a preference for a faculty member's classes to be evenly spread across days.
    """
    faculty: FacultyName
    days_to_check: Days
    priority: Priority


@dataclass(frozen=True)
class FacultyNoRoomSwitch:
    """
    Represents a preference for a faculty member to not have to switch rooms between back-to-back classes.
    """
    faculty: FacultyName
    days_to_check: Days
    max_gap_within_cluster: Duration
    priority: Priority


@dataclass(frozen=True)
class FacultyTooManyRooms:
    """
    Represents a preference for a faculty member to not have classes in too many different rooms.
    """
    faculty: FacultyName
    desired_max_rooms: int
    priority: Priority


@dataclass(frozen=True)
class FacultyGapTooLong:
    """
    Represents a time gap between class clusters that is too long in a faculty member's schedule.
    """
    faculty: FacultyName
    days_to_check: Days
    duration: Duration
    max_gap_within_cluster: Duration
    priority: Priority


@dataclass(frozen=True)
class FacultyGapTooShort:
    """
    Represents a time gap between class clusters that is too short in a faculty member's schedule.
    """
    faculty: FacultyName
    days_to_check: Days
    duration: Duration
    max_gap_within_cluster: Duration
    priority: Priority


@dataclass(frozen=True)
class FacultyClusterTooLong:
    """
    Represents a cluster of classes that is too long in a faculty member's schedule.
    """
    faculty: FacultyName
    days_to_check: Days
    duration: Duration
    max_gap_within_cluster: Duration
    priority: Priority


@dataclass(frozen=True)
class FacultyClusterTooShort:
    """
    Represents a cluster of classes that is too short in a faculty member's schedule.
    """
    faculty: FacultyName
    days_to_check: Days
    duration: Duration
    max_gap_within_cluster: Duration
    priority: Priority


@dataclass(frozen=True)
class TimePatternMatch:
    """
    Represents a constraint that all sections in the group should have the same time pattern.
    """
    sections: FrozenSet[SectionName]
    priority: Priority


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
    FacultyGapTooLong,
    FacultyGapTooShort,
    FacultyClusterTooLong,
    FacultyClusterTooShort,
    TimePatternMatch
]


@dataclass(frozen=True)
class TimetableData:
    """
    Main container for all timetabling data.
    """
    term_name: str
    rooms: Mapping[RoomName, Room]
    time_slots: Mapping[TimeSlotName, TimeSlot]
    sections: Mapping[SectionName, Section]
    faculty: Mapping[FacultyName, Faculty]
    time_slot_conflicts: FrozenSet[tuple[TimeSlotName, TimeSlotName]]
    conflicts: FrozenSet[Conflict]
    anti_conflicts: FrozenSet[AntiConflict]
    room_preferences: FrozenSet[RoomPreference]
    time_slot_preferences: FrozenSet[TimeSlotPreference]
    faculty_days_off: FrozenSet[FacultyDaysOff]
    faculty_evenly_spread: FrozenSet[FacultyEvenlySpread]
    faculty_no_room_switch: FrozenSet[FacultyNoRoomSwitch]
    faculty_too_many_rooms: FrozenSet[FacultyTooManyRooms]
    faculty_gap_too_short: FrozenSet[FacultyGapTooShort]
    faculty_gap_too_long: FrozenSet[FacultyGapTooLong]
    faculty_cluster_too_short: FrozenSet[FacultyClusterTooShort]
    faculty_cluster_too_long: FrozenSet[FacultyClusterTooLong]
    time_pattern_matches: FrozenSet[TimePatternMatch]
    
    def do_time_slots_conflict(self, time_slot1: TimeSlotName, time_slot2: TimeSlotName) -> bool:
        """Check if two time slots conflict."""
        return (time_slot1, time_slot2) in self.time_slot_conflicts
    
    def get_constraints_by_priority(self, priority: Priority) -> set[ConstraintType]:
        """
        Get the set of all constraints for a given priority level.
        """
        return { elt for elt in self.get_all_constraints() if elt.priority == priority }

    def get_max_priority(self) -> Priority:
        all_constraints = self.get_all_constraints()
        return max((elt.priority for elt in all_constraints), default=Priority(0))
            
    def get_all_constraints(self) -> set[ConstraintType]:
        """Return all constraints as a flat set."""
        return set().union(
            self.conflicts,
            self.anti_conflicts,
            self.room_preferences,
            self.time_slot_preferences,
            self.faculty_days_off,
            self.faculty_evenly_spread,
            self.faculty_no_room_switch,
            self.faculty_too_many_rooms,
            self.faculty_gap_too_short,
            self.faculty_gap_too_long,
            self.faculty_cluster_too_short,
            self.faculty_cluster_too_long,
            self.time_pattern_matches
        )


class Score(list[int]):
    """
    Represents a score as a list of violation counts at each priority level.

    A score is better if it has fewer violations at the highest priority level
    where the scores differ.
    """
    # Maximum number of priority levels
    PRIORITY_LEVELS = 25

    def __init__(self) -> None:
        """Initialize a score with zero violations at all priority levels."""
        super().__init__([0] * self.PRIORITY_LEVELS)

    def inc_priority(self, priority: int) -> None:
        """Increment the violation count at a specific priority level."""
        if 0 <= priority < self.PRIORITY_LEVELS:
            self[priority] += 1
        else:
            raise ValueError(f"Priority level {priority} out of range")

    def dec_priority(self, priority: int) -> None:
        """Decrement the violation count at a specific priority level."""
        if 0 <= priority < self.PRIORITY_LEVELS:
            self[priority] -= 1
        else:
            raise ValueError(f"Priority level {priority} out of range")

    def is_zero(self) -> bool:
        """Check if the score has zero violations at all priority levels."""
        return all(count == 0 for count in self)

    def sortable(self) -> str:
        """
        Generate a sortable string representation of the score.

        Format: <<99×00,98×00,...>> where first number is inverted priority level
        and second is the count of violations at that level.
        Lower values sort first, so better scores come earlier.

        Returns:
            String representation for sorting scores
        """
        if self.is_zero():
            return "<<00:00>>"

        parts = []
        for level, count in enumerate(self):
            if count != 0:
                parts.append(f"{99-level:02}×{count:02}")

        return "<<" + ",".join(parts) + ">>"

    def __str__(self) -> str:
        """
        Generate a human-readable string representation of the score.

        Format: <0×1,5×2> where the first number is priority level and
        the second is the count of violations at that level.

        Returns:
            String representation of the score
        """
        if self.is_zero():
            return "zero"

        parts = []
        for level, count in enumerate(self):
            if count != 0:
                parts.append(f"{level}×{count}")

        return "<" + ",".join(parts) + ">"

@dataclass(frozen=True)
class Placement:
    """
    Represents the placement of a section in a time slot and optionally a room.
    """
    time_slot: TimeSlotName
    room: Optional[RoomName] = None


@dataclass(frozen=True)
class Schedule:
    """
    Represents a complete timetable schedule with assignments and constraint violations.
    """
    placements: Mapping[SectionName, Placement]
    score: Score
    problems: FrozenSet[tuple[Priority, str]]
