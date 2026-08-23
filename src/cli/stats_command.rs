// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io::Write;

use crate::analysis::AnalysisResult;
use crate::cli::cli_error::CliResult;
use crate::output::reporter::Reporter;

/// `stats`: the summary and nothing else.
pub struct StatsCommand<'a> {
    result: &'a AnalysisResult,
    reporter: &'a dyn Reporter,
}

impl<'a> StatsCommand<'a> {
    #[must_use]
    pub const fn new(result: &'a AnalysisResult, reporter: &'a dyn Reporter) -> Self {
        Self { result, reporter }
    }

    /// Write the duplication summary.
    ///
    /// # Errors
    ///
    /// Returns [`crate::cli::cli_error::CliError::Io`] if the writer fails.
    pub fn run(&self, writer: &mut impl Write) -> CliResult {
        self.reporter.report_stats(&self.result.stats, writer)?;
        Ok(())
    }
}
