# core.py
"""
Core SAT encoding for the Marmot timetabling system.

This module provides functions to encode the timetabling problem as a SAT instance.
"""
from typing import Optional, Any, NamedTuple
import collections

from data import TimetableData, ConstraintType
from data import SectionName, RoomName, TimeSlotName, Priority
from data import Conflict, AntiConflict, RoomPreference, TimeSlotPreference, TimePatternMatch
from data import FacultyDaysOff, FacultyEvenlySpread, FacultyNoRoomSwitch, FacultyTooManyRooms
from data import FacultyGapTooLong, FacultyGapTooShort, FacultyClusterTooLong, FacultyClusterTooShort
from data import Placement, Schedule, Score
from encoding import Encoding

def create_sat_instance(
    timetable: TimetableData,
    score_so_far: Score,
    current_priority: Priority,
    current_violations: int
) -> Encoding:
    """
    Create a SAT instance for the timetabling problem.
    """
    # Create CNF formula and ID pool
    encoding = Encoding()
    
    # Create the basic variables
    create_basic_variables(timetable, encoding)
    
    # Encode the basic constraints
    encode_basic_constraints(timetable, encoding)
    
    # Encode room conflicts
    encode_room_conflicts(timetable, encoding)
    
    # Encode all constraints up to and including the current priority level
    for p in range(current_priority + 1):
        priority = Priority(p)

        # Determine max violations allowed for this priority level
        max_violations = score_so_far[priority] if priority < current_priority else current_violations

        # Encode constraints at this priority level
        encode_constraints(timetable, encoding, priority, max_violations)

    # Return the CNF formula and variable mappings
    return encoding

_already_reported: set[ConstraintType] = set()

def encode_constraints(
    timetable: TimetableData,
    encoding: Encoding,
    priority: Priority,
    max_violations: int
) -> None:
    """
    Encode all constraints at a specific priority level.
    """
    # Import constraint encoder functions
    from encoders.conflicts import encode_conflict
    from encoders.anticonflicts import encode_anti_conflict
    from encoders.room_pref import encode_room_preference
    from encoders.time_pref import encode_time_slot_preference
    from encoders.faculty_days_off import encode_faculty_days_off
    from encoders.faculty_evenly_spread import encode_faculty_evenly_spread
    from encoders.time_pattern import encode_time_pattern_match
    from encoders.faculty_cluster_too_long import encode_faculty_cluster_too_long
    from encoders.faculty_cluster_too_short import encode_faculty_cluster_too_short
    from encoders.faculty_gap_too_long import encode_faculty_gap_too_long
    from encoders.faculty_gap_too_short import encode_faculty_gap_too_short
    from encoders.faculty_no_room_switch import encode_faculty_no_room_switch
    from encoders.faculty_too_many_rooms import encode_faculty_too_many_rooms

    # Collect all hallpass variables for this priority level
    encoding.hallpass.clear()

    # Get all constraints at this priority level
    constraints = timetable.get_constraints_by_priority(priority)

    # Encode each constraint based on its type
    for constraint in constraints:
        if isinstance(constraint, Conflict):
            encode_conflict(timetable, encoding, priority, constraint)

        elif isinstance(constraint, AntiConflict):
            encode_anti_conflict(timetable, encoding, priority, constraint)

        elif isinstance(constraint, RoomPreference):
            encode_room_preference(timetable, encoding, priority, constraint)

        elif isinstance(constraint, TimeSlotPreference):
            encode_time_slot_preference(timetable, encoding, priority, constraint)

        elif isinstance(constraint, FacultyDaysOff):
            encode_faculty_days_off(timetable, encoding, priority, constraint)

        elif isinstance(constraint, FacultyEvenlySpread):
            encode_faculty_evenly_spread(timetable, encoding, priority, constraint)

        elif isinstance(constraint, TimePatternMatch):
            encode_time_pattern_match(timetable, encoding, priority, constraint)

        elif isinstance(constraint, FacultyClusterTooLong):
            encode_faculty_cluster_too_long(timetable, encoding, priority, constraint)

        elif isinstance(constraint, FacultyClusterTooShort):
            encode_faculty_cluster_too_short(timetable, encoding, priority, constraint)

        elif isinstance(constraint, FacultyGapTooLong):
            encode_faculty_gap_too_long(timetable, encoding, priority, constraint)

        elif isinstance(constraint, FacultyGapTooShort):
            encode_faculty_gap_too_short(timetable, encoding, priority, constraint)

        elif isinstance(constraint, FacultyNoRoomSwitch):
            encode_faculty_no_room_switch(timetable, encoding, priority, constraint)

        elif isinstance(constraint, FacultyTooManyRooms):
            encode_faculty_too_many_rooms(timetable, encoding, priority, constraint)

        else:
            if constraint not in _already_reported:
                print(f"        Unknown constraint type: {type(constraint).__name__}")
                _already_reported.add(constraint)
            continue

    # Apply cardinality constraint if needed
    if max_violations == 0:
        # No violations allowed: force all hallpass variables to be false
        for var in encoding.hallpass:
            encoding.add_clause({-var})
    elif len(encoding.hallpass) > 0 and max_violations < len(encoding.hallpass):
        # Limited violations allowed: add cardinality constraint
        if max_violations == 1 and len(encoding.hallpass) <= 30:
            encoding.pairwise_at_most_one(encoding.hallpass)
        else:
            encoding.totalizer_at_most_k(encoding.hallpass, max_violations)
    encoding.hallpass.clear()

