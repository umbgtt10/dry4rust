# ADR-DecompositionOverThresholdRelaxation

## Status

- **Status:** Accepted
- **Date:** 2026-08-23

## Context

Every repository in this family gates on `cargo crap4rust` at a threshold of
15, with no override. CRAP is
`complexity^2 * (1 - coverage)^3 + complexity`.

The inherited engine had five functions above that line. Three of them could
not be brought under it by testing, and the reason is arithmetic rather than
effort: the `+ complexity` term means CRAP never falls below the complexity
itself. A function of complexity 27 scores 27 at perfect coverage.

| function | complexity | coverage | CRAP |
|---|---:|---:|---:|
| `grouper::find_near_duplicates` | 27 | 82% | 31.3 |
| `extractor::extract_recursive` | 20 | 92% | 20.2 |
| `main` | 14 | 79.3% | 15.7 |

`main` looked reachable -- complexity 14 needs 83% coverage -- and was not.
Coverage is collected from the test harness process, not from spawned
children, so seven tests driving the binary through `stats`, `ignored`,
`cleanup`, `check` twice, `ignore` and a missing path moved it by nothing.

## Decision

Split the functions. The gate runs at 15 with no override, no baseline, no
skip list and no project-threshold budget.

| was | became |
|---|---|
| `find_near_duplicates` | `NearDuplicateFinder`, `UnionFind`, `SimilarityPair` |
| `extract_recursive` | `SubUnitExtractor` |
| `main` | `CommandDispatcher` |

Each is a struct in a file of its own with a mirrored test file.

## Forcing constraints / Evidence

Two escape hatches were available and both were tried before being rejected.

Raising the threshold to 32 clears all three. It also admits any newly written
function up to 32, which is the opposite of a gate: the number that hides
inherited debt hides new debt identically, and nothing distinguishes them.

`-UseProjectThreshold` is better -- every function stays measured at 15 and
named in the report, and the three spend a budget of 2.4% against 5%. It
shipped for one commit. What it gives up was measured directly: a probe
function at complexity 35 and 14.3% coverage scores 806.4, and the same run
reports `verdict=warn` at 0.7%. Under the budget that function passes. Under
zero tolerance stage 2 exits 1.

The decomposition was verified not to change behaviour before any new test was
written: all 301 existing tests passed against the rewrites as they were. The
46 new tests came after.

## Rejected alternatives

**Threshold 32.** Rejected: hides new debt as effectively as old.

**Threshold 15 with `-UseProjectThreshold`.** Rejected after shipping it:
tolerates roughly three more crappy functions before failing, and the probe
above showed exactly what walks through.

**A baseline file.** `crap4rust` has no baseline mechanism, and adding one to
the gate script would have reinvented the budget with extra steps.

**Leaving `main` alone as untestable-by-nature.** Rejected: it was untestable
because of where it lived, not what it did. Moving the dispatch into the
library made it ordinary.

## Consequences

Five new source files and five new test files. The public library surface grew
by five types, each of which `tested-public-api` now requires tests for.

Two improvements fell out of the split rather than being sought. The original
recovered candidate indices through `HashMap<*const CodeUnit, usize>`,
comparing raw pointers to answer a question it had just discarded the answer
to; buckets hold indices now and `use std::ptr` is gone. And grouping ran
through a `HashMap` keyed by union-find root, so group order came out of hash
iteration; `UnionFind::groups` is deterministic, which
[ADR-DeterministicGrouping](ADR-DeterministicGrouping.md) covers.

If this gate ever needs an override again, the honest move is the one taken
here.

## Enforcement

`xtask`'s `CrapGate`, run by `just stage2`, judges the parsed report rather
than the exit code and fails when `crappy_functions` is greater than zero. The
`--threshold` it passes decides only what crap4rust *labels* crappy; the gate
itself tolerates none.

## Related

- [ADR-DeterministicGrouping](ADR-DeterministicGrouping.md)
