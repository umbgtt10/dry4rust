# Architecture

`cargo-dry4rust` answers one question: which pieces of this codebase are the
same piece written twice.

It answers it structurally. Source is parsed to an AST, the AST is normalised
until incidental differences disappear, and what remains is hashed for exact
matches and compared tree-to-tree for near ones. Nothing is inferred from
names, comments or formatting, because all three are exactly what a copied
block changes first.

60 source files, roughly 5,500 lines, one type each. Four directories group
the modules that answer one question between them: `src/near_duplicate/`,
which only `grouper` reaches; `src/baseline/`, the record of inherited
duplication; `src/cli/`, one file per command; and `src/rust/`, the only
`LanguageAnalyzer` there is.

## Pipeline

```
  scanner        walk the tree, honour excludes and extensions
     |
     v
  rust::parser   syn -> one CodeUnit per fn / method / closure
     |
     v
  rust::normalizer
                 identifiers -> positional placeholders
                 literals    -> type-preserving placeholders
                 types       -> positional placeholders
                 control flow, preserved exactly
                 macros      -> opaque nodes keyed by name
     |
     +--> node_encoder --> stable_hasher      the fingerprint
     |
     v
  grouper
     |  group_exact_duplicates    equal fingerprints
     |  NearDuplicateFinder       PairScanner picks the pairs worth scoring,
     |                            UnionFind closes them into groups
     v
  ignore         drop fingerprints listed in .dry4rust-ignore.toml
     |
     v
  baseline       drop groups a recorded baseline already accounts for,
     |           and count how many that was
     v
  output         text or json
```

The two filters are not the same thing. `ignore` drops duplication somebody
decided should stay; `baseline` drops duplication nobody has got to yet. Both
run before the statistics, so a ceiling on a percentage measures what is left.

Sub-function analysis, when enabled, re-enters at the normaliser: each
function body yields its own if-branches, match arms, loop bodies and closure
bodies as further units, which then travel the same path.

## Components

| module | responsibility |
|---|---|
| `scanner` | Walk a root, apply exclude globs and extension filters. |
| `analyzer` | `LanguageAnalyzer`, the trait every backend implements. One implementation today. |
| `rust::parser` | `syn` -> `CodeUnit`, one per function, method and closure. |
| `rust::normalizer` | The normalisation rules, split by syntax category: `expr`, `pat`, `helpers`, `normalize`. |
| `normalization_context` | The placeholder counters. Two identifiers map to the same placeholder only within one context. |
| `node` | `NormalizedNode` and `NodeKind` -- the normalised tree, plus `count_nodes` and `reindex_placeholders`. |
| `fingerprint` | `Fingerprint`, a hashed `NormalizedNode`. |
| `node_encoder` | The byte format a fingerprint is computed over -- variants by name, children counted. |
| `stable_hasher` | FNV-1a 64, defined here so a fingerprint survives a toolchain upgrade. |
| `extractor` / `sub_unit_extractor` | Compound structures inside a body, as further comparable units. |
| `grouper` | `DuplicateGroup`, `DuplicationStats`, exact grouping. |
| `near_duplicate::near_duplicate_finder` | Candidate filtering and transitive closure into groups. |
| `near_duplicate::pair_scanner` | Which pairs are worth scoring, and their scores. |
| `near_duplicate::similarity` | The Dice score between two normalised trees. |
| `near_duplicate::union_find` | Disjoint-set forest turning similar-pairs into groups. |
| `near_duplicate::similarity_pair` | One scored pair, and the ordered key a symmetric lookup needs. |
| `ignore` | The suppression file: read, write, filter. |
| `baseline::baseline_kind` | Which of the four group sets an entry was recorded from. |
| `baseline::baseline_entry` | One recorded group: kind, fingerprint, member count, names. |
| `baseline::baseline_file` | The file itself -- record, load, save, and where it lives. |
| `baseline::baseline_filter` | Keeps the groups the baseline does not already account for. |
| `threshold` | `Threshold`, a proportion that cannot be built out of range. |
| `config` | Defaults, `dry4rust.toml`, `[package.metadata.dry4rust]`, CLI overrides. |
| `analysis` | `analyze` and `analyze_units` -- the pipeline as one call. |
| `cli::cli_error` | `CliError` and the exit code each variant maps to. |
| `cli::command` | The subcommands, as `clap` sees them. |
| `cli::cli_overrides` | What the command line said, applied over what the files said. |
| `cli::output_format` | Text or JSON, and the reporter each builds. |
| `cli::analysis_output` | One run's config, result and reporter, produced together. |
| `cli::*_command` | One struct per subcommand, each with a single `run`. |
| `cli::checking` | `Ceiling`, `CheckThresholds`, `StaleReport` -- what `check` and `cleanup` measure with. |
| `command_dispatcher` | Routes a parsed `Command` to the type that serves it. |
| `output` | `Reporter`, with `text` and `json` implementations. |

## Data model

`CodeUnit` is what the pipeline carries:

```rust
pub struct CodeUnit {
    pub kind: CodeUnitKind,       // Function, Method, Closure, IfBranch, MatchArm, LoopBody, Block
    pub name: String,
    pub file: PathBuf,
    pub line_start: usize,
    pub line_end: usize,
    pub signature: NormalizedNode,
    pub body: NormalizedNode,
    pub fingerprint: Fingerprint,
    pub node_count: usize,
    pub parent_name: Option<String>,   // set for sub-function units
    pub is_test: bool,
}
```

`NormalizedNode` is a `NodeKind` plus children. It is the only thing compared;
the original source is kept only as line numbers, for reporting.

`DuplicateGroup` is a fingerprint, its members, and a similarity -- `1.0` for
exact groups, the weakest pair's score for near ones.

## Normalisation is the whole idea

Two functions are the same duplicate if they normalise to the same tree. What
normalisation erases decides what counts as a duplicate, so the rules are the
tool's actual opinion:

- **Identifiers** become positional placeholders, so `foo(x)` and `bar(y)`
  agree.
- **Literals** are erased but their type is kept, so `42` and `99` agree while
  `42` and `"42"` do not.
- **Types** become positional placeholders within a context.
- **Control flow** is preserved exactly. An `if` is never an `unwrap_or`.
- **Macros** become opaque nodes keyed by name. Two `println!` calls agree
  with each other and with nothing else.

Placeholders are positional, which means they only mean anything relative to
one `NormalizationContext`. `reindex_placeholders` re-bases a sub-tree so an
extracted fragment can be compared against another extracted fragment.

## CLI layer

`main` unpacks arguments and produces an exit code. Everything else is in the
library, where tests reach it: `CommandDispatcher` splits the two commands that
touch only the ignore file from the five that need an analysis first, runs the
analysis once, and hands the result to the command struct that serves it.

Each command is a type with a constructor and a single `run`, taking a writer
rather than printing, so each is driven directly against a fixture in tests and
its output read back. `baseline` is the one command dispatched through
`AnalysisOutput::produce_ignoring_baseline`: a recording that judged against
the previous recording would hold only what had been added since, and the
second run would empty the file.

## Related

- [FORMULA.md](FORMULA.md) -- the scoring, precisely
- [ADRs/](ADRs/README.md) -- why the load-bearing pieces are shaped this way
- [OPEN_POINTS.md](OPEN_POINTS.md) -- where the model is known to be thin
