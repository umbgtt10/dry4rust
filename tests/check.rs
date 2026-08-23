// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::{cargo_dry4rust, fixture_path};
use predicate::str;
use predicates::prelude::*;

#[test]
fn check_absolute_passes_percentage_fails() {
    // Arrange & Act & Assert
    // Absolute threshold is generous (passes), but percentage is strict (fails)
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "check",
            "--max-exact",
            "100",
            "--max-exact-percent",
            "0.0",
        ])
        .assert()
        .code(1)
        .stdout(str::contains("Check FAILED"));
}

#[test]
fn check_fails_with_duplicates() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "check",
            "--max-exact",
            "0",
        ])
        .assert()
        .code(1)
        .stdout(str::contains("Check FAILED"));
}

#[test]
fn check_fails_with_percentage_threshold_exceeded() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "check",
            "--max-exact",
            "100",
            "--max-exact-percent",
            "0.0",
        ])
        .assert()
        .code(1)
        .stdout(str::contains("Check FAILED"))
        .stdout(str::contains("exact duplicate lines"));
}

#[test]
fn check_no_dupes_passes() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("no_dupes").to_str().unwrap(),
            "check",
            "--max-exact",
            "0",
        ])
        .assert()
        .success()
        .stdout(str::contains("Check passed"));
}

#[test]
fn check_no_thresholds_passes_with_duplicates() {
    // Arrange & Act & Assert
    // With no thresholds set, check should pass even when duplicates exist
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "check",
        ])
        .assert()
        .success()
        .stdout(str::contains("Check passed"));
}

#[test]
fn check_passes_with_generous_percentage_threshold() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "check",
            "--max-exact",
            "100",
            "--max-exact-percent",
            "100.0",
        ])
        .assert()
        .success()
        .stdout(str::contains("Check passed"));
}

#[test]
fn check_passes_with_high_threshold() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "check",
            "--max-exact",
            "100",
        ])
        .assert()
        .success()
        .stdout(str::contains("Check passed"));
}
