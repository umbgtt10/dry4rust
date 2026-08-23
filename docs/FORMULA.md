# Formula

Two numbers decide everything this tool reports: a fingerprint, which answers
*identical or not*, and a similarity score, which answers *how close*.

## The exact case

```
fingerprint(unit) = hash(normalised_signature, normalised_body)
```

Two units are exact duplicates when their fingerprints are equal. There is no
threshold and no tolerance -- normalisation has already absorbed everything
that was allowed to differ, so what reaches the hash either matches or does
not.

Group fingerprints for near-duplicate groups are composed rather than hashed
from source:

```
composite(fps) = hash(sort(fps))
```

Sorting first makes the composite independent of member order, so the same
group yields the same identifier however it was assembled. That identifier is
what `.dry4rust-ignore.toml` records.

## The score

Near duplication uses the Dice coefficient over normalised trees:

```
                 2 * matching(a, b)
  score(a, b) = ----------------------
                  |a| + |b|
```

where `|a|` is the node count of tree `a`, and `matching` walks both trees
together counting nodes that agree.

The range is `0.0` to `1.0`. Two empty trees score `1.0` by definition. The
default threshold is `0.8`.

## What `matching` counts

`matching` descends both trees in lockstep and stops at the first
disagreement on a branch:

- either node being `None` -- the absent-child marker -- contributes `0`
- different `NodeKind` discriminants contribute `0` and end that branch
- two `MacroCall` nodes with different names contribute `0` and end that
  branch, without descending
- otherwise the node contributes `1` if the kinds are fully equal, and the
  walk continues into children, pairwise, in order

The pairwise descent is positional. Children are zipped, so the third child of
`a` is only ever compared with the third child of `b`, and a tree with more
children than the other simply runs out of pairs.

This is the property most worth knowing before trusting a score. Two blocks
that differ by one inserted statement do not score "one statement apart" --
everything after the insertion is compared against its neighbour and disagrees.
The measure is sensitive to alignment, not just to content.
[OPEN_POINTS.md](OPEN_POINTS.md) records this as a known limitation rather
than a setting.

## Why the score is not computed for every pair

Comparing every unit with every other is quadratic. Two filters run first, and
neither can separate a pair the scorer would have accepted:

**Kind.** A `Function` is never compared with a `MatchArm`.

**Size.** `matching` can never exceed the smaller tree's node count, so a
pair's score is bounded before either tree is examined:

```
                        2 * min(|a|, |b|)
  ceiling(a, b) = ---------------------------
                        |a| + |b|
```

A pair is scored only when `ceiling >= t`. This is exact: every pair it
discards provably cannot reach the threshold, and every pair that could is
kept. Rearranged, it is a ratio bound of `(2 - t) / t` -- `1.5` at the default
`0.8` -- but the ceiling form is what the code computes, because it needs no
special case for a zero-size unit.

Candidates are sorted by node count within each kind, so once a partner is too
large the ceiling only falls further and the remainder of the list is skipped.
The filter therefore costs one comparison per pair examined and nothing per
pair avoided.

The comparison leans towards keeping. `similarity_score` admits a score equal
to the threshold, so a pair whose ceiling lands exactly on it must survive the
filter; an epsilon of slack stops floating-point rounding from discarding one.
Comparing one pair too many costs a score. Comparing one too few loses a
finding silently, and silence is the expensive failure.

This replaces a `floor(log2(n))` banding inherited from upstream, which
discarded pairs one node apart when they straddled a power of two -- 7 against
8, ceiling `0.933`, never compared -- while comparing pairs nearly twice apart
that could not clear the bar. See
[ADR-SizeFilterIsAProvableBound](ADRs/ADR-SizeFilterIsAProvableBound.md).

## From pairs to groups

Scored pairs are joined by transitive closure -- if `A` matches `B` and `B`
matches `C`, all three are one group, whether or not `A` was ever compared
with `C`.

A group reports its **weakest link**: the lowest score recorded between any
two of its members.

```
  similarity(group) = min { score(i, j) : i, j in group, (i, j) scored }
```

When no pair inside a group was scored directly -- possible under transitive
closure -- the threshold stands in, since every pair that built the group
cleared it.

Reporting the minimum rather than the mean is deliberate: a group's claim is
that all of its members are alike, and that claim is only as strong as its
weakest pair.

## Thresholding

`check` compares the result against four independent ceilings, any of which
fails the run:

| flag | counts |
|---|---|
| `--max-exact` | exact duplicate groups |
| `--max-near` | near duplicate groups |
| `--max-exact-percent` | exact duplicate lines as a percentage of total lines |
| `--max-near-percent` | near duplicate lines as a percentage of total lines |

Unset means unlimited. Percentages use

```
  percent = duplicated_lines / total_lines * 100
```

with `0.0` when there are no lines at all, and lines counted as
`line_end - line_start + 1` per member.

## Related

- [ARCHITECTURE.md](ARCHITECTURE.md) -- where each of these runs
- [OPEN_POINTS.md](OPEN_POINTS.md) -- the positional-alignment limit, and the
  hasher's stability
