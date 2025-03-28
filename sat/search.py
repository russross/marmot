# search.py
"""
Iterative SAT solver for the Marmot timetabling system.

This module implements the core search algorithm that iteratively searches
for the minimum number of violations at each priority level.
"""
from typing import Optional, Any
import time
import sys

from pysat.formula import CNF # type: ignore
from pysat.solvers import Solver # type: ignore

from data import TimetableData
from core import create_sat_instance, decode_solution

# Type alias for schedule representation
Schedule = dict[str, tuple[Optional[str], str]]


def solve_timetable(
    timetable: TimetableData, 
    solver_name: str = "cd", 
    max_time_seconds: int = 3600, 
    verbose: bool = False
) -> Optional[Schedule]:
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
    
    # Transform criteria from the input by priority level
    constraints_by_priority = timetable.get_constraints_by_priority()
    
    # Store the best schedule found
    best_schedule = None
    
    # Keep track of maximum violations allowed at each priority level
    max_violations = {}
    
    # Get all priority levels from the constraints
    all_priorities = sorted(constraints_by_priority.keys())
    
    # Process each priority level in order
    for priority in all_priorities:
        if time.time() - start_time > max_time_seconds:
            print(f"Time limit reached after processing priority level {priority-1}")
            break
        
        # Get criteria count at this level
        criteria_count = len(constraints_by_priority.get(priority, []))
        
        # Skip if no criteria at this level and not priority 0
        if criteria_count == 0:
            max_violations[priority] = 0
            continue
        
        print(f"Solving for priority level {priority} with {criteria_count} criteria")
        
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
    
    print("\nFinal solution maximum violations per priority level:")
    print_max_violations(max_violations)
    
    return best_schedule


def solve_at_priority_level(
    timetable: TimetableData,
    priority: int,
    max_violations: dict[int, int],
    solver_name: str,
    remaining_time: float,
    verbose: bool
) -> tuple[bool, int, Optional[Schedule]]:
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
    solution_found = False
    schedule = None
    
    start_time = time.time()
    while not solution_found:
        if k > criteria_count:
            print(f"    k > criteria count for this priority, giving up")
            return False, 0, None
            
        if time.time() - start_time > remaining_time:
            print(f"    time limit reached while searching for solutions at level {priority}")
            return False, 0, None
        
        # Create the SAT instance
        cnf, var_mappings = create_sat_instance(
            timetable, 
            max_violations,
            priority, 
            k
        )
        
        print(f"    priority {priority}, k={k} solving encoding with {cnf.nv} variables and {len(cnf.clauses)} clauses")
        
        # Solve the SAT instance
        solver = Solver(name=solver_name, bootstrap_with=cnf)
        solved = solver.solve()
        
        if solved:
            # We've found a solution with this many violations
            model = solver.get_model()
            schedule = decode_solution(model, var_mappings)
            solution_found = True
        else:
            # Increment violations and try again
            k += 1
            
            # We must find a solution for hard constraints with no violations
            if priority == 0 and not solution_found:
                print(f"    could not find a solution for priority level {priority}")
                return False, 0, None
                
        # Clean up the solver
        solver.delete()
    
    return True, k, schedule


def print_max_violations(max_violations: dict[int, int]) -> None:
    """Print a summary of violations at each priority level."""
    for priority, violations in sorted(max_violations.items()):
        if violations > 0:
            print(f"  Priority level {priority}: {violations} violations")
