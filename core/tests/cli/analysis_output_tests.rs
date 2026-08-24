// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::analysis_output::AnalysisOutput;
use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::cli::output_format::OutputFormat;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use dry4rust::suppression::baseline_file::BaselineFile;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

const DUPLICATED: &str = r"pub fn process_data(input: Vec<i32>) -> i32 {
    let mut sum = 0;
    for item in input.iter() {
        if *item > 0 {
            sum += *item;
        }
    }
    sum
}

pub fn compute_total(values: Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in values.iter() {
        if *value > 0 {
            sum += *value;
        }
    }
    sum
}
";

#[test]
fn produce_ignoring_baseline_reports_what_produce_would_have_suppressed() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    fs::create_dir_all(tmp.path().join("src")).expect("src");
    fs::write(tmp.path().join("src/lib.rs"), DUPLICATED).expect("write");
    let judged = AnalysisOutput::produce(
        &RustAnalyzer::new(),
        tmp.path(),
        OutputFormat::Text,
        &CliOverrides::default(),
    )
    .expect("it analyses cleanly");
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
    .expect("it analyses cleanly");

    // Assert
    let judging = AnalysisOutput::produce(
        &RustAnalyzer::new(),
        tmp.path(),
        OutputFormat::Text,
        &overrides,
    )
    .expect("it analyses cleanly");
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
