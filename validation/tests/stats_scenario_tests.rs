// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::stats_command::StatsCommand;
use dry4rust_cli::invocation::output_format::OutputFormat;
use predicate::str;
use predicates::prelude::*;
use serde_json::Value;
use serde_json::from_str;
use validation::dry_binary::DryBinary;
use validation::fixture::Fixture;
use validation::stats_scenario::StatsScenario;

#[test]
fn json_format_stats() {
    // Arrange & Act
    let output = StatsScenario::command_on("exact_dupes")
        .args(["--format", "json", "stats"])
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
    let output = StatsScenario::command_on("exact_dupes")
        .args(["--format", "json", "stats"])
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
    DryBinary::command()
        .arg("stats")
        .arg("--path")
        .arg(Fixture::path("exact_dupes"))
        .assert()
        .success();
}

#[test]
fn run_over_a_clean_fixture_still_writes_the_summary() {
    // Arrange
    let (_, result) = Fixture::analysed("no_dupes");
    let reporter = OutputFormat::Text.reporter(None);

    // Act
    let text = StatsCommand::new(&result, reporter.as_ref())
        .run()
        .expect("reporting succeeds")
        .text()
        .to_string();

    // Assert
    assert!(text.contains("Exact duplicates: 0 groups"), "{text}");
}

#[test]
fn run_writes_the_summary_and_nothing_else() {
    // Arrange
    let (_, result) = Fixture::analysed("exact_dupes");
    let reporter = OutputFormat::Text.reporter(None);

    // Act
    let text = StatsCommand::new(&result, reporter.as_ref())
        .run()
        .expect("reporting succeeds")
        .text()
        .to_string();

    // Assert
    assert!(text.contains("Duplication Statistics"), "{text}");
    assert!(!text.contains("Exact Duplicates"), "{text}");
}

#[test]
fn stats_shows_duplicate_lines() {
    // Arrange & Act & Assert
    StatsScenario::command_on("exact_dupes")
        .args(["stats"])
        .assert()
        .success()
        .stdout(str::contains("Duplicated lines (exact):"))
        .stdout(str::contains("Duplicated lines (near):"));
}

#[test]
fn stats_shows_summary() {
    // Arrange & Act & Assert
    StatsScenario::command_on("exact_dupes")
        .args(["stats"])
        .assert()
        .success()
        .stdout(str::contains("Total code units analyzed"))
        .stdout(str::contains("Exact duplicates"));
}

#[test]
fn sub_function_json_stats() {
    // Arrange & Act
    let output = StatsScenario::command_on("sub_function_dupes")
        .args(["--sub-function", "--format", "json", "stats"])
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
    StatsScenario::command_on("sub_function_dupes")
        .args(["--sub-function", "--min-sub-nodes", "1000", "stats"])
        .assert()
        .success()
        .stdout(str::contains("Sub-function").not());
}

#[test]
fn sub_function_stats_shown() {
    // Arrange & Act & Assert
    StatsScenario::command_on("sub_function_dupes")
        .args(["--sub-function", "stats"])
        .assert()
        .success()
        .stdout(str::contains("Sub-function exact: 3 groups"));
}

#[test]
fn without_sub_function_json_no_sub_fields() {
    // Arrange & Act
    let output = StatsScenario::command_on("sub_function_dupes")
        .args(["--format", "json", "stats"])
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
