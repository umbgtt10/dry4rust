// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::code_unit_kind::CodeUnitKind;
use crate::node::normalized_node::NormalizedNode;
use crate::sub_unit_extractor::SubUnitExtractor;

/// A sub-unit extracted from a normalized function body.
pub struct SubUnit {
    kind: CodeUnitKind,
    node: NormalizedNode,
    node_count: usize,
    description: String,
}

impl SubUnit {
    /// Extract candidate sub-units from a normalized AST node.
    /// Walks the tree recursively and extracts natural compound structures
    /// (if branches, match arm bodies, loop bodies, closure bodies).
    /// Each sub-tree is re-indexed to canonical placeholder form.
    /// Only sub-trees meeting `min_node_count` are returned.
    #[must_use]
    pub fn extract_sub_units(node: &NormalizedNode, min_node_count: usize) -> Vec<Self> {
        SubUnitExtractor::new(min_node_count).extract(node)
    }

    #[must_use]
    pub const fn new(
        kind: CodeUnitKind,
        node: NormalizedNode,
        node_count: usize,
        description: String,
    ) -> Self {
        Self {
            kind,
            node,
            node_count,
            description,
        }
    }

    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub const fn kind(&self) -> &CodeUnitKind {
        &self.kind
    }

    #[must_use]
    pub const fn node(&self) -> &NormalizedNode {
        &self.node
    }

    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.node_count
    }
}
