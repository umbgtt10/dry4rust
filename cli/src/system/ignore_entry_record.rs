// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::suppression::ignore_entry::IgnoreEntry;
use serde::{Deserialize, Serialize};

/// One entry of `.dry4rust-ignore.toml` as the file spells it: the two optional
/// fields are left out when there is nothing to say, so a hand-maintained file
/// gains no keys nobody wrote.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IgnoreEntryRecord {
    pub fingerprint: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<String>,
}

impl From<&IgnoreEntry> for IgnoreEntryRecord {
    fn from(entry: &IgnoreEntry) -> Self {
        Self {
            fingerprint: entry.fingerprint.clone(),
            reason: entry.reason.clone(),
            members: entry.members.clone(),
        }
    }
}

impl From<IgnoreEntryRecord> for IgnoreEntry {
    fn from(record: IgnoreEntryRecord) -> Self {
        Self {
            fingerprint: record.fingerprint,
            reason: record.reason,
            members: record.members,
        }
    }
}
