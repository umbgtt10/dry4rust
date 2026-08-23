// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::error::Result;
use crate::threshold::Threshold;

/// The four ceilings `check` measures a result against.
///
/// Every field is optional and an absent one is not a ceiling of zero -- it
/// means the caller did not ask about that dimension, so nothing can breach
/// it. `Default` therefore describes a `check` that reports and never fails,
/// which is what running `check` with no flags does.
#[derive(Debug, Clone, Default)]
pub struct CheckThresholds {
    /// Most exact-duplicate groups allowed.
    pub max_exact: Option<usize>,
    /// Most near-duplicate groups allowed.
    pub max_near: Option<usize>,
    /// Largest share of lines allowed to be exact duplicates.
    pub max_exact_percent: Option<Threshold>,
    /// Largest share of lines allowed to be near duplicates.
    pub max_near_percent: Option<Threshold>,
}

impl CheckThresholds {
    /// Build the ceilings from what the command line asked for.
    ///
    /// # Errors
    ///
    /// Returns [`crate::error::Error::InvalidConfig`] naming the flag whose
    /// value is not a share of a hundred.
    pub fn new(
        max_exact: Option<usize>,
        max_near: Option<usize>,
        max_exact_percent: Option<f64>,
        max_near_percent: Option<f64>,
    ) -> Result<Self> {
        Ok(Self {
            max_exact,
            max_near,
            max_exact_percent: max_exact_percent
                .map(|value| Threshold::percent("--max-exact-percent", value))
                .transpose()?,
            max_near_percent: max_near_percent
                .map(|value| Threshold::percent("--max-near-percent", value))
                .transpose()?,
        })
    }

    /// Whether any ceiling was set at all.
    ///
    /// A `check` with none set can still report, and still exits zero however
    /// much duplication it finds.
    #[must_use]
    pub const fn is_unbounded(&self) -> bool {
        self.max_exact.is_none()
            && self.max_near.is_none()
            && self.max_exact_percent.is_none()
            && self.max_near_percent.is_none()
    }
}
