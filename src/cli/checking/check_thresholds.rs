// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

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
    pub max_exact_percent: Option<f64>,
    /// Largest share of lines allowed to be near duplicates.
    pub max_near_percent: Option<f64>,
}

impl CheckThresholds {
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
