// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::node::PlaceholderKind;
use dry4rust::normalization_context::NormalizationContext;

#[test]
fn placeholder_counts_each_kind_from_zero_independently() {
    // Arrange
    let mut ctx = NormalizationContext::new();

    // Act
    let var = ctx.placeholder("a", PlaceholderKind::Variable);
    let ty = ctx.placeholder("a", PlaceholderKind::Type);

    // Assert
    assert_eq!(var, 0);
    assert_eq!(ty, 0);
}

#[test]
fn placeholder_for_distinct_identifiers_returns_rising_indices() {
    // Arrange
    let mut ctx = NormalizationContext::new();

    // Act
    let first = ctx.placeholder("a", PlaceholderKind::Variable);
    let second = ctx.placeholder("b", PlaceholderKind::Variable);

    // Assert
    assert_eq!(first, 0);
    assert_eq!(second, 1);
}

#[test]
fn placeholder_for_the_same_identifier_twice_returns_the_same_index() {
    // Arrange
    let mut ctx = NormalizationContext::new();

    // Act
    let first = ctx.placeholder("value", PlaceholderKind::Variable);
    let again = ctx.placeholder("value", PlaceholderKind::Variable);

    // Assert
    assert_eq!(first, again);
}
