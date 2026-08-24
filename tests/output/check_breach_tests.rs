// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use dry4rust::output::check_breach::CheckBreach;

#[test]
fn groups_returns_the_groups_the_breach_was_built_over() {
    // Arrange
    let groups = vec![group(0x11, &["a", "b"])];

    // Act
    let breach = CheckBreach::new(
        String::from("1 exact duplicate groups (max: 0)"),
        &groups,
        true,
    );

    // Assert
    assert_eq!(breach.groups().len(), 1);
    assert_eq!(breach.groups()[0].fingerprint.value(), 0x11);
}

#[test]
fn is_of_exact_distinguishes_which_set_the_groups_came_from() {
    // Arrange
    let groups = vec![group(0x11, &["a", "b"])];

    // Act
    let exact = CheckBreach::new(String::from("exact"), &groups, true);
    let near = CheckBreach::new(String::from("near"), &groups, false);

    // Assert
    assert!(exact.is_of_exact());
    assert!(
        !near.is_of_exact(),
        "the same groups can breach either kind of ceiling, so the breach has \
         to say which it was"
    );
}

#[test]
fn message_returns_the_sentence_it_was_given() {
    // Arrange
    let groups = Vec::new();

    // Act
    let breach = CheckBreach::new(
        String::from("42.9% exact duplicate lines (max: 0.0%)"),
        &groups,
        true,
    );

    // Assert
    assert_eq!(breach.message(), "42.9% exact duplicate lines (max: 0.0%)");
}
