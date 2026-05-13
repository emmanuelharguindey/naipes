//! Spanish 40-card deck primitives shared across naipes games.
//! See spec/SPEC.md §3.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Oros,
    Copas,
    Espadas,
    Bastos,
}

impl Suit {
    pub fn code(&self) -> char {
        match self {
            Suit::Oros => 'O',
            Suit::Copas => 'C',
            Suit::Espadas => 'E',
            Suit::Bastos => 'B',
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Suit::Oros => "Oros",
            Suit::Copas => "Copas",
            Suit::Espadas => "Espadas",
            Suit::Bastos => "Bastos",
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.code()))
    }
}

/// Canonical suit ordering. SPEC.md §3.4.
pub const SUITS: [Suit; 4] = [Suit::Oros, Suit::Copas, Suit::Espadas, Suit::Bastos];

/// Canonical rank ordering. 8 and 9 do not exist in the Spanish deck.
pub const RANKS: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 10, 11, 12];

/// A single Spanish playing card. Trivially copyable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub rank: u8,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: u8, suit: Suit) -> Self {
        debug_assert!(
            RANKS.contains(&rank),
            "invalid rank {rank}: must be one of {RANKS:?}"
        );
        Self { rank, suit }
    }

    /// Canonical short form, e.g. "AsO", "3E", "RC".
    pub fn label(&self) -> String {
        let rank_label = match self.rank {
            1 => "As",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            10 => "S",   // Sota
            11 => "C",   // Caballo
            12 => "R",   // Rey
            _ => unreachable!("invalid rank reached label()"),
        };
        format!("{}{}", rank_label, self.suit.code())
    }

    /// Human-readable Spanish name, e.g. "As de Oros".
    pub fn long_name(&self) -> String {
        let rank_name = match self.rank {
            1 => "As",
            2 => "Dos",
            3 => "Tres",
            4 => "Cuatro",
            5 => "Cinco",
            6 => "Seis",
            7 => "Siete",
            10 => "Sota",
            11 => "Caballo",
            12 => "Rey",
            _ => unreachable!("invalid rank reached long_name()"),
        };
        format!("{} de {}", rank_name, self.suit.name())
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.label())
    }
}

/// The canonical unshuffled 40-card deck. See SPEC.md §3.4.
pub fn build_ordered_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(40);
    for suit in SUITS {
        for rank in RANKS {
            deck.push(Card::new(rank, suit));
        }
    }
    deck
}
