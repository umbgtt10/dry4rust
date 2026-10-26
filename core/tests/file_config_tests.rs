// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::config::Config;
use dry4rust::file_config::FileConfig;
use dry4rust::source_path::SourcePath;
use dry4rust::threshold::Threshold;

#[test]
fn apply_to_carries_every_field_the_file_states() {
    // Arrange
    let file = FileConfig::default()
        .with_min_nodes(Some(30))
        .with_similarity_threshold(Some(0.8))
        .with_exclude(Some(vec![String::from("vendor")]))
        .with_max_exact_duplicates(Some(1))
        .with_max_near_duplicates(Some(2))
        .with_max_exact_percent(Some(3.0))
        .with_max_near_percent(Some(4.0))
        .with_min_lines(Some(5))
        .with_exclude_tests(Some(true))
        .with_sub_function(Some(true))
        .with_min_sub_nodes(Some(6))
        .with_baseline(Some(SourcePath::new(String::from("b.json"))));

    // Act
    let config = file
        .apply_to(Config::default())
        .expect("every value is in range");

    // Assert
    assert_eq!(config.min_nodes(), 30);
    assert!((config.similarity_threshold().as_fraction() - 0.8).abs() < f64::EPSILON);
    assert_eq!(config.exclude(), vec![String::from("vendor")]);
    assert_eq!(config.max_exact_duplicates(), Some(1));
    assert_eq!(config.max_near_duplicates(), Some(2));
    assert_eq!(
        config.max_exact_percent().map(Threshold::as_percent),
        Some(3.0)
    );
    assert_eq!(
        config.max_near_percent().map(Threshold::as_percent),
        Some(4.0)
    );
    assert_eq!(config.min_lines(), 5);
    assert!(config.exclude_tests());
    assert!(config.sub_function());
    assert_eq!(config.min_sub_nodes(), 6);
    assert_eq!(
        config.baseline(),
        Some(&SourcePath::new(String::from("b.json")))
    );
}

#[test]
fn apply_to_changes_only_the_fields_the_file_states() {
    // Arrange
    let config = Config::default().with_min_nodes(7).with_min_lines(3);

    // Act
    let config = FileConfig::default()
        .with_min_nodes(Some(25))
        .apply_to(config)
        .expect("25 is in range");

    // Assert
    assert_eq!(config.min_nodes(), 25);
    assert_eq!(config.min_lines(), 3, "a file says nothing about the rest");
}

#[test]
fn apply_to_of_an_empty_file_changes_nothing() {
    // Arrange
    let config = Config::default().with_min_nodes(7).with_sub_function(true);

    // Act
    let config = FileConfig::default()
        .apply_to(config)
        .expect("nothing stated is always in range");

    // Assert
    assert_eq!(config.min_nodes(), 7);
    assert!(config.sub_function());
}

#[test]
fn apply_to_reads_a_baseline_path_as_written() {
    // Arrange & Act
    let config = FileConfig::default()
        .with_baseline(Some(SourcePath::new(String::from("ci/recorded.json"))))
        .apply_to(Config::default())
        .expect("a path is always in range");

    // Assert
    assert_eq!(
        config.baseline(),
        Some(&SourcePath::new(String::from("ci/recorded.json")))
    );
}

#[test]
fn apply_to_rejects_a_percentage_outside_a_hundred() {
    // Arrange & Act
    let outcome = FileConfig::default()
        .with_max_near_percent(Some(150.0))
        .apply_to(Config::default());

    // Assert
    assert_eq!(
        outcome.unwrap_err().to_string(),
        "max_near_percent must be a percentage between 0.0 and 100.0, got 150"
    );
}

#[test]
fn apply_to_rejects_a_similarity_threshold_outside_one() {
    // Arrange & Act
    let outcome = FileConfig::default()
        .with_similarity_threshold(Some(5.0))
        .apply_to(Config::default());

    // Assert
    assert_eq!(
        outcome.unwrap_err().to_string(),
        "similarity_threshold must be a fraction between 0.0 and 1.0, got 5"
    );
}

#[test]
fn apply_to_replaces_the_excludes_rather_than_appending_to_them() {
    // Arrange
    let config = Config::default().with_exclude(vec![String::from("vendor")]);

    // Act
    let config = FileConfig::default()
        .with_exclude(Some(vec![String::from("tests")]))
        .apply_to(config)
        .expect("a list is always in range");

    // Assert
    assert_eq!(
        config.exclude(),
        vec![String::from("tests")],
        "a config file states the whole list, unlike --exclude, which adds to it"
    );
}

#[test]
fn apply_to_takes_a_percentage_as_a_share_of_a_hundred() {
    // Arrange & Act
    let config = FileConfig::default()
        .with_max_exact_percent(Some(5.0))
        .apply_to(Config::default())
        .expect("5% is in range");

    // Assert
    assert_eq!(
        config.max_exact_percent().map(Threshold::as_percent),
        Some(5.0)
    );
}
