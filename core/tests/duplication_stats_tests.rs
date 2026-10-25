// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::code_unit::CodeUnit;
use dry4rust::duplicate_group::DuplicateGroup;
use dry4rust::duplication_stats::DuplicationStats;

#[test]
fn duplication_stats_percentages_match_their_counts() {
    // Arrange & Act
    let stats = DuplicationStats::default()
        .with_total_code_units(10)
        .with_total_lines(200)
        .with_exact_duplicate_groups(2)
        .with_exact_duplicate_units(4)
        .with_near_duplicate_groups(1)
        .with_near_duplicate_units(3)
        .with_exact_duplicate_lines(50)
        .with_near_duplicate_lines(30);

    // Assert
    assert!((stats.exact_duplicate_percent() - 25.0).abs() < f64::EPSILON);
    assert!((stats.near_duplicate_percent() - 15.0).abs() < f64::EPSILON);
}

#[test]
fn percentage_helpers_zero_total() {
    // Arrange & Act
    let stats = DuplicationStats::default();

    // Assert
    assert!((stats.exact_duplicate_percent() - 0.0).abs() < f64::EPSILON);
    assert!((stats.near_duplicate_percent() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn with_baseline_suppressed_distinguishes_none_suppressed_from_no_baseline() {
    // Arrange
    let units: Vec<CodeUnit> = Vec::new();
    let none: Vec<DuplicateGroup> = Vec::new();
    let stats = DuplicateGroup::compute_stats_with_sub(&units, &none, &none, &none, &none);

    // Act
    let judged = stats.clone().with_baseline_suppressed(Some(0));
    let unjudged = stats.with_baseline_suppressed(None);

    // Assert
    assert_eq!(
        judged.baseline_suppressed(),
        Some(0),
        "a baseline that suppressed nothing is still a baseline that was applied"
    );
    assert_eq!(
        unjudged.baseline_suppressed(),
        None,
        "and no baseline at all is a different fact"
    );
}

#[test]
fn with_near_duplicate_units_sets_the_units_in_near_groups() {
    // Arrange
    let stats = DuplicationStats::default();

    // Act
    let stats = stats.with_near_duplicate_units(4);

    // Assert
    assert_eq!(stats.near_duplicate_units(), 4);
}

#[test]
fn with_sub_exact_groups_sets_the_sub_function_exact_groups() {
    // Arrange
    let stats = DuplicationStats::default();

    // Act
    let stats = stats.with_sub_exact_groups(2);

    // Assert
    assert_eq!(stats.sub_exact_groups(), 2);
}

#[test]
fn with_sub_exact_units_sets_the_units_in_sub_function_exact_groups() {
    // Arrange
    let stats = DuplicationStats::default();

    // Act
    let stats = stats.with_sub_exact_units(5);

    // Assert
    assert_eq!(stats.sub_exact_units(), 5);
}

#[test]
fn with_sub_near_groups_sets_the_sub_function_near_groups() {
    // Arrange
    let stats = DuplicationStats::default();

    // Act
    let stats = stats.with_sub_near_groups(3);

    // Assert
    assert_eq!(stats.sub_near_groups(), 3);
}

#[test]
fn with_sub_near_units_sets_the_units_in_sub_function_near_groups() {
    // Arrange
    let stats = DuplicationStats::default();

    // Act
    let stats = stats.with_sub_near_units(6);

    // Assert
    assert_eq!(stats.sub_near_units(), 6);
}
