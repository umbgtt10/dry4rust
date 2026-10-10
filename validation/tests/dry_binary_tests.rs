// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use predicate::str;
use predicates::prelude::*;
use tempfile::TempDir;
use validation::dry_binary::DryBinary;
use validation::fixture::Fixture;

#[test]
fn command_finds_the_built_binary_beside_the_test_binary() {
    // Arrange & Act & Assert
    DryBinary::command()
        .arg("--help")
        .assert()
        .success()
        .stdout(str::contains("Usage: cargo-dry4rust"));
}

#[test]
fn command_in_a_fixture_analyses_that_fixture() {
    // Arrange & Act & Assert
    DryBinary::command_in(&Fixture::path("exact_dupes"))
        .arg("stats")
        .assert()
        .success()
        .stdout(str::contains("Exact duplicates: 1 groups (3 code units)"));
}

#[test]
fn command_in_a_root_without_sources_names_that_root() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");

    // Act & Assert
    DryBinary::command_in(tmp.path())
        .arg("stats")
        .assert()
        .code(2)
        .stderr(str::contains(format!(
            "No source files found in {}",
            tmp.path().display()
        )));
}
