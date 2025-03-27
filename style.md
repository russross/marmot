# Style Guide

## Rust style notes

*   Prefer a single long line for function headers. These will be
    reformatted by "cargo fmt" eventually, but when working on the
    code I prefer to see a single line header.

*   Add explicit type annotations when it may be confusing from
    context. For example, while not every Vec should be annotated,
    those that are being created from complex iterator
    transformations where the end type is difficult to infer are
    good candidates for explicit annotations.

*   Variable names should not always be long and verbose. If a
    variable is short-lived, a single letter variable may be the
    best choice, e.g., "i" for an index or the first letter of a
    type for an instance of that type. If the variable is more
    long-lived, prefer longer, more explicit names. A reader should
    not have to scroll back to figure out what a name means.

*   Prefer iterators to loops generally, but prefer loops over
    convoluted iterator chains.

*   Prefer "if corner case then continue/return" to "if normal case
    indent the rest of the block". Deep indentation indicates
    nested loops and complex conditions, and should be avoided in
    other cases where it can be misleading and hard to work with.

## Python style notes

*   Use Python 3 type hints consistently. All functions should have complete
    type annotations for parameters and return values. Use type aliases for 
    clarity (e.g., `SectionName = str`).

*   Prefer dataclasses for data structures with minimal behavior. Data structures
    should be immutable where possible, with clear fields and documentation.

*   Prefer ordinary functions to classes with methods when they
    mainly serve to group related functions.

*   Prefer single quoted strings to double quoted strings. Do not
    update a file just to make this change, but if you are ever
    rewriting code anyway be sure to convert to this style.

*   Use assertions for internal consistency checks. If a lookup should always 
    succeed, assert this rather than silently continuing with a default value.
    This helps identify bugs early.

*   Follow function naming conventions:
    - Public functions use lowercase with underscores (snake_case)
    - Internal/private helper functions start with an underscore
    - Function names should be descriptive but not excessively verbose

*   Keep a clean separation between modules:
    - data.py: Contains core data structures
    - core.py: Core SAT encoding and variable management
    - conflicts.py: Specialized constraint encodings
    - search.py: High-level search logic
    - Specialized modules for other constraint types

*   For error cases where recovery isn't possible, use assertions for internal
    errors and explicit exit with error messages for data integrity issues.

*   Functions should do one thing well - prefer multiple smaller functions 
    with clear responsibilities over large monolithic functions.

*   When working with libraries like PySAT, follow their patterns but abstract
    away their specific details where possible to keep the core logic clean.

*   For collections, prefer:
    - Dict/Set for lookups by key
    - defaultdict for building maps with default values
    - NamedTuple for immutable structured data with fields
