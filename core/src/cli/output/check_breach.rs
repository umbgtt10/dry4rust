// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::duplicate_group::DuplicateGroup;

/// One ceiling that was exceeded, and the groups that exceeded it.
///
/// The groups travel with the breach because text reports them under it,
/// immediately after the sentence that named it. JSON gathers them instead,
/// so two exact ceilings breached at once list the exact groups once rather
/// than twice.
pub struct CheckBreach<'a> {
    /// What to say about it: the count, the subject and the limit.
    pub message: String,
    /// The groups behind it.
    pub groups: &'a [DuplicateGroup],
    /// Whether those groups are exact duplicates rather than near ones.
    pub of_exact: bool,
}
