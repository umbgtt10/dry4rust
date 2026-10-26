# Architecture

`cargo-dry4rust` answers one question: which pieces of this codebase are the
same piece written twice.

It answers it structurally. Source is parsed to an AST, the AST is normalised
until incidental differences disappear, and what remains is hashed for exact
matches and compared tree-to-tree for near ones. Nothing is inferred from
names, comments or formatting, because all three are exactly what a copied
block changes first.

Two published crates. `core/` -- `dry4rust-core`, library `dry4rust` -- is
the analysis: `#![no_std]`, `#![forbid(unsafe_code)]`, depending on proc-macro2,
syn and thiserror only, and touching no file system, process, environment or
format. It declares a trait for everything it needs from the machine, and
`cli/` -- `cargo-dry4rust`, library `dry4rust_cli` -- implements those traits,
parses the command line, prints, and returns the exit code. `cli` depends on
`core`, never the reverse.

`core/src` holds 84 source files, roughly 6,900 lines, one object each and
records where `record-home` puts them. Five directories group the modules that
answer one question between them: `near_duplicate/`, which only
`duplicate_group` reaches; `cli/`, one file per command, with `checking/`,
`output/` -- the text reporter -- and `suppression/`, the two ways duplication
is kept out of a report, under it; `node/`, the normalised tree, the kinds its
nodes are named by, and the encoder and the placeholder collector that read it;
`rust/`, the only `LanguageAnalyzer` there is; and `traits/`, the ports.

The house rules allow twenty files at the top of `src/` and five directories
under it, and both are full: nineteen files beside `lib.rs`, and `traits/` took
the fifth directory slot, freed by moving `suppression/` under `cli/`. The next
file or directory forces a grouping decision rather than another loose one,
which is what the limits are for.

## Pipeline

