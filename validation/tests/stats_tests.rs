// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::analysed;
use crate::common::cargo_dry4rust;
use crate::common::fixture_path;
use dry4rust::cli::output_format::OutputFormat;
use dry4rust::cli::stats_command::StatsCommand;
use predicate::str;
use predicates::prelude::*;
use serde_json::Value;
use serde_json::from_str;

#[test]
fn json_format_stats() {
    // Arrange & Act
    let output = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "--format",
            "json",
            "stats",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    let parsed: Value = from_str(&text).unwrap();

    // Assert
    assert!(parsed["total_code_units"].as_u64().unwrap() > 0);
}

#[test]
fn json_stats_includes_line_counts() {
    // Arrange & Act
    let output = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "--format",
            "json",
            "stats",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    let parsed: Value = from_str(&text).unwrap();

    // Assert
    assert!(parsed["exact_duplicate_lines"].is_u64());
    assert!(parsed["near_duplicate_lines"].is_u64());
}

#[test]
fn main_dispatches_the_stats_subcommand_to_a_successful_summary() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .arg("stats")
        .arg("--path")
        .arg(fixture_path("exact_dupes"))
        .assert()
        .success();
}

#[test]
fn run_over_a_clean_fixture_still_writes_the_summary() {
    // Arrange
    let (_, result) = analysed("no_dupes");
    let reporter = OutputFormat::Text.reporter(None);
    let mut out = Vec::new();

    // Act
    StatsCommand::new(&result, reporter.as_ref())
        .run(&mut out)
        .expect("reporting succeeds");

    // Assert
    let text = String::from_utf8(out).expect("utf-8");
    assert!(text.contains("Exact duplicates: 0 groups"), "{text}");
}

#[test]
fn run_writes_the_summary_and_nothing_else() {
    // Arrange
    let (_, result) = analysed("exact_dupes");
    let reporter = OutputFormat::Text.reporter(None);
    let mut out = Vec::new();

    // Act
    StatsCommand::new(&result, reporter.as_ref())
        .run(&mut out)
        .expect("reporting succeeds");

    // Assert
    let text = String::from_utf8(out).expect("utf-8");
    assert!(text.contains("Duplication Statistics"), "{text}");
    assert!(!text.contains("Exact Duplicates"), "{text}");
}

#[test]
fn stats_shows_duplicate_lines() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "stats",
        ])
        .assert()
        .success()
        .stdout(str::contains("Duplicated lines (exact):"))
        .stdout(str::contains("Duplicated lines (near):"));
}

#[test]
fn stats_shows_summary() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "stats",
        ])
        .assert()
        .success()
        .stdout(str::contains("Total code units analyzed"))
        .stdout(str::contains("Exact duplicates"));
}

#[test]
fn sub_function_json_stats() {
    // Arrange & Act
    let output = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("sub_function_dupes").to_str().unwrap(),
            "--sub-function",
            "--format",
            "json",
            "stats",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    let parsed: Value = from_str(&text).unwrap();

    // Assert
    assert_eq!(parsed["sub_exact_groups"].as_u64().unwrap(), 3);
    assert_eq!(parsed["sub_exact_units"].as_u64().unwrap(), 6);
}

#[test]
fn sub_function_min_sub_nodes_filters() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("sub_function_dupes").to_str().unwrap(),
            "--sub-function",
            "--min-sub-nodes",
            "1000",
            "stats",
        ])
        .assert()
        .success()
        .stdout(str::contains("Sub-function").not());
}

#[test]
fn sub_function_stats_shown() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("sub_function_dupes").to_str().unwrap(),
            "--sub-function",
            "stats",
        ])
        .assert()
        .success()
        .stdout(str::contains("Sub-function exact: 3 groups"));
}

#[test]
fn without_sub_function_json_no_sub_fields() {
    // Arrange & Act
    let output = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("sub_function_dupes").to_str().unwrap(),
            "--format",
            "json",
            "stats",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    let parsed: Value = from_str(&text).unwrap();

    // Assert
    assert!(parsed.get("sub_exact_groups").is_none());
    assert!(parsed.get("sub_near_groups").is_none());
}
