// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::analysed;
use crate::common::group;
use dry4rust::baseline::baseline_file::BaselineFile;
use dry4rust::baseline::baseline_file::DEFAULT_BASELINE_FILE;
use dry4rust::baseline::baseline_file::FORMAT_VERSION;
use dry4rust::baseline::baseline_file::baseline_path;
use dry4rust::baseline::baseline_kind::BaselineKind;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn admits_a_group_some_entry_recorded() {
    // Arrange
    let (_, result) = analysed("exact_dupes");
    let recorded = BaselineFile::record(&result);
    let already_there = result.exact_groups[0].clone();

    // Act
    let admitted = recorded.admits(BaselineKind::Exact, &already_there);

    // Assert
    assert!(admitted);
}

#[test]
fn admits_nothing_a_group_it_never_saw() {
    // Arrange
    let (_, result) = analysed("exact_dupes");
    let recorded = BaselineFile::record(&result);

    // Act
    let admitted = recorded.admits(BaselineKind::Exact, &group(0xdead_beef, &["new", "copy"]));

    // Assert
    assert!(!admitted);
}

#[test]
fn baseline_path_joins_a_relative_name_to_the_analysed_root() {
    // Arrange
    let root = Path::new("/projects/thing");

    // Act
    let path = baseline_path(root, Some(Path::new("ci/dry4rust-baseline.json")));

    // Assert
    assert_eq!(
        path,
        PathBuf::from("/projects/thing/ci/dry4rust-baseline.json")
    );
}

#[test]
fn baseline_path_leaves_an_absolute_name_alone() {
    // Arrange
    let named = PathBuf::from("/var/ci/recorded.json");

    // Act
    let path = baseline_path(Path::new("/projects/thing"), Some(&named));

    // Assert
    assert_eq!(path, named);
}

#[test]
fn baseline_path_without_a_name_uses_the_family_default() {
    // Arrange & Act
    let path = baseline_path(Path::new("/projects/thing"), None);

    // Assert
    assert_eq!(
        path,
        PathBuf::from("/projects/thing").join(DEFAULT_BASELINE_FILE)
    );
    assert_eq!(DEFAULT_BASELINE_FILE, "dry4rust-baseline.json");
}

#[test]
fn is_empty_reports_a_recording_of_a_clean_codebase() {
    // Arrange
    let (_, result) = analysed("no_dupes");

    // Act
    let recorded = BaselineFile::record(&result);

    // Assert
    assert!(recorded.is_empty());
    assert_eq!(recorded.len(), 0);
}

#[test]
fn load_of_a_baseline_from_a_later_format_says_to_re_record_it() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let path = tmp.path().join("dry4rust-baseline.json");
    fs::write(&path, "{\"version\": 99, \"entries\": []}").expect("write");

    // Act
    let outcome = BaselineFile::load(&path);

    // Assert
    let message = outcome
        .expect_err("a format this build does not read is not one it can judge against")
        .to_string();
    assert!(message.contains("written in format 99"), "{message}");
    assert!(message.contains("cargo dry4rust baseline"), "{message}");
}

#[test]
fn load_of_a_malformed_baseline_is_an_error_rather_than_an_empty_one() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let path = tmp.path().join("dry4rust-baseline.json");
    fs::write(&path, "{ this is not json").expect("write");

    // Act
    let outcome = BaselineFile::load(&path);

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
    let outcome = BaselineFile::load(&path);

    // Assert
    let message = outcome.expect_err("nothing is there to load").to_string();
    assert!(message.contains("no such file"), "{message}");
    assert!(message.contains("cargo dry4rust baseline"), "{message}");
}

#[test]
fn record_orders_entries_so_an_unchanged_codebase_writes_an_unchanged_file() {
    // Arrange
    let (_, result) = analysed("mixed");

    // Act
    let first = BaselineFile::record(&result);
    let second = BaselineFile::record(&result);

    // Assert
    assert_eq!(
        first.entries, second.entries,
        "a baseline is committed, so a re-record of the same tree has to diff \
         as nothing at all"
    );
    let fingerprints: Vec<_> = first.entries.iter().map(|e| &e.fingerprint).collect();
    let mut sorted = fingerprints.clone();
    sorted.sort();
    assert_eq!(fingerprints, sorted);
}

#[test]
fn record_takes_every_group_the_result_holds() {
    // Arrange
    let (_, result) = analysed("exact_dupes");

    // Act
    let recorded = BaselineFile::record(&result);

    // Assert
    assert_eq!(recorded.version, FORMAT_VERSION);
    assert_eq!(recorded.len(), result.exact_groups.len());
    assert_eq!(recorded.entries[0].kind, BaselineKind::Exact);
}

#[test]
fn save_then_load_returns_what_was_recorded() {
    // Arrange
    let (_, result) = analysed("exact_dupes");
    let recorded = BaselineFile::record(&result);
    let tmp = TempDir::new().expect("temp dir");
    let path = tmp.path().join("nested").join("dry4rust-baseline.json");

    // Act
    recorded.save(&path).expect("the baseline is written");
    let read_back = BaselineFile::load(&path).expect("and read back");

    // Assert
    assert_eq!(read_back.version, recorded.version);
    assert_eq!(read_back.entries, recorded.entries);
}
