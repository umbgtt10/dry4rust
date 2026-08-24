// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::result_with;
use dry4rust::cli::cleanup_command::CleanupCommand;
use dry4rust::cli::ignore_command::IgnoreCommand;
use dry4rust::cli::output_format::OutputFormat;
use dry4rust::suppression::ignore_file::IgnoreFile;
use std::fs;
use tempfile::TempDir;

fn cleaned(dry_run: bool) -> (TempDir, String) {
    let tmp = TempDir::new().expect("temp dir");
    let mut added = Vec::new();
    IgnoreCommand::new(tmp.path(), "deadbeef12345678", None)
        .run(&mut added)
        .expect("ignore accepted");
    let result = result_with(vec![], vec![], vec![], vec![]);
    let mut out = Vec::new();

    CleanupCommand::new(tmp.path(), &result, dry_run)
        .run(&mut out)
        .expect("cleanup succeeds");

    (tmp, String::from_utf8(out).expect("utf-8"))
}

#[test]
fn run_in_dry_run_names_the_stale_entry_and_leaves_the_file_alone() {
    // Arrange & Act
    let (tmp, text) = cleaned(true);

    // Assert
    assert!(text.contains("deadbeef12345678"), "{text}");
    assert!(text.contains("would be removed"), "{text}");
    let kept = fs::read_to_string(IgnoreFile::path_in(tmp.path())).expect("read back");
    assert!(kept.contains("deadbeef12345678"), "{kept}");
}

#[test]
fn run_outside_dry_run_prunes_the_entry_that_matches_nothing() {
    // Arrange & Act
    let (tmp, text) = cleaned(false);

    // Assert
    assert!(text.contains("Removed 1 stale entries"), "{text}");
    let pruned = fs::read_to_string(IgnoreFile::path_in(tmp.path())).expect("read back");
    assert!(!pruned.contains("deadbeef12345678"), "{pruned}");
}

#[test]
fn run_with_nothing_stale_says_so_and_writes_no_file() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let result = result_with(vec![], vec![], vec![], vec![]);
    let mut out = Vec::new();
    let _ = OutputFormat::Text;

    // Act
    CleanupCommand::new(tmp.path(), &result, false)
        .run(&mut out)
        .expect("cleanup succeeds");

    // Assert
    assert_eq!(
        String::from_utf8(out).expect("utf-8"),
        "No stale entries found.\n"
    );
    assert!(!IgnoreFile::path_in(tmp.path()).exists());
}
