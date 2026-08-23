// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::error::Error;
use std::fmt;
use std::io;
use std::path::PathBuf;

use crate::error::Error as AnalysisError;

/// Errors returned by CLI command functions.
#[derive(Debug)]
pub enum CliError {
    /// An I/O error (exit code 2).
    Io(io::Error),
    /// No source files found (exit code 2).
    NoSourceFiles(PathBuf),
    /// No recognized source files for language auto-detection (exit code 2).
    NoRecognizedFiles,
    /// Multiple languages detected — user must specify `--language` (exit code 2).
    AmbiguousLanguage(Vec<String>),
    /// Analysis pipeline failed (exit code 2).
    Analysis(AnalysisError),
    /// Invalid fingerprint string (exit code 2).
    InvalidFingerprint(String),
    /// Configuration outside the range its field allows (exit code 2).
    InvalidConfig(String),
    /// Check thresholds exceeded (exit code 1).
    CheckFailed,
}

impl CliError {
    /// Map to an appropriate process exit code.
    #[must_use]
    pub const fn exit_code(&self) -> i32 {
        match self {
            Self::CheckFailed => 1,
            Self::Io(_)
            | Self::NoSourceFiles(_)
            | Self::NoRecognizedFiles
            | Self::AmbiguousLanguage(_)
            | Self::Analysis(_)
            | Self::InvalidFingerprint(_)
            | Self::InvalidConfig(_) => 2,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{e}"),
            Self::NoSourceFiles(path) => {
                write!(f, "No source files found in {}", path.display())
            }
            Self::NoRecognizedFiles => {
                write!(
                    f,
                    "No recognized source files found. Use --language to specify the language."
                )
            }
            Self::AmbiguousLanguage(langs) => {
                write!(
                    f,
                    "Multiple languages detected: {}. Use --language to specify which to analyze.",
                    langs.join(", ")
                )
            }
            Self::Analysis(e) => write!(f, "{e}"),
            Self::InvalidFingerprint(fp) => write!(f, "Invalid fingerprint: {fp}"),
            Self::InvalidConfig(message) => write!(f, "{message}"),
            Self::CheckFailed => write!(f, "Check failed"),
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Analysis(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for CliError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AnalysisError> for CliError {
    fn from(e: AnalysisError) -> Self {
        Self::Analysis(e)
    }
}

/// Result type for CLI operations.
pub type CliResult<T = ()> = Result<T, CliError>;
