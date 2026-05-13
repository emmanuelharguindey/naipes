"""Root CLI dispatcher: `naipes [subcommand] ...`. See SPEC.md §2."""

from __future__ import annotations

import argparse
import secrets
import sys
from importlib import metadata

from naipes.cli.loop import play_interactive
from naipes.games import AVAILABLE_GAMES


HELP_TEXT = """\
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
"""


BRISCA_RULES = """\
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
"""


def _get_version() -> str:
    try:
        return metadata.version("naipes")
    except metadata.PackageNotFoundError:
        return "0.1.0+dev"


def _random_seed() -> int:
    return secrets.randbits(64)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="naipes",
        add_help=False,
        description="Spanish card games CLI",
    )
    parser.add_argument("--version", action="store_true")
    parser.add_argument("--help", "-h", action="store_true", dest="help")
    subparsers = parser.add_subparsers(dest="command")

    subparsers.add_parser("list", add_help=False)

    rules_p = subparsers.add_parser("rules", add_help=False)
    rules_p.add_argument("game", nargs="?")

    play_p = subparsers.add_parser("play", add_help=False)
    play_p.add_argument("game", nargs="?")
    play_p.add_argument("--seed", type=lambda s: int(s, 0))
    play_p.add_argument(
        "--ai-level",
        choices=("easy", "normal", "hard"),
        default="normal",
        dest="ai_level",
    )
    play_p.add_argument("--no-color", action="store_true", dest="no_color")
    play_p.add_argument("--quiet", action="store_true")

    return parser


def main(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(argv)

    if args.help or (args.command is None and not args.version):
        print(HELP_TEXT)
        return 0

    if args.version:
        print(f"naipes {_get_version()}")
        return 0

    if args.command == "list":
        for game in AVAILABLE_GAMES:
            print(game)
        return 0

    if args.command == "rules":
        if args.game is None:
            print(
                "naipes: 'rules' requires a game name. Run 'naipes list'.",
                file=sys.stderr,
            )
            return 2
        if args.game == "brisca":
            print(BRISCA_RULES)
            return 0
        print(
            f"naipes: unknown game {args.game!r}. Run 'naipes list' to see available games.",
            file=sys.stderr,
        )
        return 2

    if args.command == "play":
        if args.game is None:
            print(
                "naipes: 'play' requires a game name. Run 'naipes list'.",
                file=sys.stderr,
            )
            return 2
        if args.game != "brisca":
            print(
                f"naipes: unknown game {args.game!r}. Run 'naipes list' to see available games.",
                file=sys.stderr,
            )
            return 2
        seed = args.seed if args.seed is not None else _random_seed()
        try:
            return play_interactive(
                seed=seed,
                ai_level=args.ai_level,
                quiet=args.quiet,
            )
        except KeyboardInterrupt:
            print()
            return 130

    return 0
