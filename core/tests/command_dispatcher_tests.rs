// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::cli::command::Command;
use dry4rust::cli::output::json_reporter::JsonReporter;
use dry4rust::cli::output::text_reporter::TextReporter;
use dry4rust::command_dispatcher::CommandDispatcher;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use serde_json::Value;
use serde_json::from_str;
use std::fs;
use std::path::Path;
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

fn dispatch_in(root: &Path, command: &Command) -> (Result<(), CliError>, String) {
    let analyzer = RustAnalyzer::new();
    let reporter = TextReporter::new(Some(root.to_path_buf()));
    let dispatcher = CommandDispatcher::new(&analyzer, root, &reporter, CliOverrides::default());
    match dispatcher.dispatch(command) {
        Ok(outcome) => outcome.into_parts(),
        Err(error) => (Err(error), String::new()),
    }
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
    let (outcome, _) = dispatch_in(tmp.path(), &command);

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
    dispatch_in(tmp.path(), &recorded)
        .0
        .expect("ignore accepted");

    // Act
    let (outcome, text) = dispatch_in(tmp.path(), &Command::Ignored);

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
    let tmp = TempDir::new().expect("temp dir");
    fs::create_dir_all(tmp.path().join("src")).expect("src");
    fs::write(tmp.path().join("src/lib.rs"), DUPLICATED).expect("write");
    let analyzer = RustAnalyzer::new();
    let reporter = JsonReporter::new(Some(tmp.path().to_path_buf()));
    let dispatcher =
        CommandDispatcher::new(&analyzer, tmp.path(), &reporter, CliOverrides::default());

    // Act
    let (_, stats) = dispatcher
        .dispatch(&Command::Stats)
        .expect("it analyses cleanly")
        .into_parts();

    // Assert
    assert!(from_str::<Value>(&stats).is_ok(), "{stats}");
}
