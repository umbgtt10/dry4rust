// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::ignore_command::IgnoreCommand;
use dry4rust::cli::ignored_command::IgnoredCommand;
use tempfile::TempDir;

#[test]
fn run_after_ignore_lists_what_was_added() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let mut added = Vec::new();
    let mut listed = Vec::new();

    // Act
    IgnoreCommand::new(tmp.path(), "deadbeef12345678", Some("known duplicate"))
        .run(&mut added)
        .expect("a valid fingerprint is accepted");
    IgnoredCommand::new(tmp.path())
        .run(&mut listed)
        .expect("listing succeeds");

    // Assert
    let listed = String::from_utf8(listed).expect("utf-8");
    assert!(listed.contains("Ignored fingerprints:"), "{listed}");
    assert!(listed.contains("deadbeef12345678"), "{listed}");
    assert!(listed.contains("known duplicate"), "{listed}");
}

#[test]
fn run_without_an_ignore_file_says_there_is_nothing_ignored() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let mut listed = Vec::new();

    // Act
    IgnoredCommand::new(tmp.path())
        .run(&mut listed)
        .expect("listing succeeds");

    // Assert
    let listed = String::from_utf8(listed).expect("utf-8");
    assert_eq!(listed, "No ignored fingerprints.\n");
}
