// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use assert_cmd::Command;
use assert_cmd::cargo::cargo_bin_cmd;
use dry4rust::analysis::AnalysisResult;
use dry4rust::analysis::analyze;
use dry4rust::code_unit::CodeUnit;
use dry4rust::code_unit::CodeUnitKind;
use dry4rust::config::Config;
use dry4rust::fingerprint::Fingerprint;
use dry4rust::grouper::DuplicateGroup;
use dry4rust::grouper::compute_stats_with_sub;
use dry4rust::node::NodeKind;
use dry4rust::node::NormalizedNode;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use dry4rust::scanner::ScanConfig;
use dry4rust::scanner::scan_files;
use std::collections::HashSet;
use std::path::PathBuf;

/// Run the analysis pipeline over a fixture crate, as the CLI would.
pub fn analysed(fixture: &str) -> (Config, AnalysisResult) {
    let root = fixture_path(fixture);
    let config = Config::load(&root).expect("the fixture configuration is in range");
    let files = scan_files(&ScanConfig::new(root));
    let result =
        analyze(&RustAnalyzer::new(), &files, &config).expect("the fixture analyses cleanly");
    (config, result)
}

/// The same, with sub-function analysis on, which no fixture configures for
/// itself.
pub fn analysed_with_sub_function(fixture: &str) -> (Config, AnalysisResult) {
    let root = fixture_path(fixture);
    let config = Config {
        sub_function: true,
        ..Config::load(&root).expect("the fixture configuration is in range")
    };
    let files = scan_files(&ScanConfig::new(root));
    let result =
        analyze(&RustAnalyzer::new(), &files, &config).expect("the fixture analyses cleanly");
    (config, result)
}

pub fn cargo_dry4rust() -> Command {
    cargo_bin_cmd!("cargo-dry4rust")
}

pub fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

/// A duplicate group with a stated fingerprint and members, for tests that
/// care about grouping rather than about the code behind it.
pub fn group(fingerprint: u64, names: &[&str]) -> DuplicateGroup {
    let fingerprint = Fingerprint::new(fingerprint);
    DuplicateGroup {
        fingerprint,
        members: names
            .iter()
            .map(|name| CodeUnit {
                kind: CodeUnitKind::Function,
                name: (*name).to_owned(),
                file: PathBuf::from("src/lib.rs"),
                line_start: 1,
                line_end: 9,
                signature: NormalizedNode::leaf(NodeKind::Opaque),
                body: NormalizedNode::leaf(NodeKind::Opaque),
                fingerprint,
                node_count: 9,
                parent_name: None,
                is_test: false,
            })
            .collect(),
        similarity: 1.0,
    }
}

/// An analysis result holding exactly these groups, for tests about what is
/// done with groups rather than about how they were found.
pub fn result_with(
    exact: Vec<DuplicateGroup>,
    near: Vec<DuplicateGroup>,
    sub_exact: Vec<DuplicateGroup>,
    sub_near: Vec<DuplicateGroup>,
) -> AnalysisResult {
    AnalysisResult {
        stats: compute_stats_with_sub(&[], &exact, &near, &sub_exact, &sub_near),
        exact_groups: exact,
        near_groups: near,
        sub_exact_groups: sub_exact,
        sub_near_groups: sub_near,
        warnings: Vec::new(),
        all_fingerprints: HashSet::new(),
    }
}
