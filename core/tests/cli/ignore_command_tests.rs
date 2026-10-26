// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_ignore_store::InMemoryIgnoreStore;
use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::ignore_command::IgnoreCommand;
use dry4rust::cli::suppression::ignore_file::IgnoreFile;
use dry4rust::source_path::SourcePath;

fn root() -> SourcePath {
    SourcePath::new(String::from("/p"))
}

#[test]
fn run_over_the_same_fingerprint_twice_records_it_once() {
    // Arrange
    let store = InMemoryIgnoreStore::new();
    IgnoreCommand::new(&store, &root(), "deadbeef12345678", None)
        .run()
        .expect("the first is accepted");

    // Act
    IgnoreCommand::new(&store, &root(), "deadbeef12345678", None)
        .run()
        .expect("the second is accepted");

    // Assert
    assert_eq!(IgnoreFile::load(&store, &root()).ignore().len(), 1);
}

#[test]
fn run_records_the_fingerprint_with_the_reason_it_was_given() {
    // Arrange
    let store = InMemoryIgnoreStore::new();

    // Act
    IgnoreCommand::new(
        &store,
        &root(),
        "deadbeef12345678",
        Some("trait impls are meant to look alike"),
    )
    .run()
    .expect("a valid fingerprint is accepted");

    // Assert
    let ignore_file = IgnoreFile::load(&store, &root());
    assert_eq!(ignore_file.ignore().len(), 1);
    assert_eq!(ignore_file.ignore()[0].fingerprint, "deadbeef12345678");
    assert_eq!(
        ignore_file.ignore()[0].reason.as_deref(),
        Some("trait impls are meant to look alike")
    );
}

#[test]
fn run_with_a_malformed_fingerprint_saves_nothing() {
    // Arrange
    let store = InMemoryIgnoreStore::new();

    // Act
    let outcome = IgnoreCommand::new(&store, &root(), "not-a-fingerprint", None).run();

    // Assert
    assert!(matches!(outcome, Err(CliError::InvalidFingerprint(_))));
    assert!(!store.holds(&IgnoreFile::path_in(&root())));
}

#[test]
fn run_with_a_store_that_cannot_write_fails_with_its_reason() {
    // Arrange
    let store = InMemoryIgnoreStore::failing("Permission denied (os error 13)");

    // Act
    let outcome = IgnoreCommand::new(&store, &root(), "deadbeef12345678", None).run();

    // Assert
    let Err(error) = outcome else {
        panic!("a write the store refused is not a recorded fingerprint");
    };
    assert!(matches!(error, CliError::Io(_)));
    assert_eq!(error.to_string(), "Permission denied (os error 13)");
}
