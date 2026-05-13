# BUILD.md — `naipes` operational guide

Document for maintainers. Read me before touching anything after a long break.

---

## 1. Install dev dependencies (once)

On macOS or Linux:

```bash
# For the Python port:
python3 -m venv ~/naipes-tools
~/naipes-tools/bin/pip install build twine pytest

# For the TypeScript port:
# Node 20+ via fnm, nvm, or your preferred manager.

# For the Rust port:
# Via rustup: https://rustup.rs
```

---

## 2. Repo structure

```
naipes/
├── spec/
│   ├── SPEC.md
│   └── vectors/v001.json..v020.json
├── ports/
│   ├── python/
│   ├── typescript/
│   └── rust/
├── .github/workflows/
├── README.md
├── BUILD.md
├── CONTRIBUTING.md
├── CHANGELOG.md
├── LICENSE
├── Makefile
└── .gitignore
```

`spec/` is the source of truth. Any change to rules or vectors forces an update of the three ports to keep them conformant.

---

## 3. Daily commands

From the repo root:

```bash
make test          # Python port tests
make build         # builds wheel + sdist into ports/python/dist/
make clean         # removes build artefacts and caches
make verify        # build + install in temp venv + run the CLI
```

For TypeScript:

```bash
cd ports/typescript
npm install
npx tsc
npx tsx --test tests/*.test.ts
```

For Rust:

```bash
cd ports/rust
cargo test
cargo build --release
```

---

## 4. Bumping a port version

Each port versions independently. The procedure is similar across the three.

### 4.1 Version bump

Edit the two places that mention the version:

**Python:**
- `ports/python/pyproject.toml` → `version = "0.X.Y"`
- `ports/python/naipes/__init__.py` → `__version__ = "0.X.Y"`

**TypeScript:**
- `ports/typescript/package.json` → `"version"` field
- `ports/typescript/src/cli/root.ts` → `VERSION` constant

**Rust:**
- `ports/rust/Cargo.toml` → `version = "0.X.Y"`
- `ports/rust/src/cli/root.rs` → `VERSION` constant

Semver rules:
- Patch `0.1.0 → 0.1.1`: bug fix without interface changes.
- Minor `0.1.X → 0.2.0`: new compatible feature (e.g. adding Hard AI, adding tute).
- Major `0.X.Y → 1.0.0`: incompatible spec change.

### 4.2 Changelog update

Edit `CHANGELOG.md` adding an entry under the corresponding port section.

### 4.3 Tag

Tags are prefixed by port name so they don't collide:

```bash
git tag -a python-v0.X.Y -m "Python port v0.X.Y"
git tag -a npm-v0.X.Y    -m "npm port v0.X.Y"
git tag -a rust-v0.X.Y   -m "Rust port v0.X.Y"
git push origin <tag-name>
```

The `.github/workflows/<lang>-publish.yml` workflows handle the rest when remote publishing is configured.

---

## 5. Regenerating test vectors

Only if rules change in the spec.

⚠️ Regenerating vectors is a spec decision, not a cleanup. If you regenerate them, all three ports become non-conformant until you update them.

```bash
cd ports/python
~/naipes-tools/bin/python generate_vectors.py ../../spec/vectors/
```

Afterwards: run tests on all three ports and fix what breaks.

---

## 6. Adding a new game

1. Write the game's spec (rules, card values, trick logic, win conditions) as a new section in `spec/SPEC.md`.
2. Implement it in at least one of the existing ports (recommended: Python first, as it is the reference).
3. Generate test vectors specific to the new game under `spec/vectors/`.
4. Port to the other ports, verifying against the vectors.
5. Update the CLI dispatcher (`naipes list`, `naipes rules <game>`, `naipes play <game>`) in the three ports.

---

## 7. Adding a port in another language

See [CONTRIBUTING.md](CONTRIBUTING.md), section "Adding a port in another language".

---

## 8. Resources

- Canonical spec: [`spec/SPEC.md`](spec/SPEC.md)
- Test vectors: [`spec/vectors/`](spec/vectors/)

---

## 9. When something breaks

- **Conformance tests fail after a code change:** you've touched game logic. Either fix the implementation or accept a spec change (regenerate vectors and update all ports).
- **GitHub Actions fails but local works:** usually a runtime version mismatch. Check `matrix.<lang>-version` in the workflow.
