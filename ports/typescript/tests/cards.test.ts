import { test } from "node:test";
import assert from "node:assert/strict";
import { Card, buildOrderedDeck, RANKS } from "../src/core/cards.js";

test("deck has 40 cards", () => {
  assert.equal(buildOrderedDeck().length, 40);
});

test("deck has no duplicates", () => {
  const deck = buildOrderedDeck();
  const labels = new Set(deck.map((c) => c.label));
  assert.equal(labels.size, 40);
});

test("first card is As de Oros", () => {
  const deck = buildOrderedDeck();
  assert.equal(deck[0]!.label, "AsO");
});

test("last card is Rey de Bastos", () => {
  const deck = buildOrderedDeck();
  assert.equal(deck[39]!.label, "RB");
});

test("no 8 or 9 in ranks", () => {
  assert.ok(!RANKS.includes(8));
  assert.ok(!RANKS.includes(9));
});

test("card labels are correct", () => {
  assert.equal(new Card(1, "O").label, "AsO");
  assert.equal(new Card(3, "E").label, "3E");
  assert.equal(new Card(12, "C").label, "RC");
  assert.equal(new Card(10, "B").label, "SB");
  assert.equal(new Card(11, "O").label, "CO");
});

test("invalid rank throws", () => {
  assert.throws(() => new Card(8, "O"));
  assert.throws(() => new Card(9, "O"));
});
