# ADR-ThresholdsCarryTheirRange

## Status

- **Status:** Accepted
- **Date:** 2026-08-23

## Context

`Config` had twelve public fields and nothing enforcing any of them. Four of
those fields have a range that is not the range of their type:

| field | means | accepted |
|---|---|---|
| `similarity_threshold` | a Dice score to clear | any `f64` |
| `max_exact_percent` | a share of lines | any `f64` |
| `max_near_percent` | a share of lines | any `f64` |

Out-of-range values were accepted from all three sources -- `dry4rust.toml`,
`[package.metadata.dry4rust]`, and the command line -- and the damage was
silent in the worst possible direction.

`--threshold 5` is the sharpest case. The size pre-filter is derived from the
threshold: a pair is scored only when `2 * min(|a|, |b|) / (|a| + |b|) >= t`,
which for `t > 1` is never. So every pair is discarded before it is compared,
and the report says

```
Near duplicates:  0 groups (0 code units)
```

That is the same sentence a genuinely clean codebase produces. A CI run
configured that way passes forever and reports nothing, and nothing in the
output says why.

`--max-exact-percent 150` is a ceiling nothing can breach. `-5` is a ceiling
everything breaches. Neither was reported as anything.

## Decision

Every threshold is a `Threshold`, a newtype over a fraction in `0.0..=1.0`
that cannot be constructed out of range.

```rust
Threshold::fraction("similarity_threshold", 0.85)?   // 0.0 ..= 1.0
Threshold::percent("max_exact_percent", 85.0)?       // 0.0 ..= 100.0
```

One type with two constructors, because a similarity to clear and a share of
lines to stay under are the same quantity written two ways. `as_fraction` and
`as_percent` read it back in either.

`Config::load` and `CliOverrides::apply_to` are fallible and name the field and
the value:

```
similarity_threshold must be a fraction between 0.0 and 1.0, got 5
```

The process exits 2 -- an error, distinct from the 1 a breached ceiling
produces.

## Forcing constraints / Evidence

**The fields stay public and settable.** The invariant is in the type, not in a
constructor, so there is nothing to bypass: a `Config` assembled by a library
caller with struct-update syntax cannot hold a similarity threshold of five,
because there is no such `Threshold` to assign.

**The range test is containment, not comparison.** `!(value < 0.0 || value >
1.0)` admits `NaN`, because every comparison against `NaN` is false, and a
`NaN` threshold makes every score comparison false in turn -- the same silent
emptiness as five, arrived at differently. `(0.0..=1.0).contains(&value)`
rejects it, and `fraction_of_nan_is_rejected` pins that.

**A file that cannot be read is still passed over.** `Config::load` ignores an
unreadable or unparseable `dry4rust.toml`, because a project with no
configuration is the ordinary case and looks identical.
`load_over_an_unparseable_file_falls_back_to_the_defaults` pins that
distinction: only a value the tool can read and cannot honour is an error.

**The counts are left alone.** `min_nodes`, `min_lines` and `min_sub_nodes` are
`usize`, so the impossible half of their range is already gone, and a floor of
zero admits everything -- a choice, not a mistake. There is no upper bound to
state that would not be invented.

## Rejected alternatives

**Clamp instead of reject.** Silently turning `5` into `1.0` produces a run
that does something the caller did not ask for and does not say so. The whole
defect being fixed is a silence.

**Validate in `check` only.** `--threshold` affects `report` and `stats` too,
and it is the one whose failure is invisible. Validating at the gate and not at
the report would leave the quiet case quiet.

**A separate `Percentage` newtype.** Two types for one quantity, and every
call site would then have to know which of the two a given field wanted.
Rejected in favour of two constructors over one type, which is where the
difference actually lives -- in how the number is written, not in what it is.

**`Result<_, String>` from the constructors.** Rejected: the error names a
field and a range, and those are data. `Error::InvalidConfig { field, value,
expected }` lets a test assert on the field rather than on a sentence.

## Consequences

`Config::load` and `CliOverrides::apply_to` return `Result`. Callers that
treated configuration as infallible must now say what they do when it is not;
inside this crate that is one `map_err` at the CLI boundary, which turns it
into `CliError::InvalidConfig` and exit code 2.

`CheckThresholds` holds `Option<Threshold>` rather than `Option<f64>`, and is
built through `CheckThresholds::new`, which is where `--max-exact-percent` is
checked.

A `dry4rust.toml` that has been quietly wrong until now will start failing.
That is the point: it was not doing what it said.

## Enforcement

`tests/threshold_tests.rs` holds both ranges, both ends of each, `NaN`, and the
equality of `fraction(0.05)` with `percent(5.0)`. `tests/config_tests.rs` holds
the file-level rejections and the unparseable-file fallback.
`tests/cli/cli_overrides_tests.rs` and `tests/cli/check_command_tests.rs` hold
the two command-line rejections end to end, including the exit code.

## Related

- [ADR-SizeFilterIsAProvableBound](ADR-SizeFilterIsAProvableBound.md)
- [FORMULA.md](../FORMULA.md)
- [OPEN_POINTS.md](../OPEN_POINTS.md)
