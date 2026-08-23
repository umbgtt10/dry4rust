# ADR-TheFingerprintFormatIsOwned

## Status

- **Status:** Accepted
- **Date:** 2026-08-23

## Context

A fingerprint is written into `.dry4rust-ignore.toml` with a reason, and is
expected to still identify the same code the next time the tool runs --
possibly weeks later, on a different machine, under a different toolchain.

Upstream computed it with `std::collections::hash_map::DefaultHasher` over
derived `Hash` implementations. Three things in that arrangement are outside
this repository's control, and all three are load-bearing for a persisted
identifier:

- **The algorithm.** The standard library states plainly that
  `DefaultHasher`'s internal algorithm is unspecified and that its hashes
  should not be relied upon across releases.
- **The width and byte order of integers.** `Hash for usize` writes native-
  endian bytes at the platform's word size, so a 32-bit machine and a 64-bit
  one disagree about the same tree.
- **The encoding of enum variants.** A derived `Hash` writes the variant's
  *position*, so reordering `NodeKind` -- or inserting a variant in the middle
  of it -- silently changes every fingerprint in the corpus.

The failure mode is the quiet one. A suppression stops matching, the duplicate
reappears in the report, and nothing anywhere says why.

## Decision

The fingerprint format lives here.

`StableHasher` is FNV-1a 64-bit, eight lines, with the offset basis and prime
named as constants. Integers are written big-endian at a fixed eight bytes;
strings are written length-first.

`NodeEncoder` walks a `NormalizedNode` and writes each variant as **its own
name** rather than its position, followed by whatever payload it carries, then
the child count, then the children. Every match in it is exhaustive over its
enum.

## Forcing constraints / Evidence

Naming variants rather than numbering them is what makes the format survive
ordinary refactoring. `NodeKind` has 61 variants and will gain more; under the
derived encoding, adding one in the middle invalidated every suppression in
every consuming repository. Under this one, adding a variant is a compile
error in `NodeEncoder` -- the match is exhaustive -- and existing fingerprints
are untouched.

The child count prefix is not decoration. Without it `[[a], b]` and `[a, [b]]`
flatten to the same byte stream. The string length prefix does the same job
for adjacent variant names, keeping `("ab", "c")` from colliding with
`("a", "bc")`.

The algorithm is checked against the published FNV-1a vectors rather than
against itself: the empty hash is the documented offset basis, `"a"` is
`0xaf63dc4c8601ec8c`, `"foobar"` is `0x85944171f73967e8`. A test that only
compared the implementation with its own previous output would have passed for
any algorithm at all.

FNV-1a is weaker than SipHash against adversarial collisions, which is not a
threat model here -- nobody is crafting Rust functions to collide a
duplication report. At 64 bits and realistic corpus sizes the birthday
probability is negligible.

## Rejected alternatives

**Keep `DefaultHasher`, override the integer widths.** Implementing
`std::hash::Hasher` with fixed-width big-endian writes fixes platform
dependence and lets the derives keep working. Rejected: it leaves the variant
*position* encoding in place, which is the part that breaks under ordinary
refactoring.

**Add a hashing crate.** `rustc-hash`, `twox-hash` and friends are stable in
practice, and none of them promises stability across its own major versions
either. A dependency also cannot solve the variant-position problem, which
lives in the derive rather than the hasher.

**Encode via `Debug`.** Derived `Debug` gives variant names and payloads for
free, covers new variants automatically, and is stable in practice. Rejected:
it trades one undocumented format for another, and a fingerprint is exactly
the wrong place to depend on a formatting implementation detail.

**Version the format and support both.** Reading old fingerprints under the
old scheme and writing new ones under the new. Rejected as the wrong trade at
this point in the tool's life: there is one repository with suppressions --
this one -- and carrying a compatibility shim forever to avoid one prune is a
bad bargain.

## Consequences

Every existing fingerprint changed, once. This repository's own
`.dry4rust-ignore.toml` held fifty entries and all fifty went stale; `cleanup`
pruned them in the same commit. Their reasons are preserved in that commit's
message and in git history.

This is the cost `OPEN_POINTS.md` predicted and the reason it argued for
paying it early: the longer the format stays borrowed, the more suppression
files a change like this invalidates.

Anyone with a suppression file must re-record their suppressions. There is no
migration path and deliberately so -- a fingerprint that silently maps to a
different piece of code would be worse than one that plainly no longer
matches.

## Enforcement

`tests/stable_hasher_tests.rs` pins the algorithm against published vectors
and the integer encoding against a hand-spelled big-endian byte sequence.
`tests/node_encoder_tests.rs` pins that nesting, ordering, placeholder index,
placeholder position, macro name and reference mutability all reach the hash.

## Related

- [ADR-SequenceChildrenAreAligned](ADR-SequenceChildrenAreAligned.md)
- [FORMULA.md](../FORMULA.md)
