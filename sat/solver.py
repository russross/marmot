"""
External SAT solver interface for the Marmot timetabling system.

This module provides functions to invoke external SAT solvers using the DIMACS format.
It replaces the PySAT dependency with direct solver invocation.
"""
import subprocess
import tempfile
import os
from typing import Optional

from encoding import Encoding

def solve_with_external_solver(
    encoding: Encoding,
    solver_executable: str,
    timeout_seconds: Optional[int] = None
) -> Optional[list[int]]:
    """
    Solve a SAT instance using an external solver.
    
    Args:
        encoding: The Encoding object containing clauses and variables
        solver_executable: Name of the solver executable (e.g., "cadical", "glucose", etc.)
        timeout_seconds: Maximum time in seconds to allow the solver to run
        
    Returns:
        (solved, model): Whether the problem was solved and model if solved, None otherwise
    """
    # Create a temporary file to store the DIMACS format CNF
    with tempfile.NamedTemporaryFile(mode='w', suffix='.cnf', delete=False) as cnf_file:
        cnf_path = cnf_file.name
        
        # Write the CNF to the file in DIMACS format
        # Count the number of clauses
        num_clauses = len(encoding.clauses)
        
        # Write the header
        cnf_file.write(f"p cnf {encoding.last_var} {num_clauses}\n")
        
        # Write each clause
        for clause in encoding.clauses:
            # Convert frozenset to a list of integers followed by 0
            line = " ".join(str(lit) for lit in sorted(clause)) + " 0\n"
            cnf_file.write(line)
    
    try:
        # Run the solver with direct stdout capture
        cmd = [solver_executable, cnf_path]
        
        # Run the solver with timeout
        result = subprocess.run(
            cmd, 
            stdout=subprocess.PIPE, 
            stderr=subprocess.PIPE, 
            timeout=timeout_seconds,
            text=True,
            check=False
        )
        
        # Check exit code first (contest standard: 10 for SAT, 20 for UNSAT)
        if result.returncode == 10:
            # SAT result, parse the model
            return parse_solver_output(result.stdout, encoding.last_var)
        elif result.returncode == 20:
            # UNSAT result
            return None
        else:
            # Exit code doesn't match standard, try to parse output directly
            model = parse_solver_output(result.stdout, encoding.last_var)
            
            # Log unexpected exit code
            if result.returncode != 0:
                print(f"Solver exited with unexpected code: {result.returncode}")
                print(f"Stderr: {result.stderr}")
            
            # Consider solved if we found a model
            return model
            
        return model
            
    except subprocess.TimeoutExpired:
        print(f"Solver timed out after {timeout_seconds} seconds")
        return None
    finally:
        # Clean up temporary CNF file
        try:
            os.unlink(cnf_path)
        except:
            pass


def parse_solver_output(output: str, num_vars: int) -> Optional[list[int]]:
    """
    Parse the model directly from the solver's stdout.
    
    Args:
        output: Solver's stdout content
        num_vars: Number of variables in the problem
        
    Returns:
        (solved, model): Whether the problem was solved and model if solved, None otherwise
    """
    # Parse the model
    model = []
    for line in output.splitlines():
        # Skip comments and solver information
        if not line or line.startswith('c') or line.startswith('s'):
            continue
            
        # Parse the model line (should start with 'v')
        if line.startswith('v'):
            parts = line[2:].strip().split()
            for part in parts:
                if part == '0':
                    continue
                try:
                    var = int(part)
                    model.append(var)
                except ValueError:
                    continue
    
    # If no model was found in 'v' lines, try looking at other lines
    # (some solvers might not use the 'v' prefix)
    if not model:
        for line in output.splitlines():
            if line.startswith('c') or line.startswith('s'):
                continue
                
            # Look for lines that have space-separated integers
            parts = line.strip().split()
            if parts and all(p.lstrip('-').isdigit() for p in parts if p != '0'):
                for part in parts:
                    if part == '0':
                        continue
                    try:
                        var = int(part)
                        model.append(var)
                    except ValueError:
                        continue
    
    # Verify model completeness
    if model and len(model) < num_vars:
        # If the model is incomplete (solver may not report unassigned vars),
        # fill in missing variables as negative (false)
        assigned_vars = {abs(var) for var in model}
        for var in range(1, num_vars + 1):
            if var not in assigned_vars:
                model.append(-var)
    
    return model if model else None
