"""
Iterative SAT solver for the Marmot timetabling system.

This module implements the core search algorithm that iteratively searches
for the minimum number of violations at each priority level.
"""

from typing import Optional
import time

from core import create_sat_instance, decode_solution
from data import TimetableData, Schedule, Score, Priority
from solver import solve_with_external_solver

def solve_timetable(
    timetable: TimetableData,
    solver_name: str,
    max_time_seconds: int
) -> Optional[Schedule]:
    """
    Solve the timetabling problem using iterative SAT solving.

    Args:
        timetable: The timetable data
        solver_name: The SAT solver executable to use
        max_time_seconds: Maximum time to spend solving in seconds

    Returns:
        The best schedule found or None if no feasible schedule was found
    """
    start_time = time.time()

    # Store the best schedule found
    best_schedule: Optional[Schedule] = None

    # Track maximum violations allowed using Score
    max_violations = Score()

    # Process each priority level in order
    max_priority = timetable.get_max_priority()
    for p in range(max_priority + 1):
        priority = Priority(p)
        constraints = timetable.get_constraints_by_priority(priority)
        if len(constraints) == 0 and priority > 0:
            continue

        if time.time() - start_time > max_time_seconds:
            print(f"Time limit reached after processing priority level {priority-1}")
            break

        print(f"Solving for priority level {priority} with {len(constraints)} criteria")

        # Solve at this priority level, updating max_violations in place
        success, new_schedule = solve_at_priority_level(
            timetable,
            priority,
            max_violations,
            solver_name,
            max_time_seconds - (time.time() - start_time)
        )

        if success:
            if new_schedule is not None:
                best_schedule = new_schedule
        else:
            print(f"Failed to find solution at priority level {priority}, keeping best schedule so far")
            if priority == 0:
                # If we can't satisfy the hard constraints, we have no solution
                return None
            break

    if best_schedule is not None:
        # Sanity check: compare constructed max_violations with solution score
        if max_violations != best_schedule.score:
            print("\nWARNING: Inconsistency detected in violation counts:")
            print(f"  Search algorithm found: {max_violations}")
            print(f"  Solution reports:       {best_schedule.score}")
        print(f"\nFinal solution score: {best_schedule.score}")

    return best_schedule


def solve_at_priority_level(
    timetable: TimetableData,
    priority: Priority,
    max_violations: Score,
    solver_name: str,
    remaining_time: float
) -> tuple[bool, Optional[Schedule]]:
    """
    Solve for a specific priority level, finding minimum violations.
    
    Args:
        timetable: The timetable data
        priority: The priority level to solve for
        max_violations: Score object tracking maximum allowed violations, modified in place
        solver_name: The SAT solver executable to use
        remaining_time: Time remaining in seconds
    
    Returns:
        (success, schedule): Success flag and resulting schedule
    """
    # Get constraints at this priority level
    constraints_at_level = [c for c in timetable.get_all_constraints() if c.priority == priority]
    criteria_count = len(constraints_at_level)
    
    # Reset violations at this priority level to start at 0
    max_violations[priority] = 0
    schedule: Optional[Schedule] = None
    
    start_time = time.time()
    while schedule is None:
        if max_violations[priority] > criteria_count:
            print(f"    violations > criteria count for this priority, giving up")
            return False, None
            
        if time.time() - start_time > remaining_time:
            print(f"    time limit reached while searching for solutions at level {priority}")
            return False, None
        
        # Create the SAT instance using current violations from max_violations
        encoding = create_sat_instance(timetable, max_violations, priority, max_violations[priority])
        
        print(f"    priority {priority}, violations={max_violations[priority]} solving encoding with {encoding.last_var} variables and {len(encoding.clauses)} clauses")
        
        # call the SAT solver backend
        timeout = remaining_time - (time.time() - start_time)
        model = solve_with_external_solver(
            encoding, 
            solver_name, 
            timeout_seconds=int(timeout)
        )
        
        if model is not None:
            # We've found a solution with this many violations
            schedule = decode_solution(encoding, model)
        else:
            # Increment violations at this priority level and try again
            max_violations.inc_priority(priority)
            
            # Make a note if we fail to satisfy hard constraints
            if priority == 0 and schedule is None:
                print(f"    could not find a solution using only hard constraints")
    
    return True, schedule
