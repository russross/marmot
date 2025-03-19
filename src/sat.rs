use super::input::*;
use super::solver::*;
use std::collections::HashMap;
use rustsat::encodings::am1::{Encode, Pairwise};
use rustsat::instances::{BasicVarManager, ManageVars, Cnf};
use rustsat::solvers::{Solve, SolverResult};
use rustsat::types::{Var, Lit, Clause, Assignment};
use rustsat_cadical::CaDiCaL;
use rustsat_kissat::Kissat;

pub struct SatSolver {
    // Variables for time slot assignments
    pub section_time_vars: HashMap<(usize, usize), Var>, // (section, time_slot) -> var
    
    // Variables for room assignments
    pub section_room_vars: HashMap<(usize, usize), Var>, // (section, room) -> var
    
    // Reverse lookups
    pub var_to_section_time: HashMap<Var, (usize, usize)>,
    pub var_to_section_room: HashMap<Var, (usize, usize)>,
    
    // RustSAT encoding objects
    pub var_manager: BasicVarManager,
    pub cnf: Cnf,
}

impl SatSolver {
    pub fn new() -> Self {
        Self {
            section_time_vars: HashMap::new(),
            section_room_vars: HashMap::new(),
            var_to_section_time: HashMap::new(),
            var_to_section_room: HashMap::new(),
            var_manager: BasicVarManager::default(),
            cnf: Cnf::default(),
        }
    }

    // Generates a schedule from an input
    pub fn generate_schedule(&mut self, input: &Input, backend: &str) -> Result<Schedule, String> {
        self.initialize_variables(input);
        self.encode_basic_constraints(input)?;
        self.encode_room_conflicts(input);
        self.encode_hard_conflicts(input);

        println!("Created CNF with {} variables and {} clauses", self.var_manager.n_used(), self.cnf.len());
        
        // Solve and decode
        match backend {
            "kissat" => {
                let mut solver = Kissat::default();
                solver.add_cnf(self.cnf.clone()).map_err(|e| format!("{}", e))?;
                
                match solver.solve().map_err(|e| format!("{}", e))? {
                    SolverResult::Sat => {
                        let solution = solver.full_solution().map_err(|e| format!("{}", e))?;
                        Ok(self.decode_solution(solution, input))
                    },
                    _ => {
                        Err("Schedule is unsatisfiable with current constraints".to_string())
                    }
                }
            },
            "cadical" => {
                let mut solver = CaDiCaL::default();
                solver.add_cnf(self.cnf.clone()).map_err(|e| format!("{}", e))?;
                
                match solver.solve().map_err(|e| format!("{}", e))? {
                    SolverResult::Sat => {
                        let solution = solver.full_solution().map_err(|e| format!("{}", e))?;
                        Ok(self.decode_solution(solution, input))
                    },
                    _ => {
                        Err("Schedule is unsatisfiable with current constraints".to_string())
                    }
                }
            },
            _ => {
                Err(format!("Unknown SAT solver: {} (valid values are cadical and kissat)", backend))
            }
        }
    }
    
    pub fn initialize_variables(&mut self, input: &Input) {
        let mut time_var_count = 0;
        let mut room_var_count = 0;
        
        // Create variables for sections and time slots
        for (section_idx, section) in input.sections.iter().enumerate() {
            for ts in &section.time_slots {
                // Create variable for section scheduled at this time slot
                let time_slot = ts.time_slot;
                let var = self.var_manager.new_var();
                self.section_time_vars.insert((section_idx, time_slot), var);
                self.var_to_section_time.insert(var, (section_idx, time_slot));
                time_var_count += 1;
            }
            
            // Create variables for section and rooms
            if !section.rooms.is_empty() {
                for r in &section.rooms {
                    let room = r.room;
                    // Create variable for section scheduled in this room
                    let var = self.var_manager.new_var();
                    self.section_room_vars.insert((section_idx, room), var);
                    self.var_to_section_room.insert(var, (section_idx, room));
                    room_var_count += 1;
                }
            }
        }
        
        println!("Created {} time slot variables and {} room variables", time_var_count, room_var_count);
    }
    
