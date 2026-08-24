// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::fingerprint::Fingerprint;
use dry4rust::grouper::DuplicateGroup;
use dry4rust::node::{LiteralKind, NodeKind, NormalizedNode};
use dry4rust::suppression::ignore_file::IgnoreFile;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn group_of(fingerprint: Fingerprint, similarity: f64) -> DuplicateGroup {
    DuplicateGroup {
        fingerprint,
        members: vec![],
        similarity,
    }
}

fn other_fingerprint() -> Fingerprint {
    Fingerprint::from_node(&NormalizedNode::with_children(NodeKind::Block, vec![]))
}

fn suppressing(fingerprints: &[(Fingerprint, Option<&str>)]) -> IgnoreFile {
    fingerprints
        .iter()
        .fold(IgnoreFile::default(), |file, (fp, reason)| {
            file.with_ignored(fp, reason.map(ToOwned::to_owned), vec![])
        })
}

fn test_fingerprint() -> Fingerprint {
    Fingerprint::from_node(&NormalizedNode::leaf(NodeKind::Literal(LiteralKind::Int)))
}

#[test]
fn contains_reports_whether_the_fingerprint_is_suppressed() {
    // Arrange
    let fp = test_fingerprint();
    let empty = IgnoreFile::default();

    // Act
    let before = empty.contains(&fp);
    let after = suppressing(&[(fp, None)]).contains(&fp);

    // Assert
    assert!(!before);
    assert!(after);
}

#[test]
fn load_over_a_root_with_no_ignore_file_returns_an_empty_one() {
    // Arrange
    let tmp = TempDir::new().unwrap();

    // Act
    let ignore = IgnoreFile::load(tmp.path());

    // Assert
    assert!(
        ignore.ignore.is_empty(),
        "most projects suppress nothing, and that is indistinguishable from a \
         project that has not started"
    );
}

#[test]
fn path_in_names_the_file_beside_the_project_root() {
    // Arrange & Act
    let path = IgnoreFile::path_in(Path::new("/project"));

    // Assert
    assert_eq!(path, PathBuf::from("/project/.dry4rust-ignore.toml"));
}

#[test]
fn retain_unsuppressed_keeps_a_near_group_with_no_matching_entry() {
    // Arrange
    let fp = test_fingerprint();
    let ignore = suppressing(&[(other_fingerprint(), None)]);

    // Act
    let kept = ignore.retain_unsuppressed(vec![group_of(fp, 0.85)]);

    // Assert
    assert_eq!(kept.len(), 1);
}

#[test]
fn retain_unsuppressed_removes_a_near_group_whose_fingerprint_matches() {
    // Arrange
    let fp = test_fingerprint();
    let ignore = suppressing(&[(fp, None)]);

    // Act
    let kept = ignore.retain_unsuppressed(vec![group_of(fp, 0.85)]);

    // Assert
    assert!(kept.is_empty());
}

#[test]
fn retain_unsuppressed_removes_the_group_whose_fingerprint_matches() {
    // Arrange
    let fp = test_fingerprint();
    let ignore = suppressing(&[(fp, None)]);
    let groups = vec![
        group_of(fp, 1.0),
        group_of(
            Fingerprint::from_node(&NormalizedNode::leaf(NodeKind::Opaque)),
            1.0,
        ),
    ];

    // Act
    let kept = ignore.retain_unsuppressed(groups);

    // Assert
    assert_eq!(kept.len(), 1);
}

#[test]
fn save_then_load_returns_what_was_recorded() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    let fp = test_fingerprint();
    let ignore = IgnoreFile::default().with_ignored(
        &fp,
        Some("test reason".to_string()),
        vec!["foo".to_string(), "bar".to_string()],
    );

    // Act
    ignore.save(tmp.path()).unwrap();
    let loaded = IgnoreFile::load(tmp.path());

    // Assert
    assert_eq!(loaded.ignore.len(), 1);
    assert_eq!(loaded.ignore[0].fingerprint, fp.to_hex());
    assert_eq!(loaded.ignore[0].reason, Some("test reason".to_string()));
    assert_eq!(loaded.ignore[0].members, vec!["foo", "bar"]);
}

#[test]
fn stale_separates_the_entries_that_no_longer_match_anything() {
    // Arrange
    let live_fp = test_fingerprint();
    let ignore = suppressing(&[
        (live_fp, Some("live")),
        (other_fingerprint(), Some("stale")),
    ]);
    let live: HashSet<Fingerprint> = [live_fp].into_iter().collect();

    // Act
    let stale = ignore.stale(&live);

    // Assert
    assert_eq!(stale.len(), 1);
    assert_eq!(stale[0].reason, Some("stale".to_string()));
}

#[test]
fn with_ignored_over_the_same_fingerprint_twice_records_it_once() {
    // Arrange
    let fp = test_fingerprint();

    // Act
    let ignore = suppressing(&[(fp, None), (fp, None)]);

    // Assert
    assert_eq!(ignore.ignore.len(), 1);
}

#[test]
fn without_an_entry_that_is_not_there_says_so() {
    // Arrange
    let ignore = IgnoreFile::default();

    // Act
    let (ignore, removed) = ignore.without("nonexistent");

    // Assert
    assert!(!removed);
    assert!(ignore.ignore.is_empty());
}

#[test]
fn without_removes_the_entry_and_says_it_did() {
    // Arrange
    let fp = test_fingerprint();
    let ignore = suppressing(&[(fp, None)]);

    // Act
    let (ignore, removed) = ignore.without(&fp.to_hex());

    // Assert
    assert!(removed);
    assert!(ignore.ignore.is_empty());
}

#[test]
fn without_stale_keeps_the_live_entries_and_hands_back_the_rest() {
    // Arrange
    let live_fp = test_fingerprint();
    let ignore = suppressing(&[
        (live_fp, Some("live")),
        (other_fingerprint(), Some("stale")),
    ]);
    let live: HashSet<Fingerprint> = [live_fp].into_iter().collect();

    // Act
    let (kept, taken) = ignore.without_stale(&live);

    // Assert
    assert_eq!(taken.len(), 1);
    assert_eq!(taken[0].reason, Some("stale".to_string()));
    assert_eq!(kept.ignore.len(), 1);
    assert_eq!(kept.ignore[0].reason, Some("live".to_string()));
}
