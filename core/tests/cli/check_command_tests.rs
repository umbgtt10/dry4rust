// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use crate::common::result_with;
use dry4rust::analysis::Analysis;
use dry4rust::cli::check_command::CheckCommand;
use dry4rust::cli::checking::check_thresholds::CheckThresholds;
use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::cli_error::CliResult;
use dry4rust::cli::output::text_reporter::TextReporter;
use dry4rust::config::Config;
use dry4rust::groups::duplication_stats::DuplicationStats;
use std::collections::BTreeSet;

fn checked(exact: usize, thresholds: &CheckThresholds) -> (CliResult, String) {
    let groups = (0..exact)
        .map(|n| group(0x11 + n as u64, &["a", "b"]))
        .collect();
    let result = result_with(groups, vec![], vec![], vec![]);
    let config = Config::default();
    let reporter = TextReporter::new(None);

    let outcome = CheckCommand::new(&config, &result, &reporter, thresholds)
        .run()
        .expect("check produces a report");

    outcome.into_parts()
}

// A check of a result whose statistics state half its lines exact
// duplicates and a quarter near ones, with one near group.
fn checked_by_share(thresholds: &CheckThresholds) -> (CliResult, String) {
    let stats = DuplicationStats::default()
        .with_total_lines(100)
        .with_exact_duplicate_lines(50)
        .with_near_duplicate_lines(25)
        .with_near_duplicate_groups(1);
    let result = Analysis::new(
        stats,
        Vec::new(),
        vec![group(0x21, &["near_one", "near_other"])],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        BTreeSet::new(),
    );
    let config = Config::default();
    let reporter = TextReporter::new(None);

    let outcome = CheckCommand::new(&config, &result, &reporter, thresholds)
        .run()
        .expect("check produces a report");

    outcome.into_parts()
}

#[test]
fn run_over_a_clean_result_passes_with_a_ceiling_of_zero() {
    // Arrange
    let thresholds =
        CheckThresholds::new(Some(0), None, None, None).expect("every ceiling is in range");

    // Act
    let (outcome, text) = checked(0, &thresholds);

    // Assert
    assert!(outcome.is_ok());
    assert!(text.contains("Check passed."), "{text}");
}

#[test]
fn run_with_a_ceiling_above_the_findings_passes() {
    // Arrange
    let thresholds =
        CheckThresholds::new(Some(9999), None, None, None).expect("every ceiling is in range");

    // Act
    let (outcome, _) = checked(2, &thresholds);

    // Assert
    assert!(outcome.is_ok());
}

#[test]
fn run_with_a_near_ceiling_given_fails_above_it_naming_the_groups_behind_it() {
    // Arrange
    let thresholds =
        CheckThresholds::new(None, Some(0), None, None).expect("every ceiling is in range");

    // Act
    let (outcome, text) = checked_by_share(&thresholds);

    // Assert
    assert!(matches!(outcome, Err(CliError::CheckFailed)));
    assert!(text.contains("1 near duplicate groups (max: 0)"), "{text}");
    assert!(text.contains("near_one"), "{text}");
}

#[test]
fn run_with_a_near_share_given_fails_above_it() {
    // Arrange
    let thresholds =
        CheckThresholds::new(None, None, None, Some(20.0)).expect("every ceiling is in range");

    // Act
    let (outcome, text) = checked_by_share(&thresholds);

    // Assert
    assert!(matches!(outcome, Err(CliError::CheckFailed)));
    assert!(
        text.contains("25.0% near duplicate lines (max: 20.0%)"),
        "{text}"
    );
}

#[test]
fn run_with_a_zero_ceiling_fails_on_any_group() {
    // Arrange
    let thresholds =
        CheckThresholds::new(Some(0), None, None, None).expect("every ceiling is in range");

    // Act
    let (outcome, text) = checked(1, &thresholds);

    // Assert
    assert!(matches!(outcome, Err(CliError::CheckFailed)));
    assert!(text.contains("1 exact duplicate groups (max: 0)"), "{text}");
}

#[test]
fn run_with_an_exact_share_given_fails_above_it() {
    // Arrange
    let thresholds =
        CheckThresholds::new(None, None, Some(40.0), None).expect("every ceiling is in range");

    // Act
    let (outcome, text) = checked_by_share(&thresholds);

    // Assert
    assert!(matches!(outcome, Err(CliError::CheckFailed)));
    assert!(
        text.contains("50.0% exact duplicate lines (max: 40.0%)"),
        "{text}"
    );
}

#[test]
fn run_without_thresholds_passes_however_much_it_finds() {
    // Arrange & Act
    let (outcome, _) = checked(5, &CheckThresholds::default());

    // Assert
    assert!(
        outcome.is_ok(),
        "an unset ceiling means the caller did not ask, not that it asked for zero"
    );
}
