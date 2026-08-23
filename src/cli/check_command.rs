// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io;
use std::io::Write;

use crate::analysis::AnalysisResult;
use crate::cli::checking::ceiling::Ceiling;
use crate::cli::checking::check_thresholds::CheckThresholds;
use crate::cli::cli_error::CliError;
use crate::cli::cli_error::CliResult;
use crate::config::Config;
use crate::output::reporter::Reporter;

/// `check`: the summary, then a non-zero exit if any ceiling is breached.
///
/// A ceiling given on the command line wins over the same ceiling in the
/// config file, and one set in neither place cannot be breached.
pub struct CheckCommand<'a> {
    config: &'a Config,
    result: &'a AnalysisResult,
    reporter: &'a dyn Reporter,
    thresholds: &'a CheckThresholds,
}

impl<'a> CheckCommand<'a> {
    #[must_use]
    pub const fn new(
        config: &'a Config,
        result: &'a AnalysisResult,
        reporter: &'a dyn Reporter,
        thresholds: &'a CheckThresholds,
    ) -> Self {
        Self {
            config,
            result,
            reporter,
            thresholds,
        }
    }

    /// Report, then measure against every ceiling.
    ///
    /// Every breach is reported, not just the first, so one run tells the
    /// whole story.
    ///
    /// # Errors
    ///
    /// Returns [`CliError::CheckFailed`] if any ceiling was breached, and
    /// [`CliError::Io`] if the writer fails.
    pub fn run(&self, writer: &mut impl Write) -> CliResult {
        self.reporter.report_stats(&self.result.stats, writer)?;

        let mut failed = false;
        for (ceiling, breached_by_exact) in &self.ceilings() {
            let Some(message) = ceiling.breach() else {
                continue;
            };
            writeln!(writer, "\nCheck FAILED: {message}")?;
            self.report_offenders(*breached_by_exact, writer)?;
            failed = true;
        }

        if failed {
            Err(CliError::CheckFailed)
        } else {
            writeln!(writer, "\nCheck passed.")?;
            Ok(())
        }
    }

    fn ceilings(&self) -> [(Ceiling, bool); 4] {
        let stats = &self.result.stats;
        [
            (
                Ceiling::count(
                    self.thresholds
                        .max_exact
                        .or(self.config.max_exact_duplicates),
                    stats.exact_duplicate_groups,
                    "exact duplicate groups",
                ),
                true,
            ),
            (
                Ceiling::count(
                    self.thresholds.max_near.or(self.config.max_near_duplicates),
                    stats.near_duplicate_groups,
                    "near duplicate groups",
                ),
                false,
            ),
            (
                Ceiling::percent(
                    self.thresholds
                        .max_exact_percent
                        .or(self.config.max_exact_percent),
                    stats.exact_duplicate_percent(),
                    "exact duplicate lines",
                ),
                true,
            ),
            (
                Ceiling::percent(
                    self.thresholds
                        .max_near_percent
                        .or(self.config.max_near_percent),
                    stats.near_duplicate_percent(),
                    "near duplicate lines",
                ),
                false,
            ),
        ]
    }

    fn report_offenders(&self, breached_by_exact: bool, writer: &mut impl Write) -> io::Result<()> {
        if breached_by_exact {
            self.reporter
                .report_exact(&self.result.exact_groups, writer)
        } else {
            self.reporter.report_near(&self.result.near_groups, writer)
        }
    }
}
