// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::near_duplicate::similarity::*;
use dry4rust::node::{BinOpKind, LiteralKind, PlaceholderKind};
use dry4rust::node::{NodeKind, NormalizedNode};

fn block_of(kinds: &[NodeKind]) -> NormalizedNode {
    NormalizedNode::with_children(
        NodeKind::Block,
        kinds.iter().cloned().map(NormalizedNode::leaf).collect(),
    )
}

const fn int_lit() -> NormalizedNode {
    NormalizedNode::leaf(NodeKind::Literal(LiteralKind::Int))
}

const fn var(idx: usize) -> NormalizedNode {
    NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, idx))
}

#[test]
fn both_none_sentinels_score_one() {
    // Arrange & Act
    let a = NormalizedNode::none();
    let b = NormalizedNode::none();

    // Assert
    assert!((similarity_score(&a, &b) - 1.0).abs() < f64::EPSILON);
}

#[test]
fn completely_different_kinds_score_zero() {
    // Arrange & Act
    let a = int_lit();
    let b = NormalizedNode::leaf(NodeKind::PatWild);

    // Assert
    assert!(similarity_score(&a, &b) < f64::EPSILON);
}

#[test]
fn different_child_counts_uses_shared_prefix() {
    // Arrange & Act
    // Block with 3 children vs Block with 5 children
    // zip compares only first 3 pairs; extra 2 are unmatched
    let a = NormalizedNode::with_children(NodeKind::Block, vec![int_lit(), int_lit(), int_lit()]);
    let b = NormalizedNode::with_children(
        NodeKind::Block,
        vec![int_lit(), int_lit(), int_lit(), var(0), var(1)],
    );
    let score = similarity_score(&a, &b);
    // matching = Block(1) + 3 Int(1) = 4
    // nodes_a = 4, nodes_b = 6 => score = 8/10 = 0.8

    // Assert
    assert!((score - 0.8).abs() < f64::EPSILON);
}

#[test]
fn identical_leaves_score_one() {
    // Arrange & Act
    let a = int_lit();
    let b = int_lit();

    // Assert
    assert!((similarity_score(&a, &b) - 1.0).abs() < f64::EPSILON);
}

#[test]
fn if_with_else_vs_if_without_else() {
    // Arrange & Act
    // If -> [condition, then, else_or_None]
    let with_else = NormalizedNode::with_children(
        NodeKind::If,
        vec![
            var(0),
            NormalizedNode::with_children(NodeKind::Block, vec![int_lit()]),
            NormalizedNode::with_children(NodeKind::Block, vec![int_lit()]),
        ],
    );
    let without_else = NormalizedNode::with_children(
        NodeKind::If,
        vec![
            var(0),
            NormalizedNode::with_children(NodeKind::Block, vec![int_lit()]),
            NormalizedNode::none(),
        ],
    );
    let score = similarity_score(&with_else, &without_else);
    // with_else: If + var + Block + Int + Block + Int = 6 nodes
    // without_else: If + var + Block + Int + None(0) = 4 nodes
    // matching: If(1) + var(1) + Block(1) + Int(1) + (Block vs None = 0) = 4
    // score = 2*4 / (6+4) = 0.8

    // Assert
    assert!((score - 0.8).abs() < f64::EPSILON);
}

#[test]
fn macro_call_different_names_score_zero() {
    // Arrange & Act
    let a = NormalizedNode::with_children(
        NodeKind::MacroCall {
            name: "println".to_string(),
        },
        vec![int_lit()],
    );
    let b = NormalizedNode::with_children(
        NodeKind::MacroCall {
            name: "eprintln".to_string(),
        },
        vec![int_lit()],
    );

    // Assert
    assert!(similarity_score(&a, &b) < f64::EPSILON);
}

#[test]
fn macro_call_same_name_different_args_partial() {
    // Arrange & Act
    let a = NormalizedNode::with_children(
        NodeKind::MacroCall {
            name: "println".to_string(),
        },
        vec![int_lit()],
    );
    let b = NormalizedNode::with_children(
        NodeKind::MacroCall {
            name: "println".to_string(),
        },
        vec![int_lit(), var(0)],
    );
    let score = similarity_score(&a, &b);
    // a: MacroCall + Int = 2; b: MacroCall + Int + var = 3
    // matching: MacroCall(1) + Int(1) = 2; score = 4/5 = 0.8

    // Assert
    assert!((score - 0.8).abs() < f64::EPSILON);
}

#[test]
fn none_vs_real_node_score_zero() {
    // Arrange & Act
    let a = NormalizedNode::none();
    let b = int_lit();

    // Assert
    assert!(similarity_score(&a, &b) < f64::EPSILON);
}

