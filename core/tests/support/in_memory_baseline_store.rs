// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::suppression::baseline_file::BaselineFile;
use dry4rust::source_path::SourcePath;
use dry4rust::storage_failure::StorageFailure;
use dry4rust::traits::baseline_store::BaselineStore;
use std::cell::RefCell;
use std::collections::BTreeMap;

// Baselines held in memory by path. A path marked unreadable fails its read
// with the reason given; a failing store refuses every write.
#[derive(Debug, Default)]
pub struct InMemoryBaselineStore {
    files: RefCell<BTreeMap<SourcePath, BaselineFile>>,
    unreadable: BTreeMap<SourcePath, String>,
    failing: Option<String>,
}

impl InMemoryBaselineStore {
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

    pub fn held(&self, path: &SourcePath) -> Option<BaselineFile> {
        self.files.borrow().get(path).cloned()
    }

    pub fn holds(&self, path: &SourcePath) -> bool {
        self.files.borrow().contains_key(path)
    }

    #[must_use]
    pub fn with_baseline(self, path: &str, baseline: BaselineFile) -> Self {
        self.files
            .borrow_mut()
            .insert(SourcePath::new(String::from(path)), baseline);
        self
    }

    #[must_use]
    pub fn with_unreadable(self, path: &str, reason: &str) -> Self {
        let mut unreadable = self.unreadable;
        unreadable.insert(SourcePath::new(String::from(path)), String::from(reason));
        Self { unreadable, ..self }
    }
}

impl BaselineStore for InMemoryBaselineStore {
    fn load(&self, file: &SourcePath) -> Result<BaselineFile, StorageFailure> {
        if let Some(reason) = self.unreadable.get(file) {
            return Err(StorageFailure::Failed(reason.clone()));
        }
        self.held(file)
            .ok_or_else(|| StorageFailure::Missing(String::from("no such file")))
    }

    fn save(&self, file: &SourcePath, baseline: &BaselineFile) -> Result<(), StorageFailure> {
        if let Some(reason) = &self.failing {
            return Err(StorageFailure::Failed(reason.clone()));
        }
        self.files
            .borrow_mut()
            .insert(file.clone(), baseline.clone());
        Ok(())
    }
}
