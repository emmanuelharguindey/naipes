//! naipes — Spanish card games on the command line.
//!
//! v0.1.0 ships brisca. Future versions will add tute, mus, chinchón
//! under the same umbrella.
//!
//! Public programmatic API. For CLI usage, run `naipes` after `cargo install`.

pub mod cli;
pub mod core;
pub mod games;

pub use cli::{replay, Outcome, ReplayResult};
pub use core::{Card, Rng, Suit};
pub use games::brisca;
