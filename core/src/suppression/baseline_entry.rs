// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::grouper::DuplicateGroup;
use crate::suppression::baseline_kind::BaselineKind;

/// One duplicate group as the baseline recorded it.
///
/// `members` is part of the identity, not decoration. An exact group is keyed
/// by the fingerprint its members share, so a third copy of an already-recorded
/// function does not change the fingerprint. Without the count, adding that
/// copy would be inherited duplication; with it, the group has grown past what
/// was recorded and is reported.
///
/// `names` is decoration, and deliberately so: a baseline nobody can read is a
/// baseline nobody audits. Nothing matches on it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaselineEntry {
    /// Which set of groups this was recorded from.
    pub kind: BaselineKind,
    /// The group's fingerprint, as hex.
    pub fingerprint: String,
    /// How many members the group had when it was recorded.
    pub members: usize,
    /// What those members were called, for a reader.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
}

impl BaselineEntry {
    /// Record `group` as inherited duplication of `kind`.
    #[must_use]
    pub fn of(kind: BaselineKind, group: &DuplicateGroup) -> Self {
        Self {
            kind,
            fingerprint: group.fingerprint.to_hex(),
            members: group.members.len(),
            names: group.members.iter().map(|m| m.name.clone()).collect(),
        }
    }

    /// Whether this entry accounts for `group` in full.
    ///
    /// A group that has grown since it was recorded is not accounted for, so
    /// the new copy is reported rather than inherited.
    #[must_use]
    pub fn admits(&self, kind: BaselineKind, group: &DuplicateGroup) -> bool {
        self.kind == kind
            && self.fingerprint == group.fingerprint.to_hex()
            && group.members.len() <= self.members
    }
}
