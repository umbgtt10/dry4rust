// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::cli::suppression::baseline_filter::BaselineFilter;
use crate::cli::suppression::baseline_kind::BaselineKind;
use crate::cli::suppression::ignore_file::IgnoreFile;
use crate::code_unit::CodeUnit;
use crate::fingerprint::Fingerprint;
use crate::groups::duplicate_group::DuplicateGroup;
use crate::groups::duplication_stats::DuplicationStats;
use alloc::collections::BTreeSet;
use alloc::vec::Vec;

/// The groups an analysis found, by kind: exact and near, over whole units
/// and within them. Every stage after the finding -- the ignore file, the
/// baseline, the count -- treats the four kinds alike.
pub struct Findings {
    exact: Vec<DuplicateGroup>,
    near: Vec<DuplicateGroup>,
    sub_exact: Vec<DuplicateGroup>,
    sub_near: Vec<DuplicateGroup>,
}

impl Findings {
    #[must_use]
    pub const fn new(
        exact: Vec<DuplicateGroup>,
        near: Vec<DuplicateGroup>,
        sub_exact: Vec<DuplicateGroup>,
        sub_near: Vec<DuplicateGroup>,
    ) -> Self {
        Self {
            exact,
            near,
            sub_exact,
            sub_near,
        }
    }

    #[must_use]
    pub fn count(&self) -> usize {
        self.kinds().iter().map(|groups| groups.len()).sum()
    }

    #[must_use]
    pub fn exact(&self) -> &[DuplicateGroup] {
        &self.exact
    }

    #[must_use]
    pub fn fingerprints(&self) -> BTreeSet<Fingerprint> {
        self.kinds()
            .iter()
            .flat_map(|groups| groups.iter().map(DuplicateGroup::fingerprint))
            .collect()
    }

    #[must_use]
    pub fn near(&self) -> &[DuplicateGroup] {
        &self.near
    }

    #[must_use]
    pub fn retain_new(self, baseline: &BaselineFilter) -> Self {
        Self {
            exact: baseline.retain_new(BaselineKind::Exact, self.exact),
            near: baseline.retain_new(BaselineKind::Near, self.near),
            sub_exact: baseline.retain_new(BaselineKind::SubExact, self.sub_exact),
            sub_near: baseline.retain_new(BaselineKind::SubNear, self.sub_near),
        }
    }

    #[must_use]
    pub fn retain_unsuppressed(self, ignore_file: &IgnoreFile) -> Self {
        Self {
            exact: ignore_file.retain_unsuppressed(self.exact),
            near: ignore_file.retain_unsuppressed(self.near),
            sub_exact: ignore_file.retain_unsuppressed(self.sub_exact),
            sub_near: ignore_file.retain_unsuppressed(self.sub_near),
        }
    }

    #[must_use]
    pub fn stats(&self, units: &[CodeUnit]) -> DuplicationStats {
        DuplicateGroup::compute_stats_with_sub(
            units,
            &self.exact,
            &self.near,
            &self.sub_exact,
            &self.sub_near,
        )
    }

    #[must_use]
    pub fn sub_exact(&self) -> &[DuplicateGroup] {
        &self.sub_exact
    }

    #[must_use]
    pub fn sub_near(&self) -> &[DuplicateGroup] {
        &self.sub_near
    }

    const fn kinds(&self) -> [&Vec<DuplicateGroup>; 4] {
        [&self.exact, &self.near, &self.sub_exact, &self.sub_near]
    }
}
