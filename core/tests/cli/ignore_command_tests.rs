// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::ignore_command::IgnoreCommand;
use dry4rust::suppression::ignore_file::IgnoreFile;
use tempfile::TempDir;

#[test]
fn run_over_the_same_fingerprint_twice_records_it_once() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let mut out = Vec::new();
    IgnoreCommand::new(tmp.path(), "deadbeef12345678", None)
        .run(&mut out)
        .expect("the first is accepted");

    // Act
    IgnoreCommand::new(tmp.path(), "deadbeef12345678", None)
        .run(&mut out)
        .expect("the second is accepted");

    // Assert
    assert_eq!(IgnoreFile::load(tmp.path()).ignore.len(), 1);
}

#[test]
fn run_records_the_fingerprint_with_the_reason_it_was_given() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let mut out = Vec::new();

    // Act
    IgnoreCommand::new(
        tmp.path(),
        "deadbeef12345678",
        Some("trait impls are meant to look alike"),
    )
    .run(&mut out)
    .expect("a valid fingerprint is accepted");

    // Assert
    let ignore_file = IgnoreFile::load(tmp.path());
    assert_eq!(ignore_file.ignore.len(), 1);
    assert_eq!(ignore_file.ignore[0].fingerprint, "deadbeef12345678");
    assert_eq!(
        ignore_file.ignore[0].reason.as_deref(),
        Some("trait impls are meant to look alike")
    );
}

#[test]
fn run_with_a_malformed_fingerprint_is_rejected() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let mut out = Vec::new();

    // Act
    let outcome = IgnoreCommand::new(tmp.path(), "not-a-fingerprint", None).run(&mut out);

    // Assert
    assert!(matches!(outcome, Err(CliError::InvalidFingerprint(_))));
    assert!(
        !tmp.path().join(".dry4rust-ignore.toml").exists(),
        "a rejected fingerprint leaves no file behind"
    );
}
