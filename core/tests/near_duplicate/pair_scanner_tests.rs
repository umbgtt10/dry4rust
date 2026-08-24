// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::code_unit::CodeUnit;
use dry4rust::code_unit::CodeUnitKind;
use dry4rust::fingerprint::Fingerprint;
use dry4rust::near_duplicate::pair_scanner::PairScanner;
use dry4rust::near_duplicate::similarity_pair::SimilarityPair;
use dry4rust::node::NodeKind;
use dry4rust::node::NormalizedNode;
use std::path::PathBuf;

fn sized_unit(kind: CodeUnitKind, node_count: usize) -> CodeUnit {
    let children = vec![NormalizedNode::leaf(NodeKind::Block); node_count - 1];
    CodeUnit {
        kind,
        name: format!("unit_of_{node_count}"),
        file: PathBuf::from("sized.rs"),
        line_start: 1,
        line_end: 2,
        signature: NormalizedNode::leaf(NodeKind::Block),
        body: NormalizedNode::with_children(NodeKind::Block, children),
        fingerprint: Fingerprint::new(node_count as u64),
        node_count,
        parent_name: None,
        is_test: false,
    }
}

#[test]
fn could_reach_threshold_for_a_pair_straddling_a_power_of_two_is_true() {
    // Arrange
    let scanner = PairScanner::new(0.8);

    // Act
    let reachable = scanner.could_reach_threshold(7, 8);

    // Assert
    assert!(
        reachable,
        "7 and 8 nodes have a ceiling of 0.933; a log2 banding rejected this pair"
    );
}

#[test]
fn could_reach_threshold_for_a_pair_too_far_apart_is_false() {
    // Arrange
    let scanner = PairScanner::new(0.8);

    // Act
    let reachable = scanner.could_reach_threshold(8, 15);

    // Assert
    assert!(!reachable, "the ceiling for 8 and 15 nodes is 0.696");
}

#[test]
fn could_reach_threshold_for_two_empty_trees_is_true() {
    // Arrange
    let scanner = PairScanner::new(1.0);

    // Act
    let reachable = scanner.could_reach_threshold(0, 0);

    // Assert
    assert!(reachable, "two empty trees score 1.0 by definition");
}

#[test]
fn could_reach_threshold_on_the_exact_boundary_is_true() {
    // Arrange
    let scanner = PairScanner::new(0.8);

    // Act
    let reachable = scanner.could_reach_threshold(4, 6);

    // Assert
    assert!(
        reachable,
        "4 and 6 nodes reach exactly 0.8, and a score equal to the threshold is admitted"
    );
}

#[test]
fn could_reach_threshold_one_node_past_the_boundary_is_false() {
    // Arrange
    let scanner = PairScanner::new(0.8);

    // Act
    let reachable = scanner.could_reach_threshold(4, 7);

    // Assert
    assert!(!reachable, "4 and 7 nodes reach only 0.727");
}

#[test]
fn could_reach_threshold_with_equal_sizes_is_always_true() {
    // Arrange
    let scanner = PairScanner::new(1.0);

    // Act
    let reachable = scanner.could_reach_threshold(12, 12);

    // Assert
    assert!(
        reachable,
        "equal sizes have a ceiling of 1.0 at any threshold"
    );
}

#[test]
fn new_records_the_threshold_that_could_reach_threshold_then_applies() {
    // Arrange & Act
    let permissive = PairScanner::new(0.5);
    let strict = PairScanner::new(0.95);

    // Assert
    assert!(permissive.could_reach_threshold(8, 15));
    assert!(!strict.could_reach_threshold(8, 15));
}

#[test]
fn scan_over_a_single_candidate_finds_no_pairs() {
    // Arrange
    let only = sized_unit(CodeUnitKind::Function, 8);
    let candidates = vec![&only];

    // Act
    let pairs = PairScanner::new(0.8).scan(&candidates);

    // Assert
    assert!(pairs.is_empty());
}

#[test]
fn scan_pairs_a_function_with_a_method_whose_body_matches() {
    // Arrange
    let function = sized_unit(CodeUnitKind::Function, 8);
    let method = sized_unit(CodeUnitKind::Method, 8);
    let candidates = vec![&function, &method];

    // Act
    let pairs = PairScanner::new(0.5).scan(&candidates);

    // Assert
    assert_eq!(
        pairs.len(),
        1,
        "group_exact_duplicates already reports this pair when the bodies are \
         identical; kind must not hide it when they merely match closely"
    );
    assert!((pairs[0].score - 1.0).abs() < f64::EPSILON);
}

#[test]
fn scan_pairs_sub_function_units_of_different_kinds() {
    // Arrange
    let branch = sized_unit(CodeUnitKind::IfBranch, 8);
    let arm = sized_unit(CodeUnitKind::MatchArm, 8);
    let candidates = vec![&branch, &arm];

    // Act
    let pairs = PairScanner::new(0.5).scan(&candidates);

    // Assert
    assert_eq!(
        pairs.len(),
        1,
        "a block of logic repeated as an if-branch and as a match arm is the \
         duplication a reader would want extracted, not noise"
    );
}

#[test]
fn scan_pairs_units_whose_sizes_straddle_a_power_of_two() {
    // Arrange
    let seven = sized_unit(CodeUnitKind::Function, 7);
    let eight = sized_unit(CodeUnitKind::Function, 8);
    let candidates = vec![&seven, &eight];

    // Act
    let pairs = PairScanner::new(0.8).scan(&candidates);

    // Assert
    assert_eq!(pairs.len(), 1);
    assert!((pairs[0].score - 14.0 / 15.0).abs() < 1e-9);
}

#[test]
fn scan_skips_units_too_far_apart_in_size() {
    // Arrange
    let eight = sized_unit(CodeUnitKind::Function, 8);
    let fifteen = sized_unit(CodeUnitKind::Function, 15);
    let candidates = vec![&eight, &fifteen];

    // Act
    let pairs = PairScanner::new(0.8).scan(&candidates);

    // Assert
    assert!(pairs.is_empty());
}

#[test]
fn scan_stops_at_the_first_partner_too_large_rather_than_the_last() {
    // Arrange
    let four = sized_unit(CodeUnitKind::Function, 4);
    let six = sized_unit(CodeUnitKind::Function, 6);
    let ten = sized_unit(CodeUnitKind::Function, 10);
    let candidates = vec![&four, &six, &ten];

    // Act
    let pairs = PairScanner::new(0.8).scan(&candidates);

    // Assert
    let partners: Vec<(usize, usize)> = pairs.iter().map(SimilarityPair::key).collect();
    assert_eq!(
        partners,
        vec![(0, 1)],
        "4 pairs with 6 and stops; 6 and 10 have a ceiling of 0.75"
    );
}
