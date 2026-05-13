//! Brisca AI strategies. Pure functions over GameState. See SPEC.md §6.

use crate::core::Card;
use crate::games::brisca::rules::{card_points, card_strength, trick_winner_is_follower};
use crate::games::brisca::state::GameState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiLevel {
    Easy,
    Normal,
    Hard,
}

impl AiLevel {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "easy" => Some(AiLevel::Easy),
            "normal" => Some(AiLevel::Normal),
            "hard" => Some(AiLevel::Hard),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AiLevel::Easy => "easy",
            AiLevel::Normal => "normal",
            AiLevel::Hard => "hard",
        }
    }
}

/// Index of the lowest-strength card in hand matching `predicate`.
/// Ties broken by hand position. SPEC.md §6.4.
fn lowest_strength_index<F>(hand: &[Card], predicate: F) -> Option<usize>
where
    F: Fn(Card) -> bool,
{
    let mut best_idx: Option<usize> = None;
    let mut best_strength: u8 = u8::MAX;
    for (i, card) in hand.iter().enumerate() {
        if !predicate(*card) {
            continue;
        }
        let s = card_strength(*card);
        if s < best_strength {
            best_strength = s;
            best_idx = Some(i);
        }
    }
    best_idx
}

/// Always lowest-strength card in hand. SPEC.md §6.1.
pub fn choose_easy(state: &GameState) -> usize {
    lowest_strength_index(&state.ai_hand, |_| true)
        .expect("AI asked to play with empty hand")
}

/// SPEC.md §6.2. Different logic for leading vs following.
pub fn choose_normal(state: &GameState) -> usize {
    let hand = &state.ai_hand;
    let trump = state.trump_suit;

    match state.pending_leader_card {
        None => {
            // Leading.
            // 1. Lowest-strength non-trump rank-0-points card.
            if let Some(idx) =
                lowest_strength_index(hand, |c| c.suit != trump && card_points(c) == 0)
            {
                return idx;
            }
            // 2. Lowest-strength non-trump card.
            if let Some(idx) = lowest_strength_index(hand, |c| c.suit != trump) {
                return idx;
            }
            // 3. Lowest-strength trump card.
            lowest_strength_index(hand, |_| true).expect("AI asked to play with empty hand")
        }
        Some(leader_card) => {
            let would_win = |c: Card| trick_winner_is_follower(leader_card, c, trump);

            // 1. Lowest non-trump that wins.
            if let Some(idx) = lowest_strength_index(hand, |c| c.suit != trump && would_win(c)) {
                return idx;
            }
            // 2. Leader's card carries >= 10 points: trump to win if possible.
            if card_points(leader_card) >= 10 {
                if let Some(idx) =
                    lowest_strength_index(hand, |c| c.suit == trump && would_win(c))
                {
                    return idx;
                }
            }
            // 3. Dump lowest non-trump, else lowest trump.
            if let Some(idx) = lowest_strength_index(hand, |c| c.suit != trump) {
                return idx;
            }
            lowest_strength_index(hand, |_| true).expect("AI asked to play with empty hand")
        }
    }
}

/// Hard deferred to v0.2.0 per SPEC.md §6.3; falls back to Normal.
pub fn choose_hard(state: &GameState) -> usize {
    choose_normal(state)
}

pub fn choose(state: &GameState, level: AiLevel) -> usize {
    match level {
        AiLevel::Easy => choose_easy(state),
        AiLevel::Normal => choose_normal(state),
        AiLevel::Hard => choose_hard(state),
    }
}
