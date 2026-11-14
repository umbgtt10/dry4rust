// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::baseline_recording;
use crate::common::group;
use dry4rust::cli::suppression::baseline_kind::BaselineKind;
use dry4rust::cli::suppression::ignore_entry::IgnoreEntry;
use dry4rust::cli::suppression::ignore_file::IgnoreFile;
use dry4rust::fingerprint::Fingerprint;
use dry4rust::groups::duplicate_group::DuplicateGroup;
use dry4rust::groups::findings::Findings;

fn fingerprints(groups: &[DuplicateGroup]) -> Vec<Fingerprint> {
    groups.iter().map(DuplicateGroup::fingerprint).collect()
}

fn ignoring(groups: &[&DuplicateGroup]) -> IgnoreFile {
    IgnoreFile::new(
        groups
            .iter()
            .map(|group| IgnoreEntry {
                fingerprint: group.fingerprint().to_hex(),
                reason: None,
                members: Vec::new(),
            })
            .collect(),
    )
}

#[test]
fn count_adds_the_groups_of_every_kind() {
    // Arrange
    let findings = Findings::new(
        vec![group(1, &["a", "b"])],
        vec![group(2, &["a", "b"]), group(3, &["a", "b"])],
        vec![
            group(4, &["a", "b"]),
            group(5, &["a", "b"]),
            group(6, &["a", "b"]),
        ],
        vec![
            group(7, &["a", "b"]),
            group(8, &["a", "b"]),
            group(9, &["a", "b"]),
            group(10, &["a", "b"]),
        ],
    );

    // Act
    let count = findings.count();

    // Assert
    assert_eq!(count, 10);
}

#[test]
fn fingerprints_gathers_those_of_every_kind() {
    // Arrange
    let findings = Findings::new(
        vec![group(1, &["a", "b"])],
        vec![group(2, &["a", "b"])],
        vec![group(3, &["a", "b"])],
        vec![group(4, &["a", "b"])],
    );

    // Act
    let gathered = findings.fingerprints();

    // Assert
    assert_eq!(
        gathered.into_iter().collect::<Vec<_>>(),
        vec![
            Fingerprint::new(1),
            Fingerprint::new(2),
            Fingerprint::new(3),
            Fingerprint::new(4)
        ]
    );
}

// A group the baseline recorded under one kind is new under another: each
// kind keeps the group recorded under the next one.
#[test]
fn retain_new_drops_each_group_recorded_under_its_own_kind_only() {
    // Arrange
    let (a, b, c, d) = (
        group(1, &["a", "b"]),
        group(2, &["a", "b"]),
        group(3, &["a", "b"]),
        group(4, &["a", "b"]),
    );
    let baseline = baseline_recording(&[
        (BaselineKind::Exact, &a),
        (BaselineKind::Near, &b),
        (BaselineKind::SubExact, &c),
        (BaselineKind::SubNear, &d),
    ]);
    let findings = Findings::new(
        vec![a.clone(), b.clone()],
        vec![b, c.clone()],
        vec![c, d.clone()],
        vec![d, a],
    );

    // Act
    let new = findings.retain_new(&baseline);

    // Assert
    assert_eq!(fingerprints(new.exact()), vec![Fingerprint::new(2)]);
    assert_eq!(fingerprints(new.near()), vec![Fingerprint::new(3)]);
    assert_eq!(fingerprints(new.sub_exact()), vec![Fingerprint::new(4)]);
    assert_eq!(fingerprints(new.sub_near()), vec![Fingerprint::new(1)]);
}

#[test]
fn retain_unsuppressed_drops_an_ignored_group_of_every_kind() {
    // Arrange
    let (a, b, c, d, kept) = (
        group(1, &["a", "b"]),
        group(2, &["a", "b"]),
        group(3, &["a", "b"]),
        group(4, &["a", "b"]),
        group(5, &["a", "b"]),
    );
    let ignore_file = ignoring(&[&a, &b, &c, &d]);
    let findings = Findings::new(
        vec![a, kept.clone()],
        vec![b, kept.clone()],
        vec![c, kept.clone()],
        vec![d, kept],
    );

    // Act
    let unsuppressed = findings.retain_unsuppressed(&ignore_file);

    // Assert
    assert_eq!(
        fingerprints(unsuppressed.exact()),
        vec![Fingerprint::new(5)]
    );
    assert_eq!(fingerprints(unsuppressed.near()), vec![Fingerprint::new(5)]);
    assert_eq!(
        fingerprints(unsuppressed.sub_exact()),
        vec![Fingerprint::new(5)]
    );
    assert_eq!(
        fingerprints(unsuppressed.sub_near()),
        vec![Fingerprint::new(5)]
    );
}

#[test]
fn stats_counts_the_groups_of_every_kind() {
    // Arrange
    let findings = Findings::new(
        vec![group(1, &["a", "b"])],
        vec![group(2, &["a", "b", "c"]), group(3, &["a", "b"])],
        vec![
            group(4, &["a", "b"]),
            group(5, &["a", "b"]),
            group(6, &["a", "b"]),
        ],
        vec![group(7, &["a", "b", "c", "d"])],
    );

    // Act
    let stats = findings.stats(&[]);

    // Assert
    assert_eq!(stats.exact_duplicate_groups(), 1);
    assert_eq!(stats.near_duplicate_groups(), 2);
    assert_eq!(stats.near_duplicate_units(), 5);
    assert_eq!(stats.sub_exact_groups(), 3);
    assert_eq!(stats.sub_near_groups(), 1);
    assert_eq!(stats.sub_near_units(), 4);
}
