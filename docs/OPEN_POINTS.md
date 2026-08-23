# Open Points

Known limits of the current model. Each is a real property of the code as it
stands, not a wish list -- what to build next is
[ROADMAP.md](ROADMAP.md).

None of these is a bug in the sense of "the code does not do what it says".
They are places where what it says is narrower than a reader might assume.

## Child comparison is positional, so an inserted statement misaligns the rest

`similarity::count_matching` descends two trees together and zips their
children. The third child of `a` is only ever compared with the third child of
`b`.

The consequence is that the score measures alignment as much as content. Two
blocks that differ by one inserted statement near the top do not score "one
statement apart": everything after the insertion is compared against its
neighbour, disagrees, and contributes nothing. A pair a human would call
obviously duplicated can score below the threshold because of where the
difference sits rather than how large it is.

This is the single most consequential limitation in the tool, and the one a
tree-edit distance or a longest-common-subsequence over children would
address. It is not a setting -- there is no flag that changes it.

## Fingerprints are not guaranteed stable across Rust releases

`Fingerprint` hashes with `std::collections::hash_map::DefaultHasher`. That
type is deterministic within a toolchain -- it is seeded with fixed keys, not
random ones -- but the standard library explicitly does not guarantee the
algorithm across Rust versions.

Fingerprints are persisted. `.dry4rust-ignore.toml` records them, and a
suppression written under one toolchain may silently stop matching under
another: the entry stays in the file, the duplicate reappears in the report,
and nothing explains why.

The fix is a hasher this repository controls rather than one it borrows. It
has not been done because it invalidates every existing suppression file once,
and doing that deliberately is better than doing it twice.

## The size band drops near-duplicates that straddle a power of two

Candidates are bucketed by `floor(log2(node_count))` and only compared within
a bucket. This is usually described as "within a factor of two", and that
description is wrong in the direction that costs findings.

Sharing a bucket implies a size ratio below two. The converse does not hold,
and the gap is where real duplicates are lost:

| pair | ratio | same bucket | Dice ceiling |
|---|---|---|---|
| 7 and 8 nodes | 1.14 | **no** | **0.933** |
| 8 and 15 nodes | 1.88 | yes | 0.696 |

A 7-node unit and an 8-node unit are one node apart and could score `0.933`,
far above the default threshold of `0.8`. They are never compared, because
`log2` puts a boundary between them. An 8-node and a 15-node unit are nearly
twice apart, cannot reach `0.8`, and are compared anyway.

So the filter misses pairs it should catch and evaluates pairs it cannot
benefit from, and the misses are silent -- a report that says "no near
duplicates" does not distinguish "none exist" from "none survived bucketing".

What a size filter should express, for threshold `t`, is
`|a| / |b| <= (2 - t) / t` -- a ratio of `1.5` at `t = 0.8`. A sliding
comparison over units sorted by node count implements that directly and costs
no more.

This is a correctness defect rather than a tuning question, and it is the
second thing to fix after positional alignment. Neither has a flag.

## A group with no directly scored pair reports the threshold, not a measurement

Groups are built by transitive closure, so `A~B` and `B~C` puts `A`, `B` and
`C` together whether or not `A` was ever compared with `C`. The group's
reported similarity is the lowest score among pairs that *were* scored.

When no pair inside a group was scored directly, the threshold is reported
instead. That number is a floor -- every pair that built the group cleared it
-- but it reads like a measurement, and a reader comparing two groups cannot
tell which of them was measured.

## Sub-function duplicates can restate function-level ones

With `--sub-function`, each body also yields its if-branches, match arms, loop
bodies and closure bodies as units. These are grouped separately from
top-level units and reported under their own counts, so a function and its own
branch never share a group.

What is not separated is the case where two functions are already reported as
duplicates of each other: their matching internals are then reported again as
sub-function duplicates. The counts are honest about being distinct sets, but
the second finding carries no information the first did not.

## Macros are matched by name alone

A `MacroCall` node keeps the macro's name and nothing else -- the invocation
body is opaque. Two calls to the same macro with entirely different arguments
are identical to this tool, and two calls to different macros are always
distinct even when they expand to the same thing.

The name is unqualified, so two macros called `log!` from different crates
match each other.

Opacity is the right default -- expanding macros before comparison would make
the tool report duplication a reader cannot see in the source. But it means a
codebase that pushes its logic into macros is measured as less duplicated than
it is.

## Erased literals cannot distinguish a sentinel from a count

Literals are erased with their type preserved, so `42` and `99` are both
"integer literal". That is what makes two copies of the same function with
different constants match, which is the point.

It also means `0` used as a sentinel and `1` used as a count are the same
node. Two functions differing only in a magic number -- where the magic number
is the entire difference in behaviour -- are reported as exact duplicates.

## Test detection is a parse-time flag, not a policy

`CodeUnit::is_test` is set when parsing, and `RustAnalyzer::is_test_code`
returns it unchanged. `--exclude-tests` filters on it.

The flag records what the parser saw -- `#[test]` attributes and `#[cfg(test)]`
modules. A test helper in a non-test module, or a fixture builder used only by
tests, is not marked and is analysed as production code.

## Related

- [FORMULA.md](FORMULA.md) -- the scoring these limits apply to
- [ROADMAP.md](ROADMAP.md) -- which of these are next
