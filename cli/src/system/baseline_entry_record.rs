// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::suppression::baseline_entry::BaselineEntry;
use dry4rust::cli::suppression::baseline_kind::BaselineKind;
use serde::{Deserialize, Serialize};

/// One baseline entry as the file spells it. `names` is decoration and is left
/// out when there are none.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BaselineEntryRecord {
    #[serde(with = "BaselineKindRecord")]
    pub kind: BaselineKind,
    pub fingerprint: String,
    pub members: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
}

impl From<&BaselineEntry> for BaselineEntryRecord {
    fn from(entry: &BaselineEntry) -> Self {
        Self {
            kind: entry.kind(),
            fingerprint: entry.fingerprint().to_owned(),
            members: entry.members(),
            names: entry.names().to_vec(),
        }
    }
}

impl From<BaselineEntryRecord> for BaselineEntry {
    fn from(record: BaselineEntryRecord) -> Self {
        Self::new(
            record.kind,
            record.fingerprint,
            record.members,
            record.names,
        )
    }
}

/// How the file spells a baseline entry's kind: by name, never by position, so
/// reordering the variants cannot repoint a recorded entry. A mirror of the
/// core's enum for serde, which a field names with
/// `#[serde(with = "BaselineKindRecord")]`.
#[derive(Serialize, Deserialize)]
#[serde(remote = "BaselineKind", rename_all = "snake_case")]
enum BaselineKindRecord {
    Exact,
    Near,
    SubExact,
    SubNear,
}
