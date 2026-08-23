// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::node::LiteralKind;
use dry4rust::node::NodeKind;
use dry4rust::node::NormalizedNode;
use dry4rust::node::PlaceholderKind;
use dry4rust::node_encoder::NodeEncoder;

fn encoded(node: &NormalizedNode) -> u64 {
    let mut encoder = NodeEncoder::new();
    encoder.encode(node);
    encoder.finish()
}

fn leaf(kind: NodeKind) -> NormalizedNode {
    NormalizedNode::leaf(kind)
}

#[test]
fn encode_distinguishes_a_macro_call_from_one_with_another_name() {
    // Arrange
    let println = leaf(NodeKind::MacroCall {
        name: "println".to_string(),
    });
    let eprintln = leaf(NodeKind::MacroCall {
        name: "eprintln".to_string(),
    });

    // Act & Assert
    assert_ne!(encoded(&println), encoded(&eprintln));
}

#[test]
fn encode_distinguishes_a_mutable_reference_from_a_shared_one() {
    // Arrange
    let shared = leaf(NodeKind::Reference { mutable: false });
    let mutable = leaf(NodeKind::Reference { mutable: true });

    // Act & Assert
    assert_ne!(encoded(&shared), encoded(&mutable));
}

#[test]
fn encode_distinguishes_placeholders_by_index() {
    // Arrange
    let first = leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 0));
    let second = leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 1));

    // Act & Assert
    assert_ne!(encoded(&first), encoded(&second));
}

#[test]
fn encode_distinguishes_placeholders_by_kind() {
    // Arrange
    let variable = leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 0));
    let function = leaf(NodeKind::Placeholder(PlaceholderKind::Function, 0));

    // Act & Assert
    assert_ne!(encoded(&variable), encoded(&function));
}

#[test]
fn encode_distinguishes_the_same_placeholder_in_three_different_positions() {
    // Arrange
    let expression = leaf(NodeKind::Placeholder(PlaceholderKind::Type, 0));
    let pattern = leaf(NodeKind::PatPlaceholder(PlaceholderKind::Type, 0));
    let type_position = leaf(NodeKind::TypePlaceholder(PlaceholderKind::Type, 0));

    // Act
    let hashes = [
        encoded(&expression),
        encoded(&pattern),
        encoded(&type_position),
    ];

    // Assert
    assert_ne!(hashes[0], hashes[1]);
    assert_ne!(hashes[1], hashes[2]);
    assert_ne!(hashes[0], hashes[2]);
}

#[test]
fn encode_distinguishes_trees_that_differ_only_in_where_the_nesting_falls() {
    // Arrange
    let a = leaf(NodeKind::Return);
    let b = leaf(NodeKind::Break);
    let left_nested = NormalizedNode::with_children(
        NodeKind::Block,
        vec![
            NormalizedNode::with_children(NodeKind::Block, vec![a.clone()]),
            b.clone(),
        ],
    );
    let right_nested = NormalizedNode::with_children(
        NodeKind::Block,
        vec![a, NormalizedNode::with_children(NodeKind::Block, vec![b])],
    );

    // Act & Assert
    assert_ne!(
        encoded(&left_nested),
        encoded(&right_nested),
        "the child-count prefix is what separates these two"
    );
}

#[test]
fn encode_distinguishes_two_literals_of_different_type() {
    // Arrange
    let integer = leaf(NodeKind::Literal(LiteralKind::Int));
    let string = leaf(NodeKind::Literal(LiteralKind::Str));

    // Act & Assert
    assert_ne!(encoded(&integer), encoded(&string));
}

#[test]
fn encode_gives_equal_trees_equal_fingerprints() {
    // Arrange
    let build = || {
        NormalizedNode::with_children(
            NodeKind::Block,
            vec![
                leaf(NodeKind::Return),
                leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 3)),
            ],
        )
    };

    // Act & Assert
    assert_eq!(encoded(&build()), encoded(&build()));
}

#[test]
fn encode_ignores_the_order_of_nothing_and_reflects_the_order_of_children() {
    // Arrange
    let forwards = NormalizedNode::with_children(
        NodeKind::Block,
        vec![leaf(NodeKind::Return), leaf(NodeKind::Break)],
    );
    let backwards = NormalizedNode::with_children(
        NodeKind::Block,
        vec![leaf(NodeKind::Break), leaf(NodeKind::Return)],
    );

    // Act & Assert
    assert_ne!(
        encoded(&forwards),
        encoded(&backwards),
        "statement order is part of what a block is"
    );
}

#[test]
fn finish_without_encoding_anything_matches_a_fresh_hasher() {
    // Arrange & Act
    let untouched = NodeEncoder::new().finish();
    let defaulted = NodeEncoder::default().finish();

    // Assert
    assert_eq!(untouched, defaulted);
}

#[test]
fn new_starts_every_encoder_from_the_same_state() {
    // Arrange
    let node = leaf(NodeKind::Try);

    // Act
    let first = encoded(&node);
    let second = encoded(&node);

    // Assert
    assert_eq!(
        first, second,
        "a fingerprint must not depend on how many trees were hashed before it"
    );
}
