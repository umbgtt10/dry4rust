// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::scanner::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn create_test_tree(dir: &Path) {
    fs::create_dir_all(dir.join("src")).unwrap();
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

#[test]
fn is_excluded_works() {
    // Arrange & Act
    let path = Path::new("/foo/bar/tests/test.rs");

    // Assert
    assert!(is_excluded(path, &["tests".to_string()]));
    assert!(!is_excluded(path, &["benches".to_string()]));
}

#[test]
fn scan_empty_directory() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    let config = ScanConfig::new(tmp.path().to_path_buf());
    let files = scan_files(&config);

    // Assert
    assert!(files.is_empty());
}

#[test]
fn scan_finds_rust_files() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    create_test_tree(tmp.path());
    let config = ScanConfig::new(tmp.path().to_path_buf());
    let files = scan_files(&config);

    // Assert
    assert_eq!(files.len(), 3);
    assert!(files.iter().all(|f| f.extension().unwrap() == "rs"));
}

#[test]
fn scan_respects_exclude_patterns() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    create_test_tree(tmp.path());
    let config = ScanConfig::new(tmp.path().to_path_buf()).with_excludes(vec!["utils".to_string()]);
    let files = scan_files(&config);

    // Assert
    assert!(!files.iter().any(|f| f.to_string_lossy().contains("utils")));
    assert_eq!(files.len(), 2);
}

#[test]
fn scan_skips_hidden_directories() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    create_test_tree(tmp.path());
    let config = ScanConfig::new(tmp.path().to_path_buf());
    let files = scan_files(&config);

    // Assert
    assert!(
        !files
            .iter()
            .any(|f| f.to_string_lossy().contains(".hidden"))
    );
}

#[test]
fn scan_skips_target_directory() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    create_test_tree(tmp.path());
    let config = ScanConfig::new(tmp.path().to_path_buf());
    let files = scan_files(&config);

    // Assert
    assert!(!files.iter().any(|f| f.to_string_lossy().contains("target")));
}
