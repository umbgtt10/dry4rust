// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::suppression::baseline_file::BaselineFile;
use serde::{Deserialize, Serialize};

use crate::system::baseline_entry_record::BaselineEntryRecord;

/// `dry4rust-baseline.json` as a document: the format version, then every
/// entry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BaselineDocument {
    pub version: u32,
    pub entries: Vec<BaselineEntryRecord>,
}

impl From<&BaselineFile> for BaselineDocument {
    fn from(file: &BaselineFile) -> Self {
        Self {
            version: file.version(),
            entries: file
                .entries()
                .iter()
                .map(BaselineEntryRecord::from)
                .collect(),
        }
    }
}

impl From<BaselineDocument> for BaselineFile {
    fn from(document: BaselineDocument) -> Self {
        Self::new(
            document.version,
            document.entries.into_iter().map(Into::into).collect(),
        )
    }
}
