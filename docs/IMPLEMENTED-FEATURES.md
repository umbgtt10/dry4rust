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

**Corrected while restructuring**

- Candidate indices no longer recovered through `HashMap<*const CodeUnit, usize>`
  pointer identity; buckets carry indices directly
- `UnionFind::groups` orders groups by lowest member and members ascending;
  grouping had previously depended on `HashMap` iteration order
- `Fingerprint::new` added so tests can name a hash without the tuple field
  being public

**Documented**

- Seven ADRs with an index; `ARCHITECTURE.md`, `FORMULA.md`,
  `OPEN_POINTS.md`, `ROADMAP.md`, this file
- README credits the creator on its first line of body text

**Verified**

- 324 tests, up from 213
- `stern4rust`: 0 offences over 65 files, 21 rules applied, nothing skipped,
  nothing baselined, one exclusion (`tests/fixtures/**`)
- `crap4rust`: 0 crappy functions at 15, no override
- `twin4rust`: every source file mirrored
- `iceberg4rust`: no file at or above 10, worst is 9.55
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
