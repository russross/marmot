use crate::error::Result;
use crate::options::*;
use crate::print::*;
use crate::sat_solver::*;
use crate::score::MAX_PRIORITY;
use crate::search::*;
use crate::solver::*;
use crate::sqlite::*;
use std::collections::HashMap;
use std::process::ExitCode;
use std::time::Instant;

static DEFAULT_DB_PATH: &str = "../data/timetable.db";

pub fn run() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    match &args[1..] {
        [help] if matches!(help.as_str(), "-h" | "--help") => {
            print_usage(None);
            return ExitCode::SUCCESS;
        }
        [command, help]
            if matches!(command.as_str(), "gen" | "sat" | "dfs" | "print" | "dump" | "tweak")
                && matches!(help.as_str(), "-h" | "--help") =>
        {
            print_usage(Some(command.clone()));
            return ExitCode::SUCCESS;
        }
        _ => {}
    }

    match dispatch_subcommands(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn dispatch_subcommands(args: &[String]) -> Result<()> {
    match parse_args(args) {
        Ok(Opts::Gen(config)) => {
            let input = load_input(&config.db_path, &[], config.show_faculty_preference_priorities)?;
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
            let best = solve(&config, &input, &mut schedule, config.solve_seconds, |schedule, message| {
                id = Some(save_schedule(&config.db_path, &input, schedule, message, id)?);
                Ok(())
            });
            print_schedule(&input, &best);
            print_problems(&input, &best);
            Ok(())
        }

        Ok(Opts::Sat(config)) => {
            let input = load_input(&config.db_path, &[], config.show_faculty_preference_priorities)?;
            let mut id = None;
            let schedule = generate_schedule(&config, &input, |schedule, message| {
                if id.is_none() {
                    print!("\r\x1b[K");
                }
                id = Some(save_schedule(&config.db_path, &input, schedule, message, id)?);
                Ok(())
            })?;
            if config.schedule_output == ScheduleOutput::All {
                print_schedule(&input, &schedule);
            }
            if config.schedule_output != ScheduleOutput::None {
                print_problems(&input, &schedule);
            }
            Ok(())
        }

        Ok(Opts::Dfs(config)) => {
            let input = load_input(&config.db_path, &[], config.show_faculty_preference_priorities)?;
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
            let input = load_input(&config.db_path, &[], config.show_faculty_preference_priorities)?;
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
            let input = load_input(&config.db_path, &[], config.show_faculty_preference_priorities)?;
            dump_input(&[], &input);
            Ok(())
        }

        Ok(Opts::Tweak(config)) => {
            let input = load_input(&config.db_path, &[], config.show_faculty_preference_priorities)?;
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
                let section_idx = input
                    .sections
                    .iter()
                    .position(|s| s.name == tweak.section)
                    .ok_or_else(|| format!("Section '{}' not found", tweak.section))?;

                // Look up time slot by name
                let time_slot_idx = input
                    .time_slots
                    .iter()
                    .position(|ts| ts.name == tweak.time_slot)
                    .ok_or_else(|| format!("Time slot '{}' not found", tweak.time_slot))?;

                // Look up room by name, handle "-" as no room
                let room_idx = if tweak.room == "-" {
                    None
                } else {
                    Some(
                        input
                            .rooms
                            .iter()
                            .position(|r| r.name == tweak.room)
                            .ok_or_else(|| format!("Room '{}' not found", tweak.room))?,
                    )
                };

                parsed_tweaks.push((section_idx, time_slot_idx, room_idx));
            }

            // Validate tweaks
            for (section_idx, time_slot_idx, room_idx) in &parsed_tweaks {
                let section = &input.sections[*section_idx];

                // Check if time slot is valid for this section
                if !section.time_slots.iter().any(|ts| ts.time_slot == *time_slot_idx) {
                    return Err(format!(
                        "Time slot '{}' is not valid for section '{}'",
                        input.time_slots[*time_slot_idx].name, section.name
                    )
                    .into());
                }

                // Check if room assignment is valid
                match room_idx {
                    Some(room) => {
                        if !section.rooms.iter().any(|r| r.room == *room) {
                            return Err(format!(
                                "Room '{}' is not valid for section '{}'",
                                input.rooms[*room].name, section.name
                            )
                            .into());
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
            let tweak_descriptions: Vec<String> =
                config.tweaks.iter().map(|t| format!("{}→{},{}", t.section, t.room, t.time_slot)).collect();
            let comment = format!("tweaked: {}", tweak_descriptions.join("; "));

            // Save the new schedule
            save_schedule(&config.db_path, &input, &schedule, &comment, None)?;

            // Print the result
            print_schedule(&input, &schedule);
            print_problems(&input, &schedule);
            Ok(())
        }

        Err(msg) => {
            print_usage(args.get(1).cloned());
            Err(msg)
        }
    }
}

fn parse_args(args: &[String]) -> Result<Opts> {
    let mut parser = CliParser::from_args(args)?;

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
            parser.boolean("", "--show-faculty-preference-priorities", &mut opts.show_faculty_preference_priorities)?;
            parser.leftover()?;
            Ok(Opts::Gen(opts))
        }

        "sat" => {
            let mut opts = SatOpts::default();
            parser.string("-d", "--db-path", &mut opts.db_path)?;
            parser.minimum_hallpasses("", "--min-hallpass", &mut opts.minimum_hallpasses)?;
            parser.value("--search", &mut opts.search)?;
            parser.boolean("", "--sat-timings", &mut opts.sat_timings)?;
            parser.value("--kissat", &mut opts.kissat)?;
            parser.value("--totalizer-order", &mut opts.totalizer_order)?;
            parser.value("--room-conflict-encoding", &mut opts.room_conflict_encoding)?;
            parser.value("--shuffle-seed", &mut opts.shuffle_seed)?;
            parser.value("--schedule-output", &mut opts.schedule_output)?;
            if let Some((_, value)) = parser.pair("", "--stop-after-priority") {
                let priority: u8 = value.parse().map_err(|_| "Expected a priority level from 0 to 49")?;
                if priority > MAX_PRIORITY {
                    return Err("Expected a priority level from 0 to 49".into());
                }
                opts.stop_after_priority = Some(priority);
            }
            parser.boolean("", "--show-faculty-preference-priorities", &mut opts.show_faculty_preference_priorities)?;
            parser.leftover()?;
            Ok(Opts::Sat(opts))
        }

        "dfs" => {
            let mut opts = DfsOpts::default();
            parser.string("-d", "--db-path", &mut opts.db_path)?;
            parser.int64("-i", "--id", &mut opts.starting_id)?;
            parser.uint("-p", "--dfs-depth", &mut opts.dfs_depth)?;
            parser.boolean("-r", "--repeat", &mut opts.repeat)?;
            parser.boolean("", "--show-faculty-preference-priorities", &mut opts.show_faculty_preference_priorities)?;
            parser.leftover()?;
            Ok(Opts::Dfs(opts))
        }

        "print" => {
            let mut opts = PrintOpts::default();
            parser.string("-d", "--db-path", &mut opts.db_path)?;
            parser.int64("-i", "--id", &mut opts.starting_id)?;
            parser.boolean("", "--show-faculty-preference-priorities", &mut opts.show_faculty_preference_priorities)?;
            parser.leftover()?;
            Ok(Opts::Print(opts))
        }

        "dump" => {
            let mut opts = DumpOpts::default();
            parser.string("-d", "--db-path", &mut opts.db_path)?;
            parser.boolean("", "--show-faculty-preference-priorities", &mut opts.show_faculty_preference_priorities)?;
            parser.leftover()?;
            Ok(Opts::Dump(opts))
        }

        "tweak" => {
            let mut opts = TweakOpts::default();
            parser.string("-d", "--db-path", &mut opts.db_path)?;
            parser.int64("-i", "--id", &mut opts.starting_id)?;
            parser.boolean("", "--show-faculty-preference-priorities", &mut opts.show_faculty_preference_priorities)?;
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

pub struct PrintOpts {
    pub db_path: String,
    pub starting_id: i64,
    pub show_faculty_preference_priorities: bool,
}

impl Default for PrintOpts {
    fn default() -> Self {
        Self { db_path: DEFAULT_DB_PATH.to_string(), starting_id: 0, show_faculty_preference_priorities: false }
    }
}

pub struct DumpOpts {
    pub db_path: String,
    pub show_faculty_preference_priorities: bool,
}

impl Default for DumpOpts {
    fn default() -> Self {
        Self { db_path: DEFAULT_DB_PATH.to_string(), show_faculty_preference_priorities: false }
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
    pub show_faculty_preference_priorities: bool,
}

pub struct DfsOpts {
    pub db_path: String,
    pub starting_id: i64,
    pub dfs_depth: usize,
    pub repeat: bool,
    pub show_faculty_preference_priorities: bool,
}

impl Default for TweakOpts {
    fn default() -> Self {
        Self {
            db_path: DEFAULT_DB_PATH.to_string(),
            starting_id: 0,
            tweaks: Vec::new(),
            show_faculty_preference_priorities: false,
        }
    }
}

impl Default for DfsOpts {
    fn default() -> Self {
        Self {
            db_path: DEFAULT_DB_PATH.to_string(),
            starting_id: 0,
            dfs_depth: 4,
            repeat: true,
            show_faculty_preference_priorities: false,
        }
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
            print_preference_balance_usage();
        }

        Some("sat") => {
            let default = SatOpts::default();
            eprintln!("Usage: marmot sat [options]");
            eprintln!();
            eprintln!("Options:");
            eprintln!("  -d, --db-path <path>           Database path (default: {})", default.db_path);
            eprintln!("      --search <linear|binary>  Hallpass search strategy (default: binary)");
            eprintln!("      --stop-after-priority <0..49>  Stop after optimizing this priority");
            eprintln!("      --schedule-output <all|problems|none>  Console schedule output (default: problems)");
            eprintln!("      --sat-timings              Print one timing row per SAT probe (default: false)");
            eprintln!("      --kissat <default|plain|race>  Kissat strategy (default: default)");
            eprintln!(
                "      --totalizer-order <default|shuffle|cluster>  Hallpass leaf order (default: {})",
                default.totalizer_order
            );
            eprintln!("      --room-conflict-encoding <proxy|direct>  Room overlap encoding (default: proxy)");
            eprintln!("      --shuffle-seed <u64>       Shuffle seed (default: 0)");
            eprintln!("      --min-hallpass <level:count,...>  First hallpass count to try at each priority level");
            print_preference_balance_usage();
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
            print_preference_balance_usage();
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
            print_preference_balance_usage();
        }

        Some("dump") => {
            let default = DumpOpts::default();
            eprintln!("Usage: marmot dump [options]");
            eprintln!();
            eprintln!("Options:");
            eprintln!("  -d, --db-path <path>           Database path (default: {})", default.db_path);
            print_preference_balance_usage();
        }

        Some("tweak") => {
            let default = TweakOpts::default();
            eprintln!("Usage: marmot tweak [options]");
            eprintln!();
            eprintln!("Options:");
            eprintln!("  -d, --db-path <path>           Database path (default: {})", default.db_path);
            eprintln!("  -i, --id <int>                 ID of schedule to start from (0 to use best in DB)");
            eprintln!("  -t, --tweak <section,room,time> Move a section to specified room and time (repeatable)");
            print_preference_balance_usage();
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

fn print_preference_balance_usage() {
    eprintln!(
        "      --show-faculty-preference-priorities <bool>  Show entropy redistribution details (default: false)"
    );
}

struct CliParser {
    command: String,
    pairs: HashMap<String, String>,
}

impl CliParser {
    fn from_args(args: &[String]) -> Result<Self> {
        if args.len() < 2 {
            return Err("Error: no command specified".into());
        }
        let command = &args[1];
        // SAT timing output accepts a bare flag or an explicit boolean.
        let mut pairs = HashMap::new();
        let mut options = args[2..].iter().peekable();
        while let Some(key) = options.next() {
            let bare_switch =
                command == "sat" && key == "--sat-timings" && options.peek().is_none_or(|value| value.starts_with('-'));
            let value = if bare_switch {
                "true".to_string()
            } else {
                options
                    .next()
                    .ok_or_else(|| format!("Error: 'marmot {command}' option {key} requires a value"))?
                    .clone()
            };
            pairs.insert(key.clone(), value);
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

    // Typed options report their accepted values at the command boundary.
    fn value<T>(&mut self, long: &str, target: &mut T) -> Result<()>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        if let Some((key, value)) = self.pair("", long) {
            *target = value.parse().map_err(|error| format!("Error parsing option {key}: {error}"))?;
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

    fn minimum_hallpasses(&mut self, short: &str, long: &str, target: &mut MinimumHallpasses) -> Result<()> {
        if let Some((key, val)) = self.pair(short, long) {
            *target =
                MinimumHallpasses::parse(&val).map_err(|message| format!("Error parsing option {key}: {message}"))?;
        }

        Ok(())
    }

    fn tweak_specs(&mut self, short: &str, long: &str, tweaks: &mut Vec<TweakSpec>) -> Result<bool> {
        if let Some((key, val)) = self.pair(short, long) {
            let parts: Vec<&str> = val.split(',').collect();
            if parts.len() != 3 {
                return Err(format!(
                    "Error parsing option {}: expected 3 comma-separated values (section,room,time), got {}",
                    key,
                    parts.len()
                )
                .into());
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

#[cfg(test)]
mod tests {
    use super::MinimumHallpasses;
    use crate::options::KissatMode;
    use crate::sat_prep::TotalizerOrder;

    #[test]
    fn sat_kissat_defaults_to_default_and_accepts_all_modes() {
        for (options, expected) in [
            (&[][..], KissatMode::Default),
            (&["--kissat", "default"][..], KissatMode::Default),
            (&["--kissat", "plain"][..], KissatMode::Plain),
            (&["--kissat", "race"][..], KissatMode::Race),
        ] {
            let args: Vec<_> = ["marmot", "sat"].into_iter().chain(options.iter().copied()).map(String::from).collect();
            let super::Opts::Sat(opts) = super::parse_args(&args).unwrap() else {
                panic!("expected SAT options");
            };
            assert_eq!(opts.kissat, expected);
        }
        let args = ["marmot", "sat", "--kissat", "fast"].map(String::from);
        assert!(super::parse_args(&args).is_err());
    }

    #[test]
    fn sat_totalizer_order_defaults_to_cluster_and_accepts_creation_order() {
        for (options, expected) in
            [(&[][..], TotalizerOrder::Cluster), (&["--totalizer-order", "default"][..], TotalizerOrder::Default)]
        {
            let args: Vec<_> = ["marmot", "sat"].into_iter().chain(options.iter().copied()).map(String::from).collect();
            let super::Opts::Sat(opts) = super::parse_args(&args).unwrap() else {
                panic!("expected SAT options");
            };
            assert_eq!(opts.totalizer_order, expected);
        }
        assert!("current".parse::<TotalizerOrder>().is_err());
    }

    #[test]
    fn parses_sat_experiment_switches_and_rejects_unknown_modes() {
        let args = [
            "marmot",
            "sat",
            "--sat-timings",
            "--totalizer-order",
            "shuffle",
            "--shuffle-seed",
            "42",
            "--schedule-output",
            "none",
        ]
        .map(String::from);
        let mut parser = super::CliParser::from_args(&args).unwrap();
        let mut opts = super::SatOpts::default();
        parser.boolean("", "--sat-timings", &mut opts.sat_timings).unwrap();
        parser.value("--totalizer-order", &mut opts.totalizer_order).unwrap();
        parser.value("--shuffle-seed", &mut opts.shuffle_seed).unwrap();
        parser.value("--schedule-output", &mut opts.schedule_output).unwrap();
        parser.leftover().unwrap();
        assert!(opts.sat_timings);
        assert_eq!(opts.totalizer_order, TotalizerOrder::Shuffle);
        assert_eq!(opts.shuffle_seed, 42);
        assert_eq!(opts.schedule_output, super::ScheduleOutput::None);
        assert!("shuffled".parse::<TotalizerOrder>().is_err());
        assert!("silent".parse::<super::ScheduleOutput>().is_err());
    }

    #[test]
    fn parses_search_strategies() {
        assert_eq!(super::SatOpts::default().search, super::Search::Binary);
        for (value, expected) in [("linear", super::Search::Linear), ("binary", super::Search::Binary)] {
            let args = ["marmot", "sat", "--search", value].map(String::from);
            let mut parser = super::CliParser::from_args(&args).unwrap();
            let mut search = super::Search::Binary;
            parser.value("--search", &mut search).unwrap();
            parser.leftover().unwrap();
            assert_eq!(search, expected);
        }
        let args = ["marmot", "sat", "--search", "hybrid"].map(String::from);
        let mut parser = super::CliParser::from_args(&args).unwrap();
        assert!(parser.value("--search", &mut super::Search::Binary).is_err());
        let args = ["marmot", "sat", "--search"].map(String::from);
        assert!(super::CliParser::from_args(&args).is_err());
    }

    #[test]
    fn parses_minimum_hallpasses_for_distinct_priorities() {
        let minimums = MinimumHallpasses::parse("7:1,8:2").unwrap();

        assert_eq!(minimums.at_priority(6), 0);
        assert_eq!(minimums.at_priority(7), 1);
        assert_eq!(minimums.at_priority(8), 2);
    }

    #[test]
    fn rejects_duplicate_minimum_hallpass_priorities() {
        let error = MinimumHallpasses::parse("7:1,8:2,7:3").unwrap_err();

        assert_eq!(error, "priority 7 is specified more than once");
    }

    #[test]
    fn rejects_minimum_hallpasses_for_hard_constraints() {
        let error = MinimumHallpasses::parse("0:1").unwrap_err();

        assert_eq!(error, "priority 0 contains hard constraints and cannot have a minimum hallpass count");
    }

    #[test]
    fn rejects_invalid_minimum_hallpass_values() {
        assert!(MinimumHallpasses::parse("").is_err());
        assert!(MinimumHallpasses::parse("7").is_err());
        assert!(MinimumHallpasses::parse("7:-1").is_err());
        assert!(MinimumHallpasses::parse("50:1").is_err());
    }
}
