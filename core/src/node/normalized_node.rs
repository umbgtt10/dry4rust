// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use crate::node::node_kind::NodeKind;
use crate::node::placeholder_kind::PlaceholderKind;
use crate::node::placeholder_order_collector::PlaceholderOrderCollector;

/// A normalized AST node. Uses a data-driven `{ kind, children }` representation
/// instead of a large enum with differently-shaped variants. This allows generic
/// traversal algorithms (count_nodes, reindex, count_matching, extract) to work
/// without exhaustive matching on every variant.
///
/// ## Child ordering conventions
///
/// - **Fixed with None sentinels** (always same child count):
///   - `If` -> [condition, then_branch, else_or_None]
///   - `LetBinding` -> [pattern, type_or_None, init_or_None, diverge_or_None]
///   - `Range` / `PatRange` -> [from_or_None, to_or_None]
///   - `MatchArm` -> [pattern, guard_or_None, body]
/// - **Fixed children first, variable after** (for zip alignment):
///   - `Call` -> [func, arg0, arg1, ...]
///   - `MethodCall` -> [receiver, method, arg0, ...]
///   - `Closure` -> [body, param0, ...]
///   - `FnSignature` -> [return_type_or_None, param0, ...]
///   - `Match` -> [expr, arm0, arm1, ...]
///   - `StructInit` -> [rest_or_None, field0, field1, ...]
///   - `MacroCall` -> [arg0, arg1, ...]
/// - **Variable-length (0 or 1)**: `Return`, `Break` -> `[]` or `[value]`
/// - **Homogeneous**: `Block`, `Tuple`, `Array`, `Path`, `PatTuple`, etc. -> [elem0, ...]
/// - **All other fixed**: e.g. `BinaryOp` -> [left, right], `ForLoop` -> [pat, iter, body]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct NormalizedNode {
    kind: NodeKind,
    children: Vec<Self>,
}

impl NormalizedNode {
    /// Create a None sentinel node.
    #[must_use]
    pub const fn none() -> Self {
        Self::leaf(NodeKind::None)
    }

    /// Create a leaf node (no children).
    #[must_use]
    pub const fn leaf(kind: NodeKind) -> Self {
        Self {
            kind,
            children: vec![],
        }
    }

    /// Convert an `Option<NormalizedNode>` to a node, using the [`Self::none`] sentinel for
    /// absent values.
    pub fn opt(node: Option<Self>) -> Self {
        node.unwrap_or_else(Self::none)
    }

    /// Re-index all placeholders in a sub-tree so that indices start from 0
    /// per kind, assigned by first-occurrence depth-first order.
    /// This allows comparing sub-trees extracted from different function contexts.
    #[must_use]
    pub fn reindex_placeholders(node: &Self) -> Self {
        let mut collector = PlaceholderOrderCollector::new();
        collector.collect(node);
        let order = collector.into_order();

        // Build mapping: (kind, old_index) -> new sequential index per kind
        let mut counters: HashMap<PlaceholderKind, usize> = HashMap::new();
        let mut mapping: HashMap<(PlaceholderKind, usize), usize> = HashMap::new();
        for (kind, old_idx) in order {
            let counter = counters.entry(kind).or_insert(0);
            mapping.insert((kind, old_idx), *counter);
            *counter += 1;
        }

        Self::apply_reindex(node, &mapping)
    }

    /// Applies the reindex mapping to a node, returning a new node with remapped indices.
    fn apply_reindex(node: &Self, mapping: &HashMap<(PlaceholderKind, usize), usize>) -> Self {
        let kind = match &node.kind {
            NodeKind::Placeholder(kind, idx) => {
                let new_idx = mapping.get(&(*kind, *idx)).copied().unwrap_or(*idx);
                NodeKind::Placeholder(*kind, new_idx)
            }
            NodeKind::PatPlaceholder(kind, idx) => {
                let new_idx = mapping.get(&(*kind, *idx)).copied().unwrap_or(*idx);
                NodeKind::PatPlaceholder(*kind, new_idx)
            }
            NodeKind::TypePlaceholder(kind, idx) => {
                let new_idx = mapping.get(&(*kind, *idx)).copied().unwrap_or(*idx);
                NodeKind::TypePlaceholder(*kind, new_idx)
            }
            other => other.clone(),
        };
        let children = node
            .children
            .iter()
            .map(|c| Self::apply_reindex(c, mapping))
            .collect();
        Self { kind, children }
    }

    #[must_use]
    pub const fn new(kind: NodeKind, children: Vec<Self>) -> Self {
        Self { kind, children }
    }

    /// Create a node with children.
    #[must_use]
    pub const fn with_children(kind: NodeKind, children: Vec<Self>) -> Self {
        Self { kind, children }
    }

    #[must_use]
    pub fn children(&self) -> &[Self] {
        &self.children
    }

    /// Count the number of nodes in a normalized tree.
    /// None sentinel nodes are not counted.
    pub fn count_nodes(node: &Self) -> usize {
        if node.is_none() {
            return 0;
        }
        1 + node.children.iter().map(Self::count_nodes).sum::<usize>()
    }

    /// Check if this is a None sentinel node.
    #[must_use]
    pub const fn is_none(&self) -> bool {
        matches!(self.kind, NodeKind::None)
    }

    #[must_use]
    pub const fn kind(&self) -> &NodeKind {
        &self.kind
    }
}

// -- Placeholder re-indexing --------------------------------------------------
