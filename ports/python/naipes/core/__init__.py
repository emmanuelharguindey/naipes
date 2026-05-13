"""naipes.core — primitives shared across all card games."""

from naipes.core.cards import (
    Card,
    SUITS,
    SUIT_NAMES,
    RANKS,
    RANK_LABELS,
    RANK_NAMES,
    build_ordered_deck,
    iter_deck,
)
from naipes.core.rng import Rng

__all__ = [
    "Card",
    "SUITS",
    "SUIT_NAMES",
    "RANKS",
    "RANK_LABELS",
    "RANK_NAMES",
    "Rng",
    "build_ordered_deck",
    "iter_deck",
]
