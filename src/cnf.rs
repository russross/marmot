use crate::options::KissatMode;
use kissat::{Configuration, SolveOutcome, Solver};
use std::collections::{HashMap, HashSet};
use std::sync::mpsc;
use std::thread;

#[derive(Clone)]
pub struct Encoding {
    // the last variable ID used
    pub last_var: i32,

    // collection of clauses in the encoding
    pub clauses: Vec<Vec<i32>>,

    // maps (section, room) pairs to variable IDs
    pub section_room_vars: HashMap<(usize, usize), i32>,

    // maps (section, time slot) pairs to variable IDs
    pub section_time_vars: HashMap<(usize, usize), i32>,

    // Each cached variable is linked to all section-time assignments for that faculty day.
    pub faculty_day_vars: HashMap<(usize, u8), i32>,

    // maps variable IDs to problem descriptions with priority levels
    pub problems: HashMap<i32, (u8, String)>,

    // hallpass variables that are allowed to violate constraints, grouped by priority
    pub hallpasses: HashMap<u8, HashSet<i32>>,

    // Creation order associates each criterion with the hallpasses it emits.
    pub hallpass_order: Vec<i32>,
}

impl Encoding {
    pub fn new() -> Self {
        Encoding {
            last_var: 0,
            clauses: Vec::new(),
            section_room_vars: HashMap::new(),
            section_time_vars: HashMap::new(),
            faculty_day_vars: HashMap::new(),
            problems: HashMap::new(),
            hallpasses: HashMap::new(),
            hallpass_order: Vec::new(),
        }
    }

    pub fn new_var(&mut self) -> i32 {
        self.last_var += 1;
        self.last_var
    }

    pub fn new_hallpass(&mut self, priority: u8, problem: String) -> i32 {
        let hallpass = self.new_var();
        self.hallpasses.entry(priority).or_default().insert(hallpass);
        self.hallpass_order.push(hallpass);
        self.problems.insert(hallpass, (priority, problem));
        hallpass
    }

    pub fn add_clause(&mut self, clause: Vec<i32>) {
        self.clauses.push(clause);
    }

    // encode a pairwise at-most-one constraint
    pub fn pairwise_at_most_one(&mut self, literals: &[i32]) {
        // No constraints needed if there are 0 or 1 literals
        if literals.len() <= 1 {
            return;
        }

        // For each pair of literals, add a clause stating they can't both be True
        for i in 0..literals.len() {
            for j in (i + 1)..literals.len() {
                self.add_clause(vec![-literals[i], -literals[j]]);
            }
        }
    }

    // encoding a totalizer at-most-k constraint
    pub fn totalizer_at_most_k(&mut self, literals: &[i32], k: usize, hallpass: Option<i32>) {
        let mut sorted_literals = literals.to_vec();
        sorted_literals.sort_unstable();
        self.ordered_totalizer_at_most_k(&sorted_literals, k, hallpass);
    }

    // Preserve the caller's leaf order while keeping the same counting clauses.
    pub fn ordered_totalizer_at_most_k(&mut self, literals: &[i32], k: usize, hallpass: Option<i32>) {
        let n = literals.len();

        if k >= n {
            // Constraint is trivially true, no clauses needed.
            return;
        }
        if k == 0 {
            // If k=0, all literals must be false. Add unit clauses.
            for &lit in literals {
                match hallpass {
                    Some(h) => self.add_clause(vec![-lit, h]),
                    None => self.add_clause(vec![-lit]),
                }
            }
            return;
        }

        let output_count = k + 1;
        let output_vars = self.build_bounded_totalizer_tree(literals, output_count);

        // At least k+1 true inputs force output_vars[k] true, so forbidding that
        // output enforces the upper bound. Other output assignments are irrelevant.
        match hallpass {
            Some(h) => self.add_clause(vec![-output_vars[k], h]),
            None => self.add_clause(vec![-output_vars[k]]),
        }
    }

    // Build a totalizer tree with only the outputs needed to detect the bound.
    fn build_bounded_totalizer_tree(&mut self, input_literals: &[i32], output_limit: usize) -> Vec<i32> {
        let n = input_literals.len();

        // Base case: if only one literal, the sum is just that literal itself.
        if n == 1 {
            return vec![input_literals[0]];
        }

        if n == 0 {
            return Vec::new();
        }

        let mid = n / 2;
        let left_lits = &input_literals[..mid];
        let right_lits = &input_literals[mid..];

        let left_outputs = self.build_bounded_totalizer_tree(left_lits, output_limit);
        let right_outputs = self.build_bounded_totalizer_tree(right_lits, output_limit);

        self.merge_bounded_totalizer_nodes(&left_outputs, &right_outputs, output_limit)
    }

