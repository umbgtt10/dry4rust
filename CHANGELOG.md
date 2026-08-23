# Changelog

All notable changes to `cargo-dry4rust` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- Forked [`cargo-dupes`](https://github.com/mpecan/cargo-dupes) by Matjaz Domen Pecan
  (MIT) into this repository, history included, replacing the placeholder skeleton with a
  working duplicate-detection engine: AST normalisation, fingerprint hashing and
  Dice-coefficient near-duplicate detection.
- `LICENSE` now carries both copyright notices, upstream's first.

### Changed
- Upstream's changelog is preserved below, under its own heading, rather than replaced.


## [0.1.0] - 2026-07-26

### Added
- Initial placeholder release â€” reserves the `cargo-dry4rust` name on crates.io.
- `cargo dry4rust` subcommand plumbing: argument forwarding in `main.rs`, `run`/`run_from_args`
  library entry points in `lib.rs`.
- No duplication analysis yet â€” the binary prints a placeholder message and exits successfully.

---

# Upstream changelog (cargo-dupes)

Everything below is the changelog of the project this was forked from, kept
verbatim so its release history is not lost.

All notable changes to this project will be documented in this file.
See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

---
## [0.1.5](https://github.com/mpecan/cargo-dupes/compare/cargo-dupes-v0.1.4...cargo-dupes-v0.1.5) (2026-02-13)


### Features

* composite fingerprints for near-duplicate groups + cleanup command ([#11](https://github.com/mpecan/cargo-dupes/issues/11)) ([2e49add](https://github.com/mpecan/cargo-dupes/commit/2e49add086a412c6debfb34f77bc1dbe5087c519))

## [0.1.4](https://github.com/mpecan/cargo-dupes/compare/cargo-dupes-v0.1.3...cargo-dupes-v0.1.4) (2026-02-11)


### Features

* add percentage-based duplication thresholds to check ([#8](https://github.com/mpecan/cargo-dupes/issues/8)) ([f528953](https://github.com/mpecan/cargo-dupes/commit/f528953f0ebb1aeb1d67773094ca1012ebf45657))


### Bug Fixes

* make all check thresholds default to disabled when not set ([#10](https://github.com/mpecan/cargo-dupes/issues/10)) ([de3743f](https://github.com/mpecan/cargo-dupes/commit/de3743f2615b9d4bc3a1f3bd077a3bbaa49efeae))

## [0.1.3](https://github.com/mpecan/cargo-dupes/compare/cargo-dupes-v0.1.2...cargo-dupes-v0.1.3) (2026-02-11)


### Features

* add --exclude-tests flag to filter out test code ([#2](https://github.com/mpecan/cargo-dupes/issues/2)) ([c116c95](https://github.com/mpecan/cargo-dupes/commit/c116c959197373664c470ea6edb00dba407c1f04))

## [0.1.2](https://github.com/mpecan/cargo-dupes/compare/cargo-dupes-v0.1.1...cargo-dupes-v0.1.2) (2026-02-11)


### Bug Fixes

* **ci:** trigger release on GitHub release instead of tag push ([#5](https://github.com/mpecan/cargo-dupes/issues/5)) ([2bc5059](https://github.com/mpecan/cargo-dupes/commit/2bc505956df8ce4b8a78e461aa038e7c1c07fc06))

## [0.1.1](https://github.com/mpecan/cargo-dupes/compare/cargo-dupes-v0.1.0...cargo-dupes-v0.1.1) (2026-02-11)


### Features

* add --min-lines filter and duplicated line statistics ([7de772b](https://github.com/mpecan/cargo-dupes/commit/7de772bda588c92d71341be0b50edfeddd26ccb5))
* add --min-lines filter and duplicated line statistics ([c47dd33](https://github.com/mpecan/cargo-dupes/commit/c47dd3348799a9f735122b29de2af92c49c7b02c))
* initial implementation of cargo-dupes ([5c8b4c1](https://github.com/mpecan/cargo-dupes/commit/5c8b4c1c47eb1a1c89a380e44a7adb53d24cb905))


### Bug Fixes

* **ci:** upgrade cocogitto-action from v3 to v4 ([b5621a6](https://github.com/mpecan/cargo-dupes/commit/b5621a66042da23cc8936d4070b4b15a975c491f))


### Documentation

* update CLAUDE.md for --min-lines and line stats ([faac27b](https://github.com/mpecan/cargo-dupes/commit/faac27b1fb913511d33a3249ff912478799d6c3b))
* update README with --min-lines flag and line stats output ([c21ec7c](https://github.com/mpecan/cargo-dupes/commit/c21ec7c9daeccfef6bc8734f6dc7b06b157ddd68))

## [0.1.0] - 2026-02-10

### Initial Release
- AST-based duplicate and near-duplicate code detection
- Text and JSON output formats
- CLI with report, stats, check, ignore, and ignored subcommands
- Configuration via dupes.toml or Cargo.toml metadata

