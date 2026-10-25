# Architecture

`cargo-dry4rust` answers one question: which pieces of this codebase are the
same piece written twice.

It answers it structurally. Source is parsed to an AST, the AST is normalised
until incidental differences disappear, and what remains is hashed for exact
matches and compared tree-to-tree for near ones. Nothing is inferred from
names, comments or formatting, because all three are exactly what a copied
block changes first.

79 source files, roughly 6,800 lines, one object each and records where
`record-home` puts them. Five directories group the modules that answer one
question between them: `src/near_duplicate/`, which only `duplicate_group`
reaches; `src/suppression/`, the two ways duplication is kept out of a report;
`src/cli/`, one file per command, with `checking/` and `output/` -- the
reporters, which only the command line uses -- under it; `src/node/`, the
normalised tree, the kinds its nodes are named by, and the encoder and the
placeholder collector that read it; and `src/rust/`, the only
`LanguageAnalyzer` there is.

The house rules allow twenty files at the top of `src/` and five directories
under it. The node kinds, which the library's public API names, each needed a
file of their own; `src/node/` took the fifth directory slot, freed by moving
`output/` under `cli/`, and the top holds eighteen files. The next directory
forces a grouping decision rather than another loose one, which is what the
limits are for.

## Pipeline

```
  scanner        walk the tree, honour excludes and extensions
     |
     v
  rust::parser   syn -> one CodeUnit per fn / method / closure,
                 visited by rust::code_unit_extractor
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
  duplicate_group
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
| `scanner` | `Scanner`: walk a root, apply exclude globs and extension filters. |
| `analyzer` | `LanguageAnalyzer`, the trait every backend implements. One implementation today. |
| `rust::parser` | `syn` -> `CodeUnit`, one per function, method and closure. |
| `rust::code_unit_extractor` | `CodeUnitExtractor`, the visitor that walks a parsed file and keeps each unit over the floors. |
| `rust::normalizer` | The normalisation rules, and the normalised function they return. |
| `rust::normalizer::normalization_context` | `NormalizationContext`: every normalisation rule, a method per syntax category, and the placeholder counters they share, so the context that numbers the placeholders is `self` rather than a `&mut` threaded through every call. Two identifiers map to the same placeholder only within one context. |
| `rust::normalizer::normalized_fn` | `NormalizedFn`, a normalised function's signature and body, named and kept apart. |
| `node::normalized_node` | `NormalizedNode`, the normalised tree, with `NormalizedNode::count_nodes` and `NormalizedNode::reindex_placeholders`. |
| `node::node_kind` and its kinds | `NodeKind`, and `LiteralKind`, `PlaceholderKind`, `BinOpKind` and `UnOpKind` in `node::literal_kind`, `node::placeholder_kind`, `node::bin_op_kind` and `node::un_op_kind` -- the vocabulary a node is named in. |
| `node::placeholder_order_collector` | The placeholders of a tree in first-occurrence order, which is what `reindex_placeholders` renumbers by. |
| `fingerprint` | `Fingerprint`, a hashed `NormalizedNode`. |
| `node::node_encoder` | The byte format a fingerprint is computed over -- variants by name, children counted. |
| `stable_hasher` | FNV-1a 64, defined here so a fingerprint survives a toolchain upgrade. |
| `sub_unit` / `sub_unit_extractor` | Compound structures inside a body, as further comparable units. |
| `duplicate_group` | `DuplicateGroup`, exact grouping, and the statistics computed over the groups. |
| `near_duplicate::near_duplicate_finder` | Candidate filtering and transitive closure into groups. |
| `near_duplicate::pair_scanner` | Which pairs are worth scoring, and their scores. |
| `near_duplicate::similarity` | The Dice score between two normalised trees. |
| `near_duplicate::union_find` | Disjoint-set forest turning similar-pairs into groups. |
| `near_duplicate::similarity_pair` | One scored pair, and the ordered key a symmetric lookup needs. |
| `suppression::ignore_entry` | One duplicate somebody decided should stay, with their reason. |
| `suppression::ignore_file` | The ignore file: read, write, add, prune, filter. |
| `suppression::baseline_kind` | Which of the four group sets an entry was recorded from. |
| `suppression::baseline_entry` | One recorded group: kind, fingerprint, member count, names. |
| `suppression::baseline_file` | The file itself -- record, load, save, and where it lives. |
| `suppression::baseline_filter` | Keeps the groups the baseline does not already account for. |
| `threshold` | `Threshold`, a proportion that cannot be built out of range. |
| `config` | `Config`: what a run is configured with, assembled and range-checked. |
| `file_config` | What `dry4rust.toml` and `[package.metadata.dry4rust]` may state. |
| `analysis_config` | The two floors a language analyzer needs, and nothing else. |
| `analysis` | `Analysis`, what a run returns, and `Analysis::analyze` and `Analysis::analyze_units` -- the pipeline as one call. |
| `cli::cli_error` | `CliError` and the exit code each variant maps to. |
| `cli::command` | The subcommands, as `clap` sees them. |
| `cli::command_outcome` | What a command produced: the text to print, and the verdict that sets the exit code once it is printed. |
| `cli::entry_point` | The command line and what it means: which root, which command, which overrides. |
| `cli::cli_overrides` | What the command line said, applied over what the files said. |
| `cli::output_format` | Text or JSON, and the reporter each builds. |
| `cli::analysis_output` | One run's config and result, produced together. |
| `cli::*_command` | One struct per subcommand, each with a single `run`. |
| `cli::checking` | `Ceiling`, `CheckThresholds`, `StaleReport` -- what `check` and `cleanup` measure with. |
| `command_dispatcher` | Routes a parsed `Command` to the type that serves it. |
| `cli::output::reporter` | `Reporter`: five section methods, plus `report` and `report_check` for a whole document. |
| `cli::output::report` | What a full report shows, handed over in one piece. |
| `cli::output::check_breach` | One exceeded ceiling and the groups behind it. |
| `cli::output::group_section` | One headed section of a text report, and how its groups read. |
| `cli::output::display_path` | `DisplayPath::of`, a file as a reporter shows it: relative to the base when it lies under one. |
| `duplication_stats` | `DuplicationStats`, the counts and percentages a report states. |
| `cli::output` | `text_reporter` and `json_reporter`, the implementations of `Reporter`. |

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

`main` is eleven lines: it hands `argv` to `EntryPoint::run` and returns the
`ExitCode` that comes back. The mapping from arguments to a run lives in the
library, so a test can call it, and everything after it is there too:
`EntryPoint` composes the run -- the analyzer, the root, and the reporter
`--format` names -- and `CommandDispatcher` splits the two commands that touch
only the ignore file from the five that need an analysis first, runs the
analysis once, and hands the result and that reporter to the command struct
that serves it.

Each command is a type with a constructor and a single `run`, returning a
`CommandOutcome` -- the text it means to print and the verdict that sets the
exit code -- rather than printing, so each is driven directly against a fixture
in tests and its output read back. `EntryPoint` is the one place that writes to
stdout, and it writes the text before it acts on the verdict, which is how
`check` prints its report and still exits 1. `baseline` is the one command dispatched through
`AnalysisOutput::produce_ignoring_baseline`: a recording that judged against
the previous recording would hold only what had been added since, and the
second run would empty the file.

## Related

- [FORMULA.md](FORMULA.md) -- the scoring, precisely
- [ADRs/](ADRs/README.md) -- why the load-bearing pieces are shaped this way
- [OPEN_POINTS.md](OPEN_POINTS.md) -- where the model is known to be thin
