//! Brisca game state and transitions. Pure state machine, no I/O.
//! See SPEC.md §5.

use crate::core::{build_ordered_deck, Card, Rng, Suit};
use crate::games::brisca::rules::{card_points, trick_points, trick_winner_is_follower};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Human,
    Ai,
}

impl Player {
    pub fn other(self) -> Self {
        match self {
            Player::Human => Player::Ai,
            Player::Ai => Player::Human,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Player::Human => "human",
            Player::Ai => "ai",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrickRecord {
    pub trick_number: u8,
    pub leader: Player,
    pub human_card: Card,
    pub ai_card: Card,
    pub winner: Player,
    pub points: u8,
}

#[derive(Debug)]
pub struct GameState {
    pub rng: Rng,
    pub trump_suit: Suit,
    pub trump_card: Card,
    /// deck[deck.len()-1] is the trump card until drawn. Draw from index 0.
    pub deck: Vec<Card>,
    pub human_hand: Vec<Card>,
    pub ai_hand: Vec<Card>,
    pub human_captured: Vec<Card>,
    pub ai_captured: Vec<Card>,
    pub leader: Player,
    pub pending_leader_card: Option<Card>,
    pub pending_leader: Option<Player>,
    pub trick_log: Vec<TrickRecord>,
}

impl GameState {
    /// Initial state after dealing. Human leads trick 1. SPEC.md §5.1.
    pub fn new(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck = build_ordered_deck();
        rng.shuffle(&mut deck);

        // Deal 3 to human, then 3 to AI, taking from the top (index 0).
        let mut human_hand = Vec::with_capacity(3);
        for _ in 0..3 {
            human_hand.push(deck.remove(0));
        }
        let mut ai_hand = Vec::with_capacity(3);
        for _ in 0..3 {
            ai_hand.push(deck.remove(0));
        }
        // Flip the trump card; place at the bottom of the remaining deck.
        let trump_card = deck.remove(0);
        let trump_suit = trump_card.suit;
        deck.push(trump_card); // bottom == end of Vec (drawn last)

        Self {
            rng,
            trump_suit,
            trump_card,
            deck,
            human_hand,
            ai_hand,
            human_captured: Vec::new(),
            ai_captured: Vec::new(),
            leader: Player::Human,
            pending_leader_card: None,
            pending_leader: None,
            trick_log: Vec::new(),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.human_hand.is_empty()
            && self.ai_hand.is_empty()
            && self.pending_leader_card.is_none()
    }

    pub fn hand_of(&self, player: Player) -> &Vec<Card> {
        match player {
            Player::Human => &self.human_hand,
            Player::Ai => &self.ai_hand,
        }
    }

    pub fn captured_of(&self, player: Player) -> &Vec<Card> {
        match player {
            Player::Human => &self.human_captured,
            Player::Ai => &self.ai_captured,
        }
    }

    pub fn score_of(&self, player: Player) -> u32 {
        self.captured_of(player)
            .iter()
            .map(|c| card_points(*c) as u32)
            .sum()
    }

    /// Play card at `hand_index` for `player`. Returns Some(TrickRecord) when
    /// the trick completes (i.e. when the follower plays).
    ///
    /// Caller is responsible for ordering (leader first, then follower).
    /// Out-of-turn plays panic.
    pub fn play_card(&mut self, player: Player, hand_index: usize) -> Option<TrickRecord> {
        if self.pending_leader_card.is_none() {
            // Leader's play.
            assert_eq!(player, self.leader, "out-of-turn: leader is {:?}", self.leader);
            let hand = match player {
                Player::Human => &mut self.human_hand,
                Player::Ai => &mut self.ai_hand,
            };
            assert!(hand_index < hand.len(), "hand_index {hand_index} out of range");
            let card = hand.remove(hand_index);
            self.pending_leader_card = Some(card);
            self.pending_leader = Some(player);
            return None;
        }

        // Follower's play.
        let leader_player = self.pending_leader.expect("pending_leader set");
        let expected_follower = leader_player.other();
        assert_eq!(
            player, expected_follower,
            "out-of-turn: follower must be {expected_follower:?}"
        );
        let hand = match player {
            Player::Human => &mut self.human_hand,
            Player::Ai => &mut self.ai_hand,
        };
        assert!(hand_index < hand.len(), "hand_index {hand_index} out of range");
        let follower_card = hand.remove(hand_index);
        let leader_card = self.pending_leader_card.expect("pending_leader_card set");

        let follower_wins = trick_winner_is_follower(leader_card, follower_card, self.trump_suit);
        let winner = if follower_wins {
            expected_follower
        } else {
            leader_player
        };
        let points = trick_points(leader_card, follower_card);

        let (human_card, ai_card) = match leader_player {
            Player::Human => (leader_card, follower_card),
            Player::Ai => (follower_card, leader_card),
        };

        let record = TrickRecord {
            trick_number: (self.trick_log.len() + 1) as u8,
            leader: leader_player,
            human_card,
            ai_card,
            winner,
            points,
        };
        self.trick_log.push(record.clone());

        // Move both played cards to winner's captured pile.
        match winner {
            Player::Human => {
                self.human_captured.push(leader_card);
                self.human_captured.push(follower_card);
            }
            Player::Ai => {
                self.ai_captured.push(leader_card);
                self.ai_captured.push(follower_card);
            }
        }

        // Clear pending.
        self.pending_leader_card = None;
        self.pending_leader = None;

        // Draw phase: winner first, then loser. SPEC.md §5.4.
        let loser = winner.other();
        self.draw_one(winner);
        self.draw_one(loser);

        // Winner leads next trick. SPEC.md §5.5.
        self.leader = winner;

        Some(record)
    }

    fn draw_one(&mut self, player: Player) {
        if self.deck.is_empty() {
            return;
        }
        let card = self.deck.remove(0);
        match player {
            Player::Human => self.human_hand.push(card),
            Player::Ai => self.ai_hand.push(card),
        }
    }
}
