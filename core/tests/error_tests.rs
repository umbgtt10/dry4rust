// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// The crate's error: one variant per way the library fails, each saying what a
// reader of a warning or a terminal needs.

use dry4rust::error::Error;
use std::path::PathBuf;

#[test]
fn to_string_of_a_parse_failure_names_the_file_and_why() {
    // Arrange
    let failure = Error::Parse {
        path: PathBuf::from("src/a.rs"),
        reason: String::from("expected `;`"),
    };

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "Failed to parse src/a.rs: expected `;`");
}

#[test]
fn to_string_of_a_read_failure_names_the_file_and_why() {
    // Arrange
    let failure = Error::Read {
        path: PathBuf::from("src/a.rs"),
        reason: String::from("denied"),
    };

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "Failed to read src/a.rs: denied");
}
