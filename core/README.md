# dry4rust-core

The analysis behind [`cargo-dry4rust`](https://crates.io/crates/cargo-dry4rust):
duplicate and near-duplicate code in Rust, found by normalising the AST,
fingerprinting it and comparing trees with the Dice coefficient. This crate is
the normalisation, the grouping, the suppression rules and the reports; the
command, `cargo dry4rust`, is the `cargo-dry4rust` crate.

What it finds, how it scores and how to suppress a finding are in the
[main README](https://github.com/umbgtt10/dry4rust#readme) and
[`docs/FORMULA.md`](https://github.com/umbgtt10/dry4rust/blob/main/docs/FORMULA.md).

MIT -- see [`LICENSE`](https://github.com/umbgtt10/dry4rust/blob/main/LICENSE).
