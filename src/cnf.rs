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
    
    // gathered set of "hallpass" variables that are allowed to violate certain constraints
    // this is reset as each priority level is encoded
    pub hallpass: HashSet<i32>,
}

impl Encoding {
    pub fn new() -> Self {
        Encoding {
            last_var: 0,
            clauses: Vec::new(),
            section_room_vars: HashMap::new(),
            section_time_vars: HashMap::new(),
            problems: HashMap::new(),
            hallpass: HashSet::new(),
        }
    }

    pub fn new_var(&mut self) -> i32 {
        self.last_var += 1;
        self.last_var
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
        if literals.is_empty() {
            // No literals to constrain, nothing to do.
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

        // Build the full totalizer tree recursively.
        // The result is a list of n output variables representing the unary sum.
        // output_vars[i] means "at least i+1 literals are true".
        let output_vars = self.build_full_totalizer_tree(literals);

        // Ensure the output_vars list has the expected length n
        if output_vars.len() != n {
            // This case should ideally not be reached if literals is not empty
            if literals.is_empty() {
                // If input was empty, output is empty, k>=0 is trivial
                return;
            } else {
                // Should not happen with non-empty literals
                panic!("Internal Error: Totalizer tree construction failed. Expected {} outputs, got {}", n, output_vars.len());
            }
        }

        // Add the final constraint: "not (at least k+1 literals are true)"
        // This corresponds to negating the (k+1)-th output variable, which is
        // at index k in the 0-indexed list.
        // Since we checked k < n earlier, output_vars[k] is guaranteed to exist.
        match hallpass {
            Some(h) => self.add_clause(vec![-output_vars[k], h]),
            None => self.add_clause(vec![-output_vars[k]]),
        }
    }

    // build the full totalizer tree recursively
    fn build_full_totalizer_tree(&mut self, input_literals: &[i32]) -> Vec<i32> {
        let n = input_literals.len();

        // base Case: If only one literal, the sum is just that literal itself.
        if n == 1 {
            return vec![input_literals[0]];
        }

        // handle empty input case within recursion if needed
        if n == 0 {
            return Vec::new();
        }

        // recursive step: split literals and build subtrees
        let mid = n / 2;
        let left_lits = &input_literals[..mid];
        let right_lits = &input_literals[mid..];

        // recursively build the full trees for children
        let left_outputs = self.build_full_totalizer_tree(left_lits);
        let right_outputs = self.build_full_totalizer_tree(right_lits);

        // merge the results from left and right subtrees without k-pruning
        self.merge_full_totalizer_nodes(&left_outputs, &right_outputs)
    }

    // merge the outputs of two child nodes in the totalizer tree
    fn merge_full_totalizer_nodes(&mut self, left_outputs: &[i32], right_outputs: &[i32]) -> Vec<i32> {
        let n_left = left_outputs.len();
        let n_right = right_outputs.len();
        
        // number of output vars needed
        // the maximum possible sum from this node is n_left + n_right.
        let max_output_index = n_left + n_right;

        // create fresh output variables for this merge node for the full sum
        let mut current_outputs = Vec::with_capacity(max_output_index);
        for _ in 0..max_output_index {
            current_outputs.push(self.new_var());
        }

        // add merging clauses (implements adder logic)
        // formula: (a_i AND b_j) => c_{i+j}  which is equivalent to
        // cnf: (~a_i OR ~b_j OR c_{i+j})
        for i in 0..=n_left {
            for j in 0..=n_right {
                let target_sum = i + j;
                
                // skip if the target sum is 0 (no constraint needed)
                if target_sum == 0 {
                    continue;
                }
                
                // ensure the target sum does not exceed the bounds of the created output vars
                // should not happen if max_output_index = n_left + n_right
                if target_sum > max_output_index {
                    continue;
                }

                // clause: ~a_i V ~b_j V c_{i+j}
                let mut clause = Vec::new();

                // add ~a_i if i > 0
                if i > 0 {
                    // left_outputs[i-1] represents "at least i"
                    clause.push(-left_outputs[i - 1]);
                }

                // add ~b_j if j > 0
                if j > 0 {
                    // right_outputs[j-1] represents "at least j"
                    clause.push(-right_outputs[j - 1]);
                }

                // add c_{i+j}
                // current_outputs[target_sum-1] represents "at least target_sum"
                clause.push(current_outputs[target_sum - 1]);

                // Add the clause to the solver
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
        let mut solver = kissat::Solver::new();

        // Create a mapping from our variable indices to kissat variables
        let mut var_map = Vec::with_capacity(self.last_var as usize + 1);
        
        // Initialize with a placeholder for variable 0 (not used in our encoding)
        var_map.push(solver.var());
        
        // Create variables in the kissat solver
        for i in 1..=self.last_var {
            let v = solver.var();
            if i == 1 {
                // we need something at index 0, which is unused
                var_map.push(v);
            }
            var_map.push(v);
        }

        // Add all clauses to the solver
        for clause in &self.clauses {
            let mut kissat_clause = Vec::with_capacity(clause.len());
            
            for &lit in clause {
                let var_idx = lit.abs() as usize;
                
                // ensure the variable index is valid
                if var_idx >= var_map.len() || var_idx == 0 {
                    return Err(format!("Invalid variable index: {}", var_idx));
                }
                
                // get the corresponding kissat variable
                let var = var_map[var_idx];
                
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
                for i in 1..=self.last_var as usize {
                    match solution.get(var_map[i]) {
                        Some(true) => {
                            // add the variable to the set if it's true
                            true_vars.insert(i as i32);
                        },
                        Some(false) | None => (), // skip false or don't care values
                    }
                }
                
                Ok(Some(true_vars))
            },
            None => {
                // unsatisfiable
                Ok(None)
            },
        }
    }
}

impl Default for Encoding {
    fn default() -> Self {
        Self::new()
    }
}
