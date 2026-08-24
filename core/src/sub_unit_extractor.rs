// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::code_unit::CodeUnitKind;
use crate::extractor::SubUnit;
use crate::node::NodeKind;
use crate::node::NormalizedNode;
use crate::node::count_nodes;
use crate::node::reindex_placeholders;

/// Walks a normalized tree and collects the compound structures inside it.
///
/// Every node kind that carries a body contributes that body as a candidate
/// sub-unit, and the walk then descends into all children regardless. What
/// varies between kinds is only *which* child holds the body, so most of them
/// reduce to a lookup; `If` and `Match` are the two that do not, because one
/// has an optional second branch and the other has a variable number.
pub struct SubUnitExtractor {
    min_node_count: usize,
}

impl SubUnitExtractor {
    #[must_use]
    pub const fn new(min_node_count: usize) -> Self {
        Self { min_node_count }
    }

    /// Collect every sub-unit at or below `node` that meets the node floor.
    ///
    /// A node's own contributions come before its children's, and children
    /// are visited in order, so the result is a pre-order walk.
    #[must_use]
    pub fn extract(&self, node: &NormalizedNode) -> Vec<SubUnit> {
        let mut found = self.own_units(node);
        for child in &node.children {
            found.extend(self.extract(child));
        }
        found
    }

    fn own_units(&self, node: &NormalizedNode) -> Vec<SubUnit> {
        match &node.kind {
            NodeKind::If => self.if_units(node),
            NodeKind::Match => self.match_units(node),
            _ => self.bodied_unit(node),
        }
    }

    /// `If` is `[condition, then_branch, else_or_None]`. The else child is
    /// always present in the normalized form, so an absent else has to be
    /// told apart from a real one by asking the node rather than the slot.
    fn if_units(&self, node: &NormalizedNode) -> Vec<SubUnit> {
        let then_branch = node
            .children
            .get(1)
            .and_then(|n| self.made(n, CodeUnitKind::IfBranch, "if-then branch"));
        let else_branch = node
            .children
            .get(2)
            .filter(|n| !n.is_none())
            .and_then(|n| self.made(n, CodeUnitKind::IfBranch, "if-else branch"));
        then_branch.into_iter().chain(else_branch).collect()
    }

    /// `Match` is `[expr, arm0, arm1, ...]` and each arm is
    /// `[pattern, guard_or_None, body]`. Arms are numbered from one in the
    /// description because that is what a reader counting them would say.
    fn match_units(&self, node: &NormalizedNode) -> Vec<SubUnit> {
        node.children
            .iter()
            .skip(1)
            .enumerate()
            .filter_map(|(i, arm)| {
                let body = arm.children.get(2)?;
                self.made(
                    body,
                    CodeUnitKind::MatchArm,
                    &format!("match arm {}", i + 1),
                )
            })
            .collect()
    }

    fn bodied_unit(&self, node: &NormalizedNode) -> Vec<SubUnit> {
        let Some(target) = Self::body_target(&node.kind) else {
            return Vec::new();
        };
        let (index, kind, description) = target;
        node.children
            .get(index)
            .and_then(|body| self.made(body, kind, description))
            .into_iter()
            .collect()
    }

    /// Which child holds the body, what to call the unit, and how to describe
    /// it -- for every kind whose body sits at a fixed position.
    const fn body_target(kind: &NodeKind) -> Option<(usize, CodeUnitKind, &'static str)> {
        match kind {
            NodeKind::Loop => Some((0, CodeUnitKind::LoopBody, "loop body")),
            NodeKind::While => Some((1, CodeUnitKind::LoopBody, "while body")),
            NodeKind::ForLoop => Some((2, CodeUnitKind::LoopBody, "for body")),
            NodeKind::Closure => Some((0, CodeUnitKind::Block, "closure body")),
            _ => None,
        }
    }

    /// Re-index to canonical placeholder form and keep the result only if it
    /// is big enough to be worth comparing against anything else.
    fn made(
        &self,
        node: &NormalizedNode,
        kind: CodeUnitKind,
        description: &str,
    ) -> Option<SubUnit> {
        let reindexed = reindex_placeholders(node);
        let node_count = count_nodes(&reindexed);
        if node_count < self.min_node_count {
            return None;
        }
        Some(SubUnit {
            kind,
            node: reindexed,
            node_count,
            description: description.to_string(),
        })
    }
}
