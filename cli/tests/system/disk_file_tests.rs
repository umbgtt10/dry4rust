// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::source_path;
use dry4rust::storage_failure::StorageFailure;
use dry4rust_cli::system::disk_file::DiskFile;
use std::fs;
use tempfile::TempDir;

#[test]
fn create_parent_makes_every_directory_above_the_file() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    let file = source_path(&tmp.path().join("a").join("b").join("f.json"));

    // Act
    let outcome = DiskFile::new(&file).create_parent();

    // Assert
    assert_eq!(outcome, Ok(()));
    assert!(tmp.path().join("a").join("b").is_dir());
}

#[test]
fn parsed_of_a_document_the_parser_accepts_returns_what_it_made() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("f.txt"), "42").unwrap();
    let file = source_path(&tmp.path().join("f.txt"));

    // Act
    let outcome = DiskFile::new(&file).parsed(str::parse::<u32>);

    // Assert
    assert_eq!(outcome, Ok(42));
}

#[test]
fn parsed_of_a_document_the_parser_refuses_fails_in_its_words() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("f.txt"), "not a number").unwrap();
    let file = source_path(&tmp.path().join("f.txt"));

    // Act
    let outcome = DiskFile::new(&file).parsed(str::parse::<u32>);

    // Assert
    assert_eq!(
        outcome,
        Err(StorageFailure::Failed(String::from(
            "invalid digit found in string"
        )))
    );
}

#[test]
fn read_of_a_directory_fails_with_what_the_platform_said() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    let directory = source_path(tmp.path());

    // Act
    let outcome = DiskFile::new(&directory).read();

    // Assert
    assert!(
        matches!(outcome, Err(StorageFailure::Failed(_))),
        "{outcome:?}"
    );
}

#[test]
fn read_of_a_file_that_is_not_there_says_it_is_missing() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    let file = source_path(&tmp.path().join("gone.rs"));

    // Act
    let outcome = DiskFile::new(&file).read();

    // Assert
    assert!(
        matches!(outcome, Err(StorageFailure::Missing(_))),
        "{outcome:?}"
    );
}

#[test]
fn remove_takes_the_file_away() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("f.txt"), "x").unwrap();
    let file = source_path(&tmp.path().join("f.txt"));

    // Act
    let outcome = DiskFile::new(&file).remove();

    // Assert
    assert_eq!(outcome, Ok(()));
    assert!(!tmp.path().join("f.txt").exists());
}

#[test]
fn write_then_read_returns_what_was_written() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    let file = source_path(&tmp.path().join("f.txt"));
    let disk = DiskFile::new(&file);

    // Act
    disk.write("hello\n").expect("written");

    // Assert
    assert_eq!(disk.read(), Ok(String::from("hello\n")));
}