    pub fn encode_basic_constraints(&mut self, input: &Input) -> Result<(), String> {
        // For each section: exactly one time slot
        for (section_idx, section) in input.sections.iter().enumerate() {
            // get the SAT variables for these time slots
            let time_slots_for_section: Vec<_> = section.time_slots.iter().map(|ts| ts.time_slot).collect();
            let time_vars: Vec<Lit> = time_slots_for_section.iter()
                .filter_map(|&ts| self.section_time_vars.get(&(section_idx, ts)))
                .map(|&var| var.pos_lit())
                .collect();
            
            assert!(time_vars.len() == section.time_slots.len());
            
            // at least one time slot
            self.cnf.add_clause(Clause::from_iter(time_vars.iter().copied()));
            
            // at most one time slot using pairwise encoding
            Pairwise::from_iter(time_vars.iter().copied()).encode(&mut self.cnf, &mut self.var_manager).map_err(|e| format!("{}", e))?;
        }
        
        // for each section with rooms: exactly one room
        for section_idx in 0..input.sections.len() {
            let rooms_for_section: Vec<_> = input.sections[section_idx].rooms.iter()
                .map(|r| r.room)
                .collect();

            if rooms_for_section.is_empty() {
                continue;
            }
                
            // get the SAT variables for these rooms
            let room_vars: Vec<Lit> = rooms_for_section.iter()
                .filter_map(|&r| self.section_room_vars.get(&(section_idx, r)))
                .map(|&var| Lit::positive(u32::try_from(var.idx()).unwrap()))
                .collect();
            
            assert!(rooms_for_section.len() == room_vars.len());
            
            // at least one room
            self.cnf.add_clause(Clause::from_iter(room_vars.iter().copied()));
            
            // at most one room using pairwise encoding
            Pairwise::from_iter(room_vars.iter().copied()).encode(&mut self.cnf, &mut self.var_manager).map_err(|e| format!("{}", e))?;
        }
        
        Ok(())
    }

    // Encode constraints that prevent room conflicts: no two sections can be in the same room at overlapping times
    pub fn encode_room_conflicts(&mut self, input: &Input) {
        let mut conflict_clauses = 0;
        
        // for each room
        for room_idx in 0..input.rooms.len() {
            // find all sections that could be scheduled in this room
            let mut sections_for_room: Vec<usize> = Vec::new();
            
            for (section_idx, section) in input.sections.iter().enumerate() {
                if section.rooms.iter().any(|r| r.room == room_idx) {
                    sections_for_room.push(section_idx);
                }
            }
            
            // no possible conflicts if there are not at least two sections
            if sections_for_room.len() < 2 {
                continue;
            }
            
            // for each pair of sections
            for i in 0..sections_for_room.len() {
                let section_a_idx = sections_for_room[i];
                
                for j in (i+1)..sections_for_room.len() {
                    let section_b_idx = sections_for_room[j];
                    
                    // skip pairs that are already in hard conflict with each other
                    // they will be covered in the hard conflict encoding
                    if input.sections[section_a_idx].hard_conflicts.contains(&section_b_idx) || 
                       input.sections[section_b_idx].hard_conflicts.contains(&section_a_idx) {
                        continue;
                    }
                    
                    // for each time slot pair
                    for &time_a in &input.sections[section_a_idx].time_slots.iter()
                        .map(|ts| ts.time_slot)
                        .collect::<Vec<_>>() {
                        
                        for &time_b in &input.sections[section_b_idx].time_slots.iter()
                            .map(|ts| ts.time_slot)
                            .collect::<Vec<_>>() {
                            
                            // skip if the time slots don't conflict
                            if !input.time_slot_conflicts[time_a][time_b] {
                                continue;
                            }
                            
                            // get the section-time and section-room variables
                            let var_a_time = self.section_time_vars.get(&(section_a_idx, time_a)).unwrap();
                            let var_b_time = self.section_time_vars.get(&(section_b_idx, time_b)).unwrap();
                            let var_a_room = self.section_room_vars.get(&(section_a_idx, room_idx)).unwrap();
                            let var_b_room = self.section_room_vars.get(&(section_b_idx, room_idx)).unwrap();
                            
                            // add conflict clause: ~(A_time && A_room && B_time && B_room)
                            // which is equivalent to (!A_time || !A_room || !B_time || !B_room)
                            self.cnf.add_clause(Clause::from_iter([
                                var_a_time.neg_lit(),
                                var_a_room.neg_lit(),
                                var_b_time.neg_lit(),
                                var_b_room.neg_lit(),
                            ]));
                            
                            conflict_clauses += 1;
                        }
                    }
                }
            }
        }
        
        println!("Added {} room conflict clauses", conflict_clauses);
    }
    
