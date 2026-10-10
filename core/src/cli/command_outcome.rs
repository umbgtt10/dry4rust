// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::cli::cli_error::CliResult;

/// What a command produced: the text it means to print, and the verdict that
/// decides the exit code once that text has reached stdout.
///
/// Keeping the two together lets `check` write its report and still fail: the
/// text and the [`crate::cli::cli_error::CliError::CheckFailed`] both travel up
/// to the one place that prints and then exits.
pub struct CommandOutcome {
    pub text: String,
    pub result: CliResult,
}

impl CommandOutcome {
    #[must_use]
    pub const fn new(text: String, result: CliResult) -> Self {
        Self { text, result }
    }

    #[must_use]
    pub const fn succeeded(text: String) -> Self {
        Self {
            text,
            result: Ok(()),
        }
    }
}
