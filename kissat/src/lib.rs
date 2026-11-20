//! A simple wrapper for the Kissat SAT solver.
//!
//! Kissat is a state of the art SAT solver by Armin Biere and others. It is
//! written in C rather than C++, unlike CaDiCaL (upon which it is based) and
//! older solvers such as minisat and glucose. This makes it particularly nice
//! to embed in Rust.
//!
//! This crate builds the entire source code of Kissat, and provides a safe
//! interface over it.
//!
//! ```rust
//! extern crate kissat;
//! fn main() {
//!     let mut solver = kissat::Solver::new();
//!     let a = solver.var();
//!     let b = solver.var();
//!     solver.add2(a, !b);
//!     solver.add1(b);
//!     match solver.sat() {
//!         Some(solution) => println!("SAT: {:?} {:?}", solution.get(a), solution.get(b)),
//!         None => println!("UNSAT"),
//!     }
//! }
//! ```

use std::cell::Cell;
use std::marker::PhantomData;
use std::ops::Not;
use std::os::raw::{c_char, c_int};
use std::sync::Arc;

const MAX_VAR_ID: c_int = (1 << 30) - 1;

#[repr(C)]
struct kissat {
    _unused: [u8; 0],
}

unsafe extern "C" {
    fn kissat_init() -> *mut kissat;
    fn kissat_set_configuration(solver: *mut kissat, name: *const c_char) -> c_int;
    fn kissat_add(solver: *mut kissat, lit: c_int);
    fn kissat_solve(solver: *mut kissat) -> c_int;
    fn kissat_terminate(solver: *mut kissat);
    fn kissat_value(solver: *mut kissat, lit: c_int) -> c_int;
    fn kissat_release(solver: *mut kissat);
}

/// A built-in Kissat solver configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Configuration {
    /// Disable advanced preprocessing and use stable CDCL search.
    Plain,
}

impl Configuration {
    fn name(self) -> *const c_char {
        match self {
            Self::Plain => c"plain".as_ptr(),
        }
    }
}

/// A single literal or its negation
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Var {
    id: c_int,
}

impl Not for Var {
    type Output = Var;

    fn not(self) -> Var {
        Var { id: -self.id }
    }
}

struct Instance(*mut kissat);

// The Solver owns access to the native state. A cancellation handle shares its
// lifetime and can only set the native atomic stop flag. Release runs once the
// solver, any solution, and all cancellation handles are gone.
unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { kissat_release(self.0) };
    }
}

/// A handle that can request cooperative termination from another thread.
pub struct CancellationHandle(Arc<Instance>);

impl CancellationHandle {
    pub fn cancel(&self) {
        unsafe { kissat_terminate(self.0.0) };
    }
}

/// The outcome of a solve that may be interrupted.
pub enum SolveOutcome {
    Sat(Solution),
    Unsat,
    Interrupted,
}

/// A SAT problem under construction
pub struct Solver {
    instance: Arc<Instance>,
    n_vars: c_int,
    // Model lookup can mutate Kissat's native state during model extension.
    _not_sync: PhantomData<Cell<()>>,
}

impl Solver {
    /// Create a new solver instance
    pub fn new() -> Solver {
        unsafe {
            let p = kissat_init();
            assert!(!p.is_null());
            Solver { instance: Arc::new(Instance(p)), n_vars: 0, _not_sync: PhantomData }
        }
    }

    /// Create a new solver instance using a built-in configuration.
    pub fn with_configuration(configuration: Configuration) -> Solver {
        let solver = Solver::new();
        unsafe {
            assert_eq!(kissat_set_configuration(solver.instance.0, configuration.name()), 1);
        }
        solver
    }

    pub fn cancellation_handle(&self) -> CancellationHandle {
        CancellationHandle(Arc::clone(&self.instance))
    }

    /// Create a new literal
    pub fn var(&mut self) -> Var {
        assert!(self.n_vars < MAX_VAR_ID);
        let id = self.n_vars + 1;
        self.n_vars = id;
        Var { id }
    }

    /// Assert a literal should be true in the solution
    pub fn add1(&mut self, a: Var) {
        unsafe {
            kissat_add(self.instance.0, a.id);
            kissat_add(self.instance.0, 0);
        }
    }

    /// Assert `a or b` should be true in the solution
    pub fn add2(&mut self, a: Var, b: Var) {
        unsafe {
            kissat_add(self.instance.0, a.id);
            kissat_add(self.instance.0, b.id);
            kissat_add(self.instance.0, 0);
        }
    }

    /// Assert `a or b or c` should be true in the solution
    pub fn add3(&mut self, a: Var, b: Var, c: Var) {
        unsafe {
            kissat_add(self.instance.0, a.id);
            kissat_add(self.instance.0, b.id);
            kissat_add(self.instance.0, c.id);
            kissat_add(self.instance.0, 0);
        }
    }

    /// Add an assertion for an arbitrarily long clause
    pub fn add(&mut self, x: &[Var]) {
        unsafe {
            for v in x {
                kissat_add(self.instance.0, v.id);
            }
            kissat_add(self.instance.0, 0);
        }
    }

