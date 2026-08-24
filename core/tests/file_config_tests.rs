// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::config::Config;
use dry4rust::file_config::FileConfig;
use dry4rust::threshold::Threshold;
use std::path::PathBuf;
use toml::from_str;

fn stating(toml: &str) -> FileConfig {
    from_str(toml).expect("the test states valid toml")
}

#[test]
fn apply_to_changes_only_the_fields_the_file_states() {
    // Arrange
    let config = Config {
        min_nodes: 7,
        min_lines: 3,
        ..Config::default()
    };

    // Act
    let config = stating("min_nodes = 25\n")
        .apply_to(config)
        .expect("25 is in range");

    // Assert
    assert_eq!(config.min_nodes, 25);
    assert_eq!(config.min_lines, 3, "a file says nothing about the rest");
}

#[test]
fn apply_to_of_an_empty_file_changes_nothing() {
    // Arrange
    let config = Config {
        min_nodes: 7,
        sub_function: true,
        ..Config::default()
    };

    // Act
    let config = FileConfig::default()
        .apply_to(config)
        .expect("nothing stated is always in range");

    // Assert
    assert_eq!(config.min_nodes, 7);
    assert!(config.sub_function);
}

#[test]
fn apply_to_reads_a_baseline_path_as_written() {
    // Arrange & Act
    let config = stating("baseline = \"ci/recorded.json\"\n")
        .apply_to(Config::default())
        .expect("a path is always in range");

    // Assert
    assert_eq!(config.baseline, Some(PathBuf::from("ci/recorded.json")));
}

#[test]
fn apply_to_rejects_a_percentage_outside_a_hundred() {
    // Arrange & Act
    let outcome = stating("max_near_percent = 150.0\n").apply_to(Config::default());

    // Assert
    assert_eq!(
        outcome.unwrap_err().to_string(),
        "max_near_percent must be a percentage between 0.0 and 100.0, got 150"
    );
}

#[test]
fn apply_to_rejects_a_similarity_threshold_outside_one() {
    // Arrange & Act
    let outcome = stating("similarity_threshold = 5.0\n").apply_to(Config::default());

    // Assert
    assert_eq!(
        outcome.unwrap_err().to_string(),
        "similarity_threshold must be a fraction between 0.0 and 1.0, got 5"
    );
}

#[test]
fn apply_to_replaces_the_excludes_rather_than_appending_to_them() {
    // Arrange
    let config = Config {
        exclude: vec![String::from("vendor")],
        ..Config::default()
    };

    // Act
    let config = stating("exclude = [\"tests\"]\n")
        .apply_to(config)
        .expect("a list is always in range");

    // Assert
    assert_eq!(
        config.exclude,
        vec![String::from("tests")],
        "a config file states the whole list, unlike --exclude, which adds to it"
    );
}

#[test]
fn apply_to_takes_a_percentage_as_a_share_of_a_hundred() {
    // Arrange & Act
    let config = stating("max_exact_percent = 5.0\n")
        .apply_to(Config::default())
        .expect("5% is in range");

    // Assert
    assert_eq!(
        config.max_exact_percent.map(Threshold::as_percent),
        Some(5.0)
    );
}
