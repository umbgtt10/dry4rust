// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::{cargo_dry4rust, fixture_path};
use predicate::str;
use predicates::prelude::*;
use serde_json::from_str;

#[test]
fn error_on_nonexistent_path() {
    cargo_dry4rust()
        .args(["--path", "/nonexistent/path/that/does/not/exist", "stats"])
        .assert()
        .code(2)
        .stderr(str::contains("No source files"));
}

#[test]
fn exclude_option() {
    // When all files are excluded, the tool reports no source files found
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "--exclude",
            "lib.rs",
            "stats",
        ])
        .assert()
        .code(2)
        .stderr(str::contains("No source files"));
}

#[test]
fn exclude_tests_flag_reduces_duplicates() {
    // Without --exclude-tests: 3 units in 1 group (2 production + 1 in #[cfg(test)] mod)
    let output_all = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("test_code").to_str().unwrap(),
            "--format",
            "json",
            "stats",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let all: serde_json::Value = from_str(&String::from_utf8(output_all).unwrap()).unwrap();
    assert_eq!(all["exact_duplicate_units"].as_u64().unwrap(), 3);

    // With --exclude-tests: only 2 production units remain
    let output_excl = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("test_code").to_str().unwrap(),
            "--exclude-tests",
            "--format",
            "json",
            "stats",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let excl: serde_json::Value = from_str(&String::from_utf8(output_excl).unwrap()).unwrap();
    assert_eq!(excl["exact_duplicate_units"].as_u64().unwrap(), 2);
    assert_eq!(excl["total_code_units"].as_u64().unwrap(), 2);
}

#[test]
fn exclude_tests_text_report() {
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("test_code").to_str().unwrap(),
            "--exclude-tests",
            "report",
        ])
        .assert()
        .success()
        .stdout(str::contains("Exact Duplicates"))
        .stdout(str::contains("Group 1"));
}

#[test]
fn help_works() {
    cargo_dry4rust()
        .arg("--help")
        .assert()
        .success()
        .stdout(str::contains("Detect duplicate code"));
}

#[test]
fn min_lines_option() {
    // With very high min_lines, short functions should be excluded
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "--min-lines",
            "1000",
            "stats",
        ])
        .assert()
        .success()
        .stdout(str::contains("Exact duplicates: 0 groups"));
}

#[test]
fn min_nodes_option() {
    // With very high min_nodes, nothing should be analyzed
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "--min-nodes",
            "1000",
            "stats",
        ])
        .assert()
        .success()
        .stdout(str::contains("Exact duplicates: 0 groups"));
}
