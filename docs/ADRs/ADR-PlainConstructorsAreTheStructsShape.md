# ADR-PlainConstructorsAreTheStructsShape

## Status

- **Status:** Accepted
- **Date:** 2026-10-23

## Context

stern4rust's `private-fields` rule (R035) makes every struct field private, so every struct is built through a
constructor. Most of those constructors only move their parameters into the struct: `fn new(call: PublicEntryPoint,
line: usize, visibility: String, owner: String) -> Self { Self { call, line, visibility, owner } }`.

Normalized, two such constructors with as many fields are the same tree whatever they build, and fingerprint alike.
The first day of R035's adoption in stern4rust showed it: `RestrictedMethod::new` and `TestFileItem::new`, four fields
each and nothing else in common, were reported as one exact-duplicate group and turned the gate red. Across the family
the rule adds hundreds of constructors, so every repository adopting it would have met the same wall -- a group per
field count, growing with each struct.

What such a group counts is fields. There is no logic in it to copy, nothing a change would have to make twice, and
no shared helper that could replace it: the struct's shape is the constructor's whole content.

## Decision

**A plain constructor is not a code unit.** A method in an inherent impl is left out of the analysis when its body
is a single expression building the impl's own type -- written `Self` or by name -- from its parameters alone:

- `Self { x, y }`, by shorthand;
- `Self { start: first, end: last }`, each field given a parameter;
- `Self(left, right)`, for a tuple struct.

Anything more makes it a code unit as before: a parameter converted (`name.to_string()`), a field defaulted (`None`,
`Vec::new()`), a base expression (`..Self::default()`), a statement before the value, another type built.

`PlainConstructor::is_plain` decides; `CodeUnitExtractor` asks it for each method of an inherent impl.

## Rejected alternatives

**An ignore entry per field count.** `.dry4rust-ignore.toml` with a reason, in every adopting repository, for every
arity that appears. It works, and it makes each repository carry a growing list of the same fact. Kept only as a
bridge until a release carries this decision.

**A baseline re-recorded to hold the groups.** It would call a constructor inherited duplication, and a baseline that
grows with every struct is not one a check can trust.

**Writing constructors differently so their shapes differ.** Code shaped to please the checker.

**Leaving out trait impls' plain functions too.** `impl From<i32> for Meters { fn from(value: i32) -> Self {
Self(value) } }` is as plain, but R035 does not force it, and a trait's function is the trait's contract rather than
the struct's shape. Left in until a codebase shows the need.

## Consequences

A codebase adopting private fields keeps a gate it can meet without exceptions. A constructor that does any work is
still compared, so the duplication worth finding -- the same conversion written twice -- is still found. A baseline
recorded before this change may hold a group a plain constructor belonged to; `check` admits the shrink, and a
re-record drops it.

## Amendment (2026-10-25)

Before it was released, stern4rust's `private-fields` rule became `records-and-objects` (R035 v2). A struct with no
behaviour -- a record -- keeps its fields `pub`, has no getter and no plain constructor, and is built by literal; only
an object, which has behaviour, keeps its fields private behind getters and the constructors it needs. The plain
constructors this decision answers are therefore every object's rather than every struct's -- fewer of them, and the
same shape -- and the decision stands for them unchanged. The history above is left as it was decided.
