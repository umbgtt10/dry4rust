// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::node_encoder::NodeEncoder;
use crate::stable_hasher::StableHasher;
use std::fmt;

use crate::node::NormalizedNode;

/// A fingerprint of a normalized AST node, wrapping a u64 hash.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Fingerprint(u64);

impl Fingerprint {
    /// Wrap a known hash value. Exists so a caller -- in practice a test --
    /// can name a specific fingerprint without reaching into the field.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Compute a fingerprint from a normalized node.
    #[must_use]
    pub fn from_node(node: &NormalizedNode) -> Self {
        let mut encoder = NodeEncoder::new();
        encoder.encode(node);
        Self(encoder.finish())
    }

    /// Compute a fingerprint from a signature + body pair.
    #[must_use]
    pub fn from_sig_and_body(sig: &NormalizedNode, body: &NormalizedNode) -> Self {
        let mut encoder = NodeEncoder::new();
        encoder.encode(sig);
        encoder.encode(body);
        Self(encoder.finish())
    }

    /// Compute a composite fingerprint from a set of fingerprints.
    /// Sorts by u64 value for order-independence, then hashes the sorted sequence.
    #[must_use]
    pub fn from_fingerprints(fps: &[Self]) -> Self {
        let mut sorted: Vec<u64> = fps.iter().map(|fp| fp.0).collect();
        sorted.sort_unstable();
        let mut hasher = StableHasher::new();
        for value in &sorted {
            hasher.write_u64(*value);
        }
        Self(hasher.finish())
    }

    /// Get the raw u64 value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Convert to hex string.
    #[must_use]
    pub fn to_hex(self) -> String {
        format!("{:016x}", self.0)
    }

    /// Parse from hex string.
    pub fn from_hex(s: &str) -> Option<Self> {
        u64::from_str_radix(s, 16).ok().map(Fingerprint)
    }
}

impl fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}
