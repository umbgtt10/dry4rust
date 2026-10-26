// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::suppression::ignore_file::IgnoreFile;
use dry4rust::source_path::SourcePath;
use dry4rust::storage_failure::StorageFailure;
use dry4rust::traits::ignore_store::IgnoreStore;
use toml::from_str;
use toml::to_string_pretty;

use crate::system::disk_file::DiskFile;
use crate::system::ignore_document::IgnoreDocument;

// The ignore file on disk, in TOML.
#[derive(Debug, Clone, Copy, Default)]
pub struct TomlIgnoreStore;

impl TomlIgnoreStore {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl IgnoreStore for TomlIgnoreStore {
    fn load(&self, file: &SourcePath) -> Result<IgnoreFile, StorageFailure> {
        DiskFile::new(file)
            .parsed(from_str::<IgnoreDocument>)
            .map(IgnoreFile::from)
    }

    fn remove(&self, file: &SourcePath) -> Result<(), StorageFailure> {
        DiskFile::new(file).remove()
    }

    fn save(&self, file: &SourcePath, ignore: &IgnoreFile) -> Result<(), StorageFailure> {
        let content = to_string_pretty(&IgnoreDocument::from(ignore)).map_err(|error| {
            StorageFailure::Failed(format!("Failed to serialize ignore file: {error}"))
        })?;
        DiskFile::new(file).write(&content)
    }
}
