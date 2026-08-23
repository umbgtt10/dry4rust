// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::checking::ceiling::Ceiling;

#[test]
fn breach_of_a_count_exactly_on_its_limit_reports_nothing() {
    // Arrange
    let ceiling = Ceiling::count(Some(3), 3, "exact duplicate groups");

    // Act
    let message = ceiling.breach();

    // Assert
    assert_eq!(
        message, None,
        "the limit is a maximum, so reaching it is allowed"
    );
}

#[test]
fn breach_of_a_count_over_its_limit_names_both_numbers() {
    // Arrange
    let ceiling = Ceiling::count(Some(2), 5, "exact duplicate groups");

    // Act
    let message = ceiling.breach();

    // Assert
    assert_eq!(
        message.as_deref(),
        Some("5 exact duplicate groups (max: 2)")
    );
}

#[test]
fn breach_of_a_count_under_its_limit_reports_nothing() {
    // Arrange
    let ceiling = Ceiling::count(Some(9), 2, "near duplicate groups");

    // Act & Assert
    assert_eq!(ceiling.breach(), None);
}

#[test]
fn breach_of_a_count_with_no_limit_reports_nothing_however_large() {
    // Arrange
    let ceiling = Ceiling::count(None, 9_999, "exact duplicate groups");

    // Act
    let message = ceiling.breach();

    // Assert
    assert_eq!(
        message, None,
        "an unset limit means the caller did not ask, not that it asked for zero"
    );
}

#[test]
fn breach_of_a_percentage_over_its_limit_reports_one_decimal_place() {
    // Arrange
    let ceiling = Ceiling::percent(Some(5.0), 17.649, "exact duplicate lines");

    // Act
    let message = ceiling.breach();

    // Assert
    assert_eq!(
        message.as_deref(),
        Some("17.6% exact duplicate lines (max: 5.0%)"),
        "a share of lines reads to one decimal place, where a group count is whole"
    );
}

#[test]
fn breach_of_a_percentage_with_no_limit_reports_nothing() {
    // Arrange
    let ceiling = Ceiling::percent(None, 100.0, "near duplicate lines");

    // Act & Assert
    assert_eq!(ceiling.breach(), None);
}

#[test]
fn breach_of_a_zero_percentage_limit_reports_any_duplication_at_all() {
    // Arrange
    let ceiling = Ceiling::percent(Some(0.0), 0.1, "near duplicate lines");

    // Act
    let message = ceiling.breach();

    // Assert
    assert_eq!(
        message.as_deref(),
        Some("0.1% near duplicate lines (max: 0.0%)"),
        "zero is a real limit, unlike an absent one"
    );
}

#[test]
fn count_and_percent_format_the_same_number_differently() {
    // Arrange
    let counted = Ceiling::count(Some(1), 4, "groups");
    let shared = Ceiling::percent(Some(1.0), 4.0, "lines");

    // Act
    let (from_count, from_percent) = (counted.breach(), shared.breach());

    // Assert
    assert_eq!(from_count.as_deref(), Some("4 groups (max: 1)"));
    assert_eq!(from_percent.as_deref(), Some("4.0% lines (max: 1.0%)"));
}