#[test]
fn return_with_vs_without_value() {
    // Arrange & Act
    // Return -> [value] vs Return -> []
    let with_val = NormalizedNode::with_children(NodeKind::Return, vec![int_lit()]);
    let without_val = NormalizedNode::with_children(NodeKind::Return, vec![]);
    let score = similarity_score(&with_val, &without_val);
    // with_val: Return + Int = 2 nodes; without_val: Return = 1 node
    // matching: Return(1) + zip(empty) = 1
    // score = 2*1 / (2+1) = 2/3

    // Assert
    assert!((score - 2.0 / 3.0).abs() < f64::EPSILON);
}

#[test]
fn same_discriminant_different_data_no_self_match() {
    // Arrange & Act
    // BinaryOp(Add) vs BinaryOp(Sub) — same discriminant, different data
    let a =
        NormalizedNode::with_children(NodeKind::BinaryOp(BinOpKind::Add), vec![var(0), int_lit()]);
    let b =
        NormalizedNode::with_children(NodeKind::BinaryOp(BinOpKind::Sub), vec![var(0), int_lit()]);
    let score = similarity_score(&a, &b);
    // matching: BinOp self_match=0 (Add!=Sub) + var(1) + int(1) = 2
    // nodes_a = 3, nodes_b = 3 => score = 4/6 = 0.667

    // Assert
    assert!((score - 2.0 / 3.0).abs() < f64::EPSILON);
}

#[test]
fn similarity_is_symmetric() {
    // Arrange & Act
    let a = NormalizedNode::with_children(NodeKind::Block, vec![int_lit(), var(0)]);
    let b = NormalizedNode::with_children(NodeKind::Block, vec![int_lit(), var(0), var(1)]);

    // Assert
    assert!((similarity_score(&a, &b) - similarity_score(&b, &a)).abs() < f64::EPSILON);
}

#[test]
fn similarity_score_between_blocks_differing_by_one_inserted_statement_stays_high() {
    // Arrange
    let original = block_of(&[
        NodeKind::Return,
        NodeKind::Break,
        NodeKind::Continue,
        NodeKind::Await,
    ]);
    let with_insertion = block_of(&[
        NodeKind::Try,
        NodeKind::Return,
        NodeKind::Break,
        NodeKind::Continue,
        NodeKind::Await,
    ]);

    // Act
    let score = similarity_score(&original, &with_insertion);

    // Assert
    assert!(
        (score - 10.0 / 11.0).abs() < 1e-9,
        "five of eleven nodes differ by one insertion, so ten match; a \
         positional comparison scored this 2/11 because every statement after \
         the insertion lined up against its neighbour, got {score}"
    );
}

#[test]
fn similarity_score_between_blocks_with_a_statement_removed_stays_high() {
    // Arrange
    let original = block_of(&[NodeKind::Return, NodeKind::Break, NodeKind::Continue]);
    let shortened = block_of(&[NodeKind::Return, NodeKind::Continue]);

    // Act
    let score = similarity_score(&original, &shortened);

    // Assert
    assert!(
        (score - 6.0 / 7.0).abs() < 1e-9,
        "the surviving statements align across the gap, got {score}"
    );
}

#[test]
fn similarity_score_between_tuples_differing_by_one_inserted_element_stays_high() {
    // Arrange
    let pair = NormalizedNode::with_children(
        NodeKind::Tuple,
        vec![
            NormalizedNode::leaf(NodeKind::Return),
            NormalizedNode::leaf(NodeKind::Await),
        ],
    );
    let triple = NormalizedNode::with_children(
        NodeKind::Tuple,
        vec![
            NormalizedNode::leaf(NodeKind::Try),
            NormalizedNode::leaf(NodeKind::Return),
            NormalizedNode::leaf(NodeKind::Await),
        ],
    );

    // Act
    let score = similarity_score(&pair, &triple);

    // Assert
    assert!(
        (score - 6.0 / 7.0).abs() < 1e-9,
        "a tuple is a list too, got {score}"
    );
}

#[test]
fn similarity_score_never_matches_a_then_branch_against_an_else_branch() {
    // Arrange
    let then_only = NormalizedNode::with_children(
        NodeKind::If,
        vec![
            NormalizedNode::leaf(NodeKind::Return),
            NormalizedNode::leaf(NodeKind::Await),
            NormalizedNode::none(),
        ],
    );
    let else_only = NormalizedNode::with_children(
        NodeKind::If,
        vec![
            NormalizedNode::leaf(NodeKind::Return),
            NormalizedNode::none(),
            NormalizedNode::leaf(NodeKind::Await),
        ],
    );

    // Act
    let score = similarity_score(&then_only, &else_only);

    // Assert
    assert!(
        (score - 2.0 / 3.0).abs() < 1e-9,
        "an If holds named slots, so only the condition and the If itself \
         match; free alignment would have paired a then-branch against an \
         else-branch and called these two identical, got {score}"
    );
}
