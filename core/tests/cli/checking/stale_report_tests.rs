// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::checking::stale_report::StaleReport;
use dry4rust::suppression::ignore_entry::IgnoreEntry;

fn entry(fingerprint: &str) -> IgnoreEntry {
    IgnoreEntry {
        fingerprint: fingerprint.to_string(),
        reason: None,
        members: Vec::new(),
    }
}

#[test]
fn entries_come_back_in_the_order_they_were_given() {
    // Arrange
    let held = vec![entry("aaaa"), entry("bbbb")];
    let report = StaleReport::dry_run(held.iter().collect());

    // Act
    let fingerprints: Vec<&str> = report
        .entries()
        .iter()
        .map(|e| e.fingerprint.as_str())
        .collect();

    // Assert
    assert_eq!(fingerprints, ["aaaa", "bbbb"]);
}

#[test]
fn heading_distinguishes_a_dry_run_from_a_removal() {
    // Arrange
    let held = vec![entry("aaaa")];

    // Act
    let planned = StaleReport::dry_run(held.iter().collect());
    let done = StaleReport::removed(held.iter().collect());

    // Assert
    assert_eq!(planned.heading(), "Stale entries (dry run):");
    assert_eq!(done.heading(), "Removed stale entries:");
}

#[test]
fn is_empty_reports_an_ignore_file_with_nothing_stale_in_it() {
    // Arrange & Act
    let report = StaleReport::dry_run(Vec::new());

    // Assert
    assert!(report.is_empty());
}

#[test]
fn is_empty_reports_false_once_something_is_stale() {
    // Arrange
    let held = vec![entry("aaaa")];

    // Act
    let report = StaleReport::removed(held.iter().collect());

    // Assert
    assert!(!report.is_empty());
}

#[test]
fn summary_of_a_dry_run_speaks_of_what_would_happen() {
    // Arrange
    let held = vec![entry("aaaa"), entry("bbbb"), entry("cccc")];

    // Act
    let summary = StaleReport::dry_run(held.iter().collect()).summary();

    // Assert
    assert_eq!(summary, "\n3 stale entries would be removed.");
}

#[test]
fn summary_of_a_removal_speaks_of_what_did() {
    // Arrange
    let held = vec![entry("aaaa"), entry("bbbb")];

    // Act
    let summary = StaleReport::removed(held.iter().collect()).summary();

    // Assert
    assert_eq!(
        summary, "\nRemoved 2 stale entries.",
        "the tense is the only difference between the two halves of cleanup"
    );
}
