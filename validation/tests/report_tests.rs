// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::analysed;
use crate::common::analysed_with_sub_function;
use crate::common::cargo_dry4rust;
use crate::common::fixture_path;
use dry4rust::cli::output_format::OutputFormat;
use dry4rust::cli::report_command::ReportCommand;
use predicate::str;
use predicates::prelude::*;
use serde_json::Value;
use serde_json::from_str;

#[test]
fn json_format_report() {
    // Arrange & Act
    let output = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "--format",
            "json",
            "report",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();

    // Assert
    let document: Value = from_str(&text).expect("the whole report is one JSON document");
    assert!(document["stats"]["total_code_units"].as_u64().unwrap() > 0);
    assert!(
        document["stats"]["exact_duplicate_groups"]
            .as_u64()
            .unwrap()
            > 0
    );
    let groups = document["exact"].as_array().expect("a named exact section");
    assert!(!groups.is_empty());
    assert!(groups[0]["fingerprint"].is_string());
    assert!(groups[0]["members"].is_array());
}

#[test]
fn near_dupes_detected() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("near_dupes").to_str().unwrap(),
            "--threshold",
            "0.7",
            "report",
        ])
        .assert()
        .success()
        .stdout(str::contains("Near Duplicates"))
        .stdout(str::contains("Group 1"))
        .stdout(str::contains("similarity:"));
}

#[test]
fn report_exact_dupes_fixture() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "report",
        ])
        .assert()
        .success()
        .stdout(str::contains("Exact Duplicates"))
        .stdout(str::contains("Group 1"));
}

#[test]
fn report_mixed_fixture() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args(["--path", fixture_path("mixed").to_str().unwrap(), "report"])
        .assert()
        .success()
        .stdout(str::contains("Exact Duplicates"))
        .stdout(str::contains("Group 1"));
}

#[test]
fn report_no_dupes_fixture() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("no_dupes").to_str().unwrap(),
            "report",
        ])
        .assert()
        .success()
        .stdout(str::contains("No exact duplicates"));
}

#[test]
fn run_over_a_clean_fixture_says_so_rather_than_printing_an_empty_section() {
    // Arrange
    let (_, result) = analysed("no_dupes");
    let reporter = OutputFormat::Text.reporter(None);
    let mut out = Vec::new();

    // Act
    ReportCommand::new(&result, reporter.as_ref())
        .run(&mut out)
        .expect("reporting succeeds");

    // Assert
    let text = String::from_utf8(out).expect("utf-8");
    assert!(text.contains("No exact duplicates found."), "{text}");
    assert!(
        !text.contains("Near Duplicates"),
        "an empty near section is left out entirely, got: {text}"
    );
}

#[test]
fn run_writes_both_the_stats_and_the_groups() {
    // Arrange
    let (_, result) = analysed("exact_dupes");
    let reporter = OutputFormat::Text.reporter(None);
    let mut out = Vec::new();

    // Act
    ReportCommand::new(&result, reporter.as_ref())
        .run(&mut out)
        .expect("reporting succeeds");

    // Assert
    let text = String::from_utf8(out).expect("utf-8");
    assert!(text.contains("Duplication Statistics"), "{text}");
    assert!(text.contains("Exact Duplicates"), "{text}");
}

#[test]
fn run_writes_the_sub_function_sections_when_there_are_any() {
    // Arrange
    let (_, result) = analysed_with_sub_function("sub_function_near_dupes");
    let reporter = OutputFormat::Text.reporter(None);
    let mut out = Vec::new();

    // Act
    ReportCommand::new(&result, reporter.as_ref())
        .run(&mut out)
        .expect("reporting succeeds");

    // Assert
    let text = String::from_utf8(out).expect("utf-8");
    assert_eq!(
        result.sub_near_groups.len(),
        1,
        "the fixture's two loop bodies differ by one operator, which is what \
         puts them in a near group rather than an exact one"
    );
    assert!(
        text.contains("Sub-function Near Duplicates"),
        "the section a result with sub-near groups must produce, got: {text}"
    );
    assert!(
        !text.contains("Sub-function Exact Duplicates"),
        "and not the one it has no groups for, got: {text}"
    );
}

#[test]
fn sub_function_detects_duplicate_branches() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("sub_function_dupes").to_str().unwrap(),
            "--sub-function",
            "report",
        ])
        .assert()
        .success()
        .stdout(str::contains("Sub-function Exact Duplicates"))
        .stdout(str::contains("if-then branch"))
        .stdout(str::contains("match arm"))
        .stdout(str::contains("for body"));
}

#[test]
fn sub_function_json_report_carries_the_sub_near_section() {
    // Arrange & Act
    let output = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("sub_function_near_dupes").to_str().unwrap(),
            "--sub-function",
            "--format",
            "json",
            "report",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();

    // Assert
    let document: Value = from_str(&text).expect("the whole report is one JSON document");
    assert!(
        document["stats"]["total_code_units"]
            .as_u64()
            .expect("a count")
            > 0
    );
    let sub_near = document["sub_near"]
        .as_array()
        .expect("a named sub_near section");
    assert_eq!(sub_near.len(), 1);
    assert!(
        sub_near[0]["similarity"].as_f64().expect("a score") < 1.0,
        "a near group scores below one; an exact one would be in the other \
         section, got: {text}"
    );
    assert_eq!(sub_near[0]["members"].as_array().expect("members").len(), 2);
    assert!(
        document.get("sub_exact").is_none(),
        "a section the run produced nothing for is absent, not an empty array \
         that would suggest it was looked at, got: {text}"
    );
}

#[test]
fn sub_function_near_duplicates_are_reported_under_their_own_heading() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("sub_function_near_dupes").to_str().unwrap(),
            "--sub-function",
            "report",
        ])
        .assert()
        .success()
        .stdout(str::contains("Sub-function near:  1 groups (2 units)"))
        .stdout(str::contains("Sub-function Near Duplicates"))
        .stdout(str::contains("for body (loop body) in total_rising"))
        .stdout(str::contains("for body (loop body) in total_falling"));
}

#[test]
fn sub_function_shows_parent_names() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("sub_function_dupes").to_str().unwrap(),
            "--sub-function",
            "report",
        ])
        .assert()
        .success()
        .stdout(str::contains("in handle_positive"))
        .stdout(str::contains("in process_value"))
        .stdout(str::contains("in classify_number"))
        .stdout(str::contains("in describe_value"));
}

#[test]
fn without_sub_function_flag_no_sub_sections() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("sub_function_dupes").to_str().unwrap(),
            "report",
        ])
        .assert()
        .success()
        .stdout(str::contains("Exact Duplicates"))
        .stdout(str::contains("Sub-function").not());
}