def create_basic_variables(
    timetable: TimetableData,
    encoding: Encoding,
) -> None:
    """
    Create the basic variables for sections, time slots, and rooms.
    """
    
    # Create section-room variables
    for (section_name, section) in timetable.sections.items():
        for room_name in section.available_rooms:
            encoding.section_room_vars[(section_name, room_name)] = encoding.new_var()
    
    # Create section-time variables
    for section_name, section in timetable.sections.items():
        for time_slot_name in section.available_time_slots:
            encoding.section_time_vars[(section_name, time_slot_name)] = encoding.new_var()


def encode_basic_constraints(
    timetable: TimetableData,
    encoding: Encoding
) -> None:
    """
    Encode the basic constraints of the timetabling problem:
    1. Each section must be assigned exactly one time slot
    2. Each section must be assigned exactly one room (if it has available rooms)
   """
    # Group variables by section for easier processing
    section_to_rooms = collections.defaultdict(set)
    section_to_times = collections.defaultdict(set)
    
    # Organize variables by section
    for (section, room), var in encoding.section_room_vars.items():
        section_to_rooms[section].add(var)
    
    for (section, time_slot), var in encoding.section_time_vars.items():
        section_to_times[section].add(var)
    
    # Constraint 1: Each section must be assigned exactly one room (if it has available rooms)
    for section, room_vars in section_to_rooms.items():
        if not room_vars:
            continue
            
        # At least one room must be assigned
        encoding.add_clause(room_vars)

        # At most one room must be assigned
        encoding.pairwise_at_most_one(room_vars)

    # Constraint 2: Each section must be assigned exactly one time slot
    for section, time_vars in section_to_times.items():
        assert time_vars, f"Section {section} has no available time slots"
        
        # At least one time slot must be assigned
        encoding.add_clause(time_vars)

        # At most one time slot must be assigned
        encoding.pairwise_at_most_one(time_vars)
    

def encode_room_conflicts(
    timetable: TimetableData,
    encoding: Encoding
) -> None:
    """
    Encode the constraint that two sections cannot be in the same room
    at overlapping time slots.
    """
    # Group sections by room
    room_to_sections = collections.defaultdict(set)
    
    for (section, room), _ in encoding.section_room_vars.items():
        room_to_sections[room].add(section)
    
    # For each room, prevent overlapping section assignments
    for room, sections in room_to_sections.items():
        # Skip if only one section can use this room
        if len(sections) < 2:
            continue
        
        # For each pair of sections that could use this room
        lst = list(sections)
        for i in range(len(lst)):
            for j in range(i+1, len(lst)):
                encode_room_conflict(timetable, encoding, lst[i], lst[j], room)


def encode_room_conflict(
    timetable: TimetableData,
    encoding: Encoding,
    section_a: SectionName,
    section_b: SectionName,
    room: RoomName
) -> None:
    """
    Encode the constraint that two sections cannot be in the same room
    at overlapping time slots.
    """
    # Get room variables for both sections - must exist if we've initialized correctly
    assert (section_a, room) in encoding.section_room_vars, f"Missing variable for {section_a}, {room}"
    assert (section_b, room) in encoding.section_room_vars, f"Missing variable for {section_b}, {room}"
    
    room_var_a = encoding.section_room_vars[(section_a, room)]
    room_var_b = encoding.section_room_vars[(section_b, room)]
    
    # Get time slots for both sections
    section_a_time_slots = timetable.sections[section_a].available_time_slots
    section_b_time_slots = timetable.sections[section_b].available_time_slots
    
    # Check each pair of potentially conflicting time slots
    for time_a in section_a_time_slots:
        for time_b in section_b_time_slots:
            # Skip if the time slots don't conflict
            if not timetable.do_time_slots_conflict(time_a, time_b):
                continue
            
            # Get time slot variables - must exist if we've initialized correctly
            assert (section_a, time_a) in encoding.section_time_vars, f"Missing variable for {section_a}, {time_a}"
            assert (section_b, time_b) in encoding.section_time_vars, f"Missing variable for {section_b}, {time_b}"
            
            time_var_a = encoding.section_time_vars[(section_a, time_a)]
            time_var_b = encoding.section_time_vars[(section_b, time_b)]
            
            # Add clause: ~(A_time & A_room & B_time & B_room)
            # Which is equivalent to: (!A_time | !A_room | !B_time | !B_room)
            encoding.add_clause({-time_var_a, -room_var_a, -time_var_b, -room_var_b})


def decode_solution(
    encoding: Encoding,
    model: list[int],
) -> Schedule:
    """
    Decode a SAT solution into a schedule.
    """
    # create reverse lookup tables to find section/room and section/time from var
    var_to_section_room = {v: k for k, v in encoding.section_room_vars.items()}
    var_to_section_time = {v: k for k, v in encoding.section_time_vars.items()}

    section_to_room = {}
    section_to_time_slot = {}
    problems: set[tuple[Priority, str]] = set()

    # Process all positive variable assignments
    for var in model:
        if var <= 0:
            continue

        if var in var_to_section_room:
            (section, room) = var_to_section_room[var]
            section_to_room[section] = room

        elif var in var_to_section_time:
            (section, time_slot) = var_to_section_time[var]
            section_to_time_slot[section] = time_slot

        elif var in encoding.problems:
            problems.add(encoding.problems[var])

    # Construct the placements mapping
    placements = {}
    score = Score()
    
    for section, time_slot in section_to_time_slot.items():
        maybe_room = section_to_room.get(section)  # Will be None if not assigned
        placements[section] = Placement(time_slot=time_slot, room=maybe_room)

    # Update score based on problems
    for priority, _ in problems:
        score.inc_priority(priority)

    # Create and return the Schedule object
    return Schedule(
        placements=placements,
        score=score,
        problems=frozenset(problems)
    )
