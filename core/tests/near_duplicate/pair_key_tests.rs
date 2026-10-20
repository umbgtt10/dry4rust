// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::near_duplicate::pair_key::PairKey;

#[test]
fn new_keeps_equal_indices_as_both_ends() {
    // Arrange & Act
    let key = PairKey::new(4, 4);

    // Assert
    assert_eq!(key.low, 4);
    assert_eq!(key.high, 4);
}

#[test]
fn new_puts_the_lower_index_low_whichever_way_round_it_was_given() {
    // Arrange & Act
    let forwards = PairKey::new(3, 9);
    let backwards = PairKey::new(9, 3);

    // Assert
    assert_eq!(forwards, backwards);
    assert_eq!(forwards.low, 3);
    assert_eq!(forwards.high, 9);
}
