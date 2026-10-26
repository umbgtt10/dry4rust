// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::source_path;
use dry4rust::scanner::Scanner;
use dry4rust::storage_failure::StorageFailure;
use dry4rust::traits::source_tree::SourceTree;
use dry4rust_cli::system::walkdir_source_tree::WalkdirSourceTree;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn create_test_tree(dir: &Path) {
    fs::create_dir_all(dir.join("src/utils")).unwrap();
    fs::create_dir_all(dir.join("target/debug")).unwrap();
    fs::create_dir_all(dir.join(".hidden")).unwrap();
    fs::write(dir.join("src/main.rs"), "fn main() {}").unwrap();
    fs::write(dir.join("src/lib.rs"), "pub mod utils;").unwrap();
    fs::write(dir.join("src/utils/helper.rs"), "pub fn help() {}").unwrap();
    fs::write(dir.join("target/debug/build.rs"), "fn build() {}").unwrap();
    fs::write(dir.join(".hidden/secret.rs"), "fn secret() {}").unwrap();
    fs::write(dir.join("src/readme.md"), "# README").unwrap();
}

#[cfg(target_os = "linux")]
mod not_utf8 {
    use crate::common::source_path;
    use dry4rust::scanner::Scanner;
    use dry4rust::traits::source_tree::SourceTree;
    use dry4rust_cli::system::walkdir_source_tree::WalkdirSourceTree;
    use std::ffi::OsStr;
    use std::fs;
    use std::os::unix::ffi::OsStrExt;
    use tempfile::TempDir;

    #[test]
    fn source_files_with_a_file_name_that_is_not_utf8_fails_naming_it() {
        // Arrange
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join(OsStr::from_bytes(b"\xff.rs")), "fn f() {}").unwrap();
        let scanner = Scanner::new(source_path(tmp.path()));

        // Act
        let outcome = WalkdirSourceTree::new().source_files(&scanner);

        // Assert
        let failure = outcome.expect_err("such a file cannot be held, so it is not skipped");
        assert!(
            failure
                .to_string()
                .ends_with("/\u{FFFD}.rs: the path is not valid UTF-8"),
            "{failure}"
        );
    }

    #[test]
    fn source_files_with_an_excluded_file_name_that_is_not_utf8_passes_it_over() {
        // Arrange
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("vendor")).unwrap();
        fs::write(
            tmp.path()
                .join("vendor")
                .join(OsStr::from_bytes(b"\xff.rs")),
            "fn f() {}",
        )
        .unwrap();
        fs::write(tmp.path().join("lib.rs"), "fn g() {}").unwrap();
        let scanner =
            Scanner::new(source_path(tmp.path())).with_excludes(vec![String::from("vendor")]);

        // Act
        let files = WalkdirSourceTree::new()
            .source_files(&scanner)
            .expect("an excluded file is never named");

        // Assert
        assert_eq!(files.len(), 1);
    }
}

#[test]
fn read_source_of_a_file_on_disk_returns_its_contents() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("lib.rs"), "fn f() {}").unwrap();

    // Act
    let text = WalkdirSourceTree::new().read_source(&source_path(&tmp.path().join("lib.rs")));

    // Assert
    assert_eq!(text, Ok(String::from("fn f() {}")));
}

#[test]
fn read_source_of_a_file_that_does_not_exist_says_it_is_missing() {
    // Arrange
    let tmp = TempDir::new().unwrap();

    // Act
    let outcome = WalkdirSourceTree::new().read_source(&source_path(&tmp.path().join("gone.rs")));

    // Assert
    assert!(
        matches!(outcome, Err(StorageFailure::Missing(_))),
        "{outcome:?}"
    );
}

#[test]
fn source_files_keeps_only_what_the_scanner_admits_where_it_enters() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    create_test_tree(tmp.path());
    let scanner = Scanner::new(source_path(tmp.path())).with_excludes(vec![String::from("utils")]);

    // Act
    let mut files = WalkdirSourceTree::new()
        .source_files(&scanner)
        .expect("every name is UTF-8");

    // Assert
    files.sort();
    let names: Vec<&str> = files.iter().filter_map(|file| file.file_name()).collect();
    assert_eq!(names, vec!["lib.rs", "main.rs"]);
}

#[test]
fn source_files_under_a_root_that_does_not_exist_lists_nothing() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    let scanner = Scanner::new(source_path(&tmp.path().join("gone")));

    // Act
    let files = WalkdirSourceTree::new().source_files(&scanner);

    // Assert
    assert_eq!(files, Ok(Vec::new()));
}
