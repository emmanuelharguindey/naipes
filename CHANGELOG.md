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

---

## Port: Rust

### [0.1.0] — 2026-05-13
- First release.
- Conformant: passes all 20 test vectors from `spec/vectors/`.
- Brisca with easy + normal AI.
- Zero runtime dependencies.
- MSRV: Rust 1.70.
- Public library API.
- MIT license.

---

## Port: TypeScript

### [0.1.0] — 2026-05-12
- First release on npm as `@naipes-com/naipes` (the plain `naipes` name was blocked by npm's similarity filter against `natives`).
- Conformant: passes all 20 test vectors from `spec/vectors/`.
- Brisca with easy + normal AI.
- Node 20+ supported. ESM only.
- Public TypeScript types exported for programmatic use.
- MIT license.

---

## Port: Python

### [0.1.0] — 2026-05-12
- First release.
- Reference implementation: the test vectors are generated from this one.
- Brisca with easy + normal AI.
- Python 3.10+ supported.
- MIT license.
