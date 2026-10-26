// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::suppression::ignore_file::IgnoreFile;
use serde::{Deserialize, Serialize};

use crate::system::ignore_entry_record::IgnoreEntryRecord;

/// `.dry4rust-ignore.toml` as a document: one `[[ignore]]` table per entry, and
/// none at all read as an empty list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct IgnoreDocument {
    #[serde(default)]
    pub ignore: Vec<IgnoreEntryRecord>,
}

impl From<&IgnoreFile> for IgnoreDocument {
    fn from(file: &IgnoreFile) -> Self {
        Self {
            ignore: file.ignore().iter().map(IgnoreEntryRecord::from).collect(),
        }
    }
}

impl From<IgnoreDocument> for IgnoreFile {
    fn from(document: IgnoreDocument) -> Self {
        Self::new(document.ignore.into_iter().map(Into::into).collect())
    }
}
