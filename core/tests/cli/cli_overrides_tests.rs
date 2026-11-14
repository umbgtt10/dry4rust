// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::config::Config;
use dry4rust::source_path::SourcePath;

#[test]
fn apply_to_appends_excludes_rather_than_replacing_the_configured_ones() {
    // Arrange
    let config = Config::default().with_exclude(vec![String::from("vendor")]);
    let overrides = CliOverrides::default().with_exclude(vec![String::from("benches")]);

    // Act
    let config = overrides
        .apply_to(config)
        .expect("the overrides are in range");

    // Assert
    assert_eq!(
        config.exclude(),
        vec![String::from("vendor"), String::from("benches")],
        "a --exclude on the command line adds to the config file, it does not stand in for it"
    );
}

#[test]
fn apply_to_replaces_only_the_values_that_were_given() {
    // Arrange
    let config = Config::default();
    let untouched_threshold = config.similarity_threshold().as_fraction();
    let overrides = CliOverrides::default().with_min_nodes(Some(42));

    // Act
    let config = overrides
        .apply_to(config)
        .expect("the overrides are in range");

    // Assert
    assert_eq!(config.min_nodes(), 42);
    assert!(
        (config.similarity_threshold().as_fraction() - untouched_threshold).abs() < f64::EPSILON
    );
}

#[test]
fn apply_to_with_a_threshold_sets_the_similarity_threshold() {
    // Arrange
    let overrides = CliOverrides::default().with_threshold(Some(0.75));

    // Act
    let config = overrides
        .apply_to(Config::default())
        .expect("the threshold is in range");

    // Assert
    assert!((config.similarity_threshold().as_fraction() - 0.75).abs() < f64::EPSILON);
}

// Each override given stands in for the configured value: every one set here
// differs from the config it is applied to.
#[test]
fn apply_to_with_every_override_given_sets_each_one() {
    // Arrange
    let config = Config::default()
        .with_min_lines(3)
        .with_min_sub_nodes(11)
        .with_exclude_tests(false)
        .with_sub_function(false);
    let overrides = CliOverrides::default()
        .with_min_lines(Some(4))
        .with_min_sub_nodes(Some(12))
        .with_exclude_tests(Some(true))
        .with_sub_function(Some(true))
        .with_baseline(Some(SourcePath::new(String::from("recorded.json"))));

    // Act
    let config = overrides
        .apply_to(config)
        .expect("the overrides are in range");

    // Assert
    assert_eq!(config.min_lines(), 4);
    assert_eq!(config.min_sub_nodes(), 12);
    assert!(config.exclude_tests());
    assert!(config.sub_function());
    assert_eq!(
        config.baseline().map(SourcePath::as_str),
        Some("recorded.json")
    );
}

#[test]
fn apply_to_with_nothing_set_leaves_every_field_as_it_was() {
    // Arrange
    let config = Config::default()
        .with_min_nodes(7)
        .with_min_lines(3)
        .with_sub_function(true)
        .with_min_sub_nodes(11)
        .with_exclude_tests(true);

    // Act
    let config = CliOverrides::default()
        .apply_to(config)
        .expect("no override is always in range");

    // Assert
    assert_eq!(config.min_nodes(), 7);
    assert_eq!(config.min_lines(), 3);
    assert_eq!(config.min_sub_nodes(), 11);
    assert!(config.sub_function());
    assert!(config.exclude_tests());
}
