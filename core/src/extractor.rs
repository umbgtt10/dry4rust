// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::code_unit::CodeUnitKind;
use crate::node::NormalizedNode;
use crate::sub_unit_extractor::SubUnitExtractor;

/// A sub-unit extracted from a normalized function body.
pub struct SubUnit {
    pub kind: CodeUnitKind,
    pub node: NormalizedNode,
    pub node_count: usize,
    pub description: String,
}

/// Extract candidate sub-units from a normalized AST node.
/// Walks the tree recursively and extracts natural compound structures
/// (if branches, match arm bodies, loop bodies, closure bodies).
/// Each sub-tree is re-indexed to canonical placeholder form.
/// Only sub-trees meeting `min_node_count` are returned.
#[must_use]
pub fn extract_sub_units(node: &NormalizedNode, min_node_count: usize) -> Vec<SubUnit> {
    SubUnitExtractor::new(min_node_count).extract(node)
}
