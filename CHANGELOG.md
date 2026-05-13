# Changelog

All notable changes to this project will be documented here. Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versioning: [SemVer](https://semver.org/).

Each port versions independently. The spec has its own version line.

## Spec

### [0.1.0] — 2026-05-12
- Initial canonical spec for `naipes`.
- 40-card Spanish deck primitives.
- `xorshift64*` PRNG with rejection sampling, for reproducible games.
- Brisca rules: deal, trump, trick resolution, draw, scoring.
- AI levels: easy, normal. Hard deferred to v0.2.0.
- CLI surface: `naipes [list|rules|play] ...`.
- 20 test vectors for cross-port conformance.
- All user-facing strings in English.
