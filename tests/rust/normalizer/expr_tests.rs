// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::normalization_context::NormalizationContext;
use dry4rust::rust::normalizer::expr::normalize_block;
use dry4rust::rust::normalizer::expr::normalize_stmt;
use syn::parse_str;

#[test]
fn normalize_block_erases_the_names_its_statements_bind() {
    // Arrange
    let a: syn::Block = parse_str("{ let x = 1; x + 1 }").expect("parses");
    let b: syn::Block = parse_str("{ let y = 1; y + 1 }").expect("parses");

    // Act
    let na = normalize_block(&a, &mut NormalizationContext::new());
    let nb = normalize_block(&b, &mut NormalizationContext::new());

    // Assert
    assert_eq!(na, nb);
}

#[test]
fn normalize_block_keeps_blocks_of_different_shape_apart() {
    // Arrange
    let a: syn::Block = parse_str("{ let x = 1; x + 1 }").expect("parses");
    let b: syn::Block = parse_str("{ let x = 1; }").expect("parses");

    // Act
    let na = normalize_block(&a, &mut NormalizationContext::new());
    let nb = normalize_block(&b, &mut NormalizationContext::new());

    // Assert
    assert_ne!(na, nb);
}

#[test]
fn normalize_stmt_gives_alpha_equivalent_lets_the_same_node() {
    // Arrange
    let a: syn::Stmt = parse_str("let x = 1;").expect("parses");
    let b: syn::Stmt = parse_str("let y = 1;").expect("parses");

    // Act
    let na = normalize_stmt(&a, &mut NormalizationContext::new());
    let nb = normalize_stmt(&b, &mut NormalizationContext::new());

    // Assert
    assert_eq!(na, nb);
}
