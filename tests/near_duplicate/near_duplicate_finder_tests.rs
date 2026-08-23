// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::code_unit::CodeUnit;
use dry4rust::code_unit::CodeUnitKind;
use dry4rust::fingerprint::Fingerprint;
use dry4rust::grouper::DuplicateGroup;
use dry4rust::near_duplicate::near_duplicate_finder::NearDuplicateFinder;
use dry4rust::node::NodeKind;
use dry4rust::node::NormalizedNode;
use dry4rust::rust::parser::parse_file;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

const NEAR_PAIR: &str = r"
fn alpha(items: &[i32]) -> i32 {
    let mut total = 0;
    for item in items {
        if *item > 0 {
            total += item;
        }
    }
    total
}

fn beta(values: &[i32]) -> i32 {
    let mut sum = 0;
    for value in values {
        if *value > 1 {
            sum += value;
        }
    }
    sum
}
";

fn sized_unit(name: &str, node_count: usize) -> CodeUnit {
    let children = vec![NormalizedNode::leaf(NodeKind::Block); node_count - 1];
    let body = NormalizedNode::with_children(NodeKind::Block, children);
    CodeUnit {
        kind: CodeUnitKind::Function,
        name: name.to_string(),
        file: PathBuf::from("sized.rs"),
        line_start: 1,
        line_end: 2,
        signature: NormalizedNode::leaf(NodeKind::Block),
        body,
        fingerprint: Fingerprint::new(node_count as u64),
        node_count,
        parent_name: None,
        is_test: false,
    }
}

fn units_from(code: &str) -> Vec<CodeUnit> {
    let tmp = TempDir::new().expect("temp dir");
    let file = tmp.path().join("sample.rs");
    fs::write(&file, code).expect("write");
    parse_file(&file, 1, 0).expect("the sample parses")
}

#[test]
fn find_excluding_every_fingerprint_returns_nothing() {
    // Arrange
    let units = units_from(NEAR_PAIR);
    let all: Vec<Fingerprint> = units.iter().map(|u| u.fingerprint).collect();

    // Act
    let groups = NearDuplicateFinder::new(0.5).find(&units, &all);

    // Assert
    assert!(
        groups.is_empty(),
        "units already reported as exact duplicates are not candidates"
    );
}

#[test]
fn find_groups_a_pair_whose_sizes_straddle_a_power_of_two() {
    // Arrange
    let units = vec![sized_unit("seven", 7), sized_unit("eight", 8)];

    // Act
    let groups = NearDuplicateFinder::new(0.8).find(&units, &[]);

    // Assert
    assert_eq!(
        groups.len(),
        1,
        "7 and 8 nodes are one apart and score 0.933; bucketing by log2 put a \
         boundary between them and lost the pair"
    );
    assert!((groups[0].similarity - 14.0 / 15.0).abs() < 1e-9);
}

#[test]
fn find_keeps_a_pair_whose_ceiling_lands_exactly_on_the_threshold() {
    // Arrange
    let units = vec![sized_unit("four", 4), sized_unit("six", 6)];

    // Act
    let groups = NearDuplicateFinder::new(0.8).find(&units, &[]);

    // Assert
    assert_eq!(
        groups.len(),
        1,
        "4 and 6 nodes reach exactly 0.8, and the scorer admits a score equal \
         to the threshold, so the size filter must not discard the pair first"
    );
}

#[test]
fn find_over_a_single_unit_returns_nothing() {
    // Arrange
    let units = units_from("fn only(x: i32) -> i32 { x + 1 }");

    // Act
    let groups = NearDuplicateFinder::new(0.1).find(&units, &[]);

    // Assert
    assert!(
        groups.is_empty(),
        "one unit cannot be a duplicate of anything"
    );
}

#[test]
fn find_over_no_units_returns_nothing() {
    // Arrange
    let units: Vec<CodeUnit> = Vec::new();

    // Act
    let groups = NearDuplicateFinder::new(0.8).find(&units, &[]);

    // Assert
    assert!(groups.is_empty());
}

#[test]
fn find_reports_a_similarity_no_higher_than_the_weakest_pair() {
    // Arrange
    let units = units_from(NEAR_PAIR);

    // Act
    let groups = NearDuplicateFinder::new(0.5).find(&units, &[]);

    // Assert
    for group in &groups {
        assert!(
            group.similarity >= 0.5,
            "every reported group cleared the threshold, got {}",
            group.similarity
        );
        assert!(group.similarity <= 1.0);
    }
}

#[test]
fn find_run_twice_over_the_same_units_returns_the_same_groups() {
    // Arrange
    let units = units_from(NEAR_PAIR);
    let finder = NearDuplicateFinder::new(0.5);

    // Act
    let first = finder.find(&units, &[]);
    let second = finder.find(&units, &[]);

    // Assert
    let shape = |groups: &[DuplicateGroup]| -> Vec<(usize, u64)> {
        groups
            .iter()
            .map(|g| (g.members.len(), g.fingerprint.value()))
            .collect()
    };
    assert_eq!(
        shape(&first),
        shape(&second),
        "grouping must not depend on hash iteration order"
    );
}

#[test]
fn find_skips_a_pair_too_far_apart_in_size_to_reach_the_threshold() {
    // Arrange
    let units = vec![sized_unit("eight", 8), sized_unit("fifteen", 15)];

    // Act
    let groups = NearDuplicateFinder::new(0.8).find(&units, &[]);

    // Assert
    assert!(
        groups.is_empty(),
        "the best 8 and 15 nodes can score is 0.696, below the threshold"
    );
}

#[test]
fn find_with_a_permissive_threshold_groups_the_two_similar_functions() {
    // Arrange
    let units = units_from(NEAR_PAIR);

    // Act
    let groups = NearDuplicateFinder::new(0.5).find(&units, &[]);

    // Assert
    assert!(
        groups.iter().any(|g| g.members.len() >= 2),
        "two functions differing only in names and one literal are near duplicates"
    );
}

#[test]
fn find_with_a_threshold_of_one_groups_only_what_is_identical() {
    // Arrange
    let units = units_from(NEAR_PAIR);

    // Act
    let groups = NearDuplicateFinder::new(1.0).find(&units, &[]);

    // Assert
    for group in &groups {
        assert!(
            (group.similarity - 1.0).abs() < f64::EPSILON,
            "a threshold of 1.0 admits nothing below 1.0, got {}",
            group.similarity
        );
    }
}

#[test]
fn find_with_an_unreachable_threshold_returns_nothing() {
    // Arrange
    let units = units_from(NEAR_PAIR);

    // Act
    let groups = NearDuplicateFinder::new(1.1).find(&units, &[]);

    // Assert
    assert!(
        groups.is_empty(),
        "no score exceeds 1.0, so nothing clears 1.1"
    );
}

#[test]
fn new_records_the_threshold_that_find_then_applies() {
    // Arrange
    let units = units_from(NEAR_PAIR);

    // Act
    let permissive = NearDuplicateFinder::new(0.1).find(&units, &[]);
    let strict = NearDuplicateFinder::new(1.1).find(&units, &[]);

    // Assert
    assert!(!permissive.is_empty());
    assert!(strict.is_empty());
}
