// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use dry4rust::cli::suppression::baseline_entry::BaselineEntry;
use dry4rust::cli::suppression::baseline_kind::BaselineKind;
use dry4rust_cli::system::baseline_entry_record::BaselineEntryRecord;
use serde_json::from_str;

#[test]
fn deserialize_of_an_entry_without_names_reads_none() {
    // Arrange
    let written = r#"{"kind": "near", "fingerprint": "0000000000000011", "members": 2}"#;

    // Act
    let read: BaselineEntryRecord = from_str(written).expect("names are decoration");

    // Assert
    assert!(read.names.is_empty());
    assert_eq!(read.members, 2);
}

#[test]
fn from_an_entry_and_back_returns_what_was_recorded() {
    // Arrange
    let original = BaselineEntry::of(BaselineKind::SubNear, &group(0x11, &["a", "b"]));

    // Act
    let back = BaselineEntry::from(BaselineEntryRecord::from(&original));

    // Assert
    assert_eq!(back, original);
}
