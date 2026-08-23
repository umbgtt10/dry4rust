# Changelog

All notable changes to `cargo-dry4rust` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

Version 0.2.0 is the first release with an engine in it. It is a fork of
[`cargo-dupes`](https://github.com/mpecan/cargo-dupes) by Matjaz Domen Pecan (MIT), with
upstream's history kept as an ancestor rather than a credit line.

### ⚠ BREAKING

- **The fingerprint format is defined by this repository and every previously recorded
  fingerprint is invalid.** It rested on `DefaultHasher`, a derived `Hash` and
  native-endian integers, so a `NodeKind` reorder, a toolchain upgrade or a different
  architecture silently repointed every entry in every ignore file. Variants are now
  written by name into a hasher this crate owns.
  **If you are upgrading, re-record your suppressions:** run
  `cargo dry4rust cleanup` to prune the entries that no longer match anything, then
  `cargo dry4rust ignore <new fingerprint>` for each one you still want suppressed.
  `cleanup --dry-run` lists them first. The fifty entries in this repository's own ignore
  file went stale for exactly this reason and were pruned in the same commit.
- **Out-of-range configuration is rejected instead of accepted.** A
  `similarity_threshold` outside `0.0..=1.0` or a `max_*_percent` outside `0.0..=100.0`
  now fails the run with a message naming the field, and exits 2. Previously
  `--threshold 5` was accepted and made the size pre-filter discard every pair, so the
  report said "no near duplicates" — the same sentence a clean codebase produces. A
  `dry4rust.toml` that has been quietly wrong will start failing.
- **Every user-facing name follows the tool.** The `clap` program name, the ignore file
  (`.dry4rust-ignore.toml`), the config file (`dry4rust.toml`) and the manifest key
  (`[package.metadata.dry4rust]`) were still upstream's. Rename your files and manifest
  section when upgrading.
- **The tree-sitter and Python backends are gone.** One crate, one language.
  `LanguageAnalyzer` stays as the injection seam.

### Added
- **Baseline mode.** `cargo dry4rust baseline` records the duplication a codebase already
  has; `--baseline <PATH>` judges a run against that record, so `check` fails on what is
  added rather than on what was inherited. `baseline --dry-run` lists what would be
  recorded. Every summary a baseline touched states how many groups it suppressed. See
  [ADR-BaselineIsInheritedNotForgiven](docs/ADRs/ADR-BaselineIsInheritedNotForgiven.md).
- `baseline_suppressed` in the JSON summary, present only when a baseline was in effect.
- `docs/`: `ARCHITECTURE.md`, `FORMULA.md`, `OPEN_POINTS.md`, `ROADMAP.md`,
  `IMPLEMENTED-FEATURES.md`, `dry4rust-dossier.md`, and thirteen ADRs with an index.
- `LICENSE` carries both copyright notices, upstream's first, and every source file
  repeats them.
- Two quality-gate scripts, `scripts/run_stage_1.ps1` and `scripts/run_stage_2.ps1`, both
  proven to fail rather than only to pass.

### Fixed
- **Near-duplicate pairs one node apart were never compared.** The size pre-filter
  bucketed by `floor(log2(node_count))`, so a pair of 7 and 8 nodes able to score `0.933`
  fell either side of a boundary and was skipped, while pairs nearly twice apart were
  compared anyway. The filter is now the exact bound the threshold implies. Codebases
  that reported no near duplicates may now report some, and those findings were always
  there. See
  [ADR-SizeFilterIsAProvableBound](docs/ADRs/ADR-SizeFilterIsAProvableBound.md).
- **A block with one inserted statement scored as though it were unrelated.** `Block`,
  `Tuple` and `Array` children were zipped positionally, so a single insertion shifted
  every later statement out of alignment: two blocks differing by one statement scored
  `2/11`. They are now aligned by weighted longest-common-subsequence and score `10/11`.
  Kinds whose children are named slots keep positional comparison, so an `If` with a
  then-branch is still not a match for an `If` with an else-branch.
- **`CodeUnitKind` hid near duplicates that exact detection reported.** Exact grouping
  never considered kind, so a free function and a method with identical bodies were
  reported as exact duplicates while the same pair differing by one statement was
  invisible. Near-duplicate detection no longer restricts by kind.
- **Grouping depended on `HashMap` iteration order.** `UnionFind::groups` now orders
  groups by lowest member and members ascending, so the same input produces the same
  output.
- Candidate indices are carried directly rather than recovered through a
  `HashMap<*const CodeUnit, usize>` keyed on pointer identity.

### Changed
- `Config::load` and `CliOverrides::apply_to` are fallible, and thresholds are held in a
  `Threshold` newtype that cannot be built out of range. See
  [ADR-ThresholdsCarryTheirRange](docs/ADRs/ADR-ThresholdsCarryTheirRange.md).
- `cli.rs` is now `src/cli/`, one type per file: the six subcommands are command structs
  with a single `run` method.
- Four functions the CRAP gate could not admit at any coverage became five types —
  `NearDuplicateFinder`, `UnionFind`, `SimilarityPair`, `SubUnitExtractor`,
  `CommandDispatcher` — rather than the threshold being raised. See
  [ADR-DecompositionOverThresholdRelaxation](docs/ADRs/ADR-DecompositionOverThresholdRelaxation.md).
- Nine re-exports removed, so every import names the module where the symbol is defined.
- 482 tests, up from 213 at the fork. `stern4rust` clean with all 21 rules applied and no
  baseline; `crap4rust` clean at 15 with no override; every source file mirrored; no file
  at or above 10 on `iceberg4rust`.
- Upstream's changelog is preserved below, under its own heading, rather than replaced.

### Removed
- The `dupes-treesitter` and `dupes-python` crates, and the `code-dupes` multi-language
  binary. See
  [ADR-SingleCrateSingleLanguage](docs/ADRs/ADR-SingleCrateSingleLanguage.md).


## [0.1.0] - 2026-07-26

### Added
- Initial placeholder release â€” reserves the `cargo-dry4rust` name on crates.io.
- `cargo dry4rust` subcommand plumbing: argument forwarding in `main.rs`, `run`/`run_from_args`
  library entry points in `lib.rs`.
- No duplication analysis yet â€” the binary prints a placeholder message and exits successfully.

---

# Upstream changelog (cargo-dupes)

Everything below is the changelog of the project this was forked from, kept
verbatim so its release history is not lost.

All notable changes to this project will be documented in this file.
See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

---
## [0.1.5](https://github.com/mpecan/cargo-dupes/compare/cargo-dupes-v0.1.4...cargo-dupes-v0.1.5) (2026-02-13)


### Features

* composite fingerprints for near-duplicate groups + cleanup command ([#11](https://github.com/mpecan/cargo-dupes/issues/11)) ([2e49add](https://github.com/mpecan/cargo-dupes/commit/2e49add086a412c6debfb34f77bc1dbe5087c519))

## [0.1.4](https://github.com/mpecan/cargo-dupes/compare/cargo-dupes-v0.1.3...cargo-dupes-v0.1.4) (2026-02-11)


### Features

* add percentage-based duplication thresholds to check ([#8](https://github.com/mpecan/cargo-dupes/issues/8)) ([f528953](https://github.com/mpecan/cargo-dupes/commit/f528953f0ebb1aeb1d67773094ca1012ebf45657))


### Bug Fixes

* make all check thresholds default to disabled when not set ([#10](https://github.com/mpecan/cargo-dupes/issues/10)) ([de3743f](https://github.com/mpecan/cargo-dupes/commit/de3743f2615b9d4bc3a1f3bd077a3bbaa49efeae))

## [0.1.3](https://github.com/mpecan/cargo-dupes/compare/cargo-dupes-v0.1.2...cargo-dupes-v0.1.3) (2026-02-11)


### Features

* add --exclude-tests flag to filter out test code ([#2](https://github.com/mpecan/cargo-dupes/issues/2)) ([c116c95](https://github.com/mpecan/cargo-dupes/commit/c116c959197373664c470ea6edb00dba407c1f04))

## [0.1.2](https://github.com/mpecan/cargo-dupes/compare/cargo-dupes-v0.1.1...cargo-dupes-v0.1.2) (2026-02-11)


### Bug Fixes

* **ci:** trigger release on GitHub release instead of tag push ([#5](https://github.com/mpecan/cargo-dupes/issues/5)) ([2bc5059](https://github.com/mpecan/cargo-dupes/commit/2bc505956df8ce4b8a78e461aa038e7c1c07fc06))

## [0.1.1](https://github.com/mpecan/cargo-dupes/compare/cargo-dupes-v0.1.0...cargo-dupes-v0.1.1) (2026-02-11)


### Features

* add --min-lines filter and duplicated line statistics ([7de772b](https://github.com/mpecan/cargo-dupes/commit/7de772bda588c92d71341be0b50edfeddd26ccb5))
* add --min-lines filter and duplicated line statistics ([c47dd33](https://github.com/mpecan/cargo-dupes/commit/c47dd3348799a9f735122b29de2af92c49c7b02c))
* initial implementation of cargo-dupes ([5c8b4c1](https://github.com/mpecan/cargo-dupes/commit/5c8b4c1c47eb1a1c89a380e44a7adb53d24cb905))


### Bug Fixes

* **ci:** upgrade cocogitto-action from v3 to v4 ([b5621a6](https://github.com/mpecan/cargo-dupes/commit/b5621a66042da23cc8936d4070b4b15a975c491f))


### Documentation

* update CLAUDE.md for --min-lines and line stats ([faac27b](https://github.com/mpecan/cargo-dupes/commit/faac27b1fb913511d33a3249ff912478799d6c3b))
* update README with --min-lines flag and line stats output ([c21ec7c](https://github.com/mpecan/cargo-dupes/commit/c21ec7c9daeccfef6bc8734f6dc7b06b157ddd68))

## [0.1.0] - 2026-02-10

### Initial Release
- AST-based duplicate and near-duplicate code detection
- Text and JSON output formats
- CLI with report, stats, check, ignore, and ignored subcommands
- Configuration via dupes.toml or Cargo.toml metadata

