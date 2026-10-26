// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec::Vec;

use crate::cli::cli_error::CliResult;

/// What a command produced: the text it means to print, and the verdict that
/// decides the exit code once that text has reached stdout.
///
/// Keeping the two together lets `check` write its report and still fail: the
/// text and the [`crate::cli::cli_error::CliError::CheckFailed`] both travel up
/// to the one place that prints and then exits. The warnings the analysis
/// raised travel with them, for that place to write to stderr first.
pub struct CommandOutcome {
    text: String,
    result: CliResult,
    warnings: Vec<String>,
}

impl CommandOutcome {
    #[must_use]
    pub const fn succeeded(text: String) -> Self {
        Self {
            text,
            result: Ok(()),
            warnings: Vec::new(),
        }
    }

    #[must_use]
    pub const fn new(text: String, result: CliResult) -> Self {
        Self {
            text,
            result,
            warnings: Vec::new(),
        }
    }

    pub fn into_parts(self) -> (CliResult, String) {
        (self.result, self.text)
    }

    #[must_use]
    pub const fn result(&self) -> &CliResult {
        &self.result
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    #[must_use]
    pub fn with_warnings(self, warnings: Vec<String>) -> Self {
        Self { warnings, ..self }
    }
}
