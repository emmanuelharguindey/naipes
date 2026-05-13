"""Tests for trick resolution. Covers all four cases of SPEC.md §5.3."""

from naipes.core.cards import Card
from naipes.games.brisca.rules import (
    card_points,
    card_strength,
    trick_winner_is_follower,
)


TRUMP = "O"


def test_both_trump_higher_wins():
    leader = Card(2, TRUMP)
    follower = Card(1, TRUMP)  # As de Oros, strength 10
    assert trick_winner_is_follower(leader, follower, TRUMP) is True


def test_both_trump_lower_loses():
    leader = Card(1, TRUMP)  # As de Oros wins
    follower = Card(2, TRUMP)
    assert trick_winner_is_follower(leader, follower, TRUMP) is False


def test_follower_trumps_non_trump_lead():
    leader = Card(1, "C")  # As de Copas
    follower = Card(2, TRUMP)  # 2 de Oros — lowest trump beats any non-trump
    assert trick_winner_is_follower(leader, follower, TRUMP) is True


def test_leader_trump_beats_non_trump_follower():
    leader = Card(2, TRUMP)
    follower = Card(1, "C")
    assert trick_winner_is_follower(leader, follower, TRUMP) is False


def test_same_suit_higher_follower_wins():
    leader = Card(7, "C")
    follower = Card(1, "C")  # As de Copas wins
    assert trick_winner_is_follower(leader, follower, TRUMP) is True


def test_same_suit_lower_follower_loses():
    leader = Card(1, "C")
    follower = Card(7, "C")
    assert trick_winner_is_follower(leader, follower, TRUMP) is False


def test_off_suit_non_trump_follower_loses_always():
    """If follower doesn't match suit and doesn't trump, leader wins."""
    leader = Card(2, "C")  # weakest copas
    follower = Card(1, "E")  # As de Espadas — but wrong suit, no trump
    assert trick_winner_is_follower(leader, follower, TRUMP) is False


def test_points_are_canonical():
    """SPEC.md §3.2 point values."""
    assert card_points(Card(1, "O")) == 11
    assert card_points(Card(3, "O")) == 10
    assert card_points(Card(12, "O")) == 4
    assert card_points(Card(11, "O")) == 3
    assert card_points(Card(10, "O")) == 2
    assert card_points(Card(7, "O")) == 0
    assert card_points(Card(2, "O")) == 0


def test_strength_ordering():
    """Strength must rank As highest and 2 lowest."""
    assert card_strength(Card(1, "O")) > card_strength(Card(3, "O"))
    assert card_strength(Card(3, "O")) > card_strength(Card(12, "O"))
    assert card_strength(Card(12, "O")) > card_strength(Card(11, "O"))
    assert card_strength(Card(11, "O")) > card_strength(Card(10, "O"))
    assert card_strength(Card(10, "O")) > card_strength(Card(7, "O"))
    assert card_strength(Card(2, "O")) == 1  # lowest
