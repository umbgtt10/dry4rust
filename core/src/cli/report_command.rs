// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::analysis::Analysis;
use crate::cli::cli_error::CliResult;
use crate::cli::command_outcome::CommandOutcome;
use crate::output::report::Report;
use crate::output::reporter::Reporter;

/// `report`: the summary followed by every group that survived filtering.
///
/// What the document looks like is the reporter's decision, not this one's --
/// text writes section after section, JSON writes one object with the sections
/// inside it. A command that made that choice itself could only ever make the
/// text one.
pub struct ReportCommand<'a> {
    result: &'a Analysis,
    reporter: &'a dyn Reporter,
}

impl<'a> ReportCommand<'a> {
    #[must_use]
    pub const fn new(result: &'a Analysis, reporter: &'a dyn Reporter) -> Self {
        Self { result, reporter }
    }

    /// Build the summary and the groups.
    ///
    /// # Errors
    ///
    /// Returns [`crate::cli::cli_error::CliError::Io`] if the reporter fails.
    pub fn run(&self) -> CliResult<CommandOutcome> {
        let text = self.reporter.report(&Report::new(
            self.result.stats(),
            self.result.exact_groups(),
            self.result.near_groups(),
            self.result.sub_exact_groups(),
            self.result.sub_near_groups(),
        ))?;
        Ok(CommandOutcome::succeeded(text))
    }
}
