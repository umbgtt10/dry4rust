// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::node::{BinOpKind, LiteralKind, PlaceholderKind};
use dry4rust::node::{NodeKind, NormalizedNode};
use dry4rust::similarity::*;

fn int_lit() -> NormalizedNode {
    NormalizedNode::leaf(NodeKind::Literal(LiteralKind::Int))
}

fn var(idx: usize) -> NormalizedNode {
    NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, idx))
}

#[test]
fn both_none_sentinels_score_one() {
    let a = NormalizedNode::none();
    let b = NormalizedNode::none();
    assert!((similarity_score(&a, &b) - 1.0).abs() < f64::EPSILON);
}

#[test]
fn completely_different_kinds_score_zero() {
    let a = int_lit();
    let b = NormalizedNode::leaf(NodeKind::PatWild);
    assert!(similarity_score(&a, &b) < f64::EPSILON);
}

#[test]
fn different_child_counts_uses_shared_prefix() {
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
    assert!((score - 0.8).abs() < f64::EPSILON);
}

#[test]
fn identical_leaves_score_one() {
    let a = int_lit();
    let b = int_lit();
    assert!((similarity_score(&a, &b) - 1.0).abs() < f64::EPSILON);
}

#[test]
fn if_with_else_vs_if_without_else() {
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
    assert!((score - 0.8).abs() < f64::EPSILON);
}

#[test]
fn macro_call_different_names_score_zero() {
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
    assert!(similarity_score(&a, &b) < f64::EPSILON);
}

#[test]
fn macro_call_same_name_different_args_partial() {
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
    assert!((score - 0.8).abs() < f64::EPSILON);
}

#[test]
fn none_vs_real_node_score_zero() {
    let a = NormalizedNode::none();
    let b = int_lit();
    assert!(similarity_score(&a, &b) < f64::EPSILON);
}

#[test]
fn return_with_vs_without_value() {
    // Return -> [value] vs Return -> []
    let with_val = NormalizedNode::with_children(NodeKind::Return, vec![int_lit()]);
    let without_val = NormalizedNode::with_children(NodeKind::Return, vec![]);
    let score = similarity_score(&with_val, &without_val);
    // with_val: Return + Int = 2 nodes; without_val: Return = 1 node
    // matching: Return(1) + zip(empty) = 1
    // score = 2*1 / (2+1) = 2/3
    assert!((score - 2.0 / 3.0).abs() < f64::EPSILON);
}

#[test]
fn same_discriminant_different_data_no_self_match() {
    // BinaryOp(Add) vs BinaryOp(Sub) — same discriminant, different data
    let a =
        NormalizedNode::with_children(NodeKind::BinaryOp(BinOpKind::Add), vec![var(0), int_lit()]);
    let b =
        NormalizedNode::with_children(NodeKind::BinaryOp(BinOpKind::Sub), vec![var(0), int_lit()]);
    let score = similarity_score(&a, &b);
    // matching: BinOp self_match=0 (Add!=Sub) + var(1) + int(1) = 2
    // nodes_a = 3, nodes_b = 3 => score = 4/6 = 0.667
    assert!((score - 2.0 / 3.0).abs() < f64::EPSILON);
}

#[test]
fn similarity_is_symmetric() {
    let a = NormalizedNode::with_children(NodeKind::Block, vec![int_lit(), var(0)]);
    let b = NormalizedNode::with_children(NodeKind::Block, vec![int_lit(), var(0), var(1)]);
    assert!((similarity_score(&a, &b) - similarity_score(&b, &a)).abs() < f64::EPSILON);
}
