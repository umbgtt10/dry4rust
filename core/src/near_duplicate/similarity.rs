// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::node;
use crate::node::{NodeKind, NormalizedNode};
use std::mem;

/// Compute a similarity score between two normalized trees using the Dice coefficient.
/// Returns a value between 0.0 (completely different) and 1.0 (identical).
///
/// score = (2 * matching_nodes) / (nodes_a + nodes_b)
///
/// How children are compared depends on what they are. A `Block` holds a list
/// of statements, so its children are aligned: inserting one statement shifts
/// the rest rather than misaligning them. Every other kind holds children in
/// named slots -- an `If` is `[condition, then, else]` -- so those are
/// compared position for position, and a then-branch is never matched against
/// an else-branch.
#[must_use]
pub fn similarity_score(a: &NormalizedNode, b: &NormalizedNode) -> f64 {
    let nodes_a = node::count_nodes(a);
    let nodes_b = node::count_nodes(b);
    if nodes_a == 0 && nodes_b == 0 {
        return 1.0;
    }
    let matching = count_matching(a, b);
    (2.0 * matching as f64) / (nodes_a + nodes_b) as f64
}

/// Count matching nodes between two trees by traversing them together.
fn count_matching(a: &NormalizedNode, b: &NormalizedNode) -> usize {
    if a.is_none() || b.is_none() {
        return 0;
    }
    if mem::discriminant(&a.kind) != mem::discriminant(&b.kind) {
        return 0;
    }
    // MacroCall: different names = no match (no recursion into children)
    if let (NodeKind::MacroCall { name: na }, NodeKind::MacroCall { name: nb }) = (&a.kind, &b.kind)
        && na != nb
    {
        return 0;
    }
    let self_match = usize::from(a.kind == b.kind);
    let children = if is_sequence(&a.kind) {
        aligned_children(&a.children, &b.children)
    } else {
        slotted_children(&a.children, &b.children)
    };
    self_match + children
}

/// Whether a kind's children are a homogeneous list rather than named slots.
///
/// Only these three are built by mapping over a sequence with nothing in
/// front of it. `Call` is `[callee, arg0, ...]` and `Match` is
/// `[scrutinee, arm0, ...]`, so both carry a header child and are excluded --
/// aligning them freely would let a callee match an argument.
const fn is_sequence(kind: &NodeKind) -> bool {
    matches!(kind, NodeKind::Block | NodeKind::Tuple | NodeKind::Array)
}

/// Children in named slots: the nth of one is only ever compared with the nth
/// of the other.
fn slotted_children(a: &[NormalizedNode], b: &[NormalizedNode]) -> usize {
    a.iter()
        .zip(b)
        .map(|(child_a, child_b)| count_matching(child_a, child_b))
        .sum()
}

/// Children as a list: the best total over any order-preserving pairing.
///
/// This is a longest-common-subsequence weighted by how well each candidate
/// pair matches, rather than by whether they are equal. Two blocks differing
/// by one inserted statement therefore score as one statement apart, where a
/// positional comparison would have counted everything after the insertion as
/// a mismatch.
///
/// Each child of `a` is paired with at most one child of `b`, so the result
/// stays bounded by the smaller list -- which is what lets `PairScanner`
/// treat `2 * min / (min + max)` as a ceiling.
fn aligned_children(a: &[NormalizedNode], b: &[NormalizedNode]) -> usize {
    if a.is_empty() || b.is_empty() {
        return 0;
    }
    let mut best = vec![vec![0usize; b.len() + 1]; a.len() + 1];
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let paired = best[i - 1][j - 1] + count_matching(&a[i - 1], &b[j - 1]);
            best[i][j] = paired.max(best[i - 1][j]).max(best[i][j - 1]);
        }
    }
    best[a.len()][b.len()]
}
