/**
 * Pure brisca rules. No state, no side effects. See SPEC.md §3.2 and §5.
 */

import { Card, type Suit } from "../../core/cards.js";

/** rank -> points captured. SPEC.md §3.2. */
export const POINTS: Record<number, number> = {
  1: 11,   // As
  3: 10,   // Tres
  12: 4,   // Rey
  11: 3,   // Caballo
  10: 2,   // Sota
  7: 0,
  6: 0,
  5: 0,
  4: 0,
  2: 0,
};

/** rank -> strength for trick resolution. SPEC.md §3.2. */
export const STRENGTH: Record<number, number> = {
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
};

export function cardPoints(card: Card): number {
  return POINTS[card.rank]!;
}

export function cardStrength(card: Card): number {
  return STRENGTH[card.rank]!;
}

/**
 * Return true iff the follower wins the trick. SPEC.md §5.3.
 */
export function trickWinnerIsFollower(
  leader: Card,
  follower: Card,
  trumpSuit: Suit,
): boolean {
  const leaderTrump = leader.suit === trumpSuit;
  const followerTrump = follower.suit === trumpSuit;

  // Both trump: higher strength wins.
  if (leaderTrump && followerTrump) {
    return cardStrength(follower) > cardStrength(leader);
  }
  // Exactly one trump: that one wins.
  if (followerTrump) return true;
  if (leaderTrump) return false;

  // Neither trump. Follower must match lead suit to beat the leader.
  if (follower.suit !== leader.suit) return false;
  return cardStrength(follower) > cardStrength(leader);
}

export function trickPoints(leader: Card, follower: Card): number {
  return cardPoints(leader) + cardPoints(follower);
}
