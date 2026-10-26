// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_ignore_store::InMemoryIgnoreStore;
use dry4rust::cli::suppression::ignore_file::IgnoreFile;
use dry4rust::duplicate_group::DuplicateGroup;
use dry4rust::fingerprint::Fingerprint;
use dry4rust::node::literal_kind::LiteralKind;
use dry4rust::node::node_kind::NodeKind;
use dry4rust::node::normalized_node::NormalizedNode;
use dry4rust::source_path::SourcePath;
use dry4rust::storage_failure::StorageFailure;
use std::collections::BTreeSet;
use std::iter::once;

const fn group_of(fingerprint: Fingerprint, similarity: f64) -> DuplicateGroup {
    DuplicateGroup::new(fingerprint, vec![], similarity)
}

fn other_fingerprint() -> Fingerprint {
    Fingerprint::from_node(&NormalizedNode::with_children(NodeKind::Block, vec![]))
}

fn root() -> SourcePath {
    SourcePath::new(String::from("/project"))
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
    let store = InMemoryIgnoreStore::new();

    // Act
    let ignore = IgnoreFile::load(&store, &root());

    // Assert
    assert!(
        ignore.ignore().is_empty(),
        "most projects suppress nothing, and that is indistinguishable from a \
         project that has not started"
    );
}

#[test]
fn load_over_a_store_that_cannot_read_it_returns_an_empty_one() {
    // Arrange
    let store = InMemoryIgnoreStore::unreadable("expected `=` at line 1");

    // Act
    let ignore = IgnoreFile::load(&store, &root());

    // Assert
    assert!(
        ignore.ignore().is_empty(),
        "a file the tool cannot read suppresses nothing, as it always has"
    );
}

#[test]
fn path_in_names_the_file_beside_the_project_root() {
    // Arrange & Act
    let path = IgnoreFile::path_in(&root());

    // Assert
    assert_eq!(
        path,
        SourcePath::new(String::from("/project/.dry4rust-ignore.toml"))
    );
}

#[test]
fn remove_from_a_root_with_no_ignore_file_is_not_an_error() {
    // Arrange
    let store = InMemoryIgnoreStore::new();

    // Act
    let outcome = IgnoreFile::remove_from(&store, &root());

    // Assert
    assert!(
        outcome.is_ok(),
        "taking away what was never there is the state the caller asked for"
    );
}

#[test]
fn remove_from_a_store_that_cannot_remove_it_reports_why() {
    // Arrange
    let store = InMemoryIgnoreStore::failing("Permission denied (os error 13)");

    // Act
    let outcome = IgnoreFile::remove_from(&store, &root());

    // Assert
    assert_eq!(
        outcome,
        Err(StorageFailure::Failed(String::from(
            "Permission denied (os error 13)"
        )))
    );
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
fn save_of_a_file_with_entries_hands_it_to_the_store() {
    // Arrange
    let store = InMemoryIgnoreStore::new();
    let fp = test_fingerprint();

    // Act
    suppressing(&[(fp, Some("kept"))])
        .save(&store, &root())
        .expect("the store takes it");

    // Assert
    let held = store
        .held(&IgnoreFile::path_in(&root()))
        .expect("held beside the root");
    assert_eq!(held.ignore().len(), 1);
    assert_eq!(held.ignore()[0].reason.as_deref(), Some("kept"));
}

#[test]
fn save_of_an_emptied_file_asks_the_store_to_take_it_away() {
    // Arrange
    let store = InMemoryIgnoreStore::new();
    let fp = test_fingerprint();
    suppressing(&[(fp, None)])
        .save(&store, &root())
        .expect("the first save writes it");
    let (emptied, _) = IgnoreFile::load(&store, &root()).without(&fp.to_hex());

    // Act
    emptied
        .save(&store, &root())
        .expect("saving nothing succeeds");

    // Assert
    assert!(!store.holds(&IgnoreFile::path_in(&root())));
}

#[test]
fn stale_separates_the_entries_that_no_longer_match_anything() {
    // Arrange
    let live_fp = test_fingerprint();
    let ignore = suppressing(&[
        (live_fp, Some("live")),
        (other_fingerprint(), Some("stale")),
    ]);
    let live: BTreeSet<Fingerprint> = once(live_fp).collect();

    // Act
    let stale = ignore.stale(&live);

    // Assert
    assert_eq!(stale.len(), 1);
    assert_eq!(stale[0].reason.as_deref(), Some("stale"));
}

#[test]
fn with_ignored_over_the_same_fingerprint_twice_records_it_once() {
    // Arrange
    let fp = test_fingerprint();

    // Act
    let ignore = suppressing(&[(fp, None), (fp, None)]);

    // Assert
    assert_eq!(ignore.ignore().len(), 1);
}

#[test]
fn without_an_entry_that_is_not_there_says_so() {
    // Arrange
    let ignore = IgnoreFile::default();

    // Act
    let (ignore, removed) = ignore.without("nonexistent");

    // Assert
    assert!(!removed);
    assert!(ignore.ignore().is_empty());
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
    assert!(ignore.ignore().is_empty());
}

#[test]
fn without_stale_keeps_the_live_entries_and_hands_back_the_rest() {
    // Arrange
    let live_fp = test_fingerprint();
    let ignore = suppressing(&[
        (live_fp, Some("live")),
        (other_fingerprint(), Some("stale")),
    ]);
    let live: BTreeSet<Fingerprint> = once(live_fp).collect();

    // Act
    let (kept, taken) = ignore.without_stale(&live);

    // Assert
    assert_eq!(taken.len(), 1);
    assert_eq!(taken[0].reason.as_deref(), Some("stale"));
    assert_eq!(kept.ignore().len(), 1);
    assert_eq!(kept.ignore()[0].reason.as_deref(), Some("live"));
}
