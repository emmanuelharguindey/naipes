import { test } from "node:test";
import assert from "node:assert/strict";
import { Rng } from "../src/core/rng.js";

test("zero seed is substituted", () => {
  const a = new Rng(0n);
  const b = new Rng(0xDEADBEEFCAFEBABEn);
  assert.equal(a.nextU64(), b.nextU64());
});

test("seed 1 produces 5 distinct u64 values in range", () => {
  const rng = new Rng(1n);
  const seq = Array.from({ length: 5 }, () => rng.nextU64());
  for (const v of seq) {
    assert.ok(v >= 0n);
    assert.ok(v < (1n << 64n));
  }
  assert.equal(new Set(seq).size, 5);
});

test("bounded stays in [0, n)", () => {
  const rng = new Rng(42n);
  for (let i = 0; i < 1000; i++) {
    const v = rng.bounded(40);
    assert.ok(v >= 0 && v < 40);
  }
});

test("shuffle is a permutation", () => {
  const rng = new Rng(7n);
  const items = Array.from({ length: 40 }, (_, i) => i);
  rng.shuffle(items);
  const sorted = [...items].sort((a, b) => a - b);
  assert.deepEqual(sorted, Array.from({ length: 40 }, (_, i) => i));
});

test("same seed produces same shuffle", () => {
  const a = Array.from({ length: 40 }, (_, i) => i);
  const b = Array.from({ length: 40 }, (_, i) => i);
  new Rng(123n).shuffle(a);
  new Rng(123n).shuffle(b);
  assert.deepEqual(a, b);
});

test("different seed produces different shuffle", () => {
  const a = Array.from({ length: 40 }, (_, i) => i);
  const b = Array.from({ length: 40 }, (_, i) => i);
  new Rng(1n).shuffle(a);
  new Rng(2n).shuffle(b);
  assert.notDeepEqual(a, b);
});
