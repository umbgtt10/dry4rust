// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::fingerprint::*;

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
fn display_format() {
    // Arrange & Act
    let fp = Fingerprint::new(0x0000000000000042);

    // Assert
    assert_eq!(format!("{fp}"), "0000000000000042");
}

#[test]
fn from_hex_invalid() {
    // Arrange & Act & Assert
    assert!(Fingerprint::from_hex("not_hex").is_none());
}

#[test]
fn hex_roundtrip() {
    // Arrange & Act
    let fp = Fingerprint::new(0xdeadbeef12345678);
    let hex = fp.to_hex();

    // Assert
    assert_eq!(hex, "deadbeef12345678");
    let fp2 = Fingerprint::from_hex(&hex).unwrap();
    assert_eq!(fp, fp2);
}