    /// Solve the instance, returning either `Some(Solution)` if the problem is
    /// SAT or `None` if the problem is UNSAT.
    ///
    /// Use [`Self::solve`] when requesting cancellation.
    pub fn sat(self) -> Option<Solution> {
        match self.solve() {
            SolveOutcome::Sat(solution) => Some(solution),
            SolveOutcome::Unsat => None,
            SolveOutcome::Interrupted => panic!("Kissat solve was interrupted"),
        }
    }

    pub fn solve(self) -> SolveOutcome {
        match unsafe { kissat_solve(self.instance.0) } {
            10 => SolveOutcome::Sat(Solution { solver: self }),
            20 => SolveOutcome::Unsat,
            0 => SolveOutcome::Interrupted,
            ret => unreachable!("unexpected Kissat result {ret}"),
        }
    }

    /// Create a new literal that is true if `a and b`
    pub fn and(&mut self, a: Var, b: Var) -> Var {
        let z = self.var();
        self.add3(!a, !b, z);
        self.add2(a, !z);
        self.add2(b, !z);
        z
    }

    /// Create a new literal that is true if `a or b`
    pub fn or(&mut self, a: Var, b: Var) -> Var {
        let z = self.var();
        self.add3(a, b, !z);
        self.add2(!a, z);
        self.add2(!b, z);
        z
    }

    /// Create a new literal that is true if `a xor b`
    pub fn xor(&mut self, a: Var, b: Var) -> Var {
        let z = self.var();
        self.add3(!a, !b, !z);
        self.add3(a, b, !z);
        self.add3(a, !b, z);
        self.add3(!a, b, z);
        z
    }

    /// Assert that two literals should be equal in the solution
    pub fn eq(&mut self, a: Var, b: Var) {
        self.add2(a, !b);
        self.add2(!a, b);
    }

    /// Create a new literal that if `i ? t : e` (if-then-else)
    pub fn ite(&mut self, i: Var, t: Var, e: Var) -> Var {
        let z = self.var();
        self.add3(!e, i, z);
        self.add3(e, i, !z);
        self.add3(!i, !t, z);
        self.add3(!i, t, !z);
        z
    }
}

impl Default for Solver {
    fn default() -> Solver {
        Solver::new()
    }
}

/// A solved SAT instance, for a problem which is satisfiable
pub struct Solution {
    solver: Solver,
}

impl Solution {
    /// Return the assigned value for a given variable, this can be `None`
    /// if it's is don't care for this solution.
    pub fn get(&self, x: Var) -> Option<bool> {
        let ret;
        unsafe {
            ret = kissat_value(self.solver.instance.0, x.id);
        }
        match ret {
            i if i == x.id => Some(true),
            i if i == -x.id => Some(false),
            0 => None,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SolveOutcome, Solver};

    #[test]
    fn cancellation_from_another_thread_returns_interrupted() {
        let mut solver = Solver::new();
        let a = solver.var();
        solver.add1(a);
        let handle = solver.cancellation_handle();

        std::thread::spawn(move || handle.cancel()).join().unwrap();
        assert!(matches!(solver.solve(), SolveOutcome::Interrupted));
    }

    #[test]
    fn sat() {
        let mut solver = Solver::new();
        let a = solver.var();
        let b = solver.var();
        solver.add(&[a, b]);
        solver.add(&[a, !b]);
        solver.add(&[!a, !b]);
        let solution = solver.sat();
        assert!(solution.is_some());
        let solution = solution.unwrap();
        assert!(solution.get(a) == Some(true));
        assert!(solution.get(b) == Some(false));
    }

    #[test]
    fn unsat() {
        let mut solver = Solver::new();
        let a = solver.var();
        let b = solver.var();
        solver.add(&[a, b]);
        solver.add(&[a, !b]);
        solver.add(&[!a, !b]);
        solver.add(&[b]);
        let solution = solver.sat();
        assert!(solution.is_none());
    }

    #[test]
    fn miter_mux2() {
        let mut solver = Solver::new();
        let i = solver.var();
        let t = solver.var();
        let e = solver.var();
        // mux2
        let a = solver.ite(i, t, e);
        // constructed manually
        let tmp1 = solver.and(i, t);
        let tmp2 = solver.and(!i, e);
        let b = solver.or(tmp1, tmp2);
        // miter
        let tmp = solver.xor(a, b);
        solver.add1(tmp);
        // should be unsat
        let solution = solver.sat();
        assert!(solution.is_none());
    }

    #[test]
    fn miter_buggy_mux2() {
        let mut solver = Solver::new();
        let i = solver.var();
        let t = solver.var();
        let e = solver.var();
        // mux2
        let a = solver.ite(i, t, e);
        // buggy impl
        let b = solver.and(i, t);
        // miter
        let tmp = solver.xor(a, b);
        solver.add1(tmp);
        // should be sat
        let solution = solver.sat();
        assert!(solution.is_some());
    }
}
