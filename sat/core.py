# core.py
"""
Core SAT encoding for the Marmot timetabling system.

This module provides functions to encode the timetabling problem as a SAT instance.
"""
from typing import Optional, Any, NamedTuple
import collections

from pysat.formula import CNF, IDPool # type: ignore
from pysat.card import CardEnc, EncType # type: ignore

from data import TimetableData, ConstraintType, SectionTimeVars, SectionRoomVars
from data import SectionName, RoomName, TimeSlotName, Placement, Priority
from data import Conflict, AntiConflict, RoomPreference, TimeSlotPreference, TimePatternMatch
from data import FacultyDaysOff, FacultyEvenlySpread, FacultyNoRoomSwitch, FacultyTooManyRooms
from data import FacultyGapTooLong, FacultyGapTooShort, FacultyClusterTooLong, FacultyClusterTooShort
from totalizer import totalizer_encode

def create_sat_instance(
    timetable: TimetableData,
    prior_violations: dict[Priority, int],
    current_priority: Priority,
    current_violations: int
) -> tuple[CNF, SectionRoomVars, SectionTimeVars]:
    """
    Create a SAT instance for the timetabling problem.
    
    Args:
        timetable: The timetable data
        prior_violations: Map from priority level to number of allowed violations
        current_priority: The current priority level being solved
        current_violations: Number of violations allowed at current priority level
    
    Returns:
        SAT formula and variable mappings
    """
    # Create CNF formula and ID pool
    cnf = CNF()
    pool = IDPool()
    
    # Create the basic variables
    section_room_vars, section_time_vars = create_basic_variables(timetable, pool)
    
    # Encode the basic constraints
    encode_basic_constraints(timetable, cnf, pool, section_room_vars, section_time_vars)
    
    # Encode room conflicts
    encode_room_conflicts(timetable, cnf, pool, section_room_vars, section_time_vars)
    
    # Encode all constraints up to and including the current priority level
    for p in range(current_priority + 1):
        priority = Priority(p)

        # Determine max violations allowed for this priority level
        max_violations = prior_violations.get(priority, 0) if priority < current_priority else current_violations

        # Encode constraints at this priority level
        encode_constraints(timetable, cnf, pool, section_room_vars, section_time_vars, priority, max_violations)

    # Return the CNF formula and variable mappings
    return (cnf, section_room_vars, section_time_vars)

_already_reported = set()

