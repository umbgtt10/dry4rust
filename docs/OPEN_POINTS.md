# Open Points

Known limits of the current model. Each is a real property of the code as it
stands, not a wish list -- what to build next is
[ROADMAP.md](ROADMAP.md).

None of these is a bug in the sense of "the code does not do what it says".
They are places where what it says is narrower than a reader might assume.

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

## A sub-function unit reports its parent's line range

With `--sub-function`, each extracted branch, arm or loop body carries the
line range of the function it came from, not its own. A match arm inside a
twenty-line function is reported at the function's twenty lines.

The member line is still unambiguous about *which* branch -- it names the kind
and the ordinal, `match arm 2 (match arm) in classify_number` -- but the range
cannot be read straight into an editor and landed on the finding.

It also means the duplicated-line counts treat a sub-function group as though
it spanned its parents in full, which overstates them.

## A near-duplicate group that loses a member is new duplication to a baseline

A near group's fingerprint is composite over its sorted member fingerprints,
which is what makes suppression of a near group possible at all. It also means
the identity changes whenever the membership does -- in either direction.

For a baseline that is right in one direction and wrong in the other. Adding a
member changes the fingerprint, so the grown group is reported, which is what
should happen. Removing one also changes the fingerprint, so a group that
*shrank* is reported as though it were new.

Exact groups do not have this: their fingerprint is the one their members
share, so the recorded member count carries the growth and shrinkage is
admitted. The asymmetry is between the two kinds of group, not between the two
directions.

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
