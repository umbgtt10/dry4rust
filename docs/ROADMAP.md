# dry4rust Roadmap

## Product Direction

`dry4rust` detects code that says the same thing twice.

The research in [dry4rust-dossier.md](dry4rust-dossier.md) reached a
conclusion worth restating here, because it sets everything below: syntactic
duplicate detection for Rust is **already held** -- competently, recently, by
the tool this one is forked from. Rebuilding it produces a worse copy.

The defensible ground is the kind of repetition that syntactic tools cannot
see: two functions that do the same thing by different means. Same behaviour,
different shape. Every AST-fingerprint tool, this one included, is blind to it
by construction, because normalisation preserves control flow exactly and two
different structures normalise to two different trees.

That is the direction. Everything before it is making the inherited engine
trustworthy enough to build on.

## Guiding Principles

**Structure, never names.** Nothing is inferred from identifiers, comments or
formatting. A copied block changes all three first.

**A finding must be actionable.** A report that says "these are 84% similar"
without saying where and what to do is a number, not a finding.

**Suppression is a first-class feature, not an escape hatch.** A duplicate
that is deliberate should be recordable with a reason, and stay recorded.

**No silent narrowing.** Where a filter discards work, it says so. The size
band and the macro opacity in [OPEN_POINTS.md](OPEN_POINTS.md) are documented
precisely because they change what "found nothing" means.

**The gates apply to this repository too.** A tool that measures code quality
and does not submit to it is an argument against itself.

## Current Baseline

Version 0.2.0. Forked, cleaned, corrected, and under the family's gates.

- Exact and near-duplicate detection over functions, methods and closures
- Optional sub-function analysis: if-branches, match arms, loop bodies,
  closure bodies
- Seven subcommands -- `report`, `stats`, `check`, `ignore`, `ignored`,
  `cleanup`, `baseline`
- Text and JSON output
- Four independent threshold ceilings for CI
- Suppression by fingerprint, with a reason, and a `cleanup` that prunes
  entries no longer matching anything
- Baseline mode: record inherited duplication so `check` fails on what is
  added, with the suppressed count in every summary
- Configuration from defaults, `dry4rust.toml`, `[package.metadata.dry4rust]`
  and CLI flags, with every ranged value carrying its range in its type
- 565 tests (471 core, 94 validation); `stern4rust` clean with all 21 rules applied and no baseline;
  `crap4rust` clean at 15 with no override; every source file mirrored;
  no file at or above 10 on `iceberg4rust`

`cargo-dry4rust` 0.1.0 is on crates.io as a name placeholder. 0.2.0 is the
first release with an engine in it and has not been published.

## What is left

**1. Publish 0.2.0.** The manifest is ready and the name is held. It now has
a reason beyond "the fork is tidy": four correctness fixes the incumbent does
not have -- an exact size bound, aligned sequence children, a fingerprint
format that survives a toolchain upgrade, and near-duplicate detection that no
longer hides what exact detection reports -- plus baseline mode, which is what
makes any of it adoptable on a codebase that has the problem.

**2. Semantic redundancy detection.** The direction above. Two functions with
the same behaviour and different structure -- a `for` loop accumulating a sum
against an `iter().sum()`, a hand-rolled `match` against an `unwrap_or`. This
is the work the fork exists for, and it is deliberately last, because it
depends on the engine underneath being trustworthy.

## Deferred Ideas

**More languages.** The `LanguageAnalyzer` trait is kept and the tree-sitter
backend was removed. Restoring multi-language support means adding an
implementation, not a workspace. Not planned; the differentiated ground is
Rust-specific.

**Cross-crate analysis.** Currently one root at a time. Workspace-wide
duplication is a real question and a different scanning model.

**IDE integration.** The dossier notes that batch-and-report is a weaker fit
for the agent loop than incremental querying. Real, and much later.

**Expanding macros before comparison.** Would find duplication that macros
hide. Rejected for now: it reports duplication a reader cannot see in the
source, which fails the actionability principle.

## Success Measure

The tool is succeeding when a `check` in CI fails on duplication a reviewer
agrees is duplication, and passes on repetition a reviewer would defend.

False positives cost more than false negatives here. A gate that cries wolf
gets a higher threshold, then gets ignored, then gets deleted.

## Revision Policy

This document is revised when direction changes, not when work completes.
Completed work moves to [IMPLEMENTED-FEATURES.md](IMPLEMENTED-FEATURES.md);
decisions that constrain future work become
[ADRs](ADRs/README.md).
