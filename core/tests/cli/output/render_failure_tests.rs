// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::output::render_failure::RenderFailure;
use std::error::Error;

#[test]
fn fmt_of_a_failure_is_the_reason_it_was_given() {
    // Arrange
    let failure = RenderFailure {
        reason: String::from("key must be a string"),
    };

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "key must be a string");
}

#[test]
fn source_of_a_failure_is_absent_because_the_reason_is_all_there_is() {
    // Arrange
    let failure = RenderFailure {
        reason: String::from("key must be a string"),
    };

    // Act
    let cause = failure.source();

    // Assert
    assert!(cause.is_none());
}