    // Merge the lower-bound implications from two child totalizers.
    fn merge_bounded_totalizer_nodes(
        &mut self,
        left_outputs: &[i32],
        right_outputs: &[i32],
        output_limit: usize,
    ) -> Vec<i32> {
        let n_left = left_outputs.len();
        let n_right = right_outputs.len();
        let output_count = (n_left + n_right).min(output_limit);

        let mut current_outputs = Vec::with_capacity(output_count);
        for _ in 0..output_count {
            current_outputs.push(self.new_var());
        }

        for i in 0..=n_left {
            for j in 0..=n_right {
                let target_sum = i + j;

                if target_sum == 0 || target_sum > output_count {
                    continue;
                }

                let mut clause = Vec::new();

                if i > 0 {
                    clause.push(-left_outputs[i - 1]);
                }

                if j > 0 {
                    clause.push(-right_outputs[j - 1]);
                }

                clause.push(current_outputs[target_sum - 1]);
                self.add_clause(clause);
            }
        }

        current_outputs
    }

    // solve the current CNF encoding using kissat
    //
    // Returns:
    // - `Ok(Some(HashSet<i32>))` if the problem is satisfiable, with a set of true variables
    // - `Ok(None)` if the problem is unsatisfiable
    // - `Err(String)` if an error occurs during solving
    pub fn solve(&self, mode: KissatMode) -> Result<Option<HashSet<i32>>, String> {
        match mode {
            KissatMode::Default => self.solve_with(Solver::new()),
            KissatMode::Plain => self.solve_with(Solver::with_configuration(Configuration::Plain)),
            KissatMode::Race => self.race(),
        }
    }

    fn race(&self) -> Result<Option<HashSet<i32>>, String> {
        let solvers = [Solver::new(), Solver::with_configuration(Configuration::Plain)];
        let cancellation = solvers.each_ref().map(Solver::cancellation_handle);

        // Each worker loads the same prepared encoding into its own solver.
        // The scope joins both workers before returning to the priority search.
        thread::scope(|scope| {
            let (tx, rx) = mpsc::channel();
            for (index, solver) in solvers.into_iter().enumerate() {
                let tx = tx.clone();
                if let Err(error) = thread::Builder::new().spawn_scoped(scope, move || {
                    let result = self.solve_with(solver);
                    let _ = tx.send((index, result));
                }) {
                    for handle in cancellation {
                        handle.cancel();
                    }
                    return Err(format!("could not start Kissat worker: {error}"));
                }
            }
            drop(tx);

            // SAT (with its decoded model) and UNSAT are both definitive.
            let (index, result) = rx.recv().map_err(|_| "Kissat workers exited without a result".to_string())?;
            if result.is_ok() {
                cancellation[1 - index].cancel();
                return result;
            }
            rx.recv().map_or(result, |(_, second)| second)
        })
    }

    fn solve_with(&self, mut solver: Solver) -> Result<Option<HashSet<i32>>, String> {
        // Create a mapping from our variable indices to kissat variables
        let mut var_map = Vec::with_capacity(self.last_var as usize);

        // Create variables in the kissat solver
        for _ in 0..self.last_var {
            var_map.push(solver.var());
        }

        // Add all clauses to the solver
        for clause in &self.clauses {
            let mut kissat_clause = Vec::with_capacity(clause.len());

            for &lit in clause {
                let var_idx = lit.unsigned_abs() as usize;

                // ensure the variable index is valid
                if var_idx == 0 || var_idx > var_map.len() {
                    return Err(format!("Invalid variable index: {}", var_idx));
                }

                // get the corresponding kissat variable
                let var = var_map[var_idx - 1];

                // determine if this is a positive or negative literal
                if lit > 0 {
                    kissat_clause.push(var); // positive literal
                } else {
                    kissat_clause.push(!var); // negative literal
                }
            }

            // add the clause to the solver
            solver.add(&kissat_clause);
        }

        // solve the instance
        match solver.solve() {
            SolveOutcome::Sat(solution) => {
                // problem is satisfiable, extract the true variables
                let mut true_vars = HashSet::new();

                // Add each true variable to the HashSet
                for (i, var) in var_map.iter().enumerate() {
                    if let Some(true) = solution.get(*var) {
                        // add the variable to the set if it's true
                        true_vars.insert((i + 1) as i32);
                    }
                }

                Ok(Some(true_vars))
            }
            SolveOutcome::Unsat => Ok(None),
            SolveOutcome::Interrupted => Err("Kissat solve was interrupted".to_string()),
        }
    }
}

impl Default for Encoding {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Encoding, KissatMode};

    #[test]
    fn kissat_modes_return_a_model_or_prove_infeasibility() {
        let mut encoding = Encoding::new();
        let a = encoding.new_var();
        let b = encoding.new_var();
        encoding.add_clause(vec![a, b]);
        encoding.add_clause(vec![-a, b]);
        encoding.add_clause(vec![a, -b]);

        for mode in [KissatMode::Default, KissatMode::Plain, KissatMode::Race] {
            let model = encoding.solve(mode).unwrap().unwrap();
            assert!(model.contains(&a) && model.contains(&b));

            let mut impossible = encoding.clone();
            impossible.add_clause(vec![-a, -b]);
            assert!(impossible.solve(mode).unwrap().is_none());
        }
    }
}
