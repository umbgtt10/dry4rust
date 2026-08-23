// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::code_unit::CodeUnitKind;
use dry4rust::node::NodeKind;
use dry4rust::node::NormalizedNode;
use dry4rust::sub_unit_extractor::SubUnitExtractor;

fn arm(guard: NormalizedNode, arm_body: NormalizedNode) -> NormalizedNode {
    NormalizedNode::with_children(
        NodeKind::MatchArm,
        vec![NormalizedNode::leaf(NodeKind::None), guard, arm_body],
    )
}

fn body(size: usize) -> NormalizedNode {
    NormalizedNode::with_children(
        NodeKind::Block,
        vec![NormalizedNode::leaf(NodeKind::None); size],
    )
}

#[test]
fn extract_from_a_closure_takes_the_first_child_as_the_body() {
    // Arrange
    let closure = NormalizedNode::with_children(
        NodeKind::Closure,
        vec![body(4), NormalizedNode::leaf(NodeKind::None)],
    );

    // Act
    let units = SubUnitExtractor::new(1).extract(&closure);

    // Assert
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].description, "closure body");
    assert_eq!(units[0].kind, CodeUnitKind::Block);
}

#[test]
fn extract_from_a_for_loop_takes_the_third_child_as_the_body() {
    // Arrange
    let for_loop = NormalizedNode::with_children(
        NodeKind::ForLoop,
        vec![
            NormalizedNode::leaf(NodeKind::None),
            NormalizedNode::leaf(NodeKind::None),
            body(4),
        ],
    );

    // Act
    let units = SubUnitExtractor::new(1).extract(&for_loop);

    // Assert
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].description, "for body");
    assert_eq!(units[0].kind, CodeUnitKind::LoopBody);
}

#[test]
fn extract_from_a_loop_takes_the_first_child_as_the_body() {
    // Arrange
    let loop_node = NormalizedNode::with_children(NodeKind::Loop, vec![body(4)]);

    // Act
    let units = SubUnitExtractor::new(1).extract(&loop_node);

    // Assert
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].description, "loop body");
    assert_eq!(units[0].kind, CodeUnitKind::LoopBody);
}

#[test]
fn extract_from_a_match_numbers_its_arms_from_one() {
    // Arrange
    let none = NormalizedNode::leaf(NodeKind::None);
    let match_node = NormalizedNode::with_children(
        NodeKind::Match,
        vec![
            none.clone(),
            arm(none.clone(), body(4)),
            arm(none.clone(), body(5)),
            arm(none, body(6)),
        ],
    );

    // Act
    let units = SubUnitExtractor::new(1).extract(&match_node);

    // Assert
    let descriptions: Vec<&str> = units.iter().map(|u| u.description.as_str()).collect();
    assert_eq!(
        descriptions,
        vec!["match arm 1", "match arm 2", "match arm 3"]
    );
}

#[test]
fn extract_from_a_match_skips_an_arm_that_has_no_body() {
    // Arrange
    let none = NormalizedNode::leaf(NodeKind::None);
    let bodiless = NormalizedNode::with_children(NodeKind::MatchArm, vec![none.clone()]);
    let match_node = NormalizedNode::with_children(
        NodeKind::Match,
        vec![none.clone(), bodiless, arm(none, body(4))],
    );

    // Act
    let units = SubUnitExtractor::new(1).extract(&match_node);

    // Assert
    assert_eq!(units.len(), 1);
    assert_eq!(
        units[0].description, "match arm 2",
        "the surviving arm keeps its own position, not a renumbered one"
    );
}

#[test]
fn extract_from_a_while_takes_the_second_child_as_the_body() {
    // Arrange
    let while_node = NormalizedNode::with_children(
        NodeKind::While,
        vec![NormalizedNode::leaf(NodeKind::None), body(4)],
    );

    // Act
    let units = SubUnitExtractor::new(1).extract(&while_node);

    // Assert
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].description, "while body");
}

#[test]
fn extract_from_an_if_with_a_real_else_returns_both_branches_in_order() {
    // Arrange
    let if_node = NormalizedNode::with_children(
        NodeKind::If,
        vec![NormalizedNode::leaf(NodeKind::None), body(4), body(5)],
    );

    // Act
    let units = SubUnitExtractor::new(1).extract(&if_node);

    // Assert
    let descriptions: Vec<&str> = units.iter().map(|u| u.description.as_str()).collect();
    assert_eq!(descriptions, vec!["if-then branch", "if-else branch"]);
}

#[test]
fn extract_from_an_if_without_an_else_returns_only_the_then_branch() {
    // Arrange
    let if_node = NormalizedNode::with_children(
        NodeKind::If,
        vec![
            NormalizedNode::leaf(NodeKind::None),
            body(4),
            NormalizedNode::leaf(NodeKind::None),
        ],
    );

    // Act
    let units = SubUnitExtractor::new(1).extract(&if_node);

    // Assert
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].description, "if-then branch");
}

#[test]
fn extract_over_a_nested_tree_reports_the_outer_body_before_the_inner_one() {
    // Arrange
    let inner = NormalizedNode::with_children(NodeKind::Loop, vec![body(4)]);
    let outer = NormalizedNode::with_children(
        NodeKind::While,
        vec![NormalizedNode::leaf(NodeKind::None), inner],
    );

    // Act
    let units = SubUnitExtractor::new(1).extract(&outer);

    // Assert
    let descriptions: Vec<&str> = units.iter().map(|u| u.description.as_str()).collect();
    assert_eq!(
        descriptions,
        vec!["while body", "loop body"],
        "a pre-order walk names the enclosing body first"
    );
}

#[test]
fn extract_with_a_floor_above_the_body_size_returns_nothing() {
    // Arrange
    let loop_node = NormalizedNode::with_children(NodeKind::Loop, vec![body(2)]);

    // Act
    let units = SubUnitExtractor::new(1000).extract(&loop_node);

    // Assert
    assert!(units.is_empty());
}

#[test]
fn extract_with_a_kind_that_carries_no_body_returns_nothing_for_that_node() {
    // Arrange
    let block = NormalizedNode::with_children(NodeKind::Block, vec![body(4)]);

    // Act
    let units = SubUnitExtractor::new(1).extract(&block);

    // Assert
    assert!(
        units.is_empty(),
        "a plain block is not itself a compound structure"
    );
}

#[test]
fn new_records_the_floor_that_extract_then_applies() {
    // Arrange
    let loop_node = NormalizedNode::with_children(NodeKind::Loop, vec![body(4)]);

    // Act
    let permissive = SubUnitExtractor::new(1).extract(&loop_node);
    let strict = SubUnitExtractor::new(1000).extract(&loop_node);

    // Assert
    assert_eq!(permissive.len(), 1);
    assert!(strict.is_empty());
}
