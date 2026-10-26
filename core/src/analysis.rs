// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::path::PathBuf;

use crate::analyzer::LanguageAnalyzer;
use crate::cli::suppression::baseline_filter::BaselineFilter;
use crate::cli::suppression::baseline_kind::BaselineKind;
use crate::cli::suppression::ignore_file::IgnoreFile;
use crate::code_unit::CodeUnit;
use crate::config::Config;
use crate::duplicate_group::DuplicateGroup;
use crate::duplication_stats::DuplicationStats;
use crate::error::Result;
use crate::fingerprint::Fingerprint;
use crate::node::node_kind::NodeKind;
use crate::node::normalized_node::NormalizedNode;
use crate::sub_unit::SubUnit;
use std::fs;

/// The result of a full analysis run.
pub struct Analysis {
    stats: DuplicationStats,
    exact_groups: Vec<DuplicateGroup>,
    near_groups: Vec<DuplicateGroup>,
    sub_exact_groups: Vec<DuplicateGroup>,
    sub_near_groups: Vec<DuplicateGroup>,
    warnings: Vec<String>,
    /// All group fingerprints (exact + near) before ignore filtering.
    /// Used by the cleanup command to identify stale ignore entries.
    all_fingerprints: HashSet<Fingerprint>,
}

impl Analysis {
    /// Run the full analysis pipeline using a language analyzer.
    ///
    /// Reads each file, parses it via the analyzer, optionally filters test code,
    /// then delegates to [`analyze_units`] for grouping, similarity, and stats.
    pub fn analyze(
        analyzer: &dyn LanguageAnalyzer,
        files: &[PathBuf],
        config: &Config,
    ) -> Result<Self> {
        let analysis_config = config.analysis_config();
        let mut units = Vec::new();
        let mut warnings = Vec::new();

        for path in files {
            let source = match fs::read_to_string(path) {
                Ok(s) => s,
                Err(e) => {
                    warnings.push(format!("Failed to read {}: {}", path.display(), e));
                    continue;
                }
            };
            match analyzer.parse_file(path, &source, &analysis_config) {
                Ok(mut file_units) => {
                    if config.exclude_tests() {
                        file_units.retain(|u| !analyzer.is_test_code(u));
                    }
                    units.extend(file_units);
                }
                Err(e) => warnings.push(e.to_string()),
            }
        }

        Self::analyze_units(&units, warnings, config)
    }

