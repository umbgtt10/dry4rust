// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use crate::common::result_with;
use dry4rust::cli::check_command::CheckCommand;
use dry4rust::cli::checking::check_thresholds::CheckThresholds;
use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::cli_error::CliResult;
use dry4rust::cli::output_format::OutputFormat;
use dry4rust::config::Config;

fn checked(exact: usize, thresholds: &CheckThresholds) -> (CliResult, String) {
    let groups = (0..exact)
        .map(|n| group(0x11 + n as u64, &["a", "b"]))
        .collect();
    let result = result_with(groups, vec![], vec![], vec![]);
    let config = Config::default();
    let reporter = OutputFormat::Text.reporter(None);
    let mut out = Vec::new();

    let outcome = CheckCommand::new(&config, &result, reporter.as_ref(), thresholds).run(&mut out);

    (outcome, String::from_utf8(out).expect("utf-8"))
}

#[test]
fn run_over_a_clean_result_passes_with_a_ceiling_of_zero() {
    // Arrange
    let thresholds = CheckThresholds {
        max_exact: Some(0),
        ..CheckThresholds::default()
    };

    // Act
    let (outcome, text) = checked(0, &thresholds);

    // Assert
    assert!(outcome.is_ok());
    assert!(text.contains("Check passed."), "{text}");
}

#[test]
fn run_with_a_ceiling_above_the_findings_passes() {
    // Arrange
    let thresholds = CheckThresholds {
        max_exact: Some(9999),
        ..CheckThresholds::default()
    };

    // Act
    let (outcome, _) = checked(2, &thresholds);

    // Assert
    assert!(outcome.is_ok());
}

#[test]
fn run_with_a_zero_ceiling_fails_on_any_group() {
    // Arrange
    let thresholds = CheckThresholds {
        max_exact: Some(0),
        ..CheckThresholds::default()
    };

    // Act
    let (outcome, text) = checked(1, &thresholds);

    // Assert
    assert!(matches!(outcome, Err(CliError::CheckFailed)));
    assert!(text.contains("1 exact duplicate groups (max: 0)"), "{text}");
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
