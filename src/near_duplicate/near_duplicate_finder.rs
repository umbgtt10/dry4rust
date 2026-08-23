// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::collections::HashSet;

use crate::code_unit::CodeUnit;
use crate::fingerprint::Fingerprint;
use crate::grouper::DuplicateGroup;
use crate::near_duplicate::pair_scanner::PairScanner;
use crate::near_duplicate::similarity_pair::SimilarityPair;
use crate::near_duplicate::union_find::UnionFind;
use std::cmp::Ordering;

/// Finds groups of units that are similar without being identical.
///
/// The work is four steps, and each is a method here: drop units already
/// accounted for as exact duplicates, bucket what remains so that only
/// plausibly-similar units are ever compared, score the pairs inside each
/// bucket, and close those pairs transitively into groups.
pub struct NearDuplicateFinder {
    threshold: f64,
}

type Scores = HashMap<(usize, usize), f64>;

impl NearDuplicateFinder {
    #[must_use]
    pub const fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    /// Group `units` by similarity, ignoring any whose fingerprint appears in
    /// `exact_fingerprints`.
    #[must_use]
    pub fn find(
        &self,
        units: &[CodeUnit],
        exact_fingerprints: &[Fingerprint],
    ) -> Vec<DuplicateGroup> {
        let candidates = Self::candidates(units, exact_fingerprints);
        if candidates.len() < 2 {
            return Vec::new();
        }
        let pairs = PairScanner::new(self.threshold).scan(&candidates);
        let groups = self.build_groups(&candidates, &pairs);
        Self::ranked(groups)
    }

    /// Units not already reported as exact duplicates.
    fn candidates<'a>(
        units: &'a [CodeUnit],
        exact_fingerprints: &[Fingerprint],
    ) -> Vec<&'a CodeUnit> {
        let exact: HashSet<Fingerprint> = exact_fingerprints.iter().copied().collect();
        units
            .iter()
            .filter(|unit| !exact.contains(&unit.fingerprint))
            .collect()
    }

    fn build_groups(
        &self,
        candidates: &[&CodeUnit],
        pairs: &[SimilarityPair],
    ) -> Vec<DuplicateGroup> {
        let mut forest = UnionFind::new(candidates.len());
        let mut scores: Scores = HashMap::new();
        for pair in pairs {
            forest.union(pair.left, pair.right);
            scores.insert(pair.key(), pair.score);
        }
        forest
            .groups()
            .into_iter()
            .filter(|members| members.len() > 1)
            .map(|members| self.group_from(candidates, &members, &scores))
            .collect()
    }

    /// A group is only as similar as its least similar pair, so that is the
    /// score it reports.
    fn group_from(
        &self,
        candidates: &[&CodeUnit],
        member_indices: &[usize],
        scores: &Scores,
    ) -> DuplicateGroup {
        let members: Vec<CodeUnit> = member_indices
            .iter()
            .map(|&i| candidates[i].clone())
            .collect();
        let member_fingerprints: Vec<Fingerprint> = members.iter().map(|m| m.fingerprint).collect();
        DuplicateGroup {
            fingerprint: Fingerprint::from_fingerprints(&member_fingerprints),
            members,
            similarity: self.weakest_link(member_indices, scores),
        }
    }

    /// The lowest recorded score between any two members.
    ///
    /// Transitive closure means a group can hold members that were never
    /// compared to each other -- A to B and B to C puts A and C together
    /// without an A-C score. When no pair was recorded at all the threshold
    /// stands in, since every pair that built the group cleared it.
    fn weakest_link(&self, member_indices: &[usize], scores: &Scores) -> f64 {
        let lowest = member_indices
            .iter()
            .flat_map(|&i| member_indices.iter().map(move |&j| (i, j)))
            .filter(|(i, j)| i < j)
            .filter_map(|key| scores.get(&key))
            .copied()
            .fold(f64::INFINITY, f64::min);
        if lowest.is_infinite() {
            self.threshold
        } else {
            lowest
        }
    }

    /// Biggest groups first, then most similar, then by fingerprint so that
    /// ties resolve the same way on every run.
    fn ranked(mut groups: Vec<DuplicateGroup>) -> Vec<DuplicateGroup> {
        groups.sort_by(|a, b| {
            b.members
                .len()
                .cmp(&a.members.len())
                .then_with(|| {
                    b.similarity
                        .partial_cmp(&a.similarity)
                        .unwrap_or(Ordering::Equal)
                })
                .then_with(|| a.fingerprint.cmp(&b.fingerprint))
        });
        groups
    }
}
