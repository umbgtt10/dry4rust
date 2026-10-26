# ADR-TechnologyFreeCore

## Status

- **Status:** Accepted
- **Date:** 2026-10-26

## Context

The tool is meant to be mutation-tested, and mutation testing pays for every
mutant with a build and a test run. While one crate held everything, a mutant
anywhere rebuilt a crate that linked clap, serde, serde_json, toml and walkdir,
and sixty of its tests wrote temporary directories, round-tripped TOML and JSON
or parsed a command line. The analysis -- normalisation, fingerprints, grouping,
the scanner's rules, which configuration wins, what the ignore file and the
baseline admit, the commands and the text report -- is what deserves mutating,
and it was mixed with code whose only job is to talk to the machine.

## Decision

The analysis lives in a technology-free crate, `dry4rust-core` (library
`dry4rust`): `#![no_std]` and `#![forbid(unsafe_code)]`, depending on
proc-macro2, syn and thiserror only, declaring a trait for every piece of
technology it needs, and returning its output and its warnings as strings. A
separate crate, `cargo-dry4rust` (library `dry4rust_cli`), implements those
traits with the real technology, parses the command line, prints, and decides
the exit code.

The ports are `SourceTree` (the walk and a file's text, asking the core's
`Scanner` which directories to enter and which files to keep), `ConfigSource`,
`IgnoreStore` and `BaselineStore` -- the three files a run reads and writes,
each in the format cli owns -- travelling together in `Ports`, and the
`Reporter` that already separated a run from its format: the text one in the
core, the JSON one in cli. Every port fails with `StorageFailure`, `Missing` or
`Failed` in the platform's words, so the rules that tell the two apart stay in
the core: an absent ignore file is an empty one, an absent baseline is an error
that names the command to record one.

## Forcing constraints / Evidence

`cargo mutants --list` names 989 mutants, all in the core; the workspace held
1,098 before the split, 953 of them in the single crate, technology included.
The core's 510 tests touch no disk and start no process. Measured on two fresh
copies in one build slot, at a load average around five, one expression changed
in `StableHasher::write_bytes` as cargo-mutants changes it: building the core's
tests went from 1.43-1.95 s to 1.06-1.40 s, a cold build of them -- what each
cargo-mutants worker pays first -- from 19.3 s to 9.9 s, and running them from
0.29 s to 0.23 s, from a test binary of 34.8 MB where it was 61.0 MB. Swapping
the hashed collections for ordered ones cost nothing measurable on syn's own
source.

The binary's behaviour did not move: stdout, stderr, exit codes and the files
the writing commands leave were compared byte for byte against `b31efba` on 56
invocations after every step.

## Rejected alternatives

- **One crate, with cargo-mutants' `exclude_globs` on the technology.** Scopes
  the mutants but not the build or the tests, and nothing would stop the next
  change reaching for `std::fs` inside the analysis.
- **One coarse port for every file the project holds.** Four small ones keep
  each fake and each adapter about one thing, and the dispatcher takes them as
  one `Ports` record.
- **serde in the core behind a feature.** The persisted formats are the
  adapters' business; the core states what an entry means, cli how it is
  spelled.
- **`HashMap` from hashbrown.** One more crate on the core's allow-list, for no
  measured gain: every map is a lookup or is sorted afterwards.

## Consequences

Two published crates, `dry4rust-core` first, sharing one version. A new
technology need is a trait in `core/src/traits/`, an in-memory fake in
`core/tests/support/` and an adapter in `cli/src/system/`, composed in
`EntryPoint`. Paths in the core are `SourcePath`s read with `/` as their only
separator, converted by cli at the edge and shown as they were given; a `.rs`
file the scanner keeps whose path is not UTF-8 fails the run instead of being
analysed under a name it does not have, and on Windows an exclusion pattern
matches with either separator. `suppression/` moved under the core's `cli/` so
the ports fit within `directory-subfolder-count`.

## Enforcement

The "Technology-free core" gate, first in stage 2: core's manifest names only
proc-macro2, syn and thiserror in any section; `core/src/lib.rs` carries
`#![no_std]` and `#![forbid(unsafe_code)]`; no file under `core/src` or
`core/tests` reaches `std::{env,fs,io,net,os,process}`, a printing macro, a
file-system method or `extern crate std`. `.cargo/mutants.toml` scopes
cargo-mutants to `core/src`, each mutant tested by core's tests only.

## Related

- [ADR-SingleCrateSingleLanguage](ADR-SingleCrateSingleLanguage.md)
- [ADR-CargoSubcommandPackaging](ADR-CargoSubcommandPackaging.md)
- [ADR-CorpusOutsideThePackage](ADR-CorpusOutsideThePackage.md)
- [`../ARCHITECTURE.md`](../ARCHITECTURE.md)
- [`../history/[IMPLEMENTED] CORE_CLI_SPLIT.md`](../history/%5BIMPLEMENTED%5D%20CORE_CLI_SPLIT.md)
