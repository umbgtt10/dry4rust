// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io::Write;

use crate::analysis::AnalysisResult;
use crate::cli::checking::ceiling::Ceiling;
use crate::cli::checking::check_thresholds::CheckThresholds;
use crate::cli::cli_error::CliError;
use crate::cli::cli_error::CliResult;
use crate::config::Config;
use crate::grouper::DuplicateGroup;
use crate::output::check_breach::CheckBreach;
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
        let breaches = self.breaches();
        self.reporter
            .report_check(&self.result.stats, &breaches, writer)?;
        if breaches.is_empty() {
            Ok(())
        } else {
            Err(CliError::CheckFailed)
        }
    }

    fn breaches(&self) -> Vec<CheckBreach<'a>> {
        self.ceilings()
            .into_iter()
            .filter_map(|(ceiling, of_exact)| {
                ceiling
                    .breach()
                    .map(|message| CheckBreach::new(message, self.offenders(of_exact), of_exact))
            })
            .collect()
    }

    const fn offenders(&self, of_exact: bool) -> &'a [DuplicateGroup] {
        if of_exact {
            self.result.exact_groups.as_slice()
        } else {
            self.result.near_groups.as_slice()
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
}
