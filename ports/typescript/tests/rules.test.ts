import { test } from "node:test";
import assert from "node:assert/strict";
import { Card } from "../src/core/cards.js";
import {
  cardPoints,
  cardStrength,
  trickWinnerIsFollower,
} from "../src/games/brisca/rules.js";

const TRUMP = "O" as const;

test("both trump: higher strength wins", () => {
  assert.equal(trickWinnerIsFollower(new Card(2, TRUMP), new Card(1, TRUMP), TRUMP), true);
});

test("both trump: lower loses", () => {
  assert.equal(trickWinnerIsFollower(new Card(1, TRUMP), new Card(2, TRUMP), TRUMP), false);
});

test("follower trumps non-trump lead", () => {
  assert.equal(trickWinnerIsFollower(new Card(1, "C"), new Card(2, TRUMP), TRUMP), true);
});

test("leader trump beats non-trump follower", () => {
  assert.equal(trickWinnerIsFollower(new Card(2, TRUMP), new Card(1, "C"), TRUMP), false);
});

test("same suit higher follower wins", () => {
  assert.equal(trickWinnerIsFollower(new Card(7, "C"), new Card(1, "C"), TRUMP), true);
});

test("same suit lower follower loses", () => {
  assert.equal(trickWinnerIsFollower(new Card(1, "C"), new Card(7, "C"), TRUMP), false);
});

test("off-suit non-trump follower loses always", () => {
  assert.equal(trickWinnerIsFollower(new Card(2, "C"), new Card(1, "E"), TRUMP), false);
});

test("canonical points", () => {
  assert.equal(cardPoints(new Card(1, "O")), 11);
  assert.equal(cardPoints(new Card(3, "O")), 10);
  assert.equal(cardPoints(new Card(12, "O")), 4);
  assert.equal(cardPoints(new Card(11, "O")), 3);
  assert.equal(cardPoints(new Card(10, "O")), 2);
  assert.equal(cardPoints(new Card(7, "O")), 0);
  assert.equal(cardPoints(new Card(2, "O")), 0);
});

test("strength ordering", () => {
  assert.ok(cardStrength(new Card(1, "O")) > cardStrength(new Card(3, "O")));
  assert.ok(cardStrength(new Card(3, "O")) > cardStrength(new Card(12, "O")));
  assert.ok(cardStrength(new Card(12, "O")) > cardStrength(new Card(11, "O")));
  assert.ok(cardStrength(new Card(11, "O")) > cardStrength(new Card(10, "O")));
  assert.equal(cardStrength(new Card(2, "O")), 1);
});
