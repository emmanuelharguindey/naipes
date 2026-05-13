# naipes

Traditional card games to play in the terminal.

```bash
naipes play brisca
```

Available today: **brisca**. Coming soon: tute, mus, chinchón.

## Installation

```bash
pip install naipes                       # Python
```

More installation options will be added over time. The gameplay experience is identical in all of them.

## How to play

```bash
naipes                       # general help
naipes list                  # list available games
naipes rules brisca          # rules summary
naipes play brisca           # start a game against the AI
naipes play brisca --ai-level easy
```

During play: `1`, `2`, or `3` to play a card from your hand; `q` to quit; `?` for help.

## Reproducible games

`naipes play brisca --seed 42` always starts the same game. Useful for sharing a specific game with someone or replaying the same scenario.

## About the project

`naipes` is built around a single canonical specification ([`spec/SPEC.md`](spec/SPEC.md)) that defines the rules, card values, trick logic, and CLI behaviour.

## Repository structure

```
naipes/
├── spec/                      # rules and reference games
├── ports/
│   └── python/                # Python implementation
├── README.md
└── ...
```

## Contributing

To add a new game (tute, mus, ...) or a new implementation in another language: see [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT © Manuel Harguindey
