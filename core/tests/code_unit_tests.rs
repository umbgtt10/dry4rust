// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::code_unit::CodeUnit;

#[test]
fn node_count_returns_the_count_it_was_built_with() {
    // Arrange
    let unit = CodeUnit::default().with_node_count(42);

    // Act
    let node_count = unit.node_count();

    // Assert
    assert_eq!(node_count, 42);
}

#[test]
fn parent_name_returns_the_function_a_sub_unit_came_from() {
    // Arrange
    let unit = CodeUnit::default().with_parent_name(Some(String::from("handle_positive")));

    // Act
    let parent_name = unit.parent_name();

    // Assert
    assert_eq!(parent_name, Some("handle_positive"));
}

#[test]
fn with_is_test_marks_the_unit_as_test_code() {
    // Arrange
    let unit = CodeUnit::default();

    // Act
    let unit = unit.with_is_test(true);

    // Assert
    assert!(unit.is_test());
}
