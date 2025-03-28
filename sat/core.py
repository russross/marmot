# core.py
"""
Core SAT encoding for the Marmot timetabling system.

This module provides functions to encode the timetabling problem as a SAT instance.
"""
from typing import Optional, Any, NamedTuple
import collections

from pysat.formula import CNF, IDPool # type: ignore
from pysat.card import CardEnc, EncType # type: ignore

from data import TimetableData, ConstraintType
from encoder_types import SectionTimeVars, SectionRoomVars
from encoder_registry import get_all_encoders

# Type aliases (additional ones not defined in encoder_types.py)
SectionName = str
TimeSlotName = str
RoomName = str


class SATVariable(NamedTuple):
    """Represents a variable in the SAT encoding."""
    section: Optional[SectionName] = None
    time_slot: Optional[TimeSlotName] = None
    room: Optional[RoomName] = None
    faculty: Optional[str] = None
    day: Optional[int] = None
    criterion: Optional[int] = None
    purpose: Optional[str] = None  # Additional context about what this variable represents


def create_sat_instance(
    timetable: TimetableData,
    prior_violations: dict[int, int],
    current_priority: int,
    current_violations: int
) -> tuple[CNF, dict[str, Any]]:
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
    section_time_vars, section_room_vars, var_info = create_basic_variables(timetable, pool)
    
    # Encode the basic constraints
    encode_basic_constraints(timetable, cnf, section_time_vars, section_room_vars)
    
    # Encode room conflicts
    encode_room_conflicts(timetable, cnf, section_time_vars, section_room_vars)
    
    # Get all registered encoders
    encoders = get_all_encoders()
    
    # Track criterion variables by priority
    criterion_vars_by_priority = collections.defaultdict(list)
    
    # Encode all constraints up to and including the current priority level
    for priority in range(current_priority + 1):
        # Determine max violations allowed for this priority level
        max_violations = prior_violations.get(priority, 0) if priority < current_priority else current_violations
        allow_violations = max_violations > 0
        
        # Get all constraints at this priority level
        constraints_by_type: dict[str, list[ConstraintType]] = {}
        for constraint in timetable.get_all_constraints():
            if constraint.priority == priority:
                constraint_type = type(constraint).__name__
                if constraint_type not in constraints_by_type:
                    constraints_by_type[constraint_type] = []
                constraints_by_type[constraint_type].append(constraint)
        
        # Apply each registered encoder
        for constraint_type, encoder_class in encoders.items():
            if constraint_type in constraints_by_type and constraints_by_type[constraint_type]:
                encoder = encoder_class()
                criterion_vars = encoder.encode(
                    timetable, cnf, pool, section_time_vars, section_room_vars, priority, allow_violations
                )
                criterion_vars_by_priority[priority].extend(criterion_vars)
        
        # If violations are allowed, add cardinality constraint to limit total violations
        if allow_violations:
            all_vars = criterion_vars_by_priority[priority]
            if max_violations < len(all_vars) and all_vars:
                clauses = CardEnc.atmost(all_vars, bound=max_violations, encoding=EncType.totalizer).clauses
                for clause in clauses:
                    cnf.append(clause)
    
    # Return the CNF formula and variable mappings
    var_mappings = {
        'section_time_vars': section_time_vars,
        'section_room_vars': section_room_vars,
        'var_info': var_info,
        'criterion_vars_by_priority': criterion_vars_by_priority
    }
    
    return cnf, var_mappings


def create_basic_variables(
    timetable: TimetableData,
    pool: IDPool
) -> tuple[SectionTimeVars, SectionRoomVars, dict[int, SATVariable]]:
    """
    Create the basic variables for sections, time slots, and rooms.
    
    Args:
        timetable: The timetable data
        pool: The ID pool for variable creation
        
    Returns:
        section_time_vars: Mapping from (section, time_slot) to variable IDs
        section_room_vars: Mapping from (section, room) to variable IDs
        var_info: Information about variables
    """
    section_time_vars = {}
    section_room_vars = {}
    var_info = {}
    
    # Create section-time variables
    for section_name, section in timetable.sections.items():
        for time_slot_name in section.available_time_slots:
            var = pool.id((section_name, time_slot_name, "section_time"))
            section_time_vars[(section_name, time_slot_name)] = var
            var_info[var] = SATVariable(
                section=section_name,
                time_slot=time_slot_name,
                purpose="section_time"
            )
    
    # Create section-room variables
    for section_name, section in timetable.sections.items():
        for room_name in section.available_rooms:
            var = pool.id((section_name, room_name, "section_room"))
            section_room_vars[(section_name, room_name)] = var
            var_info[var] = SATVariable(
                section=section_name,
                room=room_name,
                purpose="section_room"
            )
    
    return section_time_vars, section_room_vars, var_info


