# naipes — Specification v0.1.0

This document defines the canonical behaviour of `naipes`, a command-line application for playing traditional card games. Any implementation of `naipes` must conform to this specification. Conformance is verified by passing all test vectors in `spec/vectors/`.

The keywords MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as in RFC 2119.

---

## 1. Scope

`naipes` is an umbrella CLI for traditional card games. Version 0.1.0 ships with one game: **brisca** (a Spanish trick-taking game played with a Spanish 40-card deck). Future games such as `tute`, `mus`, or others extend the same CLI without breaking changes.

---

## 2. CLI surface

### 2.1 Commands

```
naipes                            → print help and exit 0
naipes --version                  → print "naipes <semver>" and exit 0
naipes --help                     → print help and exit 0
naipes list                       → list available games, one per line
naipes rules <game>               → print rules summary for <game>
naipes play <game> [options]      → start a game
```

### 2.2 Options for `play`

```
--seed <u64>          deterministic PRNG seed (default: random from OS entropy)
--ai-level <level>    easy | normal | hard (default: normal)
--no-color            disable ANSI colour output
--quiet               suppress non-essential output
```

### 2.3 Exit codes

| Code | Meaning |
|------|---------|
| 0 | Normal exit (game finished, help shown, version shown) |
| 1 | Generic error |
| 2 | Invalid arguments |
| 130 | User interrupted (Ctrl-C, SIGINT) |

### 2.4 Unknown game

`naipes play unknown-game` MUST print to stderr: `naipes: unknown game 'unknown-game'. Run 'naipes list' to see available games.` and exit 2.

---

## 3. The Spanish 40-card deck

### 3.1 Suits

Four suits, identified by single-character ASCII codes:

| Code | Spanish | English | Unicode glyph (display only) |
|------|---------|---------|------------------------------|
| `O` | Oros | Coins | 🪙 |
| `C` | Copas | Cups | 🏆 |
| `E` | Espadas | Swords | ⚔ |
| `B` | Bastos | Clubs | 🌿 |

### 3.2 Ranks

Ten ranks per suit. The internal numeric value (`rank`) is what gets stored; the display label is what gets printed.

| rank | label | name | points (brisca) | strength (brisca) |
|------|-------|------|-----------------|-------------------|
| 1 | `As` | As | 11 | 10 (highest) |
| 3 | `3` | Tres | 10 | 9 |
| 12 | `R` | Rey | 4 | 8 |
| 11 | `C` | Caballo | 3 | 7 |
| 10 | `S` | Sota | 2 | 6 |
| 7 | `7` | Siete | 0 | 5 |
| 6 | `6` | Seis | 0 | 4 |
| 5 | `5` | Cinco | 0 | 3 |
| 4 | `4` | Cuatro | 0 | 2 |
| 2 | `2` | Dos | 0 | 1 (lowest) |

Note: ranks 8 and 9 do not exist in the Spanish deck.

### 3.3 Card identity

A card is the pair `(rank, suit)`. Canonical string form is `<label><suit>`, e.g. `AsO` (As de Oros), `3E` (Tres de Espadas), `RC` (Rey de Copas).

### 3.4 Deck ordering

The unshuffled deck MUST be ordered as: for each suit in order `[O, C, E, B]`, all ranks in order `[1, 2, 3, 4, 5, 6, 7, 10, 11, 12]`. Resulting deck (40 cards):

```
AsO, 2O, 3O, 4O, 5O, 6O, 7O, SO, CO, RO,
AsC, 2C, 3C, 4C, 5C, 6C, 7C, SC, CC, RC,
AsE, 2E, 3E, 4E, 5E, 6E, 7E, SE, CE, RE,
AsB, 2B, 3B, 4B, 5B, 6B, 7B, SB, CB, RB
```

---

## 4. PRNG (critical for cross-implementation determinism)

### 4.1 Algorithm

All implementations MUST implement **xorshift64\*** with these exact parameters:

```
state: u64 (must be non-zero)
next():
    x = state
    x ^= x >> 12
    x ^= x << 25
    x ^= x >> 27
    state = x
    return (x * 0x2545F4914F6CDD1D) mod 2^64
```

Operations are on unsigned 64-bit integers; multiplication wraps modulo 2^64.

### 4.2 Seeding

The user-provided seed is a `u64`. If the seed is `0`, the implementation MUST substitute `0xDEADBEEFCAFEBABE` to keep state non-zero. The substitution MUST be silent.

### 4.3 Bounded integers

To produce an integer in `[0, n)` for `n > 0` and `n <= 2^32`, use **rejection sampling**:

```
bounded(n):
    threshold = (2^64) mod n           // wrap-around region
    loop:
        v = next()
        if v >= threshold: return v mod n
```

This is required (not just `next() mod n`) so that all implementations produce identical sequences for identical seeds.

### 4.4 Shuffle

Fisher-Yates from the end, using `bounded`:

```
shuffle(array):
    for i from len(array)-1 down to 1:
        j = bounded(i + 1)
        swap array[i] and array[j]
```

---

## 5. Brisca: rules

### 5.1 Setup

1. Build the ordered 40-card deck (§3.4).
2. Shuffle with the seeded PRNG (§4.4).
3. Deal **3 cards** to the human, then **3 cards** to the AI, taking from the top of the deck (index 0).
4. Flip the next card from the deck — this is the **trump card** (`pinta`). Its suit is the trump suit for the entire game.
5. The trump card is placed at the **bottom** of the remaining deck, visible until it is the last card drawn.
6. The human plays first in trick 1.

### 5.2 Trick play

A trick consists of two cards: the **leader's** card, then the **follower's** card.

- The leader plays any card from their hand. No suit-following obligation.
- The follower plays any card from their hand. No suit-following obligation.

