// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("No source files found in {0}")]
    NoSourceFiles(PathBuf),

    #[error("{0}")]
    Other(String),
}

/// The one qualified path this crate keeps. Naming the alias `Result` shadows
/// the prelude's, so the right-hand side has to say which one it means.
pub type Result<T> = std::result::Result<T, Error>;
