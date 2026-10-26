// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_source_tree::InMemorySourceTree;
use dry4rust::scanner::*;
use dry4rust::source_path::SourcePath;

fn create_test_tree() -> InMemorySourceTree {
    InMemorySourceTree::new()
        .with_file("/project/src/main.rs", "fn main() {}")
        .with_file("/project/src/lib.rs", "pub mod utils;")
        .with_file("/project/src/utils/helper.rs", "pub fn help() {}")
        .with_file("/project/target/debug/build.rs", "fn build() {}")
        .with_file("/project/.hidden/secret.rs", "fn secret() {}")
        .with_file("/project/src/readme.md", "# README")
}

fn root() -> SourcePath {
    SourcePath::new(String::from("/project"))
}

#[test]
fn admits_a_file_with_one_of_its_extensions_in_any_case() {
    // Arrange
    let scanner = Scanner::new(root());

    // Act
    let admitted = scanner.admits(&SourcePath::new(String::from("/project/src/LIB.RS")));

    // Assert
    assert!(admitted);
}

#[test]
fn admits_no_file_an_exclusion_pattern_names() {
    // Arrange
    let scanner = Scanner::new(root()).with_excludes(vec![String::from("generated")]);

    // Act
    let admitted = scanner.admits(&SourcePath::new(String::from("/project/generated/a.rs")));

    // Assert
    assert!(!admitted);
}

#[test]
fn descends_into_a_hidden_root_but_no_hidden_directory_below_it() {
    // Arrange
    let scanner = Scanner::new(SourcePath::new(String::from("/work/.checkout")));

    // Act
    let root = scanner.descends_into(&SourcePath::new(String::from("/work/.checkout")));
    let below = scanner.descends_into(&SourcePath::new(String::from("/work/.checkout/.git")));

    // Assert
    assert!(root, "the root is what was asked for, hidden or not");
    assert!(!below);
}

#[test]
fn descends_into_no_target_directory_even_at_the_root() {
    // Arrange
    let scanner = Scanner::new(SourcePath::new(String::from("/work/target")));

    // Act
    let entered = scanner.descends_into(&SourcePath::new(String::from("/work/target")));

    // Assert
    assert!(!entered);
}

#[test]
fn is_excluded_works() {
    // Arrange & Act
    let path = SourcePath::new(String::from("/foo/bar/tests/test.rs"));

    // Assert
    assert!(Scanner::is_excluded(&path, &["tests".to_string()]));
    assert!(!Scanner::is_excluded(&path, &["benches".to_string()]));
}

#[test]
fn scan_empty_directory() {
    // Arrange & Act
    let tree = InMemorySourceTree::new();
    let scanner = Scanner::new(root());
    let files = scanner.scan_files(&tree).expect("the tree lists");

    // Assert
    assert!(files.is_empty());
}

#[test]
fn scan_finds_rust_files() {
    // Arrange & Act
    let tree = create_test_tree();
    let scanner = Scanner::new(root());
    let files = scanner.scan_files(&tree).expect("the tree lists");

    // Assert
    assert_eq!(files.len(), 3);
    assert!(files.iter().all(|f| f.extension().unwrap() == "rs"));
}

#[test]
fn scan_respects_exclude_patterns() {
    // Arrange & Act
    let tree = create_test_tree();
    let scanner = Scanner::new(root()).with_excludes(vec!["utils".to_string()]);
    let files = scanner.scan_files(&tree).expect("the tree lists");

    // Assert
    assert!(!files.iter().any(|f| f.as_str().contains("utils")));
    assert_eq!(files.len(), 2);
}

#[test]
fn scan_skips_hidden_directories() {
    // Arrange & Act
    let tree = create_test_tree();
    let scanner = Scanner::new(root());
    let files = scanner.scan_files(&tree).expect("the tree lists");

    // Assert
    assert!(!files.iter().any(|f| f.as_str().contains(".hidden")));
}

#[test]
fn scan_skips_target_directory() {
    // Arrange & Act
    let tree = create_test_tree();
    let scanner = Scanner::new(root());
    let files = scanner.scan_files(&tree).expect("the tree lists");

    // Assert
    assert!(!files.iter().any(|f| f.as_str().contains("target")));
}

#[test]
fn with_extensions_replaces_the_default_and_keeps_the_root() {
    // Arrange
    let scanner = Scanner::new(root());

    // Act
    let scanner = scanner.with_extensions(vec![String::from("toml"), String::from("md")]);

    // Assert
    assert_eq!(scanner.extensions(), vec!["toml", "md"]);
    assert_eq!(scanner.root(), &root());
}
