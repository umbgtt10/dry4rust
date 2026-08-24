// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::analysed;
use crate::common::cargo_dry4rust;
use crate::common::fixture_path;
use dry4rust::cli::check_command::CheckCommand;
use dry4rust::cli::checking::check_thresholds::CheckThresholds;
use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::cli_error::CliResult;
use dry4rust::cli::output_format::OutputFormat;
use dry4rust::threshold::Threshold;
use predicate::str;
use predicates::prelude::*;
use serde_json::Value;
use serde_json::from_str;

fn checked(fixture: &str, thresholds: &CheckThresholds) -> (CliResult, String) {
    let (config, result) = analysed(fixture);
    let reporter = OutputFormat::Text.reporter(None);
    let mut out = Vec::new();

    let outcome = CheckCommand::new(&config, &result, reporter.as_ref(), thresholds).run(&mut out);

    (outcome, String::from_utf8(out).expect("utf-8"))
}

fn percent(value: f64) -> Threshold {
    Threshold::percent("a ceiling", value).expect("the test states a share of a hundred")
}

#[test]
fn check_absolute_passes_percentage_fails() {
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
fn check_in_json_is_one_document_carrying_the_verdict() {
    // Arrange & Act
    let output = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("mixed").to_str().unwrap(),
            "--format",
            "json",
            "check",
            "--max-exact",
            "0",
            "--max-exact-percent",
            "0",
        ])
        .assert()
        .code(1)
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();

    // Assert
    let document: Value = from_str(&text).expect(
        "the verdict used to be printed as a sentence between the sections, \
         which left the whole thing unparseable",
    );
    assert_eq!(document["passed"], false);
    assert_eq!(
        document["breaches"].as_array().expect("breaches").len(),
        2,
        "both exact ceilings were breached and both are named, got: {text}"
    );
    assert_eq!(
        document["exact"].as_array().expect("exact").len(),
        1,
        "and the groups behind them are listed once, not once per breach"
    );
}

#[test]
fn check_in_json_says_it_passed_when_nothing_is_breached() {
    // Arrange & Act
    let output = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("no_dupes").to_str().unwrap(),
            "--format",
            "json",
            "check",
            "--max-exact",
            "0",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    // Assert
    let document: Value = from_str(&String::from_utf8(output).unwrap()).expect("valid json");
    assert_eq!(document["passed"], true);
    assert!(
        document["breaches"]
            .as_array()
            .expect("breaches")
            .is_empty()
    );
    assert!(
        document.get("exact").is_none(),
        "nothing breached, so there are no offenders to list"
    );
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

#[test]
fn check_with_a_percentage_ceiling_above_a_hundred_is_rejected() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "check",
            "--max-exact-percent",
            "150",
        ])
        .assert()
        .code(2)
        .stderr(str::contains(
            "--max-exact-percent must be a percentage between 0.0 and 100.0, got 150",
        ));
}

#[test]
fn main_dispatches_the_check_subcommand_and_exits_one_when_a_ceiling_is_breached() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .arg("check")
        .arg("--max-exact")
        .arg("0")
        .arg("--path")
        .arg(fixture_path("exact_dupes"))
        .assert()
        .code(1);
}

#[test]
fn main_dispatches_the_check_subcommand_and_succeeds_on_a_clean_fixture() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .arg("check")
        .arg("--max-exact")
        .arg("0")
        .arg("--path")
        .arg(fixture_path("no_dupes"))
        .assert()
        .success();
}

#[test]
fn run_over_a_clean_fixture_passes_with_every_ceiling_at_zero() {
    // Arrange
    let thresholds = CheckThresholds {
        max_exact: Some(0),
        max_near: Some(0),
        max_exact_percent: Some(percent(0.0)),
        max_near_percent: Some(percent(0.0)),
    };

    // Act
    let (outcome, text) = checked("no_dupes", &thresholds);

    // Assert
    assert!(outcome.is_ok());
    assert!(text.contains("Check passed."), "{text}");
}

