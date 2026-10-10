// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::fingerprint::Fingerprint;
use validation::duplicate_group_fixture::DuplicateGroupFixture;

#[test]
fn of_a_fingerprint_gives_it_to_the_group_and_to_every_member() {
    // Arrange & Act
    let group = DuplicateGroupFixture::of(0xdead_beef, &["new", "copy"]);

    // Assert
    assert_eq!(group.fingerprint, Fingerprint::new(0xdead_beef));
    assert!(
        group
            .members
            .iter()
            .all(|member| member.fingerprint == group.fingerprint)
    );
}

#[test]
fn of_any_members_is_an_exact_group() {
    // Arrange & Act
    let group = DuplicateGroupFixture::of(7, &["left", "right"]);

    // Assert
    assert!((group.similarity - 1.0).abs() < f64::EPSILON);
}

#[test]
fn of_names_holds_one_member_per_name_in_the_order_given() {
    // Arrange & Act
    let group = DuplicateGroupFixture::of(1, &["first", "second", "third"]);

    // Assert
    let names: Vec<&str> = group
        .members
        .iter()
        .map(|member| member.name.as_str())
        .collect();
    assert_eq!(names, ["first", "second", "third"]);
}
