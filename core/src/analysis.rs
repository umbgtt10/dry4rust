// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

use crate::analyzer::LanguageAnalyzer;
use crate::cli::suppression::baseline_filter::BaselineFilter;
use crate::cli::suppression::ignore_file::IgnoreFile;
use crate::code_unit::CodeUnit;
use crate::config::Config;
use crate::fingerprint::Fingerprint;
use crate::groups::duplicate_group::DuplicateGroup;
use crate::groups::duplication_stats::DuplicationStats;
use crate::groups::findings::Findings;
use crate::node::node_kind::NodeKind;
use crate::node::normalized_node::NormalizedNode;
use crate::source_path::SourcePath;
use crate::sub_unit::SubUnit;
use crate::traits::source_tree::SourceTree;

/// The result of a full analysis run.
pub struct Analysis {
    stats: DuplicationStats,
    findings: Findings,
    warnings: Vec<String>,
    /// All group fingerprints (exact + near) before ignore filtering.
    /// Used by the cleanup command to identify stale ignore entries.
    all_fingerprints: BTreeSet<Fingerprint>,
}

impl Analysis {
    /// Run the analysis pipeline on pre-parsed code units.
    ///
    /// The caller is responsible for scanning files and parsing them into
    /// `CodeUnit`s, and for loading the ignore file and the baseline. This
    /// function handles grouping, similarity detection, suppression, and stats.
    #[must_use]
    pub fn analyze_units(
        units: &[CodeUnit],
        warnings: Vec<String>,
        config: &Config,
        ignore_file: &IgnoreFile,
        baseline: &BaselineFilter,
    ) -> Self {
        // 1. Group exact duplicates
        let exact_groups = DuplicateGroup::group_exact_duplicates(units);

        // 2. Find near-duplicates
        let exact_fps: Vec<_> = exact_groups
            .iter()
            .map(crate::groups::duplicate_group::DuplicateGroup::fingerprint)
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
                .map(crate::groups::duplicate_group::DuplicateGroup::fingerprint)
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

        // 4. Every fingerprint found, before any is set aside (for cleanup's
        //    staleness check)
        let found = Findings::new(exact_groups, near_groups, sub_exact_groups, sub_near_groups);
        let all_fingerprints = found.fingerprints();

        // 5. Set aside what the ignore file suppresses, then what the
        //    baseline already accounted for
        let found = found.retain_unsuppressed(ignore_file);
        let found_count = found.count();
        let findings = found.retain_new(baseline);

        // 6. Compute stats
        let stats = findings.stats(units).with_baseline_suppressed(
            baseline
                .is_in_effect()
                .then_some(found_count - findings.count()),
        );

        Self {
            stats,
            findings,
            warnings,
            all_fingerprints,
        }
    }

    /// Run the full analysis pipeline using a language analyzer.
    ///
    /// Reads each file from `sources`, parses it via the analyzer, optionally
    /// filters test code, then delegates to [`Self::analyze_units`] for
    /// grouping, similarity, suppression and stats.
    #[must_use]
    pub fn analyze(
        analyzer: &dyn LanguageAnalyzer,
        sources: &dyn SourceTree,
        files: &[SourcePath],
        config: &Config,
        ignore_file: &IgnoreFile,
        baseline: &BaselineFilter,
    ) -> Self {
        let analysis_config = config.analysis_config();
        let mut units = Vec::new();
        let mut warnings = Vec::new();

        for path in files {
            let source = match sources.read_source(path) {
                Ok(s) => s,
                Err(e) => {
                    warnings.push(format!("Failed to read {path}: {e}"));
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

        Self::analyze_units(&units, warnings, config, ignore_file, baseline)
    }

    #[must_use]
    pub const fn new(
        stats: DuplicationStats,
        exact_groups: Vec<DuplicateGroup>,
        near_groups: Vec<DuplicateGroup>,
        sub_exact_groups: Vec<DuplicateGroup>,
        sub_near_groups: Vec<DuplicateGroup>,
        warnings: Vec<String>,
        all_fingerprints: BTreeSet<Fingerprint>,
    ) -> Self {
        Self {
            stats,
            findings: Findings::new(exact_groups, near_groups, sub_exact_groups, sub_near_groups),
            warnings,
            all_fingerprints,
        }
    }

    #[must_use]
    pub const fn all_fingerprints(&self) -> &BTreeSet<Fingerprint> {
        &self.all_fingerprints
    }

    #[must_use]
    pub fn exact_groups(&self) -> &[DuplicateGroup] {
        self.findings.exact()
    }

    #[must_use]
    pub fn near_groups(&self) -> &[DuplicateGroup] {
        self.findings.near()
    }

    #[must_use]
    pub const fn stats(&self) -> &DuplicationStats {
        &self.stats
    }

    #[must_use]
    pub fn sub_exact_groups(&self) -> &[DuplicateGroup] {
        self.findings.sub_exact()
    }

    #[must_use]
    pub fn sub_near_groups(&self) -> &[DuplicateGroup] {
        self.findings.sub_near()
    }

    #[must_use]
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }
}
