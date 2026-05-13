//! Render brisca state to text. Pure: state in, string out.

use crate::games::brisca::state::{GameState, Player};

fn sep_double() -> String {
    "═".repeat(39)
}

fn sep_single() -> String {
    "─".repeat(39)
}

fn render_trump_line(state: &GameState) -> Option<String> {
    if !state.deck.is_empty() {
        if state.deck.len() == 1 && state.deck.last() == Some(&state.trump_card) {
            return Some(format!("Trump: {} (last)", state.trump_card));
        }
        return Some(format!("Trump: {}", state.trump_card));
    }
    Some(format!("Trump: {} (suit)", state.trump_card.suit))
}

pub fn render_board(state: &GameState) -> String {
    let ai_count = state.ai_hand.len();
    let deck_count = state.deck.len();
    let trump_line = render_trump_line(state);

    let table = match (state.pending_leader_card, state.pending_leader) {
        (Some(c), Some(Player::Ai)) => format!("AI played: {c}"),
        (Some(c), Some(Player::Human)) => format!("You played: {c} (waiting for AI)"),
        _ => "(empty)".to_string(),
    };

    let hand_line = if state.human_hand.is_empty() {
        "(no cards)".to_string()
    } else {
        state
            .human_hand
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{}:{}", i + 1, c))
            .collect::<Vec<_>>()
            .join("  ")
    };

    let mut parts: Vec<String> = vec![
        sep_double(),
        format!(" AI: {ai_count} cards              Deck: {deck_count}"),
    ];
    if let Some(t) = trump_line {
        parts.push(format!(" {t}"));
    }
    parts.extend([
        sep_single(),
        format!(" Table: {table}"),
        sep_single(),
        " Your hand:".to_string(),
        format!("   {hand_line}"),
        sep_double(),
    ]);
    parts.join("\n")
}

pub fn render_trick_result(state: &GameState) -> String {
    let record = state.trick_log.last().expect("trick log has at least one entry");
    let ai_played = if record.leader == Player::Human {
        format!(" AI plays: {}", record.ai_card)
    } else {
        format!(" AI led with: {}", record.ai_card)
    };
    let winner_label = if record.winner == Player::Human { "you" } else { "AI" };
    format!("{}\n Winner: {} (+{} points)", ai_played, winner_label, record.points)
}

pub fn render_final(state: &GameState) -> String {
    let human_score = state.score_of(Player::Human);
    let ai_score = state.score_of(Player::Ai);
    let (verdict, outcome) = if human_score > 60 {
        ("You won", "win")
    } else if human_score < 60 {
        ("You lost", "loss")
    } else {
        ("Draw", "draw")
    };
    [
        sep_double(),
        " Game over".to_string(),
        format!(" Your score: {human_score}"),
        format!(" AI score:   {ai_score}"),
        format!(" {verdict}"),
        sep_double(),
        format!("RESULT human={human_score} ai={ai_score} outcome={outcome}"),
    ]
    .join("\n")
}