    // encode hard conflict constraints
    pub fn encode_hard_conflicts(&mut self, input: &Input) {
        let mut hard_conflict_clauses = 0;
        
        // for each section
        for section_idx in 0..input.sections.len() {
            let section = &input.sections[section_idx];
            
            // get time slots for this section
            let time_slots: Vec<usize> = section.time_slots.iter()
                .map(|ts| ts.time_slot)
                .collect();
            
            // for each hard conflict
            for &conflict_idx in &section.hard_conflicts {
                let conflict_section = &input.sections[conflict_idx];
                
                // get time slots for the other section
                let conflict_time_slots: Vec<usize> = conflict_section.time_slots.iter()
                    .map(|ts| ts.time_slot)
                    .collect();
                
                // for each time slot pair
                for &time_a in &time_slots {
                    for &time_b in &conflict_time_slots {
                        // skip if the time slots don't conflict
                        if !input.time_slot_conflicts[time_a][time_b] {
                            continue;
                        }
                        
                        // get the section-time variables
                        let var_a_time = self.section_time_vars.get(&(section_idx, time_a)).unwrap();
                        let var_b_time = self.section_time_vars.get(&(conflict_idx, time_b)).unwrap();
                        
                        // add conflict clause: ~(A_time && B_time)
                        // which is equivalent to: (!A_time || !B_time)
                        self.cnf.add_clause(Clause::from_iter([
                            var_a_time.neg_lit(),
                            var_b_time.neg_lit(),
                        ]));
                        
                        hard_conflict_clauses += 1;
                    }
                }
            }
        }
        
        println!("Added {} hard conflict clauses", hard_conflict_clauses);
    }
    
    // decodes a solution from the solver into a Schedule
    pub fn decode_solution(&self, solution: Assignment, input: &Input) -> Schedule {
        let mut schedule = Schedule::new(input);
        
        // group assignments by section
        let mut section_assignments: HashMap<usize, (Option<usize>, Option<usize>)> = HashMap::new();
        
        // process time slot assignments
        for (&var, &(section, time_slot)) in &self.var_to_section_time {
            if solution.var_value(var).to_bool_with_def(false) {
                let entry = section_assignments.entry(section).or_insert((None, None));
                entry.0 = Some(time_slot);
            }
        }
        
        // process room assignments
        for (&var, &(section, room)) in &self.var_to_section_room {
            if solution.var_value(var).to_bool_with_def(false) {
                let entry = section_assignments.entry(section).or_insert((None, None));
                entry.1 = Some(room);
            }
        }
        
        // place sections in schedule using move_section to properly update scores
        for (section, (time_slot_opt, room_opt)) in section_assignments {
            if let Some(time_slot) = time_slot_opt {
                _ = move_section(input, &mut schedule, section, time_slot, &room_opt);
            }
        }
        
        schedule
    }
}
