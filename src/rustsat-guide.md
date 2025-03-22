# Guide to Using the RustSAT API

This guide covers essential aspects of working with the RustSAT library based on the implementation in the Marmot timetabling system.

## Core Components

### Variables and Literals

```rust
use rustsat::types::{Var, Lit};

// Create a new variable from the SAT instance
let var = sat_instance.new_var();

// Create literals from a variable
let positive_lit = var.pos_lit();  // Positive literal
let negative_lit = var.neg_lit();  // Negative literal
```

Note that the current API uses methods `pos_lit()` and `neg_lit()` on the `Var` type rather than constructors.

### SAT Instance Management

```rust
use rustsat::instances::{BasicVarManager, SatInstance};

// Create a new SAT instance with a variable manager
let sat_instance = SatInstance::new_with_manager(BasicVarManager::default());

// Create a new variable
let var = sat_instance.new_var();
```

## Adding Constraints

The implementation uses specialized methods for adding clauses of different sizes:

```rust
// Add a unit clause (single literal)
sat_instance.add_unit(lit);

// Add a binary clause (two literals)
sat_instance.add_binary(lit1, lit2);

// Add a ternary clause (three literals)
sat_instance.add_ternary(lit1, lit2, lit3);

// Add a clause of arbitrary size
sat_instance.add_nary(&[lit1, lit2, lit3, lit4]);
```

## Cardinality Constraints

```rust
use rustsat::types::constraints::CardConstraint;

// Create at-most-k constraint
// Example: at most 1 of these literals can be true
let literals = vec![var1.pos_lit(), var2.pos_lit(), var3.pos_lit()];
sat_instance.add_card_constr(CardConstraint::new_ub(literals, 1));

// Create at-least-k constraint
// Example: at least 1 of these literals must be true
let literals = vec![var1.pos_lit(), var2.pos_lit(), var3.pos_lit()];
sat_instance.add_nary(&literals); // Simpler way to express at-least-one
```

## Working with SAT Solvers

### CaDiCaL

```rust
use rustsat::solvers::{Solve, SolverResult};
use rustsat_cadical::CaDiCaL;

// Create a solver
let mut solver = CaDiCaL::default();

// Convert SAT instance to CNF
sat_instance.convert_to_cnf();

// Add CNF to the solver
solver.add_cnf(sat_instance.cnf().clone())?;

// Solve and check result
match solver.solve()? {
    SolverResult::Sat => {
        // Get the full solution
        let solution = solver.full_solution()?;
        // Process solution...
    },
    _ => {
        // Handle unsatisfiable or unknown result
    }
}
```

### Kissat

```rust
use rustsat_kissat::Kissat;

// Create a solver
let mut solver = Kissat::default();

// Usage follows the same pattern as CaDiCaL
solver.add_cnf(sat_instance.cnf().clone())?;
// ...
```

## Working with Solutions

```rust
use rustsat::types::Assignment;

// Check if a variable is true in the solution
if solution.var_value(var).to_bool_with_def(false) {
    // Variable is true
} else {
    // Variable is false
}
```

## Error Handling

```rust
// Common pattern for converting errors to strings
solver.add_cnf(sat_instance.cnf().clone()).map_err(|e| format!("{}", e))?;
solver.solve().map_err(|e| format!("{}", e))?;
```

## Typical SAT-Based Solving Workflow

The implementation in `sat.rs` follows a hierarchical approach to constraints:

1. **Basic Constraints**: These are encoded first and must always be satisfied (e.g., each section must have one time slot)
2. **Hard Conflicts**: These are priority 0 constraints that cannot be violated
3. **Soft Constraints**: These are grouped by priority levels (1 to PRIORITY_LEVELS)

The solver works iteratively:
1. Encode hard constraints (priority 0)
2. For each priority level k (starting from 1):
   - Use previously determined violation limits for all prior levels
   - Try to find minimum violations needed for current level
   - Record this value and proceed to next level

This iterative approach ensures the most important constraints are satisfied first, with minimal violations at each subsequent priority level.

## Advanced Pattern: Criterion Variables

When soft constraints can be violated, the code uses a pattern with "criterion variables":

```rust
// Create a criterion variable
let criterion_var = sat_instance.new_var();

// Add implications that force criterion_var to be true when constraint is violated
// Example: If var_a and var_b are true (constraint violated), criterion_var must be true
sat_instance.add_ternary(var_a.neg_lit(), var_b.neg_lit(), criterion_var.pos_lit());

// Then use at-most-k constraint on all criterion variables to limit violations
let criterion_lits = criterion_vars.iter().map(|&var| var.pos_lit()).collect::<Vec<_>>();
sat_instance.add_card_constr(CardConstraint::new_ub(criterion_lits, max_violations));
```

This pattern allows the SAT solver to determine which constraints to violate when not all can be satisfied.
