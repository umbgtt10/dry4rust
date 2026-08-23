// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::code_unit::CodeUnit;
use crate::near_duplicate::similarity::similarity_score;
use crate::near_duplicate::similarity_pair::SimilarityPair;

/// Decides which pairs of candidates are worth scoring, and scores them.
///
/// Comparing every unit with every other is quadratic, so a pair is only
/// scored when its sizes leave it able to clear the threshold. That test is
/// exact rather than approximate, which is the whole point of this type:
/// every pair it discards provably cannot reach the threshold, and every pair
/// that could is kept.
pub struct PairScanner {
    threshold: f64,
}

impl PairScanner {
    #[must_use]
    pub const fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    /// Every pair among `candidates` that scores at or above the threshold.
    ///
    /// Kind does not restrict what is compared. `group_exact_duplicates` has
    /// always ignored it -- a free function and a method with identical
    /// normalised bodies are reported as exact duplicates -- and there is no
    /// reading under which the same pair differing by one statement should be
    /// invisible instead.
    #[must_use]
    pub fn scan(&self, candidates: &[&CodeUnit]) -> Vec<SimilarityPair> {
        let ordered = Self::by_size(candidates);
        let mut pairs = Vec::new();
        for (offset, &left) in ordered.iter().enumerate() {
            for &right in ordered.iter().skip(offset + 1) {
                if !self.could_reach_threshold(
                    candidates[left].node_count,
                    candidates[right].node_count,
                ) {
                    break;
                }
                let score = similarity_score(&candidates[left].body, &candidates[right].body);
                if score >= self.threshold {
                    pairs.push(SimilarityPair::new(left, right, score));
                }
            }
        }
        pairs
    }

    /// The highest score a pair could reach given nothing but their sizes.
    ///
    /// `matching` can never exceed the smaller tree's node count, so the Dice
    /// score is bounded by `2 * smaller / (smaller + larger)`. A pair whose
    /// ceiling falls below the threshold cannot clear it however similar its
    /// contents are, and scoring it is wasted work.
    ///
    /// The comparison leans towards keeping. A pair whose ceiling lands
    /// exactly on the threshold can still score exactly the threshold, and
    /// `similarity_score` admits it -- so an epsilon of slack keeps rounding
    /// from discarding a pair the scorer would have accepted. Comparing one
    /// pair too many costs a score; comparing one too few loses a finding,
    /// and says nothing about having done so.
    #[must_use]
    pub fn could_reach_threshold(&self, smaller: usize, larger: usize) -> bool {
        let total = smaller + larger;
        if total == 0 {
            return true;
        }
        let ceiling = 2.0 * smaller as f64 / total as f64;
        ceiling >= self.threshold - f64::EPSILON
    }

    /// Candidate indices ordered by node count ascending.
    ///
    /// The order is what makes the size bound cheap: `could_reach_threshold`
    /// falls monotonically as the partner grows, so the first partner that
    /// fails ends the scan for that unit rather than merely skipping one
    /// comparison.
    fn by_size(candidates: &[&CodeUnit]) -> Vec<usize> {
        let mut ordered: Vec<usize> = (0..candidates.len()).collect();
        ordered.sort_by_key(|&index| candidates[index].node_count);
        ordered
    }
}
