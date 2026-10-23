// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use crate::common::result_with;
use dry4rust::cli::baseline_command::BaselineCommand;
use dry4rust::cli::cli_error::CliError;
use dry4rust::config::Config;
use dry4rust::suppression::baseline_file::BaselineFile;
use std::fs;
use tempfile::TempDir;

fn recorded(dry_run: bool) -> (TempDir, String) {
    let tmp = TempDir::new().expect("temp dir");
    let result = result_with(
        vec![group(0x11, &["process_data", "compute_total"])],
        vec![],
        vec![],
        vec![],
    );
    let config = Config::default().with_root(tmp.path().to_path_buf());

    let text = BaselineCommand::new(tmp.path(), &config, &result, dry_run)
        .run()
        .expect("recording succeeds")
        .text()
        .to_string();

    (tmp, text)
}

#[test]
fn run_in_dry_run_names_what_it_would_record_without_writing_it() {
    // Arrange & Act
    let (tmp, text) = recorded(true);

    // Assert
    assert!(text.contains("1 groups would be recorded"), "{text}");
    assert!(text.contains("process_data"), "{text}");
    assert!(!tmp.path().join("dry4rust-baseline.json").exists());
}

#[test]
fn run_outside_dry_run_writes_a_baseline_that_loads_back() {
    // Arrange & Act
    let (tmp, text) = recorded(false);

    // Assert
    assert!(text.contains("Recorded 1 groups"), "{text}");
    let written = BaselineFile::load(&tmp.path().join("dry4rust-baseline.json"))
        .expect("what was written is what this build reads");
    assert_eq!(written.len(), 1);
}

#[test]
fn run_over_a_clean_result_records_nothing_and_says_so() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let result = result_with(vec![], vec![], vec![], vec![]);
    let config = Config::default().with_root(tmp.path().to_path_buf());

    // Act
    let text = BaselineCommand::new(tmp.path(), &config, &result, false)
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
    let tmp = TempDir::new().expect("temp dir");
    fs::write(tmp.path().join("blocker"), "a file, not a directory").expect("blocker");
    let result = result_with(
        vec![group(0x11, &["process_data", "compute_total"])],
        vec![],
        vec![],
        vec![],
    );
    let config = Config::default()
        .with_root(tmp.path().to_path_buf())
        .with_baseline(Some(tmp.path().join("blocker").join("baseline.json")));

    // Act
    let outcome = BaselineCommand::new(tmp.path(), &config, &result, false)
        .run()
        .expect("the entries are produced before the write is attempted");

    // Assert
    assert_eq!(
        outcome.text(),
        "  exact 0000000000000011 (2 members: process_data, compute_total)\n"
    );
    assert!(matches!(outcome.result(), Err(CliError::Analysis(_))));
}
