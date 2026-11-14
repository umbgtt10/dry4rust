// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::line_span::LineSpan;

#[test]
fn lines_of_a_span_on_one_line_is_one() {
    // Arrange
    let span = LineSpan::new(5, 5);

    // Act
    let lines = span.lines();

    // Assert
    assert_eq!(lines, 1);
}

#[test]
fn lines_of_a_span_over_several_lines_counts_its_first_and_last() {
    // Arrange
    let span = LineSpan::new(3, 7);

    // Act
    let lines = span.lines();

    // Assert
    assert_eq!(lines, 5);
}
