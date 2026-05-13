//! Pure brisca rules. No state, no side effects. See SPEC.md §3.2 and §5.

use crate::core::{Card, Suit};

/// Points captured for a card of this rank. SPEC.md §3.2.
pub fn card_points(card: Card) -> u8 {
    match card.rank {
        1 => 11,   // As
        3 => 10,   // Tres
        12 => 4,   // Rey
        11 => 3,   // Caballo
        10 => 2,   // Sota
        _ => 0,
    }
}

/// Strength for trick resolution (higher beats lower within same suit). SPEC.md §3.2.
pub fn card_strength(card: Card) -> u8 {
    match card.rank {
        1 => 10,
        3 => 9,
        12 => 8,
        11 => 7,
        10 => 6,
        7 => 5,
        6 => 4,
        5 => 3,
        4 => 2,
        2 => 1,
        _ => unreachable!("invalid rank reached card_strength()"),
    }
}

/// Returns true iff the follower wins the trick. SPEC.md §5.3.
pub fn trick_winner_is_follower(leader: Card, follower: Card, trump_suit: Suit) -> bool {
    let leader_trump = leader.suit == trump_suit;
    let follower_trump = follower.suit == trump_suit;

    // Both trump: higher strength wins.
    if leader_trump && follower_trump {
        return card_strength(follower) > card_strength(leader);
    }
    // Exactly one trump: that one wins.
    if follower_trump {
        return true;
    }
    if leader_trump {
        return false;
    }
    // Neither trump. Follower must match lead suit to beat the leader.
    if follower.suit != leader.suit {
        return false;
    }
    card_strength(follower) > card_strength(leader)
}

pub fn trick_points(leader: Card, follower: Card) -> u8 {
    card_points(leader) + card_points(follower)
}
