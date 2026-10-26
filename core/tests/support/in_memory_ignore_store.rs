// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::suppression::ignore_file::IgnoreFile;
use dry4rust::source_path::SourcePath;
use dry4rust::storage_failure::StorageFailure;
use dry4rust::traits::ignore_store::IgnoreStore;
use std::cell::RefCell;
use std::collections::BTreeMap;

// Ignore files held in memory by path. A failing store refuses every write and
// removal with the reason it was given; an unreadable one every read.
#[derive(Debug, Default)]
pub struct InMemoryIgnoreStore {
    files: RefCell<BTreeMap<SourcePath, IgnoreFile>>,
    failing: Option<String>,
    unreadable: Option<String>,
}

impl InMemoryIgnoreStore {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn failing(reason: &str) -> Self {
        Self {
            failing: Some(String::from(reason)),
            ..Self::default()
        }
    }

    #[must_use]
    pub fn unreadable(reason: &str) -> Self {
        Self {
            unreadable: Some(String::from(reason)),
            ..Self::default()
        }
    }

    pub fn held(&self, path: &SourcePath) -> Option<IgnoreFile> {
        self.files.borrow().get(path).cloned()
    }

    pub fn holds(&self, path: &SourcePath) -> bool {
        self.files.borrow().contains_key(path)
    }
}

impl IgnoreStore for InMemoryIgnoreStore {
    fn load(&self, file: &SourcePath) -> Result<IgnoreFile, StorageFailure> {
        if let Some(reason) = &self.unreadable {
            return Err(StorageFailure::Failed(reason.clone()));
        }
        self.held(file)
            .ok_or_else(|| StorageFailure::Missing(String::from("no such file")))
    }

    fn remove(&self, file: &SourcePath) -> Result<(), StorageFailure> {
        if let Some(reason) = &self.failing {
            return Err(StorageFailure::Failed(reason.clone()));
        }
        self.files
            .borrow_mut()
            .remove(file)
            .map(|_| ())
            .ok_or_else(|| StorageFailure::Missing(String::from("no such file")))
    }

    fn save(&self, file: &SourcePath, ignore: &IgnoreFile) -> Result<(), StorageFailure> {
        if let Some(reason) = &self.failing {
            return Err(StorageFailure::Failed(reason.clone()));
        }
        self.files.borrow_mut().insert(file.clone(), ignore.clone());
        Ok(())
    }
}
