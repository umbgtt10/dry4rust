# ADR-SequenceChildrenAreAligned

## Status

- **Status:** Accepted
- **Date:** 2026-08-23

## Context

`similarity::count_matching` descended two normalised trees together and
zipped their children: the nth child of one was only ever compared with the
nth child of the other.

For a node whose children are named slots that is exactly right. An `If` is
`[condition, then, else]`, and comparing a then-branch against an else-branch
would be comparing the wrong things.

For a node whose children are a list it is wrong, and expensively so. A
`Block` is a sequence of statements. Insert one statement near the top and
every statement after it lines up against its neighbour, disagrees, and
contributes nothing. Two blocks that a reader would call obviously duplicated
scored as though they shared only their outermost node.

Measured on four-statement blocks differing by one insertion: `2/11`. The same
pair under alignment: `10/11`. The default threshold is `0.8`.

The score was measuring alignment as much as content, and no flag changed it.

## Decision

Children are compared according to what they are.

`Block`, `Tuple` and `Array` hold homogeneous lists, and their children are
aligned by a longest-common-subsequence weighted by how well each candidate
pair matches -- not by whether the pair is equal.

Every other kind keeps positional comparison.

## Forcing constraints / Evidence

The three sequence kinds were chosen by reading the normaliser rather than by
guessing. Each is built by mapping over a syntax list with nothing in front of
it. `Call` is `[callee, arg0, ...]` and `Match` is `[scrutinee, arm0, ...]`;
both carry a header child, so aligning them freely would let a callee match an
argument or a scrutinee match an arm.

Keeping slots positional is not caution, it is correctness, and a test says
so. An `If` with only a then-branch and an `If` with only an else-branch have
identical multisets of children. Under free alignment they score `1.0` --
identical. Under slotted comparison they score `2/3`, matching on the
condition and the `If` itself, which is what they actually share.

The weighting matters as much as the alignment. A plain LCS over equal
children would pair only identical statements; weighting each candidate pair
by its own recursive match score means two statements that are themselves
near-duplicates still contribute, which is the case the tool exists to find.

Alignment preserves the bound `PairScanner` depends on. Each child of one list
is paired with at most one child of the other, so `matching` still cannot
exceed the smaller tree's node count, and `2 * min / (min + max)` is still a
ceiling.

Cost is `O(n * m)` per node pair against the previous `O(min(n, m))`. Measured
over this repository -- 564 units, 8,654 lines -- the full analysis takes
0.29s in release. The size filter already bounds which pairs are compared at
all.

## Rejected alternatives

**Align every kind's children.** Simpler to describe and wrong, per the `If`
evidence above.

**Full tree-edit distance.** Strictly more capable: it would also handle a
statement being wrapped in an `if`, which alignment does not. Rejected for
now on cost and on complexity -- the gain over weighted LCS is small for the
duplication this tool reports, and the implementation is considerably harder
to defend line by line.

**Make the sequence set configurable.** Rejected: which kinds hold lists is a
fact about the normaliser, not a preference. A flag would invite a wrong
answer.

**Keep zipping and document it.** What the previous release did. The
documentation was accurate and the behaviour was still surprising, which is
the definition of a defect rather than a limitation.

## Consequences

Scores go up for pairs that differ by insertion or deletion inside a block,
which is the common shape of real duplication. More groups clear the
threshold; every one of them was a duplicate before and simply scored too low
to be reported.

A group's reported similarity is now closer to what a reader would estimate by
eye, which makes the threshold easier to choose.

Nothing about exact detection changes -- fingerprints are unaffected by how
similarity is computed.

## Enforcement

`tests/near_duplicate/similarity_tests.rs` holds four cases: a block with an
inserted statement, a block with one removed, a tuple with an inserted
element, and the `If` whose branches must not cross-match. The first three
fail against positional comparison; the fourth fails against unrestricted
alignment.

## Related

- [ADR-SizeFilterIsAProvableBound](ADR-SizeFilterIsAProvableBound.md)
- [FORMULA.md](../FORMULA.md)
