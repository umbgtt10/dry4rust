# ADR-KindDoesNotRestrictComparison

## Status

- **Status:** Accepted
- **Date:** 2026-08-23

## Context

Every `CodeUnit` carries a `CodeUnitKind`: `Function`, `Method`, `Closure` for
whole units, and `IfBranch`, `MatchArm`, `LoopBody`, `Block` for the
sub-function units extracted from their bodies.

The two halves of the tool disagreed about whether that kind mattered.

`group_exact_duplicates` compares fingerprints and nothing else. A free
function and a method whose normalised signature and body hash identically are
reported as exact duplicates, and always have been.

`NearDuplicateFinder` bucketed candidates by kind before comparing, and never
compared across buckets. The same free function and method, differing by a
single statement, were never scored at all.

So identical across kinds was a finding, and nearly identical across kinds was
invisible. Nothing in the output distinguished "no near duplicates" from "no
near duplicates within a kind".

## Decision

Kind does not restrict what is compared. `PairScanner::scan` orders all
candidates by node count and applies the size bound; nothing else filters.

## Forcing constraints / Evidence

The inconsistency has no reading that makes it deliberate. If a
function/method pair is worth reporting when the bodies are identical, it is
worth reporting when they differ by one statement -- that is the more common
case and the one a reader is more likely to be able to act on.

The objection was noise from sub-function units: an `IfBranch` compared with a
`MatchArm`. It does not survive contact with what the tool is for. A block of
logic that appears once inside an `if` and once inside a `match` arm is
precisely the duplication a reader would want extracted into a function, and
`group_exact_duplicates` already reports it when the two are identical. The
noise objection argues against a finding the tool already makes.

Cost was the historical reason and is no longer one.
[ADR-SizeFilterIsAProvableBound](ADR-SizeFilterIsAProvableBound.md) made the
size filter exact, and it bounds the extra comparisons: with the default
threshold a unit is only ever compared with units within a ratio of `1.5`, and
the sorted scan stops at the first partner too large. Removing the kind
bucketing removed a `HashMap` allocation along with it.

Ordering by size across all candidates also made pair generation
deterministic. Bucketing iterated `HashMap::values()`, so pairs came out in an
arbitrary order; grouping was already order-independent, but one fewer
arbitrary order is one fewer thing to reason about.

## Rejected alternatives

**Restrict near detection to kind, and restrict exact detection too.**
Consistent, and consistent in the wrong direction -- it would remove findings
that are already correct and already reported.

**Compare across `Function`, `Method` and `Closure` but not across
sub-function kinds.** The half-measure. Rejected: it keeps a special case that
would need explaining in the output, and the sub-function case is a real
finding rather than noise.

**Add a flag.** Rejected: a flag would encode the question rather than answer
it, and the answer is not configuration-dependent -- exact detection has no
such flag and needs none.

## Consequences

Results change again, in the same direction as the size-filter fix: more
findings, all of which were always there. Codebases using `--sub-function`
will see the largest difference.

A pair is now reported once as near-duplicates even when its members have
different kinds, so a report can pair a `Function` with a `Method`. The report
already names each member's file and lines, so nothing is ambiguous about
which is which.

## Enforcement

`tests/near_duplicate/pair_scanner_tests.rs` pins both directions:
`scan_pairs_a_function_with_a_method_whose_body_matches` and
`scan_pairs_sub_function_units_of_different_kinds`. The first replaced a test
asserting the opposite; the assertion was corrected rather than the test
deleted.

## Related

- [ADR-SizeFilterIsAProvableBound](ADR-SizeFilterIsAProvableBound.md)
- [FORMULA.md](../FORMULA.md)
