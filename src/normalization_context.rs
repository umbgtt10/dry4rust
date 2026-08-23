// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::node::PlaceholderKind;
use std::collections::HashMap;

/// Tracks identifier-to-placeholder mappings during normalization.
pub struct NormalizationContext {
    /// Maps (identifier_string, kind) -> placeholder index
    mappings: HashMap<(String, PlaceholderKind), usize>,
    /// Per-kind counters
    counters: HashMap<PlaceholderKind, usize>,
}

impl NormalizationContext {
    #[must_use]
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
            counters: HashMap::new(),
        }
    }

    /// Get or assign a placeholder index for the given identifier and kind.
    pub fn placeholder(&mut self, name: &str, kind: PlaceholderKind) -> usize {
        let key = (name.to_string(), kind);
        if let Some(&idx) = self.mappings.get(&key) {
            return idx;
        }
        let counter = self.counters.entry(kind).or_insert(0);
        let idx = *counter;
        *counter += 1;
        self.mappings.insert(key, idx);
        idx
    }
}

impl Default for NormalizationContext {
    fn default() -> Self {
        Self::new()
    }
}
