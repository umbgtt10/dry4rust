// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::code_unit::CodeUnit;
use dry4rust::grouper::DuplicateGroup;
use dry4rust::grouper::DuplicationStats;
use dry4rust::grouper::compute_stats_with_sub;
use dry4rust::grouper::find_near_duplicates;
use dry4rust::grouper::group_exact_duplicates;
use dry4rust::rust::parser::parse_file;
use std::fs;
use tempfile::TempDir;
#[test]
fn compute_stats_with_sub_records_the_sub_function_group_counts() {
    // Arrange
    let units: Vec<CodeUnit> = Vec::new();
    let none: Vec<DuplicateGroup> = Vec::new();

    // Act
    let stats = compute_stats_with_sub(&units, &none, &none, &none, &none);

    // Assert
    assert_eq!(stats.sub_exact_groups, 0);
    assert_eq!(stats.sub_near_groups, 0);
    assert_eq!(stats.total_code_units, 0);
}

#[test]
fn duplication_stats_percentages_match_their_counts() {
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
fn empty_input_no_groups() {
    // Arrange & Act
    let groups = group_exact_duplicates(&[]);

    // Assert
    assert!(groups.is_empty());
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

#[test]
fn find_near_duplicates_with_no_units_finds_nothing() {
    // Arrange
    let units: Vec<CodeUnit> = Vec::new();

    // Act
    let groups = find_near_duplicates(&units, 0.9, &[]);

    // Assert
    assert!(groups.is_empty());
}

#[test]
fn find_near_duplicates_with_a_threshold_of_one_demands_exactness() {
    // Arrange
    let units: Vec<CodeUnit> = Vec::new();

    // Act
    let groups = find_near_duplicates(&units, 1.0, &[]);

    // Assert
    assert!(groups.is_empty());
}

fn units_from(code: &str) -> Vec<CodeUnit> {
    let tmp = TempDir::new().expect("temp dir");
    let file = tmp.path().join("sample.rs");
    fs::write(&file, code).expect("write");
    parse_file(&file, 1, 0).expect("the sample parses")
}

#[test]
fn find_near_duplicates_groups_two_similar_but_not_identical_functions() {
    // Arrange -- same shape, one extra statement, so exact matching misses them
    let units = units_from(
        "fn a(x: i32) -> i32 { let y = x + 1; let z = y * 2; z }\n\
         fn b(p: i32) -> i32 { let q = p + 1; let r = q * 2; let s = r; s }\n",
    );

    // Act
    let groups = find_near_duplicates(&units, 0.5, &[]);

    // Assert
    assert!(!units.is_empty());
    assert!(!groups.is_empty(), "expected a near-duplicate group");
}

#[test]
fn find_near_duplicates_skips_units_already_matched_exactly() {
    // Arrange
    let units = units_from("fn a(x: i32) -> i32 { x + 1 }\nfn b(y: i32) -> i32 { y + 1 }\n");
    let exact: Vec<_> = units.iter().map(|u| u.fingerprint).collect();

    // Act
    let groups = find_near_duplicates(&units, 0.5, &exact);

    // Assert
    assert!(
        groups.is_empty(),
        "exact matches must not be reported again"
    );
}

#[test]
fn find_near_duplicates_with_a_single_candidate_returns_early() {
    // Arrange -- fewer than two candidates cannot form a pair
    let units = units_from("fn only(x: i32) -> i32 { x + 1 }\n");

    // Act
    let groups = find_near_duplicates(&units, 0.5, &[]);

    // Assert
    assert!(groups.is_empty());
}

#[test]
fn find_near_duplicates_below_the_threshold_reports_no_group() {
    // Arrange -- two functions of quite different shape
    let units = units_from(
        "fn a(x: i32) -> i32 { x + 1 }\n\
         fn b(p: i32) -> i32 { let q = p * 2; let r = q + 3; let s = r - 4; let t = s / 5; t }\n",
    );

    // Act
    let groups = find_near_duplicates(&units, 0.99, &[]);

    // Assert
    assert!(groups.is_empty(), "a 0.99 threshold should reject these");
}
