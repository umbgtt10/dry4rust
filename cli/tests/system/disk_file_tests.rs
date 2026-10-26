// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::source_path;
use dry4rust::storage_failure::StorageFailure;
use dry4rust_cli::system::disk_file::DiskFile;
use tempfile::TempDir;

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