#[test]
fn run_with_a_generous_percentage_ceiling_passes() {
    // Arrange
    let thresholds = CheckThresholds {
        max_exact_percent: Some(percent(100.0)),
        max_near_percent: Some(percent(100.0)),
        ..CheckThresholds::default()
    };

    // Act
    let (outcome, _) = checked("exact_dupes", &thresholds);

    // Assert
    assert!(outcome.is_ok());
}

#[test]
fn run_with_a_near_duplicate_ceiling_fails_when_it_is_exceeded() {
    // Arrange
    let (config, result) = analysed("near_dupes");
    let reporter = OutputFormat::Text.reporter(None);
    let mut out = Vec::new();
    let thresholds = CheckThresholds {
        max_near: Some(0),
        ..CheckThresholds::default()
    };

    // Act
    let outcome = CheckCommand::new(&config, &result, reporter.as_ref(), &thresholds).run(&mut out);

    // Assert
    assert_eq!(
        result.near_groups.len(),
        1,
        "the fixture holding a near-duplicate group is why the ceiling is \
         breached; without that this test would prove nothing"
    );
    assert!(matches!(outcome, Err(CliError::CheckFailed)));
}

#[test]
fn run_with_a_near_percentage_ceiling_of_zero_fails_on_near_duplication() {
    // Arrange
    let (config, result) = analysed("near_dupes");
    let reporter = OutputFormat::Text.reporter(None);
    let mut out = Vec::new();
    let thresholds = CheckThresholds {
        max_near_percent: Some(percent(0.0)),
        ..CheckThresholds::default()
    };

    // Act
    let outcome = CheckCommand::new(&config, &result, reporter.as_ref(), &thresholds).run(&mut out);

    // Assert
    assert!(
        result.stats.near_duplicate_percent() > 0.0,
        "a ceiling of zero percent is only breached because the fixture has \
         near-duplicated lines to measure"
    );
    assert!(matches!(outcome, Err(CliError::CheckFailed)));
}

#[test]
fn run_with_a_percentage_ceiling_of_zero_fails_on_any_duplication() {
    // Arrange
    let thresholds = CheckThresholds {
        max_exact_percent: Some(percent(0.0)),
        ..CheckThresholds::default()
    };

    // Act
    let (outcome, _) = checked("exact_dupes", &thresholds);

    // Assert
    assert!(outcome.is_err());
}

#[test]
fn run_with_a_zero_threshold_fails_on_duplicates() {
    // Arrange
    let thresholds = CheckThresholds {
        max_exact: Some(0),
        ..CheckThresholds::default()
    };

    // Act
    let (outcome, _) = checked("exact_dupes", &thresholds);

    // Assert
    assert!(outcome.is_err());
}

#[test]
fn run_with_an_exact_count_ceiling_above_the_findings_passes() {
    // Arrange
    let thresholds = CheckThresholds {
        max_exact: Some(9999),
        max_near: Some(9999),
        ..CheckThresholds::default()
    };

    // Act
    let (outcome, _) = checked("exact_dupes", &thresholds);

    // Assert
    assert!(outcome.is_ok());
}

#[test]
fn run_with_every_ceiling_set_reports_each_breach_it_finds() {
    // Arrange
    let thresholds = CheckThresholds {
        max_exact: Some(0),
        max_near: Some(0),
        max_exact_percent: Some(percent(0.0)),
        max_near_percent: Some(percent(0.0)),
    };

    // Act
    let (outcome, text) = checked("exact_dupes", &thresholds);

    // Assert
    assert!(
        outcome.is_err(),
        "a fixture with duplicates breaches a zero ceiling"
    );
    assert!(
        text.contains("exact duplicate groups") && text.contains("exact duplicate lines"),
        "every breach is named, not only the first, got: {text}"
    );
}

#[test]
fn run_without_thresholds_passes_even_with_duplicates() {
    // Arrange & Act
    let (outcome, _) = checked("exact_dupes", &CheckThresholds::default());

    // Assert
    assert!(outcome.is_ok());
}
