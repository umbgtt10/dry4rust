// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io;

use crate::grouper::{DuplicateGroup, DuplicationStats};
use crate::output::check_breach::CheckBreach;
use crate::output::group_section::GroupSection;
use crate::output::report::Report;
use crate::output::reporter::Reporter;
use std::path::PathBuf;

fn format_with_commas(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(c);
    }
    result
}

pub struct TextReporter {
    /// Base path for displaying relative paths.
    pub base_path: Option<PathBuf>,
}

impl TextReporter {
    #[must_use]
    pub const fn new(base_path: Option<PathBuf>) -> Self {
        Self { base_path }
    }
}

impl Reporter for TextReporter {
    fn report(&self, report: &Report<'_>, writer: &mut dyn io::Write) -> io::Result<()> {
        self.report_stats(report.stats, writer)?;
        writeln!(writer)?;
        self.report_exact(report.exact, writer)?;
        if !report.near.is_empty() {
            self.report_near(report.near, writer)?;
        }
        if !report.sub_exact.is_empty() {
            self.report_sub_exact(report.sub_exact, writer)?;
        }
        if !report.sub_near.is_empty() {
            self.report_sub_near(report.sub_near, writer)?;
        }
        Ok(())
    }

    fn report_check(
        &self,
        stats: &DuplicationStats,
        breaches: &[CheckBreach<'_>],
        writer: &mut dyn io::Write,
    ) -> io::Result<()> {
        self.report_stats(stats, writer)?;
        for breach in breaches {
            writeln!(writer, "\nCheck FAILED: {}", breach.message())?;
            if breach.is_of_exact() {
                self.report_exact(breach.groups(), writer)?;
            } else {
                self.report_near(breach.groups(), writer)?;
            }
        }
        if breaches.is_empty() {
            writeln!(writer, "\nCheck passed.")?;
        }
        Ok(())
    }

    fn report_stats(&self, stats: &DuplicationStats, writer: &mut dyn io::Write) -> io::Result<()> {
        writeln!(writer, "Duplication Statistics")?;
        writeln!(writer, "=====================")?;
        writeln!(
            writer,
            "Total code units analyzed: {}",
            stats.total_code_units
        )?;
        writeln!(writer)?;
        writeln!(
            writer,
            "Exact duplicates: {} groups ({} code units)",
            stats.exact_duplicate_groups, stats.exact_duplicate_units
        )?;
        writeln!(
            writer,
            "Near duplicates:  {} groups ({} code units)",
            stats.near_duplicate_groups, stats.near_duplicate_units
        )?;
        writeln!(writer)?;
        writeln!(
            writer,
            "Duplicated lines (exact): {}",
            stats.exact_duplicate_lines
        )?;
        writeln!(
            writer,
            "Duplicated lines (near):  {}",
            stats.near_duplicate_lines
        )?;
        writeln!(
            writer,
            "Duplication: {:.1}% exact, {:.1}% near (of {} total lines)",
            stats.exact_duplicate_percent(),
            stats.near_duplicate_percent(),
            format_with_commas(stats.total_lines),
        )?;
        if let Some(suppressed) = stats.baseline_suppressed {
            writeln!(writer, "Baseline: {suppressed} groups suppressed")?;
        }
        if stats.sub_exact_groups > 0 || stats.sub_near_groups > 0 {
            writeln!(writer)?;
            writeln!(
                writer,
                "Sub-function exact: {} groups ({} units)",
                stats.sub_exact_groups, stats.sub_exact_units
            )?;
            writeln!(
                writer,
                "Sub-function near:  {} groups ({} units)",
                stats.sub_near_groups, stats.sub_near_units
            )?;
        }
        Ok(())
    }

    fn report_exact(
        &self,
        groups: &[DuplicateGroup],
        writer: &mut dyn io::Write,
    ) -> io::Result<()> {
        GroupSection::exact().write(groups, self.base_path.as_deref(), writer)
    }

    fn report_near(&self, groups: &[DuplicateGroup], writer: &mut dyn io::Write) -> io::Result<()> {
        GroupSection::near().write(groups, self.base_path.as_deref(), writer)
    }

    fn report_sub_exact(
        &self,
        groups: &[DuplicateGroup],
        writer: &mut dyn io::Write,
    ) -> io::Result<()> {
        GroupSection::sub_exact().write(groups, self.base_path.as_deref(), writer)
    }

    fn report_sub_near(
        &self,
        groups: &[DuplicateGroup],
        writer: &mut dyn io::Write,
    ) -> io::Result<()> {
        GroupSection::sub_near().write(groups, self.base_path.as_deref(), writer)
    }
}
