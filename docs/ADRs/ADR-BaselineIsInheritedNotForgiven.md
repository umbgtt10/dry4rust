# ADR-BaselineIsInheritedNotForgiven

## Status

- **Status:** Accepted
- **Date:** 2026-08-23

## Context

`check` measures a whole codebase against four ceilings. On a repository that
already has duplication, every ceiling is either breached on the first run or
set so loose that nothing can breach it. Neither is a gate.

The tool had one answer, and it was the wrong shape for the problem.
Suppression by fingerprint -- `cargo dry4rust ignore <fp> --reason "..."` --
says *this duplicate is deliberate and should stay*. Inherited duplication is
not deliberate. It is duplication nobody has got to yet, and recording it as
intentional, one fingerprint at a time with a reason invented to fill the
field, is a lie written into a file that outlives the person who wrote it.

Every other tool in this family already answers this: `cargo stern4rust` takes
`--baseline <PATH>` and `--write-baseline`, and its summary always states how
many offences the baseline suppressed. `dry4rust` is the one that could not be
adopted by a codebase that has the problem it detects.

## Decision

A baseline records the duplication a codebase already has, so `check` fails on
what is added.

- `cargo dry4rust baseline` records every group it finds into
  `dry4rust-baseline.json`, or into `--baseline <PATH>`, or into the path
  `dry4rust.toml` names. `--dry-run` says what it would record and writes
  nothing.
- `--baseline <PATH>`, global, judges the run against that file. Groups it
  accounts for are not reported and cannot breach a ceiling.
- The summary states `Baseline: N groups suppressed` whenever a baseline is in
  effect, and the JSON summary carries `baseline_suppressed`. With no baseline
  the line is absent and the field is absent, which is a different fact from
  zero.

An entry records the group's kind, fingerprint, member count and member names.
A group is accounted for when **kind and fingerprint match and the group has
not grown**:

```
  admits(entry, kind, group) =
        entry.kind == kind
     && entry.fingerprint == group.fingerprint
     && group.members.len() <= entry.members
```

Recording judges nothing. `baseline` runs the analysis with the baseline left
out, so re-recording an unchanged codebase produces the same file.

## Forcing constraints / Evidence

**The member count is load-bearing, not decoration.** An exact group is keyed
by the fingerprint its members share, and that fingerprint does not change when
a third copy joins. A baseline keyed on fingerprint alone would inherit every
future copy of every function it recorded -- the exact case a duplication gate
exists to catch. Proved rather than argued: a fixture with three copies is
recorded, a fourth identical copy is added, and
`baseline_then_check_passes_on_what_was_inherited_and_fails_on_what_is_added`
sees the check fail with the group at four members. The near-duplicate case
needs no such care, because a near group's fingerprint is composite over its
sorted members and changes when the membership does.

Shrinking is allowed in the other direction. Deleting one of three copies
leaves a group the baseline still admits, because progress is not something to
fail on.

**The kind is recorded because two of the four sets share a fingerprint
space.** Exact groups and sub-function exact groups are both keyed by a raw
unit fingerprint. Without the kind, a recorded branch could stand in for a
whole function that happened to hash the same.

**A baseline that cannot be applied is an error, never an empty one.** Missing
file, malformed JSON, and a `version` this build does not read all fail the
run and name the command that would record a new one. The alternative --
treating an unreadable baseline as suppressing nothing -- turns every inherited
finding into a new one at once, and a typo in a CI flag would read as a
codebase with nothing inherited. `version` is in the file because this
repository has already invalidated every fingerprint it had written once; see
[ADR-TheFingerprintFormatIsOwned](ADR-TheFingerprintFormatIsOwned.md).

**Entries are ordered by kind and then fingerprint.** The file is committed, so
a re-record of an unchanged tree has to diff as nothing at all. Same
requirement, same answer as
[ADR-DeterministicGrouping](ADR-DeterministicGrouping.md).

## Rejected alternatives

**`--write-baseline` as a global flag, spelled as `stern4rust` spells it.**
Rejected: `dry4rust` is the only tool in the family with subcommands, and
`cargo dry4rust report --write-baseline` reads as a report that also writes a
file rather than as a recording. `baseline` sits beside `ignore`, `ignored` and
`cleanup`, which are already verbs on a suppression file. The concept, the file
naming and the never-hidden count are the family's; the spelling is this CLI's.

**Detect `dry4rust-baseline.json` automatically when it is present.** Rejected:
a file left in a working tree would then silently suppress findings, and
nothing in the output distinguishes "no duplication" from "a baseline you
forgot about". Naming it in `dry4rust.toml` is one line and is a decision a
reviewer can see.

**Suppress failure but keep reporting.** Rejected: `report` and `check` would
then disagree about what exists, which is the inconsistency
[ADR-KindDoesNotRestrictComparison](ADR-KindDoesNotRestrictComparison.md)
settled in the other direction. The suppressed count keeps the total visible
without printing the groups.

**Extend the ignore file with a `baseline = true` flag on each entry.**
Rejected: the two files differ in who writes them. An ignore entry is written
by a person and carries a reason in their words; a baseline is written by the
tool and read back by it. Merging them means a machine rewriting a file a human
maintains, and `cleanup` pruning entries a human had not finished with.

**Record line numbers so a moved duplicate is a new one.** Rejected: this tool
is structural by construction, and a finding that changes because code moved
down a file would fail the actionability principle in
[ROADMAP.md](../ROADMAP.md).

## Consequences

A codebase with existing duplication can adopt `check --max-exact 0` on the day
it installs the tool. That is the point.

A baseline that is never re-recorded hides duplication indefinitely. The
suppressed count in every summary is what keeps that visible, and re-recording
is one command.

Shrinkage is admitted for exact groups and not for near ones. A near group's
fingerprint is composite over its members, so losing one changes the identity
and the group is reported as new even though it got smaller. That is a property
of the composite fingerprint rather than of this decision, and it is recorded
in [OPEN_POINTS.md](../OPEN_POINTS.md).

`analyze_units` now returns an error where it could not before, because
loading the baseline can fail. `AnalysisOutput` gained a second constructor,
`produce_ignoring_baseline`, which is what `baseline` records with.

`DuplicationStats` gained `baseline_suppressed: Option<usize>`. The JSON
summary is unchanged for anyone not using a baseline, because the field is
skipped when absent -- the same treatment the sub-function counts already had.

## Enforcement

`tests/baseline/` holds the matching rules: growth, shrinkage, a foreign
fingerprint, a foreign kind, and the three ways a baseline fails to load.
`tests/cli/baseline_command_tests.rs` holds the whole loop through the binary
-- fail, record, pass, add a copy, fail again -- and pins that a second
recording writes the same bytes as the first.

## Related

- [ADR-TheFingerprintFormatIsOwned](ADR-TheFingerprintFormatIsOwned.md)
- [ADR-DeterministicGrouping](ADR-DeterministicGrouping.md)
- [ADR-KindDoesNotRestrictComparison](ADR-KindDoesNotRestrictComparison.md)
- [ROADMAP.md](../ROADMAP.md)
