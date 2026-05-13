"""Brisca interactive loop and headless replay driver.

Two entry points:
- `play_interactive` — used by `naipes play brisca` (reads from stdin).
- `replay` — used by test vectors (consumes a scripted list of human plays
  and returns the full trick log + final scores, no I/O).

Both share the same underlying loop so the conformance guarantee is real.
"""

from __future__ import annotations

import sys
from dataclasses import dataclass
from typing import Callable, Iterator

from naipes.cli.render import render_board, render_final, render_trick_result
from naipes.games.brisca.ai import choose as choose_ai_move
from naipes.games.brisca.state import GameState, Player, TrickRecord


@dataclass
class ReplayResult:
    """Outcome of a scripted replay. Used by test vectors."""

    human_score: int
    ai_score: int
    outcome: str  # win | loss | draw
    trick_log: list[TrickRecord]


def _outcome_for(human: int) -> str:
    if human > 60:
        return "win"
    if human < 60:
        return "loss"
    return "draw"


def replay(seed: int, ai_level: str, human_plays: list[int]) -> ReplayResult:
    """Run a brisca game from a scripted sequence of human plays.

    `human_plays` is a list of 1-indexed hand positions the human picks,
    in the order they're prompted. Length must equal the number of times
    the human is to play (20 in a complete game, since the human plays
    in every trick).

    Raises if the script under-supplies plays, ignores extras at the end.
    """
    state = GameState.new(seed)
    plays_iter = iter(human_plays)

    while not state.is_finished():
        _advance_one_step(state, ai_level, plays_iter)

    return ReplayResult(
        human_score=state.score_of(Player.HUMAN),
        ai_score=state.score_of(Player.AI),
        outcome=_outcome_for(state.score_of(Player.HUMAN)),
        trick_log=state.trick_log,
    )


def _advance_one_step(
    state: GameState,
    ai_level: str,
    human_plays_iter: Iterator[int],
) -> None:
    """Advance the state until one full trick is resolved.

    Drives leader-then-follower play, querying the human via the iterator
    (1-indexed plays) and the AI via the strategy module.
    """
    # First half of the trick: the leader plays.
    if state.leader is Player.HUMAN:
        idx = next(human_plays_iter) - 1
        state.play_card(Player.HUMAN, idx)
        ai_idx = choose_ai_move(state, ai_level)
        state.play_card(Player.AI, ai_idx)
    else:
        ai_idx = choose_ai_move(state, ai_level)
        state.play_card(Player.AI, ai_idx)
        idx = next(human_plays_iter) - 1
        state.play_card(Player.HUMAN, idx)


# ---- Interactive loop ----


def play_interactive(
    seed: int,
    ai_level: str,
    *,
    quiet: bool = False,
    stdin=None,
    stdout=None,
) -> int:
    """Run a brisca game with stdin/stdout. Returns process exit code."""
    in_ = stdin if stdin is not None else sys.stdin
    out = stdout if stdout is not None else sys.stdout

    def emit(text: str = "") -> None:
        print(text, file=out)

    def prompt_human() -> int | None:
        """Return 1-indexed hand position, or None if the user quit."""
        while True:
            emit(render_board(state))
            try:
                raw = input("> ").strip().lower()
            except EOFError:
                return None
            if raw in ("q", "quit"):
                return None
            if raw in ("?", "help"):
                emit("Commands: 1-3 (play card), q (quit), ? (help)")
                continue
            if raw in ("1", "2", "3"):
                idx_1 = int(raw)
                if idx_1 > len(state.human_hand):
                    emit(f"You only have {len(state.human_hand)} cards.")
                    continue
                return idx_1
            emit("Invalid input. Use 1-3, q, or ?.")

    state = GameState.new(seed)

    while not state.is_finished():
        if state.leader is Player.HUMAN:
            human_idx_1 = prompt_human()
            if human_idx_1 is None:
                emit("Game abandoned.")
                return 0
            state.play_card(Player.HUMAN, human_idx_1 - 1)
            ai_idx = choose_ai_move(state, ai_level)
            state.play_card(Player.AI, ai_idx)
        else:
            # AI leads. Show board with AI's card on the table, then ask human.
            ai_idx = choose_ai_move(state, ai_level)
            state.play_card(Player.AI, ai_idx)
            human_idx_1 = prompt_human()
            if human_idx_1 is None:
                emit("Game abandoned.")
                return 0
            state.play_card(Player.HUMAN, human_idx_1 - 1)

        emit(render_trick_result(state))
        if not quiet and not state.is_finished():
            try:
                input("(Press Enter to continue) ")
            except EOFError:
                pass

    emit(render_final(state))
    return 0
