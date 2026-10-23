// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// Mirrors src/rust/plain_constructor.rs.
//
// A constructor that only moves its parameters into the struct has the struct's shape and nothing else: two of them
// with as many fields fingerprint alike whatever they build, and calling that duplication would count the fields.

use dry4rust::rust::plain_constructor::PlainConstructor;
use syn::ImplItem;
use syn::ImplItemFn;
use syn::ItemImpl;
use syn::parse_str;

fn method(source: &str) -> ImplItemFn {
    let block: ItemImpl = parse_str(source).expect("parses");
    block
        .items
        .into_iter()
        .find_map(|item| match item {
            ImplItem::Fn(method) => Some(method),
            _ => None,
        })
        .expect("a method")
}

#[test]
fn is_plain_of_a_constructor_moving_its_parameters_by_shorthand_is_true() {
    // Arrange
    let new = method("impl Point { fn new(x: i32, y: i32) -> Self { Self { x, y } } }");

    // Act & Assert
    assert!(PlainConstructor::is_plain(&new, "Point"));
}

#[test]
fn is_plain_of_a_field_converted_is_false() {
    // Arrange
    let new =
        method("impl Named { fn new(name: &str) -> Self { Self { name: name.to_string() } } }");

    // Act & Assert
    assert!(!PlainConstructor::is_plain(&new, "Named"));
}

#[test]
fn is_plain_of_a_field_given_a_default_is_false() {
    // Arrange
    let new = method("impl Report { fn new(rows: usize) -> Self { Self { rows, title: None } } }");

    // Act & Assert
    assert!(!PlainConstructor::is_plain(&new, "Report"));
}

#[test]
fn is_plain_of_a_tuple_constructor_is_true() {
    // Arrange
    let new = method("impl Pair { fn new(left: u8, right: u8) -> Self { Self(left, right) } }");

    // Act & Assert
    assert!(PlainConstructor::is_plain(&new, "Pair"));
}

#[test]
fn is_plain_of_another_types_literal_is_false() {
    // Arrange
    let new = method("impl Point { fn origin(x: i32, y: i32) -> Line { Line { x, y } } }");

    // Act & Assert
    assert!(!PlainConstructor::is_plain(&new, "Point"));
}

#[test]
fn is_plain_of_fields_named_after_other_parameters_is_true() {
    // Arrange
    let new = method(
        "impl Span { fn new(first: usize, last: usize) -> Self { Self { start: first, end: last } } }",
    );

    // Act & Assert
    assert!(PlainConstructor::is_plain(&new, "Span"));
}

#[test]
fn is_plain_of_the_type_named_rather_than_self_is_true() {
    // Arrange
    let new = method("impl Point { fn new(x: i32, y: i32) -> Point { Point { x, y } } }");

    // Act & Assert
    assert!(PlainConstructor::is_plain(&new, "Point"));
}

#[test]
fn is_plain_with_a_base_expression_is_false() {
    // Arrange
    let new = method("impl Point { fn new(x: i32) -> Self { Self { x, ..Self::default() } } }");

    // Act & Assert
    assert!(!PlainConstructor::is_plain(&new, "Point"));
}

#[test]
fn is_plain_with_a_statement_before_the_struct_is_false() {
    // Arrange
    let new =
        method("impl Point { fn new(x: i32, y: i32) -> Self { let y = y + 1; Self { x, y } } }");

    // Act & Assert
    assert!(!PlainConstructor::is_plain(&new, "Point"));
}
