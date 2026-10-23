// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::duplicate_group::DuplicateGroup;
use crate::output::duplication_stats::DuplicationStats;

/// Everything a full report shows, handed to a reporter in one piece.
///
/// It exists because the shape of the document is the format's business and
/// not the command's. Text can be written a section at a time and read
/// correctly; JSON cannot, and a caller that emitted one section per call
/// produced several top-level documents rather than one.
pub struct Report<'a> {
    stats: &'a DuplicationStats,
    exact: &'a [DuplicateGroup],
    near: &'a [DuplicateGroup],
    sub_exact: &'a [DuplicateGroup],
    sub_near: &'a [DuplicateGroup],
}

impl<'a> Report<'a> {
    #[must_use]
    pub const fn new(
        stats: &'a DuplicationStats,
        exact: &'a [DuplicateGroup],
        near: &'a [DuplicateGroup],
        sub_exact: &'a [DuplicateGroup],
        sub_near: &'a [DuplicateGroup],
    ) -> Self {
        Self {
            stats,
            exact,
            near,
            sub_exact,
            sub_near,
        }
    }

    #[must_use]
    pub const fn exact(&self) -> &'a [DuplicateGroup] {
        self.exact
    }

    #[must_use]
    pub const fn near(&self) -> &'a [DuplicateGroup] {
        self.near
    }

    #[must_use]
    pub const fn stats(&self) -> &'a DuplicationStats {
        self.stats
    }

    #[must_use]
    pub const fn sub_exact(&self) -> &'a [DuplicateGroup] {
        self.sub_exact
    }

    #[must_use]
    pub const fn sub_near(&self) -> &'a [DuplicateGroup] {
        self.sub_near
    }
}
