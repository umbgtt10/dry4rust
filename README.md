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

The engine is upstream's and works. The surrounding repository is under this family's house
rules and gates: `cargo stern4rust` reports no offences with all twenty-one rules applied,
`cargo crap4rust` finds no function at or above 15, and every source file has a mirrored
test file. Commands below are `cargo dry4rust`; the upstream `cargo dry4rust` spelling is gone.


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
  stats    Show duplication statistics only
  report   Show full duplication report (default)
  check    Check for duplicates and exit with non-zero if thresholds exceeded
  ignore   Add a fingerprint to the ignore list
  ignored  List all ignored fingerprints

Options:
  -p, --path <PATH>            Path to analyze (defaults to current directory)
      --min-nodes <MIN_NODES>  Minimum AST node count for analysis [default: 10]
      --min-lines <MIN_LINES>  Minimum source line count for analysis [default: 0 (disabled)]
      --threshold <THRESHOLD>  Similarity threshold for near-duplicates (0.0-1.0) [default: 0.8]
      --format <FORMAT>        Output format [default: text] [possible values: text, json]
      --exclude <EXCLUDE>      Exclude patterns (can be repeated)
      --exclude-tests          Exclude test code (#[test] functions and #[cfg(test)] modules)
```

### Examples

**Full report:**

```sh
$ cargo dry4rust report
Duplication Statistics
=====================
Total code units analyzed: 4

Exact duplicates: 1 groups (2 code units)
Near duplicates:  0 groups (0 code units)

Duplicated lines (exact): 18
Duplicated lines (near):  0
Duplication: 50.0% exact, 0.0% near (of 36 total lines)

Exact Duplicates
================

Group 1 (fingerprint: 2a182da9e04e9428, 2 members):
  - sum_positive (function) at src/lib.rs:2-10
  - count_positive (function) at src/lib.rs:12-20
```

**Statistics only:**

```sh
$ cargo dry4rust stats
Duplication Statistics
=====================
Total code units analyzed: 4

Exact duplicates: 1 groups (2 code units)
Near duplicates:  0 groups (0 code units)

Duplicated lines (exact): 18
Duplicated lines (near):  0
Duplication: 50.0% exact, 0.0% near (of 36 total lines)
```

**JSON output:**

```sh
$ cargo dry4rust --format json stats
{
  "total_code_units": 4,
  "total_lines": 36,
  "exact_duplicate_groups": 1,
  "exact_duplicate_units": 2,
  "near_duplicate_groups": 0,
  "near_duplicate_units": 0,
  "exact_duplicate_lines": 18,
  "near_duplicate_lines": 0,
  "exact_duplicate_percent": 50.0,
  "near_duplicate_percent": 0.0
}
```

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
| `similarity_threshold` | `0.8` | Minimum similarity score (0.0-1.0) for near-duplicate detection. |
| `exclude` | `[]` | Path patterns to exclude from scanning (substring match). |
| `exclude_tests` | `false` | Exclude `#[test]` functions and `#[cfg(test)]` modules from analysis. |
| `max_exact_duplicates` | `None` | For `check` subcommand: maximum allowed exact duplicate groups. |
| `max_near_duplicates` | `None` | For `check` subcommand: maximum allowed near-duplicate groups. |
| `max_exact_percent` | `None` | For `check` subcommand: maximum allowed exact duplicate line percentage. |
| `max_near_percent` | `None` | For `check` subcommand: maximum allowed near-duplicate line percentage. |

## Ignoring Duplicates

Some duplicates are intentional (e.g., test helpers, trait implementations). You can ignore them by fingerprint:

```sh
# Add a fingerprint to the ignore list
$ cargo dry4rust ignore 2a182da9e04e9428 --reason "Intentional test helpers"
Added 2a182da9e04e9428 to ignore list.

# List ignored fingerprints
$ cargo dry4rust ignored
Ignored fingerprints:
  2a182da9e04e9428 (reason: Intentional test helpers)

# Ignored groups are automatically filtered from reports and checks
$ cargo dry4rust report
# The ignored group will not appear
```

The ignore list is stored in `.dry4rust-ignore.toml` in the project root.

## CI Integration

Use the `check` subcommand in CI pipelines:

```yaml
# GitHub Actions example
- name: Check for code duplication
  run: cargo dry4rust check --max-exact 0 --max-exact-percent 5.0
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

```sh
cargo build          # Build
cargo test           # Run all 147 tests
cargo clippy         # Lint check
cargo fmt --check    # Format check
```

Pre-commit hooks (via `cargo-husky`) run clippy and rustfmt automatically.
