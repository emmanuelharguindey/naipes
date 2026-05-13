"""Brisca game state and transitions. Pure state machine, no I/O.

The CLI layer drives this. The AI consults it but never mutates it directly.
See SPEC.md §5.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum

from naipes.core.cards import Card, build_ordered_deck
from naipes.core.rng import Rng
from naipes.games.brisca.rules import (
    card_points,
    trick_points,
    trick_winner_is_follower,
)


class Player(Enum):
    HUMAN = "human"
    AI = "ai"

    def other(self) -> "Player":
        return Player.AI if self is Player.HUMAN else Player.HUMAN


@dataclass
class TrickRecord:
    """Immutable record of a completed trick. Used in trick log output."""

    trick_number: int
    leader: Player
    human_card: Card
    ai_card: Card
    winner: Player
    points: int


@dataclass
class GameState:
    """Mutable game state. Driven by play_* methods.

    Invariants:
    - Sum of cards in deck + both hands + captured piles + (1 if pending trick else 0) == 40
    - trump_card is the bottom of the deck until drawn.
    - len(human_hand), len(ai_hand) <= 3 (and == 3 until deck empties).
    """

    rng: Rng
    trump_suit: str
    trump_card: Card  # The visible trump card at the bottom of the deck.
    deck: list[Card]  # deck[-1] is the trump card until drawn. Draw from index 0.
    human_hand: list[Card]
    ai_hand: list[Card]
    human_captured: list[Card] = field(default_factory=list)
    ai_captured: list[Card] = field(default_factory=list)
    leader: Player = Player.HUMAN
    pending_leader_card: Card | None = None  # Card on table waiting for follower.
    pending_leader: Player | None = None
    trick_log: list[TrickRecord] = field(default_factory=list)

    @classmethod
    def new(cls, seed: int) -> "GameState":
        """Initial state after dealing. Human leads trick 1. SPEC.md §5.1."""
        rng = Rng(seed)
        deck = build_ordered_deck()
        rng.shuffle(deck)
        # Deal 3 to human, then 3 to AI, taking from the top (index 0).
        human_hand = [deck.pop(0) for _ in range(3)]
        ai_hand = [deck.pop(0) for _ in range(3)]
        # Flip the trump card; place at the bottom of the remaining deck.
        trump_card = deck.pop(0)
        trump_suit = trump_card.suit
        deck.append(trump_card)  # Bottom = end of list (drawn last).
        return cls(
            rng=rng,
            trump_suit=trump_suit,
            trump_card=trump_card,
            deck=deck,
            human_hand=human_hand,
            ai_hand=ai_hand,
        )

    # ---- Queries ----

    def is_finished(self) -> bool:
        return not self.human_hand and not self.ai_hand and self.pending_leader_card is None

    def hand_of(self, player: Player) -> list[Card]:
        return self.human_hand if player is Player.HUMAN else self.ai_hand

    def captured_of(self, player: Player) -> list[Card]:
        return self.human_captured if player is Player.HUMAN else self.ai_captured

    def score_of(self, player: Player) -> int:
        return sum(card_points(c) for c in self.captured_of(player))

    # ---- Transitions ----

    def play_card(self, player: Player, hand_index: int) -> TrickRecord | None:
        """Play card at hand_index for player. Returns TrickRecord if trick completed.

        Caller is responsible for calling this for the correct player at the
        correct moment (leader first, then follower). Out-of-turn plays raise.
        """
        if self.pending_leader_card is None:
            # This is the leader's play.
            if player is not self.leader:
                raise RuntimeError(
                    f"out-of-turn: leader is {self.leader}, got {player}"
                )
            hand = self.hand_of(player)
            if not (0 <= hand_index < len(hand)):
                raise IndexError(f"hand_index {hand_index} out of range")
            card = hand.pop(hand_index)
            self.pending_leader_card = card
            self.pending_leader = player
            return None

        # This is the follower's play.
        expected_follower = self.pending_leader.other()
        if player is not expected_follower:
            raise RuntimeError(
                f"out-of-turn: follower must be {expected_follower}, got {player}"
            )
        hand = self.hand_of(player)
        if not (0 <= hand_index < len(hand)):
            raise IndexError(f"hand_index {hand_index} out of range")
        follower_card = hand.pop(hand_index)
        leader_card = self.pending_leader_card
        leader = self.pending_leader

        follower_wins = trick_winner_is_follower(
            leader_card, follower_card, self.trump_suit
        )
        winner = expected_follower if follower_wins else leader
        points = trick_points(leader_card, follower_card)

        # Order in the record: human_card and ai_card always identify by player.
        human_card = leader_card if leader is Player.HUMAN else follower_card
        ai_card = leader_card if leader is Player.AI else follower_card

        record = TrickRecord(
            trick_number=len(self.trick_log) + 1,
            leader=leader,
            human_card=human_card,
            ai_card=ai_card,
            winner=winner,
            points=points,
        )
        self.trick_log.append(record)

        # Move both played cards to the winner's captured pile.
        self.captured_of(winner).append(leader_card)
        self.captured_of(winner).append(follower_card)

        # Clear pending.
        self.pending_leader_card = None
        self.pending_leader = None

        # Draw phase: winner draws first, then loser. SPEC.md §5.4.
        loser = winner.other()
        self._draw_one(winner)
        self._draw_one(loser)

        # Winner leads next trick. SPEC.md §5.5.
        self.leader = winner

        return record

    def _draw_one(self, player: Player) -> None:
        if not self.deck:
            return
        # Draw from the top (index 0). Trump card is at the end (drawn last).
        card = self.deck.pop(0)
        self.hand_of(player).append(card)
