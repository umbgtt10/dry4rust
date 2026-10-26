// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use core::result::Result as StdResult;

use crate::source_path::SourcePath;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No source files found in {0}")]
    NoSourceFiles(SourcePath),

    #[error("{field} must be {expected}, got {value}")]
    InvalidConfig {
        field: &'static str,
        value: String,
        expected: &'static str,
    },

    #[error("baseline {path}: {problem}")]
    Baseline { path: SourcePath, problem: String },

    #[error("{0}")]
    Other(String),
    #[error("Failed to parse {path}: {reason}")]
    Parse { path: SourcePath, reason: String },
    #[error("Failed to read {path}: {reason}")]
    Read { path: SourcePath, reason: String },
}

/// Imported under another name because this alias is itself called `Result`:
/// the right-hand side has to name something other than what it defines.
pub type Result<T> = StdResult<T, Error>;
