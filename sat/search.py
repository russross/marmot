# search.py
"""
Iterative SAT solver for the Marmot timetabling system.

This module implements the core search algorithm that iteratively searches
for the minimum number of violations at each priority level.
"""
from typing import Dict, List, Optional, Tuple, Any
import time
import sys

from pysat.formula import CNF
from pysat.solvers import Solver

from data import TimetableData
from core import create_sat_instance, decode_solution

# Type alias for schedule representation
Schedule = Dict[str, Tuple[Optional[str], str]]


def solve_timetable(
    timetable: TimetableData, 
    solver_name: str = "cd", 
    max_time_seconds: int = 3600, 
    verbose: bool = True
) -> Optional[Schedule]:
    """
    Solve the timetabling problem using iterative SAT solving.
    
    Args:
        timetable: The timetable data
        solver_name: The SAT solver to use (default: "cd" for Cadical)
        max_time_seconds: Maximum time to spend solving in seconds
        verbose: Whether to print progress information
    
    Returns:
        The best schedule found or None if no feasible schedule was found
    """
    start_time = time.time()
    
    # Track the minimum number of violations required at each priority level
    violations_by_priority: Dict[int, int] = {}
    
    # The best schedule found so far
    best_schedule: Optional[Schedule] = None
    
    # Get all priority levels from the constraints
    all_priorities = sorted(set(
        constraint.priority for constraint in timetable.get_all_constraints()
    ))
    
    if verbose:
        print(f"Found {len(all_priorities)} priority levels: {all_priorities}")
    
    # Process each priority level in order
    for priority in all_priorities:
        if time.time() - start_time > max_time_seconds:
            if verbose:
                print(f"Time limit reached after processing priority level {priority-1}")
            break
        
        if verbose:
            print(f"\nSolving for priority level {priority}")
        
        success, violations, schedule = _solve_at_priority_level(
            timetable, 
            priority, 
            violations_by_priority, 
            solver_name,
            max_time_seconds - (time.time() - start_time),
            verbose
        )
        
        if not success:
            if verbose:
                print(f"Failed to find solution at priority level {priority}")
            if priority == 0:
                # If we can't satisfy the hard constraints, we have no solution
                return None
            break
        
        violations_by_priority[priority] = violations
        best_schedule = schedule
        
        if verbose:
            print(f"Priority level {priority}: {violations} violations required")
    
    if verbose:
        print("\nSearch completed")
        _print_violations_summary(violations_by_priority)
    
    return best_schedule


def _solve_at_priority_level(
    timetable: TimetableData,
    priority: int,
    prior_violations: Dict[int, int],
    solver_name: str,
    remaining_time: float,
    verbose: bool
) -> Tuple[bool, int, Optional[Schedule]]:
    """
    Solve for a specific priority level, finding minimum violations.
    
    Args:
        timetable: The timetable data
        priority: The priority level to solve for
        prior_violations: Known violations at prior priority levels
        solver_name: The SAT solver to use
        remaining_time: Time remaining in seconds
        verbose: Whether to print progress information
    
    Returns:
        (success, violations, schedule): Success flag, violations required, and resulting schedule
    """
    # Get number of constraints at this priority level
    constraints_at_level = [c for c in timetable.get_all_constraints() if c.priority == priority]
    
    if not constraints_at_level:
        # If there are no constraints at this level, we trivially satisfy it
        # Use previous solution (if any)
        return True, 0, None
        
    # Start with attempting zero violations and increase until a solution is found
    violations = 0
    max_violations = len(constraints_at_level)
    
    start_time = time.time()
    while violations <= max_violations:
        if time.time() - start_time > remaining_time:
            if verbose:
                print(f"Time limit reached while searching for violations at level {priority}")
            return False, 0, None
        
        if verbose:
            print(f"  Trying with {violations} violations at priority {priority}...")
        
        # Create the SAT instance
        cnf, var_mappings = create_sat_instance(
            timetable, 
            prior_violations,
            priority, 
            violations
        )
        
        # Solve the SAT instance
        solver = Solver(name=solver_name, bootstrap_with=cnf)
        solved = solver.solve()
        
        if solved:
            # We've found a solution with this many violations
            if verbose:
                print(f"  Found solution with {violations} violations")
            
            # Decode the solution
            model = solver.get_model()
            schedule = decode_solution(model, var_mappings)
            
            return True, violations, schedule
        
        # Increment violations and try again
        violations += 1
        
        # Clean up the solver
        solver.delete()
    
    # If we get here, we couldn't find a solution even with max_violations
    return False, max_violations, None


def _print_violations_summary(violations_by_priority: Dict[int, int]) -> None:
    """Print a summary of violations at each priority level."""
    if not violations_by_priority:
        print("No solutions found at any priority level.")
        return
    
    print("\nViolations by priority level:")
    print("-----------------------------")
    for priority in sorted(violations_by_priority.keys()):
        print(f"Priority {priority}: {violations_by_priority[priority]} violations")
