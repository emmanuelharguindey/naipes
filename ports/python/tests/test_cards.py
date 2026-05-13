"""Tests for card primitives and deck ordering."""

from naipes.core.cards import Card, build_ordered_deck, RANKS, SUITS


def test_deck_has_40_cards():
    deck = build_ordered_deck()
    assert len(deck) == 40


def test_deck_has_no_duplicates():
    deck = build_ordered_deck()
    assert len(set(deck)) == 40


def test_deck_first_card_is_as_de_oros():
    deck = build_ordered_deck()
    assert deck[0] == Card(rank=1, suit="O")
    assert deck[0].label == "AsO"


def test_deck_last_card_is_rey_de_bastos():
    deck = build_ordered_deck()
    assert deck[-1] == Card(rank=12, suit="B")
    assert deck[-1].label == "RB"


def test_no_8_or_9_in_ranks():
    assert 8 not in RANKS
    assert 9 not in RANKS


def test_card_labels():
    assert Card(1, "O").label == "AsO"
    assert Card(3, "E").label == "3E"
    assert Card(12, "C").label == "RC"
    assert Card(10, "B").label == "SB"
    assert Card(11, "O").label == "CO"


def test_invalid_card_raises():
    import pytest

    with pytest.raises(ValueError):
        Card(rank=8, suit="O")
    with pytest.raises(ValueError):
        Card(rank=1, suit="X")
