//! Command-line interface.

pub mod loop_play;
pub mod render;
pub mod root;

pub use loop_play::{play_interactive, replay, Outcome, PlayOptions, ReplayResult};
pub use root::main;
