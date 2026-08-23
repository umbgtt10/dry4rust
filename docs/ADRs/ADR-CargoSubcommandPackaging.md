# ADR-CargoSubcommandPackaging

## Status

- **Status:** Accepted
- **Date:** 2026-08-23

## Context

The tool takes `--path`, walks a source tree, and is meant to sit in a build
gate beside `cargo fmt`, `cargo clippy` and `cargo crap4rust`. It could ship
as a plain binary invoked by name or as a cargo subcommand.

The repository is named `dry4rust`, which is not necessarily the crate name.

Upstream shipped as `cargo-dupes`, and the fork inherited that name in four
places that a user actually sees: the `clap` program name, the
`.dupes-ignore.toml` suppression file, the `dupes.toml` configuration file,
and the `[package.metadata.dupes]` manifest key.

## Decision

The crate publishes as `cargo-dry4rust` with library name `dry4rust`, so cargo
resolves `cargo dry4rust`. A hidden positional argument absorbs the subcommand
name cargo re-inserts at `argv[1]`.

Every user-facing name follows:

| was | is |
|---|---|
| `cargo-dupes` (clap `name`) | `cargo-dry4rust` |
| `.dupes-ignore.toml` | `.dry4rust-ignore.toml` |
| `dupes.toml` | `dry4rust.toml` |
| `[package.metadata.dupes]` | `[package.metadata.dry4rust]` |

## Forcing constraints / Evidence

Cargo discovers subcommands by looking for a `cargo-<name>` binary on `PATH`.
The package name is therefore fixed by the invocation wanted, not chosen
freely. The sibling tools resolved this identically -- repo `twin4rust`
publishes `cargo-twin4rust`, repo `crap4rust` publishes `cargo-crap4rust` --
and all keep the library name unprefixed so `use dry4rust::...` reads
normally.

The `clap` name mattered more than it looked. The binary was already
`cargo-dry4rust` while `clap` still announced `cargo-dupes`, so `--help` and
`--version` named a different tool than the one running. 324 passing tests did
not catch it: the only assertion on help output checked the `about` string,
which had never been upstream-specific.

The three filenames are a compatibility question with no one to be compatible
with. Nobody migrates from `cargo-dupes` to `cargo-dry4rust` carrying a
suppression file, because the fingerprints in one are not addressed to the
other -- this fork's normaliser is where the intended work happens, and
fingerprints move when it does. Keeping upstream's filenames would have
promised an interoperability that does not exist.

## Rejected alternatives

**Ship as a plain binary named `dry4rust`.** Rejected: the family is invoked
as cargo subcommands from shared gate scripts, and one tool spelled
differently is a trap.

**Keep `.dupes-ignore.toml` for compatibility.** Rejected per the evidence
above -- the compatibility is nominal.

**Read both the old and new filenames.** Rejected: two names for one file is a
migration mechanism, and there is no population to migrate.

## Consequences

Anyone who used the fork before this change and wrote a suppression file must
rename it. The repository's own `.dupes-ignore.toml` was renamed in the same
commit.

`--help` and `--version` now name the tool that is running.

## Enforcement

`cargo run -- --version` prints `cargo-dry4rust 0.2.0`. The configuration
tests write `dry4rust.toml` and `[package.metadata.dry4rust]` and would fail
if the reader and the writer disagreed.

## Related

- [ADR-SingleCrateSingleLanguage](ADR-SingleCrateSingleLanguage.md)
- [ADR-ForkedNotCopied](ADR-ForkedNotCopied.md)
