// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::normalization_context::NormalizationContext;
use dry4rust::rust::normalizer::helpers::member_to_string;
use dry4rust::rust::normalizer::helpers::normalize_bin_op;
use dry4rust::rust::normalizer::helpers::normalize_lit;
use dry4rust::rust::normalizer::helpers::normalize_macro;
use dry4rust::rust::normalizer::helpers::normalize_un_op;
use syn::BinOp;
use syn::Lit;
use syn::Macro;
use syn::Member;
use syn::UnOp;
use syn::parse_str;

#[test]
fn member_to_string_for_a_named_field_returns_the_name() {
    // Arrange
    let member: Member = parse_str("field").expect("parses");

    // Act & Assert
    assert_eq!(member_to_string(&member), "field");
}

#[test]
fn member_to_string_for_a_tuple_index_returns_the_index() {
    // Arrange
    let member: Member = parse_str("0").expect("parses");

    // Act & Assert
    assert_eq!(member_to_string(&member), "0");
}

#[test]
fn normalize_bin_op_maps_different_operators_to_different_kinds() {
    // Arrange
    let add: BinOp = parse_str("+").expect("parses");
    let sub: BinOp = parse_str("-").expect("parses");

    // Act & Assert
    assert_ne!(normalize_bin_op(&add), normalize_bin_op(&sub));
    assert_eq!(normalize_bin_op(&add), normalize_bin_op(&add));
}

#[test]
fn normalize_lit_erases_the_value_but_keeps_the_type() {
    // Arrange
    let small: Lit = parse_str("42").expect("parses");
    let large: Lit = parse_str("99999").expect("parses");
    let text: Lit = parse_str(r#""hello""#).expect("parses");

    // Act & Assert
    assert_eq!(normalize_lit(&small), normalize_lit(&large));
    assert_ne!(normalize_lit(&small), normalize_lit(&text));
}

#[test]
fn normalize_macro_gives_the_same_macro_the_same_node() {
    // Arrange
    let a: Macro = parse_str(r#"println!("one")"#).expect("parses");
    let b: Macro = parse_str(r#"println!("two")"#).expect("parses");

    // Act
    let na = normalize_macro(&a, &mut NormalizationContext::new());
    let nb = normalize_macro(&b, &mut NormalizationContext::new());

    // Assert
    assert_eq!(na, nb);
}

#[test]
fn normalize_un_op_maps_different_operators_to_different_kinds() {
    // Arrange
    let neg: UnOp = parse_str("-").expect("parses");
    let not: UnOp = parse_str("!").expect("parses");

    // Act & Assert
    assert_ne!(normalize_un_op(&neg), normalize_un_op(&not));
}
