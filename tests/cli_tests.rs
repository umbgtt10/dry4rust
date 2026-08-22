// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// The command line, exercised two ways.
//
// Spawning the binary proves the whole path a user takes, and is the only way
// to see what actually reaches stdout. Calling the functions directly is what
// keeps them from being public and unexercised -- a placeholder is exactly the
// kind of code that acquires a signature nobody ever calls.

use assert_cmd::Command;
use dry4rust::cli::Cli;
use predicates::str::contains;
use std::process::ExitCode;

#[test]
fn binary_runs_prints_placeholder_message_and_exits_success() {
    // Arrange
    let mut command = Command::cargo_bin("cargo-dry4rust").expect("binary should build");

    // Act
    let assert = command.assert();

    // Assert
    assert.success().stdout(contains("not yet implemented"));
}

#[test]
fn run_from_args_ignores_whatever_it_is_given() {
    // Arrange
    let arguments = vec![
        String::from("cargo-dry4rust"),
        String::from("--not-a-real-flag"),
        String::from("neither-is-this"),
    ];

    // Act
    let code = Cli::run_from_args(arguments).expect("unknown arguments are ignored, not rejected");

    // Assert
    assert_eq!(code, ExitCode::SUCCESS);
}

// Called rather than spawned, so the signature the real implementation will
// need is under test before there is an implementation behind it.
#[test]
fn run_from_args_returns_success() {
    // Arrange
    let arguments = vec![String::from("cargo-dry4rust")];

    // Act
    let code = Cli::run_from_args(arguments).expect("the placeholder should succeed");

    // Assert
    assert_eq!(code, ExitCode::SUCCESS);
}

// Reads the real process arguments, which under a test harness are the
// harness's own -- and are ignored just the same.
#[test]
fn run_returns_success() {
    // Arrange & Act
    let code = Cli::run().expect("the placeholder should succeed");

    // Assert
    assert_eq!(code, ExitCode::SUCCESS);
}
