// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::path::PathBuf;

use crate::analyzer::LanguageAnalyzer;
use crate::code_unit::CodeUnit;
use crate::config::Config;
use crate::extractor::extract_sub_units;
use crate::fingerprint::Fingerprint;
use crate::grouper::compute_stats_with_sub;
use crate::grouper::find_near_duplicates;
use crate::grouper::group_exact_duplicates;
use crate::grouper::{DuplicateGroup, DuplicationStats};
use crate::ignore::filter_ignored;
use crate::ignore::load_ignore_file;
use crate::node::NormalizedNode;
use std::fs;

/// The result of a full analysis run.
pub struct AnalysisResult {
    pub stats: DuplicationStats,
    pub exact_groups: Vec<DuplicateGroup>,
    pub near_groups: Vec<DuplicateGroup>,
    pub sub_exact_groups: Vec<DuplicateGroup>,
    pub sub_near_groups: Vec<DuplicateGroup>,
    pub warnings: Vec<String>,
    /// All group fingerprints (exact + near) before ignore filtering.
    /// Used by the cleanup command to identify stale ignore entries.
    pub all_fingerprints: HashSet<Fingerprint>,
}

/// Run the full analysis pipeline using a language analyzer.
///
/// Reads each file, parses it via the analyzer, optionally filters test code,
/// then delegates to [`analyze_units`] for grouping, similarity, and stats.
pub fn analyze(
    analyzer: &dyn LanguageAnalyzer,
    files: &[PathBuf],
    config: &Config,
) -> crate::error::Result<AnalysisResult> {
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
                if config.exclude_tests {
                    file_units.retain(|u| !analyzer.is_test_code(u));
                }
                units.extend(file_units);
            }
            Err(e) => warnings.push(e.to_string()),
        }
    }

    analyze_units(&units, warnings, config)
}

/// Run the analysis pipeline on pre-parsed code units.
///
/// The caller is responsible for scanning files and parsing them into `CodeUnit`s.
/// This function handles grouping, similarity detection, ignore filtering, and stats.
pub fn analyze_units(
    units: &[CodeUnit],
    warnings: Vec<String>,
    config: &Config,
) -> crate::error::Result<AnalysisResult> {
    // 1. Group exact duplicates
    let exact_groups = group_exact_duplicates(units);

    // 2. Find near-duplicates
    let exact_fps: Vec<_> = exact_groups.iter().map(|g| g.fingerprint).collect();
    let near_groups = find_near_duplicates(units, config.similarity_threshold, &exact_fps);

    // 3. Sub-function duplicate detection (opt-in)
    let (sub_exact_groups, sub_near_groups) = if config.sub_function {
        // Extract sub-units from each code unit
        let sub_units: Vec<CodeUnit> = units
            .iter()
            .flat_map(|unit| {
                let sub_units = extract_sub_units(&unit.body, config.min_sub_nodes);
                sub_units.into_iter().map(|su| CodeUnit {
                    kind: su.kind,
                    name: su.description,
                    file: unit.file.clone(),
                    line_start: unit.line_start,
                    line_end: unit.line_end,
                    signature: NormalizedNode::leaf(crate::node::NodeKind::Opaque),
                    body: su.node.clone(),
                    fingerprint: Fingerprint::from_node(&su.node),
                    node_count: su.node_count,
                    parent_name: Some(unit.name.clone()),
                    is_test: unit.is_test,
                })
            })
            .collect();

        let sub_exact = group_exact_duplicates(&sub_units);
        let sub_exact_fps: Vec<_> = sub_exact.iter().map(|g| g.fingerprint).collect();
        let sub_near =
            find_near_duplicates(&sub_units, config.similarity_threshold, &sub_exact_fps);
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
        .map(|g| g.fingerprint)
        .collect();

    // 5. Apply ignore filtering
    let ignore_file = load_ignore_file(&config.root);
    let exact_groups = filter_ignored(exact_groups, &ignore_file);
    let near_groups = filter_ignored(near_groups, &ignore_file);
    let sub_exact_groups = filter_ignored(sub_exact_groups, &ignore_file);
    let sub_near_groups = filter_ignored(sub_near_groups, &ignore_file);

    // 6. Compute stats
    let stats = compute_stats_with_sub(
        units,
        &exact_groups,
        &near_groups,
        &sub_exact_groups,
        &sub_near_groups,
    );

    Ok(AnalysisResult {
        stats,
        exact_groups,
        near_groups,
        sub_exact_groups,
        sub_near_groups,
        warnings,
        all_fingerprints,
    })
}
