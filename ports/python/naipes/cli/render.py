"""Render brisca state to text for the CLI. Pure: state in, string out.

Kept separate from the input loop so it can be tested without a TTY and
reused by future TUI/web frontends.
"""

from __future__ import annotations

from naipes.games.brisca.state import GameState, Player


SEP_DOUBLE = "═" * 39
SEP_SINGLE = "─" * 39


def render_board(state: GameState) -> str:
    """The frame shown to the player before each prompt. SPEC.md §7.1."""
    ai_count = len(state.ai_hand)
    deck_count = len(state.deck)

    trump_line = _render_trump_line(state)

    if state.pending_leader_card is not None and state.pending_leader is Player.AI:
        table = f"AI played: {state.pending_leader_card}"
    elif state.pending_leader_card is not None and state.pending_leader is Player.HUMAN:
        table = f"You played: {state.pending_leader_card} (waiting for AI)"
    else:
        table = "(empty)"

    hand_line = "  ".join(
        f"{i + 1}:{card}" for i, card in enumerate(state.human_hand)
    )
    if not hand_line:
        hand_line = "(no cards)"

    parts = [
        SEP_DOUBLE,
        f" AI: {ai_count} cards              Deck: {deck_count}",
    ]
    if trump_line is not None:
        parts.append(f" {trump_line}")
    parts.extend([
        SEP_SINGLE,
        f" Table: {table}",
        SEP_SINGLE,
        " Your hand:",
        f"   {hand_line}",
        SEP_DOUBLE,
    ])
    return "\n".join(parts)


def _render_trump_line(state: GameState) -> str | None:
    if state.deck:
        if len(state.deck) == 1 and state.deck[-1] == state.trump_card:
            return f"Trump: {state.trump_card} (last)"
        return f"Trump: {state.trump_card}"
    return f"Trump: {state.trump_card.suit} (suit)"


def render_trick_result(state: GameState) -> str:
    """Shown right after a trick completes. SPEC.md §7.3."""
    record = state.trick_log[-1]
    if record.leader is Player.HUMAN:
        ai_played = f" AI plays: {record.ai_card}"
    else:
        ai_played = f" AI led with: {record.ai_card}"
    winner_label = "you" if record.winner is Player.HUMAN else "AI"
    return f"{ai_played}\n Winner: {winner_label} (+{record.points} points)"


def render_final(state: GameState) -> str:
    """Final scoreboard. SPEC.md §7.4. Ends with the RESULT machine line."""
    human_score = state.score_of(Player.HUMAN)
    ai_score = state.score_of(Player.AI)
    if human_score > 60:
        verdict = "You won"
        outcome = "win"
    elif human_score < 60:
        verdict = "You lost"
        outcome = "loss"
    else:
        verdict = "Draw"
        outcome = "draw"
    block = "\n".join([
        SEP_DOUBLE,
        " Game over",
        f" Your score: {human_score}",
        f" AI score:   {ai_score}",
        f" {verdict}",
        SEP_DOUBLE,
        f"RESULT human={human_score} ai={ai_score} outcome={outcome}",
    ])
    return block
