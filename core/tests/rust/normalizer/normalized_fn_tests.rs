// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::node::NodeKind;
use dry4rust::node::NormalizedNode;
use dry4rust::rust::normalizer::normalized_fn::NormalizedFn;

#[test]
fn new_keeps_the_signature_and_the_body_apart() {
    // Arrange
    let signature = NormalizedNode::leaf(NodeKind::FnSignature);
    let body = NormalizedNode::none();

    // Act
    let normalized = NormalizedFn::new(signature.clone(), body.clone());

    // Assert
    assert_eq!(*normalized.signature(), signature);
    assert_eq!(*normalized.body(), body);
}
