//! Primitives shared across all card games under the naipes umbrella.

pub mod cards;
pub mod rng;

pub use cards::{build_ordered_deck, Card, Suit, RANKS, SUITS};
pub use rng::Rng;
