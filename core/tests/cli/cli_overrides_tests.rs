// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::config::Config;

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
