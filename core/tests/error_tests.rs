// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// The crate's error: one variant per way the library fails, each saying what a
// reader of a warning or a terminal needs.

use dry4rust::error::Error;
use dry4rust::source_path::SourcePath;

#[test]
fn to_string_of_a_baseline_failure_names_a_native_file_as_given() {
    // Arrange
    let failure = Error::Baseline {
        path: SourcePath::native(String::from("C:\\work\\dry4rust-baseline.json"), '\\'),
        problem: String::from("denied"),
    };

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "baseline C:\\work\\dry4rust-baseline.json: denied");
}

#[test]
fn to_string_of_a_parse_failure_names_the_file_and_why() {
    // Arrange
    let failure = Error::Parse {
        path: SourcePath::new(String::from("src/a.rs")),
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
        path: SourcePath::new(String::from("src/a.rs")),
        reason: String::from("denied"),
    };

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "Failed to read src/a.rs: denied");
}
