// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use dry4rust::suppression::baseline_entry::BaselineEntry;
use dry4rust::suppression::baseline_kind::BaselineKind;

#[test]
fn admits_a_group_it_recorded_unchanged() {
    // Arrange
    let recorded = group(0x1234, &["one", "two"]);
    let entry = BaselineEntry::of(BaselineKind::Exact, &recorded);

    // Act
    let admitted = entry.admits(BaselineKind::Exact, &recorded);

    // Assert
    assert!(admitted);
}

#[test]
fn admits_a_group_that_has_shrunk_since_it_was_recorded() {
    // Arrange
    let entry = BaselineEntry::of(
        BaselineKind::Exact,
        &group(0x1234, &["one", "two", "three"]),
    );

    // Act
    let admitted = entry.admits(BaselineKind::Exact, &group(0x1234, &["one", "two"]));

    // Assert
    assert!(
        admitted,
        "one copy deleted is progress, and progress is not something to fail on"
    );
}

#[test]
fn admits_nothing_of_another_fingerprint() {
    // Arrange
    let entry = BaselineEntry::of(BaselineKind::Exact, &group(0x1234, &["one", "two"]));

    // Act
    let admitted = entry.admits(BaselineKind::Exact, &group(0x5678, &["one", "two"]));

    // Assert
    assert!(!admitted);
}

#[test]
fn admits_nothing_of_another_kind_at_the_same_fingerprint() {
    // Arrange
    let recorded = group(0x1234, &["one", "two"]);
    let entry = BaselineEntry::of(BaselineKind::SubExact, &recorded);

    // Act
    let admitted = entry.admits(BaselineKind::Exact, &recorded);

    // Assert
    assert!(
        !admitted,
        "a recorded branch does not stand in for a whole function that hashes \
         the same"
    );
}

#[test]
fn admits_nothing_once_the_group_has_grown() {
    // Arrange
    let entry = BaselineEntry::of(BaselineKind::Exact, &group(0x1234, &["one", "two"]));

    // Act
    let admitted = entry.admits(
        BaselineKind::Exact,
        &group(0x1234, &["one", "two", "three"]),
    );

    // Assert
    assert!(
        !admitted,
        "an exact group keeps its fingerprint when a third copy joins it, so \
         without the member count a new copy would be inherited duplication"
    );
}

#[test]
fn of_records_the_fingerprint_the_count_and_the_names() {
    // Arrange
    let recorded = group(0x00ab, &["process_data", "compute_total"]);

    // Act
    let entry = BaselineEntry::of(BaselineKind::Near, &recorded);

    // Assert
    assert_eq!(entry.kind, BaselineKind::Near);
    assert_eq!(entry.fingerprint, "00000000000000ab");
    assert_eq!(entry.members, 2);
    assert_eq!(entry.names, vec!["process_data", "compute_total"]);
}
