// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io;

use crate::grouper::{DuplicateGroup, DuplicationStats};
use crate::output::reporter::Reporter;
use crate::output::reporter::display_path;
use serde_json::to_string_pretty;

pub struct JsonReporter {
    pub base_path: Option<std::path::PathBuf>,
}

impl JsonReporter {
    #[must_use]
    pub const fn new(base_path: Option<std::path::PathBuf>) -> Self {
        Self { base_path }
    }
}

#[derive(serde::Serialize)]
struct JsonStats {
    total_code_units: usize,
    total_lines: usize,
    exact_duplicate_groups: usize,
    exact_duplicate_units: usize,
    near_duplicate_groups: usize,
    near_duplicate_units: usize,
    exact_duplicate_lines: usize,
    near_duplicate_lines: usize,
    exact_duplicate_percent: f64,
    near_duplicate_percent: f64,
    #[serde(skip_serializing_if = "is_zero")]
    sub_exact_groups: usize,
    #[serde(skip_serializing_if = "is_zero")]
    sub_exact_units: usize,
    #[serde(skip_serializing_if = "is_zero")]
    sub_near_groups: usize,
    #[serde(skip_serializing_if = "is_zero")]
    sub_near_units: usize,
}

#[allow(clippy::trivially_copy_pass_by_ref)] // serde skip_serializing_if requires &T
const fn is_zero(v: &usize) -> bool {
    *v == 0
}

#[derive(serde::Serialize)]
struct JsonGroup {
    fingerprint: String,
    similarity: f64,
    members: Vec<JsonMember>,
}

#[derive(serde::Serialize)]
struct JsonMember {
    name: String,
    kind: String,
    file: String,
    line_start: usize,
    line_end: usize,
}

impl Reporter for JsonReporter {
    fn report_stats(&self, stats: &DuplicationStats, writer: &mut dyn io::Write) -> io::Result<()> {
        let json_stats = JsonStats {
            total_code_units: stats.total_code_units,
            total_lines: stats.total_lines,
            exact_duplicate_groups: stats.exact_duplicate_groups,
            exact_duplicate_units: stats.exact_duplicate_units,
            near_duplicate_groups: stats.near_duplicate_groups,
            near_duplicate_units: stats.near_duplicate_units,
            exact_duplicate_lines: stats.exact_duplicate_lines,
            near_duplicate_lines: stats.near_duplicate_lines,
            exact_duplicate_percent: stats.exact_duplicate_percent(),
            near_duplicate_percent: stats.near_duplicate_percent(),
            sub_exact_groups: stats.sub_exact_groups,
            sub_exact_units: stats.sub_exact_units,
            sub_near_groups: stats.sub_near_groups,
            sub_near_units: stats.sub_near_units,
        };
        let json = to_string_pretty(&json_stats).map_err(io::Error::other)?;
        writeln!(writer, "{json}")
    }

    fn report_exact(
        &self,
        groups: &[DuplicateGroup],
        writer: &mut dyn io::Write,
    ) -> io::Result<()> {
        self.write_groups(groups, writer)
    }

    fn report_near(&self, groups: &[DuplicateGroup], writer: &mut dyn io::Write) -> io::Result<()> {
        self.write_groups(groups, writer)
    }

    fn report_sub_exact(
        &self,
        groups: &[DuplicateGroup],
        writer: &mut dyn io::Write,
    ) -> io::Result<()> {
        self.write_groups(groups, writer)
    }

    fn report_sub_near(
        &self,
        groups: &[DuplicateGroup],
        writer: &mut dyn io::Write,
    ) -> io::Result<()> {
        self.write_groups(groups, writer)
    }
}

impl JsonReporter {
    fn write_groups(
        &self,
        groups: &[DuplicateGroup],
        writer: &mut dyn io::Write,
    ) -> io::Result<()> {
        let json_groups: Vec<JsonGroup> = groups.iter().map(|g| self.to_json_group(g)).collect();
        let json = to_string_pretty(&json_groups).map_err(io::Error::other)?;
        writeln!(writer, "{json}")
    }

    fn to_json_group(&self, group: &DuplicateGroup) -> JsonGroup {
        JsonGroup {
            fingerprint: group.fingerprint.to_hex(),
            similarity: group.similarity,
            members: group
                .members
                .iter()
                .map(|m| JsonMember {
                    name: m.name.clone(),
                    kind: m.kind.to_string(),
                    file: display_path(self.base_path.as_deref(), &m.file).into_owned(),
                    line_start: m.line_start,
                    line_end: m.line_end,
                })
                .collect(),
        }
    }
}
