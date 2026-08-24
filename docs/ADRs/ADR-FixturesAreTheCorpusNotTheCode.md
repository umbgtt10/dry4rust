# ADR-FixturesAreTheCorpusNotTheCode

## Status

- **Status:** Superseded by [ADR-CorpusOutsideThePackage](ADR-CorpusOutsideThePackage.md)
- **Date:** 2026-08-23

The exclusion this ADR argued for no longer exists. The reasoning held --
the corpus is input and must not be edited to satisfy a rule about source --
but excluding it from the rule was the weaker of the two ways to honour that.
Moving it out of the package removes it from the rule's reach entirely, and
fixed a packaging defect the exclusion had been hiding. See the superseding
ADR.

## Context

`cargo stern4rust` enforces twenty-one house rules across this repository,
including a four-line copyright header on every `.rs` file. All twenty-one are
applied, none is skipped, and no baseline file exists.

`tests/fixtures/` holds small Rust crates -- `exact_dupes`, `near_dupes`,
`no_dupes`, `mixed`, `sub_function_dupes` -- that are not this tool's source.
They are its input. The tests parse them, count the units found, and assert on
the numbers.

A header rule applied to them would add four lines to every fixture file.

## Decision

`stern4rust.toml` carries one exclusion: `tests/fixtures/**`.

It is the only exclusion in the repository.

## Forcing constraints / Evidence

Stamping headers into the fixtures changes the measurement. Several tests
assert on line counts and node counts drawn from those files, so four lines
per file would have moved figures the tests exist to pin. The choice was
between editing the answers and editing the question, and neither is
acceptable; excluding the corpus leaves both alone.

The rules themselves also do not mean anything here. `paired-test-file` would
demand a test file mirroring a fixture that exists to be parsed, not called.
`test-file-structure` would demand alphabetic ordering inside crates whose
value is partly that their contents are arranged to produce specific
duplicate groupings.

The exclusion is visible rather than silent. `cargo stern4rust` reports
`files_excluded=6` alongside `files_scanned=65`, so a reader sees both what
was checked and what was not.

## Rejected alternatives

**Stamp the headers and update the affected tests.** Rejected under this
repository's assertion-integrity rule: the assertions would be changed to fit
a change that has nothing to do with what they measure.

**Move the fixtures outside `tests/`.** Would dodge the rules by relocation
while leaving the same files unheaded somewhere else, and would break the
`fixture_path` helper for no gain.

**Skip individual rules per fixture package.** The approach taken in
`braintax4rust`, which has fixture crates carrying real acceptance tests. Here
the fixtures carry no tests of their own, so a path exclusion says the simpler
and truer thing: these files are data.

**Generate fixtures at test time instead of committing them.** Several tests
already do exactly this with `TempDir` and inline source. The committed
fixtures earn their place by being shared across many tests and by being
readable when a grouping assertion fails. Rejected as a wholesale replacement,
kept as the pattern for one-off cases.

## Consequences

Fixture files carry no copyright header. They are inputs, and a header inside
one would be part of the input.

Any new fixture is excluded automatically by the glob, so adding one does not
require touching the configuration.

If a fixture ever grows a genuine test of its own, this decision needs
revisiting -- the exclusion would then be hiding a test file.

## Enforcement

`stern4rust.toml` holds the exclusion, not the gate script, so a hand-run of
`cargo stern4rust` checks exactly what `just stage2` checks.

## Related

- [ADR-DecompositionOverThresholdRelaxation](ADR-DecompositionOverThresholdRelaxation.md)
