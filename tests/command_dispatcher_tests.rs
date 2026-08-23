// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::fixture_path;
use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::cli::command::Command;
use dry4rust::cli::output_format::OutputFormat;
use dry4rust::command_dispatcher::CommandDispatcher;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use std::path::Path;
use tempfile::TempDir;

fn breaching_check() -> Command {
    Command::Check {
        max_exact: Some(0),
        max_near: None,
        max_exact_percent: None,
        max_near_percent: None,
    }
}

fn dispatch_in(root: &Path, command: &Command) -> (Result<(), CliError>, String) {
    let analyzer = RustAnalyzer::new();
    let dispatcher =
        CommandDispatcher::new(&analyzer, root, OutputFormat::Text, CliOverrides::default());
    let mut out = Vec::new();
    let outcome = dispatcher.dispatch(command, &mut out);
    (outcome, String::from_utf8(out).expect("utf-8"))
}

#[test]
fn dispatch_over_a_missing_root_returns_an_error() {
    // Arrange
    let root = fixture_path("no_such_fixture_anywhere");

    // Act
    let (outcome, _) = dispatch_in(&root, &Command::Report);

    // Assert
    assert!(
        outcome.is_err(),
        "a root with no sources cannot be analysed"
    );
}

#[test]
fn dispatch_with_check_breaching_its_ceiling_returns_check_failed() {
    // Arrange
    let root = fixture_path("exact_dupes");

    // Act
    let (outcome, _) = dispatch_in(&root, &breaching_check());

    // Assert
    assert!(matches!(outcome, Err(CliError::CheckFailed)));
}

#[test]
fn dispatch_with_check_within_its_ceiling_succeeds() {
    // Arrange
    let root = fixture_path("no_dupes");

    // Act
    let (outcome, _) = dispatch_in(&root, &breaching_check());

    // Assert
    assert!(outcome.is_ok(), "a clean fixture breaches no ceiling");
}

#[test]
fn dispatch_with_cleanup_in_dry_run_succeeds_and_writes_a_report() {
    // Arrange
    let root = fixture_path("exact_dupes");

    // Act
    let (outcome, text) = dispatch_in(&root, &Command::Cleanup { dry_run: true });

    // Assert
    assert!(outcome.is_ok());
    assert!(
        text.contains("stale"),
        "cleanup names what it found rather than merely producing output, got: {text}"
    );
}

#[test]
fn dispatch_with_ignore_records_the_fingerprint_without_analysing() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let command = Command::Ignore {
        fingerprint: "cafebabe00000001".to_string(),
        reason: Some("a reason".to_string()),
    };

    // Act
    let (outcome, _) = dispatch_in(tmp.path(), &command);

    // Assert
    assert!(
        outcome.is_ok(),
        "an empty directory has no sources, yet ignore does not analyse"
    );
}

#[test]
fn dispatch_with_ignored_lists_what_ignore_recorded() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let recorded = Command::Ignore {
        fingerprint: "cafebabe00000002".to_string(),
        reason: None,
    };
    dispatch_in(tmp.path(), &recorded)
        .0
        .expect("ignore accepted");

    // Act
    let (outcome, text) = dispatch_in(tmp.path(), &Command::Ignored);

    // Assert
    assert!(outcome.is_ok());
    assert!(
        text.contains("cafebabe00000002"),
        "the listing names the fingerprint just recorded, got: {text}"
    );
}

#[test]
fn dispatch_with_report_writes_the_full_report() {
    // Arrange
    let root = fixture_path("exact_dupes");

    // Act
    let (outcome, text) = dispatch_in(&root, &Command::Report);

    // Assert
    assert!(outcome.is_ok());
    assert!(
        text.contains("Exact Duplicates"),
        "a report over a duplicated fixture lists the duplicates, got: {text}"
    );
    assert!(
        text.contains("Exact duplicates: 1 groups"),
        "and says how many; the fixture holds exactly one group"
    );
}

#[test]
fn dispatch_with_stats_writes_a_summary() {
    // Arrange
    let root = fixture_path("exact_dupes");

    // Act
    let (outcome, text) = dispatch_in(&root, &Command::Stats);

    // Assert
    assert!(outcome.is_ok());
    assert!(
        text.contains("Duplication Statistics"),
        "stats is the summary, not the listing, got: {text}"
    );
    assert!(
        !text.contains("Exact Duplicates"),
        "and it stops short of the per-group listing report gives"
    );
}

#[test]
fn new_stores_the_overrides_it_is_given_and_applies_them_to_the_analysis() {
    // Arrange
    let analyzer = RustAnalyzer::new();
    let root = fixture_path("exact_dupes");
    let overrides = CliOverrides {
        min_nodes: Some(100_000),
        ..CliOverrides::default()
    };
    let dispatcher = CommandDispatcher::new(&analyzer, &root, OutputFormat::Text, overrides);
    let mut out = Vec::new();

    // Act
    let outcome = dispatcher.dispatch(&breaching_check(), &mut out);

    // Assert
    assert!(
        outcome.is_ok(),
        "a node floor no unit can reach leaves nothing to be duplicated"
    );
}
