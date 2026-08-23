// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::grouper::*;

#[test]
fn empty_input_no_groups() {
    // Arrange & Act
    let groups = group_exact_duplicates(&[]);

    // Assert
    assert!(groups.is_empty());
}

#[test]
fn percentage_helpers() {
    // Arrange & Act
    let stats = DuplicationStats {
        total_code_units: 10,
        total_lines: 200,
        exact_duplicate_groups: 2,
        exact_duplicate_units: 4,
        near_duplicate_groups: 1,
        near_duplicate_units: 3,
        exact_duplicate_lines: 50,
        near_duplicate_lines: 30,
        sub_exact_groups: 0,
        sub_exact_units: 0,
        sub_near_groups: 0,
        sub_near_units: 0,
    };

    // Assert
    assert!((stats.exact_duplicate_percent() - 25.0).abs() < f64::EPSILON);
    assert!((stats.near_duplicate_percent() - 15.0).abs() < f64::EPSILON);
}

#[test]
fn percentage_helpers_zero_total() {
    // Arrange & Act
    let stats = DuplicationStats {
        total_code_units: 0,
        total_lines: 0,
        exact_duplicate_groups: 0,
        exact_duplicate_units: 0,
        near_duplicate_groups: 0,
        near_duplicate_units: 0,
        exact_duplicate_lines: 0,
        near_duplicate_lines: 0,
        sub_exact_groups: 0,
        sub_exact_units: 0,
        sub_near_groups: 0,
        sub_near_units: 0,
    };

    // Assert
    assert!((stats.exact_duplicate_percent() - 0.0).abs() < f64::EPSILON);
    assert!((stats.near_duplicate_percent() - 0.0).abs() < f64::EPSILON);
}
