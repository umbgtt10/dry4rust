# dry4rust

## Meaning

`dry4rust` is a `cargo` subcommand that detects duplicated and near-duplicated
code in Rust, by normalising the AST, fingerprinting it, and comparing trees
with the Dice coefficient.

It is a fork of [`cargo-dupes`](https://github.com/mpecan/cargo-dupes) by
Matjaz Domen Pecan, MIT. The engine is his work; `LICENSE` carries both
copyright notices and every source file repeats them. See `README.md`.

`docs/ARCHITECTURE.md` maps the code and `docs/FORMULA.md` states the scoring.
`docs/ADRs/` holds the decisions this repository has settled and what forced
them; `docs/OPEN_POINTS.md` holds what is known to be wrong with the model,
which is worth reading before trusting a result. Where it is going rather than where it came from is in
`docs/dry4rust-dossier.md`, which is research and not a record.

## Boundary Rule

This repository is **SELF-CONTAINED**.

The LLM **SHALL NOT cross its boundaries without asking**.

That means:
- do not inspect, edit, or rely on files outside `dry4rust/` unless the user
  explicitly asks
- do not pull assumptions from sibling repositories or crates
- do not propose cross-repository changes by default

Upstream is the exception worth naming: `git fetch upstream` reaches
`mpecan/cargo-dupes`, and pulling from it is a deliberate act, not a default.

## Quality Gates

### Mandatory after every change

Run:

`just stage1`
`just stage2`

If either gate is not green, the work is not complete.

Both run identically on Windows, Linux and macOS, and CI runs the same two
commands -- there is no second definition of the gates to drift out of step.

Stage 1 is formatting, clippy and tests -- cargo built-ins only, so it works on
a fresh checkout. It runs with `RUSTFLAGS=-D warnings` and
`cargo clippy --workspace --all-targets`, which is stricter than a bare
`cargo test` in two ways: warnings are errors, and test targets are linted too.
Because `core` and `xtask` both set clippy's `pedantic` and `nursery` groups to
warn, that combination is what holds the tests to the same bar as the sources.

Stage 2 is `cargo xtask stage2` -- a real crate under `xtask/`, gated like any
other code, rather than a script. Each gate is a `Gate` implementation
constructed against a `CommandRunner` trait, so the argument lists and the
failure messages are covered by `xtask`'s own integration tests. It runs five
gates, in this order -- four installed cargo subcommands and this checkout's own
dry4rust, built from source:

| gate | asks |
|---|---|
| `cargo stern4rust` | do the house coding rules hold |
| own duplication (`dry4rust`, from source) | did this change add duplicated code |
| `cargo crap4rust` | is any function complex and untested |
| `cargo twin4rust` | does every source file have a mirrored test file |
| `cargo iceberg4rust` | is any file's private implementation risk too high |

stern4rust runs **first** because its corrections are renames, file moves and
directory splits: a layout it is about to reject is a layout the others would
have measured for nothing. Its findings are also the cheapest to act on.

dry4rust runs **second**, for the same reason: removing a duplicate moves code
between files, which changes what the three behind it measure. It is this
checkout's dry4rust, run with `cargo run --bin cargo-dry4rust`, never an
installed copy. A tool that enforces a check it does not pass is not worth
installing -- and a duplicate this checkout carries is one it is about to
publish.

It scans `core/src` only -- tests repeat their arrangement by design, and
`fixture/` is duplicated on purpose -- and checks with zero ceilings against
`dry4rust-baseline.json`, the duplication still there when the gate arrived. So
it fails on what a change adds, not on what it inherited; a duplicate removed is
admitted, a copy added to a recorded group is not. Exit 1 is the tool's own
"ceiling exceeded" and reads as duplication added; anything else -- 2 for a
configuration or a baseline it refused, cargo's 101 for a build that failed --
is reported as the exit code, never as a verdict.

It counts only code units of 25 AST nodes or more. Below that sit units whose
whole body is one call or one short struct literal -- accessors, two-field
constructors, the four `Reporter` section methods every format must write --
and at the default of 10 they form fifteen exact groups whose sameness is a
signature's, not a copy. At 25 there were four groups when the gate arrived.
Two were shared away: `TextReporter::section` now sits behind the four section
methods, and `Threshold::share_of` behind `fraction` and `percent`. The
baseline records the two that remain, neither of which sharing would improve:
the four-field constructors of `CommandDispatcher`, `CheckCommand` and
`BaselineCommand`, which store what they are given; and `NodeEncoder`'s
`placeholder_name` and `un_op_name`, two lookup tables over different enums
whose strings every fingerprint is computed from.

Re-record the baseline only to drop groups that are gone, never to admit new
ones, and at the same floor -- a baseline matches only at the floor it was
recorded at:
`cargo run --bin cargo-dry4rust -- dry4rust --path core/src --min-nodes 25 --baseline "$PWD/dry4rust-baseline.json" baseline`.

All twenty-two of stern4rust's rules apply to every member -- `core`,
`validation` and `xtask` -- with nothing skipped, nothing selected, nothing
baselined and nothing unconfigured. `stern4rust.toml` carries no `skip`, no
`rules`, no `baseline` and no `[package.*]` section, and the report ends
`baselined=0 ... rules_applied=22 rules_skipped=0 rules_unconfigured=0`.
`docs/header.txt` holds the four-line header every `.rs` file carries and
`stern4rust.toml` names it -- in the config rather than the gate script, so a
hand-run of `cargo stern4rust` checks exactly what the gate checks.

Four lines rather than the three its sibling repositories use, because MIT
requires upstream's copyright notice to travel with the code.

The stern gate passes no `--package` at all, so one call judges every member
and the report carries one roll-up `summary:` line. `xtask` is a member and is
gated like the rest. The crate that runs the gates is not exempt from them.

There is no stern baseline. `validation`'s tests drive the whole tool against
`fixture/`, so their subject is a scenario -- check, baseline, cleanup -- rather
than a file in `core/`. While `validation` had no `src/`, `paired-test-file`
reported its ten scenario files; the rule was stood down with a `skip`, then
held in a baseline. Both are gone. `validation/src/` holds the harness the
scenarios share -- `Fixture`, `DryBinary`, `TextReport`, `DuplicateGroupFixture`,
each with a test file of its own -- and one thin type per scenario:
`tests/check_scenario_tests.rs` pairs with `src/check_scenario.rs`, whose
`CheckScenario` holds the entry point those tests repeat and call. A scenario
added later gets the same pair -- a `src/<scenario>_scenario.rs` holding what
its tests repeat, and a `tests/<scenario>_scenario_tests.rs` calling it --
never a baseline entry.

Nothing is excluded. The corpus used to live under `tests/fixtures/` and had to
be excluded from the header rule; it now lives in `fixture/`, outside every
workspace member, so there is nothing for a rule to reach and nothing to
exclude. Its duplication is deliberate -- it is what the validation tests
measure -- and is never "fixed".

Stage 1 covers the workspace. The stern gate covers every member, which is one
step stricter than the rest of the family: the other workspaces here gate
`core` and leave `validation` unmeasured. `twin4rust` and `iceberg4rust` take
every member too, and in `validation` they have source to measure: each harness
and scenario file is mirrored and scored like core's.

`crap4rust` is the one gate that stays on core. `validation` has source -- the
harness and the scenario types its tests call -- but crap4rust drives coverage
with `-p validation`, which does not build core's binary for the tests that
spawn it. On a clean target they fail outright; on a used one they run whatever
binary an earlier run left behind, which measures nothing about this checkout.

`crap4rust` runs at 15 -- the family's number, with no override and no
tolerance. Nothing is baselined, skipped or budgeted.

That took decomposition rather than tests. CRAP is
`complexity^2 * (1 - coverage)^3 + complexity`, so it never falls below the
complexity itself: a function of complexity 27 cannot reach 15 at any
coverage, including 100%. Three inherited functions were in that position, and
each became a struct in a file of its own with a mirrored test file --
`NearDuplicateFinder`, `UnionFind`, `SimilarityPair`, `SubUnitExtractor`,
`CommandDispatcher`. If this gate ever needs an override again, the honest
move is the one taken here: split the function.

`iceberg4rust` runs at 10 rather than the default, and that one is a real bound
-- the worst file is 9.55.

`cargo install cargo-stern4rust`
`cargo install cargo-crap4rust`
`cargo install cargo-twin4rust`
`cargo install cargo-iceberg4rust`

cargo-dry4rust is deliberately not in that list; the gate builds it from this
checkout.

## Strict Rules

### Test preservation

**NEVER remove a test without explicitly asking the user first.**

If a code change causes a test to fail, the correct response is to update the
test assertion to reflect the new correct behavior — not to delete the test. A
test documents a contract. Deleting it silently removes coverage and hides
regressions.

The only acceptable reason to delete a test is if it is a deliberate exact
duplicate of another test that fully covers the same contract. Even then, ask
before deleting.

### Assertion integrity

**NEVER relax a test assertion without explicitly asking the user first.**

Relaxing means changing a specific assertion to a weaker one. If a code change
makes a previous assertion wrong, understand why, then update the assertion to
the new correct specific value — not to a weaker form that would pass
regardless of correctness.

### Visibility is not an escape hatch

`tested-public-api` asks for a test, not a smaller surface. Do not drop `pub`
to `pub(crate)` to silence it. If something is worth calling from outside, it
is worth a test; if it is not, deleting it is the honest answer.

## User coding standards

- one struct per file
- do not use fully qualified paths; use `use` imports instead
- unit tests are not allowed. Only integration tests are, under `core/tests/`
  for a single source file, or `validation/tests/` when the subject is the
  whole tool -- a scenario, paired with its `validation/src/<scenario>_scenario.rs`
  -- or one of the harness files beside it
- no unnecessary comments in code
- no `&mut` input parameters; prefer return values (`&mut self` is allowed in
  traits and impl blocks)
- only use `pub mod` in `mod.rs` and `lib.rs`
- split test files so there is one test file per source file, named
  `<source file name>_tests.rs`
- in `all_tests.rs`, reference test files one by one without `#[path = ...]`
- apply AAA (`Arrange`, `Act`, `Assert`) structure to tests, with blank-line
  separation between the sections
- use `// Arrange & Act` if there is no separate `Arrange`
- use `// Act & Assert` if there is no separate `Act`
- add the repository copyright and license header to every Rust source file
- **test naming**: `<method>_<description>_<outcome>` where `<method>` is the
  function called in the Act section

## Upstream's lint configuration

`Cargo.toml` keeps upstream's `[lints.clippy]` block: `pedantic` and `nursery`
at warn with a documented allow-list. It is stricter than stage 1's `-D
warnings` on the default lints, it already passes, and it has caught three
newly-public functions missing `#[must_use]` during this repository's cleanup.
Keep it.
