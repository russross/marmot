Rust style notes
================

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
