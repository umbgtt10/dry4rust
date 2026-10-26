// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_config_source::InMemoryConfigSource;
use dry4rust::config::Config;
use dry4rust::file_config::FileConfig;
use dry4rust::source_path::SourcePath;
use dry4rust::threshold::Threshold;

fn root() -> SourcePath {
    SourcePath::new(String::from("/p"))
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
fn load_over_a_stated_value_out_of_range_is_rejected() {
    // Arrange
    let source = InMemoryConfigSource::new().with_file_config(
        "/p/dry4rust.toml",
        FileConfig::default().with_max_near_percent(Some(-5.0)),
    );

    // Act
    let outcome = Config::load(&source, &root());

    // Assert
    assert_eq!(
        outcome
            .expect_err("a negative share of lines is not a share")
            .to_string(),
        "max_near_percent must be a percentage between 0.0 and 100.0, got -5"
    );
}

#[test]
fn load_prefers_the_named_file_over_the_package_metadata() {
    // Arrange
    let source = InMemoryConfigSource::new()
        .with_package_metadata(
            "/p/Cargo.toml",
            FileConfig::default()
                .with_min_nodes(Some(15))
                .with_min_lines(Some(4)),
        )
        .with_file_config(
            "/p/dry4rust.toml",
            FileConfig::default().with_min_nodes(Some(25)),
        );

    // Act
    let config = Config::load(&source, &root()).expect("every value is in range");

    // Assert
    assert_eq!(config.min_nodes(), 25);
    assert_eq!(
        config.min_lines(),
        4,
        "what the named file leaves alone, the metadata keeps"
    );
}

#[test]
fn load_reads_each_file_beside_the_root_it_was_given() {
    // Arrange
    let source = InMemoryConfigSource::new()
        .with_package_metadata(
            "/elsewhere/Cargo.toml",
            FileConfig::default().with_min_nodes(Some(15)),
        )
        .with_file_config(
            "/elsewhere/dry4rust.toml",
            FileConfig::default().with_min_lines(Some(4)),
        );

    // Act
    let config = Config::load(&source, &root()).expect("every value is in range");

    // Assert
    assert_eq!(config.min_nodes(), Config::default().min_nodes());
    assert_eq!(config.min_lines(), Config::default().min_lines());
}

#[test]
fn load_with_no_file_returns_the_defaults_at_the_root_it_was_given() {
    // Arrange
    let source = InMemoryConfigSource::new();

    // Act
    let config = Config::load(&source, &root()).expect("nothing stated is always in range");

    // Assert
    assert_eq!(config.min_nodes(), 10);
    assert_eq!(config.root(), &root());
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
