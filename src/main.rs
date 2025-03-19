pub mod input;
pub mod print;
pub mod score;
pub mod sat;
pub mod solver;
use self::input::*;
use self::print::*;
use self::sat::*;
use self::solver::*;
use std::collections::HashMap;
use std::time::Instant;

static DEFAULT_DB_PATH: &str = "timetable.db";

fn main() {
    match parse_args() {
        Ok(Opts::Gen(config)) => {
            _ = (|| {
                let input = load_input(&config.db_path, &[])?;
                let mut id = None;
                let mut schedule = if config.starting_id >= 0 {
                    let mut schedule = Schedule::new(&input);
                    load_schedule(
                        &config.db_path,
                        &input,
                        &mut schedule,
                        if config.starting_id == 0 { None } else { Some(config.starting_id) },
                    )?;
                    schedule
                } else {
                    println!("running warmup for {}", sec_to_string(config.warmup_seconds));
                    let Some(schedule) = warmup(&input, config.warmup_seconds) else {
                        return Err("failed to generate a schedule in the warmup stage".to_string());
                    };
                    id = Some(save_schedule(&config.db_path, &input, &schedule, "warmup schedule", None)?);
                    schedule
                };
                let best = solve(&config, &input, &mut schedule, config.solve_seconds, &mut id);
                print_schedule(&input, &best);
                print_problems(&input, &best);
                Ok(())
            })()
            .map_err(|msg| {
                eprintln!("{}", msg);
            });
        }

        Ok(Opts::SAT(config)) => {
            _ = (|| {
                let input = load_input(&config.db_path, &[])?;
                let mut solver = SatSolver::new();
                let schedule = solver.generate_schedule(&input, &config.solver)?;
                print_schedule(&input, &schedule);
                print_problems(&input, &schedule);
                Ok(())
            })()
            .map_err(|msg: String| {
                eprintln!("{}", msg);
            });
        }

        Ok(Opts::Dfs(config)) => {
            _ = (|| {
                let input = load_input(&config.db_path, &[])?;
                let mut schedule = Schedule::new(&input);
                load_schedule(
                    &config.db_path,
                    &input,
                    &mut schedule,
                    if config.starting_id == 0 { None } else { Some(config.starting_id) },
                )?;
                let pre_score = schedule.score;
                let mut save_id = None;
                let mut iterations = 0;
                let start = Instant::now();
                let mut walk = Walk::new(schedule.score);
                loop {
                    let before = schedule.score;
                    print!("running dfs with max depth {}", config.dfs_depth);
                    walk.try_dfs(&input, &mut schedule, config.dfs_depth, false);
                    iterations += 1;
                    if schedule.score < before {
                        let comment = format!(
                            "dfs at depth {}, {} iteration{} over {}",
                            config.dfs_depth,
                            iterations,
                            if iterations == 1 { "" } else { "s" },
                            ms_to_string(start.elapsed().as_millis())
                        );
                        save_id = Some(save_schedule(&config.db_path, &input, &schedule, &comment, save_id)?);
                    }
                    if schedule.score >= before || !config.repeat {
                        break;
                    }
                }
                if schedule.score < pre_score {
                    println!("score improved from {} to {} over {} iterations", pre_score, schedule.score, iterations);
                }
                Ok(())
            })()
            .map_err(|msg: String| {
                eprintln!("{}", msg);
            });
        }

        Ok(Opts::Print(config)) => {
            _ = (|| {
                let input = load_input(&config.db_path, &[])?;
                let mut schedule = Schedule::new(&input);
                load_schedule(
                    &config.db_path,
                    &input,
                    &mut schedule,
                    if config.starting_id == 0 { None } else { Some(config.starting_id) },
                )?;
                println!("score: {}", schedule.score);
                print_schedule(&input, &schedule);
                print_problems(&input, &schedule);
                Ok(())
            })()
            .map_err(|msg: String| {
                eprintln!("{}", msg);
            });
        }

        Ok(Opts::Dump(config)) => {
            _ = (|| {
                let input = load_input(&config.db_path, &[])?;
                dump_input(&[], &input);
                Ok(())
            })()
            .map_err(|msg: String| {
                eprintln!("{}", msg);
            });
        }

        Err(msg) => {
            if !msg.is_empty() {
                eprintln!("{msg}");
            }
            print_usage(std::env::args().nth(1));
        }
    }
}

