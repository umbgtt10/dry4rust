// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// What `baseline` writes when no path says otherwise.
///
/// JSON rather than the TOML the ignore file uses, and named for the tool the
/// way every other file in this family is. The two suppression files differ in
/// who writes them: an ignore entry is written by a person and carries a reason
/// in their words, and a baseline is written by the tool and read back by it.
pub const DEFAULT_BASELINE_FILE: &str = "dry4rust-baseline.json";

/// The format this build writes, and the only one it reads.
pub const FORMAT_VERSION: u32 = 1;
