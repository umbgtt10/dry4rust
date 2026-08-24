// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::grouper::DuplicateGroup;

/// One ceiling that was exceeded, and the groups that exceeded it.
///
/// The groups travel with the breach because text reports them under it,
/// immediately after the sentence that named it. JSON gathers them instead,
/// so two exact ceilings breached at once list the exact groups once rather
/// than twice.
pub struct CheckBreach<'a> {
    message: String,
    groups: &'a [DuplicateGroup],
    of_exact: bool,
}

impl<'a> CheckBreach<'a> {
    #[must_use]
    pub const fn new(message: String, groups: &'a [DuplicateGroup], of_exact: bool) -> Self {
        Self {
            message,
            groups,
            of_exact,
        }
    }

    /// What to say about it: the count, the subject and the limit.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// The groups behind it.
    #[must_use]
    pub const fn groups(&self) -> &'a [DuplicateGroup] {
        self.groups
    }

    /// Whether those groups are exact duplicates rather than near ones.
    #[must_use]
    pub const fn is_of_exact(&self) -> bool {
        self.of_exact
    }
}
