# ADR-SizeFilterIsAProvableBound

## Status

- **Status:** Accepted
- **Date:** 2026-08-23

## Context

Scoring every pair of code units against every other is quadratic. A filter
runs first so that only plausible pairs reach the Dice comparison.

Upstream's filter bucketed units by `floor(log2(node_count))` and compared
only within a bucket. It was described, in upstream's code and in this
repository's first draft of `FORMULA.md`, as comparing units "within a factor
of two of each other".

That description is sound as a design and does not describe the code. Sharing
a `log2` bucket implies a size ratio below two; a size ratio below two does
not imply sharing a bucket. The gap is not academic:

| pair | ratio | same bucket | Dice ceiling |
|---|---|---|---|
| 7 and 8 nodes | 1.14 | **no** | **0.933** |
| 8 and 15 nodes | 1.88 | yes | 0.696 |

A pair one node apart, able to score `0.933` against a default threshold of
`0.9`, was never compared, because `log2` happens to put a boundary between 7
and 8. A pair nearly twice apart, unable to reach `0.9` at all, was compared
anyway.

The misses were silent. A report saying "no near duplicates" did not
distinguish none existing from none surviving the bucketing.

## Decision

The filter is the exact bound the threshold implies.

`matching` can never exceed the smaller tree's node count, so

```
                        2 * min(|a|, |b|)
  ceiling(a, b) = ---------------------------
                        |a| + |b|
```

is the highest score a pair can reach given only its sizes. A pair is scored
when `ceiling >= t`, and skipped otherwise.

Candidates are sorted by node count. `ceiling` falls monotonically as the
partner grows, so the first partner that fails ends the scan for that unit.

## Forcing constraints / Evidence

The bound is exact in both directions -- it discards only pairs that provably
cannot clear the threshold, and keeps every pair that could. That is the
property `log2` banding lacked, and it is why this is a correctness change
rather than a tuning one.

Where the arithmetic left a choice, the choice was to compare. A pair whose
ceiling lands exactly on the threshold can score exactly the threshold, and
`similarity_score` admits it, so such a pair must survive the filter; an
epsilon of slack keeps floating-point rounding from discarding it. Comparing
one pair too many costs one score. Comparing one too few loses a finding, and
says nothing about having done so.

The defect was proved rather than argued.
`find_groups_a_pair_whose_sizes_straddle_a_power_of_two` builds units of 7 and
8 nodes scoring `0.933`, and fails against the `log2` banding. Two companion
tests pin the other edges: a pair too far apart to reach the threshold is
still skipped, and a pair whose ceiling is exactly the threshold is still
kept.

## Rejected alternatives

**Keep the banding, widen it to neighbouring buckets.** Comparing bucket `n`
with `n-1` and `n+1` catches the 7-against-8 case and admits pairs up to four
times apart. Rejected: it trades a wrong bound for a loose one, and still
cannot state what it guarantees.

**Express the filter as the ratio `(2 - t) / t`.** Algebraically identical.
Rejected in favour of the ceiling form, which needs no special case when a
unit has zero nodes and reads as what it is -- the best this pair could
possibly score.

**Drop the size filter entirely.** Correct and quadratic. Rejected: the bound
is exact, so the filter costs no findings, and there is no reason to pay for
comparisons whose outcome is already known.

**Also drop the kind filter while here.** Deferred at the time, and done
immediately afterwards once the inconsistency was stated plainly. See
[ADR-KindDoesNotRestrictComparison](ADR-KindDoesNotRestrictComparison.md).

## Consequences

More pairs are scored near bucket boundaries and fewer are scored far from
them. Runtime is unchanged in practice; the sorted scan replaces a hash-map
bucketing pass.

Results change. Codebases that reported no near duplicates may now report
some, and those findings were always there.

`--threshold` now means what it says at every value. Under banding, a
threshold below `0.667` was silently qualified by the bucketing; the bound is
derived from the threshold, so it tracks it.

## Enforcement

`tests/near_duplicate/pair_scanner_tests.rs` holds the three edges: the pair that
straddles a power of two, the pair too far apart, and the pair whose ceiling
is exactly the threshold.

## Related

- [ADR-DeterministicGrouping](ADR-DeterministicGrouping.md)
- [FORMULA.md](../FORMULA.md)