```
  scanner        the rules of the walk -- which directories to enter, which
     |           files to keep -- asked by the SourceTree as it walks
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
  ignore         drop fingerprints listed in .dry4rust-ignore.toml,
     |           loaded through the IgnoreStore before the run
     v
  baseline       drop groups a recorded baseline already accounts for,
     |           loaded through the BaselineStore, and count how many that was
     v
  output         a Reporter: text in the core, json in cli
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
| `scanner` | `Scanner`: the rules of the walk -- `descends_into` refuses `target/` and a hidden directory below the root, `admits` takes the extensions and the exclusions -- and `scan_files`, which hands them to a `SourceTree`. |
| `source_path` | `SourcePath`, a path held as text with `/` as its only separator, compared by component and shown as it was given. |
| `storage_failure` | `StorageFailure`, what a port's storage said: `Missing` or `Failed`, in the platform's words. |
| `traits::source_tree` | `SourceTree`: the source files under a root, and a file's text. |
| `traits::config_source` | `ConfigSource`: what a `dry4rust.toml`, or a `Cargo.toml`'s metadata table, states. |
| `traits::ignore_store` | `IgnoreStore`: the ignore file's load, save and remove. |
| `traits::baseline_store` | `BaselineStore`: a baseline's load and save. |
| `traits::reporter` | `Reporter`: five section methods, plus `report` and `report_check` for a whole document, failing with `RenderFailure`. |
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
| `cli::suppression::ignore_entry` | One duplicate somebody decided should stay, with their reason. |
| `cli::suppression::ignore_file` | The ignore file: load and save through its store, add, prune, filter; an unreadable one is empty, an emptied one is removed. |
| `cli::suppression::baseline_kind` | Which of the four group sets an entry was recorded from. |
| `cli::suppression::baseline_entry` | One recorded group: kind, fingerprint, member count, names. |
| `cli::suppression::baseline_file` | The file itself -- record, load and save through its store, the format it must be in, and where it lives. |
| `cli::suppression::baseline_filter` | Keeps the groups the baseline does not already account for. |
| `threshold` | `Threshold`, a proportion that cannot be built out of range. |
| `config` | `Config`: what a run is configured with, assembled from a `ConfigSource` -- the manifest's metadata first, `dry4rust.toml` over it -- and range-checked. |
| `file_config` | What `dry4rust.toml` and `[package.metadata.dry4rust]` may state, a `with_` step per field. |
| `analysis_config` | The two floors a language analyzer needs, and nothing else. |
| `analysis` | `Analysis`, what a run returns, and `Analysis::analyze` and `Analysis::analyze_units` -- the pipeline as one call. |
| `cli::cli_error` | `CliError` and the exit code each variant maps to. |
| `cli::command` | The commands, as the core dispatches them. |
| `cli::command_outcome` | What a command produced: the text to print, the verdict that sets the exit code once it is printed, and the analysis's warnings. |
| `cli::cli_overrides` | What the command line said, applied over what the files said. |
| `cli::ports` | `Ports`, the four storage ports a run is composed with, handed over in one piece. |
| `cli::analysis_output` | One run's config and result, produced together: scan, load the suppressions, analyse. |
| `cli::*_command` | One struct per subcommand, each with a single `run`. |
| `cli::checking` | `Ceiling`, `CheckThresholds`, `StaleReport` -- what `check` and `cleanup` measure with. |
| `command_dispatcher` | Routes a parsed `Command` to the type that serves it. |
| `cli::output::render_failure` | `RenderFailure`, a reporter's reason for not writing its document. |
| `cli::output::report` | What a full report shows, handed over in one piece. |
| `cli::output::check_breach` | One exceeded ceiling and the groups behind it. |
| `cli::output::group_section` | One headed section of a text report, and how its groups read. |
| `cli::output::display_path` | `DisplayPath::of`, a file as a reporter shows it: relative to the base when it lies under one. |
| `duplication_stats` | `DuplicationStats`, the counts and percentages a report states. |
| `cli::output::text_reporter` | `TextReporter`, the core's implementation of `Reporter`. |

And in `cli/src`, the crate that talks to the machine:

| module | responsibility |
|---|---|
| `invocation::args` | `Args`, the command line and what it means: which root, which command, which overrides, which format. |
| `invocation::cli_command` | `CliCommand`, the subcommands as `clap` sees them, and the core's `Command` each one is. |
| `invocation::output_format` | Text or JSON, and the reporter each builds. |
| `invocation::entry_point` | `EntryPoint`, the composition root: the adapters in `Ports`, the dispatch, and the only printing. |
| `system::walkdir_source_tree` | `WalkdirSourceTree`, the walk on disk, asking the scanner per directory and per file. |
| `system::toml_config_source` | `TomlConfigSource`, the two configuration files read as TOML. |
| `system::toml_ignore_store`, `ignore_document`, `ignore_entry_record` | The ignore file on disk, in TOML, over records of its own. |
| `system::json_baseline_store`, `baseline_document`, `baseline_entry_record`, `baseline_kind_record` | A baseline on disk, in JSON, over records of its own; the kind a serde mirror of the core's enum. |
| `system::json_reporter` | `JsonReporter`, the JSON implementation of `Reporter`. |
| `system::disk_file` | `DiskFile`, one file read, written or removed, with what the platform says as a `StorageFailure`. |
| `system::portable_path` | `PortablePath`, a platform path or pattern as the core's, `\` read as `/` on Windows; a path still shows as given. |

## Data model

`CodeUnit` is what the pipeline carries:

```rust
pub struct CodeUnit {
    pub kind: CodeUnitKind,       // Function, Method, Closure, IfBranch, MatchArm, LoopBody, Block
    pub name: String,
    pub file: SourcePath,
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

`cli`'s `main` is eleven lines: it hands `argv` to `EntryPoint::run` and
returns the `ExitCode` that comes back. `Args` maps the arguments to a run --
which root, which command, which overrides, which format -- and `EntryPoint`
composes it: the analyzer, the root as a `SourcePath`, the reporter `--format`
names, and the four adapters in `Ports`. The core's `CommandDispatcher` splits
the two commands that touch only the ignore file from the five that need an
analysis first, runs the analysis once, and hands the result and that reporter
to the command struct that serves it.

Each command is a type with a constructor and a single `run`, returning a
`CommandOutcome` -- the text it means to print and the verdict that sets the
exit code -- rather than printing, so each is driven directly against
in-memory fakes in tests and its output read back. `EntryPoint` is the one
place that writes: the analysis's warnings to stderr, then the text to
stdout, and only then does it act on the verdict, which is how `check` prints
its report and still exits 1. `baseline` is the one command dispatched through
`AnalysisOutput::produce_ignoring_baseline`: a recording that judged against
the previous recording would hold only what had been added since, and the
second run would empty the file.

## Related

- [FORMULA.md](FORMULA.md) -- the scoring, precisely
- [ADRs/](ADRs/README.md) -- why the load-bearing pieces are shaped this way
- [OPEN_POINTS.md](OPEN_POINTS.md) -- where the model is known to be thin
