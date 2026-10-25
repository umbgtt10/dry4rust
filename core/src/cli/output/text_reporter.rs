// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fmt::Write as _;
use std::io;

use crate::cli::output::check_breach::CheckBreach;
use crate::cli::output::group_section::GroupSection;
use crate::cli::output::report::Report;
use crate::cli::output::reporter::Reporter;
use crate::duplicate_group::DuplicateGroup;
use crate::duplication_stats::DuplicationStats;
use std::path::PathBuf;

pub struct TextReporter {
    /// Base path for displaying relative paths.
    base_path: Option<PathBuf>,
}

impl TextReporter {
    #[must_use]
    pub const fn new(base_path: Option<PathBuf>) -> Self {
        Self { base_path }
    }

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

    fn section(&self, section: &GroupSection, groups: &[DuplicateGroup]) -> String {
        section.write(groups, self.base_path.as_deref())
    }
}

impl Reporter for TextReporter {
    fn report(&self, report: &Report<'_>) -> io::Result<String> {
        let mut out = self.report_stats(report.stats)?;
        let _ = writeln!(out);
        out.push_str(&self.report_exact(report.exact)?);
        if !report.near.is_empty() {
            out.push_str(&self.report_near(report.near)?);
        }
        if !report.sub_exact.is_empty() {
            out.push_str(&self.report_sub_exact(report.sub_exact)?);
        }
        if !report.sub_near.is_empty() {
            out.push_str(&self.report_sub_near(report.sub_near)?);
        }
        Ok(out)
    }

    fn report_check(
        &self,
        stats: &DuplicationStats,
        breaches: &[CheckBreach<'_>],
    ) -> io::Result<String> {
        let mut out = self.report_stats(stats)?;
        for breach in breaches {
            let _ = writeln!(out, "\nCheck FAILED: {}", breach.message);
            if breach.of_exact {
                out.push_str(&self.report_exact(breach.groups)?);
            } else {
                out.push_str(&self.report_near(breach.groups)?);
            }
        }
        if breaches.is_empty() {
            let _ = writeln!(out, "\nCheck passed.");
        }
        Ok(out)
    }

    fn report_stats(&self, stats: &DuplicationStats) -> io::Result<String> {
        let mut out = String::new();
        let _ = writeln!(out, "Duplication Statistics");
        let _ = writeln!(out, "=====================");
        let _ = writeln!(
            out,
            "Total code units analyzed: {}",
            stats.total_code_units()
        );
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "Exact duplicates: {} groups ({} code units)",
            stats.exact_duplicate_groups(),
            stats.exact_duplicate_units()
        );
        let _ = writeln!(
            out,
            "Near duplicates:  {} groups ({} code units)",
            stats.near_duplicate_groups(),
            stats.near_duplicate_units()
        );
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "Duplicated lines (exact): {}",
            stats.exact_duplicate_lines()
        );
        let _ = writeln!(
            out,
            "Duplicated lines (near):  {}",
            stats.near_duplicate_lines()
        );
        let _ = writeln!(
            out,
            "Duplication: {:.1}% exact, {:.1}% near (of {} total lines)",
            stats.exact_duplicate_percent(),
            stats.near_duplicate_percent(),
            Self::format_with_commas(stats.total_lines()),
        );
        if let Some(suppressed) = stats.baseline_suppressed() {
            let _ = writeln!(out, "Baseline: {suppressed} groups suppressed");
        }
        if stats.sub_exact_groups() > 0 || stats.sub_near_groups() > 0 {
            let _ = writeln!(out);
            let _ = writeln!(
                out,
                "Sub-function exact: {} groups ({} units)",
                stats.sub_exact_groups(),
                stats.sub_exact_units()
            );
            let _ = writeln!(
                out,
                "Sub-function near:  {} groups ({} units)",
                stats.sub_near_groups(),
                stats.sub_near_units()
            );
        }
        Ok(out)
    }

    fn report_exact(&self, groups: &[DuplicateGroup]) -> io::Result<String> {
        Ok(self.section(&GroupSection::exact(), groups))
    }

    fn report_near(&self, groups: &[DuplicateGroup]) -> io::Result<String> {
        Ok(self.section(&GroupSection::near(), groups))
    }

    fn report_sub_exact(&self, groups: &[DuplicateGroup]) -> io::Result<String> {
        Ok(self.section(&GroupSection::sub_exact(), groups))
    }

    fn report_sub_near(&self, groups: &[DuplicateGroup]) -> io::Result<String> {
        Ok(self.section(&GroupSection::sub_near(), groups))
    }
}
