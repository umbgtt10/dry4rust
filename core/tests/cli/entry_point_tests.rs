// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use clap::Parser;
use dry4rust::cli::command::Command;
use dry4rust::cli::entry_point::EntryPoint;
use dry4rust::cli::output_format::OutputFormat;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

fn parsed(args: &[&str]) -> EntryPoint {
    EntryPoint::try_parse_from(args).expect("the arguments parse")
}

#[test]
fn command_defaults_to_report_when_none_is_named() {
    // Arrange & Act
    let entry = parsed(&["cargo-dry4rust", "dry4rust"]);

    // Assert
    assert!(matches!(entry.command(), Command::Report));
}

#[test]
fn command_returns_the_subcommand_that_was_named() {
    // Arrange & Act
    let entry = parsed(&["cargo-dry4rust", "dry4rust", "stats"]);

    // Assert
    assert!(matches!(entry.command(), Command::Stats));
}

#[test]
fn format_defaults_to_text() {
    // Arrange & Act
    let entry = parsed(&["cargo-dry4rust", "dry4rust"]);

    // Assert
    assert_eq!(entry.format(), OutputFormat::Text);
}

#[test]
fn format_reads_the_flag_when_it_is_given() {
    // Arrange & Act
    let entry = parsed(&["cargo-dry4rust", "dry4rust", "--format", "json"]);

    // Assert
    assert_eq!(entry.format(), OutputFormat::Json);
}

#[test]
fn overrides_carries_every_flag_that_was_passed() {
    // Arrange & Act
    let overrides = parsed(&[
        "cargo-dry4rust",
        "dry4rust",
        "--min-nodes",
        "42",
        "--min-lines",
        "7",
        "--threshold",
        "0.75",
        "--exclude",
        "vendor",
        "--exclude",
        "benches",
        "--exclude-tests",
        "--sub-function",
        "--min-sub-nodes",
        "3",
        "--baseline",
        "ci/recorded.json",
    ])
    .overrides();

    // Assert
    assert_eq!(overrides.min_nodes, Some(42));
    assert_eq!(overrides.min_lines, Some(7));
    assert_eq!(overrides.threshold, Some(0.75));
    assert_eq!(overrides.exclude, vec!["vendor", "benches"]);
    assert_eq!(overrides.exclude_tests, Some(true));
    assert_eq!(overrides.sub_function, Some(true));
    assert_eq!(overrides.min_sub_nodes, Some(3));
    assert_eq!(overrides.baseline, Some(PathBuf::from("ci/recorded.json")));
}

#[test]
fn overrides_leaves_a_flag_that_was_not_passed_with_no_opinion() {
    // Arrange & Act
    let overrides = parsed(&["cargo-dry4rust", "dry4rust"]).overrides();

    // Assert
    assert_eq!(
        overrides.sub_function, None,
        "Some(false) would switch off a dry4rust.toml that turned it on; \
         absence has to mean absence"
    );
    assert_eq!(overrides.exclude_tests, None);
    assert!(overrides.exclude.is_empty());
}

#[test]
fn root_falls_back_to_the_working_directory_when_no_path_is_given() {
    // Arrange & Act
    let root = parsed(&["cargo-dry4rust", "dry4rust"]).root();

    // Assert
    assert!(
        root.is_absolute() || root == Path::new("."),
        "either the shell's directory or the last resort, got {}",
        root.display()
    );
}

#[test]
fn root_returns_the_path_that_was_given() {
    // Arrange & Act
    let root = parsed(&["cargo-dry4rust", "dry4rust", "--path", "/projects/thing"]).root();

    // Assert
    assert_eq!(root, PathBuf::from("/projects/thing"));
}

#[test]
fn run_over_a_request_for_help_exits_zero() {
    // Arrange
    let args = ["cargo-dry4rust", "dry4rust", "--help"];

    // Act
    let code = EntryPoint::run(args.iter().map(|a| (*a).to_owned()).collect());

    // Assert
    assert_eq!(
        code,
        ExitCode::SUCCESS,
        "clap reports --help the same way it reports a typo; only use_stderr          tells them apart"
    );
}

#[test]
fn run_over_a_root_with_no_sources_reports_the_error_and_exits_two() {
    // Arrange
    let args = [
        "cargo-dry4rust",
        "dry4rust",
        "--path",
        "/no/such/place",
        "stats",
    ];

    // Act
    let code = EntryPoint::run(args.iter().map(|a| (*a).to_owned()).collect());

    // Assert
    assert_eq!(
        code,
        ExitCode::from(2),
        "2 is the code for an error the tool could not run through, as against          1 for duplication over a ceiling"
    );
}

#[test]
fn run_over_arguments_it_cannot_parse_exits_two() {
    // Arrange
    let args = ["cargo-dry4rust", "dry4rust", "--no-such-flag"];

    // Act
    let code = EntryPoint::run(args.iter().map(|a| (*a).to_owned()).collect());

    // Assert
    assert_eq!(code, ExitCode::from(2));
}
