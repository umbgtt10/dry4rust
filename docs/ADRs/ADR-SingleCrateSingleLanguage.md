# ADR-SingleCrateSingleLanguage

## Status

- **Status:** Accepted
- **Date:** 2026-08-23

## Context

Upstream `cargo-dupes` was a workspace. It split the engine from the language
backends so that more than one language could be analysed: a core crate, a
`syn`-based Rust backend, a tree-sitter backend, and Python bindings. The
split was the right shape for that goal.

`dry4rust` does not have that goal. It is one of five tools in a family --
`stern4rust`, `crap4rust`, `twin4rust`, `iceberg4rust` -- every one of which
is a single crate analysing Rust, invoked as a cargo subcommand from the same
gate scripts.

## Decision

One crate, one language. `Cargo.toml` is a single `[package]` at the
repository root publishing `cargo-dry4rust` with library `dry4rust`. The
tree-sitter backend and the Python bindings are removed. The `syn` Rust
analyser stays.

The `LanguageAnalyzer` trait stays too.

## Forcing constraints / Evidence

The family's gate scripts resolve `..\Cargo.toml` and pass `--package`. A
workspace member layout would have meant a different manifest path in four
gate functions, for no benefit any of the four gates can see.

The removed backends were not free. Tree-sitter is a C dependency with its own
grammar crates; the Python bindings carried a build system, a wheel
configuration and a release workflow. All of it existed to serve languages
this tool does not analyse.

Keeping `LanguageAnalyzer` is deliberate and is not dead generality. It is the
seam the gates measure against -- `pure-traits` and `single-implemented-type`
both apply to it -- and removing it would have inlined `RustAnalyzer` into the
pipeline, which is the direction this family's rules push against. It also
cost nothing to keep: `run_analysis` already took `&dyn LanguageAnalyzer`, so
`CommandDispatcher` injects a trait object rather than a concrete type.

`is_test_code` had a default body on the trait, delegating to
`CodeUnit::is_test`. It is required now, so an analyser has to state how it
recognises test code rather than inherit an answer about a language it has
never seen.

## Rejected alternatives

**Keep the workspace, drop only the non-Rust members.** A workspace with one
member is a workspace in name. Rejected as ceremony.

**Keep tree-sitter for future languages.** Rejected: the dossier's conclusion
is that the differentiated ground is semantic redundancy in Rust, and a
backend for languages nobody is analysing is a maintenance cost against a
speculative benefit.

**Remove `LanguageAnalyzer` as well, since there is one implementation.**
Rejected: the trait is the injection seam, and `single-implemented-type`
tolerates it precisely because it is a boundary rather than an abstraction
over nothing.

## Consequences

`cargo install cargo-dry4rust` builds without a C toolchain.

Adding a second language later means adding a `LanguageAnalyzer`
implementation, not restoring a workspace.

Upstream's history still contains the removed backends. Recovering them is a
`git show` away, which is one of the reasons for
[ADR-ForkedNotCopied](ADR-ForkedNotCopied.md).

## Enforcement

`cargo stern4rust`'s `pure-traits` and `single-implemented-type` rules apply
to `LanguageAnalyzer` with no skip.

## Related

- [ADR-ForkedNotCopied](ADR-ForkedNotCopied.md)
- [ADR-CargoSubcommandPackaging](ADR-CargoSubcommandPackaging.md)
