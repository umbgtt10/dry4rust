// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::{cargo_dry4rust, fixture_path};
use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::config::Config;
use predicate::str;
use predicates::prelude::*;
use serde_json::Value;
use serde_json::from_str;

#[test]
fn apply_to_appends_excludes_rather_than_replacing_the_configured_ones() {
    // Arrange
    let config = Config {
        exclude: vec![String::from("vendor")],
        ..Config::default()
    };
    let overrides = CliOverrides {
        exclude: vec![String::from("benches")],
        ..CliOverrides::default()
    };

    // Act
    let config = overrides
        .apply_to(config)
        .expect("the overrides are in range");

    // Assert
    assert_eq!(
        config.exclude,
        vec![String::from("vendor"), String::from("benches")],
        "a --exclude on the command line adds to the config file, it does not stand in for it"
    );
}

#[test]
fn apply_to_replaces_only_the_values_that_were_given() {
    // Arrange
    let config = Config::default();
    let untouched_threshold = config.similarity_threshold.as_fraction();
    let overrides = CliOverrides {
        min_nodes: Some(42),
        ..CliOverrides::default()
    };

    // Act
    let config = overrides
        .apply_to(config)
        .expect("the overrides are in range");

    // Assert
    assert_eq!(config.min_nodes, 42);
    assert!((config.similarity_threshold.as_fraction() - untouched_threshold).abs() < f64::EPSILON);
}

#[test]
fn apply_to_with_nothing_set_leaves_every_field_as_it_was() {
    // Arrange
    let config = Config {
        min_nodes: 7,
        min_lines: 3,
        sub_function: true,
        min_sub_nodes: 11,
        exclude_tests: true,
        ..Config::default()
    };

    // Act
    let config = CliOverrides::default()
        .apply_to(config)
        .expect("no override is always in range");

    // Assert
    assert_eq!(config.min_nodes, 7);
    assert_eq!(config.min_lines, 3);
    assert_eq!(config.min_sub_nodes, 11);
    assert!(config.sub_function);
    assert!(config.exclude_tests);
}

#[test]
fn exclude_option_drops_the_named_paths_from_the_report() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "--exclude",
            "lib.rs",
            "stats",
        ])
        .assert()
        .code(2)
        .stderr(str::contains("No source files"));
}

#[test]
fn exclude_tests_flag_reduces_duplicates() {
    // Arrange & Act
    let output_all = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("test_code").to_str().unwrap(),
            "--format",
            "json",
            "stats",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let all: Value = from_str(&String::from_utf8(output_all).unwrap()).unwrap();

    // Assert
    assert_eq!(all["exact_duplicate_units"].as_u64().unwrap(), 3);

    let output_excl = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("test_code").to_str().unwrap(),
            "--exclude-tests",
            "--format",
            "json",
            "stats",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let excl: Value = from_str(&String::from_utf8(output_excl).unwrap()).unwrap();
    assert_eq!(excl["exact_duplicate_units"].as_u64().unwrap(), 2);
    assert_eq!(excl["total_code_units"].as_u64().unwrap(), 2);
}

#[test]
fn exclude_tests_text_report() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("test_code").to_str().unwrap(),
            "--exclude-tests",
            "report",
        ])
        .assert()
        .success()
        .stdout(str::contains("Exact Duplicates"))
        .stdout(str::contains("Group 1"));
}

#[test]
fn min_lines_option() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "--min-lines",
            "1000",
            "stats",
        ])
        .assert()
        .success()
        .stdout(str::contains("Exact duplicates: 0 groups"));
}

#[test]
fn min_nodes_option() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "--min-nodes",
            "1000",
            "stats",
        ])
        .assert()
        .success()
        .stdout(str::contains("Exact duplicates: 0 groups"));
}

#[test]
fn threshold_above_one_is_rejected_rather_than_silently_finding_nothing() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("near_dupes").to_str().unwrap(),
            "--threshold",
            "5",
            "report",
        ])
        .assert()
        .code(2)
        .stderr(str::contains(
            "--threshold must be a fraction between 0.0 and 1.0, got 5",
        ));
}