def encode_constraints(
    timetable: TimetableData,
    cnf: CNF,
    pool: IDPool,
    section_room_vars: SectionRoomVars,
    section_time_vars: SectionTimeVars,
    priority: Priority,
    max_violations: int
) -> None:
    """
    Encode all constraints at a specific priority level.

    Args:
        timetable: The timetable data
        cnf: The CNF formula to add clauses to
        pool: The ID pool for variable creation
        section_room_vars: Mapping from (section, room) to variable IDs
        section_time_vars: Mapping from (section, time_slot) to variable IDs
        priority: The priority level to encode
        max_violations: Maximum allowed violations at this priority level
    """
    # Import constraint encoder functions
    from encoders.conflicts import encode_conflict
    from encoders.anticonflicts import encode_anti_conflict
    from encoders.room_pref import encode_room_preference
    from encoders.time_pref import encode_time_slot_preference
    from encoders.faculty_days_off import encode_faculty_days_off
    from encoders.faculty_evenly_spread import encode_faculty_evenly_spread
    from encoders.time_pattern import encode_time_pattern_match

    # Collect all hallpass variables for this priority level
    hallpass_vars = []

    # Get all constraints at this priority level
    constraints = timetable.get_constraints_by_priority(priority)

    # Encode each constraint based on its type
    for constraint in constraints:
        if isinstance(constraint, Conflict):
            hallpass_var = encode_conflict(timetable, cnf, pool, section_time_vars, constraint)

        elif isinstance(constraint, AntiConflict):
            hallpass_var = encode_anti_conflict(timetable, cnf, pool, section_time_vars, constraint)

        elif isinstance(constraint, RoomPreference):
            hallpass_var = encode_room_preference(timetable, cnf, pool, section_room_vars, constraint)

        elif isinstance(constraint, TimeSlotPreference):
            hallpass_var = encode_time_slot_preference(timetable, cnf, pool, section_time_vars, constraint)

        elif isinstance(constraint, FacultyDaysOff):
            hallpass_var = encode_faculty_days_off(timetable, cnf, pool, section_time_vars, constraint)
            continue

        elif isinstance(constraint, FacultyEvenlySpread):
            hallpass_var = encode_faculty_evenly_spread(timetable, cnf, pool, section_time_vars, constraint)
            continue

        elif isinstance(constraint, TimePatternMatch):
            hallpass_var = encode_time_pattern_match(timetable, cnf, pool, section_time_vars, constraint)

        # Add cases for other constraint types as they are implemented
        elif isinstance(constraint, FacultyNoRoomSwitch):
            if constraint not in _already_reported:
                print(f"        Skipping unimplemented constraint: FacultyNoRoomSwitch for {constraint.faculty}")
                _already_reported.add(constraint)
            continue

        elif isinstance(constraint, FacultyTooManyRooms):
            if constraint not in _already_reported:
                print(f"        Skipping unimplemented constraint: FacultyTooManyRooms for {constraint.faculty}")
                _already_reported.add(constraint)
            continue

        elif isinstance(constraint, FacultyGapTooLong):
            if constraint not in _already_reported:
                print(f"        Skipping unimplemented constraint: FacultyGapTooLong for {constraint.faculty}")
                _already_reported.add(constraint)
            continue

        elif isinstance(constraint, FacultyGapTooShort):
            if constraint not in _already_reported:
                print(f"        Skipping unimplemented constraint: FacultyGapTooShort for {constraint.faculty}")
                _already_reported.add(constraint)
            continue

        elif isinstance(constraint, FacultyClusterTooLong):
            if constraint not in _already_reported:
                print(f"        Skipping unimplemented constraint: FacultyClusterTooLong for {constraint.faculty}")
                _already_reported.add(constraint)
            continue

        elif isinstance(constraint, FacultyClusterTooShort):
            if constraint not in _already_reported:
                print(f"        Skipping unimplemented constraint: FacultyClusterTooShort for {constraint.faculty}")
                _already_reported.add(constraint)
            continue

        else:
            if constraint not in _already_reported:
                print(f"        Unknown constraint type: {type(constraint).__name__}")
                _already_reported.add(constraint)
            continue

        # Add hallpass variable to the list if a valid one was returned
        if hallpass_var > 0:
            hallpass_vars.append(hallpass_var)

    # Apply cardinality constraint if needed
    if max_violations == 0:
        # No violations allowed: force all hallpass variables to be false
        for var in hallpass_vars:
            cnf.append([-var])
    elif hallpass_vars and max_violations < len(hallpass_vars):
        # Limited violations allowed: add cardinality constraint
        if max_violations == 1 and len(hallpass_vars) <= 20:
            clauses = CardEnc.atmost(hallpass_vars, vpool=pool, bound=max_violations, encoding=EncType.pairwise).clauses
        else:
            clauses = totalizer_encode(hallpass_vars, max_violations, pool)
        for clause in clauses:
            cnf.append(clause)

def create_basic_variables(
    timetable: TimetableData,
    pool: IDPool
) -> tuple[SectionRoomVars, SectionTimeVars]:
    """
    Create the basic variables for sections, time slots, and rooms.
    
    Args:
        timetable: The timetable data
        pool: The ID pool for variable creation
        
    Returns:
        section_room_vars: Mapping from (section, room) to variable IDs
        section_time_vars: Mapping from (section, time_slot) to variable IDs
    """
    
    # Create section-room variables
    section_room_vars = SectionRoomVars({})
    for (section_name, section) in timetable.sections.items():
        for room_name in section.available_rooms:
            room_var: int = pool.id((section_name, room_name, "section_room"))
            section_room_vars[(section_name, room_name)] = room_var
    
    # Create section-time variables
    section_time_vars = SectionTimeVars({})
    for section_name, section in timetable.sections.items():
        for time_slot_name in section.available_time_slots:
            time_var: int = pool.id((section_name, time_slot_name, "section_time"))
            section_time_vars[(section_name, time_slot_name)] = time_var
    
    return section_room_vars, section_time_vars


def encode_basic_constraints(
    timetable: TimetableData,
    cnf: CNF,
    pool: IDPool,
    section_room_vars: SectionRoomVars,
    section_time_vars: SectionTimeVars
) -> None:
    """
    Encode the basic constraints of the timetabling problem:
    1. Each section must be assigned exactly one time slot
    2. Each section must be assigned exactly one room (if it has available rooms)
    
    Args:
        timetable: The timetable data
        cnf: The CNF formula to add clauses to
        pool: The ID pool for variable creation
        section_time_vars: Mapping from (section, time_slot) to variable IDs
        section_room_vars: Mapping from (section, room) to variable IDs
    """
    # Group variables by section for easier processing
    section_to_rooms = collections.defaultdict(list)
    section_to_times = collections.defaultdict(list)
    
    # Organize variables by section
    for (section, room), var in section_room_vars.items():
        section_to_rooms[section].append(var)
    
    for (section, time_slot), var in section_time_vars.items():
        section_to_times[section].append(var)
    
    # Constraint 1: Each section must be assigned exactly one room (if it has available rooms)
    for section, room_vars in section_to_rooms.items():
        if not room_vars:
            continue
            
        # At least one room must be assigned
        cnf.append(room_vars)

        # At most one room must be assigned
        clauses = CardEnc.atmost(room_vars, vpool=pool, bound=1, encoding=EncType.pairwise).clauses
        for clause in clauses:
            cnf.append(clause)

    # Constraint 2: Each section must be assigned exactly one time slot
    for section, time_vars in section_to_times.items():
        assert time_vars, f"Section {section} has no available time slots"
        
        # At least one time slot must be assigned
        cnf.append(time_vars)

        # At most one time slot must be assigned
        clauses = CardEnc.atmost(time_vars, vpool=pool, bound=1, encoding=EncType.pairwise).clauses
        for clause in clauses:
            cnf.append(clause)
    