fn parse_args() -> Result<Opts, String> {
    let mut parser = CliParser::new()?;

    match parser.command.as_ref() {
        "gen" => {
            let mut opts = GenOpts::default();
            parser.string("-d", "--db-path", &mut opts.db_path)?;
            parser.duration("-w", "--warmup", &mut opts.warmup_seconds)?;
            parser.int64("-i", "--id", &mut opts.starting_id)?;
            parser.duration("-t", "--time", &mut opts.solve_seconds)?;
            parser.duration("-g", "--rehome-global", &mut opts.rehome_global_seconds)?;
            parser.duration("-l", "--rehome-local", &mut opts.rehome_local_seconds)?;
            parser.duration("-u", "--update", &mut opts.update_seconds)?;
            parser.float("-n", "--bias-min", &mut opts.bias_min)?;
            parser.float("-x", "--bias-max", &mut opts.bias_max)?;
            parser.float("-s", "--bias-step", &mut opts.bias_step)?;
            parser.uint("-p", "--dfs-depth", &mut opts.dfs_depth)?;
            parser.boolean("-f", "--fallback", &mut opts.fallback)?;
            parser.leftover()?;
            Ok(Opts::Gen(opts))
        }

        "sat" => {
            let mut opts = SATOpts::default();
            parser.string("-d", "--db-path", &mut opts.db_path)?;
            parser.string("-s", "--solver", &mut opts.solver)?;
            parser.leftover()?;
            Ok(Opts::SAT(opts))
        }

        "dfs" => {
            let mut opts = DfsOpts::default();
            parser.string("-d", "--db-path", &mut opts.db_path)?;
            parser.int64("-i", "--id", &mut opts.starting_id)?;
            parser.uint("-p", "--dfs-depth", &mut opts.dfs_depth)?;
            parser.boolean("-r", "--repeat", &mut opts.repeat)?;
            parser.leftover()?;
            Ok(Opts::Dfs(opts))
        }

        "print" => {
            let mut opts = PrintOpts::default();
            parser.string("-d", "--db-path", &mut opts.db_path)?;
            parser.int64("-i", "--id", &mut opts.starting_id)?;
            parser.leftover()?;
            Ok(Opts::Print(opts))
        }

        "dump" => {
            let mut opts = DumpOpts::default();
            parser.string("-d", "--db-path", &mut opts.db_path)?;
            parser.leftover()?;
            Ok(Opts::Dump(opts))
        }

        cmd => Err(format!("Error: unknown command \"{cmd}\"")),
    }
}

enum Opts {
    Gen(GenOpts),
    SAT(SATOpts),
    Dfs(DfsOpts),
    Print(PrintOpts),
    Dump(DumpOpts),
}

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
        }
    }
}

pub struct SATOpts {
    pub db_path: String,
    pub solver: String,
}

impl Default for SATOpts {
    fn default() -> Self {
        Self { db_path: DEFAULT_DB_PATH.to_string(), solver: "cadical".to_string() }
    }
}

pub struct PrintOpts {
    pub db_path: String,
    pub starting_id: i64,
}

impl Default for PrintOpts {
    fn default() -> Self {
        Self { db_path: DEFAULT_DB_PATH.to_string(), starting_id: 0 }
    }
}

pub struct DumpOpts {
    pub db_path: String,
}

impl Default for DumpOpts {
    fn default() -> Self {
        Self { db_path: DEFAULT_DB_PATH.to_string() }
    }
}

pub struct DfsOpts {
    pub db_path: String,
    pub starting_id: i64,
    pub dfs_depth: usize,
    pub repeat: bool,
}

impl Default for DfsOpts {
    fn default() -> Self {
        Self { db_path: DEFAULT_DB_PATH.to_string(), starting_id: 0, dfs_depth: 4, repeat: true }
    }
}

fn print_usage(command: Option<String>) {
    match command.as_deref() {
        Some("gen") => {
            let default = GenOpts::default();
            eprintln!("Usage: marmot gen [options]");
            eprintln!();
            eprintln!("Options:");
            eprintln!("  -d, --db-path <path>           Database path (default: {})", default.db_path);
            eprintln!(
                "  -w, --warmup <duration>        Warmup period (default: {})",
                sec_to_string(default.warmup_seconds)
            );
            eprintln!("  -i, --id <int>                 ID of schedule to start from (0 to use best in DB)");
            eprintln!(
                "  -t, --time <duration>          Total time (default: {})",
                sec_to_string(default.solve_seconds)
            );
            eprintln!(
                "  -g, --rehome-global <duration> Global rehoming interval (default: {})",
                sec_to_string(default.rehome_global_seconds)
            );
            eprintln!(
                "  -l, --rehome-local <duration>  Local rehoming interval (default: {})",
                sec_to_string(default.rehome_local_seconds)
            );
            eprintln!(
                "  -u, --update <duration>        Status update interval (default: {})",
                sec_to_string(default.update_seconds)
            );
            eprintln!("  -n, --bias-min <float>         Minimum bias (default: {})", default.bias_min);
            eprintln!("  -x, --bias-max <float>         Maximum bias (default: {})", default.bias_max);
            eprintln!("  -s, --bias-step <float>        Bias step (default: {})", default.bias_step);
            eprintln!("  -p, --dfs-depth <int>          DFS depth (default: {})", default.dfs_depth);
            eprintln!(
                "  -f, --fallback <bool>          Rehome to fallback instead of new warmup (default: {})",
                default.fallback
            );
        }

        Some("sat") => {
            let default = SATOpts::default();
            eprintln!("Usage: marmot sat [options]");
            eprintln!();
            eprintln!("Options:");
            eprintln!("  -d, --db-path <path>           Database path (default: {})", default.db_path);
            eprintln!("  -s, --solver <name>            SAT solver to use: kissat or cadical (default: {})", default.solver);
        }

        Some("dfs") => {
            let default = DfsOpts::default();
            eprintln!("Usage: marmot dfs [options]");
            eprintln!();
            eprintln!("Options:");
            eprintln!("  -d, --db-path <path>           Database path (default: {})", default.db_path);
            eprintln!("  -i, --id <int>                 ID of schedule to start from (0 to use best in DB)");
            eprintln!("  -p, --dfs-depth <int>          DFS depth (default: {})", default.dfs_depth);
            eprintln!("  -r, --repeat <bool>            Repeat automatically on success (default: {})", default.repeat);
        }

        Some("print") => {
            let default = PrintOpts::default();
            eprintln!("Usage: marmot print [options]");
            eprintln!();
            eprintln!("Options:");
            eprintln!("  -d, --db-path <path>           Database path (default: {})", default.db_path);
            eprintln!(
                "  -i, --id <int>                 ID of schedule to use (0 to use best in DB, default: {})",
                default.starting_id
            );
        }

        Some("dump") => {
            let default = DumpOpts::default();
            eprintln!("Usage: marmot dump [options]");
            eprintln!();
            eprintln!("Options:");
            eprintln!("  -d, --db-path <path>           Database path (default: {})", default.db_path);
        }

        _ => {
            eprintln!("Usage: marmot <command> [options]");
            eprintln!();
            eprintln!("Commands:");
            eprintln!("  gen        Generate a new schedule from scratch");
            eprintln!("  sat        Generate a new schedule using SAT");
            eprintln!("  dfs        Try to improve a schedule using bounded DFS");
            eprintln!("  print      Print a schedule to the console");
            eprintln!("  dump       Dump the input data to the console");
            eprintln!();
            eprintln!("For more help run: marmot <command> -h");
        }
    }
}

