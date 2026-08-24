# ADR-CorpusOutsideThePackage

## Status

- **Status:** Accepted
- **Date:** 2026-08-24
- Supersedes [ADR-FixturesAreTheCorpusNotTheCode](ADR-FixturesAreTheCorpusNotTheCode.md)

## Context

`cargo package` skips any subdirectory holding a `Cargo.toml`, treating it as
a separate package that must not be nested inside this one. The seven fixture
crates each carry one, because the tool is pointed at them as real packages.

So while the corpus lived under `tests/fixtures/`, the tarball shipped 64 test
files and **none** of the fixtures those tests read. `cargo test` on the
published crate gave 66 failures. Not a crash and not a missing-directory
error: the scanner walked a path that was not there, found no `.rs` files, and
every assertion about content failed against an empty corpus.

`cargo publish --dry-run` was happy throughout, and always would have been. It
verifies that a crate builds. It never runs its tests.

Nothing in this repository caught it either, because both gates run from the
working tree, where `tests/fixtures/` is right where the tests expect it. The
defect existed only in the artifact.

## Decision

The repository is a workspace, in the shape the three workspace members of
this family already use:

```
Cargo.toml     [workspace] members = ["core", "validation"], exclude = ["fixture"]
core/          cargo-dry4rust -- the published crate
validation/    publish = false -- the tests with no mirror to be named after
fixture/       the corpus, outside both members
```

The split criterion is the one `crap4rust`'s validation manifest states, and
it is sharper than "reads from disk": **is the subject of this test one source
file, or the whole tool?** Fifty-three tests drive the binary and moved on that
basis. Forty more called library types directly but read the corpus for their
input; those moved too, because core touching the corpus is the defect itself.

`fixture/` is excluded from the workspace rather than made a member, so a
deliberately-duplicated crate never has to satisfy the workspace's own build.

Stage 1 covers the workspace. Stage 2 gates both members: `core` at all
twenty-one rules, `validation` at twenty. This is a step stricter than the rest
of the family, which gates `core` and leaves `validation` unmeasured.

## Forcing constraints / Evidence

**Six core mirrors emptied, and every replacement is better.** `StatsCommand`,
`ReportCommand`, `CheckCommand`, `CleanupCommand`, `BaselineCommand` and
`OutputFormat` are now driven from a `DuplicateGroup` built in the test rather
than parsed out of a fixture crate. Their subject was never the corpus -- only
what they write given a result -- so reading one was always incidental.

**`main` had to move into the library.** Once its tests left the package it was
complexity 5 at 0% coverage, which is CRAP 30 against a ceiling of 15. Raising
the number was not available; this repository has
[an ADR](ADR-DecompositionOverThresholdRelaxation.md) saying so. The argument
mapping became `EntryPoint`, so *which root, which command, which overrides*
are things a test can call, and `main` is eleven lines. `EntryPoint::run` takes
its arguments rather than reading `argv`, which is what makes it callable --
the same shape `crap4rust`'s `EntryPoint::run(args().collect())` has.

**The measure is the artifact, not the working tree.**

| | before | after |
|---|---|---|
| packaged `cargo test` | 66 failed | **469 passed, 0 failed** |
| packaged files | 165 | 137 |
| tarball | 156.6 KiB | 100.8 KiB |

## Rejected alternatives

**`exclude = ["tests/**"]` in the manifest.** Ships no tests, so none can fail.
Rejected: it hides the question rather than answering it, and a consumer
vendoring the crate gets a crate whose tests are simply absent. It also leaves
the corpus inside the package, where the next test to read it reintroduces the
defect silently.

**Delete the fixtures' `Cargo.toml` files** so cargo stops seeing nested
packages. The scanner only walks for `.rs` and never reads them, so this would
work. Rejected: the superseded ADR's reasoning applies -- the corpus is the
tool's input and is not edited to suit a rule about source. The tool is also
pointed at them as real packages.

**Stay a single crate and accept a red packaged suite.** Rejected: a tool whose
argument is that it submits to its own gates cannot ship a test suite that
cannot run.

## Consequences

`validation/` is gated at twenty rules rather than left alone. The one it
cannot take is `paired-test-file`: with no `src/` in that crate, no test file
can be named after a source file. It is skipped by name on the command line, so
the report states which rule was not applied -- a silence would have been the
thing worth objecting to, not the skip.

`crap4rust` stays on core. It scores source functions against coverage and
validation has none; pointed at validation it also fails outright, because it
drives coverage with `-p validation`, which does not build the binary those
tests spawn.

The one house-rule exclusion is gone. `stern4rust.toml` no longer names
`tests/fixtures/**`, because with the corpus outside the package there is
nothing left for a rule to reach.

Coverage measured on `core` no longer counts what `validation` exercises. That
is why `main` moved rather than being left alone: the number is now honest
about what core's own tests reach.

## Enforcement

The packaged artifact is the test: `cargo package` in `core/`, then
`cargo test` inside `target/package/cargo-dry4rust-<version>/`. It is not part
of either gate, because neither gate builds a tarball -- run it before
publishing.

## Related

- [ADR-FixturesAreTheCorpusNotTheCode](ADR-FixturesAreTheCorpusNotTheCode.md)
- [ADR-DecompositionOverThresholdRelaxation](ADR-DecompositionOverThresholdRelaxation.md)
- [ADR-CargoSubcommandPackaging](ADR-CargoSubcommandPackaging.md)
