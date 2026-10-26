// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::suppression::ignore_entry::IgnoreEntry;
use dry4rust::cli::suppression::ignore_file::IgnoreFile;
use dry4rust_cli::system::ignore_document::IgnoreDocument;
use toml::from_str;

#[test]
fn deserialize_of_a_file_with_no_table_reads_an_empty_list() {
    // Arrange
    let written = "";

    // Act
    let read: IgnoreDocument = from_str(written).expect("an empty file is a document");

    // Assert
    assert!(read.ignore.is_empty());
}

#[test]
fn from_a_file_and_back_keeps_every_entry_in_order() {
    // Arrange
    let file = IgnoreFile::new(vec![
        IgnoreEntry {
            fingerprint: String::from("0000000000000001"),
            reason: None,
            members: vec![],
        },
        IgnoreEntry {
            fingerprint: String::from("0000000000000002"),
            reason: Some(String::from("kept")),
            members: vec![String::from("a")],
        },
    ]);

    // Act
    let back = IgnoreFile::from(IgnoreDocument::from(&file));

    // Assert
    assert_eq!(back.ignore(), file.ignore());
}
