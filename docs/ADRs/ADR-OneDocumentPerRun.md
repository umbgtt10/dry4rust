# ADR-OneDocumentPerRun

## Status

- **Status:** Accepted
- **Date:** 2026-08-24

## Context

`--format json` did not produce JSON.

`report` wrote the summary object, then one bare array per section, each as its
own top-level value:

```
{ …stats… }

[ …exact… ]
[ …near… ]
```

Nothing parses that in one call. `jq .` fails. A reader has to stream-decode
concatenated values or split on section boundaries, and since empty sections
are omitted, the number of values varies with what was found -- so a positional
reader cannot tell which section it is holding.

`check` was worse. It wrote the summary object, then

```
Check FAILED: 1 exact duplicate groups (max: 0)
```

as prose, between two JSON values. No parser reads that at all, and `check` is
the command most likely to be piped into one.

The cause is one thing, not two. `Reporter` had five methods that each wrote a
section, and the *command* called them in order. That works for a format whose
parts stand alone on the page. It cannot work for a format that has to be a
single value, because by the time the second call arrives the first has already
been written.

## Decision

The document is the format's business, not the command's.

`Reporter` gains two methods that take a whole run:

```rust
fn report(&self, report: &Report<'_>, writer: &mut dyn io::Write) -> io::Result<()>;
fn report_check(&self, stats: &DuplicationStats, breaches: &[CheckBreach<'_>],
                writer: &mut dyn io::Write) -> io::Result<()>;
```

`ReportCommand` and `CheckCommand` hand over everything and render nothing.
`TextReporter` writes section after section, exactly as before. `JsonReporter`
builds one object and serialises it once.

| command | keys |
|---|---|
| `stats` | the summary fields at the top level, unchanged |
| `report` | `stats`, `exact`, `near`, and `sub_exact`/`sub_near` when found |
| `check` | `stats`, `passed`, `breaches`, and `exact`/`near` when breached |

`exact` and `near` are always present on `report`; the sub-function sections
are absent rather than empty when that analysis was not asked for, so `[]`
never stands in for "not looked at". This is the treatment the sub-function
*counts* already had in the JSON summary.

## Forcing constraints / Evidence

**The verdict had to move inside the document.** `check` printed its breach
sentences with `writeln!` straight to the writer, which is why its output was
unparseable rather than merely awkward. They are now `breaches` in the object,
and `passed` states the verdict a reader would otherwise infer from an exit
code they cannot see.

**Text output is unchanged, and the tests prove it.** The restructure was made
with every existing text assertion in place; the only two tests that failed
were the two asserting the old multi-document JSON shape, and they were updated
to the new contract rather than the contract bent to them.

**Gathering the groups fixed a second defect.** Text lists the offending groups
under each breach, so two exact ceilings breached together printed the same
array twice. A document names them once. `json_report_check_gathers_the_groups_rather_than_repeating_them`
pins that, and the text behaviour is pinned separately.

**Splitting the text formatter was forced, not chosen.** Moving document
assembly into `TextReporter` pushed `src/output/text.rs` to 10.63 on
`iceberg4rust`, over the ceiling of 10. `write_groups` was a private function
with six parameters and complexity 11 -- four of those parameters being the
four ways the four sections differ. It became `GroupSection`, with a
constructor per section. Same answer as
[ADR-DecompositionOverThresholdRelaxation](ADR-DecompositionOverThresholdRelaxation.md):
split the function, do not raise the number.

## Rejected alternatives

**Buffer inside `JsonReporter` and flush at the end.** Would need interior
mutability and a flush point nobody can forget, and `report_stats` is still
called on its own by `stats`. Rejected: it hides the ordering constraint
instead of removing it.

**Emit JSON Lines, one value per line.** A real format with real tooling, and
it would have made the old output legal by declaring it so. Rejected: the
sections are not homogeneous records, which is what JSON Lines is for, and a
`stats` object followed by two arrays of different things is not a stream.

**Leave `check` alone and fix only `report`.** Rejected: same defect, same
cause, and `check` is the one CI pipes into `jq`. Fixing the reported half and
leaving the worse half would have needed a new open point explaining why.

**Always emit `sub_exact: []` and `sub_near: []`.** Simpler for a consumer that
wants to index blindly. Rejected: sub-function analysis is opt-in, and an empty
array reads as "looked and found none" rather than "never looked" -- the same
kind of false measurement as
[reporting a threshold where no pair was scored](../OPEN_POINTS.md).

## Consequences

Breaking for anyone parsing `--format json report` by splitting on blank lines.
Free to make now, and expensive later: 0.2.0 is not published, so this lands
before the first release with an engine in it rather than as a major bump after
it.

`Reporter` has seven methods where it had five. The five remain because
`GroupSection`-per-section output is still how a text section is written and
how a test drives one.

`src/output/` gained three types: `Report`, `CheckBreach` and `GroupSection`.

## Enforcement

`tests/output/report_tests.rs` asserts a single parse reads the whole report,
in both formats. `tests/cli/check_command_tests.rs` asserts the same for
`check` in both the passing and the breached case, and that the verdict is in
the document. `tests/output/group_section_tests.rs` holds the four sections'
differences, including that an empty sub-function section says nothing.

## Related

- [ADR-DecompositionOverThresholdRelaxation](ADR-DecompositionOverThresholdRelaxation.md)
- [ARCHITECTURE.md](../ARCHITECTURE.md)
- [OPEN_POINTS.md](../OPEN_POINTS.md)
