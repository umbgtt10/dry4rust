// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use crate::common::result_with;
use crate::common::source_path;
use dry4rust::cli::baseline_command::BaselineCommand;
use dry4rust::cli::suppression::baseline_constants::FORMAT_VERSION;
use dry4rust::cli::suppression::baseline_file::BaselineFile;
use dry4rust::config::Config;
use dry4rust::traits::baseline_store::BaselineStore;
use dry4rust_cli::system::json_baseline_store::JsonBaselineStore;
use std::fs;
use tempfile::TempDir;

fn recorded(dry_run: bool) -> (TempDir, String) {
    let tmp = TempDir::new().expect("temp dir");
    let root = source_path(tmp.path());
    let result = result_with(
        vec![group(0x11, &["process_data", "compute_total"])],
        vec![],
        vec![],
        vec![],
    );
    let config = Config::default().with_root(root.clone());

    let text = BaselineCommand::new(&JsonBaselineStore::new(), &root, &config, &result, dry_run)
        .run()
        .expect("recording succeeds")
        .text()
        .to_string();

    (tmp, text)
}

#[test]
fn load_of_a_directory_reports_what_the_filesystem_said_rather_than_absence() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");

    // Act
    let outcome = BaselineFile::load(&JsonBaselineStore::new(), &source_path(tmp.path()));

    // Assert
    let message = outcome
        .expect_err("a directory is not a baseline")
        .to_string();
    assert!(
        !message.contains("no such file"),
        "a baseline that is there and cannot be read is a different problem \
         from one that was never recorded, got: {message}"
    );
    assert!(message.contains("baseline"), "{message}");
}

#[test]
fn load_of_a_malformed_baseline_is_an_error_rather_than_an_empty_one() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let path = tmp.path().join("dry4rust-baseline.json");
    fs::write(&path, "{ this is not json").expect("write");

    // Act
    let outcome = BaselineFile::load(&JsonBaselineStore::new(), &source_path(&path));

    // Assert
    assert!(
        outcome.is_err(),
        "reading a broken baseline as empty would turn every inherited finding \
         into a new one, all at once and without saying so"
    );
}

#[test]
fn load_of_a_missing_baseline_names_the_command_that_would_record_one() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let path = tmp.path().join("dry4rust-baseline.json");

    // Act
    let outcome = BaselineFile::load(&JsonBaselineStore::new(), &source_path(&path));

    // Assert
    let message = outcome.expect_err("nothing is there to load").to_string();
    assert!(message.contains("no such file"), "{message}");
    assert!(message.contains("cargo dry4rust baseline"), "{message}");
}

#[test]
fn load_of_what_save_wrote_returns_the_same_recording() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let file = source_path(&tmp.path().join("dry4rust-baseline.json"));
    let recorded = BaselineFile::record(&result_with(
        vec![group(0x11, &["a", "b"])],
        vec![group(0x22, &["c", "d"])],
        vec![],
        vec![],
    ));
    JsonBaselineStore::new()
        .save(&file, &recorded)
        .expect("written");

    // Act
    let loaded = JsonBaselineStore::new().load(&file).expect("read back");

    // Assert
    assert_eq!(loaded.version(), FORMAT_VERSION);
    assert_eq!(loaded.entries(), recorded.entries());
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
    let written = BaselineFile::load(
        &JsonBaselineStore::new(),
        &source_path(&tmp.path().join("dry4rust-baseline.json")),
    )
    .expect("what was written is what this build reads");
    assert_eq!(written.len(), 1);
}

#[test]
fn save_creates_the_directory_above_and_ends_the_document_with_a_newline() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let file = tmp.path().join("ci").join("dry4rust-baseline.json");
    let recorded = BaselineFile::record(&result_with(
        vec![group(0x11, &["a", "b"])],
        vec![],
        vec![],
        vec![],
    ));

    // Act
    JsonBaselineStore::new()
        .save(&source_path(&file), &recorded)
        .expect("the directory is made and the file written");

    // Assert
    let written = fs::read_to_string(&file).expect("read back");
    assert_eq!(
        written,
        "{\n  \"version\": 1,\n  \"entries\": [\n    {\n      \"kind\": \"exact\",\n      \"fingerprint\": \"0000000000000011\",\n      \"members\": 2,\n      \"names\": [\n        \"a\",\n        \"b\"\n      ]\n    }\n  ]\n}\n"
    );
}

#[test]
fn save_where_the_parent_is_a_file_reports_the_failure() {
    // Arrange -- the directory `save` would create is blocked by a file
    let tmp = TempDir::new().expect("temp dir");
    let blocker = tmp.path().join("blocker");
    fs::write(&blocker, "not a directory").expect("write");
    let recorded = BaselineFile::new(FORMAT_VERSION, Vec::new());

    // Act
    let outcome = recorded.save(
        &JsonBaselineStore::new(),
        &source_path(&blocker.join("dry4rust-baseline.json")),
    );

    // Assert
    let message = outcome
        .expect_err("the directory above the baseline cannot be created")
        .to_string();
    assert!(
        message.contains("baseline"),
        "a write that failed says which file it was, got: {message}"
    );
}
