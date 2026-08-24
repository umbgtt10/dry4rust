# Implemented Features

What each version added, newest first. Upstream's releases are included, under
their own heading, because the engine they built is the engine this tool runs.

## Version 0.2.0 (unreleased)

The fork. `cargo-dupes` by Matjaz Domen Pecan became `cargo-dry4rust`, with
upstream's history kept as an ancestor rather than a credit line.

**Engine, inherited and unchanged in behaviour**

- AST normalisation via `syn`: identifiers and types to positional
  placeholders, literals erased with types preserved, control flow preserved
  exactly, macros opaque and keyed by name
- Exact duplicate detection by fingerprint equality
- Near-duplicate detection by Dice coefficient over normalised trees, with
  kind and size-band pre-filtering
- Sub-function analysis: if-branches, match arms, loop bodies, closure bodies
- Six subcommands, text and JSON output, four threshold ceilings, suppression
  by fingerprint with a reason

**Renamed to its own identity**

- `clap` program name, `.dry4rust-ignore.toml`, `dry4rust.toml` and
  `[package.metadata.dry4rust]` -- `--help` and `--version` had been naming a
  different tool than the one running

**Reduced in scope**

- Tree-sitter backend and Python bindings removed; one crate, one language
- `LanguageAnalyzer` kept as the injection seam, with `is_test_code` made
  required rather than defaulted

**Restructured for the gates**

- `find_near_duplicates` (complexity 27) became `NearDuplicateFinder`,
  `UnionFind` and `SimilarityPair`
- `extract_recursive` (complexity 20) became `SubUnitExtractor`
- `main` (complexity 14) became `CommandDispatcher`
- Four module registries emptied of code; nine re-exports removed so every
  import names where the symbol lives
- `NormalizationContext` moved out of `node.rs` into its own file

**Corrected in the engine**

- The fingerprint format is defined in this repository rather than inherited
  from `DefaultHasher`, derived `Hash` and native-endian integers. Variants are
  written by name, so reordering `NodeKind` no longer invalidates every
  suppression in every consuming repository. This changed all existing
  fingerprints once; the fifty entries in this repository's own ignore file
  went stale and were pruned in the same commit.
- `Block`, `Tuple` and `Array` children are aligned by weighted
  longest-common-subsequence instead of zipped. Two blocks differing by one
  inserted statement scored `2/11` and now score `10/11`, against a default
  threshold of `0.9`. Kinds whose children are named slots keep positional
  comparison, so an `If` with a then-branch is still not a match for an `If`
  with an else-branch.

- `CodeUnitKind` no longer restricts near-duplicate detection.
  `group_exact_duplicates` had always ignored kind, so a free function and a
  method with identical bodies were reported as exact duplicates while the
  same pair differing by one statement was invisible. The two halves of the
  tool now agree.
- The size pre-filter is now the exact bound the threshold implies, rather
  than `floor(log2(n))` bucketing. The old filter never compared units 7 and 8
  nodes apart -- a pair able to score `0.933` against a threshold of `0.9` --
  because a bucket boundary fell between them, while comparing pairs nearly
  twice apart that could not clear the bar. Codebases that reported no near
  duplicates may now report some, and those findings were always there.

**Corrected while restructuring**

- Candidate indices no longer recovered through `HashMap<*const CodeUnit, usize>`
  pointer identity; buckets carry indices directly
- `UnionFind::groups` orders groups by lowest member and members ascending;
  grouping had previously depended on `HashMap` iteration order
- `Fingerprint::new` added so tests can name a hash without the tuple field
  being public

**Added beyond the fork**

- Baseline mode. `cargo dry4rust baseline` records the duplication a codebase
  already has, `--baseline <path>` judges a run against it, and `check` then
  fails on what is added rather than on what was inherited. An entry carries
  the group's member count as well as its fingerprint, because an exact group
  keeps its fingerprint when a third copy joins it -- without the count, a
  baseline would inherit every future copy of every function it recorded. A
  baseline that cannot be read is an error and never an empty one, and the
  count it suppressed is in every summary it touched.

**Made impossible rather than documented**

- Every threshold is a `Threshold`, a newtype over a fraction that cannot be
  built out of range. `--threshold 5` had been accepted, and because the size
  pre-filter is derived from the threshold it discarded every pair -- so the
  report said "no near duplicates", which is what a clean codebase says.
  `--max-exact-percent 150` was a ceiling nothing could breach; `-5` was one
  everything breached. All three now name the field and exit 2. `NaN` is
  rejected by the same containment check, which a pair of comparisons would
  have admitted.

**Restructured again**

- `cli.rs` -- 461 lines holding an error enum with three trait impls, three
  more types and eight free functions -- became `src/cli/`, one type per file.
  The six subcommands are command structs with a single `run` method, so each
  has a constructor a test can reach. `checking/` moved under it: `Ceiling`,
  `CheckThresholds` and `StaleReport` serve `check` and `cleanup` and nothing
  else.
- `write_ignore_entry`, shared by `ignored` and `cleanup`, became
  `IgnoreEntryLine` with a `Display` impl -- a line a test can compare against
  rather than something only a buffer can observe.

**Documented**

- Fourteen ADRs with an index; `ARCHITECTURE.md`, `FORMULA.md`,
  `OPEN_POINTS.md`, `ROADMAP.md`, this file
- README credits the creator on its first line of body text, and documents
  `--sub-function`, `--min-sub-nodes` and `--baseline`, which were features
  with no entry
- `CHANGELOG.md` states the fingerprint-format break and what to do about it

**Verified**

- 513 tests, up from 213
- `stern4rust`: 0 offences over 124 files, 21 rules applied, nothing skipped,
  nothing baselined, one exclusion (`tests/fixtures/**`)
- `crap4rust`: 0 crappy functions at 15 over 237 functions, no override
- `twin4rust`: every source file mirrored
- `iceberg4rust`: no file at or above 10
- Both stage gates proven to fail, not only to pass

## Version 0.1.0 (2026-07-26)

Placeholder release reserving the `cargo-dry4rust` name on crates.io.
Subcommand plumbing only -- the binary printed a message and exited. No
analysis.

---

# Upstream releases (cargo-dupes)

By Matjaz Domen Pecan, MIT. See [CHANGELOG.md](../CHANGELOG.md) for the
verbatim upstream changelog with commit links.

## Version 0.1.5 (2026-02-13)

- Composite fingerprints for near-duplicate groups, making a group's identity
  independent of member order -- what makes suppression of a near-duplicate
  group possible at all
- `cleanup` subcommand, pruning ignore entries that no longer match

## Version 0.1.4 (2026-02-11)

- Percentage-based duplication thresholds for `check`
- All `check` thresholds default to disabled when unset

## Version 0.1.3 (2026-02-11)

- `--exclude-tests` to filter out test code

## Version 0.1.2 (2026-02-11)

- Release triggered on GitHub release rather than tag push

## Version 0.1.1 (2026-02-11)

- `--min-lines` filter and duplicated-line statistics

## Version 0.1.0 (2026-02-11)

- Initial implementation: `syn` parsing, AST normalisation, fingerprinting,
  Dice-coefficient near-duplicate detection, exact and near grouping,
  reporting
