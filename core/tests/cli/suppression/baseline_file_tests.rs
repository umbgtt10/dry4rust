// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use crate::common::result_with;
use crate::support::in_memory_baseline_store::InMemoryBaselineStore;
use dry4rust::cli::suppression::baseline_constants::DEFAULT_BASELINE_FILE;
use dry4rust::cli::suppression::baseline_constants::FORMAT_VERSION;
use dry4rust::cli::suppression::baseline_file::BaselineFile;
use dry4rust::cli::suppression::baseline_kind::BaselineKind;
use dry4rust::source_path::SourcePath;

fn path(text: &str) -> SourcePath {
    SourcePath::new(String::from(text))
}

#[test]
fn baseline_path_joins_a_relative_name_to_the_analysed_root() {
    // Arrange
    let root = path("/projects/thing");

    // Act
    let path = BaselineFile::baseline_path(&root, Some(&path("ci/dry4rust-baseline.json")));

    // Assert
    assert_eq!(
        path,
        SourcePath::new(String::from("/projects/thing/ci/dry4rust-baseline.json"))
    );
}

#[test]
fn baseline_path_leaves_an_absolute_name_alone() {
    // Arrange -- a drive rather than a leading slash: `join` takes a name
    // that starts at `/` as a whole path too, so a slash would pass this test
    // without reaching the branch it names
    let named = path("C:/projects/recorded.json");

    // Act
    let path = BaselineFile::baseline_path(&path("/projects/thing"), Some(&named));

    // Assert
    assert!(named.is_absolute(), "the arrangement has to be absolute");
    assert_eq!(path, named);
}

#[test]
fn baseline_path_without_a_name_uses_the_family_default() {
    // Arrange & Act
    let path = BaselineFile::baseline_path(&path("/projects/thing"), None);

    // Assert
    assert_eq!(
        path,
        SourcePath::new(String::from("/projects/thing")).join(DEFAULT_BASELINE_FILE)
    );
    assert_eq!(DEFAULT_BASELINE_FILE, "dry4rust-baseline.json");
}

#[test]
fn load_of_a_baseline_from_a_later_format_says_to_re_record_it() {
    // Arrange
    let store = InMemoryBaselineStore::new().with_baseline(
        "/p/dry4rust-baseline.json",
        BaselineFile::new(99, Vec::new()),
    );

    // Act
    let outcome = BaselineFile::load(&store, &path("/p/dry4rust-baseline.json"));

    // Assert
    let message = outcome
        .expect_err("a format this build does not read is not one it can judge against")
        .to_string();
    assert!(message.contains("written in format 99"), "{message}");
    assert!(message.contains("cargo dry4rust baseline"), "{message}");
}

#[test]
fn load_of_a_baseline_the_store_cannot_read_reports_its_reason() {
    // Arrange
    let store = InMemoryBaselineStore::new().with_unreadable(
        "/p/dry4rust-baseline.json",
        "expected value at line 1 column 3",
    );

    // Act
    let outcome = BaselineFile::load(&store, &path("/p/dry4rust-baseline.json"));

    // Assert
    assert_eq!(
        outcome
            .expect_err("an unreadable baseline is not an empty one")
            .to_string(),
        "baseline /p/dry4rust-baseline.json: expected value at line 1 column 3"
    );
}

#[test]
fn load_of_a_baseline_the_store_does_not_hold_names_the_command_that_would_record_one() {
    // Arrange
    let store = InMemoryBaselineStore::new();

    // Act
    let outcome = BaselineFile::load(&store, &path("/p/dry4rust-baseline.json"));

    // Assert
    assert_eq!(
        outcome.expect_err("nothing is there to load").to_string(),
        "baseline /p/dry4rust-baseline.json: no such file; record one with `cargo dry4rust baseline`"
    );
}

#[test]
fn record_orders_entries_by_kind_then_fingerprint() {
    // Arrange -- out of order in both dimensions at once
    let result = result_with(
        vec![group(0x22, &["b"]), group(0x11, &["a"])],
        vec![group(0x33, &["c"])],
        vec![group(0x44, &["d"])],
        vec![group(0x55, &["e"])],
    );

    // Act
    let recorded = BaselineFile::record(&result);

    // Assert
    let order: Vec<_> = recorded
        .entries()
        .iter()
        .map(|entry| (entry.kind(), entry.fingerprint()))
        .collect();
    assert_eq!(
        order,
        vec![
            (BaselineKind::Exact, "0000000000000011"),
            (BaselineKind::Exact, "0000000000000022"),
            (BaselineKind::Near, "0000000000000033"),
            (BaselineKind::SubExact, "0000000000000044"),
            (BaselineKind::SubNear, "0000000000000055"),
        ]
    );
}

#[test]
fn save_hands_the_store_the_recording() {
    // Arrange
    let store = InMemoryBaselineStore::new();
    let recorded = BaselineFile::record(&result_with(
        vec![group(0x11, &["a", "b"])],
        vec![],
        vec![],
        vec![],
    ));

    // Act
    recorded
        .save(&store, &path("/p/dry4rust-baseline.json"))
        .expect("the store takes it");

    // Assert
    let held = store
        .held(&path("/p/dry4rust-baseline.json"))
        .expect("held where it was saved");
    assert_eq!(held.len(), 1);
    assert_eq!(held.version(), FORMAT_VERSION);
}

#[test]
fn save_through_a_store_that_fails_names_the_baseline() {
    // Arrange
    let store = InMemoryBaselineStore::failing("Not a directory (os error 20)");
    let recorded = BaselineFile::new(FORMAT_VERSION, Vec::new());

    // Act
    let outcome = recorded.save(&store, &path("/p/blocker/dry4rust-baseline.json"));

    // Assert
    assert_eq!(
        outcome.expect_err("the store refused it").to_string(),
        "baseline /p/blocker/dry4rust-baseline.json: Not a directory (os error 20)"
    );
}

#[test]
fn version_of_a_recording_is_the_format_this_build_writes() {
    // Arrange
    let recorded = BaselineFile::record(&result_with(
        vec![group(0x11, &["a", "b"])],
        vec![],
        vec![],
        vec![],
    ));

    // Act
    let version = recorded.version();

    // Assert
    assert_eq!(version, FORMAT_VERSION);
}
