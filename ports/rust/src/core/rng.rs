//! Deterministic PRNG shared across all naipes ports.
//!
//! Implements xorshift64* exactly as specified in spec/SPEC.md §4.
//!
//! Rust has native u64 with wrapping arithmetic — no BigInt gymnastics
//! like the TypeScript port. This is the cleanest of the implementations.

const MULTIPLIER: u64 = 0x2545F4914F6CDD1D;
const ZERO_SEED_SUBSTITUTE: u64 = 0xDEADBEEFCAFEBABE;

#[derive(Debug, Clone)]
pub struct Rng {
    state: u64,
}

impl Rng {
    /// Create a new PRNG with the given seed. Seed 0 is silently substituted.
    pub fn new(seed: u64) -> Self {
        let state = if seed == 0 {
            ZERO_SEED_SUBSTITUTE
        } else {
            seed
        };
        Self { state }
    }

    /// Advance state, return a u64 sample.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        // Wrapping multiply: u64 * u64 mod 2^64.
        x.wrapping_mul(MULTIPLIER)
    }

    /// Uniform integer in [0, n) via rejection sampling. SPEC.md §4.3.
    ///
    /// Required (rather than `next_u64() % n`) so that all ports produce
    /// bit-identical sequences.
    pub fn bounded(&mut self, n: u32) -> u32 {
        assert!(n > 0, "n must be positive");
        let n = n as u64;
        // (2^64) mod n, computed without overflow.
        // (2^64 - 1) mod n + 1, with overflow handled.
        let threshold = (u64::MAX % n).wrapping_add(1) % n;
        loop {
            let v = self.next_u64();
            if v >= threshold {
                return (v % n) as u32;
            }
        }
    }

    /// In-place Fisher-Yates shuffle from the end. SPEC.md §4.4.
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        if items.len() < 2 {
            return;
        }
        for i in (1..items.len()).rev() {
            let j = self.bounded((i + 1) as u32) as usize;
            items.swap(i, j);
        }
    }
}
