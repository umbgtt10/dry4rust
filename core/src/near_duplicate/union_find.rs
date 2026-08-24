// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

/// Disjoint-set forest over `0..size`, with path compression.
///
/// Near-duplicate detection produces pairs -- "these two are similar" -- and
/// needs groups. That is a transitive closure, which is what this computes.
pub struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    /// Start with `size` singletons, each its own root.
    #[must_use]
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
        }
    }

    /// How many elements the forest covers.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.parent.len()
    }

    /// Whether the forest covers nothing.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }

    /// The root of `i`'s set, compressing the path walked to reach it.
    pub fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            let root = self.find(self.parent[i]);
            self.parent[i] = root;
        }
        self.parent[i]
    }

    /// Merge the sets holding `i` and `j`.
    pub fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            self.parent[root_i] = root_j;
        }
    }

    /// Every set, as a list of its members.
    ///
    /// Groups come out ordered by their lowest member and members come out
    /// ascending, so the same input always yields the same output -- which
    /// a `HashMap` keyed by root would not have given.
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut position_of_root: HashMap<usize, usize> = HashMap::new();
        let mut groups: Vec<Vec<usize>> = Vec::new();
        for i in 0..self.len() {
            let root = self.find(i);
            if let Some(&position) = position_of_root.get(&root) {
                groups[position].push(i);
            } else {
                position_of_root.insert(root, groups.len());
                groups.push(vec![i]);
            }
        }
        groups
    }
}
