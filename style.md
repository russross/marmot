# Style Guide

## Project Structure

*   The project follows a modular structure with clear separation of concerns:
    - Core data types and structures are defined in `data.py`
    - The main SAT encoding logic is in `core.py`
    - Individual constraint encoders reside in the `encoders/` subdirectory
    - Each encoder module in `encoders/` should focus on a single constraint type
    - Registration with the encoder system happens in each encoder module
    - `encoders/__init__.py` imports all encoder modules to ensure registration

*   Encoder module naming and organization:
    - File names should clearly indicate the constraint type they handle (e.g., `time_pref.py`)
    - Each module should include a class implementing the `ConstraintEncoder` protocol
    - The class name should end with "Encoder" (e.g., `TimeSlotPreferenceEncoder`)
    - Register using the EXACT constraint class name (e.g., `"TimeSlotPreference"`)

*   The main entry point is `main.py`, which should import the encoders package

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
    - Internal/private helper functions start with an underscore.
      Only use this for minor helpers, not those that are
      significant steps in the process.
    - Function names should be descriptive but not excessively verbose

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

*   For Python static types, avoid the old Dict, Tuple, and List
    values that have to be imported in favor of the built-in dict,
    typle, and list.

*   Prefer list comprehensions and set comprehensions over explicit
    loops when they make the code more succinct.

## SAT Encoding Guidelines

*   SAT variable naming should be consistent and clear:
    - Use tuples with descriptive fields as identifiers in the pool
    - Include context as the last element in the tuple (e.g., `"section_time"`, `"criterion"`)

*   Each constraint encoder should:
    - Focus on a single constraint type
    - Follow the interface defined in `encoder_types.py`
    - Include clear comments explaining the logical encoding of constraints
    - Return criterion variables when `allow_violations` is True
    - Handle the hard constraint case when `allow_violations` is False

*   Comments for SAT encoding should be particularly detailed, explaining:
    - The high-level meaning of the constraint
    - How the constraint is mapped to CNF clauses
    - The meaning of any auxiliary variables created
    - The logical equivalence of the encoding (e.g., "Encode: A → B ⟺ ¬A ∨ B")

*   The input data must be internally consistent. The encoders
    should be strict about checking this. These are the kinds of
    invariants that I want to enforce with asserts:

    *   If a criterion refers to an entity (faculty, section, time
        slot, room, etc.) then that entity must exist.
    *   Faculty all have at least one section
    *   Sections all have at least one time slot
    *   Note: sections do NOT have to have any rooms
    *   Any cross reference between data types must be valid. So,
        for example, if a section references a TimeSlotName, that
        time slot must exist
    *   When a criterion encoder finds a time slot or room
        associated with a section, the section time
        variables/section room variables must exist.

*   Not only that, but any criterion must be non-trivial. For example:

    *   In a time pattern constraint, the data must make it possible
        for sections in the list to have mismatched time patterns
        (otherwise why add the criterion?).
    *   If a faculty requests a days off criterion, they must have
        at least two sections (otherwise the criterion cannot
        possibly have any influence on the outcome)
    *   If a faculty requests sections evenly spread, then they must
        have at least 4 sections (under our definition there is no
        way to fail the criterion with 3 or fewer).
    *   etc.
