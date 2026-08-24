# Architecture Decision Records

Each ADR documents one load-bearing decision behind `cargo-dry4rust` --
succinct, self-contained, citable on its own. Like the sibling `crap4rust`,
`twin4rust` and `iceberg4rust` tools, and unlike the larger `etheram`
ecosystem repositories, these are not priority-tiered: a single-crate CLI has
a small enough decision surface that a flat list is sufficient.

Decisions about where this tool is *going* are not here. They are in
[`../dry4rust-dossier.md`](../dry4rust-dossier.md), which is research rather
than a record of anything settled.

## Index

| ADR | Decision |
|---|---|
| [ADR-ForkedNotCopied](ADR-ForkedNotCopied.md) | Upstream's history is an ancestor of this repository rather than a credit line -- MIT's attribution duty is continuing, and a fork is a fact in the object graph that a later edit cannot quietly remove. |
| [ADR-SingleCrateSingleLanguage](ADR-SingleCrateSingleLanguage.md) | One crate analysing Rust, matching the family; the tree-sitter and Python backends are removed while `LanguageAnalyzer` stays as the injection seam. |
| [ADR-CargoSubcommandPackaging](ADR-CargoSubcommandPackaging.md) | The crate publishes as `cargo-dry4rust` with library `dry4rust`, and every user-facing name upstream left behind -- the `clap` program name, the ignore file, the config file, the manifest key -- follows it. |
| [ADR-DecompositionOverThresholdRelaxation](ADR-DecompositionOverThresholdRelaxation.md) | CRAP never falls below complexity, so three inherited functions could not be tested under the family's threshold of 15; they were split into five new types rather than the threshold being raised or budgeted. |
| [ADR-TheFingerprintFormatIsOwned](ADR-TheFingerprintFormatIsOwned.md) | The hash algorithm and the byte format are defined here, with variants written by name -- a fingerprint is persisted, so it cannot rest on `DefaultHasher`, native-endian integers, or a derive's variant ordering. |
| [ADR-SequenceChildrenAreAligned](ADR-SequenceChildrenAreAligned.md) | `Block`, `Tuple` and `Array` children are aligned by weighted longest-common-subsequence so an inserted statement shifts the rest; every other kind keeps positional slots so a then-branch never matches an else-branch. |
| [ADR-SizeFilterIsAProvableBound](ADR-SizeFilterIsAProvableBound.md) | The size pre-filter is the exact bound the threshold implies, replacing a `log2` banding that silently dropped pairs one node apart across a power of two. |
| [ADR-KindDoesNotRestrictComparison](ADR-KindDoesNotRestrictComparison.md) | Near-duplicate detection compares across `CodeUnitKind`, because exact detection always did -- one half of the tool reporting what the other hides is an inconsistency, not a policy. |
| [ADR-DeterministicGrouping](ADR-DeterministicGrouping.md) | `UnionFind::groups` orders groups by lowest member and members ascending, so the same input always produces the same output -- the inherited `HashMap` keyed by set root did not. |
| [ADR-VisibilityIsNotAnEscapeHatch](ADR-VisibilityIsNotAnEscapeHatch.md) | `tested-public-api` is answered with tests, never by dropping `pub` to `pub(crate)` -- with integration tests only, a smaller surface is an untestable one. |
| [ADR-FixturesAreTheCorpusNotTheCode](ADR-FixturesAreTheCorpusNotTheCode.md) | `tests/fixtures/**` is the repository's one house-rule exclusion, because those files are the tool's input and stamping headers into them would edit the question rather than answer it. |
| [ADR-BaselineIsInheritedNotForgiven](ADR-BaselineIsInheritedNotForgiven.md) | A baseline records the duplication a codebase already has so `check` fails on what is added; an entry carries its member count, because an exact group keeps its fingerprint when a third copy joins it. |
| [ADR-OneDocumentPerRun](ADR-OneDocumentPerRun.md) | `--format json` produces one document per run, because the document is the format's business and not the command's -- `report` was several top-level values and `check` printed its verdict as prose between two of them. |
| [ADR-ThresholdsCarryTheirRange](ADR-ThresholdsCarryTheirRange.md) | Every threshold is a `Threshold`, which cannot be built outside its range -- a similarity of five made the size filter reject every pair and the report say "no near duplicates", which is what a clean codebase says. |
