//! Root CLI dispatcher: `naipes [subcommand] ...`. See SPEC.md §2.

use std::io::{self, Write};

use crate::cli::loop_play::{play_interactive, PlayOptions};
use crate::games::brisca::ai::AiLevel;
use crate::games::AVAILABLE_GAMES;

const VERSION: &str = "0.1.0";

const HELP_TEXT: &str = "\
naipes — traditional card games on the command line

Usage:
  naipes                       show this help
  naipes --version             show version
  naipes list                  list available games
  naipes rules <game>          show the rules summary for <game>
  naipes play <game> [options]

Options for 'play':
  --seed <u64>                 deterministic PRNG seed
  --ai-level easy|normal|hard  AI level (default: normal)
  --no-color                   disable ANSI colors
  --quiet                      suppress pauses and non-essential messages

Examples:
  naipes play brisca
  naipes play brisca --seed 42 --ai-level easy
";

const BRISCA_RULES: &str = "\
Brisca — rules summary

· Spanish 40-card deck. Two players. 3 cards in hand.
· One card is flipped from the deck: its suit becomes trump.
· Each trick: the leader plays a card, the follower responds.
· The trick is won by the highest card of the lead suit, unless
  someone played a trump: trump beats the lead suit.
· After each trick the winner draws first, then the loser.
· The winner of a trick leads the next one.
· Card values: As=11, Tres=10, Rey=4, Caballo=3, Sota=2.
  The rest score 0 but are useful for defending.
· Total: 120 points in the deck. The player who exceeds 60 wins.

In-game commands: 1, 2, or 3 to play a card from hand;
q to quit; ? for help.
";

#[derive(Debug)]
struct ParsedArgs {
    command: Option<String>,
    game: Option<String>,
    seed: Option<u64>,
    ai_level: AiLevel,
    no_color: bool,
    quiet: bool,
    help: bool,
    version: bool,
}

fn parse_seed(s: &str) -> Result<u64, String> {
    let trimmed = s.trim();
    let (rest, radix) = if let Some(r) = trimmed.strip_prefix("0x").or_else(|| trimmed.strip_prefix("0X")) {
        (r, 16)
    } else if let Some(r) = trimmed.strip_prefix("0o").or_else(|| trimmed.strip_prefix("0O")) {
        (r, 8)
    } else if let Some(r) = trimmed.strip_prefix("0b").or_else(|| trimmed.strip_prefix("0B")) {
        (r, 2)
    } else {
        (trimmed, 10)
    };
    u64::from_str_radix(rest, radix).map_err(|e| format!("invalid number {trimmed:?}: {e}"))
}

fn parse_args(argv: &[String]) -> Result<ParsedArgs, String> {
    let mut out = ParsedArgs {
        command: None,
        game: None,
        seed: None,
        ai_level: AiLevel::Normal,
        no_color: false,
        quiet: false,
        help: false,
        version: false,
    };
    let mut positional: Vec<String> = Vec::new();
    let mut i = 0;
    while i < argv.len() {
        let a = &argv[i];
        match a.as_str() {
            "--help" | "-h" => out.help = true,
            "--version" => out.version = true,
            "--no-color" => out.no_color = true,
            "--quiet" => out.quiet = true,
            "--seed" => {
                i += 1;
                let v = argv.get(i).ok_or_else(|| "--seed requires a value".to_string())?;
                out.seed = Some(parse_seed(v)?);
            }
            "--ai-level" => {
                i += 1;
                let v = argv.get(i).ok_or_else(|| "--ai-level requires a value".to_string())?;
                out.ai_level = AiLevel::parse(v).ok_or_else(|| {
                    format!("--ai-level: must be easy|normal|hard, got {v:?}")
                })?;
            }
            s if s.starts_with("--") => return Err(format!("unknown option: {s}")),
            _ => positional.push(a.clone()),
        }
        i += 1;
    }
    if !positional.is_empty() {
        out.command = Some(positional[0].clone());
    }
    if positional.len() > 1 {
        out.game = Some(positional[1].clone());
    }
    Ok(out)
}

pub fn main(argv: Vec<String>) -> i32 {
    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut out = stdout.lock();
    let mut err = stderr.lock();

    let parsed = match parse_args(&argv) {
        Ok(p) => p,
        Err(e) => {
            let _ = writeln!(err, "naipes: {e}");
            return 2;
        }
    };

    if parsed.help || (parsed.command.is_none() && !parsed.version) {
        let _ = write!(out, "{HELP_TEXT}");
        return 0;
    }

    if parsed.version {
        let _ = writeln!(out, "naipes {VERSION}");
        return 0;
    }

    let cmd = parsed.command.as_deref().unwrap_or("");
    match cmd {
        "list" => {
            for g in AVAILABLE_GAMES {
                let _ = writeln!(out, "{g}");
            }
            0
        }
        "rules" => {
            let Some(game) = parsed.game.as_deref() else {
                let _ = writeln!(err, "naipes: 'rules' requires a game name. Run 'naipes list'.");
                return 2;
            };
            if game == "brisca" {
                let _ = write!(out, "{BRISCA_RULES}");
                return 0;
            }
            let _ = writeln!(
                err,
                "naipes: unknown game '{game}'. Run 'naipes list' to see available games."
            );
            2
        }
        "play" => {
            let Some(game) = parsed.game.as_deref() else {
                let _ = writeln!(err, "naipes: 'play' requires a game name. Run 'naipes list'.");
                return 2;
            };
            if game != "brisca" {
                let _ = writeln!(
                    err,
                    "naipes: unknown game '{game}'. Run 'naipes list' to see available games."
                );
                return 2;
            }
            let seed = parsed.seed.unwrap_or_else(random_seed);
            // Drop the locks so play_interactive can take its own.
            drop(out);
            drop(err);
            play_interactive(seed, parsed.ai_level, PlayOptions { quiet: parsed.quiet })
        }
        other => {
            let _ = writeln!(err, "naipes: unknown command '{other}'. Run 'naipes --help'.");
            2
        }
    }
}

fn random_seed() -> u64 {
    // SPEC.md doesn't mandate a specific source. We use the OS time-of-day
    // and process id mixed together. Determinism is opt-in via --seed.
    use std::process;
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(1);
    let pid = process::id() as u64;
    // Mix with a simple xor-multiply.
    now.wrapping_mul(0x9E3779B97F4A7C15).wrapping_add(pid)
}
