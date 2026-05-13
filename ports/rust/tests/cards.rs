use naipes::core::{build_ordered_deck, Card, Suit, RANKS};
use std::collections::HashSet;

#[test]
fn deck_has_40_cards() {
    assert_eq!(build_ordered_deck().len(), 40);
}

#[test]
fn deck_has_no_duplicates() {
    let deck = build_ordered_deck();
    let labels: HashSet<String> = deck.iter().map(|c| c.label()).collect();
    assert_eq!(labels.len(), 40);
}

#[test]
fn first_card_is_as_de_oros() {
    let deck = build_ordered_deck();
    assert_eq!(deck[0].label(), "AsO");
}

#[test]
fn last_card_is_rey_de_bastos() {
    let deck = build_ordered_deck();
    assert_eq!(deck[39].label(), "RB");
}

#[test]
fn no_8_or_9_in_ranks() {
    assert!(!RANKS.contains(&8));
    assert!(!RANKS.contains(&9));
}

#[test]
fn card_labels_are_correct() {
    assert_eq!(Card::new(1, Suit::Oros).label(), "AsO");
    assert_eq!(Card::new(3, Suit::Espadas).label(), "3E");
    assert_eq!(Card::new(12, Suit::Copas).label(), "RC");
    assert_eq!(Card::new(10, Suit::Bastos).label(), "SB");
    assert_eq!(Card::new(11, Suit::Oros).label(), "CO");
}
