// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// One duplicate somebody decided should stay.
///
/// The counterpart of [`crate::suppression::baseline_entry::BaselineEntry`],
/// and the difference is who writes it. This one is written by a person and
/// carries a reason in their words; a baseline entry is written by the tool
/// and read back by it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IgnoreEntry {
    /// The fingerprint of the duplicated code.
    fingerprint: String,
    /// Optional reason for ignoring.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
    /// Names of the code units in the group (for documentation).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    members: Vec<String>,
}

impl IgnoreEntry {
    #[must_use]
    pub const fn new(fingerprint: String, reason: Option<String>, members: Vec<String>) -> Self {
        Self {
            fingerprint,
            reason,
            members,
        }
    }

    #[must_use]
    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }

    #[must_use]
    pub fn members(&self) -> &[String] {
        &self.members
    }

    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
}
