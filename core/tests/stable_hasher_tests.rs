// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::stable_hasher::StableHasher;

fn hash_of(bytes: &[u8]) -> u64 {
    let mut hasher = StableHasher::new();
    hasher.write_bytes(bytes);
    hasher.finish()
}

#[test]
fn finish_on_an_untouched_hasher_returns_the_fnv_offset_basis() {
    // Arrange & Act
    let empty = StableHasher::new().finish();

    // Assert
    assert_eq!(
        empty, 0xcbf2_9ce4_8422_2325,
        "the published FNV-1a 64-bit basis; if this changes, every recorded \
         suppression stops matching"
    );
}

#[test]
fn new_and_default_start_from_the_same_state() {
    // Arrange & Act
    let explicit = StableHasher::new().finish();
    let defaulted = StableHasher::default().finish();

    // Assert
    assert_eq!(explicit, defaulted);
}

#[test]
fn write_bytes_matches_the_published_fnv_1a_vector_for_a() {
    // Arrange & Act
    let hash = hash_of(b"a");

    // Assert
    assert_eq!(hash, 0xaf63_dc4c_8601_ec8c);
}

#[test]
fn write_bytes_matches_the_published_fnv_1a_vector_for_foobar() {
    // Arrange & Act
    let hash = hash_of(b"foobar");

    // Assert
    assert_eq!(hash, 0x8594_4171_f739_67e8);
}

#[test]
fn write_bytes_over_two_calls_matches_one_call_with_the_whole_input() {
    // Arrange & Act
    let split = {
        let mut hasher = StableHasher::new();
        hasher.write_bytes(b"foo");
        hasher.write_bytes(b"bar");
        hasher.finish()
    };

    // Assert
    assert_eq!(split, hash_of(b"foobar"), "the hasher is a running state");
}

#[test]
fn write_str_distinguishes_inputs_that_concatenate_to_the_same_bytes() {
    // Arrange
    let first = {
        let mut hasher = StableHasher::new();
        hasher.write_str("ab");
        hasher.write_str("c");
        hasher.finish()
    };
    let second = {
        let mut hasher = StableHasher::new();
        hasher.write_str("a");
        hasher.write_str("bc");
        hasher.finish()
    };

    // Act & Assert
    assert_ne!(
        first, second,
        "the length prefix is what keeps adjacent variant names apart"
    );
}

#[test]
fn write_u64_is_eight_bytes_big_endian_regardless_of_the_platform() {
    // Arrange
    let written = {
        let mut hasher = StableHasher::new();
        hasher.write_u64(1);
        hasher.finish()
    };

    // Act
    let spelled_out = hash_of(&[0, 0, 0, 0, 0, 0, 0, 1]);

    // Assert
    assert_eq!(
        written, spelled_out,
        "a native-endian write would have hashed 1 as 01 00 00 00 00 00 00 00"
    );
}

#[test]
fn write_u8_absorbs_exactly_one_byte() {
    // Arrange & Act
    let single = {
        let mut hasher = StableHasher::new();
        hasher.write_u8(0x41);
        hasher.finish()
    };

    // Assert
    assert_eq!(single, hash_of(b"A"));
}
