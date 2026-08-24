// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io;
use std::path::Path;

use crate::code_unit::CodeUnit;
use crate::grouper::DuplicateGroup;
use crate::output::reporter::display_path;

/// One headed section of a text report, and how the groups under it read.
///
/// The four sections differ in four ways and in nothing else: what they are
/// called, whether they say anything when empty, whether a group states its
/// similarity, and whether a member names the function it came from. Those
/// four differences were four parameters on one private function, which is
/// how a formatter acquires branches it does not need.
pub struct GroupSection {
    title: &'static str,
    empty_message: Option<&'static str>,
    show_similarity: bool,
    show_parent: bool,
}

impl GroupSection {
    /// Exact duplicates: says so in words when there are none.
    #[must_use]
    pub const fn exact() -> Self {
        Self {
            title: "Exact Duplicates",
            empty_message: Some("No exact duplicates found."),
            show_similarity: false,
            show_parent: false,
        }
    }

    /// Near duplicates: scored, and says so in words when there are none.
    #[must_use]
    pub const fn near() -> Self {
        Self {
            title: "Near Duplicates",
            empty_message: Some("No near duplicates found."),
            show_similarity: true,
            show_parent: false,
        }
    }

    /// Sub-function exact duplicates: silent when empty, because the analysis
    /// they come from is opt-in and saying "none found" would suggest it ran.
    #[must_use]
    pub const fn sub_exact() -> Self {
        Self {
            title: "Sub-function Exact Duplicates",
            empty_message: None,
            show_similarity: false,
            show_parent: true,
        }
    }

    /// Sub-function near duplicates: scored, and silent when empty.
    #[must_use]
    pub const fn sub_near() -> Self {
        Self {
            title: "Sub-function Near Duplicates",
            empty_message: None,
            show_similarity: true,
            show_parent: true,
        }
    }

    /// Write the section under its heading, or its empty message, or nothing.
    ///
    /// # Errors
    ///
    /// Returns whatever the writer returns.
    pub fn write(
        &self,
        groups: &[DuplicateGroup],
        base: Option<&Path>,
        writer: &mut dyn io::Write,
    ) -> io::Result<()> {
        if groups.is_empty() {
            return self.write_empty(writer);
        }
        writeln!(writer, "{}", self.title)?;
        writeln!(writer, "{}", "=".repeat(self.title.len()))?;
        writeln!(writer)?;
        for (index, group) in groups.iter().enumerate() {
            self.write_group(index + 1, group, base, writer)?;
        }
        Ok(())
    }

    fn write_empty(&self, writer: &mut dyn io::Write) -> io::Result<()> {
        match self.empty_message {
            Some(message) => writeln!(writer, "{message}"),
            None => Ok(()),
        }
    }

    fn write_group(
        &self,
        number: usize,
        group: &DuplicateGroup,
        base: Option<&Path>,
        writer: &mut dyn io::Write,
    ) -> io::Result<()> {
        writeln!(writer, "{}", self.heading(number, group))?;
        for member in &group.members {
            writeln!(writer, "{}", self.member_line(member, base))?;
        }
        writeln!(writer)
    }

    fn heading(&self, number: usize, group: &DuplicateGroup) -> String {
        let fingerprint = group.fingerprint.to_hex();
        let members = group.members.len();
        if self.show_similarity {
            format!(
                "Group {number} (fingerprint: {fingerprint}, similarity: {:.0}%, {members} members):",
                group.similarity * 100.0
            )
        } else {
            format!("Group {number} (fingerprint: {fingerprint}, {members} members):")
        }
    }

    fn member_line(&self, member: &CodeUnit, base: Option<&Path>) -> String {
        format!(
            "  - {} ({}){} at {}:{}-{}",
            member.name,
            member.kind,
            self.parent_of(member),
            display_path(base, &member.file),
            member.line_start,
            member.line_end,
        )
    }

    fn parent_of(&self, member: &CodeUnit) -> String {
        if self.show_parent {
            member
                .parent_name
                .as_deref()
                .map_or_else(String::new, |parent| format!(" in {parent}"))
        } else {
            String::new()
        }
    }
}
