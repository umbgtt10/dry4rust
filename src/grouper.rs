// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use crate::code_unit::CodeUnit;
use crate::fingerprint::Fingerprint;
use crate::near_duplicate::near_duplicate_finder::NearDuplicateFinder;

/// A group of duplicate code units.
#[derive(Debug, Clone)]
pub struct DuplicateGroup {
    /// Shared fingerprint for exact duplicates, or composite fingerprint
    /// (derived from sorted member fingerprints) for near-duplicate groups.
    pub fingerprint: Fingerprint,
    /// The code units in this group.
    pub members: Vec<CodeUnit>,
    /// Similarity score (1.0 for exact duplicates).
    pub similarity: f64,
}

/// Statistics about duplication in the analyzed codebase.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DuplicationStats {
    pub total_code_units: usize,
    pub total_lines: usize,
    pub exact_duplicate_groups: usize,
    pub exact_duplicate_units: usize,
    pub near_duplicate_groups: usize,
    pub near_duplicate_units: usize,
    pub exact_duplicate_lines: usize,
    pub near_duplicate_lines: usize,
    // Sub-function stats
    pub sub_exact_groups: usize,
    pub sub_exact_units: usize,
    pub sub_near_groups: usize,
    pub sub_near_units: usize,
}

impl DuplicationStats {
    fn percent_of_total(&self, lines: usize) -> f64 {
        if self.total_lines == 0 {
            0.0
        } else {
            lines as f64 / self.total_lines as f64 * 100.0
        }
    }

    /// Percentage of total lines that are exact duplicates.
    #[must_use]
    pub fn exact_duplicate_percent(&self) -> f64 {
        self.percent_of_total(self.exact_duplicate_lines)
    }

    /// Percentage of total lines that are near duplicates.
    #[must_use]
    pub fn near_duplicate_percent(&self) -> f64 {
        self.percent_of_total(self.near_duplicate_lines)
    }
}

/// Group code units by exact fingerprint match.
#[must_use]
pub fn group_exact_duplicates(units: &[CodeUnit]) -> Vec<DuplicateGroup> {
    let mut groups: HashMap<Fingerprint, Vec<CodeUnit>> = HashMap::new();

    for unit in units {
        groups
            .entry(unit.fingerprint)
            .or_default()
            .push(unit.clone());
    }

    let mut result: Vec<DuplicateGroup> = groups
        .into_iter()
        .filter(|(_, members)| members.len() > 1)
        .map(|(fp, members)| DuplicateGroup {
            fingerprint: fp,
            members,
            similarity: 1.0,
        })
        .collect();

    // Sort by group size (largest first), then by fingerprint for stability
    result.sort_by(|a, b| {
        b.members
            .len()
            .cmp(&a.members.len())
            .then_with(|| a.fingerprint.cmp(&b.fingerprint))
    });

    result
}

/// Find near-duplicate groups above the similarity threshold.
/// Pre-filters by `CodeUnitKind` and approximate size to reduce pairwise comparisons.
#[must_use]
pub fn find_near_duplicates(
    units: &[CodeUnit],
    threshold: f64,
    exact_fingerprints: &[Fingerprint],
) -> Vec<DuplicateGroup> {
    NearDuplicateFinder::new(threshold).find(units, exact_fingerprints)
}

/// Compute the total number of source lines in a duplicate group.
fn group_line_count(group: &DuplicateGroup) -> usize {
    group
        .members
        .iter()
        .map(|m| m.line_end.saturating_sub(m.line_start) + 1)
        .sum()
}

/// Compute duplication statistics.
pub fn compute_stats(
    units: &[CodeUnit],
    exact_groups: &[DuplicateGroup],
    near_groups: &[DuplicateGroup],
) -> DuplicationStats {
    let total_lines: usize = units
        .iter()
        .map(|u| u.line_end.saturating_sub(u.line_start) + 1)
        .sum();

    DuplicationStats {
        total_code_units: units.len(),
        total_lines,
        exact_duplicate_groups: exact_groups.len(),
        exact_duplicate_units: exact_groups.iter().map(|g| g.members.len()).sum(),
        near_duplicate_groups: near_groups.len(),
        near_duplicate_units: near_groups.iter().map(|g| g.members.len()).sum(),
        exact_duplicate_lines: exact_groups.iter().map(group_line_count).sum(),
        near_duplicate_lines: near_groups.iter().map(group_line_count).sum(),
        sub_exact_groups: 0,
        sub_exact_units: 0,
        sub_near_groups: 0,
        sub_near_units: 0,
    }
}

/// Compute duplication statistics including sub-function results.
#[must_use]
pub fn compute_stats_with_sub(
    units: &[CodeUnit],
    exact_groups: &[DuplicateGroup],
    near_groups: &[DuplicateGroup],
    sub_exact_groups: &[DuplicateGroup],
    sub_near_groups: &[DuplicateGroup],
) -> DuplicationStats {
    let mut stats = compute_stats(units, exact_groups, near_groups);
    stats.sub_exact_groups = sub_exact_groups.len();
    stats.sub_exact_units = sub_exact_groups.iter().map(|g| g.members.len()).sum();
    stats.sub_near_groups = sub_near_groups.len();
    stats.sub_near_units = sub_near_groups.iter().map(|g| g.members.len()).sum();
    stats
}
