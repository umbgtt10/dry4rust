// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn binary_runs_prints_placeholder_message_and_exits_success() {
    // Arrange
    let mut command = Command::cargo_bin("cargo-dry4rust").expect("binary should build");

    // Act
    let assert = command.assert();

    // Assert
    assert
        .success()
        .stdout(predicate::str::contains("not yet implemented"));
}
