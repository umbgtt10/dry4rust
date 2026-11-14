// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use clap::Parser;
use dry4rust::cli::command::Command;
use dry4rust::config::Config;
use dry4rust::source_path::SourcePath;
use dry4rust_cli::invocation::args::Args;
use dry4rust_cli::invocation::output_format::OutputFormat;
use std::path::Path;
use std::path::PathBuf;

fn parsed(args: &[&str]) -> Args {
    Args::try_parse_from(args).expect("the arguments parse")
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
    let config = parsed(&[
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
    .overrides()
    .apply_to(Config::default())
    .expect("the overrides are in range");

    // Assert
    assert_eq!(config.min_nodes(), 42);
    assert_eq!(config.min_lines(), 7);
    assert!((config.similarity_threshold().as_fraction() - 0.75).abs() < f64::EPSILON);
    assert_eq!(config.exclude(), vec!["vendor", "benches"]);
    assert!(config.exclude_tests());
    assert!(config.sub_function());
    assert_eq!(config.min_sub_nodes(), 3);
    assert_eq!(
        config.baseline(),
        Some(&SourcePath::new(String::from("ci/recorded.json")))
    );
}

#[test]
fn overrides_leaves_a_flag_that_was_not_passed_with_no_opinion() {
    // Arrange
    let turned_on = Config::default()
        .with_sub_function(true)
        .with_exclude_tests(true)
        .with_exclude(vec![String::from("kept")]);

    // Act
    let overrides = parsed(&["cargo-dry4rust", "dry4rust"]).overrides();
    let on = overrides
        .apply_to(turned_on)
        .expect("no override is always in range");
    let off = overrides
        .apply_to(Config::default())
        .expect("no override is always in range");

    // Assert
    assert!(
        on.sub_function(),
        "Some(false) would switch off a dry4rust.toml that turned it on; \
         absence has to mean absence"
    );
    assert!(on.exclude_tests());
    assert_eq!(on.exclude(), vec!["kept"]);
    assert!(!off.sub_function());
    assert!(!off.exclude_tests());
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
