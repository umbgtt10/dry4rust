// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::scanner::Scanner;
use dry4rust::source_path::SourcePath;
use dry4rust::storage_failure::StorageFailure;
use dry4rust::traits::source_tree::SourceTree;
use std::collections::BTreeMap;

// A source tree held in memory, walked in path order. `None` marks a file that
// is listed but cannot be read; an unlistable tree fails the walk itself, as a
// file the platform cannot name would. The walk asks the scanner exactly what
// the real one does: every directory from the root down, then the file.
#[derive(Debug, Clone, Default)]
pub struct InMemorySourceTree {
    files: BTreeMap<SourcePath, Option<String>>,
    unlistable: Option<String>,
}

impl InMemorySourceTree {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn unlistable(reason: &str) -> Self {
        Self {
            files: BTreeMap::new(),
            unlistable: Some(String::from(reason)),
        }
    }

    #[must_use]
    pub fn with_file(self, path: &str, text: &str) -> Self {
        let mut files = self.files;
        files.insert(
            SourcePath::new(String::from(path)),
            Some(String::from(text)),
        );
        Self { files, ..self }
    }

    #[must_use]
    pub fn with_unreadable_file(self, path: &str) -> Self {
        let mut files = self.files;
        files.insert(SourcePath::new(String::from(path)), None);
        Self { files, ..self }
    }

    fn reachable(scanner: &Scanner, file: &SourcePath) -> bool {
        let Some(below) = file.strip_prefix(scanner.root()) else {
            return false;
        };
        let mut directory = scanner.root().clone();
        if !scanner.descends_into(&directory) {
            return false;
        }
        let names: Vec<&str> = below.split('/').filter(|name| !name.is_empty()).collect();
        for name in &names[..names.len().saturating_sub(1)] {
            directory = directory.join(name);
            if !scanner.descends_into(&directory) {
                return false;
            }
        }
        scanner.admits(file)
    }
}

impl SourceTree for InMemorySourceTree {
    fn read_source(&self, file: &SourcePath) -> Result<String, StorageFailure> {
        match self.files.get(file) {
            Some(Some(text)) => Ok(text.clone()),
            Some(None) => Err(StorageFailure::Failed(String::from("permission denied"))),
            None => Err(StorageFailure::Missing(String::from("no such file"))),
        }
    }

    fn source_files(&self, scanner: &Scanner) -> Result<Vec<SourcePath>, StorageFailure> {
        if let Some(reason) = &self.unlistable {
            return Err(StorageFailure::Failed(reason.clone()));
        }
        Ok(self
            .files
            .keys()
            .filter(|file| Self::reachable(scanner, file))
            .cloned()
            .collect())
    }
}
