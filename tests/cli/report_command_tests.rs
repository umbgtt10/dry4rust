// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::{analysed, analysed_with_sub_function, cargo_dry4rust, fixture_path};
use dry4rust::cli::output_format::OutputFormat;
use dry4rust::cli::report_command::ReportCommand;
use predicate::str;
use predicates::prelude::*;
use serde_json::Deserializer;
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
    let parts: Vec<&str> = text.splitn(2, "\n\n").collect();

    // Assert
    assert!(parts.len() >= 2, "expected stats + groups sections");
    let stats: Value = from_str(parts[0]).unwrap();
    assert!(stats["total_code_units"].as_u64().unwrap() > 0);
    assert!(stats["exact_duplicate_groups"].as_u64().unwrap() > 0);
    let groups: Value = from_str(parts[1]).unwrap();
    assert!(!groups.as_array().unwrap().is_empty());
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
    // Each section is its own top-level JSON document, written one after
    // another rather than wrapped in one; a stream reader is what reads that.
    let sections: Vec<Value> = Deserializer::from_str(&text)
        .into_iter::<Value>()
        .collect::<Result<_, _>>()
        .expect("every section parses");

    // Assert
    assert!(
        sections[0]["total_code_units"].as_u64().expect("a count") > 0,
        "the first section is the summary, got: {text}"
    );
    let sub_near = sections
        .last()
        .expect("a last section")
        .as_array()
        .expect("the group sections are arrays");
    assert_eq!(sub_near.len(), 1);
    assert!(
        sub_near[0]["similarity"].as_f64().expect("a score") < 1.0,
        "a near group scores below one; an exact one would be in the other \
         section, got: {text}"
    );
    assert_eq!(sub_near[0]["members"].as_array().expect("members").len(), 2);
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
