// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::fake_ports::FakePorts;
use crate::support::in_memory_source_tree::InMemorySourceTree;
use dry4rust::cli::analysis_output::AnalysisOutput;
use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::cli::suppression::baseline_file::BaselineFile;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use dry4rust::source_path::SourcePath;

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

fn duplicated_crate() -> FakePorts {
    FakePorts::with_sources(InMemorySourceTree::new().with_file("/p/src/lib.rs", DUPLICATED))
}

fn path(text: &str) -> SourcePath {
    SourcePath::new(String::from(text))
}

#[test]
fn produce_ignoring_baseline_reports_what_produce_would_have_suppressed() {
    // Arrange
    let fakes = duplicated_crate();
    let root = path("/p");
    let judged = AnalysisOutput::produce(
        &RustAnalyzer::new(),
        &fakes.ports(),
        &root,
        &CliOverrides::default(),
    )
    .expect("it analyses cleanly");
    BaselineFile::record(judged.result())
        .save(&fakes.baseline, &path("/p/dry4rust-baseline.json"))
        .expect("the baseline is written");
    let overrides = CliOverrides::default().with_baseline(Some(path("dry4rust-baseline.json")));

    // Act
    let recording = AnalysisOutput::produce_ignoring_baseline(
        &RustAnalyzer::new(),
        &fakes.ports(),
        &root,
        &overrides,
    )
    .expect("it analyses cleanly");

    // Assert
    let judging = AnalysisOutput::produce(&RustAnalyzer::new(), &fakes.ports(), &root, &overrides)
        .expect("it analyses cleanly");
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
}

#[test]
fn produce_keeps_the_configuration_its_overrides_set() {
    // Arrange
    let fakes = duplicated_crate();
    let overrides = CliOverrides::default().with_min_nodes(Some(12));

    // Act
    let output = AnalysisOutput::produce(
        &RustAnalyzer::new(),
        &fakes.ports(),
        &path("/p"),
        &overrides,
    )
    .expect("it analyses cleanly");

    // Assert
    assert_eq!(output.config().min_nodes(), 12);
}

#[test]
fn produce_over_a_root_with_no_source_files_names_the_root_it_looked_in() {
    // Arrange
    let empty = FakePorts::default();

    // Act
    let outcome = AnalysisOutput::produce(
        &RustAnalyzer::new(),
        &empty.ports(),
        &path("/empty"),
        &CliOverrides::default(),
    );

    // Assert
    let Err(CliError::NoSourceFiles(path)) = outcome else {
        panic!("an empty root has no source files to analyse");
    };
    assert_eq!(path, SourcePath::new(String::from("/empty")));
}
