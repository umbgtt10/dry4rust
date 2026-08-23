// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::normalization_context::NormalizationContext;
use dry4rust::rust::normalizer::pat::normalize_pat;
use dry4rust::rust::normalizer::pat::normalize_type;
use syn::parse::Parser;
use syn::parse_str;

#[test]
fn normalize_pat_gives_differently_named_bindings_the_same_node() {
    // Arrange
    let a: syn::Pat = syn::Pat::parse_single.parse_str("x").expect("parses");
    let b: syn::Pat = syn::Pat::parse_single.parse_str("y").expect("parses");

    // Act
    let na = normalize_pat(&a, &mut NormalizationContext::new());
    let nb = normalize_pat(&b, &mut NormalizationContext::new());

    // Assert
    assert_eq!(na, nb);
}

#[test]
fn normalize_type_gives_distinct_types_distinct_placeholders_in_one_context() {
    // Arrange -- types become positional placeholders, so telling two apart is
    // only meaningful within a single context. In separate contexts the first
    // type of each is deliberately the same node.
    let a: syn::Type = parse_str("i32").expect("parses");
    let b: syn::Type = parse_str("Vec<String>").expect("parses");
    let mut ctx = NormalizationContext::new();

    // Act
    let na = normalize_type(&a, &mut ctx);
    let nb = normalize_type(&b, &mut ctx);

    // Assert
    assert_ne!(na, nb);
}

#[test]
fn normalize_type_gives_the_same_type_the_same_node() {
    // Arrange
    let a: syn::Type = parse_str("i32").expect("parses");
    let b: syn::Type = parse_str("i32").expect("parses");

    // Act
    let mut ctx = NormalizationContext::new();
    let na = normalize_type(&a, &mut ctx);
    let nb = normalize_type(&b, &mut ctx);

    // Assert
    assert_eq!(na, nb);
}

#[test]
fn normalize_type_handles_every_type_form_it_claims_to() {
    // Arrange -- one of each arm the match names, so none of them is reached
    // for the first time by a user
    let forms = [
        "i32",           // Path
        "&str",          // Reference
        "(i32, u8)",     // Tuple
        "[u8]",          // Slice
        "[u8; 4]",       // Array
        "impl Iterator", // ImplTrait
        "_",             // Infer
        "!",             // Never
        "(i32)",         // Paren
        "make_type!()",  // Macro
        "dyn Iterator",  // the fallback arm
    ];

    // Act
    let mut ctx = NormalizationContext::new();
    let nodes: Vec<_> = forms
        .iter()
        .map(|src| {
            let ty: syn::Type = parse_str(src).unwrap_or_else(|e| panic!("{src} parses: {e}"));
            normalize_type(&ty, &mut ctx)
        })
        .collect();

    // Assert
    assert_eq!(nodes.len(), forms.len());
}

#[test]
fn normalize_type_sees_through_parentheses_to_the_inner_type() {
    // Arrange
    let bare: syn::Type = parse_str("i32").expect("parses");
    let parenthesised: syn::Type = parse_str("(i32)").expect("parses");

    // Act
    let a = normalize_type(&bare, &mut NormalizationContext::new());
    let b = normalize_type(&parenthesised, &mut NormalizationContext::new());

    // Assert
    assert_eq!(a, b);
}

#[test]
fn normalize_type_keeps_never_and_infer_apart() {
    // Arrange
    let never: syn::Type = parse_str("!").expect("parses");
    let infer: syn::Type = parse_str("_").expect("parses");

    // Act
    let mut ctx = NormalizationContext::new();
    let a = normalize_type(&never, &mut ctx);
    let b = normalize_type(&infer, &mut ctx);

    // Assert
    assert_ne!(a, b);
}
