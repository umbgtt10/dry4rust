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

**Size band.** Units are bucketed by

```
  band(n) = floor(log2(n))      for n > 0
  band(0) = 0
```

and only units sharing a band are compared.

This is *not* the same as "within a factor of two", and the difference matters
in both directions. Sharing a band does imply a ratio below two, but a ratio
below two does not imply sharing a band: 7 nodes and 8 nodes are one node
apart and fall either side of a boundary, so they are never compared even
though their Dice ceiling is `0.933`. Meanwhile 8 and 15 nodes share a band
and are compared, with a ceiling of `0.696` -- below the default threshold, so
the work cannot pay off.

The filter is therefore both too strict and too loose, and what it discards
are false negatives the report does not mention. What a size filter *should*
express, for a threshold `t`, is

```
  |a| / |b| <= (2 - t) / t
```

which is a ratio of `1.5` at `t = 0.8`. Banding by `log2` does not implement
that bound; it approximates it with a grid whose edges fall in arbitrary
places. [OPEN_POINTS.md](OPEN_POINTS.md) records this as the correctness
defect it is.

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
