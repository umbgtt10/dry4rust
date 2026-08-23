# ADR-VisibilityIsNotAnEscapeHatch

## Status

- **Status:** Accepted
- **Date:** 2026-08-23

## Context

`cargo stern4rust`'s `tested-public-api` rule requires every `pub` item to be
reached by a test. At the fork point, 28 items failed it -- functions
reachable from outside the crate with nothing calling them.

There are two ways to make that rule pass. Write the test, or drop `pub` to
`pub(crate)` so the rule stops applying.

The second is faster and, on a repository whose tests are all integration
tests under `tests/`, quietly wrong: making an item crate-private does not
make it tested, it makes it untestable, because an integration test is an
external consumer.

## Decision

`tested-public-api` is answered with tests. Visibility is never reduced to
silence it.

All 28 items got tests -- 52 of them. None was made private.

Where an item genuinely should not be public, the honest answer is to delete
it, not to hide it.

## Forcing constraints / Evidence

This repository's tests live in `tests/` and are compiled as separate crates.
`pub(crate)` is therefore not a smaller public surface, it is an unreachable
one: the item leaves the report and simultaneously leaves the reach of every
test that could have covered it.

Two other gates close the same escape. `twin4rust` requires a mirrored test
file per source file regardless of what is inside it, and `crap4rust` measures
complexity against coverage for private functions too -- so a complex item
hidden behind `pub(crate)` still scores, and still fails at 15.

The tests written were not smoke tests. The normaliser functions got
alpha-equivalence assertions: two closures differing only in parameter name
must normalise to the same node, and two blocks of different shape must not.
Every `cmd_*` in `cli.rs` takes a writer, so each is driven against a fixture
and its output read back.

One of those tests corrected a wrong belief about the code rather than
confirming a right one. `normalize_type` was asserted to keep `i32` and
`Vec<String>` apart; it does not, because types become positional
placeholders, so in fresh contexts both are `TypePlaceholder(Type, 0)`. The
contract is about one shared context, and the test says that now.

## Rejected alternatives

**Drop `pub` to `pub(crate)` for items with no external caller.** Rejected as
described: it removes the item from the report and from every test's reach in
the same edit.

**Add `#[allow]`-style suppression per item.** `stern4rust` offers a baseline
for exactly this. Rejected because a baseline is a promise to come back, and
28 items is a size that can simply be done.

**Add one trivial test per item to clear the rule.** Rejected as satisfying
the letter. A test that calls a function and asserts it returned something is
coverage without a contract.

## Consequences

The public surface is large and every part of it is exercised. Adding a `pub`
item now costs a test, which is the intended price.

`Fingerprint::new` exists because a test needed to name a specific hash value
and the alternative was exposing the tuple field. A constructor is a smaller
concession than a public field.

## Enforcement

`cargo stern4rust`'s `tested-public-api` rule, with no baseline and no skip.
`CLAUDE.md` states the rule in prose for anyone working here.

## Related

- [ADR-DecompositionOverThresholdRelaxation](ADR-DecompositionOverThresholdRelaxation.md)
- [ADR-FixturesAreTheCorpusNotTheCode](ADR-FixturesAreTheCorpusNotTheCode.md)
