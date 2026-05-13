import { test } from "node:test";
import assert from "node:assert/strict";
import { GameState } from "../src/games/brisca/state.js";
import { chooseEasy } from "../src/games/brisca/ai.js";
import { cardStrength } from "../src/games/brisca/rules.js";
import { replay } from "../src/cli/loop.js";

test("full game total points is 120", () => {
  const r = replay(1n, "normal", Array(20).fill(1));
  assert.equal(r.humanScore + r.aiScore, 120);
});

test("full game has 20 tricks", () => {
  const r = replay(1n, "normal", Array(20).fill(1));
  assert.equal(r.trickLog.length, 20);
});

test("outcome consistent with score", () => {
  const r = replay(1n, "normal", Array(20).fill(1));
  if (r.humanScore > 60) assert.equal(r.outcome, "win");
  else if (r.humanScore < 60) assert.equal(r.outcome, "loss");
  else assert.equal(r.outcome, "draw");
});

test("same seed same plays same result", () => {
  const a = replay(42n, "normal", Array(20).fill(1));
  const b = replay(42n, "normal", Array(20).fill(1));
  assert.equal(a.humanScore, b.humanScore);
  assert.equal(a.aiScore, b.aiScore);
  assert.deepEqual(
    a.trickLog.map((r) => [r.humanCard.label, r.aiCard.label]),
    b.trickLog.map((r) => [r.humanCard.label, r.aiCard.label]),
  );
});

test("initial state invariants", () => {
  const s = GameState.new(1n);
  assert.equal(s.humanHand.length, 3);
  assert.equal(s.aiHand.length, 3);
  assert.equal(s.deck.length, 34);
  assert.ok(s.deck[s.deck.length - 1]!.equals(s.trumpCard));
});

test("easy AI plays lowest strength", () => {
  const s = GameState.new(1n);
  const idx = chooseEasy(s);
  const chosen = s.aiHand[idx]!;
  for (const other of s.aiHand) {
    assert.ok(cardStrength(chosen) <= cardStrength(other));
  }
});
