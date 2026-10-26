// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::command_outcome::CommandOutcome;

#[test]
fn into_parts_hands_over_the_result_and_the_text() {
    // Arrange
    let outcome = CommandOutcome::new(String::from("body"), Err(CliError::CheckFailed));

    // Act
    let (result, text) = outcome.into_parts();

    // Assert
    assert!(matches!(result, Err(CliError::CheckFailed)));
    assert_eq!(text, "body");
}

#[test]
fn new_pairs_the_text_with_the_result_it_was_given() {
    // Arrange & Act
    let outcome = CommandOutcome::new(String::from("body"), Err(CliError::CheckFailed));

    // Assert
    assert_eq!(outcome.text(), "body");
    assert!(matches!(outcome.result(), Err(CliError::CheckFailed)));
}

#[test]
fn succeeded_carries_the_text_with_an_ok_result() {
    // Arrange & Act
    let outcome = CommandOutcome::succeeded(String::from("done"));

    // Assert
    assert_eq!(outcome.text(), "done");
    assert!(outcome.result().is_ok());
}

#[test]
fn with_warnings_carries_them_beside_the_text_and_the_verdict() {
    // Arrange
    let outcome = CommandOutcome::succeeded(String::from("report\n"));

    // Act
    let outcome = outcome.with_warnings(vec![String::from("Failed to parse src/a.rs: x")]);

    // Assert
    assert_eq!(outcome.warnings(), ["Failed to parse src/a.rs: x"]);
    assert_eq!(outcome.text(), "report\n");
    assert!(outcome.result().is_ok());
}
