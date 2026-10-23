// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::analysis_output::AnalysisOutput;
use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::cli::output_format::OutputFormat;
use dry4rust::config::Config;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use dry4rust::suppression::baseline_file::BaselineFile;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tempfile::TempDir;
use validation::fixture::Fixture;
use validation::pipeline_scenario::PipelineScenario;

#[test]
fn analyze_over_a_duplicated_fixture_reports_exact_groups() {
    // Arrange & Act
    let (_, result) = Fixture::analysed("exact_dupes");

    // Assert
    assert!(result.stats().total_code_units() > 0);
    assert!(!result.exact_groups().is_empty());
}

#[test]
fn produce_applies_the_overrides_before_scanning() {
    // Arrange
    let root = Fixture::path("exact_dupes");
    let overrides = CliOverrides::default().with_min_nodes(Some(1000));

    // Act
    let output = PipelineScenario::produced(&root, &overrides);

    // Assert
    assert_eq!(output.config().min_nodes(), 1000);
    assert_eq!(
        output.result().stats().total_code_units(),
        0,
        "a floor of 1000 nodes admits nothing this fixture contains"
    );
}

#[test]
fn produce_ignoring_baseline_reports_what_produce_would_have_suppressed() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    fs::create_dir_all(tmp.path().join("src")).expect("src");
    fs::copy(
        Fixture::path("exact_dupes").join("src/target.rs"),
        tmp.path().join("src/lib.rs"),
    )
    .expect("copy the fixture");
    let judged = PipelineScenario::produced(tmp.path(), &CliOverrides::default());
    BaselineFile::record(judged.result())
        .save(&tmp.path().join("dry4rust-baseline.json"))
        .expect("the baseline is written");
    let overrides =
        CliOverrides::default().with_baseline(Some(PathBuf::from("dry4rust-baseline.json")));

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
        judging.result().exact_groups().len(),
        0,
        "judging against the baseline hides what it recorded"
    );
    assert_eq!(
        recording.result().exact_groups().len(),
        1,
        "recording against it must not, or the second recording empties the file"
    );
    assert_eq!(
        recording.config().baseline(),
        Some(Path::new("dry4rust-baseline.json")),
        "and the path survives, because that is where the recording is written"
    );
}

#[test]
fn produce_over_a_fixture_returns_config_and_result_together() {
    // Arrange
    let root = Fixture::path("exact_dupes");

    // Act
    let output = PipelineScenario::produced(&root, &CliOverrides::default());

    // Assert
    assert!(output.result().stats().total_code_units() > 0);
    assert_eq!(output.config().min_nodes(), Config::default().min_nodes());
}
