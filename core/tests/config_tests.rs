// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::source_path;
use dry4rust::config::Config;
use dry4rust::threshold::Threshold;
use std::fs;
use tempfile::TempDir;

#[test]
fn config_with_exclude_tests() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("dry4rust.toml"),
        r"
        exclude_tests = true
        ",
    )
    .unwrap();
    let config = Config::load(&source_path(tmp.path())).expect("every value is in range");

    // Assert
    assert!(config.exclude_tests());
}

#[test]
fn config_with_min_lines() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("dry4rust.toml"),
        r"
        min_lines = 5
        ",
    )
    .unwrap();
    let config = Config::load(&source_path(tmp.path())).expect("every value is in range");

    // Assert
    assert_eq!(config.min_lines(), 5);
}

#[test]
fn config_with_percentage_thresholds() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("dry4rust.toml"),
        r"
        max_exact_percent = 5.0
        max_near_percent = 10.5
        ",
    )
    .unwrap();
    let config = Config::load(&source_path(tmp.path())).expect("every value is in range");

    // Assert
    assert_eq!(
        config.max_exact_percent().map(Threshold::as_percent),
        Some(5.0)
    );
    assert_eq!(
        config.max_near_percent().map(Threshold::as_percent),
        Some(10.5)
    );
}

#[test]
fn config_with_thresholds() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("dry4rust.toml"),
        r"
        max_exact_duplicates = 0
        max_near_duplicates = 5
        ",
    )
    .unwrap();
    let config = Config::load(&source_path(tmp.path())).expect("every value is in range");

    // Assert
    assert_eq!(config.max_exact_duplicates(), Some(0));
    assert_eq!(config.max_near_duplicates(), Some(5));
}

#[test]
fn default_returns_the_documented_thresholds() {
    // Arrange & Act
    let config = Config::default();

    // Assert
    assert_eq!(config.min_nodes(), 10);
    assert!((config.similarity_threshold().as_fraction() - 0.9).abs() < f64::EPSILON);
    assert!(config.exclude().is_empty());
}

#[test]
fn dry4rust_toml_overrides_cargo_toml() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("Cargo.toml"),
        r#"
        [package]
        name = "test"
        version = "0.1.0"
        edition = "2021"

        [package.metadata.dry4rust]
        min_nodes = 15
        "#,
    )
    .unwrap();
    fs::write(
        tmp.path().join("dry4rust.toml"),
        r"
        min_nodes = 25
        ",
    )
    .unwrap();
    let config = Config::load(&source_path(tmp.path())).expect("every value is in range");

    // Assert
    assert_eq!(config.min_nodes(), 25);
}

#[test]
fn load_from_cargo_toml_metadata() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("Cargo.toml"),
        r#"
        [package]
        name = "test"
        version = "0.1.0"
        edition = "2021"

        [package.metadata.dry4rust]
        min_nodes = 15
        similarity_threshold = 0.75
        "#,
    )
    .unwrap();
    let config = Config::load(&source_path(tmp.path())).expect("every value is in range");

    // Assert
    assert_eq!(config.min_nodes(), 15);
    assert!((config.similarity_threshold().as_fraction() - 0.75).abs() < f64::EPSILON);
}

#[test]
fn load_from_dry4rust_toml() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("dry4rust.toml"),
        r#"
        min_nodes = 20
        similarity_threshold = 0.9
        exclude = ["tests"]
        "#,
    )
    .unwrap();
    let config = Config::load(&source_path(tmp.path())).expect("every value is in range");

    // Assert
    assert_eq!(config.min_nodes(), 20);
    assert!((config.similarity_threshold().as_fraction() - 0.9).abs() < f64::EPSILON);
    assert_eq!(config.exclude(), vec!["tests".to_string()]);
}

#[test]
fn load_no_config_files() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    let config = Config::load(&source_path(tmp.path())).expect("every value is in range");

    // Assert
    assert_eq!(config.min_nodes(), 10); // default
}

#[test]
fn load_over_a_negative_percentage_ceiling_is_rejected() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("dry4rust.toml"),
        "max_near_percent = -5.0\n",
    )
    .unwrap();

    // Act
    let outcome = Config::load(&source_path(tmp.path()));

    // Assert
    let message = outcome
        .expect_err("a negative share of lines is not a share")
        .to_string();
    assert_eq!(
        message,
        "max_near_percent must be a percentage between 0.0 and 100.0, got -5"
    );
}

#[test]
fn load_over_a_similarity_threshold_above_one_names_the_field_and_the_value() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("dry4rust.toml"),
        "similarity_threshold = 5.0\n",
    )
    .unwrap();

    // Act
    let outcome = Config::load(&source_path(tmp.path()));

    // Assert
    let message = outcome
        .expect_err("no pair can score five, so the run would find nothing and say nothing")
        .to_string();
    assert_eq!(
        message,
        "similarity_threshold must be a fraction between 0.0 and 1.0, got 5"
    );
}

#[test]
fn load_over_an_unparseable_file_falls_back_to_the_defaults() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("dry4rust.toml"), "this is not toml = = =\n").unwrap();

    // Act
    let config =
        Config::load(&source_path(tmp.path())).expect("an unreadable file is not a stated value");

    // Assert
    assert_eq!(
        config.min_nodes(),
        Config::default().min_nodes(),
        "a file the tool cannot read looks the same as no file at all; only a \
         value it can read and cannot honour is an error"
    );
}

#[test]
fn with_max_exact_duplicates_sets_the_exact_ceiling() {
    // Arrange
    let config = Config::default();

    // Act
    let config = config.with_max_exact_duplicates(Some(3));

    // Assert
    assert_eq!(config.max_exact_duplicates(), Some(3));
}

#[test]
fn with_max_exact_percent_sets_the_exact_percentage_ceiling() {
    // Arrange
    let ceiling = Threshold::percent("max_exact_percent", 5.0).expect("the ceiling is in range");

    // Act
    let config = Config::default().with_max_exact_percent(Some(ceiling));

    // Assert
    assert_eq!(config.max_exact_percent(), Some(ceiling));
}

#[test]
fn with_max_near_duplicates_sets_the_near_ceiling() {
    // Arrange
    let config = Config::default();

    // Act
    let config = config.with_max_near_duplicates(Some(4));

    // Assert
    assert_eq!(config.max_near_duplicates(), Some(4));
}

#[test]
fn with_max_near_percent_sets_the_near_percentage_ceiling() {
    // Arrange
    let ceiling = Threshold::percent("max_near_percent", 7.5).expect("the ceiling is in range");

    // Act
    let config = Config::default().with_max_near_percent(Some(ceiling));

    // Assert
    assert_eq!(config.max_near_percent(), Some(ceiling));
}

#[test]
fn with_similarity_threshold_sets_the_threshold_near_duplicates_meet() {
    // Arrange
    let threshold =
        Threshold::fraction("similarity_threshold", 0.8).expect("the threshold is in range");

    // Act
    let config = Config::default().with_similarity_threshold(threshold);

    // Assert
    assert_eq!(config.similarity_threshold(), threshold);
}
