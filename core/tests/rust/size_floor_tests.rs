// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::rust::size_floor::SizeFloor;

#[test]
fn admits_a_unit_at_both_floors() {
    // Arrange
    let floor = SizeFloor::new(10, 3);

    // Act
    let admitted = floor.admits(10, 3);

    // Assert
    assert!(admitted);
}

#[test]
fn admits_a_unit_of_any_length_with_no_line_floor() {
    // Arrange
    let floor = SizeFloor::new(10, 0);

    // Act
    let admitted = floor.admits(10, 1);

    // Assert
    assert!(admitted);
}

#[test]
fn admits_a_unit_one_line_short_is_false() {
    // Arrange
    let floor = SizeFloor::new(10, 3);

    // Act
    let admitted = floor.admits(10, 2);

    // Assert
    assert!(!admitted);
}

#[test]
fn admits_a_unit_one_node_short_is_false() {
    // Arrange
    let floor = SizeFloor::new(10, 3);

    // Act
    let admitted = floor.admits(9, 3);

    // Assert
    assert!(!admitted);
}
