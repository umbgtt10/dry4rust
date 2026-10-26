// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::output::check_breach::CheckBreach;
use dry4rust::cli::output::render_failure::RenderFailure;
use dry4rust::cli::output::report::Report;
use dry4rust::duplicate_group::DuplicateGroup;
use dry4rust::duplication_stats::DuplicationStats;
use dry4rust::traits::reporter::Reporter;

// A reporter that writes one line naming what it was handed, so a test can see
// which document a command asked for and with how much in it.
#[derive(Debug, Default)]
pub struct SummaryReporter;

impl Reporter for SummaryReporter {
    fn report(&self, report: &Report<'_>) -> Result<String, RenderFailure> {
        Ok(format!(
            "report units={} exact={}\n",
            report.stats.total_code_units(),
            report.exact.len()
        ))
    }

    fn report_check(
        &self,
        stats: &DuplicationStats,
        breaches: &[CheckBreach<'_>],
    ) -> Result<String, RenderFailure> {
        Ok(format!(
            "check units={} breaches={}\n",
            stats.total_code_units(),
            breaches.len()
        ))
    }

    fn report_stats(&self, stats: &DuplicationStats) -> Result<String, RenderFailure> {
        Ok(format!("stats units={}\n", stats.total_code_units()))
    }

    fn report_exact(&self, groups: &[DuplicateGroup]) -> Result<String, RenderFailure> {
        Ok(format!("exact={}\n", groups.len()))
    }

    fn report_near(&self, groups: &[DuplicateGroup]) -> Result<String, RenderFailure> {
        Ok(format!("near={}\n", groups.len()))
    }

    fn report_sub_exact(&self, groups: &[DuplicateGroup]) -> Result<String, RenderFailure> {
        Ok(format!("sub_exact={}\n", groups.len()))
    }

    fn report_sub_near(&self, groups: &[DuplicateGroup]) -> Result<String, RenderFailure> {
        Ok(format!("sub_near={}\n", groups.len()))
    }
}
