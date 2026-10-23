// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use crate::code_unit::CodeUnit;
use crate::fingerprint::Fingerprint;
use crate::near_duplicate::near_duplicate_finder::NearDuplicateFinder;
use crate::output::duplication_stats::DuplicationStats;

/// A group of duplicate code units.
#[derive(Debug, Clone)]
pub struct DuplicateGroup {
    /// Shared fingerprint for exact duplicates, or composite fingerprint
    /// (derived from sorted member fingerprints) for near-duplicate groups.
    fingerprint: Fingerprint,
    /// The code units in this group.
    members: Vec<CodeUnit>,
    /// Similarity score (1.0 for exact duplicates).
    similarity: f64,
}

impl DuplicateGroup {
    /// Group code units by exact fingerprint match.
    #[must_use]
    pub fn group_exact_duplicates(units: &[CodeUnit]) -> Vec<Self> {
        let mut groups: HashMap<Fingerprint, Vec<CodeUnit>> = HashMap::new();

        for unit in units {
            groups
                .entry(unit.fingerprint())
                .or_default()
                .push(unit.clone());
        }

        let mut result: Vec<Self> = groups
            .into_iter()
            .filter(|(_, members)| members.len() > 1)
            .map(|(fp, members)| Self {
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
    ) -> Vec<Self> {
        NearDuplicateFinder::new(threshold).find(units, exact_fingerprints)
    }

    #[must_use]
    pub const fn new(fingerprint: Fingerprint, members: Vec<CodeUnit>, similarity: f64) -> Self {
        Self {
            fingerprint,
            members,
            similarity,
        }
    }

    /// Compute duplication statistics.
    pub fn compute_stats(
        units: &[CodeUnit],
        exact_groups: &[Self],
        near_groups: &[Self],
    ) -> DuplicationStats {
        let total_lines: usize = units
            .iter()
            .map(|u| u.line_end().saturating_sub(u.line_start()) + 1)
            .sum();

        DuplicationStats::default()
            .with_total_code_units(units.len())
            .with_total_lines(total_lines)
            .with_exact_duplicate_groups(exact_groups.len())
            .with_exact_duplicate_units(exact_groups.iter().map(|g| g.members.len()).sum())
            .with_near_duplicate_groups(near_groups.len())
            .with_near_duplicate_units(near_groups.iter().map(|g| g.members.len()).sum())
            .with_exact_duplicate_lines(exact_groups.iter().map(Self::group_line_count).sum())
            .with_near_duplicate_lines(near_groups.iter().map(Self::group_line_count).sum())
    }

    /// Compute duplication statistics including sub-function results.
    #[must_use]
    pub fn compute_stats_with_sub(
        units: &[CodeUnit],
        exact_groups: &[Self],
        near_groups: &[Self],
        sub_exact_groups: &[Self],
        sub_near_groups: &[Self],
    ) -> DuplicationStats {
        Self::compute_stats(units, exact_groups, near_groups)
            .with_sub_exact_groups(sub_exact_groups.len())
            .with_sub_exact_units(sub_exact_groups.iter().map(|g| g.members.len()).sum())
            .with_sub_near_groups(sub_near_groups.len())
            .with_sub_near_units(sub_near_groups.iter().map(|g| g.members.len()).sum())
    }

    #[must_use]
    pub const fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }

    #[must_use]
    pub fn members(&self) -> &[CodeUnit] {
        &self.members
    }

    #[must_use]
    pub const fn similarity(&self) -> f64 {
        self.similarity
    }

    /// Compute the total number of source lines in a duplicate group.
    fn group_line_count(group: &Self) -> usize {
        group
            .members
            .iter()
            .map(|m| m.line_end().saturating_sub(m.line_start()) + 1)
            .sum()
    }
}
