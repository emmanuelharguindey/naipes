use naipes::brisca::ai::{choose_easy, AiLevel};
use naipes::brisca::rules::card_strength;
use naipes::brisca::state::GameState;
use naipes::cli::replay;

#[test]
fn full_game_total_points_is_120() {
    let r = replay(1, AiLevel::Normal, &[1u8; 20]);
    assert_eq!(r.human_score + r.ai_score, 120);
}

#[test]
fn full_game_has_20_tricks() {
    let r = replay(1, AiLevel::Normal, &[1u8; 20]);
    assert_eq!(r.trick_log.len(), 20);
}

#[test]
fn outcome_consistent_with_score() {
    let r = replay(1, AiLevel::Normal, &[1u8; 20]);
    if r.human_score > 60 {
        assert_eq!(r.outcome.as_str(), "win");
    } else if r.human_score < 60 {
        assert_eq!(r.outcome.as_str(), "loss");
    } else {
        assert_eq!(r.outcome.as_str(), "draw");
    }
}

#[test]
fn same_seed_same_plays_same_result() {
    let a = replay(42, AiLevel::Normal, &[1u8; 20]);
    let b = replay(42, AiLevel::Normal, &[1u8; 20]);
    assert_eq!(a.human_score, b.human_score);
    assert_eq!(a.ai_score, b.ai_score);
    let a_log: Vec<_> = a
        .trick_log
        .iter()
        .map(|r| (r.human_card.label(), r.ai_card.label()))
        .collect();
    let b_log: Vec<_> = b
        .trick_log
        .iter()
        .map(|r| (r.human_card.label(), r.ai_card.label()))
        .collect();
    assert_eq!(a_log, b_log);
}

#[test]
fn initial_state_invariants() {
    let s = GameState::new(1);
    assert_eq!(s.human_hand.len(), 3);
    assert_eq!(s.ai_hand.len(), 3);
    assert_eq!(s.deck.len(), 34);
    assert_eq!(s.deck.last().unwrap(), &s.trump_card);
}

#[test]
fn easy_ai_plays_lowest_strength() {
    let s = GameState::new(1);
    let idx = choose_easy(&s);
    let chosen = s.ai_hand[idx];
    for other in &s.ai_hand {
        assert!(card_strength(chosen) <= card_strength(*other));
    }
}
