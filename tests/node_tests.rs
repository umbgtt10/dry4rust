// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::node::*;

#[test]
fn context_assigns_sequential_indices() {
    let mut ctx = NormalizationContext::new();
    assert_eq!(ctx.placeholder("x", PlaceholderKind::Variable), 0);
    assert_eq!(ctx.placeholder("y", PlaceholderKind::Variable), 1);
    assert_eq!(ctx.placeholder("z", PlaceholderKind::Variable), 2);
}

#[test]
fn context_per_kind_counters_are_independent() {
    let mut ctx = NormalizationContext::new();
    let var_idx = ctx.placeholder("foo", PlaceholderKind::Variable);
    let fn_idx = ctx.placeholder("foo", PlaceholderKind::Function);
    let type_idx = ctx.placeholder("foo", PlaceholderKind::Type);
    // Each kind starts from 0 independently
    assert_eq!(var_idx, 0);
    assert_eq!(fn_idx, 0);
    assert_eq!(type_idx, 0);
}

#[test]
fn context_returns_same_index_for_same_name() {
    let mut ctx = NormalizationContext::new();
    let first = ctx.placeholder("x", PlaceholderKind::Variable);
    let second = ctx.placeholder("x", PlaceholderKind::Variable);
    assert_eq!(first, second);
    assert_eq!(first, 0);
}

#[test]
fn context_same_name_different_kind_are_distinct() {
    let mut ctx = NormalizationContext::new();
    ctx.placeholder("x", PlaceholderKind::Variable);
    ctx.placeholder("x", PlaceholderKind::Function);
    // Second variable should get index 1, not 0
    let y_var = ctx.placeholder("y", PlaceholderKind::Variable);
    assert_eq!(y_var, 1);
    let y_fn = ctx.placeholder("y", PlaceholderKind::Function);
    assert_eq!(y_fn, 1);
}

#[test]
fn count_nodes_basic() {
    let node = NormalizedNode::with_children(
        NodeKind::BinaryOp(BinOpKind::Add),
        vec![
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 0)),
            NormalizedNode::leaf(NodeKind::Literal(LiteralKind::Int)),
        ],
    );
    assert_eq!(count_nodes(&node), 3);
}

#[test]
fn count_nodes_skips_none_sentinels() {
    let node = NormalizedNode::with_children(
        NodeKind::If,
        vec![
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 0)),
            NormalizedNode::with_children(NodeKind::Block, vec![]),
            NormalizedNode::none(),
        ],
    );
    // If(1) + Placeholder(1) + Block(1) = 3 (None is not counted)
    assert_eq!(count_nodes(&node), 3);
}

#[test]
fn reindex_handles_multiple_placeholder_kinds() {
    let node = NormalizedNode::with_children(
        NodeKind::Call,
        vec![
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Function, 3)),
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 5)),
            NormalizedNode::with_children(
                NodeKind::Cast,
                vec![
                    NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 5)),
                    NormalizedNode::leaf(NodeKind::TypePlaceholder(PlaceholderKind::Type, 2)),
                ],
            ),
        ],
    );
    let reindexed = reindex_placeholders(&node);
    let expected = NormalizedNode::with_children(
        NodeKind::Call,
        vec![
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Function, 0)),
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 0)),
            NormalizedNode::with_children(
                NodeKind::Cast,
                vec![
                    NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 0)),
                    NormalizedNode::leaf(NodeKind::TypePlaceholder(PlaceholderKind::Type, 0)),
                ],
            ),
        ],
    );
    assert_eq!(reindexed, expected);
}

#[test]
fn reindex_makes_equivalent_subtrees_equal() {
    let subtree1 = NormalizedNode::with_children(
        NodeKind::Block,
        vec![
            NormalizedNode::with_children(
                NodeKind::LetBinding,
                vec![
                    NormalizedNode::leaf(NodeKind::PatPlaceholder(PlaceholderKind::Variable, 2)),
                    NormalizedNode::none(),
                    NormalizedNode::with_children(
                        NodeKind::BinaryOp(BinOpKind::Add),
                        vec![
                            NormalizedNode::leaf(NodeKind::Placeholder(
                                PlaceholderKind::Variable,
                                0,
                            )),
                            NormalizedNode::leaf(NodeKind::Literal(LiteralKind::Int)),
                        ],
                    ),
                    NormalizedNode::none(),
                ],
            ),
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 2)),
        ],
    );
    let subtree2 = NormalizedNode::with_children(
        NodeKind::Block,
        vec![
            NormalizedNode::with_children(
                NodeKind::LetBinding,
                vec![
                    NormalizedNode::leaf(NodeKind::PatPlaceholder(PlaceholderKind::Variable, 7)),
                    NormalizedNode::none(),
                    NormalizedNode::with_children(
                        NodeKind::BinaryOp(BinOpKind::Add),
                        vec![
                            NormalizedNode::leaf(NodeKind::Placeholder(
                                PlaceholderKind::Variable,
                                5,
                            )),
                            NormalizedNode::leaf(NodeKind::Literal(LiteralKind::Int)),
                        ],
                    ),
                    NormalizedNode::none(),
                ],
            ),
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 7)),
        ],
    );

    assert_ne!(subtree1, subtree2);
    assert_eq!(
        reindex_placeholders(&subtree1),
        reindex_placeholders(&subtree2)
    );
}

#[test]
fn reindex_preserves_same_placeholder_identity() {
    let node = NormalizedNode::with_children(
        NodeKind::BinaryOp(BinOpKind::Add),
        vec![
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 3)),
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 3)),
        ],
    );
    let reindexed = reindex_placeholders(&node);
    let expected = NormalizedNode::with_children(
        NodeKind::BinaryOp(BinOpKind::Add),
        vec![
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 0)),
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 0)),
        ],
    );
    assert_eq!(reindexed, expected);
}

#[test]
fn reindex_remaps_from_zero() {
    let node = NormalizedNode::with_children(
        NodeKind::BinaryOp(BinOpKind::Add),
        vec![
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 5)),
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 8)),
        ],
    );
    let reindexed = reindex_placeholders(&node);
    let expected = NormalizedNode::with_children(
        NodeKind::BinaryOp(BinOpKind::Add),
        vec![
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 0)),
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 1)),
        ],
    );
    assert_eq!(reindexed, expected);
}
