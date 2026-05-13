"""Spanish 40-card deck primitives shared across naipes games.

See spec/SPEC.md §3.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Iterator

# Suits in canonical order. Order matters for deterministic deck building.
SUITS: tuple[str, ...] = ("O", "C", "E", "B")  # Oros, Copas, Espadas, Bastos

SUIT_NAMES: dict[str, str] = {
    "O": "Oros",
    "C": "Copas",
    "E": "Espadas",
    "B": "Bastos",
}

# Ranks in canonical deck order. Note: 8 and 9 do not exist in Spanish deck.
RANKS: tuple[int, ...] = (1, 2, 3, 4, 5, 6, 7, 10, 11, 12)

# Display labels per rank (single token, no whitespace).
RANK_LABELS: dict[int, str] = {
    1: "As",
    2: "2",
    3: "3",
    4: "4",
    5: "5",
    6: "6",
    7: "7",
    10: "S",   # Sota
    11: "C",   # Caballo
    12: "R",   # Rey
}

RANK_NAMES: dict[int, str] = {
    1: "As",
    2: "Dos",
    3: "Tres",
    4: "Cuatro",
    5: "Cinco",
    6: "Seis",
    7: "Siete",
    10: "Sota",
    11: "Caballo",
    12: "Rey",
}


@dataclass(frozen=True, slots=True)
class Card:
    """A single Spanish playing card. Immutable, hashable."""

    rank: int
    suit: str

    def __post_init__(self) -> None:
        if self.rank not in RANK_LABELS:
            raise ValueError(f"invalid rank {self.rank!r}")
        if self.suit not in SUIT_NAMES:
            raise ValueError(f"invalid suit {self.suit!r}")

    @property
    def label(self) -> str:
        """Canonical short form, e.g. 'AsO', '3E', 'RC'."""
        return f"{RANK_LABELS[self.rank]}{self.suit}"

    @property
    def long_name(self) -> str:
        """Human-readable Spanish name, e.g. 'As de Oros'."""
        return f"{RANK_NAMES[self.rank]} de {SUIT_NAMES[self.suit]}"

    def __str__(self) -> str:
        return self.label


def build_ordered_deck() -> list[Card]:
    """The canonical unshuffled 40-card deck. See SPEC.md §3.4."""
    return [Card(rank=r, suit=s) for s in SUITS for r in RANKS]


def iter_deck() -> Iterator[Card]:
    """Yield the canonical ordered deck without allocating a list."""
    for s in SUITS:
        for r in RANKS:
            yield Card(rank=r, suit=s)
