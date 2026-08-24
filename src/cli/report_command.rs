// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io::Write;

use crate::analysis::AnalysisResult;
use crate::cli::cli_error::CliResult;
use crate::output::report::Report;
use crate::output::reporter::Reporter;

/// `report`: the summary followed by every group that survived filtering.
///
/// What the document looks like is the reporter's decision, not this one's --
/// text writes section after section, JSON writes one object with the sections
/// inside it. A command that made that choice itself could only ever make the
/// text one.
pub struct ReportCommand<'a> {
    result: &'a AnalysisResult,
    reporter: &'a dyn Reporter,
}

impl<'a> ReportCommand<'a> {
    #[must_use]
    pub const fn new(result: &'a AnalysisResult, reporter: &'a dyn Reporter) -> Self {
        Self { result, reporter }
    }

    /// Write the summary and the groups.
    ///
    /// # Errors
    ///
    /// Returns [`crate::cli::cli_error::CliError::Io`] if the writer fails.
    pub fn run(&self, writer: &mut impl Write) -> CliResult {
        self.reporter.report(
            &Report {
                stats: &self.result.stats,
                exact: &self.result.exact_groups,
                near: &self.result.near_groups,
                sub_exact: &self.result.sub_exact_groups,
                sub_near: &self.result.sub_near_groups,
            },
            writer,
        )?;
        Ok(())
    }
}
