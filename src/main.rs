pub mod cnf;
pub mod error;
pub mod input;
pub mod print;
pub mod sat_criteria;
pub mod sat_encoders;
pub mod sat_solver;
pub mod score;
pub mod solver;
use self::error::Result;
use self::input::*;
use self::print::*;
use self::sat_solver::*;
use self::solver::*;
use std::collections::HashMap;
use std::time::Instant;

static DEFAULT_DB_PATH: &str = "../data/timetable.db";

fn main() {
    if let Err(e) = dispatch_subcommands() {
        eprintln!("{}", e);
    };
}

fn dispatch_subcommands() -> Result<()> {
    match parse_args() {
        Ok(Opts::Gen(config)) => {
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
                    return Err("failed to generate a schedule in the warmup stage".into());
                };
                id = Some(save_schedule(&config.db_path, &input, &schedule, "warmup schedule", None)?);
                schedule
            };
            let best = solve(&config, &input, &mut schedule, config.solve_seconds, &mut id);
            print_schedule(&input, &best);
            print_problems(&input, &best);
            Ok(())
        }

        Ok(Opts::Sat(config)) => {
            let input = load_input(&config.db_path, &[])?;
            let schedule = generate_schedule(&config, &input)?;
            print_schedule(&input, &schedule);
            print_problems(&input, &schedule);
            Ok(())
        }

        Ok(Opts::Dfs(config)) => {
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
        }

        Ok(Opts::Print(config)) => {
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
        }

        Ok(Opts::Dump(config)) => {
            let input = load_input(&config.db_path, &[])?;
            dump_input(&[], &input);
            Ok(())
        }

        Ok(Opts::Tweak(config)) => {
            let input = load_input(&config.db_path, &[])?;
            let mut schedule = Schedule::new(&input);
            load_schedule(
                &config.db_path,
                &input,
                &mut schedule,
                if config.starting_id == 0 { None } else { Some(config.starting_id) },
            )?;

            let mut parsed_tweaks = Vec::new();
            for tweak in &config.tweaks {
                // Look up section by name
                let section_idx = input.sections.iter().position(|s| s.name == tweak.section)
                    .ok_or_else(|| format!("Section '{}' not found", tweak.section))?;

                // Look up time slot by name
                let time_slot_idx = input.time_slots.iter().position(|ts| ts.name == tweak.time_slot)
                    .ok_or_else(|| format!("Time slot '{}' not found", tweak.time_slot))?;

                // Look up room by name, handle "-" as no room
                let room_idx = if tweak.room == "-" {
                    None
                } else {
                    Some(input.rooms.iter().position(|r| r.name == tweak.room)
                        .ok_or_else(|| format!("Room '{}' not found", tweak.room))?)
                };

                parsed_tweaks.push((section_idx, time_slot_idx, room_idx));
            }

            // Validate tweaks
            for (section_idx, time_slot_idx, room_idx) in &parsed_tweaks {
                let section = &input.sections[*section_idx];
                
                // Check if time slot is valid for this section
                if !section.time_slots.iter().any(|ts| ts.time_slot == *time_slot_idx) {
                    return Err(format!("Time slot '{}' is not valid for section '{}'", 
                        input.time_slots[*time_slot_idx].name, section.name).into());
                }

                // Check if room assignment is valid
                match room_idx {
                    Some(room) => {
                        if !section.rooms.iter().any(|r| r.room == *room) {
                            return Err(format!("Room '{}' is not valid for section '{}'", 
                                input.rooms[*room].name, section.name).into());
                        }
                    }
                    None => {
                        if !section.rooms.is_empty() {
                            return Err(format!("Section '{}' requires a room, cannot use '-'", section.name).into());
                        }
                    }
                }
            }

            // Apply tweaks
            for (section_idx, time_slot_idx, room_idx) in &parsed_tweaks {
                let _undo = move_section(&input, &mut schedule, *section_idx, *time_slot_idx, room_idx);
            }

            // Create comment describing the tweaks
            let tweak_descriptions: Vec<String> = config.tweaks.iter()
                .map(|t| format!("{}→{},{}", t.section, t.room, t.time_slot))
                .collect();
            let comment = format!("tweaked: {}", tweak_descriptions.join("; "));

            // Save the new schedule
            save_schedule(&config.db_path, &input, &schedule, &comment, None)?;

            // Print the result
            print_schedule(&input, &schedule);
            print_problems(&input, &schedule);
            Ok(())
        }

        Err(msg) => {
            print_usage(std::env::args().nth(1));
            Err(msg)
        }
    }
}

fn parse_args() -> Result<Opts> {
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
            let mut opts = SatOpts::default();
            parser.string("-d", "--db-path", &mut opts.db_path)?;
            parser.leftover()?;
            Ok(Opts::Sat(opts))
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

        "tweak" => {
            let mut opts = TweakOpts::default();
            parser.string("-d", "--db-path", &mut opts.db_path)?;
            parser.int64("-i", "--id", &mut opts.starting_id)?;
            while parser.tweak_specs("-t", "--tweak", &mut opts.tweaks)? {}
            if opts.tweaks.is_empty() {
                return Err("Error: at least one tweak must be specified with -t/--tweak".into());
            }
            parser.leftover()?;
            Ok(Opts::Tweak(opts))
        }

        cmd => Err(format!("Error: unknown command \"{}\"", cmd).into()),
    }
}

