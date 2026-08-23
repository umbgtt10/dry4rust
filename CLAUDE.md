# dry4rust

## Meaning

`dry4rust` is a `cargo` subcommand that detects duplicated and near-duplicated
code in Rust, by normalising the AST, fingerprinting it, and comparing trees
with the Dice coefficient.

It is a fork of [`cargo-dupes`](https://github.com/mpecan/cargo-dupes) by
Matjaz Domen Pecan, MIT. The engine is his work; `LICENSE` carries both
copyright notices and every source file repeats them. See `README.md`.

`docs/ADRs/` holds the decisions this repository has settled and what forced
them. Where it is going rather than where it came from is in
`docs/dry4rust-dossier.md`, which is research and not a record.

## Boundary Rule

This repository is **SELF-CONTAINED**.

The LLM **SHALL NOT cross its boundaries without asking**.

That means:
- do not inspect, edit, or rely on files outside `dry4rust/` unless the user
  explicitly asks
- do not pull assumptions from sibling repositories or crates
- do not propose cross-repository changes by default

Upstream is the exception worth naming: `git fetch upstream` reaches
`mpecan/cargo-dupes`, and pulling from it is a deliberate act, not a default.

## Quality Gates

### Mandatory after every change

Run:

`powershell -File scripts\run_stage_1.ps1`
`powershell -File scripts\run_stage_2.ps1`

If either gate is not green, the work is not complete.

Stage 1 is formatting, clippy and tests -- cargo built-ins only, so it works on
a fresh checkout. It runs with `RUSTFLAGS=-D warnings`, which is stricter than
a bare `cargo test`. Stage 2 is four installed cargo subcommands, run in this
order:

| gate | asks |
|---|---|
| `cargo stern4rust` | do the house coding rules hold |
| `cargo crap4rust` | is any function complex and untested |
| `cargo twin4rust` | does every source file have a mirrored test file |
| `cargo iceberg4rust` | is any file's private implementation risk too high |

stern4rust runs **first** because its corrections are renames, file moves and
directory splits: a layout it is about to reject is a layout the other three
would have measured for nothing. Its findings are also the cheapest to act on.

All twenty-one of its rules are enforced, with nothing skipped, nothing
unconfigured and no baseline file. `docs/header.txt` holds the four-line header
every `.rs` file carries and `stern4rust.toml` names it -- in the config rather
than the gate script, so a hand-run of `cargo stern4rust` checks exactly what
the gate checks.

Four lines rather than the three its sibling repositories use, because MIT
requires upstream's copyright notice to travel with the code.

`tests/fixtures/**` is excluded, and that is the one exclusion. Those crates
are the corpus the tool measures, and several tests assert on their line
counts, so a header stamped into them would edit the question rather than
answer it. The report names the exclusion and how many files it removed.

`crap4rust` runs at 15 -- the family's number, with no override and no
tolerance. Nothing is baselined, skipped or budgeted.

That took decomposition rather than tests. CRAP is
`complexity^2 * (1 - coverage)^3 + complexity`, so it never falls below the
complexity itself: a function of complexity 27 cannot reach 15 at any
coverage, including 100%. Three inherited functions were in that position, and
each became a struct in a file of its own with a mirrored test file --
`NearDuplicateFinder`, `UnionFind`, `SimilarityPair`, `SubUnitExtractor`,
`CommandDispatcher`. If this gate ever needs an override again, the honest
move is the one taken here: split the function.

`iceberg4rust` runs at 10 rather than the default, and that one is a real bound
-- the worst file is 9.55.

`cargo install cargo-stern4rust`
`cargo install cargo-crap4rust`
`cargo install cargo-twin4rust`
`cargo install cargo-iceberg4rust`

## Strict Rules

### Test preservation

**NEVER remove a test without explicitly asking the user first.**

If a code change causes a test to fail, the correct response is to update the
test assertion to reflect the new correct behavior — not to delete the test. A
test documents a contract. Deleting it silently removes coverage and hides
regressions.

The only acceptable reason to delete a test is if it is a deliberate exact
duplicate of another test that fully covers the same contract. Even then, ask
before deleting.

### Assertion integrity

**NEVER relax a test assertion without explicitly asking the user first.**

Relaxing means changing a specific assertion to a weaker one. If a code change
makes a previous assertion wrong, understand why, then update the assertion to
the new correct specific value — not to a weaker form that would pass
regardless of correctness.

### Visibility is not an escape hatch

`tested-public-api` asks for a test, not a smaller surface. Do not drop `pub`
to `pub(crate)` to silence it. If something is worth calling from outside, it
is worth a test; if it is not, deleting it is the honest answer.

## User coding standards

- one struct per file
- do not use fully qualified paths; use `use` imports instead
- unit tests are not allowed. Only integration tests are, under `tests/`
- no unnecessary comments in code
- no `&mut` input parameters; prefer return values (`&mut self` is allowed in
  traits and impl blocks)
- only use `pub mod` in `mod.rs` and `lib.rs`
- split test files so there is one test file per source file, named
  `<source file name>_tests.rs`
- in `all_tests.rs`, reference test files one by one without `#[path = ...]`
- apply AAA (`Arrange`, `Act`, `Assert`) structure to tests, with blank-line
  separation between the sections
- use `// Arrange & Act` if there is no separate `Arrange`
- use `// Act & Assert` if there is no separate `Act`
- add the repository copyright and license header to every Rust source file
- **test naming**: `<method>_<description>_<outcome>` where `<method>` is the
  function called in the Act section

## Upstream's lint configuration

`Cargo.toml` keeps upstream's `[lints.clippy]` block: `pedantic` and `nursery`
at warn with a documented allow-list. It is stricter than stage 1's `-D
warnings` on the default lints, it already passes, and it has caught three
newly-public functions missing `#[must_use]` during this repository's cleanup.
Keep it.
