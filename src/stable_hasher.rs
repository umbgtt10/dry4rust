// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// FNV-1a, 64-bit, specified here rather than borrowed.
///
/// Fingerprints are written into `.dry4rust-ignore.toml` and are expected to
/// still match the next time the tool runs -- possibly under a different
/// toolchain. `DefaultHasher` cannot promise that: the standard library says
/// its algorithm is unspecified and not to be relied upon across releases. A
/// suppression that silently stops matching is the worst kind of failure,
/// because the duplicate simply reappears with no explanation.
///
/// So the algorithm lives here, in eight lines anyone can check against the
/// published FNV specification, and every integer is written big-endian at a
/// fixed width so that a 32-bit machine and a 64-bit one agree.
pub struct StableHasher {
    state: u64,
}

/// The FNV-1a 64-bit offset basis.
const OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;

/// The FNV-1a 64-bit prime.
const PRIME: u64 = 0x0000_0100_0000_01b3;

impl StableHasher {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: OFFSET_BASIS,
        }
    }

    /// Absorb raw bytes.
    pub const fn write_bytes(&mut self, bytes: &[u8]) {
        let mut index = 0;
        while index < bytes.len() {
            self.state ^= bytes[index] as u64;
            self.state = self.state.wrapping_mul(PRIME);
            index += 1;
        }
    }

    /// Absorb a single byte.
    pub const fn write_u8(&mut self, value: u8) {
        self.write_bytes(&[value]);
    }

    /// Absorb an integer, big-endian and eight bytes wide regardless of the
    /// platform's word size.
    pub const fn write_u64(&mut self, value: u64) {
        self.write_bytes(&value.to_be_bytes());
    }

    /// Absorb a string, length first.
    ///
    /// The length prefix is what keeps `("ab", "c")` from hashing the same as
    /// `("a", "bc")`, which matters because variant names are written one
    /// after another with nothing between them.
    pub const fn write_str(&mut self, value: &str) {
        self.write_u64(value.len() as u64);
        self.write_bytes(value.as_bytes());
    }

    /// The hash of everything absorbed so far.
    #[must_use]
    pub const fn finish(&self) -> u64 {
        self.state
    }
}

impl Default for StableHasher {
    fn default() -> Self {
        Self::new()
    }
}
