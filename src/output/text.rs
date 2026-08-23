// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io;

use crate::grouper::{DuplicateGroup, DuplicationStats};
use crate::output::reporter::Reporter;
use crate::output::reporter::display_path;

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
    pub base_path: Option<std::path::PathBuf>,
}

impl TextReporter {
    #[must_use]
    pub const fn new(base_path: Option<std::path::PathBuf>) -> Self {
        Self { base_path }
    }

    fn write_groups(
        &self,
        groups: &[DuplicateGroup],
        writer: &mut dyn io::Write,
        title: &str,
        empty_message: Option<&str>,
        show_similarity: bool,
        show_parent: bool,
    ) -> io::Result<()> {
        if groups.is_empty() {
            if let Some(msg) = empty_message {
                writeln!(writer, "{msg}")?;
            }
            return Ok(());
        }

        writeln!(writer, "{title}")?;
        writeln!(writer, "{}", "=".repeat(title.len()))?;
        writeln!(writer)?;

        for (i, group) in groups.iter().enumerate() {
            let fp = group.fingerprint.to_hex();
            if show_similarity {
                writeln!(
                    writer,
                    "Group {} (fingerprint: {}, similarity: {:.0}%, {} members):",
                    i + 1,
                    fp,
                    group.similarity * 100.0,
                    group.members.len()
                )?;
            } else {
                writeln!(
                    writer,
                    "Group {} (fingerprint: {}, {} members):",
                    i + 1,
                    fp,
                    group.members.len()
                )?;
            }
            for member in &group.members {
                let parent = if show_parent {
                    member
                        .parent_name
                        .as_deref()
                        .map(|p| format!(" in {p}"))
                        .unwrap_or_default()
                } else {
                    String::new()
                };
                writeln!(
                    writer,
                    "  - {} ({}){} at {}:{}-{}",
                    member.name,
                    member.kind,
                    parent,
                    display_path(self.base_path.as_deref(), &member.file),
                    member.line_start,
                    member.line_end,
                )?;
            }
            writeln!(writer)?;
        }
        Ok(())
    }
}

impl Reporter for TextReporter {
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
        self.write_groups(
            groups,
            writer,
            "Exact Duplicates",
            Some("No exact duplicates found."),
            false,
            false,
        )
    }

    fn report_near(&self, groups: &[DuplicateGroup], writer: &mut dyn io::Write) -> io::Result<()> {
        self.write_groups(
            groups,
            writer,
            "Near Duplicates",
            Some("No near duplicates found."),
            true,
            false,
        )
    }

    fn report_sub_exact(
        &self,
        groups: &[DuplicateGroup],
        writer: &mut dyn io::Write,
    ) -> io::Result<()> {
        self.write_groups(
            groups,
            writer,
            "Sub-function Exact Duplicates",
            None,
            false,
            true,
        )
    }

    fn report_sub_near(
        &self,
        groups: &[DuplicateGroup],
        writer: &mut dyn io::Write,
    ) -> io::Result<()> {
        self.write_groups(
            groups,
            writer,
            "Sub-function Near Duplicates",
            None,
            true,
            true,
        )
    }
}
