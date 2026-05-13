/**
 * Deterministic PRNG shared across all naipes ports.
 *
 * Implements xorshift64* exactly as specified in spec/SPEC.md §4.
 *
 * CRITICAL: JavaScript numbers lose precision above 2^53, so we use BigInt
 * with explicit 64-bit masking on every operation. Performance is fine
 * (a brisca game needs ~80 PRNG calls).
 */

const U64_MASK = (1n << 64n) - 1n;
const MULTIPLIER = 0x2545F4914F6CDD1Dn;
const ZERO_SEED_SUBSTITUTE = 0xDEADBEEFCAFEBABEn;

export class Rng {
  private state: bigint;

  constructor(seed: bigint | number) {
    let s = typeof seed === "bigint" ? seed : BigInt(seed);
    s = s & U64_MASK;
    if (s === 0n) {
      s = ZERO_SEED_SUBSTITUTE;
    }
    this.state = s;
  }

  /** Advance state, return a u64 sample as bigint. */
  nextU64(): bigint {
    let x = this.state;
    x = (x ^ (x >> 12n)) & U64_MASK;
    x = (x ^ ((x << 25n) & U64_MASK)) & U64_MASK;
    x = (x ^ (x >> 27n)) & U64_MASK;
    this.state = x;
    return (x * MULTIPLIER) & U64_MASK;
  }

  /**
   * Uniform integer in [0, n) via rejection sampling.
   *
   * Required (rather than `nextU64() % n`) so that all ports produce
   * bit-identical sequences. See SPEC.md §4.3.
   *
   * `n` accepts a regular number for ergonomics — must be in (0, 2^32].
   */
  bounded(n: number): number {
    if (!Number.isInteger(n) || n <= 0) {
      throw new RangeError("n must be a positive integer");
    }
    if (n > 2 ** 32) {
      throw new RangeError("n must be <= 2^32");
    }
    const nb = BigInt(n);
    const threshold = (1n << 64n) % nb; // wrap-around region
    while (true) {
      const v = this.nextU64();
      if (v >= threshold) {
        // Result fits in 32 bits since n <= 2^32.
        return Number(v % nb);
      }
    }
  }

  /** In-place Fisher-Yates shuffle from the end. SPEC.md §4.4. */
  shuffle<T>(items: T[]): void {
    for (let i = items.length - 1; i > 0; i--) {
      const j = this.bounded(i + 1);
      // Non-null assertions safe: i and j are valid indices by construction.
      const tmp = items[i]!;
      items[i] = items[j]!;
      items[j] = tmp;
    }
  }
}
