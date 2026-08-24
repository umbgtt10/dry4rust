# Changelog

All notable changes to `cargo-dry4rust` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

## [0.3.0] - 2026-08-24

How the gates are run, not what the tool does. No detection behaviour changed,
no flag was added or removed, and no score moves. Minor rather than patch
because the workspace gained a member and the test suite grew by 63.

### Added
- `xtask/`, a real crate replacing the stage 2 PowerShell script. Each gate is a
  `Gate` implementation constructed against a `CommandRunner` trait, so the
  argument lists and failure messages are covered by 63 integration tests rather
  than being unobservable shell. It is a workspace member and is gated like the
  rest -- the crate that runs the gates is not exempt from them.
- `.github/workflows/ci.yml`: both stages on Ubuntu, Windows and macOS, for
  every pull request and every push to `main`. CI runs `just stage1` /
  `just stage2` -- the same two commands a developer runs -- so there is no
  second definition of the gates to drift out of step.

### Changed
- Gates run through `just stage1` / `just stage2` on all three platforms.
- Stage 1 now lints test targets too (`cargo clippy --workspace --all-targets`),
  which the PowerShell script never did. Combined with the `pedantic` and
  `nursery` groups `core` already enables, that surfaced 68 pre-existing
  offences across thirteen files under `core/tests/`, all fixed. Fifty-seven
  were `cargo clippy --fix` material; the rest were `collect()` into a `Vec`
  only to call `.len()`, single-item `into_iter()`, and two unseparated hex
  literals. No assertion was weakened -- `collect().len()` and `count()` yield
  the same number, so every asserted value is unchanged.
- CI checks formatting instead of applying it (`cargo fmt --check` when `CI` is
  set), so drift fails the build rather than being silently rewritten where
  nobody is there to review it. A local `just stage1` still formats in place.
- `twin4rust` and `iceberg4rust` each ran twice, once per member. They now take
  both members in a single call, verified equivalent: scoping either to
  `validation` alone scans nothing and passes vacuously, because it has no
  source files to mirror or to score.

### Removed
- `scripts/run_stage_1.ps1` and `scripts/run_stage_2.ps1`. A Windows-only gate
  is not a gate contributors on Linux or macOS can run.

## [0.2.0] - 2026-08-24

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
- **`--format json` now emits one JSON document per run.** `report` was the summary
  followed by one bare array per section, each a separate top-level value, so `jq .`
  failed on it and the number of documents varied with what was found. `check` was worse:
  it printed `Check FAILED: …` as prose *between* two JSON values, which no parser could
  read at all. Both are now a single object with named keys — `report` gives
  `stats`/`exact`/`near`/`sub_exact`/`sub_near`, and `check` gives
  `stats`/`passed`/`breaches`/`exact`/`near`. Anything splitting the old output on blank
  lines needs rewriting to index by key. **Text output is unchanged.**

### Added
- **Baseline mode.** `cargo dry4rust baseline` records the duplication a codebase already
  has; `--baseline <PATH>` judges a run against that record, so `check` fails on what is
  added rather than on what was inherited. `baseline --dry-run` lists what would be
  recorded. Every summary a baseline touched states how many groups it suppressed. See
  [ADR-BaselineIsInheritedNotForgiven](docs/ADRs/ADR-BaselineIsInheritedNotForgiven.md).
- `baseline_suppressed` in the JSON summary, present only when a baseline was in effect.
- `docs/`: `ARCHITECTURE.md`, `FORMULA.md`, `OPEN_POINTS.md`, `ROADMAP.md`,
  `IMPLEMENTED-FEATURES.md`, `dry4rust-dossier.md`, and fourteen ADRs with an index.
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
- `ignore` and `baseline` became `src/suppression/`, one type per file. `IgnoreFile` has
  methods where nine free functions took it as an argument, three of them as `&mut`; every
  operation that changes the file now takes it by value and hands back the changed one.
- `config.rs` split into `Config`, `FileConfig` and `AnalysisConfig`, one per file. Three
  structs that existed only to spell out the path `package` → `metadata` → `dry4rust`
  through `Cargo.toml` are gone, replaced by navigating the parsed document.
- 565 tests, up from 213 at the fork. `stern4rust` clean with all 21 rules applied and no
  baseline; `crap4rust` clean at 15 with no override; every source file mirrored; no file
  at or above 10 on `iceberg4rust`.
- Upstream's changelog is preserved below, under its own heading, rather than replaced.

- The repository is a workspace: `core/` publishes as `cargo-dry4rust`, `validation/` is
  `publish = false` and holds the tests whose subject is the whole tool, and `fixture/`
  holds the corpus outside both. `cargo package` drops any subdirectory holding a
  `Cargo.toml`, so the corpus had been dropped from the tarball while the tests reading it
  were kept -- `cargo test` on the packaged crate gave 66 failures and now gives none. See
  [ADR-CorpusOutsideThePackage](docs/ADRs/ADR-CorpusOutsideThePackage.md).
- Both workspace members are gated. `core` takes all twenty-one house rules;
  `validation` takes twenty, with `paired-test-file` skipped by name because a crate with
  no `src/` has nothing for a test file to be named after.
- The fixture corpus carries the four-line header, and each crate keeps its code in
  `src/target.rs` behind a `src/lib.rs` that only names it -- matching the family, and
  putting productive code out of a `lib.rs` the house rules keep for registries.
- `cleanup` takes the ignore file away when it prunes the last entry, instead of writing
  back `ignore = []`. An empty suppression list makes exactly the claim no file makes, and
  `load` cannot tell them apart -- this repository acquired one of these itself when the
  fingerprint format changed and all fifty entries went stale at once.
- `main` is eleven lines. The argument mapping moved into the library as `EntryPoint`, so
  which root, which command and which overrides are things a test can call.

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

