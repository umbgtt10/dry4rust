// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fmt::Write as _;
use std::path::Path;

use crate::code_unit::CodeUnit;
use crate::duplicate_group::DuplicateGroup;
use crate::output::display_path::DisplayPath;

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

    /// Build the section under its heading, or its empty message, or nothing.
    #[must_use]
    pub fn write(&self, groups: &[DuplicateGroup], base: Option<&Path>) -> String {
        if groups.is_empty() {
            return self.write_empty();
        }
        let mut out = String::new();
        let _ = writeln!(out, "{}", self.title);
        let _ = writeln!(out, "{}", "=".repeat(self.title.len()));
        let _ = writeln!(out);
        for (index, group) in groups.iter().enumerate() {
            out.push_str(&self.write_group(index + 1, group, base));
        }
        out
    }

    fn write_empty(&self) -> String {
        match self.empty_message {
            Some(message) => format!("{message}\n"),
            None => String::new(),
        }
    }

    fn write_group(&self, number: usize, group: &DuplicateGroup, base: Option<&Path>) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "{}", self.heading(number, group));
        for member in &group.members {
            let _ = writeln!(out, "{}", self.member_line(member, base));
        }
        let _ = writeln!(out);
        out
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
            DisplayPath::of(base, &member.file),
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
