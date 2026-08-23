// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::output::reporter::display_path;
use std::path::Path;

#[test]
fn display_path_without_a_base_returns_the_path_unchanged() {
    // Arrange
    let path = Path::new("src/lib.rs");

    // Act
    let shown = display_path(None, path);

    // Assert
    assert_eq!(shown, "src/lib.rs");
}

#[test]
fn display_path_under_its_base_returns_the_relative_part() {
    // Arrange
    let base = Path::new("/project");
    let path = Path::new("/project/src/lib.rs");

    // Act
    let shown = display_path(Some(base), path);

    // Assert
    assert!(!shown.contains("project"), "{shown}");
    assert!(shown.contains("lib.rs"), "{shown}");
}

#[test]
fn display_path_outside_its_base_falls_back_to_the_whole_path() {
    // Arrange
    let base = Path::new("/project");
    let path = Path::new("/elsewhere/lib.rs");

    // Act
    let shown = display_path(Some(base), path);

    // Assert
    assert!(shown.contains("elsewhere"), "{shown}");
}
