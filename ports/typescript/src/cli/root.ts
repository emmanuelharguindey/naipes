/**
 * Root CLI dispatcher: `naipes [subcommand] ...`. See SPEC.md §2.
 */

import { AVAILABLE_GAMES } from "../games/index.js";
import { playInteractive } from "./loop.js";
import type { AiLevel } from "../games/brisca/ai.js";

const VERSION = "0.1.0";

const HELP_TEXT = `\
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
`;

const BRISCA_RULES = `\
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
`;

function randomSeed(): bigint {
  // 64 random bits via crypto. SPEC.md doesn't constrain how to source it.
  const bytes = new Uint8Array(8);
  crypto.getRandomValues(bytes);
  let v = 0n;
  for (const b of bytes) {
    v = (v << 8n) | BigInt(b);
  }
  return v;
}

interface ParsedArgs {
  command: string | null;
  game: string | null;
  seed: bigint | null;
  aiLevel: AiLevel;
  noColor: boolean;
  quiet: boolean;
  help: boolean;
  version: boolean;
}

function parseArgs(argv: readonly string[]): ParsedArgs | { error: string } {
  const out: ParsedArgs = {
    command: null,
    game: null,
    seed: null,
    aiLevel: "normal",
    noColor: false,
    quiet: false,
    help: false,
    version: false,
  };

  const positional: string[] = [];
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i]!;
    if (a === "--help" || a === "-h") out.help = true;
    else if (a === "--version") out.version = true;
    else if (a === "--no-color") out.noColor = true;
    else if (a === "--quiet") out.quiet = true;
    else if (a === "--seed") {
      const v = argv[++i];
      if (v === undefined) return { error: "--seed requires a value" };
      try {
        // Accept decimal, 0x... hex, 0o... oct, 0b... bin
        out.seed = BigInt(v);
      } catch {
        return { error: `--seed: invalid number ${JSON.stringify(v)}` };
      }
    } else if (a === "--ai-level") {
      const v = argv[++i];
      if (v !== "easy" && v !== "normal" && v !== "hard") {
        return { error: `--ai-level: must be easy|normal|hard, got ${v}` };
      }
      out.aiLevel = v;
    } else if (a.startsWith("--")) {
      return { error: `unknown option: ${a}` };
    } else {
      positional.push(a);
    }
  }

  if (positional.length > 0) out.command = positional[0]!;
  if (positional.length > 1) out.game = positional[1]!;

  return out;
}

export async function main(argv: readonly string[] = process.argv.slice(2)): Promise<number> {
  const parsed = parseArgs(argv);
  if ("error" in parsed) {
    process.stderr.write(`naipes: ${parsed.error}\n`);
    return 2;
  }

  if (parsed.help || (parsed.command === null && !parsed.version)) {
    process.stdout.write(HELP_TEXT);
    return 0;
  }

  if (parsed.version) {
    process.stdout.write(`naipes ${VERSION}\n`);
    return 0;
  }

  if (parsed.command === "list") {
    for (const g of AVAILABLE_GAMES) process.stdout.write(g + "\n");
    return 0;
  }

  if (parsed.command === "rules") {
    if (parsed.game === null) {
      process.stderr.write(
        "naipes: 'rules' requires a game name. Run 'naipes list'.\n",
      );
      return 2;
    }
    if (parsed.game === "brisca") {
      process.stdout.write(BRISCA_RULES);
      return 0;
    }
    process.stderr.write(
      `naipes: unknown game '${parsed.game}'. Run 'naipes list' to see available games.\n`,
    );
    return 2;
  }

  if (parsed.command === "play") {
    if (parsed.game === null) {
      process.stderr.write(
        "naipes: 'play' requires a game name. Run 'naipes list'.\n",
      );
      return 2;
    }
    if (parsed.game !== "brisca") {
      process.stderr.write(
        `naipes: unknown game '${parsed.game}'. Run 'naipes list' to see available games.\n`,
      );
      return 2;
    }
    const seed = parsed.seed ?? randomSeed();
    return await playInteractive(seed, parsed.aiLevel, { quiet: parsed.quiet });
  }

  process.stderr.write(
    `naipes: unknown command '${parsed.command}'. Run 'naipes --help'.\n`,
  );
  return 2;
}
