// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use crate::node::node_kind::NodeKind;
use crate::node::normalized_node::NormalizedNode;
use crate::node::placeholder_kind::PlaceholderKind;

/// Gathers every placeholder occurrence in first-occurrence depth-first order,
/// deduplicating by (kind, old index) so a placeholder is recorded once.
pub struct PlaceholderOrderCollector {
    order: Vec<(PlaceholderKind, usize)>,
    seen: BTreeSet<(PlaceholderKind, usize)>,
}

impl PlaceholderOrderCollector {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            order: Vec::new(),
            seen: BTreeSet::new(),
        }
    }

    /// Visit `node` and its descendants, recording each placeholder the first
    /// time it is seen.
    pub fn collect(&mut self, node: &NormalizedNode) {
        if let NodeKind::Placeholder(kind, idx)
        | NodeKind::PatPlaceholder(kind, idx)
        | NodeKind::TypePlaceholder(kind, idx) = node.kind()
            && self.seen.insert((*kind, *idx))
        {
            self.order.push((*kind, *idx));
        }
        for child in node.children() {
            self.collect(child);
        }
    }

    #[must_use]
    pub fn into_order(self) -> Vec<(PlaceholderKind, usize)> {
        self.order
    }
}

impl Default for PlaceholderOrderCollector {
    fn default() -> Self {
        Self::new()
    }
}
