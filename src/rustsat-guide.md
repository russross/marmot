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

The SatInstance type provides several specialized methods for adding clauses of different sizes and encodings:

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

### Implication Encodings
SatInstance provides direct methods for encoding common implication patterns:

```rust
// a -> b
sat_instance.add_lit_impl_lit(a, b);

// a -> (b1 | b2 | ... | bm)
sat_instance.add_lit_impl_clause(a, &[b1, b2, b3]);

// a -> (b1 & b2 & ... & bm)
sat_instance.add_lit_impl_cube(a, &[b1, b2, b3]);

// (a1 & a2 & ... & an) -> b
sat_instance.add_cube_impl_lit(&[a1, a2, a3], b);

// (a1 | a2 | ... | an) -> b
sat_instance.add_clause_impl_lit(&[a1, a2, a3], b);

// (a1 & a2 & ... & an) -> (b1 | b2 | ... | bm)
sat_instance.add_cube_impl_clause(&[a1, a2, a3], &[b1, b2, b3]);

// (a1 | a2 | ... | an) -> (b1 | b2 | ... | bm)
sat_instance.add_clause_impl_clause(&[a1, a2, a3], &[b1, b2, b3]);

// (a1 | a2 | ... | an) -> (b1 & b2 & ... & bm)
sat_instance.add_clause_impl_cube(&[a1, a2, a3], &[b1, b2, b3]);

// (a1 & a2 & ... & an) -> (b1 & b2 & ... & bm)
sat_instance.add_cube_impl_cube(&[a1, a2, a3], &[b1, b2, b3]);
```

Note: This is the complete list of implication encoding methods available on the SatInstance type.

## Cardinality Constraints

So far we have only needed upper-bound constraints, i.e., the
at-most-k constraint, so we document it here. Support is built in
for upper bounds, lower-bounds, and exact bounds.

```rust
use rustsat::types::constraints::CardConstraint;

// Create at-most-k constraint
// Example: at most 1 of these literals can be true
let literals = vec![var1.pos_lit(), var2.pos_lit(), var3.pos_lit()];
sat_instance.add_card_constr(CardConstraint::new_ub(literals, 1));

// Note: CardConstraint::new_lb creates a lower bound, i.e., at-least-k,
// and CardConstraint::eq creates an exactly-k constraint.

// Create at-least-1 constraint
// Example: at least 1 of these literals must be true
let literals = vec![var1.pos_lit(), var2.pos_lit(), var3.pos_lit()];
sat_instance.add_nary(&literals); // a simple direct encoding of at-least-one

Update: where practical, use the encode_cardinality method of
SatEncoding, which chooses an appropriate encoding for various
cardinality constraints. That will provide a single place to
optimize choice of encoding. If you see reason to use a particular
encoding and encode_cardinality would not choose that encoding
already, let me know so we can update it rather than bypassing it.
```
