// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::suppression::ignore_entry::IgnoreEntry;
use toml::from_str;
use toml::to_string_pretty;

fn entry(reason: Option<&str>, members: &[&str]) -> IgnoreEntry {
    IgnoreEntry {
        fingerprint: String::from("deadbeef12345678"),
        reason: reason.map(ToOwned::to_owned),
        members: members.iter().map(|m| (*m).to_owned()).collect(),
    }
}

#[test]
fn deserialize_defaults_the_two_optional_fields_when_the_file_omits_them() {
    // Arrange
    let written = "fingerprint = \"deadbeef12345678\"\n";

    // Act
    let read: IgnoreEntry = from_str(written).expect("a bare fingerprint is a whole entry");

    // Assert
    assert_eq!(read.reason, None);
    assert!(read.members.is_empty());
}

#[test]
fn serialize_omits_a_reason_that_was_never_given() {
    // Arrange
    let entry = entry(None, &[]);

    // Act
    let written = to_string_pretty(&entry).expect("an entry serializes");

    // Assert
    assert!(
        !written.contains("reason"),
        "a hand-maintained file should not gain empty keys nobody wrote, got: {written}"
    );
    assert!(!written.contains("members"), "{written}");
}

#[test]
fn serialize_then_deserialize_returns_what_was_recorded() {
    // Arrange
    let entry = entry(Some("trait impls are meant to look alike"), &["a", "b"]);

    // Act
    let written = to_string_pretty(&entry).expect("an entry serializes");
    let read: IgnoreEntry = from_str(&written).expect("and reads back");

    // Assert
    assert_eq!(read, entry);
}
