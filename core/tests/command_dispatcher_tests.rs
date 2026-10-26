// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::source_path;
use crate::support::in_memory_source_tree::InMemorySourceTree;
use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::cli::command::Command;
use dry4rust::cli::output::json_reporter::JsonReporter;
use dry4rust::cli::output::text_reporter::TextReporter;
use dry4rust::command_dispatcher::CommandDispatcher;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use dry4rust::source_path::SourcePath;
use serde_json::Value;
use serde_json::from_str;
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

fn dispatch_in(
    root: &SourcePath,
    sources: &InMemorySourceTree,
    command: &Command,
) -> (Result<(), CliError>, String) {
    let analyzer = RustAnalyzer::new();
    let reporter = TextReporter::new(Some(root.clone()));
    let dispatcher =
        CommandDispatcher::new(&analyzer, root, &reporter, CliOverrides::default(), sources);
    match dispatcher.dispatch(command) {
        Ok(outcome) => outcome.into_parts(),
        Err(error) => (Err(error), String::new()),
    }
}

#[test]
fn dispatch_over_sources_that_cannot_be_listed_names_the_root() {
    // Arrange
    let root = SourcePath::new(String::from("/p"));
    let sources = InMemorySourceTree::unlistable("/p/src/\u{FFFD}.rs: the path is not valid UTF-8");

    // Act
    let (outcome, _) = dispatch_in(&root, &sources, &Command::Stats);

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
fn dispatch_with_ignore_records_the_fingerprint_without_analysing() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let command = Command::Ignore {
        fingerprint: "cafebabe00000001".to_string(),
        reason: Some("a reason".to_string()),
    };

    // Act
    let (outcome, _) = dispatch_in(
        &source_path(tmp.path()),
        &InMemorySourceTree::new(),
        &command,
    );

    // Assert
    assert!(
        outcome.is_ok(),
        "an empty directory has no sources, yet ignore does not analyse"
    );
}

#[test]
fn dispatch_with_ignored_lists_what_ignore_recorded() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let recorded = Command::Ignore {
        fingerprint: "cafebabe00000002".to_string(),
        reason: None,
    };
    dispatch_in(
        &source_path(tmp.path()),
        &InMemorySourceTree::new(),
        &recorded,
    )
    .0
    .expect("ignore accepted");

    // Act
    let (outcome, text) = dispatch_in(
        &source_path(tmp.path()),
        &InMemorySourceTree::new(),
        &Command::Ignored,
    );

    // Assert
    assert!(outcome.is_ok());
    assert!(
        text.contains("cafebabe00000002"),
        "the listing names the fingerprint just recorded, got: {text}"
    );
}

#[test]
fn dispatch_with_stats_writes_through_the_reporter_it_was_given() {
    // Arrange
    let root = SourcePath::new(String::from("/p"));
    let sources = InMemorySourceTree::new().with_file("/p/src/lib.rs", DUPLICATED);
    let analyzer = RustAnalyzer::new();
    let reporter = JsonReporter::new(Some(root.clone()));
    let dispatcher = CommandDispatcher::new(
        &analyzer,
        &root,
        &reporter,
        CliOverrides::default(),
        &sources,
    );

    // Act
    let (_, stats) = dispatcher
        .dispatch(&Command::Stats)
        .expect("it analyses cleanly")
        .into_parts();

    // Assert
    assert!(from_str::<Value>(&stats).is_ok(), "{stats}");
}
