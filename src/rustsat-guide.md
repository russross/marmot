# Guide to Using the Latest RustSAT API

This guide covers essential aspects of working with the RustSAT library based on the implementation in `sat.rs`. The RustSAT API has evolved, and this document highlights key components and patterns for effectively using the current version.

## Core Components

### Variables and Literals

```rust
use rustsat::types::{Var, Lit};

// Create a variable from the variable manager
let var = var_manager.new_var();

// Create literals from a variable
let positive_lit = var.pos_lit();  // Positive literal
let negative_lit = var.neg_lit();  // Negative literal
```

Note: The current API uses methods like `pos_lit()` and `neg_lit()` on the `Var` type rather than constructors.

### Clauses

```rust
use rustsat::types::Clause;

// Create a clause from an iterator of literals
let clause = Clause::from_iter(lits.iter().copied());

// Add clause to CNF
cnf.add_clause(clause);

// You can also create and add a clause in one step
cnf.add_clause(Clause::from_iter([lit1, lit2, lit3]));
```

### Variable Management

```rust
use rustsat::instances::{BasicVarManager, ManageVars};

// Create a variable manager
let mut var_manager = BasicVarManager::default();

// Create new variables
let var = var_manager.new_var();

// Get number of variables used
let num_vars = var_manager.n_used();
```

The `ManageVars` trait provides methods for variable management.

## CNF Representation

```rust
use rustsat::instances::Cnf;

// Create a new CNF
let mut cnf = Cnf::default();

// Add clauses to the CNF
cnf.add_clause(Clause::from_iter(literals));

// Get the number of clauses
let num_clauses = cnf.len();

// Clone a CNF for use with multiple solvers
let cnf_copy = cnf.clone();
```

## Working with SAT Solvers

### CaDiCaL

```rust
use rustsat::solvers::{Solve, SolverResult};
use rustsat_cadical::CaDiCaL;

// Create a solver
let mut solver = CaDiCaL::default();

// Add CNF to the solver
solver.add_cnf(cnf.clone()).map_err(|e| format!("{}", e))?;

// Solve the CNF
match solver.solve().map_err(|e| format!("{}", e))? {
    SolverResult::Sat => {
        // Get the full solution
        let solution = solver.full_solution().map_err(|e| format!("{}", e))?;
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

// Usage is identical to CaDiCaL
solver.add_cnf(cnf.clone()).map_err(|e| format!("{}", e))?;
// ...
```

## Working with Solutions

```rust
use rustsat::types::Assignment;

// Get the full solution from a solver
let solution = solver.full_solution().map_err(|e| format!("{}", e))?;

// Check if a variable is true in the solution
if solution.var_value(var).to_bool_with_def(false) {
    // Variable is true
} else {
    // Variable is false
}
```

## Encodings

### At-Most-One Encodings

```rust
use rustsat::encodings::am1::{Encode, Pairwise};

// Create a pairwise encoding from an iterator of literals
let am1_encoding = Pairwise::from_iter(literals.iter().copied());

// Encode and add to the CNF
am1_encoding.encode(&mut cnf, &mut var_manager).map_err(|e| format!("{}", e))?;
```

The `Pairwise` encoder ensures that at most one of the literals can be true. Other at-most-one encoders are available in the `am1` module.

## Error Handling

Note that the RustSAT functions return `Result` types that need to be handled:

```rust
// Common pattern for converting errors to strings
solver.add_cnf(cnf.clone()).map_err(|e| format!("{}", e))?;
```

## Literal Creation Examples

Creating literals has important nuances in the API:

```rust
// From a Var object
let pos_lit = var.pos_lit();
let neg_lit = var.neg_lit();
```

## Iterating Over Literals and Clauses

```rust
// Collect literals into a vector
let lits: Vec<Lit> = vars.iter()
    .map(|&var| var.pos_lit())
    .collect();

// Adding literals to a clause
let clause = Clause::from_iter(lits.iter().copied());
```

## Typical Workflow

1. Define the problem variables with a `BasicVarManager`
2. Create a `Cnf` instance to store clauses
3. Add problem constraints as clauses
4. Use encodings like `Pairwise` for complex constraints
5. Create a solver (CaDiCaL or Kissat)
6. Add the CNF to the solver
7. Solve and process the solution if satisfiable

This guide covers the key aspects of the RustSAT API as demonstrated in the provided implementation.
