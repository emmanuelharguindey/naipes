"""Integration test: full game with conservation invariants."""

from naipes.cli.loop import replay
from naipes.games.brisca.state import GameState, Player


def test_full_game_total_points_is_120():
    """Sum of human + ai scores must equal 120 (total points in deck)."""
    result = replay(seed=1, ai_level="normal", human_plays=[1] * 20)
    assert result.human_score + result.ai_score == 120


def test_full_game_has_20_tricks():
    result = replay(seed=1, ai_level="normal", human_plays=[1] * 20)
    assert len(result.trick_log) == 20


def test_outcome_consistent_with_score():
    result = replay(seed=1, ai_level="normal", human_plays=[1] * 20)
    if result.human_score > 60:
        assert result.outcome == "win"
    elif result.human_score < 60:
        assert result.outcome == "loss"
    else:
        assert result.outcome == "draw"


def test_same_seed_same_plays_same_result():
    a = replay(seed=42, ai_level="normal", human_plays=[1] * 20)
    b = replay(seed=42, ai_level="normal", human_plays=[1] * 20)
    assert a.human_score == b.human_score
    assert a.ai_score == b.ai_score
    assert [(r.human_card, r.ai_card) for r in a.trick_log] == [
        (r.human_card, r.ai_card) for r in b.trick_log
    ]


def test_different_seed_likely_different_game():
    a = replay(seed=1, ai_level="normal", human_plays=[1] * 20)
    b = replay(seed=2, ai_level="normal", human_plays=[1] * 20)
    # Trick logs should differ (extremely unlikely they collide).
    assert a.trick_log != b.trick_log


def test_initial_state_invariants():
    state = GameState.new(seed=1)
    assert len(state.human_hand) == 3
    assert len(state.ai_hand) == 3
    # 40 - 3 - 3 = 34. Trump is at the bottom of the deck.
    assert len(state.deck) == 34
    assert state.deck[-1] == state.trump_card
    assert state.trump_suit == state.trump_card.suit


def test_ai_easy_always_plays_lowest_strength():
    """Easy AI never plays a high card when a low one is available."""
    from naipes.games.brisca.ai import choose_easy
    from naipes.games.brisca.rules import card_strength

    state = GameState.new(seed=1)
    idx = choose_easy(state)
    chosen = state.ai_hand[idx]
    for other in state.ai_hand:
        assert card_strength(chosen) <= card_strength(other)
