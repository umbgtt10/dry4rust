# ADR-ForkedNotCopied

## Status

- **Status:** Accepted
- **Date:** 2026-08-23

## Context

`dry4rust` needed a duplicate-detection engine for Rust. One already existed:
`cargo-dupes` by Matjaz Domen Pecan, MIT-licensed, published February 2026 --
`syn` AST normalisation, fingerprint hashing, Dice-coefficient similarity, a
threshold gate and a suppression file. The research dossier in
`docs/dry4rust-dossier.md` concluded it holds that ground competently and that
rebuilding it would produce a worse copy of a working tool.

MIT permits taking the code. It also requires the copyright notice to travel
with copies and with substantial portions. So the question was never whether
attribution was owed, only what form discharges it honestly.

Two forms were available. Copy the source files into a fresh repository and
credit upstream in prose, or fork the repository so upstream's commits remain
in the history of this one.

## Decision

Fork. Upstream's 28 commits are ancestors of this repository's history and
remain authored by Matjaz Domen Pecan. `LICENSE` carries both copyright
notices with upstream's first. `docs/header.txt` is four lines rather than the
three its sibling repositories use, because the fourth is upstream's notice,
and every `.rs` file carries it. The README credits the creator on its first
line of body text, above this repository's own description.

`git remote` keeps `upstream` pointing at `mpecan/cargo-dupes`.

## Forcing constraints / Evidence

Prose attribution is deniable and drifts. A fork is a fact in the object
graph: `git log --author` shows upstream's authorship on the commits that
introduced the engine, and no later edit can quietly remove it. A copy with a
credit line in the README has no such property -- the credit is one edit away
from gone, and nothing in the repository would notice.

The four-line header is enforced rather than intended. `cargo stern4rust`
applies its `header` rule to all 65 files with no exclusions beyond
`tests/fixtures/**`, so a file that loses upstream's notice fails the gate.

## Rejected alternatives

**Copy the sources, credit in the README.** Discharges the licence's letter at
the moment of copying and nothing after. Rejected because the obligation is
continuing and this form does not survive routine editing.

**Depend on `cargo-dupes` as a crate.** Would have kept attribution automatic
via the dependency graph, but the intended work -- semantic redundancy
detection, per the dossier -- changes the normaliser and the grouper. A
dependency cannot be modified, and vendoring it is the copy option again.

**Fork without keeping upstream's name in the file headers.** Rejected as a
licence violation, not a style preference: the notice must travel with
substantial portions.

## Consequences

Upstream's history is permanent here, including commits whose code has since
been deleted -- the tree-sitter and Python backends among them. That is the
intended cost.

Pulling from upstream stays possible but is a deliberate act. `CLAUDE.md`
names it as the one exception to the self-contained boundary rule.

The GitHub fork relationship makes this repository visible on upstream's
network page. No notification is sent to the owner.

## Enforcement

`cargo stern4rust`'s `header` rule, run first in `just stage2`, fails on any
`.rs` file that does not carry both notices.

## Related

- [ADR-SingleCrateSingleLanguage](ADR-SingleCrateSingleLanguage.md)
- `docs/dry4rust-dossier.md` -- why an engine already existing was the
  starting condition
