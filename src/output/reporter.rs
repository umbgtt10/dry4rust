// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::borrow::Cow;
use std::io;
use std::path::Path;

use crate::grouper::{DuplicateGroup, DuplicationStats};
use crate::output::check_breach::CheckBreach;
use crate::output::report::Report;

/// Compute a display path relative to an optional base, falling back to the absolute path.
#[must_use]
pub fn display_path<'a>(base: Option<&Path>, path: &'a Path) -> Cow<'a, str> {
    if let Some(base) = base
        && let Ok(rel) = path.strip_prefix(base)
    {
        return rel.to_string_lossy();
    }
    path.to_string_lossy()
}

/// Trait for reporting analysis results.
///
/// The section methods write one part each, which suits a format whose parts
/// stand alone. `report` and `report_check` write a whole document, which is
/// what a format that has to be one value needs -- the implementation decides
/// how its parts fit together, because only it knows whether they can be
/// concatenated.
pub trait Reporter {
    /// Write a full report as one document.
    fn report(&self, report: &Report<'_>, writer: &mut dyn io::Write) -> io::Result<()>;

    /// Write a check verdict as one document. No breaches means it passed.
    fn report_check(
        &self,
        stats: &DuplicationStats,
        breaches: &[CheckBreach<'_>],
        writer: &mut dyn io::Write,
    ) -> io::Result<()>;

    fn report_stats(&self, stats: &DuplicationStats, writer: &mut dyn io::Write) -> io::Result<()>;
    fn report_exact(&self, groups: &[DuplicateGroup], writer: &mut dyn io::Write)
    -> io::Result<()>;
    fn report_near(&self, groups: &[DuplicateGroup], writer: &mut dyn io::Write) -> io::Result<()>;
    fn report_sub_exact(
        &self,
        groups: &[DuplicateGroup],
        writer: &mut dyn io::Write,
    ) -> io::Result<()>;
    fn report_sub_near(
        &self,
        groups: &[DuplicateGroup],
        writer: &mut dyn io::Write,
    ) -> io::Result<()>;
}
