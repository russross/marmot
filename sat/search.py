# search.py
"""
Iterative SAT solver for the Marmot timetabling system.

This module implements the core search algorithm that iteratively searches
for the minimum number of violations at each priority level.
"""

from typing import Optional, Any
import time
import sys

from pysat.solvers import Solver # type: ignore

from core import create_sat_instance, decode_solution
from data import TimetableData, SectionName, RoomName, TimeSlotName, Priority
from encoding import Placement

def solve_timetable(
    timetable: TimetableData,
    solver_name: str = "cd",
    max_time_seconds: int = 3600,
    verbose: bool = False
) -> Optional[tuple[Placement, set[tuple[int, str]]]]:
    """
    Solve the timetabling problem using iterative SAT solving.

    Args:
        timetable: The timetable data
        solver_name: The SAT solver to use (default: "cd" for Cadical)
        max_time_seconds: Maximum time to spend solving in seconds
        verbose: Whether to print detailed progress information

    Returns:
        The best schedule found or None if no feasible schedule was found
    """
    start_time = time.time()

    # Store the best schedule found
    best_schedule = None

    # Keep track of maximum violations allowed at each priority level
    max_violations = {}

    # Process each priority level in order
    max_priority = timetable.get_max_priority()
    for p in range(max_priority + 1):
        priority = Priority(p)
        constraints = timetable.get_constraints_by_priority(priority)
        if len(constraints) == 0 and priority > 0:
            max_violations[priority] = 0
            continue

        if time.time() - start_time > max_time_seconds:
            print(f"Time limit reached after processing priority level {priority-1}")
            break

        print(f"Solving for priority level {priority} with {len(constraints)} criteria")

        # Solve at this priority level
        success, k, new_schedule = solve_at_priority_level(
            timetable,
            priority,
            max_violations,
            solver_name,
            max_time_seconds - (time.time() - start_time),
            verbose
        )

        if success:
            # Update max violations for this level
            max_violations[priority] = k
            if new_schedule is not None:
                best_schedule = new_schedule
        else:
            print(f"Failed to find solution at priority level {priority}, keeping best schedule so far")
            if priority == 0:
                # If we can't satisfy the hard constraints, we have no solution
                return None
            break

    # Calculate score vectors for validation
    search_score_vector = get_score_vector_from_max_violations(max_violations, max_priority)

    if best_schedule is not None:
        # Get the score vector from the actual problems in the solution
        solution_score_vector = get_score_vector_from_problems(best_schedule[1], max_priority)

        # Validate that both score vectors match
        if search_score_vector != solution_score_vector:
            print("\nWARNING: Inconsistency detected in violation counts:")
            print(f"  Search algorithm found: {search_score_vector}")
            print(f"  Solution reports:       {solution_score_vector}")

        # Print the final score in the score format from save.py
        score_str = score_string(best_schedule[1])
        print(f"\nFinal solution score: {score_str}")

    return best_schedule


def get_score_vector_from_max_violations(
    max_violations: dict[Priority, int],
    max_priority: int
) -> list[int]:
    """
    Convert the max_violations dictionary into a score vector.

    Args:
        max_violations: Dictionary mapping priority levels to maximum violations
        max_priority: The maximum priority level

    Returns:
        A list where each index corresponds to a priority level and the value
        is the number of violations at that level
    """
    score_vector = [0] * (max_priority + 1)

    for priority, violations in max_violations.items():
        score_vector[priority] = violations

    return score_vector


def get_score_vector_from_problems(
    problems: set[tuple[int, str]],
    max_priority: int
) -> list[int]:
    """
    Convert the problems set into a score vector.

    Args:
        problems: Set of (priority, description) tuples
        max_priority: The maximum priority level

    Returns:
        A list where each index corresponds to a priority level and the value
        is the number of violations at that level
    """
    # Count problems by priority level
    counts_by_priority: dict[int, int] = {}
    for priority, _ in problems:
        counts_by_priority[priority] = counts_by_priority.get(priority, 0) + 1

    # Create the score vector
    score_vector = [0] * (max_priority + 1)

    for priority, count in counts_by_priority.items():
        score_vector[priority] = count

    return score_vector


def solve_at_priority_level(
    timetable: TimetableData,
    priority: Priority,
    max_violations: dict[Priority, int],
    solver_name: str,
    remaining_time: float,
    verbose: bool
) -> tuple[bool, int, Optional[tuple[Placement, set[tuple[int, str]]]]]:
    """
    Solve for a specific priority level, finding minimum violations.
    
    Args:
        timetable: The timetable data
        priority: The priority level to solve for
        max_violations: Maximum allowed violations for each prior priority level
        solver_name: The SAT solver to use
        remaining_time: Time remaining in seconds
        verbose: Whether to print detailed progress information
    
    Returns:
        (success, k, schedule): Success flag, violations required, and resulting schedule
    """
    # Get constraints at this priority level
    constraints_at_level = [c for c in timetable.get_all_constraints() if c.priority == priority]
    criteria_count = len(constraints_at_level)
    
    # Start with attempting zero violations and increase until a solution is found
    k = 0
    schedule = None
    
    start_time = time.time()
    while schedule is None:
        if k > criteria_count:
            print(f"    k > criteria count for this priority, giving up")
            return False, 0, None
            
        if time.time() - start_time > remaining_time:
            print(f"    time limit reached while searching for solutions at level {priority}")
            return False, 0, None
        
        # Create the SAT instance
        encoding = create_sat_instance(timetable, max_violations, priority, k)
        
        print(f"    priority {priority}, k={k} solving encoding with {encoding.last_var} variables and {len(encoding.clauses)} clauses")
        
        # Solve the SAT instance
        solver = Solver(name=solver_name, bootstrap_with=encoding.clauses)
        solved = solver.solve()
        
        if solved:
            # We've found a solution with this many violations
            model = solver.get_model()
            schedule = decode_solution(encoding, model)
        else:
            # Increment violations and try again
            k += 1
            
            # Make a note if we fail to satisfy hard constraints
            if priority == 0 and schedule is None:
                print(f"    could not find a solution using only hard constraints")
                
        # Clean up the solver
        solver.delete()
    
    return True, k, schedule

def print_max_violations(max_violations: dict[Priority, int]) -> None:
    """Print a summary of violations at each priority level."""
    for priority, violations in sorted(max_violations.items()):
        if violations > 0:
            print(f"  Priority level {priority}: {violations} violations")

def score_string(problems: set[tuple[int, str]]) -> str:
    """
    Generate a human-readable string representation of the score.

    Format: <0×1,5×2> where the first number is priority level and
    the second is the count of violations at that level.

    Args:
        problems: Set of (priority, description) tuples

    Returns:
        String representation of the score
    """
    # Count problems by priority level
    counts_by_priority: dict[int, int] = {}
    for priority, _ in problems:
        counts_by_priority[priority] = counts_by_priority.get(priority, 0) + 1

    if not counts_by_priority:
        return "zero"

    parts = []
    for priority, count in sorted(counts_by_priority.items()):
        parts.append(f"{priority}×{count}")

    return "<" + ",".join(parts) + ">"
