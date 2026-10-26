# A technology-free core and a cli crate

Status: **in progress** -- one commit per slice, in the order below.

## Why

Mutation testing pays for every mutant with a build and a test run. A mutant in
code that walks a disk, reads and writes TOML and JSON, or parses a command line
is slow to kill, and a test that needs a temporary directory is slow to run:
today sixty of core's tests write to disk, round-trip a serialised format or
parse arguments with clap, and clap, serde, toml and walkdir are linked into
every test binary. So the analysis, the grouping, the suppression rules, the
commands and the text report move into a crate that touches no technology at
all, and mutation testing is scoped to that crate alone.

## The shape

The workspace grows a member:

| member | package | library | what it holds |
|---|---|---|---|
| `core/` | `dry4rust-core` | `dry4rust` | `#![no_std]`, `#![forbid(unsafe_code)]`; normalisation, fingerprints, grouping, the scanner's rules, configuration precedence, the ignore file's and the baseline's rules, the commands and the text report. Depends on `proc-macro2`, `syn` and `thiserror` only |
| `cli/` | `cargo-dry4rust` | `dry4rust_cli` | the binary, `Args` and `CliCommand` (clap), the composition root `EntryPoint`, the JSON reporter, and every adapter: the walk, the files on disk, TOML and JSON |
| `validation/` | `validation` | `validation` | the end-to-end scenarios, now composing cli's adapters |
| `xtask/` | `xtask` | `xtask` | the gates |

`cli` depends on `core`, never the reverse. Both published crates share one
workspace version. `fixture/` stays outside the workspace.

### Ports

Core declares a trait for everything technological it needs, implemented in
`cli/src/system/` and, for core's own tests, by in-memory fakes under
`core/tests/support/`:

| port (`core/src/traits/`) | what it answers | cli adapter |
|---|---|---|
| `SourceTree` | the source files under a root -- asking the scanner which directories to enter and which files to keep -- and a file's text | `WalkdirSourceTree` |
| `ConfigSource` | what a `dry4rust.toml`, or a `Cargo.toml`'s `[package.metadata.dry4rust]`, states | `TomlConfigSource` |
| `IgnoreStore` | the ignore file: load, save, remove | `TomlIgnoreStore` |
| `BaselineStore` | a baseline: load, save | `JsonBaselineStore` |
| `Reporter` | the reports as text | `TextReporter` in core; `JsonReporter` in cli |

The four storage ports travel together in a `Ports` record. Core works on
`SourcePath` -- a path held as text with `/` as its only separator, which cli
converts to at the edge -- and every port fails with one `StorageFailure`
(`Missing` or `Failed`, in the platform's words). The rules stay in core: which
file wins, that an unreadable ignore file suppresses nothing and an emptied one
is removed, what a missing baseline says, which format a baseline must be in,
where it lives. Core returns its output and its warnings as strings; printing
them and turning the verdict into an exit code is `EntryPoint`'s.

The binary's behaviour does not change: stdout, stderr, exit codes and the
files the writing commands leave are compared byte for byte against `b31efba`
after every slice that touches them. Exceptions, deliberate: a `.rs` file the
scanner keeps whose path is not UTF-8 fails the run naming it, where today it is
analysed under a name it does not have; on Windows, paths in the output show
`/`, and an exclusion pattern matches with either separator.

### The gate

A stage-2 gate, first in the run, keeps core technology-free: its manifest may
name only `proc-macro2`, `syn` and `thiserror` in any section, its crate root
must carry `#![no_std]` and `#![forbid(unsafe_code)]`, and no file under
`core/src` or `core/tests` may reach `std::{env,fs,io,net,os,process}`, the
printing macros, the file-system methods or `extern crate std`.

### Mutation testing

`.cargo/mutants.toml` scopes cargo-mutants to `core/src`; each mutant runs
core's tests only.

## Slices

| # | slice | status |
|---|---|---|
| S0 | this record | done |
| S1 | the duplication gate: one run from the root, the trees it must not judge excluded by full path | |
| S2 | the split: `cli` takes `main.rs`, the command line (`Args`, `CliCommand`, `OutputFormat`) and a new `EntryPoint`; core is renamed and keeps a plain `Command`; one workspace version; every member forbids unsafe code | |
| S3 | the regroup: `suppression/` moves under core's `cli/`, so `traits/` fits beside the other subfolders | |
| S4 | the `SourceTree` port, `SourcePath` and `StorageFailure`; the scanner keeps its rules and the walk becomes cli's `WalkdirSourceTree` | |
| S5 | the `ConfigSource`, `IgnoreStore` and `BaselineStore` ports and `Ports`; TOML and JSON files on disk go to cli | |
| S6 | the `Reporter` port moves to `traits/` with `RenderFailure`; the JSON reporter goes to cli; warnings travel in the outcome | |
| S7 | `#![no_std]`: alloc imports, ordered collections, thiserror without std | |
| S8 | the technology-free gate, first in stage 2 | |
| S9 | `.cargo/mutants.toml` | |
| S10 | docs, and this record moves to `docs/history/` | |

## Tests that move

No test is deleted, and every moved test keeps its name and its assertions.
Tests whose subject moves go with it:

| from | to |
|---|---|
| `core/tests/cli/entry_point_tests.rs` (11) | `cli/tests/invocation/args_tests.rs` (8) and `entry_point_tests.rs` (the three `run_*`) |
| `core/tests/cli/command_tests.rs` (6) | `cli/tests/invocation/cli_command_tests.rs` |
| `core/tests/cli/output_format_tests.rs` (3) | `cli/tests/invocation/output_format_tests.rs` |
| `config_tests`: the 11 that write a configuration file | `cli/tests/system/toml_config_source_tests.rs` |
| `baseline_file_tests` (4) and `baseline_command_tests` (2) that read or write the disk | `cli/tests/system/json_baseline_store_tests.rs` |
| `cleanup_command_tests` (3), `ignore_command_tests::run_with_a_malformed_fingerprint_is_rejected`, `ignore_file_tests` (2) that read the disk | `cli/tests/system/toml_ignore_store_tests.rs` |
| `ignore_entry_tests` (3) and `baseline_kind_tests` (2) that round-trip a format | `cli/tests/system/ignore_entry_record_tests.rs`, `baseline_kind_record_tests.rs` |
| `json_reporter_tests` (9), `report_tests` (2 JSON), `command_dispatcher_tests::dispatch_with_stats_writes_through_the_reporter_it_was_given` | `cli/tests/system/json_reporter_tests.rs` |

Each moved test runs the core against the real adapter. Core gains tests of the
same contracts against its fakes. The tests that stay keep their names and
assertions and arrange in-memory fakes instead of temporary directories; path
assertions compare `SourcePath`s. Two Arrange values change because what they
built leaves core: `from_an_io_error_keeps_it_as_the_source` builds a
`StorageFailure` -- the I/O failure as core holds it -- instead of an
`io::Error`, and `baseline_path_leaves_an_absolute_name_alone` names a drive
path, `C:/projects/recorded.json`, instead of the crate's directory.
