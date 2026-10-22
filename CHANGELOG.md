# Changelog

All notable changes to `cargo-dry4rust` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

The binary is unchanged: its command line, output, exit codes and file formats
are exactly 0.4.0's. The library is not: seven `core/` modules are renamed,
split or moved, and two types renamed, so each file is named after its
subject.

### Changed
- `DEFAULT_BASELINE_FILE` and `FORMAT_VERSION` move out of
  `suppression::baseline_file` into `suppression::baseline_constants`, a file
  of constants, which stern4rust's coming `shared-constants` rule asks of a
  public constant several files read: the baseline file reads both, and three
  test files `FORMAT_VERSION`. Their values are unchanged.
- Every source file is named after its subject, which stern4rust 0.16.0's new
  `named-subject` rule asks for. `output::json` and `output::text` are
  `output::json_reporter` and `output::text_reporter`. `extractor` is
  `sub_unit`, after the `SubUnit` it declares. `grouper` is `duplicate_group`,
  and `DuplicationStats` moves out of it to `output::duplication_stats`.
  `CodeUnitKind` moves out of `code_unit` to `code_unit_kind`.
  `rust::parser`'s visitor moves to `rust::code_unit_extractor` as
  `CodeUnitExtractor`, with `new` and `into_units` public and three tests of
  its own. `AnalysisResult` is `Analysis`. `ScanConfig` is `Scanner`, and
  `scan_files` is its method. `normalization_context` moves under
  `rust::normalizer`, the only code that uses it, which keeps the top of `src/`
  at the twenty files `directory-file-count` allows. Tests moved with the code
  they test; none was removed and no assertion changed.
- No source file with a subject holds a free function, which stern4rust
  0.17.0's new `owned-functions` rule asks for: twenty-six became associated
  functions of the type beside them. `analyze` and `analyze_units` are
  `Analysis`'s; the grouping and statistics functions are `DuplicateGroup`'s;
  `count_nodes` and `reindex_placeholders` are `NormalizedNode`'s; the
  normaliser's public entry points and helpers are `NormalizationContext`'s;
  `extract_sub_units`, `is_excluded` and `baseline_path` belong to `SubUnit`,
  `Scanner` and `BaselineFile`. `output::reporter::display_path`, beside a trait
  that cannot hold it, is `output::display_path::DisplayPath::of`, with its
  three tests. All twenty-four stern4rust rules apply to every member.
- `validation` pairs its scenario files with source instead of baselining
  `paired-test-file`. Its shared harness moved out of `tests/common.rs` into
  `validation/src/`, one type per file -- `Fixture` (`path`, `analysed`,
  `analysed_with_sub_function`, `duplicated_crate_in`), `DryBinary`
  (`command`, `command_in`), `TextReport` (`fingerprint_in`) and
  `DuplicateGroupFixture` (`of`) -- each with a test file of its own. The ten
  scenario files are renamed `<scenario>_scenario_tests.rs`, and each pairs
  with a `src/<scenario>_scenario.rs` holding the entry point its tests call:
  the private helpers `check`, `dispatch` and `format` carried, the
  `AnalysisOutput::produce` call `pipeline` repeats, and for the other six the
  binary with `--path` already set. No test was removed and no assertion
  changed; `validation` runs 109 tests, up from 94.
- `stern4rust-baseline.json` is deleted and `stern4rust.toml` names no
  baseline. All twenty-three rules apply to every member with nothing skipped and
  nothing baselined: `offences=0 baselined=0 rules_applied=24`.
- No return type holds a tuple whose elements only their position tells apart,
  which stern4rust's coming `distinct-tuple-elements` rule asks for.
  `SimilarityPair::key` returns a `near_duplicate::pair_key::PairKey`, whose
  `low` and `high` say which index is which, and the near-duplicate finder keys
  its scores by it. `NormalizationContext::normalize_item_fn` and
  `normalize_impl_item_fn` return a `rust::normalizer::normalized_fn::NormalizedFn`
  naming the `signature` and the `body`, and `normalize_impl_block` pairs one
  with each method's name.
