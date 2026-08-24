// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::near_duplicate::similarity_pair::SimilarityPair;

#[test]
fn key_is_the_same_whichever_way_round_the_pair_was_built() {
    // Arrange
    let forwards = SimilarityPair::new(2, 7, 0.9);
    let backwards = SimilarityPair::new(7, 2, 0.9);

    // Act
    let (first, second) = (forwards.key(), backwards.key());

    // Assert
    assert_eq!(
        first, second,
        "similarity is symmetric, so the lookup key must be too"
    );
}

#[test]
fn key_of_a_pair_with_equal_indices_returns_that_index_twice() {
    // Arrange
    let pair = SimilarityPair::new(4, 4, 1.0);

    // Act
    let key = pair.key();

    // Assert
    assert_eq!(key, (4, 4));
}

#[test]
fn key_puts_the_lower_index_first() {
    // Arrange
    let pair = SimilarityPair::new(9, 3, 0.75);

    // Act
    let key = pair.key();

    // Assert
    assert_eq!(key, (3, 9));
}

#[test]
fn new_keeps_the_indices_in_the_order_they_were_given() {
    // Arrange & Act
    let pair = SimilarityPair::new(9, 3, 0.75);

    // Assert
    assert_eq!(
        pair.left, 9,
        "the pair records the comparison as it was made"
    );
    assert_eq!(pair.right, 3);
    assert!((pair.score - 0.75).abs() < f64::EPSILON);
}
