//! Timetabling data, preparation, placement editing, and optional native services.
#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "sat")]
pub mod cnf;
pub mod error;
pub mod faculty_preferences;
pub mod input;
#[cfg(feature = "sat")]
pub mod options;
pub mod preparation;
pub mod print;
#[cfg(feature = "sat")]
pub mod sat_criteria;
#[cfg(feature = "sat")]
pub mod sat_encoders;
#[cfg(feature = "sat")]
pub mod sat_prep;
#[cfg(feature = "sat")]
pub mod sat_solver;
pub mod score;
#[cfg(feature = "search")]
pub mod search;
#[cfg(test)]
mod shared_day_off_tests;
pub mod solver;
#[cfg(feature = "sqlite")]
pub mod sqlite;
#[cfg(feature = "sat")]
pub use options::{MinimumHallpasses, Search};
#[cfg(feature = "search")]
pub mod search_options;
