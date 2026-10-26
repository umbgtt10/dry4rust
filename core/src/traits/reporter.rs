// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

use crate::cli::output::check_breach::CheckBreach;
use crate::cli::output::render_failure::RenderFailure;
use crate::cli::output::report::Report;
use crate::duplicate_group::DuplicateGroup;
use crate::duplication_stats::DuplicationStats;

/// Trait for reporting analysis results.
///
/// The section methods write one part each, which suits a format whose parts
/// stand alone. `report` and `report_check` write a whole document, which is
/// what a format that has to be one value needs -- the implementation decides
/// how its parts fit together, because only it knows whether they can be
/// concatenated.
pub trait Reporter {
    /// Build a full report as one document.
    fn report(&self, report: &Report<'_>) -> Result<String, RenderFailure>;

    /// Build a check verdict as one document. No breaches means it passed.
    fn report_check(
        &self,
        stats: &DuplicationStats,
        breaches: &[CheckBreach<'_>],
    ) -> Result<String, RenderFailure>;

    fn report_stats(&self, stats: &DuplicationStats) -> Result<String, RenderFailure>;
    fn report_exact(&self, groups: &[DuplicateGroup]) -> Result<String, RenderFailure>;
    fn report_near(&self, groups: &[DuplicateGroup]) -> Result<String, RenderFailure>;
    fn report_sub_exact(&self, groups: &[DuplicateGroup]) -> Result<String, RenderFailure>;
    fn report_sub_near(&self, groups: &[DuplicateGroup]) -> Result<String, RenderFailure>;
}
