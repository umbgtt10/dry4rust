// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::{cargo_dry4rust, fixture_path};
use dry4rust::baseline::baseline_file::BaselineFile;
use dry4rust::cli::analysis_output::AnalysisOutput;
use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::cli::output_format::OutputFormat;
use dry4rust::config::Config;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use predicate::str;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn error_on_nonexistent_path() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args(["--path", "/nonexistent/path/that/does/not/exist", "stats"])
        .assert()
        .code(2)
        .stderr(str::contains("No source files"));
}

#[test]
fn main_over_a_path_that_does_not_exist_reports_the_error_and_exits_non_zero() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .arg("--path")
        .arg(fixture_path("no_such_fixture_anywhere"))
        .assert()
        .failure()
        .stderr(str::contains("Error"));
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
        fixture_path("exact_dupes").join("src/lib.rs"),
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

#[test]
fn produce_over_a_root_with_no_source_files_names_the_root_it_looked_in() {
    // Arrange
    let empty = TempDir::new().expect("temp dir");

    // Act
    let outcome = AnalysisOutput::produce(
        &RustAnalyzer::new(),
        empty.path(),
        OutputFormat::Text,
        &CliOverrides::default(),
    );

    // Assert
    let Err(CliError::NoSourceFiles(path)) = outcome else {
        panic!("an empty root has no source files to analyse");
    };
    assert_eq!(path, empty.path());
}
