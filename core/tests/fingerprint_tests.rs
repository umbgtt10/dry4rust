// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::code_unit::CodeUnit;
use dry4rust::fingerprint::*;
use dry4rust::rust::parser::parse_file;
use std::fs;
use tempfile::TempDir;

/// Fingerprints are written into `.dry4rust-ignore.toml` and are expected to
/// still match later. Any change to the normaliser, the encoder or the hasher
/// moves them and silently breaks every suppression anyone has recorded.
///
/// These values are arbitrary. What matters is that they never change without
/// someone deciding they should -- so a failure here is not a bug in the test,
/// it is the format changing, and it needs a note in `CHANGELOG.md` marked
/// BREAKING.
const GOLDEN: &str = r"
fn add_one(x: i32) -> i32 {
    x + 1
}
";

fn only_unit(code: &str) -> CodeUnit {
    let tmp = TempDir::new().expect("temp dir");
    let file = tmp.path().join("golden.rs");
    fs::write(&file, code).expect("write");
    let mut units = parse_file(&file, 1, 0).expect("the sample parses");
    assert_eq!(units.len(), 1, "the sample holds exactly one unit");
    units.remove(0)
}

#[test]
fn composite_fingerprint_deterministic() {
    // Arrange & Act
    let fp1 = Fingerprint::new(42);
    let fp2 = Fingerprint::new(99);
    let a = Fingerprint::from_fingerprints(&[fp1, fp2]);
    let b = Fingerprint::from_fingerprints(&[fp1, fp2]);

    // Assert
    assert_eq!(a, b);
}

#[test]
fn composite_fingerprint_different_sets_differ() {
    // Arrange & Act
    let fp1 = Fingerprint::new(1);
    let fp2 = Fingerprint::new(2);
    let fp3 = Fingerprint::new(3);

    // Assert
    assert_ne!(
        Fingerprint::from_fingerprints(&[fp1, fp2]),
        Fingerprint::from_fingerprints(&[fp2, fp3])
    );
}

#[test]
fn composite_fingerprint_order_independent() {
    // Arrange & Act
    let fp1 = Fingerprint::new(1);
    let fp2 = Fingerprint::new(2);
    let fp3 = Fingerprint::new(3);

    // Assert
    assert_eq!(
        Fingerprint::from_fingerprints(&[fp1, fp2, fp3]),
        Fingerprint::from_fingerprints(&[fp3, fp1, fp2])
    );
}

#[test]
fn fmt_pads_the_hash_to_sixteen_hex_digits() {
    // Arrange & Act
    let fp = Fingerprint::new(0x0000_0000_0000_0042);

    // Assert
    assert_eq!(format!("{fp}"), "0000000000000042");
}

#[test]
fn from_hex_invalid() {
    // Arrange & Act & Assert
    assert!(Fingerprint::from_hex("not_hex").is_none());
}

#[test]
fn parse_file_gives_a_renamed_copy_of_the_golden_sample_the_same_fingerprint() {
    // Arrange
    let renamed = r"
fn increment(value: i32) -> i32 {
    value + 1
}
";

    // Act
    let original = only_unit(GOLDEN).fingerprint;
    let copy = only_unit(renamed).fingerprint;

    // Assert
    assert_eq!(
        original, copy,
        "identifiers normalise to positional placeholders, so a rename is not \
         a different function"
    );
}

#[test]
fn parse_file_gives_a_structurally_different_function_a_different_fingerprint() {
    // Arrange
    let different = r"
fn add_one(x: i32) -> i32 {
    if x > 0 { x + 1 } else { x }
}
";

    // Act
    let original = only_unit(GOLDEN).fingerprint;
    let other = only_unit(different).fingerprint;

    // Assert
    assert_ne!(original, other);
}

#[test]
fn parse_file_gives_the_golden_sample_a_stable_fingerprint() {
    // Arrange & Act
    let unit = only_unit(GOLDEN);

    // Assert
    assert_eq!(
        unit.fingerprint.value(),
        16_368_680_241_809_348_210,
        "the fingerprint format changed. That is a BREAKING change: every          recorded suppression stops matching. If it was deliberate, update          this value and say so in CHANGELOG.md"
    );
}

#[test]
fn to_hex_round_trips_back_through_from_hex() {
    // Arrange & Act
    let fp = Fingerprint::new(0xdead_beef_1234_5678);
    let hex = fp.to_hex();

    // Assert
    assert_eq!(hex, "deadbeef12345678");
    let fp2 = Fingerprint::from_hex(&hex).unwrap();
    assert_eq!(fp, fp2);
}
