// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::node::node_kind::NodeKind;
use dry4rust::node::normalized_node::NormalizedNode;
use dry4rust::node::placeholder_kind::PlaceholderKind;
use dry4rust::node::placeholder_order_collector::PlaceholderOrderCollector;

#[test]
fn collect_records_each_placeholder_once_in_first_occurrence_order() {
    // Arrange
    let node = NormalizedNode::with_children(
        NodeKind::Block,
        vec![
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 5)),
            NormalizedNode::leaf(NodeKind::TypePlaceholder(PlaceholderKind::Type, 2)),
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 5)),
        ],
    );
    let mut collector = PlaceholderOrderCollector::new();

    // Act
    collector.collect(&node);

    // Assert
    assert_eq!(
        collector.into_order(),
        vec![(PlaceholderKind::Variable, 5), (PlaceholderKind::Type, 2),]
    );
}

#[test]
fn collect_visits_children_depth_first() {
    // Arrange
    let node = NormalizedNode::with_children(
        NodeKind::Call,
        vec![
            NormalizedNode::with_children(
                NodeKind::Cast,
                vec![
                    NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, 9)),
                    NormalizedNode::leaf(NodeKind::PatPlaceholder(PlaceholderKind::Variable, 1)),
                ],
            ),
            NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Function, 4)),
        ],
    );
    let mut collector = PlaceholderOrderCollector::new();

    // Act
    collector.collect(&node);

    // Assert
    assert_eq!(
        collector.into_order(),
        vec![
            (PlaceholderKind::Variable, 9),
            (PlaceholderKind::Variable, 1),
            (PlaceholderKind::Function, 4),
        ]
    );
}

#[test]
fn into_order_over_a_node_without_placeholders_is_empty() {
    // Arrange
    let node = NormalizedNode::leaf(NodeKind::Opaque);
    let mut collector = PlaceholderOrderCollector::new();

    // Act
    collector.collect(&node);

    // Assert
    let order = collector.into_order();
    assert_eq!(order, Vec::new());
}
