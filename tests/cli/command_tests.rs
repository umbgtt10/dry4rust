// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::helpers::{cargo_dry4rust, fixture_path};
use predicate::str;
use predicates::prelude::*;

#[test]
fn default_command_is_report() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args(["--path", fixture_path("exact_dupes").to_str().unwrap()])
        .assert()
        .success()
        .stdout(str::contains("Duplication Statistics"))
        .stdout(str::contains("Exact Duplicates"));
}

#[test]
fn help_flag_lists_every_subcommand_the_enum_declares() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .arg("--help")
        .assert()
        .success()
        .stdout(str::contains("stats"))
        .stdout(str::contains("report"))
        .stdout(str::contains("check"))
        .stdout(str::contains("ignore"))
        .stdout(str::contains("ignored"))
        .stdout(str::contains("cleanup"));
}

#[test]
fn help_flag_prints_usage_and_exits_success() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .arg("--help")
        .assert()
        .success()
        .stdout(str::contains("Detect duplicate code"));
}
