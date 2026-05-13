use naipes::brisca::rules::{card_points, card_strength, trick_winner_is_follower};
use naipes::core::{Card, Suit};

const TRUMP: Suit = Suit::Oros;

#[test]
fn both_trump_higher_wins() {
    assert!(trick_winner_is_follower(
        Card::new(2, TRUMP),
        Card::new(1, TRUMP),
        TRUMP
    ));
}

#[test]
fn both_trump_lower_loses() {
    assert!(!trick_winner_is_follower(
        Card::new(1, TRUMP),
        Card::new(2, TRUMP),
        TRUMP
    ));
}

#[test]
fn follower_trumps_non_trump_lead() {
    assert!(trick_winner_is_follower(
        Card::new(1, Suit::Copas),
        Card::new(2, TRUMP),
        TRUMP
    ));
}

#[test]
fn leader_trump_beats_non_trump_follower() {
    assert!(!trick_winner_is_follower(
        Card::new(2, TRUMP),
        Card::new(1, Suit::Copas),
        TRUMP
    ));
}

#[test]
fn same_suit_higher_follower_wins() {
    assert!(trick_winner_is_follower(
        Card::new(7, Suit::Copas),
        Card::new(1, Suit::Copas),
        TRUMP
    ));
}

#[test]
fn same_suit_lower_follower_loses() {
    assert!(!trick_winner_is_follower(
        Card::new(1, Suit::Copas),
        Card::new(7, Suit::Copas),
        TRUMP
    ));
}

#[test]
fn off_suit_non_trump_follower_loses() {
    assert!(!trick_winner_is_follower(
        Card::new(2, Suit::Copas),
        Card::new(1, Suit::Espadas),
        TRUMP
    ));
}

#[test]
fn canonical_points() {
    assert_eq!(card_points(Card::new(1, Suit::Oros)), 11);
    assert_eq!(card_points(Card::new(3, Suit::Oros)), 10);
    assert_eq!(card_points(Card::new(12, Suit::Oros)), 4);
    assert_eq!(card_points(Card::new(11, Suit::Oros)), 3);
    assert_eq!(card_points(Card::new(10, Suit::Oros)), 2);
    assert_eq!(card_points(Card::new(7, Suit::Oros)), 0);
    assert_eq!(card_points(Card::new(2, Suit::Oros)), 0);
}

#[test]
fn strength_ordering() {
    assert!(card_strength(Card::new(1, Suit::Oros)) > card_strength(Card::new(3, Suit::Oros)));
    assert!(card_strength(Card::new(3, Suit::Oros)) > card_strength(Card::new(12, Suit::Oros)));
    assert!(card_strength(Card::new(12, Suit::Oros)) > card_strength(Card::new(11, Suit::Oros)));
    assert_eq!(card_strength(Card::new(2, Suit::Oros)), 1);
}
