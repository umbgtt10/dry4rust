// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::output::display_path::DisplayPath;
use dry4rust::source_path::SourcePath;

#[test]
fn of_outside_its_base_falls_back_to_the_whole_path() {
    // Arrange
    let base = SourcePath::new(String::from("/project"));
    let path = SourcePath::new(String::from("/elsewhere/lib.rs"));

    // Act
    let shown = DisplayPath::of(Some(&base), &path);

    // Assert
    assert!(shown.contains("elsewhere"), "{shown}");
}

#[test]
fn of_under_a_native_base_returns_the_relative_part_as_given() {
    // Arrange
    let base = SourcePath::native(String::from("C:\\project"), '\\');
    let path = SourcePath::native(String::from("C:\\project\\src\\lib.rs"), '\\');

    // Act
    let shown = DisplayPath::of(Some(&base), &path);

    // Assert
    assert_eq!(shown, "src\\lib.rs");
}

#[test]
fn of_under_its_base_returns_the_relative_part() {
    // Arrange
    let base = SourcePath::new(String::from("/project"));
    let path = SourcePath::new(String::from("/project/src/lib.rs"));

    // Act
    let shown = DisplayPath::of(Some(&base), &path);

    // Assert
    assert!(!shown.contains("project"), "{shown}");
    assert!(shown.contains("lib.rs"), "{shown}");
}

#[test]
fn of_without_a_base_returns_a_native_path_as_given() {
    // Arrange
    let path = SourcePath::native(String::from("C:\\project\\src\\lib.rs"), '\\');

    // Act
    let shown = DisplayPath::of(None, &path);

    // Assert
    assert_eq!(shown, "C:\\project\\src\\lib.rs");
}

#[test]
fn of_without_a_base_returns_the_path_unchanged() {
    // Arrange
    let path = SourcePath::new(String::from("src/lib.rs"));

    // Act
    let shown = DisplayPath::of(None, &path);

    // Assert
    assert_eq!(shown, "src/lib.rs");
}
