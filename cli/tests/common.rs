// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::analysis::Analysis;
use dry4rust::duplicate_group::DuplicateGroup;
use std::collections::HashSet;

/// An analysis result holding exactly these groups, for tests about what is
/// done with groups rather than about how they were found.
#[must_use]
pub fn result_with(
    exact: Vec<DuplicateGroup>,
    near: Vec<DuplicateGroup>,
    sub_exact: Vec<DuplicateGroup>,
    sub_near: Vec<DuplicateGroup>,
) -> Analysis {
    Analysis::new(
        DuplicateGroup::compute_stats_with_sub(&[], &exact, &near, &sub_exact, &sub_near),
        exact,
        near,
        sub_exact,
        sub_near,
        Vec::new(),
        HashSet::new(),
    )
}
