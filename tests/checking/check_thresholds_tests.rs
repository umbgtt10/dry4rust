// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::checking::check_thresholds::CheckThresholds;

#[test]
fn default_sets_no_ceiling_at_all() {
    // Arrange & Act
    let thresholds = CheckThresholds::default();

    // Assert
    assert_eq!(thresholds.max_exact, None);
    assert_eq!(thresholds.max_near, None);
    assert_eq!(thresholds.max_exact_percent, None);
    assert_eq!(thresholds.max_near_percent, None);
}

#[test]
fn is_unbounded_is_false_once_a_count_ceiling_is_set() {
    // Arrange
    let thresholds = CheckThresholds {
        max_exact: Some(0),
        ..CheckThresholds::default()
    };

    // Act & Assert
    assert!(!thresholds.is_unbounded());
}

#[test]
fn is_unbounded_is_false_once_a_percentage_ceiling_is_set() {
    // Arrange
    let thresholds = CheckThresholds {
        max_near_percent: Some(0.0),
        ..CheckThresholds::default()
    };

    // Act & Assert
    assert!(
        !thresholds.is_unbounded(),
        "zero is a ceiling; absent is not"
    );
}

#[test]
fn is_unbounded_of_a_default_reports_a_check_that_cannot_fail() {
    // Arrange & Act
    let thresholds = CheckThresholds::default();

    // Assert
    assert!(
        thresholds.is_unbounded(),
        "running check with no flags reports and exits zero however much it finds"
    );
}
