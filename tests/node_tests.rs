// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::node::*;
use dry4rust::normalization_context::NormalizationContext;

#[test]
fn context_assigns_sequential_indices() {
    // Arrange & Act
    let mut ctx = NormalizationContext::new();

    // Assert
    assert_eq!(ctx.placeholder("x", PlaceholderKind::Variable), 0);
    assert_eq!(ctx.placeholder("y", PlaceholderKind::Variable), 1);
    assert_eq!(ctx.placeholder("z", PlaceholderKind::Variable), 2);
}

#[test]
fn context_per_kind_counters_are_independent() {
    // Arrange & Act
    let mut ctx = NormalizationContext::new();
    let var_idx = ctx.placeholder("foo", PlaceholderKind::Variable);
    let fn_idx = ctx.placeholder("foo", PlaceholderKind::Function);
    let type_idx = ctx.placeholder("foo", PlaceholderKind::Type);
    // Each kind starts from 0 independently

    // Assert
    assert_eq!(var_idx, 0);
    assert_eq!(fn_idx, 0);
    assert_eq!(type_idx, 0);
}

#[test]
fn context_returns_same_index_for_same_name() {
    // Arrange & Act
    let mut ctx = NormalizationContext::new();
    let first = ctx.placeholder("x", PlaceholderKind::Variable);
    let second = ctx.placeholder("x", PlaceholderKind::Variable);

    // Assert
    assert_eq!(first, second);
    assert_eq!(first, 0);
}

#[test]
fn context_same_name_different_kind_are_distinct() {
    // Arrange & Act
    let mut ctx = NormalizationContext::new();
    ctx.placeholder("x", PlaceholderKind::Variable);
    ctx.placeholder("x", PlaceholderKind::Function);
    // Second variable should get index 1, not 0
    let y_var = ctx.placeholder("y", PlaceholderKind::Variable);

    // Assert
    assert_eq!(y_var, 1);
    let y_fn = ctx.placeholder("y", PlaceholderKind::Function);
    assert_eq!(y_fn, 1);
}

#[test]
fn count_nodes_basic() {
    // Arrange & Act
    let node = NormalizedNode::with_children(
        NodeKind::BinaryOp(BinOpKind::Add),
        vec![
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 0)),
            NormalizedNode::leaf(NodeKind::Literal(LiteralKind::Int)),
        ],
    );

    // Assert
    assert_eq!(count_nodes(&node), 3);
}

#[test]
fn count_nodes_skips_none_sentinels() {
    // Arrange & Act
    let node = NormalizedNode::with_children(
        NodeKind::If,
        vec![
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 0)),
            NormalizedNode::with_children(NodeKind::Block, vec![]),
            NormalizedNode::none(),
        ],
    );
    // If(1) + Placeholder(1) + Block(1) = 3 (None is not counted)

    // Assert
    assert_eq!(count_nodes(&node), 3);
}

#[test]
fn opt_with_a_value_returns_that_value() {
    // Arrange
    let inner = NormalizedNode::leaf(NodeKind::None);

    // Act
    let node = NormalizedNode::opt(Some(inner.clone()));

    // Assert
    assert_eq!(node, inner);
}

#[test]
fn opt_without_a_value_returns_the_none_sentinel() {
    // Arrange & Act
    let node = NormalizedNode::opt(None);

    // Assert
    assert!(node.is_none());
}

#[test]
fn reindex_handles_multiple_placeholder_kinds() {
    // Arrange & Act
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

    // Assert
    assert_eq!(reindexed, expected);
}

#[test]
fn reindex_makes_equivalent_subtrees_equal() {
    // Arrange & Act
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

    // Assert
    assert_ne!(subtree1, subtree2);
    assert_eq!(
        reindex_placeholders(&subtree1),
        reindex_placeholders(&subtree2)
    );
}

#[test]
fn reindex_preserves_same_placeholder_identity() {
    // Arrange & Act
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

    // Assert
    assert_eq!(reindexed, expected);
}

#[test]
fn reindex_remaps_from_zero() {
    // Arrange & Act
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

    // Assert
    assert_eq!(reindexed, expected);
}