def encode_basic_constraints(
    timetable: TimetableData,
    cnf: CNF,
    section_time_vars: SectionTimeVars,
    section_room_vars: SectionRoomVars
) -> None:
    """
    Encode the basic constraints of the timetabling problem:
    1. Each section must be assigned exactly one time slot
    2. Each section must be assigned exactly one room (if it has available rooms)
    
    Args:
        timetable: The timetable data
        cnf: The CNF formula to add clauses to
        section_time_vars: Mapping from (section, time_slot) to variable IDs
        section_room_vars: Mapping from (section, room) to variable IDs
    """
    # Group variables by section for easier processing
    section_to_times = collections.defaultdict(list)
    section_to_rooms = collections.defaultdict(list)
    
    # Organize variables by section
    for (section, time_slot), var in section_time_vars.items():
        section_to_times[section].append(var)
    
    for (section, room), var in section_room_vars.items():
        section_to_rooms[section].append(var)
    
    # Constraint 1: Each section must be assigned exactly one time slot
    for section, time_vars in section_to_times.items():
        assert time_vars, f"Section {section} has no available time slots"
        
        # At least one time slot must be assigned
        cnf.append(time_vars)

        # At most one time slot must be assigned
        clauses = CardEnc.atmost(time_vars, bound=1, encoding=EncType.pairwise).clauses
        for clause in clauses:
            cnf.append(clause)
    
    # Constraint 2: Each section must be assigned exactly one room (if it has available rooms)
    for section, room_vars in section_to_rooms.items():
        if not room_vars:
            continue
            
        # At least one room must be assigned
        cnf.append(room_vars)

        # At most one room must be assigned
        clauses = CardEnc.atmost(room_vars, bound=1, encoding=EncType.pairwise).clauses
        for clause in clauses:
            cnf.append(clause)


def encode_room_conflicts(
    timetable: TimetableData,
    cnf: CNF,
    section_time_vars: SectionTimeVars,
    section_room_vars: SectionRoomVars
) -> None:
    """
    Encode the constraint that two sections cannot be in the same room
    at overlapping time slots.
    
    Args:
        timetable: The timetable data
        cnf: The CNF formula to add clauses to
        section_time_vars: Mapping from (section, time_slot) to variable IDs
        section_room_vars: Mapping from (section, room) to variable IDs
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
                encode_room_conflict(timetable, cnf, section_time_vars, section_room_vars, section_a, section_b, room)


def encode_room_conflict(
    timetable: TimetableData,
    cnf: CNF,
    section_time_vars: SectionTimeVars,
    section_room_vars: SectionRoomVars,
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
        section_time_vars: Mapping from (section, time_slot) to variable IDs
        section_room_vars: Mapping from (section, room) to variable IDs
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


def decode_solution(model: list[int], var_mappings: dict[str, Any]) -> dict[SectionName, tuple[Optional[RoomName], TimeSlotName]]:
    """
    Decode a SAT solution into a schedule.
    
    Args:
        model: The solution model from the SAT solver
        var_mappings: Variable mappings from the encoding
    
    Returns:
        Schedule mapping sections to (room, time_slot) tuples
    """
    schedule = {}
    var_info = var_mappings['var_info']
    
    # Create a set of positive assignments for quick lookups
    positive_assignments = set(var for var in model if var > 0)
    
    # Group by section for faster processing
    section_assignments: dict[str, dict[str, str]] = collections.defaultdict(dict)
    
    # Process all positive variable assignments
    for var in positive_assignments:
        if var not in var_info:
            continue
            
        info = var_info[var]
        
        # Handle section-time assignments
        if info.purpose == "section_time" and info.section and info.time_slot:
            section_assignments[info.section]['time_slot'] = info.time_slot
            
        # Handle section-room assignments
        elif info.purpose == "section_room" and info.section and info.room:
            section_assignments[info.section]['room'] = info.room
    
    # Construct the final schedule
    for section, assignments in section_assignments.items():
        time_slot = assignments.get('time_slot')
        if time_slot:  # Only include sections that have a time slot assigned
            schedule[section] = (assignments.get('room'), time_slot)
    
    return schedule
