// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec::Vec;
use core::error::Error;
use core::fmt;

use crate::cli::output::render_failure::RenderFailure;
use crate::error::Error as AnalysisError;
use crate::source_path::SourcePath;
use crate::storage_failure::StorageFailure;

/// Errors returned by CLI command functions.
#[derive(Debug)]
pub enum CliError {
    /// A file could not be read, written or removed (exit code 2).
    Io(StorageFailure),
    /// No source files found (exit code 2).
    NoSourceFiles(SourcePath),
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
    /// A reporter could not write its document (exit code 2).
    Render(RenderFailure),
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
            | Self::InvalidConfig(_)
            | Self::Render(_) => 2,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{e}"),
            Self::NoSourceFiles(path) => {
                write!(f, "No source files found in {path}")
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
            Self::Render(e) => write!(f, "{e}"),
            Self::CheckFailed => write!(f, "Check failed"),
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Analysis(e) => Some(e),
            Self::Render(e) => Some(e),
            _ => None,
        }
    }
}

impl From<StorageFailure> for CliError {
    fn from(e: StorageFailure) -> Self {
        Self::Io(e)
    }
}

impl From<AnalysisError> for CliError {
    fn from(e: AnalysisError) -> Self {
        Self::Analysis(e)
    }
}

impl From<RenderFailure> for CliError {
    fn from(e: RenderFailure) -> Self {
        Self::Render(e)
    }
}

/// Result type for CLI operations.
pub type CliResult<T = ()> = Result<T, CliError>;
