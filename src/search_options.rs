const DEFAULT_DB_PATH: &str = "../data/timetable.db";
pub struct GenOpts {
    pub db_path: String,
    pub warmup_seconds: u64,
    pub starting_id: i64,
    pub solve_seconds: u64,
    pub rehome_global_seconds: u64,
    pub rehome_local_seconds: u64,
    pub update_seconds: u64,
    pub bias_min: f64,
    pub bias_max: f64,
    pub bias_step: f64,
    pub dfs_depth: usize,
    pub fallback: bool,
    pub show_faculty_preference_priorities: bool,
}

impl Default for GenOpts {
    fn default() -> Self {
        Self {
            db_path: DEFAULT_DB_PATH.to_string(),
            warmup_seconds: 1,
            starting_id: -1,
            solve_seconds: 30 * 60,
            rehome_global_seconds: 5 * 60,
            rehome_local_seconds: 2 * 60,
            update_seconds: 5,
            bias_min: -10.0,
            bias_max: 10.0,
            bias_step: 0.125,
            dfs_depth: 2,
            fallback: false,
            show_faculty_preference_priorities: false,
        }
    }
}
