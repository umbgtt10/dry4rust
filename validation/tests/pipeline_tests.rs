// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::analysed;
use crate::common::fixture_path;
use dry4rust::cli::analysis_output::AnalysisOutput;
use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::cli::output_format::OutputFormat;
use dry4rust::config::Config;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use dry4rust::suppression::baseline_file::BaselineFile;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn analyze_over_a_duplicated_fixture_reports_exact_groups() {
    // Arrange & Act
    let (_, result) = analysed("exact_dupes");

    // Assert
    assert!(result.stats.total_code_units > 0);
    assert!(!result.exact_groups.is_empty());
}

#[test]
fn produce_applies_the_overrides_before_scanning() {
    // Arrange
    let root = fixture_path("exact_dupes");
    let overrides = CliOverrides {
        min_nodes: Some(1000),
        ..CliOverrides::default()
    };

    // Act
    let output =
        AnalysisOutput::produce(&RustAnalyzer::new(), &root, OutputFormat::Text, &overrides)
            .expect("the fixture analyses cleanly");

    // Assert
    assert_eq!(output.config.min_nodes, 1000);
    assert_eq!(
        output.result.stats.total_code_units, 0,
        "a floor of 1000 nodes admits nothing this fixture contains"
    );
}

#[test]
fn produce_ignoring_baseline_reports_what_produce_would_have_suppressed() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    fs::create_dir_all(tmp.path().join("src")).expect("src");
    fs::copy(
        fixture_path("exact_dupes").join("src/target.rs"),
        tmp.path().join("src/lib.rs"),
    )
    .expect("copy the fixture");
    let judged = AnalysisOutput::produce(
        &RustAnalyzer::new(),
        tmp.path(),
        OutputFormat::Text,
        &CliOverrides::default(),
    )
    .expect("the fixture analyses cleanly");
    BaselineFile::record(&judged.result)
        .save(&tmp.path().join("dry4rust-baseline.json"))
        .expect("the baseline is written");
    let overrides = CliOverrides {
        baseline: Some(PathBuf::from("dry4rust-baseline.json")),
        ..CliOverrides::default()
    };

    // Act
    let recording = AnalysisOutput::produce_ignoring_baseline(
        &RustAnalyzer::new(),
        tmp.path(),
        OutputFormat::Text,
        &overrides,
    )
    .expect("the fixture analyses cleanly");

    // Assert
    let judging = AnalysisOutput::produce(
        &RustAnalyzer::new(),
        tmp.path(),
        OutputFormat::Text,
        &overrides,
    )
    .expect("the fixture analyses cleanly");
    assert_eq!(
        judging.result.exact_groups.len(),
        0,
        "judging against the baseline hides what it recorded"
    );
    assert_eq!(
        recording.result.exact_groups.len(),
        1,
        "recording against it must not, or the second recording empties the file"
    );
    assert_eq!(
        recording.config.baseline,
        Some(PathBuf::from("dry4rust-baseline.json")),
        "and the path survives, because that is where the recording is written"
    );
}

#[test]
fn produce_over_a_fixture_returns_config_and_result_together() {
    // Arrange
    let root = fixture_path("exact_dupes");

    // Act
    let output = AnalysisOutput::produce(
        &RustAnalyzer::new(),
        &root,
        OutputFormat::Text,
        &CliOverrides::default(),
    )
    .expect("the fixture analyses cleanly");

    // Assert
    assert!(output.result.stats.total_code_units > 0);
    assert_eq!(output.config.min_nodes, Config::default().min_nodes);
}