- `NormalizationContext` is implemented in one file, which stern4rust's coming
  `implemented-in-one-file` rule asks for. Its methods sat in five --
  `normalization_context.rs`, `normalize.rs`, `expr.rs`, `pat.rs` and
  `helpers.rs` -- and are all in `rust::normalizer::normalization_context` now,
  with their 79 tests in its test file; the four modules that held only impl
  blocks are gone. Nothing they declared changes its name or its path.
- Parsing fails with the crate's own `error::Error` rather than a `String` or a
  boxed `dyn Error`, which stern4rust's coming `typed-errors` rule asks of a
  library's public functions. `LanguageAnalyzer::parse_file` returns
  `Result<Vec<CodeUnit>, Error>` -- an implementor of the trait changes its
  signature -- and `rust::parser::parse_source` and `parse_file` do too. Two
  variants are new, `Read` and `Parse`, each naming the file and why, worded
  as the warnings were.

## [0.4.0] - 2026-10-10

What the library hands back, not what the tool does. The command line, the text
and JSON it prints, its exit codes, and the ignore and baseline file formats are
byte-for-byte what 0.3.0 produces: the two binaries were run side by side over
every fixture and over this crate's own source, text and JSON, across every
subcommand and through multi-step ignore -> cleanup and baseline -> check
sequences, and stdout, stderr, exit codes and files written were identical. No
fingerprint moves, so no recorded suppression goes stale.

Minor rather than patch because public signatures of the library changed, as
listed under Changed: code built against `dry4rust` as a library -- a
`Reporter` implementation, a call to a command's `run`, the normaliser
functions -- has to follow.

### Changed
- **Output is returned rather than written through a `&mut` parameter**, which
  is stern4rust 0.14.0's new `returned-mutations` rule; it found 31 offences
  here and all are cleared.
  - `Reporter`'s seven methods drop their `writer: &mut dyn io::Write` and
    return `io::Result<String>`. `GroupSection::write` returns its `String`.
  - Every command's `run` -- `StatsCommand`, `ReportCommand`, `CheckCommand`,
    `CleanupCommand`, `BaselineCommand`, `IgnoreCommand`, `IgnoredCommand` --
    and `CommandDispatcher::dispatch` drop their writer and return
    `CliResult<CommandOutcome>`: the text to print and the verdict. A breached
    `check`, and a baseline that cannot be written, travel in the outcome's
    `result` beside the text rather than as the `Err` of `run`, so their output
    still reaches stdout ahead of the error, exactly as before.
  - The normaliser's free functions that threaded a `&mut NormalizationContext`
    -- `normalize_expr`, `normalize_stmt`, `normalize_block`, `normalize_pat`,
    `normalize_type`, `normalize_macro`, `normalize_signature` -- are methods of
    the same names on `NormalizationContext`. Evaluation order is unchanged, so
    placeholder numbering is too.
- Stage 2 applies every one of stern4rust's twenty-two rules to every member
  with nothing skipped. `validation`'s `skip = ["paired-test-file"]` is gone;
  the ten scenario files it covered, which have no source file to be named
  after, are recorded in `stern4rust-baseline.json` instead, so the rule keeps
  running on `validation` and reports any scenario file added later.

### Added
- `cli::command_outcome::CommandOutcome` and
  `placeholder_order_collector::PlaceholderOrderCollector`, both public, each
  with a mirrored test file.
- A fifth stage 2 gate, second after stern4rust: this checkout's own dry4rust,
  built from source rather than installed, over `core/src` with zero ceilings
  against `dry4rust-baseline.json` at a floor of 25 AST nodes. A tool that
  enforces a check it does not pass is not worth installing. It found four
  groups; two were shared away (`TextReporter::section`, `Threshold::share_of`)
  and the baseline holds the two near-duplicate groups that remain. CI needs no
  new install for it.

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