    /// Run the analysis pipeline on pre-parsed code units.
    ///
    /// The caller is responsible for scanning files and parsing them into `CodeUnit`s.
    /// This function handles grouping, similarity detection, ignore filtering, and stats.
    pub fn analyze_units(
        units: &[CodeUnit],
        warnings: Vec<String>,
        config: &Config,
    ) -> Result<Self> {
        // 1. Group exact duplicates
        let exact_groups = DuplicateGroup::group_exact_duplicates(units);

        // 2. Find near-duplicates
        let exact_fps: Vec<_> = exact_groups
            .iter()
            .map(super::duplicate_group::DuplicateGroup::fingerprint)
            .collect();
        let near_groups = DuplicateGroup::find_near_duplicates(
            units,
            config.similarity_threshold().as_fraction(),
            &exact_fps,
        );

        // 3. Sub-function duplicate detection (opt-in)
        let (sub_exact_groups, sub_near_groups) = if config.sub_function() {
            // Extract sub-units from each code unit
            let sub_units: Vec<CodeUnit> = units
                .iter()
                .flat_map(|unit| {
                    let sub_units = SubUnit::extract_sub_units(&unit.body, config.min_sub_nodes());
                    sub_units.into_iter().map(|su| CodeUnit {
                        kind: su.kind().clone(),
                        name: su.description().to_string(),
                        file: unit.file.clone(),
                        line_start: unit.line_start,
                        line_end: unit.line_end,
                        signature: NormalizedNode::leaf(NodeKind::Opaque),
                        body: su.node().clone(),
                        fingerprint: Fingerprint::from_node(su.node()),
                        node_count: su.node_count(),
                        parent_name: Some(unit.name.clone()),
                        is_test: unit.is_test,
                    })
                })
                .collect();

            let sub_exact = DuplicateGroup::group_exact_duplicates(&sub_units);
            let sub_exact_fps: Vec<_> = sub_exact
                .iter()
                .map(super::duplicate_group::DuplicateGroup::fingerprint)
                .collect();
            let sub_near = DuplicateGroup::find_near_duplicates(
                &sub_units,
                config.similarity_threshold().as_fraction(),
                &sub_exact_fps,
            );
            (sub_exact, sub_near)
        } else {
            (Vec::new(), Vec::new())
        };

        // 4. Collect all fingerprints before filtering (for cleanup staleness check)
        let all_fingerprints: HashSet<Fingerprint> = exact_groups
            .iter()
            .chain(near_groups.iter())
            .chain(sub_exact_groups.iter())
            .chain(sub_near_groups.iter())
            .map(super::duplicate_group::DuplicateGroup::fingerprint)
            .collect();

        // 5. Apply ignore filtering
        let ignore_file = IgnoreFile::load(config.root());
        let exact_groups = ignore_file.retain_unsuppressed(exact_groups);
        let near_groups = ignore_file.retain_unsuppressed(near_groups);
        let sub_exact_groups = ignore_file.retain_unsuppressed(sub_exact_groups);
        let sub_near_groups = ignore_file.retain_unsuppressed(sub_near_groups);

        // 6. Set aside what the baseline already accounted for
        let baseline = BaselineFilter::load(config)?;
        let found =
            exact_groups.len() + near_groups.len() + sub_exact_groups.len() + sub_near_groups.len();
        let exact_groups = baseline.retain_new(BaselineKind::Exact, exact_groups);
        let near_groups = baseline.retain_new(BaselineKind::Near, near_groups);
        let sub_exact_groups = baseline.retain_new(BaselineKind::SubExact, sub_exact_groups);
        let sub_near_groups = baseline.retain_new(BaselineKind::SubNear, sub_near_groups);
        let reported =
            exact_groups.len() + near_groups.len() + sub_exact_groups.len() + sub_near_groups.len();

        // 7. Compute stats
        let stats = DuplicateGroup::compute_stats_with_sub(
            units,
            &exact_groups,
            &near_groups,
            &sub_exact_groups,
            &sub_near_groups,
        )
        .with_baseline_suppressed(baseline.is_in_effect().then_some(found - reported));

        Ok(Self {
            stats,
            exact_groups,
            near_groups,
            sub_exact_groups,
            sub_near_groups,
            warnings,
            all_fingerprints,
        })
    }

    #[must_use]
    pub const fn new(
        stats: DuplicationStats,
        exact_groups: Vec<DuplicateGroup>,
        near_groups: Vec<DuplicateGroup>,
        sub_exact_groups: Vec<DuplicateGroup>,
        sub_near_groups: Vec<DuplicateGroup>,
        warnings: Vec<String>,
        all_fingerprints: HashSet<Fingerprint>,
    ) -> Self {
        Self {
            stats,
            exact_groups,
            near_groups,
            sub_exact_groups,
            sub_near_groups,
            warnings,
            all_fingerprints,
        }
    }

    #[must_use]
    pub const fn all_fingerprints(&self) -> &HashSet<Fingerprint> {
        &self.all_fingerprints
    }

    #[must_use]
    pub fn exact_groups(&self) -> &[DuplicateGroup] {
        &self.exact_groups
    }

    #[must_use]
    pub fn near_groups(&self) -> &[DuplicateGroup] {
        &self.near_groups
    }

    #[must_use]
    pub const fn stats(&self) -> &DuplicationStats {
        &self.stats
    }

    #[must_use]
    pub fn sub_exact_groups(&self) -> &[DuplicateGroup] {
        &self.sub_exact_groups
    }

    #[must_use]
    pub fn sub_near_groups(&self) -> &[DuplicateGroup] {
        &self.sub_near_groups
    }

    #[must_use]
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }
}
