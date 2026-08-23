// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::normalization_context::NormalizationContext;
use dry4rust::rust::normalizer::pat::normalize_pat;
use dry4rust::rust::normalizer::pat::normalize_type;
use syn::parse::Parser;
use syn::parse_str;

fn pat_of(source: &str) -> String {
    let pat = syn::Pat::parse_single
        .parse_str(source)
        .expect("the pattern parses");
    let mut ctx = NormalizationContext::new();
    format!("{:?}", normalize_pat(&pat, &mut ctx))
}

fn type_of(source: &str) -> String {
    let ty: syn::Type = parse_str(source).expect("the type parses");
    let mut ctx = NormalizationContext::new();
    format!("{:?}", normalize_type(&ty, &mut ctx))
}

#[test]
fn normalize_pat_gives_a_tuple_struct_the_same_kind_as_a_braced_struct() {
    // Arrange & Act
    let tuple_struct = pat_of("Some(inner)");
    let braced = pat_of("Point { x }");

    // Assert
    assert!(tuple_struct.contains("PatStruct"), "got {tuple_struct}");
    assert!(braced.contains("PatStruct"), "got {braced}");
    assert!(
        !tuple_struct.contains("FieldValue"),
        "a tuple struct destructures by position, so its elements are bare"
    );
    assert!(
        braced.contains("FieldValue"),
        "a braced struct destructures by name, so each field carries the name          alongside the binding -- which is why the two are not interchangeable          despite sharing a kind"
    );
}

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
fn normalize_pat_keeps_a_literal_pattern_apart_from_a_wildcard() {
    // Arrange & Act
    let literal = pat_of("42");
    let wildcard = pat_of("_");

    // Assert
    assert!(literal.contains("PatLiteral"));
    assert!(wildcard.contains("PatWild"));
}

#[test]
fn normalize_pat_over_a_mutable_reference_records_the_mutability() {
    // Arrange & Act
    let shared = pat_of("&value");
    let mutable = pat_of("&mut value");

    // Assert
    assert_ne!(
        shared, mutable,
        "a pattern binding through &mut is not the same shape as one through &"
    );
}

#[test]
fn normalize_pat_over_a_range_pattern_yields_a_range_node() {
    // Arrange & Act
    let node = pat_of("1..=9");

    // Assert
    assert!(node.contains("PatRange"), "got {node}");
}

#[test]
fn normalize_pat_over_a_rest_pattern_yields_a_rest_node() {
    // Arrange & Act
    let node = pat_of("[first, ..]");

    // Assert
    assert!(node.contains("PatRest"), "got {node}");
}

#[test]
fn normalize_pat_over_a_slice_pattern_yields_a_slice_node() {
    // Arrange & Act
    let node = pat_of("[a, b, c]");

    // Assert
    assert!(node.contains("PatSlice"), "got {node}");
}

#[test]
fn normalize_pat_over_a_struct_pattern_yields_a_struct_node() {
    // Arrange & Act
    let node = pat_of("Point { x, y }");

    // Assert
    assert!(node.contains("PatStruct"), "got {node}");
}

#[test]
fn normalize_pat_over_an_alternation_yields_an_or_node() {
    // Arrange
    let pat = syn::Pat::parse_multi
        .parse_str("Red | Green | Blue")
        .expect("an alternation needs parse_multi; parse_single rejects it");
    let mut ctx = NormalizationContext::new();

    // Act
    let node = format!("{:?}", normalize_pat(&pat, &mut ctx));

    // Assert
    assert!(node.contains("PatOr"), "got {node}");
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

#[test]
fn normalize_type_over_a_never_type_yields_a_never_node() {
    // Arrange & Act
    let node = type_of("!");

    // Assert
    assert!(node.contains("TypeNever"), "got {node}");
}

#[test]
fn normalize_type_over_a_parenthesised_type_unwraps_the_parentheses() {
    // Arrange & Act
    let bare = type_of("i32");
    let wrapped = type_of("(i32)");

    // Assert
    assert_eq!(
        bare, wrapped,
        "parentheses group the source, not the type itself"
    );
}

#[test]
fn normalize_type_over_a_slice_and_an_array_keeps_them_apart() {
    // Arrange & Act
    let slice = type_of("[u8]");
    let array = type_of("[u8; 4]");

    // Assert
    assert!(slice.contains("TypeSlice"), "got {slice}");
    assert!(array.contains("TypeArray"), "got {array}");
}

#[test]
fn normalize_type_over_an_impl_trait_yields_an_impl_trait_node() {
    // Arrange & Act
    let node = type_of("impl Iterator<Item = u8>");

    // Assert
    assert!(node.contains("TypeImplTrait"), "got {node}");
}

#[test]
fn normalize_type_over_an_inferred_type_yields_an_infer_node() {
    // Arrange & Act
    let node = type_of("_");

    // Assert
    assert!(node.contains("TypeInfer"), "got {node}");
}

#[test]
fn normalize_type_over_the_unit_type_yields_a_unit_node() {
    // Arrange & Act
    let node = type_of("()");

    // Assert
    assert!(node.contains("TypeUnit"), "got {node}");
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
