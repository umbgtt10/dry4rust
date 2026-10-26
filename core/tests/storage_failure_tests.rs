// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::storage_failure::StorageFailure;
use std::error::Error;

#[test]
fn fmt_of_a_failure_is_the_reason_it_was_given() {
    // Arrange
    let failure = StorageFailure::Failed(String::from("Permission denied (os error 13)"));

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "Permission denied (os error 13)");
}

#[test]
fn fmt_of_a_missing_file_is_the_reason_it_was_given() {
    // Arrange
    let failure = StorageFailure::Missing(String::from("No such file or directory (os error 2)"));

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "No such file or directory (os error 2)");
}

#[test]
fn source_of_a_failure_is_absent_because_the_reason_is_all_there_is() {
    // Arrange
    let failure = StorageFailure::Failed(String::from("denied"));

    // Act
    let cause = failure.source();

    // Assert
    assert!(cause.is_none());
}
