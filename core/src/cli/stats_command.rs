// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::analysis::Analysis;
use crate::cli::cli_error::CliResult;
use crate::cli::command_outcome::CommandOutcome;
use crate::output::reporter::Reporter;

/// `stats`: the summary and nothing else.
pub struct StatsCommand<'a> {
    result: &'a Analysis,
    reporter: &'a dyn Reporter,
}

impl<'a> StatsCommand<'a> {
    #[must_use]
    pub const fn new(result: &'a Analysis, reporter: &'a dyn Reporter) -> Self {
        Self { result, reporter }
    }

    /// Build the duplication summary.
    ///
    /// # Errors
    ///
    /// Returns [`crate::cli::cli_error::CliError::Io`] if the reporter fails.
    pub fn run(&self) -> CliResult<CommandOutcome> {
        let text = self.reporter.report_stats(&self.result.stats)?;
        Ok(CommandOutcome::succeeded(text))
    }
}
