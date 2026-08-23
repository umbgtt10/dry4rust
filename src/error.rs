// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io;
use std::path::PathBuf;
use std::result::Result as StdResult;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("No source files found in {0}")]
    NoSourceFiles(PathBuf),

    #[error("{0}")]
    Other(String),
}

/// Imported under another name because this alias is itself called `Result`:
/// the right-hand side has to name something other than what it defines.
pub type Result<T> = StdResult<T, Error>;
