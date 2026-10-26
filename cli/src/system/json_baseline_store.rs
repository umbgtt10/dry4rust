// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::suppression::baseline_file::BaselineFile;
use dry4rust::source_path::SourcePath;
use dry4rust::storage_failure::StorageFailure;
use dry4rust::traits::baseline_store::BaselineStore;
use serde_json::from_str;
use serde_json::to_string_pretty;

use crate::system::baseline_document::BaselineDocument;
use crate::system::disk_file::DiskFile;

// A baseline on disk, in JSON, ending in a newline. Saving creates the
// directory above it.
#[derive(Debug, Clone, Copy, Default)]
pub struct JsonBaselineStore;

impl JsonBaselineStore {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl BaselineStore for JsonBaselineStore {
    fn load(&self, file: &SourcePath) -> Result<BaselineFile, StorageFailure> {
        DiskFile::new(file)
            .parsed(|text| from_str::<BaselineDocument>(text))
            .map(BaselineFile::from)
    }

    fn save(&self, file: &SourcePath, baseline: &BaselineFile) -> Result<(), StorageFailure> {
        let disk = DiskFile::new(file);
        disk.create_parent()?;
        let json = to_string_pretty(&BaselineDocument::from(baseline))
            .map_err(|error| StorageFailure::Failed(error.to_string()))?;
        disk.write(&format!("{json}\n"))
    }
}
