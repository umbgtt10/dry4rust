// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use validation::dry_binary::DryBinary;
use validation::fixture::Fixture;
use validation::text_report::TextReport;

#[test]
#[should_panic(expected = "the report names a fingerprint")]
fn fingerprint_in_a_report_naming_none_panics() {
    // Arrange
    let report = "Exact Duplicates\n================\n\nNo exact duplicates found.\n";

    // Act & Assert
    let _ = TextReport::fingerprint_in(report);
}

#[test]
fn fingerprint_in_a_report_returns_the_first_fingerprint_it_names() {
    // Arrange
    let report = "Group 1 (fingerprint: 396fa8f6b728ff01, 3 members):\n\
                  Group 2 (fingerprint: 0000000000000002, 2 members):\n";

    // Act
    let fingerprint = TextReport::fingerprint_in(report);

    // Assert
    assert_eq!(fingerprint, "396fa8f6b728ff01");
}

#[test]
fn fingerprint_in_the_report_the_binary_prints_returns_sixteen_hex_digits() {
    // Arrange
    let output = DryBinary::command_in(&Fixture::path("exact_dupes"))
        .arg("report")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let report = String::from_utf8(output).expect("the report is utf-8");

    // Act
    let fingerprint = TextReport::fingerprint_in(&report);

    // Assert
    assert_eq!(fingerprint.len(), 16);
    assert!(fingerprint.chars().all(|digit| digit.is_ascii_hexdigit()));
}
