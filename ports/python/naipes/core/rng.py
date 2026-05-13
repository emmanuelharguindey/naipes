"""Deterministic PRNG shared across all naipes ports.

Implements xorshift64* exactly as specified in spec/SPEC.md §4.
This module is the single source of randomness in naipes — no use of
random.random() or secrets anywhere else.
"""

from __future__ import annotations

_U64_MASK = (1 << 64) - 1
_MULTIPLIER = 0x2545F4914F6CDD1D
_ZERO_SEED_SUBSTITUTE = 0xDEADBEEFCAFEBABE


class Rng:
    """xorshift64* deterministic PRNG.

    Identical sequences across all naipes ports for identical seeds.
    """

    __slots__ = ("_state",)

    def __init__(self, seed: int) -> None:
        if not isinstance(seed, int):
            raise TypeError("seed must be int")
        seed &= _U64_MASK
        if seed == 0:
            seed = _ZERO_SEED_SUBSTITUTE
        self._state = seed

    def next_u64(self) -> int:
        """Advance state, return a u64 sample."""
        x = self._state
        x ^= (x >> 12) & _U64_MASK
        x = (x ^ ((x << 25) & _U64_MASK)) & _U64_MASK
        x ^= (x >> 27) & _U64_MASK
        self._state = x
        return (x * _MULTIPLIER) & _U64_MASK

    def bounded(self, n: int) -> int:
        """Uniform integer in [0, n) via rejection sampling.

        Required (rather than next() % n) so that all ports produce
        bit-identical sequences. See SPEC.md §4.3.
        """
        if n <= 0:
            raise ValueError("n must be positive")
        if n > (1 << 32):
            raise ValueError("n must be <= 2^32")
        threshold = ((1 << 64) % n)
        while True:
            v = self.next_u64()
            if v >= threshold:
                return v % n

    def shuffle(self, items: list) -> None:
        """In-place Fisher-Yates shuffle from the end. SPEC.md §4.4."""
        for i in range(len(items) - 1, 0, -1):
            j = self.bounded(i + 1)
            items[i], items[j] = items[j], items[i]
