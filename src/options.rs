use crate::sat_prep::TotalizerOrder;
use crate::score::{MAX_PRIORITY, ScoreLevel};
use std::collections::HashMap;
const DEFAULT_DB_PATH: &str = "../data/timetable.db";
pub struct SatOpts {
    pub stop_after_priority: Option<u8>,
    pub schedule_output: ScheduleOutput,
    pub sat_timings: bool,
    pub kissat: KissatMode,
    pub totalizer_order: TotalizerOrder,
    pub room_conflict_encoding: RoomConflictEncoding,
    pub shuffle_seed: u64,
    pub search: Search,
    pub db_path: String,
    pub minimum_hallpasses: MinimumHallpasses,
    pub balance_faculty_preferences: bool,
    pub show_faculty_preference_priorities: bool,
}

impl Default for SatOpts {
    fn default() -> Self {
        Self {
            stop_after_priority: None,
            schedule_output: ScheduleOutput::Problems,
            sat_timings: false,
            kissat: KissatMode::Default,
            totalizer_order: TotalizerOrder::Cluster,
            room_conflict_encoding: RoomConflictEncoding::Proxy,
            shuffle_seed: 0,
            db_path: DEFAULT_DB_PATH.to_string(),
            minimum_hallpasses: MinimumHallpasses::default(),
            search: Search::Binary,
            balance_faculty_preferences: true,
            show_faculty_preference_priorities: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KissatMode {
    Default,
    Plain,
    Race,
}

impl std::str::FromStr for KissatMode {
    type Err = &'static str;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value {
            "default" => Ok(Self::Default),
            "plain" => Ok(Self::Plain),
            "race" => Ok(Self::Race),
            _ => Err("expected default, plain, or race"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoomConflictEncoding {
    Proxy,
    Direct,
}

impl std::str::FromStr for RoomConflictEncoding {
    type Err = &'static str;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value {
            "proxy" => Ok(Self::Proxy),
            "direct" => Ok(Self::Direct),
            _ => Err("expected proxy or direct"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Search {
    Linear,
    Binary,
}

impl std::str::FromStr for Search {
    type Err = &'static str;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value {
            "linear" => Ok(Self::Linear),
            "binary" => Ok(Self::Binary),
            _ => Err("expected linear or binary"),
        }
    }
}

// Console output does not affect saved schedules or solver progress reporting.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduleOutput {
    All,
    Problems,
    None,
}

impl std::str::FromStr for ScheduleOutput {
    type Err = &'static str;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value {
            "all" => Ok(Self::All),
            "problems" => Ok(Self::Problems),
            "none" => Ok(Self::None),
            _ => Err("expected all, problems, or none"),
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MinimumHallpasses {
    by_priority: HashMap<u8, ScoreLevel>,
}

impl MinimumHallpasses {
    pub fn parse(value: &str) -> std::result::Result<Self, String> {
        let mut by_priority = HashMap::new();

        if value.is_empty() {
            return Err("expected one or more LEVEL:COUNT pairs".to_string());
        }

        for pair in value.split(',') {
            let mut fields = pair.split(':');
            let (Some(priority), Some(count), None) = (fields.next(), fields.next(), fields.next()) else {
                return Err(format!("expected LEVEL:COUNT, got '{pair}'"));
            };
            let priority = priority
                .parse::<u8>()
                .map_err(|_| format!("priority '{priority}' must be an integer from 1 to {MAX_PRIORITY}"))?;
            if priority == 0 {
                return Err("priority 0 contains hard constraints and cannot have a minimum hallpass count".to_string());
            }
            if priority > MAX_PRIORITY {
                return Err(format!("priority {priority} exceeds the maximum priority {MAX_PRIORITY}"));
            }
            let count = count
                .parse::<ScoreLevel>()
                .map_err(|_| format!("hallpass count '{count}' must be an integer from 0 to {}", ScoreLevel::MAX))?;
            if count < 0 {
                return Err(format!("hallpass count {count} cannot be negative"));
            }
            if by_priority.insert(priority, count).is_some() {
                return Err(format!("priority {priority} is specified more than once"));
            }
        }

        Ok(Self { by_priority })
    }

    pub fn at_priority(&self, priority: u8) -> ScoreLevel {
        self.by_priority.get(&priority).copied().unwrap_or(0)
    }
}

#[cfg(feature = "search")]
pub use crate::search_options::GenOpts;
