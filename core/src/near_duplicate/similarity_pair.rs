// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::near_duplicate::pair_key::PairKey;

/// Two candidate units found similar enough to be worth grouping, and how
/// similar they were.
///
/// The indices address the candidate list the pair was drawn from; the pair
/// carries no meaning without it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimilarityPair {
    pub left: usize,
    pub right: usize,
    pub score: f64,
}

impl SimilarityPair {
    #[must_use]
    pub const fn new(left: usize, right: usize, score: f64) -> Self {
        Self { left, right, score }
    }

    /// The pair's indices in ascending order, as a [`PairKey`].
    #[must_use]
    pub const fn key(&self) -> PairKey {
        PairKey::new(self.left, self.right)
    }
}
