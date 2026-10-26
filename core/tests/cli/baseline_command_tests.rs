// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use crate::common::result_with;
use crate::support::in_memory_baseline_store::InMemoryBaselineStore;
use dry4rust::cli::baseline_command::BaselineCommand;
use dry4rust::cli::cli_error::CliError;
use dry4rust::config::Config;
use dry4rust::source_path::SourcePath;

fn path(text: &str) -> SourcePath {
    SourcePath::new(String::from(text))
}

fn recorded(dry_run: bool) -> (InMemoryBaselineStore, String) {
    let store = InMemoryBaselineStore::new();
    let root = path("/p");
    let result = result_with(
        vec![group(0x11, &["process_data", "compute_total"])],
        vec![],
        vec![],
        vec![],
    );
    let config = Config::default().with_root(root.clone());

    let text = BaselineCommand::new(&store, &root, &config, &result, dry_run)
        .run()
        .expect("recording succeeds")
        .text()
        .to_string();

    (store, text)
}

#[test]
fn run_in_dry_run_leaves_the_store_untouched() {
    // Arrange & Act
    let (store, text) = recorded(true);

    // Assert
    assert!(
        text.contains("1 groups would be recorded in /p/dry4rust-baseline.json."),
        "{text}"
    );
    assert!(!store.holds(&path("/p/dry4rust-baseline.json")));
}

#[test]
fn run_names_the_baseline_below_a_native_root_as_it_was_given() {
    // Arrange
    let store = InMemoryBaselineStore::new();
    let root = SourcePath::native(String::from("C:\\work"), '\\');
    let result = result_with(vec![], vec![], vec![], vec![]);
    let config = Config::default().with_root(root.clone());

    // Act
    let text = BaselineCommand::new(&store, &root, &config, &result, false)
        .run()
        .expect("recording succeeds")
        .text()
        .to_string();

    // Assert
    assert_eq!(
        text,
        "Recorded 0 groups in C:\\work\\dry4rust-baseline.json.\n"
    );
}

#[test]
fn run_outside_dry_run_hands_the_store_the_recording() {
    // Arrange & Act
    let (store, text) = recorded(false);

    // Assert
    assert!(
        text.contains("Recorded 1 groups in /p/dry4rust-baseline.json."),
        "{text}"
    );
    let written = store
        .held(&path("/p/dry4rust-baseline.json"))
        .expect("the recording reached the store");
    assert_eq!(written.len(), 1);
    assert_eq!(
        written.entries()[0].names(),
        ["process_data", "compute_total"]
    );
}

#[test]
fn run_over_a_clean_result_records_nothing_and_says_so() {
    // Arrange
    let store = InMemoryBaselineStore::new();
    let root = path("/p");
    let result = result_with(vec![], vec![], vec![], vec![]);
    let config = Config::default().with_root(root.clone());

    // Act
    let text = BaselineCommand::new(&store, &root, &config, &result, false)
        .run()
        .expect("recording succeeds")
        .text()
        .to_string();

    // Assert
    assert!(
        text.contains("Recorded 0 groups"),
        "a clean codebase has nothing to inherit"
    );
}

// The entries are listed before the file is written, so a write that fails
// still leaves them on stdout ahead of the error -- as it did while they were
// streamed to a writer rather than returned.
#[test]
fn run_with_an_unwritable_baseline_keeps_the_entries_it_listed_and_fails() {
    // Arrange
    let store = InMemoryBaselineStore::failing("Not a directory (os error 20)");
    let root = path("/p");
    let result = result_with(
        vec![group(0x11, &["process_data", "compute_total"])],
        vec![],
        vec![],
        vec![],
    );
    let config = Config::default()
        .with_root(root.clone())
        .with_baseline(Some(path("/p/blocker/baseline.json")));

    // Act
    let outcome = BaselineCommand::new(&store, &root, &config, &result, false)
        .run()
        .expect("the entries are produced before the write is attempted");

    // Assert
    assert_eq!(
        outcome.text(),
        "  exact 0000000000000011 (2 members: process_data, compute_total)\n"
    );
    assert!(matches!(outcome.result(), Err(CliError::Analysis(_))));
}
