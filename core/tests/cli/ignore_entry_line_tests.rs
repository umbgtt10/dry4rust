// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::ignore_entry_line::IgnoreEntryLine;
use dry4rust::suppression::ignore_entry::IgnoreEntry;

fn entry(reason: Option<&str>, members: &[&str]) -> IgnoreEntry {
    IgnoreEntry {
        fingerprint: String::from("deadbeef12345678"),
        reason: reason.map(ToOwned::to_owned),
        members: members.iter().map(|m| (*m).to_owned()).collect(),
    }
}

#[test]
fn fmt_never_ends_the_line_itself_so_the_caller_decides_the_newline() {
    // Arrange
    let entry = entry(Some("deliberate"), &["a"]);

    // Act
    let line = IgnoreEntryLine::new(&entry).to_string();

    // Assert
    assert!(!line.ends_with('\n'));
}

#[test]
fn fmt_of_a_bare_entry_is_the_indented_fingerprint_alone() {
    // Arrange
    let entry = entry(None, &[]);

    // Act
    let line = IgnoreEntryLine::new(&entry).to_string();

    // Assert
    assert_eq!(line, "  deadbeef12345678");
}

#[test]
fn fmt_of_an_entry_with_a_reason_names_the_reason() {
    // Arrange
    let entry = entry(Some("trait impls are meant to look alike"), &[]);

    // Act
    let line = IgnoreEntryLine::new(&entry).to_string();

    // Assert
    assert_eq!(
        line,
        "  deadbeef12345678 (reason: trait impls are meant to look alike)"
    );
}

#[test]
fn fmt_of_an_entry_with_both_puts_the_reason_before_the_members() {
    // Arrange
    let entry = entry(Some("deliberate"), &["a", "b"]);

    // Act
    let line = IgnoreEntryLine::new(&entry).to_string();

    // Assert
    assert_eq!(line, "  deadbeef12345678 (reason: deliberate) [a, b]");
}

#[test]
fn fmt_of_an_entry_with_members_lists_them_comma_separated() {
    // Arrange
    let entry = entry(None, &["sum_positive", "count_positive"]);

    // Act
    let line = IgnoreEntryLine::new(&entry).to_string();

    // Assert
    assert_eq!(line, "  deadbeef12345678 [sum_positive, count_positive]");
}
