/**
 * Spanish 40-card deck primitives shared across naipes games.
 * See spec/SPEC.md §3.
 */

export type Suit = "O" | "C" | "E" | "B";

export const SUITS: readonly Suit[] = ["O", "C", "E", "B"];

export const SUIT_NAMES: Record<Suit, string> = {
  O: "Oros",
  C: "Copas",
  E: "Espadas",
  B: "Bastos",
};

/** Canonical rank ordering. 8 and 9 do not exist in the Spanish deck. */
export const RANKS: readonly number[] = [1, 2, 3, 4, 5, 6, 7, 10, 11, 12];

export const RANK_LABELS: Record<number, string> = {
  1: "As",
  2: "2",
  3: "3",
  4: "4",
  5: "5",
  6: "6",
  7: "7",
  10: "S",   // Sota
  11: "C",   // Caballo
  12: "R",   // Rey
};

export const RANK_NAMES: Record<number, string> = {
  1: "As",
  2: "Dos",
  3: "Tres",
  4: "Cuatro",
  5: "Cinco",
  6: "Seis",
  7: "Siete",
  10: "Sota",
  11: "Caballo",
  12: "Rey",
};

const VALID_RANKS = new Set(RANKS);
const VALID_SUITS = new Set<string>(SUITS);

/**
 * A single Spanish playing card. Frozen at construction (effectively immutable).
 */
export class Card {
  readonly rank: number;
  readonly suit: Suit;

  constructor(rank: number, suit: Suit) {
    if (!VALID_RANKS.has(rank)) {
      throw new RangeError(`invalid rank ${rank}`);
    }
    if (!VALID_SUITS.has(suit)) {
      throw new RangeError(`invalid suit ${suit}`);
    }
    this.rank = rank;
    this.suit = suit;
    Object.freeze(this);
  }

  /** Canonical short form, e.g. 'AsO', '3E', 'RC'. */
  get label(): string {
    return `${RANK_LABELS[this.rank]}${this.suit}`;
  }

  /** Human-readable Spanish name, e.g. 'As de Oros'. */
  get longName(): string {
    return `${RANK_NAMES[this.rank]} de ${SUIT_NAMES[this.suit]}`;
  }

  toString(): string {
    return this.label;
  }

  equals(other: Card): boolean {
    return this.rank === other.rank && this.suit === other.suit;
  }
}

/** The canonical unshuffled 40-card deck. See SPEC.md §3.4. */
export function buildOrderedDeck(): Card[] {
  const deck: Card[] = [];
  for (const s of SUITS) {
    for (const r of RANKS) {
      deck.push(new Card(r, s));
    }
  }
  return deck;
}
