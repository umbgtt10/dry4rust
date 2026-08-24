// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::error::Error;
use dry4rust::threshold::Threshold;

#[test]
fn as_fraction_returns_what_fraction_was_given() {
    // Arrange
    let threshold = Threshold::fraction("t", 0.85).expect("0.85 is a fraction");

    // Act
    let value = threshold.as_fraction();

    // Assert
    assert!((value - 0.85).abs() < f64::EPSILON);
}

#[test]
fn as_percent_of_a_fraction_scales_it_by_a_hundred() {
    // Arrange
    let threshold = Threshold::fraction("t", 0.85).expect("0.85 is a fraction");

    // Act
    let value = threshold.as_percent();

    // Assert
    assert!(
        (value - 85.0).abs() < 1e-9,
        "one value written two ways, got {value}"
    );
}

#[test]
fn default_similarity_is_the_documented_nine_tenths() {
    // Arrange & Act
    let threshold = Threshold::DEFAULT_SIMILARITY;

    // Assert
    assert!((threshold.as_fraction() - 0.9).abs() < f64::EPSILON);
}

#[test]
fn fraction_and_percent_of_the_same_proportion_are_equal() {
    // Arrange
    let written_as_fraction = Threshold::fraction("t", 0.05).expect("0.05 is a fraction");

    // Act
    let written_as_percent = Threshold::percent("t", 5.0).expect("5% is a percentage");

    // Assert
    assert!(
        (written_as_fraction.as_fraction() - written_as_percent.as_fraction()).abs() < 1e-9,
        "the two constructors take the same proportion in different clothes"
    );
}

#[test]
fn fraction_at_either_end_of_its_range_is_accepted() {
    // Arrange & Act
    let none = Threshold::fraction("t", 0.0);
    let all = Threshold::fraction("t", 1.0);

    // Assert
    assert!(none.is_ok(), "zero means every pair clears it");
    assert!(all.is_ok(), "one means only identical pairs clear it");
}

#[test]
fn fraction_of_a_negative_value_names_the_field_it_came_from() {
    // Arrange & Act
    let outcome = Threshold::fraction("similarity_threshold", -0.1);

    // Assert
    let Err(Error::InvalidConfig {
        field,
        value,
        expected,
    }) = outcome
    else {
        panic!("a negative similarity is not a similarity");
    };
    assert_eq!(field, "similarity_threshold");
    assert_eq!(value, "-0.1");
    assert_eq!(expected, "a fraction between 0.0 and 1.0");
}

#[test]
fn fraction_of_a_value_above_one_is_rejected() {
    // Arrange & Act
    let outcome = Threshold::fraction("similarity_threshold", 5.0);

    // Assert
    assert_eq!(
        outcome.unwrap_err().to_string(),
        "similarity_threshold must be a fraction between 0.0 and 1.0, got 5"
    );
}

#[test]
fn fraction_of_nan_is_rejected() {
    // Arrange & Act
    let outcome = Threshold::fraction("similarity_threshold", f64::NAN);

    // Assert
    assert!(
        outcome.is_err(),
        "NaN compares false against every bound, so a range check that admits \
         it admits it silently"
    );
}

#[test]
fn percent_at_either_end_of_its_range_is_accepted() {
    // Arrange & Act
    let none = Threshold::percent("t", 0.0);
    let all = Threshold::percent("t", 100.0);

    // Assert
    assert!(
        none.is_ok(),
        "zero percent is a real ceiling, not an absent one"
    );
    assert!(all.is_ok());
}

#[test]
fn percent_of_a_value_above_a_hundred_is_rejected() {
    // Arrange & Act
    let outcome = Threshold::percent("max_exact_percent", 150.0);

    // Assert
    assert_eq!(
        outcome.unwrap_err().to_string(),
        "max_exact_percent must be a percentage between 0.0 and 100.0, got 150"
    );
}

#[test]
fn percent_of_a_whole_hundred_is_the_whole_fraction() {
    // Arrange & Act
    let threshold = Threshold::percent("t", 100.0).expect("100% is a percentage");

    // Assert
    assert!((threshold.as_fraction() - 1.0).abs() < f64::EPSILON);
}
