/**
 * Brisca game state and transitions. Pure state machine, no I/O.
 * See SPEC.md §5.
 */

import { Card, buildOrderedDeck, type Suit } from "../../core/cards.js";
import { Rng } from "../../core/rng.js";
import { cardPoints, trickPoints, trickWinnerIsFollower } from "./rules.js";

export type Player = "human" | "ai";

export function otherPlayer(p: Player): Player {
  return p === "human" ? "ai" : "human";
}

export interface TrickRecord {
  trickNumber: number;
  leader: Player;
  humanCard: Card;
  aiCard: Card;
  winner: Player;
  points: number;
}

export class GameState {
  rng: Rng;
  trumpSuit: Suit;
  trumpCard: Card;
  /** deck[deck.length - 1] is the trump card until drawn. Draw from index 0. */
  deck: Card[];
  humanHand: Card[];
  aiHand: Card[];
  humanCaptured: Card[] = [];
  aiCaptured: Card[] = [];
  leader: Player = "human";
  pendingLeaderCard: Card | null = null;
  pendingLeader: Player | null = null;
  trickLog: TrickRecord[] = [];

  private constructor(
    rng: Rng,
    trumpSuit: Suit,
    trumpCard: Card,
    deck: Card[],
    humanHand: Card[],
    aiHand: Card[],
  ) {
    this.rng = rng;
    this.trumpSuit = trumpSuit;
    this.trumpCard = trumpCard;
    this.deck = deck;
    this.humanHand = humanHand;
    this.aiHand = aiHand;
  }

  /** Initial state after dealing. Human leads trick 1. SPEC.md §5.1. */
  static new(seed: bigint | number): GameState {
    const rng = new Rng(seed);
    const deck = buildOrderedDeck();
    rng.shuffle(deck);
    // Deal 3 to human, then 3 to AI, taking from the top (index 0).
    const humanHand: Card[] = [];
    for (let i = 0; i < 3; i++) humanHand.push(deck.shift()!);
    const aiHand: Card[] = [];
    for (let i = 0; i < 3; i++) aiHand.push(deck.shift()!);
    // Flip the trump card; place at the bottom of the remaining deck.
    const trumpCard = deck.shift()!;
    const trumpSuit = trumpCard.suit;
    deck.push(trumpCard); // bottom == end of list (drawn last)
    return new GameState(rng, trumpSuit, trumpCard, deck, humanHand, aiHand);
  }

  // ---- Queries ----

  isFinished(): boolean {
    return (
      this.humanHand.length === 0 &&
      this.aiHand.length === 0 &&
      this.pendingLeaderCard === null
    );
  }

  handOf(player: Player): Card[] {
    return player === "human" ? this.humanHand : this.aiHand;
  }

  capturedOf(player: Player): Card[] {
    return player === "human" ? this.humanCaptured : this.aiCaptured;
  }

  scoreOf(player: Player): number {
    return this.capturedOf(player).reduce((sum, c) => sum + cardPoints(c), 0);
  }

  // ---- Transitions ----

  /**
   * Play card at handIndex for player. Returns TrickRecord if trick completed.
   *
   * Caller is responsible for the correct player at the correct moment
   * (leader first, then follower). Out-of-turn plays throw.
   */
  playCard(player: Player, handIndex: number): TrickRecord | null {
    if (this.pendingLeaderCard === null) {
      // Leader's play.
      if (player !== this.leader) {
        throw new Error(
          `out-of-turn: leader is ${this.leader}, got ${player}`,
        );
      }
      const hand = this.handOf(player);
      if (handIndex < 0 || handIndex >= hand.length) {
        throw new RangeError(`handIndex ${handIndex} out of range`);
      }
      const card = hand.splice(handIndex, 1)[0]!;
      this.pendingLeaderCard = card;
      this.pendingLeader = player;
      return null;
    }

    // Follower's play.
    const expectedFollower = otherPlayer(this.pendingLeader!);
    if (player !== expectedFollower) {
      throw new Error(
        `out-of-turn: follower must be ${expectedFollower}, got ${player}`,
      );
    }
    const hand = this.handOf(player);
    if (handIndex < 0 || handIndex >= hand.length) {
      throw new RangeError(`handIndex ${handIndex} out of range`);
    }
    const followerCard = hand.splice(handIndex, 1)[0]!;
    const leaderCard = this.pendingLeaderCard;
    const leader = this.pendingLeader!;

    const followerWins = trickWinnerIsFollower(
      leaderCard,
      followerCard,
      this.trumpSuit,
    );
    const winner = followerWins ? expectedFollower : leader;
    const points = trickPoints(leaderCard, followerCard);

    const humanCard = leader === "human" ? leaderCard : followerCard;
    const aiCard = leader === "ai" ? leaderCard : followerCard;

    const record: TrickRecord = {
      trickNumber: this.trickLog.length + 1,
      leader,
      humanCard,
      aiCard,
      winner,
      points,
    };
    this.trickLog.push(record);

    // Move both played cards to winner's captured pile.
    this.capturedOf(winner).push(leaderCard);
    this.capturedOf(winner).push(followerCard);

    // Clear pending.
    this.pendingLeaderCard = null;
    this.pendingLeader = null;

    // Draw phase: winner draws first, then loser. SPEC.md §5.4.
    const loser = otherPlayer(winner);
    this.drawOne(winner);
    this.drawOne(loser);

    // Winner leads next trick. SPEC.md §5.5.
    this.leader = winner;

    return record;
  }

  private drawOne(player: Player): void {
    if (this.deck.length === 0) return;
    // Draw from the top (index 0). Trump card is at the end.
    const card = this.deck.shift()!;
    this.handOf(player).push(card);
  }
}