struct CliParser {
    command: String,
    pairs: HashMap<String, String>,
}

impl CliParser {
    fn new() -> Result<Self, String> {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            return Err("Error: no command specified".to_string());
        }
        let command = &args[1];
        if args[2..].contains(&"-h".to_string()) {
            return Err(String::new());
        }
        if args.len() % 2 == 1 {
            return Err(format!("Error: 'marmot {}' options must each have a value, e.g., -t 30m", command));
        }
        let mut pairs = HashMap::new();
        for pair in args[2..].chunks_exact(2) {
            pairs.insert(pair[0].clone(), pair[1].clone());
        }

        Ok(CliParser { command: args[1].clone(), pairs })
    }

    fn leftover(&self) -> Result<(), String> {
        // form an error based on the first unprocessed option we happen to find
        if let Some((key, val)) = self.pairs.iter().next() {
            return Err(format!("Error: 'marmot {}' with unknown option: {} {}", self.command, key, val));
        }
        Ok(())
    }

    fn pair(&mut self, short: &str, long: &str) -> Option<(String, String)> {
        self.pairs
            .remove(short)
            .map(|val| (short.to_string(), val))
            .or_else(|| self.pairs.remove(long).map(|val| (long.to_string(), val)))
    }

    fn string(&mut self, short: &str, long: &str, s: &mut String) -> Result<(), String> {
        if let Some((_, val)) = self.pair(short, long) {
            *s = val;
        }

        Ok(())
    }

    fn duration(&mut self, short: &str, long: &str, seconds: &mut u64) -> Result<(), String> {
        if let Some((key, val)) = self.pair(short, long) {
            match string_to_sec(val.as_str()) {
                Ok(n) => *seconds = n,
                Err(msg) => return Err(format!("Error parsing option {key}: {msg}")),
            }
        }

        Ok(())
    }

    fn float(&mut self, short: &str, long: &str, target: &mut f64) -> Result<(), String> {
        if let Some((key, val)) = self.pair(short, long) {
            match val.parse() {
                Ok(n) => *target = n,
                Err(msg) => return Err(format!("Error parsing option {key}: {msg}")),
            }
        }

        Ok(())
    }

    fn int64(&mut self, short: &str, long: &str, target: &mut i64) -> Result<(), String> {
        if let Some((key, val)) = self.pair(short, long) {
            match val.parse() {
                Ok(n) => *target = n,
                Err(msg) => return Err(format!("Error parsing option {key}: {msg}")),
            }
        }

        Ok(())
    }

    fn uint(&mut self, short: &str, long: &str, target: &mut usize) -> Result<(), String> {
        if let Some((key, val)) = self.pair(short, long) {
            match val.parse() {
                Ok(n) => *target = n,
                Err(msg) => return Err(format!("Error parsing option {key}: {msg}")),
            }
        }

        Ok(())
    }

    fn boolean(&mut self, short: &str, long: &str, target: &mut bool) -> Result<(), String> {
        if let Some((key, val)) = self.pair(short, long) {
            match val.parse() {
                Ok(n) => *target = n,
                Err(msg) => return Err(format!("Error parsing option {key}: {msg}")),
            }
        }

        Ok(())
    }
}