def encode_room_conflicts(
    timetable: TimetableData,
    cnf: CNF,
    pool: IDPool,
    section_room_vars: SectionRoomVars,
    section_time_vars: SectionTimeVars
) -> None:
    """
    Encode the constraint that two sections cannot be in the same room
    at overlapping time slots.
    
    Args:
        timetable: The timetable data
        cnf: The CNF formula to add clauses to
        pool: The ID pool for variable creation
        section_room_vars: Mapping from (section, room) to variable IDs
        section_time_vars: Mapping from (section, time_slot) to variable IDs
    """
    # Group sections by room
    room_to_sections = collections.defaultdict(list)
    
    for (section, room), _ in section_room_vars.items():
        room_to_sections[room].append(section)
    
    # For each room, prevent overlapping section assignments
    for room, sections in room_to_sections.items():
        # Skip if only one section can use this room
        if len(sections) < 2:
            continue
        
        # For each pair of sections that could use this room
        for i, section_a in enumerate(sections):
            for section_b in sections[i+1:]:
                encode_room_conflict(timetable, cnf, pool, section_room_vars, section_time_vars, section_a, section_b, room)


def encode_room_conflict(
    timetable: TimetableData,
    cnf: CNF,
    pool: IDPool,
    section_room_vars: SectionRoomVars,
    section_time_vars: SectionTimeVars,
    section_a: SectionName,
    section_b: SectionName,
    room: RoomName
) -> None:
    """
    Encode the constraint that two sections cannot be in the same room
    at overlapping time slots.
    
    Args:
        timetable: The timetable data
        cnf: The CNF formula to add clauses to
        pool: The ID pool for variable creation
        section_room_vars: Mapping from (section, room) to variable IDs
        section_time_vars: Mapping from (section, time_slot) to variable IDs
        section_a: First section name
        section_b: Second section name
        room: Room name
    """
    # Get room variables for both sections - must exist if we've initialized correctly
    assert (section_a, room) in section_room_vars, f"Missing variable for {section_a}, {room}"
    assert (section_b, room) in section_room_vars, f"Missing variable for {section_b}, {room}"
    
    room_var_a = section_room_vars[(section_a, room)]
    room_var_b = section_room_vars[(section_b, room)]
    
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
            assert (section_a, time_a) in section_time_vars, f"Missing variable for {section_a}, {time_a}"
            assert (section_b, time_b) in section_time_vars, f"Missing variable for {section_b}, {time_b}"
            
            time_var_a = section_time_vars[(section_a, time_a)]
            time_var_b = section_time_vars[(section_b, time_b)]
            
            # Add clause: ~(A_time & A_room & B_time & B_room)
            # Which is equivalent to: (!A_time | !A_room | !B_time | !B_room)
            cnf.append([-time_var_a, -room_var_a, -time_var_b, -room_var_b])


def decode_solution(
    model: list[int],
    section_room_vars: SectionRoomVars,
    section_time_vars: SectionTimeVars,
) -> Placement:
    """
    Decode a SAT solution into a schedule.
    
    Args:
        model: The solution model from the SAT solver
        section_time_vars: Mapping from (section, time_slot) to variable IDs
        section_room_vars: Mapping from (section, room) to variable IDs
    
    Returns:
        Placement mapping sections to (room, time_slot) tuples
    """
    # create reverse lookup tables to find section/room and section/time from var
    var_to_section_room = {v: k for k, v in section_room_vars.items()}
    var_to_section_time = {v: k for k, v in section_time_vars.items()}

    section_to_room = {}
    section_to_time_slot = {}

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

    # Construct the final placement
    placement = Placement({})
    for (section, time_slot) in section_to_time_slot.items():
        if section in section_to_room:
            placement[section] = (section_to_room[section], time_slot)
        else:
            placement[section] = (None, time_slot)
    
    return placement
