// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::source_path;
use dry4rust::cli::ignore_command::IgnoreCommand;
use dry4rust::cli::ignored_command::IgnoredCommand;
use tempfile::TempDir;

#[test]
fn run_after_ignore_lists_what_was_added() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");

    // Act
    IgnoreCommand::new(
        &source_path(tmp.path()),
        "deadbeef12345678",
        Some("known duplicate"),
    )
    .run()
    .expect("a valid fingerprint is accepted");
    let listed = IgnoredCommand::new(&source_path(tmp.path()))
        .run()
        .expect("listing succeeds")
        .text()
        .to_string();

    // Assert
    assert!(listed.contains("Ignored fingerprints:"), "{listed}");
    assert!(listed.contains("deadbeef12345678"), "{listed}");
    assert!(listed.contains("known duplicate"), "{listed}");
}

#[test]
fn run_without_an_ignore_file_says_there_is_nothing_ignored() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");

    // Act
    let listed = IgnoredCommand::new(&source_path(tmp.path()))
        .run()
        .expect("listing succeeds")
        .text()
        .to_string();

    // Assert
    assert_eq!(listed, "No ignored fingerprints.\n");
}
