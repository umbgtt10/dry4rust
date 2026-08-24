# dry4rust

Created by **Matjaz Domen Pecan** as [`cargo-dupes`](https://github.com/mpecan/cargo-dupes),
forked and extended by **Umberto Gotti** under the [MIT licence](LICENSE).

**A `cargo` subcommand for detecting duplicated code patterns in Rust -- DRY (Don't Repeat
Yourself) analysis.**

## Provenance

`dry4rust` is a fork of [`cargo-dupes`](https://github.com/mpecan/cargo-dupes) by
**Matjaz Domen Pecan**, used under the [MIT licence](LICENSE). The duplicate-detection
engine documented below — AST normalisation, fingerprint hashing and Dice-coefficient
similarity — is his work, and both copyright notices are carried in `LICENSE` and in every
source file.

The fork keeps the Rust analyser and drops the tree-sitter and Python backends, so this is a
single-language tool by intent rather than a general one. What is added beyond that point is
noted in `CHANGELOG.md`.

## Family

Part of the same family as [`grip`](https://crates.io/crates/cargo-grip4rust) (testability),
[`braintax`](https://crates.io/crates/cargo-braintax4rust) (cognitive load), and
[`crap4rust`](https://crates.io/crates/cargo-crap4rust) (change-risk complexity × coverage).
Where those three measure how safe, understandable, and risky a codebase is, `dry4rust`
measures how much of it is needlessly repeated.

## Status

The engine is upstream's, with four corrections it does not have — an exact size bound,
aligned sequence children, a fingerprint format that survives a toolchain upgrade, and
near-duplicate detection that no longer hides what exact detection reports. Added on top:
baseline mode, and configuration that cannot hold an impossible threshold.

The surrounding repository is under this family's house rules and gates: `cargo stern4rust`
reports no offences with all twenty-one rules applied and no baseline of its own,
`cargo crap4rust` finds no function at or above 15, every source file has a mirrored test
file, and no file reaches 10 on `cargo iceberg4rust`.

Commands below are `cargo dry4rust`; the upstream `cargo dupes` spelling is gone.


## Install

```powershell
cargo install cargo-dry4rust
```

## License

MIT — see [LICENSE](LICENSE), which carries both copyright notices.

## How It Works

`dry4rust` parses Rust source files into ASTs using [syn](https://github.com/dtolnay/syn), then normalizes each function, method, and closure into a canonical form where:

- **Identifiers are replaced** with positional placeholders (so `foo(x)` and `bar(y)` are identical)
- **Literal values are erased** but types preserved (`42` and `99` are both "integer literal")
- **Control flow structure is preserved** exactly
- **Macro invocations become opaque** nodes

This normalized AST is hashed into a fingerprint for exact duplicate detection, and compared tree-by-tree using the Dice coefficient for near-duplicate detection.

## Usage

```
cargo dry4rust [OPTIONS] [COMMAND]

Commands:
  stats     Show duplication statistics only
  report    Show full duplication report (default)
  check     Check for duplicates and exit with non-zero if thresholds exceeded
  ignore    Add a fingerprint to the ignore list
  ignored   List all ignored fingerprints
  cleanup   Remove ignore entries that no longer match anything
  baseline  Record the duplication that is already there

Options:
  -p, --path <PATH>                Path to analyze (defaults to current directory)
      --min-nodes <MIN_NODES>      Minimum AST node count for analysis
      --min-lines <MIN_LINES>      Minimum source line count for analysis
      --threshold <THRESHOLD>      Similarity threshold (0.0-1.0)
      --format <FORMAT>            Output format [default: text] [possible values: text, json]
      --exclude <EXCLUDE>          Exclude patterns (can be repeated)
      --exclude-tests              Exclude test code (#[test] functions and #[cfg(test)] modules)
  -s, --sub-function               Also analyse if-branches, match arms, loop bodies and closure bodies
      --min-sub-nodes <N>          Minimum AST node count for a sub-function unit [default: 5]
      --baseline <PATH>            Judge the run against a recorded baseline of inherited duplication
  -h, --help                       Print help
  -V, --version                    Print version
```

Every example below is real output from `fixture/exact_dupes`, which ships in the
repository -- `cargo run -p cargo-dry4rust -- --path fixture/exact_dupes report` reproduces
it.

`--threshold` and the two `--max-*-percent` ceilings are checked against their ranges. A
threshold outside `0.0..=1.0`, or a percentage outside `0.0..=100.0`, fails the run with a
message naming the field rather than being accepted and quietly finding nothing.

### Examples

**Full report:**

```sh
$ cargo dry4rust report
Duplication Statistics
=====================
Total code units analyzed: 3

Exact duplicates: 1 groups (3 code units)
Near duplicates:  0 groups (0 code units)

Duplicated lines (exact): 27
Duplicated lines (near):  0
Duplication: 100.0% exact, 0.0% near (of 27 total lines)

Exact Duplicates
================

Group 1 (fingerprint: 396fa8f6b728ff01, 3 members):
  - process_data (function) at src/target.rs:6-14
  - compute_total (function) at src/target.rs:16-24
  - aggregate (function) at src/target.rs:26-34
```

**Statistics only:**

```sh
$ cargo dry4rust stats
Duplication Statistics
=====================
Total code units analyzed: 3

Exact duplicates: 1 groups (3 code units)
Near duplicates:  0 groups (0 code units)

Duplicated lines (exact): 27
Duplicated lines (near):  0
Duplication: 100.0% exact, 0.0% near (of 27 total lines)
```

**JSON output:**

```sh
$ cargo dry4rust --format json stats
{
  "total_code_units": 3,
  "total_lines": 27,
  "exact_duplicate_groups": 1,
  "exact_duplicate_units": 3,
  "near_duplicate_groups": 0,
  "near_duplicate_units": 0,
  "exact_duplicate_lines": 27,
  "near_duplicate_lines": 0,
  "exact_duplicate_percent": 100.0,
  "near_duplicate_percent": 0.0
}
```

Every `--format json` run is a **single JSON document**, so `jq` and any ordinary parser
read it in one call. `report` and `check` name their sections:

```sh
$ cargo dry4rust --format json report | jq '.exact | length'
$ cargo dry4rust --format json check --max-exact 0 | jq '.passed, .breaches'
false
[
  "1 exact duplicate groups (max: 0)"
]
```

| command | keys |
|---|---|
| `stats` | the summary fields, at the top level |
| `report` | `stats`, `exact`, `near`, and `sub_exact`/`sub_near` when sub-function analysis found any |
| `check` | `stats`, `passed`, `breaches`, and `exact`/`near` when a ceiling was breached |

`exact` and `near` are always present on `report` because those are always analysed. The
sub-function sections are absent rather than empty when the analysis was not asked for, so
`[]` never stands in for "not looked at". On `check`, the groups behind a breach are listed
once even when two ceilings on the same set are breached together.

**CI check (fail if any exact duplicates exist):**

```sh
$ cargo dry4rust check --max-exact 0
# Exits with code 1 if exact duplicate groups > 0
# Exits with code 0 if within thresholds
```

**CI check with percentage thresholds (fail if >5% of lines are exact duplicates):**

```sh
$ cargo dry4rust check --max-exact-percent 5.0
# Exits with code 1 if exact duplicate lines exceed 5% of total lines
```

**Exclude test code (inline `#[cfg(test)]` modules and `#[test]` functions):**

```sh
$ cargo dry4rust --exclude-tests report
```

**Exclude test directories by path:**

```sh
$ cargo dry4rust --exclude tests --exclude benches report
```

**Only report duplicates that are at least 10 lines long:**

```sh
$ cargo dry4rust --min-lines 10 report
```

**Lower the similarity threshold:**

```sh
$ cargo dry4rust --threshold 0.7 report
```

## Sub-function Analysis

By default a code unit is a whole function, method or closure. Two functions that share a
copy-pasted `match` arm but differ elsewhere are not duplicates of each other, and nothing
is reported.

`--sub-function` (or `-s`) also treats each if-branch, match arm, loop body and closure
body as a unit in its own right:

```sh
$ cargo dry4rust --sub-function report
...
Sub-function exact: 3 groups (6 units)
Sub-function near:  0 groups (0 units)
...
Sub-function Exact Duplicates
=============================

Group 1 (fingerprint: 004522adf0425ce1, 2 members):
  - for body (loop body) in sum_doubled at src/target.rs:70-78
  - for body (loop body) in accumulate at src/target.rs:80-88

Group 2 (fingerprint: 847270a821ab17a2, 2 members):
  - match arm 2 (match arm) in classify_number at src/target.rs:33-53
  - match arm 2 (match arm) in describe_value at src/target.rs:55-66
```

Each member names the function it came from, and the line range shown is that parent
function's — not the branch's. Sub-function units are grouped separately from top-level
ones and counted under their own headings, so a function never shares a group with its own
branch. `--min-sub-nodes` (default `5`) is the floor a branch must reach to be considered
at all; raise it when small branches produce noise.

Two limits are worth knowing before trusting the output, both in
[docs/OPEN_POINTS.md](docs/OPEN_POINTS.md): sub-function findings restate a function-level
one when two functions are already duplicates of each other, and the parent line range
means a report cannot be read straight to the branch.

## Configuration

Configuration can be provided in three ways (in order of precedence):

1. **CLI flags** (highest priority)
2. **`dry4rust.toml`** in the project root
3. **`Cargo.toml`** under `[package.metadata.dry4rust]`

### `dry4rust.toml`

```toml
min_nodes = 15
min_lines = 5
similarity_threshold = 0.85
exclude = ["tests", "benches"]
exclude_tests = true
max_exact_duplicates = 0
max_near_duplicates = 10
max_exact_percent = 5.0
max_near_percent = 10.0
```

### `Cargo.toml`

```toml
[package.metadata.dry4rust]
min_nodes = 15
similarity_threshold = 0.85
exclude = ["tests"]
```

### Configuration Options

| Option | Default | Description |
|--------|---------|-------------|
| `min_nodes` | `10` | Minimum AST node count for a code unit to be analyzed. Increase to skip trivial functions. |
| `min_lines` | `0` | Minimum source line count for a code unit to be analyzed. `0` means disabled. |
| `similarity_threshold` | `0.9` | Minimum similarity score for near-duplicate detection. Must be within `0.0..=1.0`. |
| `exclude` | `[]` | Path patterns to exclude from scanning (substring match). |
| `exclude_tests` | `false` | Exclude `#[test]` functions and `#[cfg(test)]` modules from analysis. |
| `sub_function` | `false` | Also analyse if-branches, match arms, loop bodies and closure bodies. |
| `min_sub_nodes` | `5` | Minimum AST node count for a sub-function unit to be analyzed. |
| `baseline` | `None` | Path to a recorded baseline of inherited duplication, relative to the analysed root. |
| `max_exact_duplicates` | `None` | For `check` subcommand: maximum allowed exact duplicate groups. |
| `max_near_duplicates` | `None` | For `check` subcommand: maximum allowed near-duplicate groups. |
| `max_exact_percent` | `None` | For `check` subcommand: maximum allowed exact duplicate line percentage. Must be within `0.0..=100.0`. |
| `max_near_percent` | `None` | For `check` subcommand: maximum allowed near-duplicate line percentage. Must be within `0.0..=100.0`. |

A value outside the range its field allows fails the run with a message naming the field,
and exits 2. A config file that cannot be read or parsed is passed over, because a project
with no configuration looks the same.

## Ignoring Duplicates

Some duplicates are intentional (e.g., test helpers, trait implementations). You can ignore them by fingerprint:

```sh
# Add a fingerprint to the ignore list
$ cargo dry4rust ignore 396fa8f6b728ff01 --reason "Intentional test helpers"
Added 396fa8f6b728ff01 to ignore list.

# List ignored fingerprints
$ cargo dry4rust ignored
Ignored fingerprints:
  396fa8f6b728ff01 (reason: Intentional test helpers)

# Ignored groups are automatically filtered from reports and checks
$ cargo dry4rust report
# The ignored group will not appear
```

The ignore list is stored in `.dry4rust-ignore.toml` in the project root. When `cleanup`
prunes the last entry it removes the file rather than leaving an empty one behind: an empty
suppression list says exactly what no file says.

Entries whose fingerprint no longer matches anything go stale — after a refactor, or after
an upgrade that changed the fingerprint format. `cleanup` prunes them:

```sh
$ cargo dry4rust cleanup --dry-run   # list them
$ cargo dry4rust cleanup             # remove them
```

## Adopting on a Codebase That Already Has Duplication

`ignore` is for duplication that is *meant* to be there. Duplication nobody has got to yet
is a different thing, and recording it as intentional — one fingerprint at a time, with a
reason invented to fill the field — writes a lie into a file that outlives whoever wrote
it.

Record it as a baseline instead. `check` then fails on what is added, not on what was
inherited:

```sh
# Record what is already there
$ cargo dry4rust baseline
  exact 396fa8f6b728ff01 (3 members: process_data, compute_total, aggregate)
Recorded 1 groups in dry4rust-baseline.json.

# From now on, a zero ceiling is a gate rather than a wall
$ cargo dry4rust --baseline dry4rust-baseline.json check --max-exact 0
...
Baseline: 1 groups suppressed

Check passed.
```

Commit `dry4rust-baseline.json`, or put `baseline = "dry4rust-baseline.json"` in
`dry4rust.toml` so every run picks it up without the flag. `baseline --dry-run` lists what
would be recorded without writing anything.

A baseline records a group's fingerprint **and its member count**. A third copy of an
already-recorded duplicate makes the group larger than what was recorded, so it is
reported — an exact group keeps its fingerprint when a copy joins it, and a baseline keyed
on the fingerprint alone would inherit every future copy. Deleting a copy is admitted:
progress is not something to fail on.

A baseline that cannot be read — missing, malformed, or written by a version with a
different format — fails the run and says so, rather than being treated as empty. Every
summary a baseline touched states how many groups it suppressed, so a stale baseline is
visible rather than silent.

The reasoning is in
[ADR-BaselineIsInheritedNotForgiven](docs/ADRs/ADR-BaselineIsInheritedNotForgiven.md).

## CI Integration

Use the `check` subcommand in CI pipelines:

```yaml
# GitHub Actions example
- name: Check for code duplication
  run: cargo dry4rust check --max-exact 0 --max-exact-percent 5.0
```

On a codebase with existing duplication, add a baseline so the gate measures what the
change introduced:

```yaml
- name: Check for new code duplication
  run: cargo dry4rust --baseline dry4rust-baseline.json check --max-exact 0
```

Exit codes:
- **0** — Check passed (within thresholds)
- **1** — Check failed (thresholds exceeded)
- **2** — Error (no source files, invalid path, etc.)

## What Gets Analyzed

| Code Unit | Description |
|-----------|-------------|
| **Functions** | Top-level `fn` items |
| **Methods** | `fn` items inside `impl` blocks |
| **Trait impls** | `fn` items inside `impl Trait for Type` blocks |
| **Closures** | Closure expressions (above the min node threshold) |

The scanner automatically:
- Skips `target/` directories
- Skips hidden directories (starting with `.`)
- Respects exclude patterns
- Handles parse errors gracefully (skips unparseable files with a warning)

## Documentation

| Where | What |
|---|---|
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | The pipeline, the components, and the data model |
| [docs/FORMULA.md](docs/FORMULA.md) | Fingerprinting and the similarity score, precisely |
| [docs/ADRs/](docs/ADRs/README.md) | The load-bearing decisions and why they were forced |
| [docs/OPEN_POINTS.md](docs/OPEN_POINTS.md) | Where the model is known to be thin, including two silent false-negative sources |
| [docs/ROADMAP.md](docs/ROADMAP.md) | Direction, current baseline, and what is left |
| [docs/IMPLEMENTED-FEATURES.md](docs/IMPLEMENTED-FEATURES.md) | What each version added, upstream's included |
| [docs/dry4rust-dossier.md](docs/dry4rust-dossier.md) | Research on where this tool is going, and what already holds the ground |


## Development

**Requirements:** Rust 1.85+ (edition 2024)

The repository is a workspace: `core/` is the published crate, `validation/` holds the
end-to-end tests, and `fixture/` is the corpus both are pointed at.

```sh
cargo build          # Build
cargo test           # Run the suite
cargo clippy         # Lint check
cargo fmt --check    # Format check
```

Both gates are scripted, and both are mandatory after any change under `src/` or `tests/`:

```powershell
powershell -File scripts\run_stage_1.ps1   # fmt, clippy under -D warnings, tests
powershell -File scripts\run_stage_2.ps1   # stern4rust, crap4rust, twin4rust, iceberg4rust
```

Stage 2 needs the four subcommands installed:

```sh
cargo install cargo-stern4rust cargo-crap4rust cargo-twin4rust cargo-iceberg4rust
```
