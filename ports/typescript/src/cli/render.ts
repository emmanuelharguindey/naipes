/**
 * Render brisca state to text for the CLI. Pure: state in, string out.
 */

import { GameState } from "../games/brisca/state.js";

const SEP_DOUBLE = "═".repeat(39);
const SEP_SINGLE = "─".repeat(39);

function renderTrumpLine(state: GameState): string | null {
  if (state.deck.length > 0) {
    if (state.deck.length === 1 && state.deck[state.deck.length - 1]!.equals(state.trumpCard)) {
      return `Trump: ${state.trumpCard} (last)`;
    }
    return `Trump: ${state.trumpCard}`;
  }
  return `Trump: ${state.trumpCard.suit} (suit)`;
}

export function renderBoard(state: GameState): string {
  const aiCount = state.aiHand.length;
  const deckCount = state.deck.length;
  const trumpLine = renderTrumpLine(state);

  let table: string;
  if (state.pendingLeaderCard !== null && state.pendingLeader === "ai") {
    table = `AI played: ${state.pendingLeaderCard}`;
  } else if (state.pendingLeaderCard !== null && state.pendingLeader === "human") {
    table = `You played: ${state.pendingLeaderCard} (waiting for AI)`;
  } else {
    table = "(empty)";
  }

  let handLine = state.humanHand
    .map((c, i) => `${i + 1}:${c}`)
    .join("  ");
  if (!handLine) handLine = "(no cards)";

  const parts: string[] = [
    SEP_DOUBLE,
    ` AI: ${aiCount} cards              Deck: ${deckCount}`,
  ];
  if (trumpLine !== null) parts.push(` ${trumpLine}`);
  parts.push(
    SEP_SINGLE,
    ` Table: ${table}`,
    SEP_SINGLE,
    " Your hand:",
    `   ${handLine}`,
    SEP_DOUBLE,
  );
  return parts.join("\n");
}

export function renderTrickResult(state: GameState): string {
  const record = state.trickLog[state.trickLog.length - 1]!;
  const aiPlayed =
    record.leader === "human"
      ? ` AI plays: ${record.aiCard}`
      : ` AI led with: ${record.aiCard}`;
  const winnerLabel = record.winner === "human" ? "you" : "AI";
  return `${aiPlayed}\n Winner: ${winnerLabel} (+${record.points} points)`;
}

export function renderFinal(state: GameState): string {
  const humanScore = state.scoreOf("human");
  const aiScore = state.scoreOf("ai");
  let verdict: string;
  let outcome: "win" | "loss" | "draw";
  if (humanScore > 60) {
    verdict = "You won";
    outcome = "win";
  } else if (humanScore < 60) {
    verdict = "You lost";
    outcome = "loss";
  } else {
    verdict = "Draw";
    outcome = "draw";
  }
  return [
    SEP_DOUBLE,
    " Game over",
    ` Your score: ${humanScore}`,
    ` AI score:   ${aiScore}`,
    ` ${verdict}`,
    SEP_DOUBLE,
    `RESULT human=${humanScore} ai=${aiScore} outcome=${outcome}`,
  ].join("\n");
}
