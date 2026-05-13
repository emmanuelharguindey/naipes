"""Brisca AI strategies. Pure functions over GameState. See SPEC.md §6.

Each strategy returns a hand index for the AI to play. The state is not
mutated here; the caller applies the play.
"""

from __future__ import annotations

from typing import Callable

from naipes.core.cards import Card
from naipes.games.brisca.rules import (
    card_points,
    card_strength,
    trick_winner_is_follower,
)
from naipes.games.brisca.state import GameState, Player


def _lowest_strength_index(
    hand: list[Card],
    predicate: Callable[[Card], bool] | None = None,
) -> int | None:
    """Index of the lowest-strength card in hand matching predicate.

    Ties broken by hand position (earliest insertion = lowest index).
    SPEC.md §6.4. Returns None if no card matches.
    """
    best_idx: int | None = None
    best_strength = 999
    for i, card in enumerate(hand):
        if predicate is not None and not predicate(card):
            continue
        s = card_strength(card)
        if s < best_strength:
            best_strength = s
            best_idx = i
    return best_idx


def choose_easy(state: GameState) -> int:
    """Always lowest-strength card in hand. SPEC.md §6.1."""
    hand = state.ai_hand
    idx = _lowest_strength_index(hand)
    assert idx is not None, "AI asked to play with empty hand"
    return idx


def choose_normal(state: GameState) -> int:
    """SPEC.md §6.2. Different logic for leading vs following."""
    hand = state.ai_hand
    trump = state.trump_suit

    if state.pending_leader_card is None:
        # Leading.
        # 1. Lowest-strength non-trump rank-0-points card.
        idx = _lowest_strength_index(
            hand, lambda c: c.suit != trump and card_points(c) == 0
        )
        if idx is not None:
            return idx
        # 2. Lowest-strength non-trump card.
        idx = _lowest_strength_index(hand, lambda c: c.suit != trump)
        if idx is not None:
            return idx
        # 3. Lowest-strength trump card (everything left is trump).
        idx = _lowest_strength_index(hand)
        assert idx is not None
        return idx

    # Following.
    leader_card = state.pending_leader_card

    def would_win(card: Card) -> bool:
        # AI is the follower here, so check whether playing `card` makes
        # the follower (AI) win.
        return trick_winner_is_follower(leader_card, card, trump)

    # 1. Lowest non-trump that wins.
    idx = _lowest_strength_index(hand, lambda c: c.suit != trump and would_win(c))
    if idx is not None:
        return idx

    # 2. If the leader's card carries >= 10 points, use trump to win if possible.
    if card_points(leader_card) >= 10:
        idx = _lowest_strength_index(
            hand, lambda c: c.suit == trump and would_win(c)
        )
        if idx is not None:
            return idx

    # 3. Cannot or should not win: dump lowest non-trump, else lowest trump.
    idx = _lowest_strength_index(hand, lambda c: c.suit != trump)
    if idx is not None:
        return idx
    idx = _lowest_strength_index(hand)
    assert idx is not None
    return idx


# Hard is deferred to v0.2.0 per SPEC.md §6.3.
# For now, "hard" silently falls back to normal.
choose_hard = choose_normal


STRATEGIES = {
    "easy": choose_easy,
    "normal": choose_normal,
    "hard": choose_hard,
}


def choose(state: GameState, level: str) -> int:
    if level not in STRATEGIES:
        raise ValueError(f"unknown AI level {level!r}")
    return STRATEGIES[level](state)
