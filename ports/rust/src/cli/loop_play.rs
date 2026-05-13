//! Brisca interactive loop and headless replay driver.

use std::io::{self, BufRead, Write};

use crate::cli::render::{render_board, render_final, render_trick_result};
use crate::games::brisca::ai::{choose as choose_ai_move, AiLevel};
use crate::games::brisca::state::{GameState, Player, TrickRecord};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Outcome::Win => "win",
            Outcome::Loss => "loss",
            Outcome::Draw => "draw",
        }
    }

    pub fn for_score(human: u32) -> Self {
        if human > 60 {
            Outcome::Win
        } else if human < 60 {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
}

#[derive(Debug)]
pub struct ReplayResult {
    pub human_score: u32,
    pub ai_score: u32,
    pub outcome: Outcome,
    pub trick_log: Vec<TrickRecord>,
}

/// Run a brisca game from a scripted sequence of human plays.
///
/// `human_plays` is a list of 1-indexed hand positions the human picks.
pub fn replay(seed: u64, ai_level: AiLevel, human_plays: &[u8]) -> ReplayResult {
    let mut state = GameState::new(seed);
    let mut play_idx = 0usize;

    while !state.is_finished() {
        match state.leader {
            Player::Human => {
                let idx = human_plays[play_idx] as usize - 1;
                play_idx += 1;
                state.play_card(Player::Human, idx);
                let ai_idx = choose_ai_move(&state, ai_level);
                state.play_card(Player::Ai, ai_idx);
            }
            Player::Ai => {
                let ai_idx = choose_ai_move(&state, ai_level);
                state.play_card(Player::Ai, ai_idx);
                let idx = human_plays[play_idx] as usize - 1;
                play_idx += 1;
                state.play_card(Player::Human, idx);
            }
        }
    }

    ReplayResult {
        human_score: state.score_of(Player::Human),
        ai_score: state.score_of(Player::Ai),
        outcome: Outcome::for_score(state.score_of(Player::Human)),
        trick_log: state.trick_log,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlayOptions {
    pub quiet: bool,
}

/// Run a brisca game with stdin/stdout. Returns process exit code.
pub fn play_interactive(seed: u64, ai_level: AiLevel, opts: PlayOptions) -> i32 {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stdout = io::stdout();
    let mut writer = stdout.lock();
    let mut state = GameState::new(seed);

    let mut line_buf = String::new();

    let read_line = |writer: &mut io::StdoutLock<'_>,
                          reader: &mut io::StdinLock<'_>,
                          prompt: &str,
                          line: &mut String|
     -> Option<String> {
        write!(writer, "{prompt}").ok()?;
        writer.flush().ok()?;
        line.clear();
        match reader.read_line(line) {
            Ok(0) => None, // EOF
            Ok(_) => Some(line.trim().to_string()),
            Err(_) => None,
        }
    };

    // Inner helper: render board and ask for a 1-indexed hand position.
    // Returns None if the user quits.
    fn prompt_human(
        state: &GameState,
        writer: &mut io::StdoutLock<'_>,
        reader: &mut io::StdinLock<'_>,
        line_buf: &mut String,
    ) -> Option<u8> {
        loop {
            writeln!(writer, "{}", render_board(state)).ok()?;
            write!(writer, "> ").ok()?;
            writer.flush().ok()?;
            line_buf.clear();
            let n = reader.read_line(line_buf).ok()?;
            if n == 0 {
                return None; // EOF
            }
            let raw = line_buf.trim().to_lowercase();
            if raw == "q" || raw == "quit" {
                return None;
            }
            if raw == "?" || raw == "help" {
                writeln!(writer, "Commands: 1-3 (play card), q (quit), ? (help)").ok()?;
                continue;
            }
            if raw == "1" || raw == "2" || raw == "3" {
                let idx1: u8 = raw.parse().expect("digit parsed");
                if (idx1 as usize) > state.human_hand.len() {
                    writeln!(writer, "You only have {} cards.", state.human_hand.len()).ok()?;
                    continue;
                }
                return Some(idx1);
            }
            writeln!(writer, "Invalid input. Use 1-3, q, or ?.").ok()?;
        }
    }

    while !state.is_finished() {
        let human_idx_1 = match state.leader {
            Player::Human => match prompt_human(&state, &mut writer, &mut reader, &mut line_buf) {
                Some(v) => {
                    state.play_card(Player::Human, (v - 1) as usize);
                    let ai_idx = choose_ai_move(&state, ai_level);
                    state.play_card(Player::Ai, ai_idx);
                    v
                }
                None => {
                    writeln!(writer, "Game abandoned.").ok();
                    return 0;
                }
            },
            Player::Ai => {
                let ai_idx = choose_ai_move(&state, ai_level);
                state.play_card(Player::Ai, ai_idx);
                match prompt_human(&state, &mut writer, &mut reader, &mut line_buf) {
                    Some(v) => {
                        state.play_card(Player::Human, (v - 1) as usize);
                        v
                    }
                    None => {
                        writeln!(writer, "Game abandoned.").ok();
                        return 0;
                    }
                }
            }
        };
        let _ = human_idx_1; // not needed beyond control flow

        writeln!(writer, "{}", render_trick_result(&state)).ok();

        if !opts.quiet && !state.is_finished() {
            let _ = read_line(&mut writer, &mut reader, "(Press Enter to continue) ", &mut line_buf);
        }
    }

    writeln!(writer, "{}", render_final(&state)).ok();
    0
}
