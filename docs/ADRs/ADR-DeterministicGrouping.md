# ADR-DeterministicGrouping

## Status

- **Status:** Accepted
- **Date:** 2026-08-23

## Context

Near-duplicate detection produces pairs -- "these two units are similar" --
and must report groups. Turning pairs into groups is a transitive closure,
computed with a disjoint-set forest.

The inherited implementation collected the closure into
`HashMap<usize, Vec<usize>>` keyed by each set's root, then iterated
`into_values()`. `HashMap` iteration order in Rust is unspecified and, with
the default hasher, varies between runs of the same binary on the same input.

The final result was sorted by member count, then similarity, then
fingerprint, which masked the problem in most cases. It does not mask it when
two groups tie on all three keys, and it does nothing for any caller reading
the sequence before that sort.

A tool whose output feeds a CI gate should not have runs that differ.

## Decision

`UnionFind::groups` returns `Vec<Vec<usize>>` ordered by each group's lowest
member, with members ascending. The same input always produces the same
output, before any sorting is applied.

The forest itself is a type -- `UnionFind` in `src/union_find.rs` -- rather
than a pair of free functions taking `&mut [usize]`.

## Forcing constraints / Evidence

The original `find` and `union` took `&mut [usize]`, which this repository's
house rules forbid as input parameters, and which no test could exercise
because both were private free functions in `grouper.rs` with no public path
to them. Making the forest a type gave the ordering rule somewhere to live and
made it testable at the same time; `tests/union_find_tests.rs` now pins it
directly.

Determinism is asserted rather than assumed:
`find_run_twice_over_the_same_units_returns_the_same_groups` runs the finder
twice over identical input and compares group sizes and fingerprints.

Ordering by lowest member was chosen over sorting by root because roots are an
implementation detail of path compression -- which root a set ends up with
depends on union order, so ordering by root would have been stable within a
run and meaningless across changes to the algorithm.

## Rejected alternatives

**`BTreeMap` keyed by root.** Deterministic, and still ordered by an
implementation detail. Rejected for the same reason as sorting by root.

**Rely on the final sort.** The sort has three keys and no tiebreak beyond
them; two groups equal on all three would still be ordered arbitrarily.
Rejected because the guarantee is wanted at the source, not patched at the
end.

**Seeded hasher.** Would make a single build reproducible while leaving the
order arbitrary and unexplained. Rejected as hiding the question.

## Consequences

`UnionFind::groups` returns every set including singletons; callers filter for
`len() > 1`. That is one line at the call site and keeps the type honest about
what a disjoint-set forest contains.

The type is public, so `tested-public-api` requires tests for each method.
Ten exist.

## Enforcement

`tests/union_find_tests.rs` asserts the ordering contract directly.
`tests/near_duplicate_finder_tests.rs` asserts that two runs agree.

## Related

- [ADR-DecompositionOverThresholdRelaxation](ADR-DecompositionOverThresholdRelaxation.md)
