/**
 * Conformance test: load each test vector and verify the TypeScript port
 * reproduces it exactly, baza por baza.
 *
 * If this test fails, the TS port has diverged from spec — fix the port,
 * not the vectors.
 */

import { test } from "node:test";
import assert from "node:assert/strict";
import { readdirSync, readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

import { replay } from "../src/cli/loop.js";
import type { AiLevel } from "../src/games/brisca/ai.js";

interface Vector {
  id: string;
  seed: number;
  ai_level: AiLevel;
  human_plays: number[];
  expected_result: {
    human: number;
    ai: number;
    outcome: "win" | "loss" | "draw";
  };
  expected_trick_log: Array<{
    trick: number;
    leader: "human" | "ai";
    human_card: string;
    ai_card: string;
    winner: "human" | "ai";
    points: number;
  }>;
}

const here = dirname(fileURLToPath(import.meta.url));
const vectorsDir = join(here, "..", "..", "..", "spec", "vectors");

const vectorFiles = readdirSync(vectorsDir)
  .filter((f) => /^v\d{3}\.json$/.test(f))
  .sort();

for (const file of vectorFiles) {
  const vector: Vector = JSON.parse(
    readFileSync(join(vectorsDir, file), "utf8"),
  );

  test(`conformance ${vector.id} (seed=${vector.seed} ai=${vector.ai_level})`, () => {
    const result = replay(
      BigInt(vector.seed),
      vector.ai_level,
      vector.human_plays,
    );

    assert.equal(
      result.humanScore,
      vector.expected_result.human,
      `${vector.id}: human score mismatch`,
    );
    assert.equal(
      result.aiScore,
      vector.expected_result.ai,
      `${vector.id}: ai score mismatch`,
    );
    assert.equal(
      result.outcome,
      vector.expected_result.outcome,
      `${vector.id}: outcome mismatch`,
    );
    assert.equal(
      result.trickLog.length,
      vector.expected_trick_log.length,
      `${vector.id}: trick count mismatch`,
    );

    for (let i = 0; i < result.trickLog.length; i++) {
      const got = result.trickLog[i]!;
      const expected = vector.expected_trick_log[i]!;
      assert.equal(got.trickNumber, expected.trick, `${vector.id} trick ${i + 1} number`);
      assert.equal(got.leader, expected.leader, `${vector.id} trick ${i + 1} leader`);
      assert.equal(
        got.humanCard.label,
        expected.human_card,
        `${vector.id} trick ${i + 1} human_card`,
      );
      assert.equal(
        got.aiCard.label,
        expected.ai_card,
        `${vector.id} trick ${i + 1} ai_card`,
      );
      assert.equal(got.winner, expected.winner, `${vector.id} trick ${i + 1} winner`);
      assert.equal(got.points, expected.points, `${vector.id} trick ${i + 1} points`);
    }
  });
}
