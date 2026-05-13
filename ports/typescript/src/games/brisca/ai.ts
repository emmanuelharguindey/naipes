/**
 * Brisca AI strategies. Pure functions over GameState. See SPEC.md §6.
 */

import { Card } from "../../core/cards.js";
import { cardPoints, cardStrength, trickWinnerIsFollower } from "./rules.js";
import { GameState } from "./state.js";

/**
 * Index of the lowest-strength card in hand matching predicate.
 * Ties broken by hand position (earliest insertion = lowest index). SPEC.md §6.4.
 */
function lowestStrengthIndex(
  hand: readonly Card[],
  predicate?: (c: Card) => boolean,
): number | null {
  let bestIdx: number | null = null;
  let bestStrength = 999;
  for (let i = 0; i < hand.length; i++) {
    const card = hand[i]!;
    if (predicate && !predicate(card)) continue;
    const s = cardStrength(card);
    if (s < bestStrength) {
      bestStrength = s;
      bestIdx = i;
    }
  }
  return bestIdx;
}

/** Always lowest-strength card in hand. SPEC.md §6.1. */
export function chooseEasy(state: GameState): number {
  const idx = lowestStrengthIndex(state.aiHand);
  if (idx === null) throw new Error("AI asked to play with empty hand");
  return idx;
}

/** SPEC.md §6.2. Different logic for leading vs following. */
export function chooseNormal(state: GameState): number {
  const hand = state.aiHand;
  const trump = state.trumpSuit;

  if (state.pendingLeaderCard === null) {
    // Leading.
    // 1. Lowest-strength non-trump rank-0-points card.
    let idx = lowestStrengthIndex(
      hand,
      (c) => c.suit !== trump && cardPoints(c) === 0,
    );
    if (idx !== null) return idx;
    // 2. Lowest-strength non-trump card.
    idx = lowestStrengthIndex(hand, (c) => c.suit !== trump);
    if (idx !== null) return idx;
    // 3. Lowest-strength trump card.
    idx = lowestStrengthIndex(hand);
    if (idx === null) throw new Error("AI asked to play with empty hand");
    return idx;
  }

  // Following.
  const leaderCard = state.pendingLeaderCard;
  const wouldWin = (c: Card) =>
    trickWinnerIsFollower(leaderCard, c, trump);

  // 1. Lowest non-trump that wins.
  let idx = lowestStrengthIndex(
    hand,
    (c) => c.suit !== trump && wouldWin(c),
  );
  if (idx !== null) return idx;

  // 2. Leader's card carries >= 10 points: trump to win if possible.
  if (cardPoints(leaderCard) >= 10) {
    idx = lowestStrengthIndex(
      hand,
      (c) => c.suit === trump && wouldWin(c),
    );
    if (idx !== null) return idx;
  }

  // 3. Dump lowest non-trump, else lowest trump.
  idx = lowestStrengthIndex(hand, (c) => c.suit !== trump);
  if (idx !== null) return idx;
  idx = lowestStrengthIndex(hand);
  if (idx === null) throw new Error("AI asked to play with empty hand");
  return idx;
}

// Hard deferred to v0.2.0 per SPEC.md §6.3; falls back to Normal.
export const chooseHard = chooseNormal;

export type AiLevel = "easy" | "normal" | "hard";

export function choose(state: GameState, level: AiLevel): number {
  switch (level) {
    case "easy":
      return chooseEasy(state);
    case "normal":
      return chooseNormal(state);
    case "hard":
      return chooseHard(state);
  }
}
