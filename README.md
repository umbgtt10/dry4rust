# dry4rust

**A `cargo` subcommand for detecting duplicated code patterns in Rust — DRY (Don't Repeat
Yourself) analysis.**

Part of the same family as [`grip`](https://crates.io/crates/cargo-grip4rust) (testability),
[`braintax`](https://crates.io/crates/cargo-braintax4rust) (cognitive load), and
[`crap4rust`](https://crates.io/crates/cargo-crap4rust) (change-risk complexity × coverage).
Where those three measure how safe, understandable, and risky a codebase is, `dry4rust` will
measure how much of it is needlessly repeated.

## Status

**Early placeholder.** This release exists to reserve the crate name and stand up a working,
publishable skeleton before the real analysis is designed and built. `cargo dry4rust`
currently prints a placeholder message and exits successfully — no duplication detection yet.

## Install

```powershell
cargo install cargo-dry4rust
```

## License

Licensed under [MIT](LICENSE).
