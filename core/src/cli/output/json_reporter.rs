// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io;

use crate::cli::output::check_breach::CheckBreach;
use crate::cli::output::display_path::DisplayPath;
use crate::cli::output::report::Report;
use crate::cli::output::reporter::Reporter;
use crate::duplicate_group::DuplicateGroup;
use crate::duplication_stats::DuplicationStats;
use serde_json::to_string_pretty;
use std::path::PathBuf;

pub struct JsonReporter {
    base_path: Option<PathBuf>,
}

impl JsonReporter {
    #[must_use]
    pub const fn new(base_path: Option<PathBuf>) -> Self {
        Self { base_path }
    }

    #[allow(clippy::trivially_copy_pass_by_ref)] // serde skip_serializing_if requires &T
    const fn is_zero(v: &usize) -> bool {
        *v == 0
    }
}

#[derive(serde::Serialize)]
struct JsonStats {
    pub total_code_units: usize,
    pub total_lines: usize,
    pub exact_duplicate_groups: usize,
    pub exact_duplicate_units: usize,
    pub near_duplicate_groups: usize,
    pub near_duplicate_units: usize,
    pub exact_duplicate_lines: usize,
    pub near_duplicate_lines: usize,
    pub exact_duplicate_percent: f64,
    pub near_duplicate_percent: f64,
    #[serde(skip_serializing_if = "JsonReporter::is_zero")]
    pub sub_exact_groups: usize,
    #[serde(skip_serializing_if = "JsonReporter::is_zero")]
    pub sub_exact_units: usize,
    #[serde(skip_serializing_if = "JsonReporter::is_zero")]
    pub sub_near_groups: usize,
    #[serde(skip_serializing_if = "JsonReporter::is_zero")]
    pub sub_near_units: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_suppressed: Option<usize>,
}

/// A full report as one document, so a reader can parse it in one call and
/// reach a section by name.
#[derive(serde::Serialize)]
struct JsonReport {
    pub stats: JsonStats,
    pub exact: Vec<JsonGroup>,
    pub near: Vec<JsonGroup>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sub_exact: Vec<JsonGroup>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sub_near: Vec<JsonGroup>,
}

/// A check verdict as one document. The breach sentences are inside it rather
/// than printed between the sections, which is what stopped the old output
/// being JSON at all.
#[derive(serde::Serialize)]
struct JsonCheck {
    pub stats: JsonStats,
    pub passed: bool,
    pub breaches: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exact: Vec<JsonGroup>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub near: Vec<JsonGroup>,
}

#[derive(serde::Serialize)]
struct JsonGroup {
    pub fingerprint: String,
    pub similarity: f64,
    pub members: Vec<JsonMember>,
}

#[derive(serde::Serialize)]
struct JsonMember {
    pub name: String,
    pub kind: String,
    pub file: String,
    pub line_start: usize,
    pub line_end: usize,
}

impl Reporter for JsonReporter {
    fn report(&self, report: &Report<'_>) -> io::Result<String> {
        let document = JsonReport {
            stats: Self::to_json_stats(report.stats),
            exact: self.to_json_groups(report.exact),
            near: self.to_json_groups(report.near),
            sub_exact: self.to_json_groups(report.sub_exact),
            sub_near: self.to_json_groups(report.sub_near),
        };
        Self::write_document(&document)
    }

    fn report_check(
        &self,
        stats: &DuplicationStats,
        breaches: &[CheckBreach<'_>],
    ) -> io::Result<String> {
        let groups_of = |exact: bool| {
            breaches
                .iter()
                .find(|breach| breach.of_exact == exact)
                .map_or_else(Vec::new, |breach| self.to_json_groups(breach.groups))
        };
        let document = JsonCheck {
            stats: Self::to_json_stats(stats),
            passed: breaches.is_empty(),
            breaches: breaches
                .iter()
                .map(|breach| breach.message.clone())
                .collect(),
            exact: groups_of(true),
            near: groups_of(false),
        };
        Self::write_document(&document)
    }

    fn report_stats(&self, stats: &DuplicationStats) -> io::Result<String> {
        Self::write_document(&Self::to_json_stats(stats))
    }

    fn report_exact(&self, groups: &[DuplicateGroup]) -> io::Result<String> {
        self.write_groups(groups)
    }

    fn report_near(&self, groups: &[DuplicateGroup]) -> io::Result<String> {
        self.write_groups(groups)
    }

    fn report_sub_exact(&self, groups: &[DuplicateGroup]) -> io::Result<String> {
        self.write_groups(groups)
    }

    fn report_sub_near(&self, groups: &[DuplicateGroup]) -> io::Result<String> {
        self.write_groups(groups)
    }
}

impl JsonReporter {
    fn to_json_group(&self, group: &DuplicateGroup) -> JsonGroup {
        JsonGroup {
            fingerprint: group.fingerprint().to_hex(),
            similarity: group.similarity(),
            members: group
                .members()
                .iter()
                .map(|m| JsonMember {
                    name: m.name.clone(),
                    kind: m.kind.to_string(),
                    file: DisplayPath::of(self.base_path.as_deref(), &m.file).into_owned(),
                    line_start: m.line_start,
                    line_end: m.line_end,
                })
                .collect(),
        }
    }

    fn to_json_groups(&self, groups: &[DuplicateGroup]) -> Vec<JsonGroup> {
        groups.iter().map(|g| self.to_json_group(g)).collect()
    }

    fn to_json_stats(stats: &DuplicationStats) -> JsonStats {
        JsonStats {
            total_code_units: stats.total_code_units(),
            total_lines: stats.total_lines(),
            exact_duplicate_groups: stats.exact_duplicate_groups(),
            exact_duplicate_units: stats.exact_duplicate_units(),
            near_duplicate_groups: stats.near_duplicate_groups(),
            near_duplicate_units: stats.near_duplicate_units(),
            exact_duplicate_lines: stats.exact_duplicate_lines(),
            near_duplicate_lines: stats.near_duplicate_lines(),
            exact_duplicate_percent: stats.exact_duplicate_percent(),
            near_duplicate_percent: stats.near_duplicate_percent(),
            sub_exact_groups: stats.sub_exact_groups(),
            sub_exact_units: stats.sub_exact_units(),
            sub_near_groups: stats.sub_near_groups(),
            sub_near_units: stats.sub_near_units(),
            baseline_suppressed: stats.baseline_suppressed(),
        }
    }

    fn write_document<T: serde::Serialize>(document: &T) -> io::Result<String> {
        let json = to_string_pretty(document).map_err(io::Error::other)?;
        Ok(format!("{json}\n"))
    }

    fn write_groups(&self, groups: &[DuplicateGroup]) -> io::Result<String> {
        Self::write_document(&self.to_json_groups(groups))
    }
}
