// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::fingerprint::Fingerprint;
use dry4rust::grouper::DuplicateGroup;
use dry4rust::ignore::*;
use dry4rust::node::{LiteralKind, NodeKind, NormalizedNode};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn test_fingerprint() -> Fingerprint {
    Fingerprint::from_node(&NormalizedNode::leaf(NodeKind::Literal(LiteralKind::Int)))
}

#[test]
fn add_ignore_deduplicates() {
    let fp = test_fingerprint();
    let mut ignore = IgnoreFile::default();
    add_ignore(&mut ignore, &fp, None, vec![]);
    add_ignore(&mut ignore, &fp, None, vec![]);
    assert_eq!(ignore.ignore.len(), 1);
}

#[test]
fn filter_ignored_keeps_near_duplicates_without_matching_entry() {
    let fp = test_fingerprint();
    let other_fp = Fingerprint::from_node(&NormalizedNode::with_children(NodeKind::Block, vec![]));
    let mut ignore = IgnoreFile::default();
    add_ignore(&mut ignore, &other_fp, None, vec![]);

    let groups = vec![DuplicateGroup {
        fingerprint: fp,
        members: vec![],
        similarity: 0.85,
    }];

    let filtered = filter_ignored(groups, &ignore);
    assert_eq!(filtered.len(), 1);
}

#[test]
fn filter_ignored_removes_matching_groups() {
    let fp = test_fingerprint();
    let mut ignore = IgnoreFile::default();
    add_ignore(&mut ignore, &fp, None, vec![]);

    let groups = vec![
        DuplicateGroup {
            fingerprint: fp,
            members: vec![],
            similarity: 1.0,
        },
        DuplicateGroup {
            fingerprint: Fingerprint::from_node(&NormalizedNode::leaf(NodeKind::Opaque)),
            members: vec![],
            similarity: 1.0,
        },
    ];

    let filtered = filter_ignored(groups, &ignore);
    assert_eq!(filtered.len(), 1);
}

#[test]
fn filter_ignored_removes_near_duplicates_with_matching_fingerprint() {
    let fp = test_fingerprint();
    let mut ignore = IgnoreFile::default();
    add_ignore(&mut ignore, &fp, None, vec![]);

    let groups = vec![DuplicateGroup {
        fingerprint: fp,
        members: vec![],
        similarity: 0.85,
    }];

    let filtered = filter_ignored(groups, &ignore);
    assert!(filtered.is_empty());
}

#[test]
fn find_stale_entries_identifies_stale_vs_live() {
    let live_fp = test_fingerprint();
    let stale_fp = Fingerprint::from_node(&NormalizedNode::with_children(NodeKind::Block, vec![]));

    let mut ignore = IgnoreFile::default();
    add_ignore(&mut ignore, &live_fp, Some("live".to_string()), vec![]);
    add_ignore(&mut ignore, &stale_fp, Some("stale".to_string()), vec![]);

    let mut live_set = HashSet::new();
    live_set.insert(live_fp);

    let stale = find_stale_entries(&ignore, &live_set);
    assert_eq!(stale.len(), 1);
    assert_eq!(stale[0].reason, Some("stale".to_string()));
}

#[test]
fn ignore_file_path_is_correct() {
    let path = ignore_file_path(Path::new("/project"));
    assert_eq!(path, PathBuf::from("/project/.dupes-ignore.toml"));
}

#[test]
fn is_ignored_works() {
    let fp = test_fingerprint();
    let mut ignore = IgnoreFile::default();
    assert!(!is_ignored(&ignore, &fp));
    add_ignore(&mut ignore, &fp, None, vec![]);
    assert!(is_ignored(&ignore, &fp));
}

#[test]
fn load_nonexistent_returns_default() {
    let tmp = TempDir::new().unwrap();
    let ignore = load_ignore_file(tmp.path());
    assert!(ignore.ignore.is_empty());
}

#[test]
fn remove_ignore_works() {
    let fp = test_fingerprint();
    let mut ignore = IgnoreFile::default();
    add_ignore(&mut ignore, &fp, None, vec![]);
    assert!(remove_ignore(&mut ignore, &fp.to_hex()));
    assert!(ignore.ignore.is_empty());
}

#[test]
fn remove_nonexistent_returns_false() {
    let mut ignore = IgnoreFile::default();
    assert!(!remove_ignore(&mut ignore, "nonexistent"));
}

#[test]
fn remove_stale_entries_removes_only_stale() {
    let live_fp = test_fingerprint();
    let stale_fp = Fingerprint::from_node(&NormalizedNode::with_children(NodeKind::Block, vec![]));

    let mut ignore = IgnoreFile::default();
    add_ignore(&mut ignore, &live_fp, Some("live".to_string()), vec![]);
    add_ignore(&mut ignore, &stale_fp, Some("stale".to_string()), vec![]);

    let mut live_set = HashSet::new();
    live_set.insert(live_fp);

    let removed = remove_stale_entries(&mut ignore, &live_set);
    assert_eq!(removed.len(), 1);
    assert_eq!(removed[0].reason, Some("stale".to_string()));
    assert_eq!(ignore.ignore.len(), 1);
    assert_eq!(ignore.ignore[0].reason, Some("live".to_string()));
}

#[test]
fn roundtrip_save_and_load() {
    let tmp = TempDir::new().unwrap();
    let fp = test_fingerprint();
    let mut ignore = IgnoreFile::default();
    add_ignore(
        &mut ignore,
        &fp,
        Some("test reason".to_string()),
        vec!["foo".to_string(), "bar".to_string()],
    );
    save_ignore_file(tmp.path(), &ignore).unwrap();
    let loaded = load_ignore_file(tmp.path());
    assert_eq!(loaded.ignore.len(), 1);
    assert_eq!(loaded.ignore[0].fingerprint, fp.to_hex());
    assert_eq!(loaded.ignore[0].reason, Some("test reason".to_string()));
    assert_eq!(loaded.ignore[0].members, vec!["foo", "bar"]);
}
