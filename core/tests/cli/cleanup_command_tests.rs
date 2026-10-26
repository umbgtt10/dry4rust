// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::result_with;
use crate::support::in_memory_ignore_store::InMemoryIgnoreStore;
use dry4rust::cli::cleanup_command::CleanupCommand;
use dry4rust::cli::ignore_command::IgnoreCommand;
use dry4rust::cli::suppression::ignore_file::IgnoreFile;
use dry4rust::source_path::SourcePath;

fn cleaned(dry_run: bool) -> (InMemoryIgnoreStore, String) {
    let store = InMemoryIgnoreStore::new();
    let root = root();
    IgnoreCommand::new(&store, &root, "deadbeef12345678", None)
        .run()
        .expect("ignore accepted");
    let result = result_with(vec![], vec![], vec![], vec![]);

    let text = CleanupCommand::new(&store, &root, &result, dry_run)
        .run()
        .expect("cleanup succeeds")
        .text()
        .to_string();

    (store, text)
}

fn root() -> SourcePath {
    SourcePath::new(String::from("/p"))
}

#[test]
fn run_in_dry_run_names_the_stale_entry_and_leaves_the_store_holding_it() {
    // Arrange & Act
    let (store, text) = cleaned(true);

    // Assert
    assert!(text.contains("deadbeef12345678"), "{text}");
    assert!(text.contains("would be removed"), "{text}");
    let kept = store
        .held(&IgnoreFile::path_in(&root()))
        .expect("the file is still held");
    assert_eq!(kept.ignore()[0].fingerprint, "deadbeef12345678");
}

#[test]
fn run_outside_dry_run_takes_away_a_file_left_with_nothing_in_it() {
    // Arrange & Act
    let (store, text) = cleaned(false);

    // Assert
    assert!(text.contains("Removed 1 stale entries"), "{text}");
    assert!(!store.holds(&IgnoreFile::path_in(&root())));
}

#[test]
fn run_with_nothing_stale_says_so_and_saves_nothing() {
    // Arrange
    let store = InMemoryIgnoreStore::new();
    let result = result_with(vec![], vec![], vec![], vec![]);

    // Act
    let text = CleanupCommand::new(&store, &root(), &result, false)
        .run()
        .expect("cleanup succeeds")
        .text()
        .to_string();

    // Assert
    assert_eq!(text, "No stale entries found.\n");
    assert!(!store.holds(&IgnoreFile::path_in(&root())));
}
