use std::collections::{HashMap, HashSet};

pub struct Encoding {
    // the last variable ID used
    pub last_var: i32,

    // collection of clauses in the encoding
    pub clauses: Vec<Vec<i32>>,

    // maps (section, room) pairs to variable IDs
    pub section_room_vars: HashMap<(usize, usize), i32>,

    // maps (section, time slot) pairs to variable IDs
    pub section_time_vars: HashMap<(usize, usize), i32>,

    // maps variable IDs to problem descriptions with priority levels
    pub problems: HashMap<i32, (u8, String)>,

    // hallpass variables that are allowed to violate constraints, grouped by priority
    pub hallpasses: HashMap<u8, HashSet<i32>>,
}

impl Encoding {
    pub fn new() -> Self {
        Encoding {
            last_var: 0,
            clauses: Vec::new(),
            section_room_vars: HashMap::new(),
            section_time_vars: HashMap::new(),
            problems: HashMap::new(),
            hallpasses: HashMap::new(),
        }
    }

    pub fn new_var(&mut self) -> i32 {
        self.last_var += 1;
        self.last_var
    }

    pub fn new_hallpass(&mut self, priority: u8, problem: String) -> i32 {
        let hallpass = self.new_var();
        self.hallpasses.entry(priority).or_default().insert(hallpass);
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
        let mut sorted_literals = literals.to_vec();
        sorted_literals.sort_unstable();
        let output_vars = self.build_bounded_totalizer_tree(&sorted_literals, output_count);

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
    pub fn solve(&self) -> Result<Option<HashSet<i32>>, String> {
        // Kissat 4's default preprocessing spends about three seconds on each of these dense,
        // structured CNFs. This optimization loop creates a fresh solver for every bound, while
        // the plain configuration solves the same instances directly without that repeated cost.
        let mut solver = kissat::Solver::with_configuration(kissat::Configuration::Plain);

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
        match solver.sat() {
            Some(solution) => {
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
            None => {
                // unsatisfiable
                Ok(None)
            }
        }
    }
}

impl Default for Encoding {
    fn default() -> Self {
        Self::new()
    }
}
