"""Pure brisca rules. No state, no side effects. See SPEC.md §3.2 and §5.

Separating rules from state lets us test trick resolution and scoring
in isolation, and lets the AI introspect outcomes without mutating play.
"""

from __future__ import annotations

from naipes.core.cards import Card

# rank -> points awarded for capturing this card. SPEC.md §3.2.
POINTS: dict[int, int] = {
    1: 11,   # As
    3: 10,   # Tres
    12: 4,   # Rey
    11: 3,   # Caballo
    10: 2,   # Sota
    7: 0,
    6: 0,
    5: 0,
    4: 0,
    2: 0,
}

# rank -> strength for trick resolution (higher beats lower within same suit).
# SPEC.md §3.2.
STRENGTH: dict[int, int] = {
    1: 10,
    3: 9,
    12: 8,
    11: 7,
    10: 6,
    7: 5,
    6: 4,
    5: 3,
    4: 2,
    2: 1,
}


def card_points(card: Card) -> int:
    return POINTS[card.rank]


def card_strength(card: Card) -> int:
    return STRENGTH[card.rank]


def trick_winner_is_follower(
    leader_card: Card,
    follower_card: Card,
    trump_suit: str,
) -> bool:
    """Return True iff the follower wins the trick. SPEC.md §5.3.

    The leader led, the follower responded. Determine who captures.
    """
    leader_is_trump = leader_card.suit == trump_suit
    follower_is_trump = follower_card.suit == trump_suit

    # Both trump: higher strength wins.
    if leader_is_trump and follower_is_trump:
        return card_strength(follower_card) > card_strength(leader_card)

    # Exactly one trump: that one wins.
    if follower_is_trump:
        return True
    if leader_is_trump:
        return False

    # Neither trump. Follower must match lead suit to beat the leader.
    if follower_card.suit != leader_card.suit:
        return False

    return card_strength(follower_card) > card_strength(leader_card)


def trick_points(leader_card: Card, follower_card: Card) -> int:
    """Sum of points captured in this trick (goes to winner)."""
    return card_points(leader_card) + card_points(follower_card)
