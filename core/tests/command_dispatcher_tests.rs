// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::fake_ports::FakePorts;
use crate::support::in_memory_source_tree::InMemorySourceTree;
use crate::support::summary_reporter::SummaryReporter;
use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::cli::command::Command;
use dry4rust::cli::output::text_reporter::TextReporter;
use dry4rust::command_dispatcher::CommandDispatcher;
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

fn dispatch_in(fakes: &FakePorts, command: &Command) -> (Result<(), CliError>, String) {
    let analyzer = RustAnalyzer::new();
    let root = root();
    let reporter = TextReporter::new(Some(root.clone()));
    let dispatcher = CommandDispatcher::new(
        &analyzer,
        &root,
        &reporter,
        CliOverrides::default(),
        fakes.ports(),
    );
    match dispatcher.dispatch(command) {
        Ok(outcome) => outcome.into_parts(),
        Err(error) => (Err(error), String::new()),
    }
}

fn root() -> SourcePath {
    SourcePath::new(String::from("/p"))
}

#[test]
fn dispatch_over_a_file_that_does_not_parse_carries_its_warning_in_the_outcome() {
    // Arrange
    let fakes = FakePorts::with_sources(
        InMemorySourceTree::new()
            .with_file("/p/src/lib.rs", DUPLICATED)
            .with_file("/p/src/broken.rs", "fn broken( {"),
    );
    let analyzer = RustAnalyzer::new();
    let root = root();
    let dispatcher = CommandDispatcher::new(
        &analyzer,
        &root,
        &SummaryReporter,
        CliOverrides::default(),
        fakes.ports(),
    );

    // Act
    let outcome = dispatcher
        .dispatch(&Command::Stats)
        .expect("one file that does not parse is a warning, not a failure");

    // Assert
    assert_eq!(outcome.warnings().len(), 1);
    assert!(
        outcome.warnings()[0].starts_with("Failed to parse /p/src/broken.rs: "),
        "{:?}",
        outcome.warnings()
    );
    assert_eq!(outcome.text(), "stats units=2\n");
}

#[test]
fn dispatch_over_sources_that_cannot_be_listed_names_the_root() {
    // Arrange
    let fakes = FakePorts::with_sources(InMemorySourceTree::unlistable(
        "/p/src/\u{FFFD}.rs: the path is not valid UTF-8",
    ));

    // Act
    let (outcome, _) = dispatch_in(&fakes, &Command::Stats);

    // Assert
    let Err(error) = outcome else {
        panic!("a tree that cannot be listed cannot be analysed");
    };
    assert_eq!(
        error.to_string(),
        "Failed to read /p: /p/src/\u{FFFD}.rs: the path is not valid UTF-8"
    );
}

#[test]
fn dispatch_with_a_ceiling_out_of_range_keeps_the_warnings_beside_the_error() {
    // Arrange
    let fakes = FakePorts::with_sources(
        InMemorySourceTree::new()
            .with_file("/p/src/lib.rs", DUPLICATED)
            .with_file("/p/src/broken.rs", "fn broken( {"),
    );
    let command = Command::Check {
        max_exact: None,
        max_near: None,
        max_exact_percent: Some(150.0),
        max_near_percent: None,
    };
    let analyzer = RustAnalyzer::new();
    let root = root();
    let dispatcher = CommandDispatcher::new(
        &analyzer,
        &root,
        &SummaryReporter,
        CliOverrides::default(),
        fakes.ports(),
    );

    // Act
    let outcome = dispatcher
        .dispatch(&command)
        .expect("the analysis ran, so its warnings travel with the error");

    // Assert
    assert_eq!(outcome.warnings().len(), 1);
    assert_eq!(outcome.text(), "");
    let Err(CliError::InvalidConfig(message)) = outcome.result() else {
        panic!("a ceiling of 150% is refused");
    };
    assert_eq!(
        message,
        "--max-exact-percent must be a percentage between 0.0 and 100.0, got 150"
    );
}

#[test]
fn dispatch_with_ignore_records_the_fingerprint_without_analysing() {
    // Arrange
    let fakes = FakePorts::default();
    let command = Command::Ignore {
        fingerprint: "cafebabe00000001".to_string(),
        reason: Some("a reason".to_string()),
    };

    // Act
    let (outcome, _) = dispatch_in(&fakes, &command);

    // Assert
    assert!(
        outcome.is_ok(),
        "an empty directory has no sources, yet ignore does not analyse"
    );
}

#[test]
fn dispatch_with_ignored_lists_what_ignore_recorded() {
    // Arrange
    let fakes = FakePorts::default();
    let recorded = Command::Ignore {
        fingerprint: "cafebabe00000002".to_string(),
        reason: None,
    };
    dispatch_in(&fakes, &recorded).0.expect("ignore accepted");

    // Act
    let (outcome, text) = dispatch_in(&fakes, &Command::Ignored);

    // Assert
    assert!(outcome.is_ok());
    assert!(
        text.contains("cafebabe00000002"),
        "the listing names the fingerprint just recorded, got: {text}"
    );
}

#[test]
fn dispatch_with_stats_hands_the_summary_to_the_reporter_it_was_given() {
    // Arrange
    let fakes =
        FakePorts::with_sources(InMemorySourceTree::new().with_file("/p/src/lib.rs", DUPLICATED));
    let analyzer = RustAnalyzer::new();
    let root = root();
    let dispatcher = CommandDispatcher::new(
        &analyzer,
        &root,
        &SummaryReporter,
        CliOverrides::default(),
        fakes.ports(),
    );

    // Act
    let (_, stats) = dispatcher
        .dispatch(&Command::Stats)
        .expect("it analyses cleanly")
        .into_parts();

    // Assert
    assert_eq!(stats, "stats units=2\n");
}
