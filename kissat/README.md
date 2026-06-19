Kissat SAT solver
=================

A simple wrapper for the Kissat SAT solver.

Kissat is a state of the art SAT solver by Armin Biere and others. It is
written in C rather than C++, unlike CaDiCaL (upon which it is based) and older
solvers such as minisat and glucose. This makes it particularly nice to embed
in Rust.

This crate builds the entire source code of Kissat, and provides a safe
interface over it.

Wrapper version 0.2.0 vendors Kissat release 4.0.4, upstream tag
`rel-4.0.4`, commit `8af8e56f174b778aef3aa45af9f739b2a5f492c2`.

The upstream release metadata and license are retained in the `kissat`
directory.
