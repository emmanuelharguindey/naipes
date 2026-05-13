# naipes

Traditional card games to play in the terminal. Version 0.1.0 includes **brisca**. Future versions will add tute, mus, and more.

## Installation

```bash
pip install naipes
```

Requires Python 3.10 or later.

## Usage

```bash
naipes                       # help
naipes list                  # list available games
naipes rules brisca          # rules summary
naipes play brisca           # play against the AI
naipes play brisca --seed 42 --ai-level easy
```

During play: `1`, `2`, or `3` to play a card from your hand; `q` to quit; `?` for help.

## Reproducible games

```bash
naipes play brisca --seed 42 --ai-level normal
```

This game always starts exactly the same. Useful for sharing a specific game or replaying the same scenario.

## License

MIT © Manuel Harguindey