### 5.3 Trick winner

Let `lead_suit` be the suit of the leader's card, `trump_suit` be the trump suit.

- If exactly one of the two cards is trump, that card wins.
- If both cards are trump, the one with higher **strength** (§3.2) wins.
- If neither card is trump, and both cards share `lead_suit`, the one with higher strength wins.
- If neither card is trump and the follower played off-suit (not `lead_suit`), the leader wins regardless.

### 5.4 Drawing

After each trick, **the winner draws first**, then the loser. Each draws one card from the top of the deck. The trump card is the last card to be drawn (it goes to whoever draws when only the trump remains).

If the deck is empty, no cards are drawn. Players continue until hands are exhausted.

### 5.5 Next leader

The winner of trick N leads trick N+1.

### 5.6 Game end

The game ends when both hands are empty (after 20 tricks, since 40 cards / 2 players / 2 cards per trick = 20).

### 5.7 Scoring

Each player sums the **points** (§3.2) of all cards they won across all tricks. The deck contains 120 points total.

- Player with > 60 points wins.
- Player with exactly 60 points draws.
- Player with < 60 points loses.

### 5.8 Output of final result

The CLI MUST print the final scores on the final line in this exact format:

```
RESULT human=<int> ai=<int> outcome=<win|loss|draw>
```

Where `outcome` is from the human's perspective. This line is parsed by test vectors.

---

## 6. AI strategy

### 6.1 Easy

Always play the lowest-strength card in hand. Ties broken by deck order (lower rank index first, then suit order `O,C,E,B`).

### 6.2 Normal (default)

When **leading**:
- If hand has a non-trump rank-0-points card, lead the lowest-strength such card.
- Else lead the lowest-strength non-trump card.
- Else lead the lowest-strength trump card.

When **following**:
- Compute, for each card in hand, whether it would win the trick (§5.3).
- If any non-trump card wins: play the lowest-strength winning non-trump.
- Else if the leader's card has points (>= 10) and a trump would win: play the lowest-strength winning trump.
- Else: play the lowest-strength non-trump if any, else the lowest-strength trump.

### 6.3 Hard

Same as Normal, but with one addition: track which high cards (As, 3) have been played for each suit. When leading, prefer suits where the opponent's likely highest remaining card is lower than your own.

(Detailed pseudocode for Hard is in `spec/AI-HARD.md` — implementations MAY ship without Hard in v0.1.0 and add it in v0.2.0. Easy and Normal are required for v0.1.0.)

### 6.4 Tie-breaking determinism

When multiple cards tie under any rule above, the tie is broken by **hand position** (lowest index first). Hand index is defined by insertion order: cards dealt earlier sit at lower indices, and newly drawn cards are appended to the end.

---

## 7. CLI session protocol (brisca)

### 7.1 Render after every state change

Before each human prompt, the CLI MUST render the state in this format (UTF-8, ANSI codes only if colour enabled):

```
═══════════════════════════════════════
 AI: <n> cards              Deck: <n>
 Trump: <card>
───────────────────────────────────────
 Table: <leader_card_or_blank>
───────────────────────────────────────
 Your hand:
   1:<card>  2:<card>  3:<card>
═══════════════════════════════════════
> 
```

The number of cards in the hand decreases as the deck empties. When the deck is empty, the `Deck: 0` line still appears but the trump line shows `Trump: <card> (last)` if the trump card is still in play, or disappears once drawn (replaced by `Trump: <suit> (suit)` to keep the trump suit visible).

### 7.2 Input

The prompt accepts:
- A digit `1`, `2`, or `3`: play that card from hand (1-indexed).
- `q` or `quit`: abandon game, exit 0.
- `?` or `help`: print short command reminder.
- Anything else: print `Invalid input. Use 1-3, q, or ?.` and reprompt.

### 7.3 Trick resolution display

After both players have played, the CLI MUST print:

```
 AI plays: <card>
 Winner: <you|AI> (+<points> points)
```

Then pause until the human presses Enter (skipped if `--quiet`).

### 7.4 Final display

After the last trick:

```
═══════════════════════════════════════
 Game over
 Your score: <int>
 AI score:   <int>
 <You won | You lost | Draw>
═══════════════════════════════════════
RESULT human=<int> ai=<int> outcome=<win|loss|draw>
```

The final `RESULT` line is the machine-readable contract for test vectors.

---

## 8. Test vectors

### 8.1 Format

`spec/vectors/v<NNN>.json`:

```json
{
  "id": "v001",
  "seed": 1,
  "ai_level": "normal",
  "human_plays": [1, 1, 2, 1, 3, ...],
  "expected_result": {
    "human": 47,
    "ai": 73,
    "outcome": "loss"
  },
  "expected_trick_log": [
    {"trick": 1, "leader": "human", "human_card": "AsO", "ai_card": "2O", "winner": "human", "points": 11},
    ...
  ]
}
```

### 8.2 Conformance

An implementation is conformant for v0.1.0 when:
- All 20 vectors in `spec/vectors/v001.json`–`v020.json` produce the exact `expected_result`.
- All trick-by-trick events in `expected_trick_log` match exactly.

Vectors are generated by the Python reference implementation and committed to the repo. All other implementations MUST replay them identically.

---

## 9. Versioning

The spec follows semver. v0.1.0 covers brisca with easy and normal AI. Incompatible rule or PRNG changes bump the major version. The CLI surface (commands, flags, exit codes) is also covered by semver.

---

## 10. Non-goals for v0.1.0

- Multiplayer over network.
- Save/load mid-game.
- Hard AI (deferred to v0.2.0).
- Other games (tute, mus). The umbrella structure must permit them, but they are not implemented.
- Internationalisation. All user-facing strings are in English.
