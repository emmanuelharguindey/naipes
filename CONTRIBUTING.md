# Contributing to naipes

## Reporting bugs

Open an issue with:

- Exact command you ran
- Full output (copy and paste)
- Version: output of `naipes --version`
- For reproducibility bugs: the exact `--seed`

## Proposing a new game

`naipes` aims to cover the classic card games of various traditions: brisca, tute, mus, chinchón, julepe, and so on. If you want to add one:

1. Read [`spec/SPEC.md`](spec/SPEC.md) to understand how the project is organised.
2. Open an issue describing the exact rules of the game you propose, card values, and any regional variants you want to cover.
3. The game's spec is discussed first in the issue before any implementation. Rules must be unambiguous (cover every case: tricks, hands, etc.).
4. Once the spec is closed, it's implemented in at least one of the existing ports (Python, TypeScript, or Rust) and corresponding test vectors are generated.

## Adding a port in another language

The current ports (Python, TypeScript, Rust) are independent but all conform to the same spec. If you want to add one in another language:

1. Create `ports/<language>/` with the idiomatic structure of the language.
2. Implement the spec: `xorshift64*` PRNG, deck, game rules, AI levels, CLI with the subcommands `list`, `rules`, `play` and the standard flags.
3. Load `spec/vectors/v001.json`–`v020.json` and verify that your implementation reproduces each one trick by trick.
4. **Your implementation is conformant when it passes all 20 vectors exactly.** If any fails, it's usually a divergence from the spec — fix the implementation, not the vectors. If you find a real ambiguity in the spec, propose a spec correction in the issue.
5. Add a workflow in `.github/workflows/` that runs the new language's tests.

## Contributing to an existing port

- Follow the language's conventions (format with the standard tool, tests with the usual framework).
- If your change affects game logic, the test vectors must continue to pass.
- Changes that require modifying the test vectors imply modifying the spec — propose them first in an issue.

## Code of conduct

Be respectful. Discussions about design decisions are welcome, personal attacks are not.
