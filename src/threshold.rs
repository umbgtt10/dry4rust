// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::error::Error;
use crate::error::Result;

/// A proportion between none and all, held as a fraction.
///
/// Every threshold this tool takes is one of these: a similarity score to
/// clear, or a share of duplicated lines to stay under. They differ only in
/// how they are written -- `0.85` in one place, `85.0` in another -- so they
/// are one type with two constructors rather than two types.
///
/// The invariant is the point. A similarity threshold of `5` makes the size
/// pre-filter reject every pair, and the report then says "no near
/// duplicates" -- a silence indistinguishable from a clean codebase. Nothing
/// downstream can tell that apart, so the value never gets that far.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Threshold(f64);

impl Threshold {
    /// The similarity threshold used when nothing sets one.
    pub const DEFAULT_SIMILARITY: Self = Self(0.9);

    /// A threshold written as a fraction, as `similarity_threshold` is.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidConfig`] naming `field` when `value` is
    /// outside `0.0..=1.0`, which `NaN` also is.
    pub fn fraction(field: &'static str, value: f64) -> Result<Self> {
        if (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(Error::InvalidConfig {
                field,
                value: value.to_string(),
                expected: "a fraction between 0.0 and 1.0",
            })
        }
    }

    /// A threshold written as a percentage, as the `check` ceilings are.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidConfig`] naming `field` when `value` is
    /// outside `0.0..=100.0`, which `NaN` also is.
    pub fn percent(field: &'static str, value: f64) -> Result<Self> {
        if (0.0..=100.0).contains(&value) {
            Ok(Self(value / 100.0))
        } else {
            Err(Error::InvalidConfig {
                field,
                value: value.to_string(),
                expected: "a percentage between 0.0 and 100.0",
            })
        }
    }

    /// The threshold as a fraction of one.
    #[must_use]
    pub const fn as_fraction(self) -> f64 {
        self.0
    }

    /// The threshold as a share of a hundred.
    #[must_use]
    pub fn as_percent(self) -> f64 {
        self.0 * 100.0
    }
}
