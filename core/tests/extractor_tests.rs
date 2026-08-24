// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::extractor::extract_sub_units;
use dry4rust::node::NodeKind;
use dry4rust::node::NormalizedNode;

#[test]
fn extract_sub_units_from_a_leaf_returns_nothing() {
    // Arrange
    let leaf = NormalizedNode::leaf(NodeKind::None);

    // Act
    let units = extract_sub_units(&leaf, 1);

    // Assert
    assert!(units.is_empty());
}

#[test]
fn extract_sub_units_with_a_high_minimum_returns_nothing() {
    // Arrange
    let block = NormalizedNode::with_children(
        NodeKind::Block,
        vec![NormalizedNode::leaf(NodeKind::None); 3],
    );

    // Act
    let units = extract_sub_units(&block, 1000);

    // Assert
    assert!(units.is_empty());
}
