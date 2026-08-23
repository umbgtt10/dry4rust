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

## Near-duplicate detection is restricted by kind; exact detection is not

`group_exact_duplicates` compares fingerprints and nothing else, so a free
function and a method with identical normalised signature and body are
reported as exact duplicates.

`NearDuplicateFinder` groups candidates by `CodeUnitKind` first and never
compares across groups. The same free function and method, differing by one
statement, are never compared at all.

So the two halves of the tool disagree about whether kind matters. Identical
across kinds is a finding; nearly identical across kinds is invisible. Nothing
in the output distinguishes "no near duplicates" from "no near duplicates
within a kind".

Whether the restriction should go is a real question rather than an obvious
fix. Removing it compares `Function` against `Method` against `Closure`, which
is likely wanted, and under sub-function analysis also compares `IfBranch`
against `MatchArm` against `LoopBody`, which may be noise. The size filter now
bounds the extra work, so cost is not the objection it once was.

Unresolved deliberately. It changes what the tool reports.

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
