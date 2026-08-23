// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io::Write;

use crate::analysis::AnalysisResult;
use crate::cli::cli_error::CliResult;
use crate::output::reporter::Reporter;

/// `report`: the summary followed by every group that survived filtering.
///
/// A section with nothing in it is left out rather than printed empty, except
/// for exact duplicates, whose reporter says so in words.
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
        self.reporter.report_stats(&self.result.stats, writer)?;
        writeln!(writer)?;
        self.reporter
            .report_exact(&self.result.exact_groups, writer)?;
        if !self.result.near_groups.is_empty() {
            self.reporter
                .report_near(&self.result.near_groups, writer)?;
        }
        if !self.result.sub_exact_groups.is_empty() {
            self.reporter
                .report_sub_exact(&self.result.sub_exact_groups, writer)?;
        }
        if !self.result.sub_near_groups.is_empty() {
            self.reporter
                .report_sub_near(&self.result.sub_near_groups, writer)?;
        }
        Ok(())
    }
}