enum Opts {
    Gen(GenOpts),
    Sat(SatOpts),
    Dfs(DfsOpts),
    Print(PrintOpts),
    Dump(DumpOpts),
    Tweak(TweakOpts),
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

pub struct SatOpts {
    pub db_path: String,
}

impl Default for SatOpts {
    fn default() -> Self {
        Self { db_path: DEFAULT_DB_PATH.to_string() }
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

#[derive(Debug)]
pub struct TweakSpec {
    pub section: String,
    pub room: String,
    pub time_slot: String,
}

pub struct TweakOpts {
    pub db_path: String,
    pub starting_id: i64,
    pub tweaks: Vec<TweakSpec>,
}

pub struct DfsOpts {
    pub db_path: String,
    pub starting_id: i64,
    pub dfs_depth: usize,
    pub repeat: bool,
}

impl Default for TweakOpts {
    fn default() -> Self {
        Self { db_path: DEFAULT_DB_PATH.to_string(), starting_id: 0, tweaks: Vec::new() }
    }
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
            let default = SatOpts::default();
            eprintln!("Usage: marmot sat [options]");
            eprintln!();
            eprintln!("Options:");
            eprintln!("  -d, --db-path <path>           Database path (default: {})", default.db_path);
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

        Some("tweak") => {
            let default = TweakOpts::default();
            eprintln!("Usage: marmot tweak [options]");
            eprintln!();
            eprintln!("Options:");
            eprintln!("  -d, --db-path <path>           Database path (default: {})", default.db_path);
            eprintln!("  -i, --id <int>                 ID of schedule to start from (0 to use best in DB)");
            eprintln!("  -t, --tweak <section,room,time> Move a section to specified room and time (repeatable)");
            eprintln!();
            eprintln!("Examples:");
            eprintln!("  marmot tweak -i 123 -t \"CS 3400-01,Smith 108,MWF0900+50\" -t \"MATH 3400-01,-,TR1030+75\"");
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
            eprintln!("  tweak      Make manual adjustments to an existing schedule");
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
    fn new() -> Result<Self> {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            return Err("Error: no command specified".into());
        }
        let command = &args[1];
        if args[2..].contains(&"-h".to_string()) {
            return Err(String::new().into());
        }
        if args.len() % 2 == 1 {
            return Err(format!("Error: 'marmot {}' options must each have a value, e.g., -t 30m", command).into());
        }
        let mut pairs = HashMap::new();
        for pair in args[2..].chunks_exact(2) {
            pairs.insert(pair[0].clone(), pair[1].clone());
        }

        Ok(CliParser { command: args[1].clone(), pairs })
    }

    fn leftover(&self) -> Result<()> {
        // form an error based on the first unprocessed option we happen to find
        if let Some((key, val)) = self.pairs.iter().next() {
            return Err(format!("Error: 'marmot {}' with unknown option: {} {}", self.command, key, val).into());
        }
        Ok(())
    }

    fn pair(&mut self, short: &str, long: &str) -> Option<(String, String)> {
        self.pairs
            .remove(short)
            .map(|val| (short.to_string(), val))
            .or_else(|| self.pairs.remove(long).map(|val| (long.to_string(), val)))
    }

    fn string(&mut self, short: &str, long: &str, s: &mut String) -> Result<()> {
        if let Some((_, val)) = self.pair(short, long) {
            *s = val;
        }

        Ok(())
    }

    fn duration(&mut self, short: &str, long: &str, seconds: &mut u64) -> Result<()> {
        if let Some((key, val)) = self.pair(short, long) {
            match string_to_sec(val.as_str()) {
                Ok(n) => *seconds = n,
                Err(msg) => return Err(format!("Error parsing option {}: {}", key, msg).into()),
            }
        }

        Ok(())
    }

    fn float(&mut self, short: &str, long: &str, target: &mut f64) -> Result<()> {
        if let Some((key, val)) = self.pair(short, long) {
            match val.parse() {
                Ok(n) => *target = n,
                Err(msg) => return Err(format!("Error parsing option {}: {}", key, msg).into()),
            }
        }

        Ok(())
    }

    fn int64(&mut self, short: &str, long: &str, target: &mut i64) -> Result<()> {
        if let Some((key, val)) = self.pair(short, long) {
            match val.parse() {
                Ok(n) => *target = n,
                Err(msg) => return Err(format!("Error parsing option {}: {}", key, msg).into()),
            }
        }

        Ok(())
    }

    fn uint(&mut self, short: &str, long: &str, target: &mut usize) -> Result<()> {
        if let Some((key, val)) = self.pair(short, long) {
            match val.parse() {
                Ok(n) => *target = n,
                Err(msg) => return Err(format!("Error parsing option {}: {}", key, msg).into()),
            }
        }

        Ok(())
    }

    fn boolean(&mut self, short: &str, long: &str, target: &mut bool) -> Result<()> {
        if let Some((key, val)) = self.pair(short, long) {
            match val.parse() {
                Ok(n) => *target = n,
                Err(msg) => return Err(format!("Error parsing option {}: {}", key, msg).into()),
            }
        }

        Ok(())
    }

    fn tweak_specs(&mut self, short: &str, long: &str, tweaks: &mut Vec<TweakSpec>) -> Result<bool> {
        if let Some((key, val)) = self.pair(short, long) {
            let parts: Vec<&str> = val.split(',').collect();
            if parts.len() != 3 {
                return Err(format!("Error parsing option {}: expected 3 comma-separated values (section,room,time), got {}", key, parts.len()).into());
            }
            tweaks.push(TweakSpec {
                section: parts[0].to_string(),
                room: parts[1].to_string(),
                time_slot: parts[2].to_string(),
            });
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
