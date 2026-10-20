// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// A pair's indices in ascending order: the key a similarity is stored and
/// looked up by.
///
/// Similarity is symmetric, so a lookup keyed on the raw pair would miss half
/// the time. Ordering the key removes the question, and naming its ends says
/// which index is which.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PairKey {
    pub low: usize,
    pub high: usize,
}

impl PairKey {
    #[must_use]
    pub const fn new(first: usize, second: usize) -> Self {
        if first < second {
            Self {
                low: first,
                high: second,
            }
        } else {
            Self {
                low: second,
                high: first,
            }
        }
    }
}
