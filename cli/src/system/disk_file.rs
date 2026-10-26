// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fmt::Display;
use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;

use dry4rust::source_path::SourcePath;
use dry4rust::storage_failure::StorageFailure;

// One file on disk, read, written and removed with what the platform says
// turned into the core's storage failure: a file that is not there is
// `Missing`, anything else `Failed`, each in the platform's words. A document
// that does not parse is `Failed` in the parser's.
pub struct DiskFile<'a> {
    path: &'a Path,
}

impl<'a> DiskFile<'a> {
    #[must_use]
    pub fn new(file: &'a SourcePath) -> Self {
        Self {
            path: Path::new(file.as_str()),
        }
    }

    pub fn create_parent(&self) -> Result<(), StorageFailure> {
        match self.path.parent() {
            Some(parent) if !parent.as_os_str().is_empty() => {
                fs::create_dir_all(parent).map_err(|error| Self::failure(&error))
            }
            _ => Ok(()),
        }
    }

    pub fn parsed<T, E: Display>(
        &self,
        parse: impl FnOnce(&str) -> Result<T, E>,
    ) -> Result<T, StorageFailure> {
        let content = self.read()?;
        parse(&content).map_err(|error| StorageFailure::Failed(error.to_string()))
    }

    pub fn read(&self) -> Result<String, StorageFailure> {
        fs::read_to_string(self.path).map_err(|error| Self::failure(&error))
    }

    pub fn remove(&self) -> Result<(), StorageFailure> {
        fs::remove_file(self.path).map_err(|error| Self::failure(&error))
    }

    pub fn write(&self, content: &str) -> Result<(), StorageFailure> {
        fs::write(self.path, content).map_err(|error| Self::failure(&error))
    }

    fn failure(error: &Error) -> StorageFailure {
        if error.kind() == ErrorKind::NotFound {
            StorageFailure::Missing(error.to_string())
        } else {
            StorageFailure::Failed(error.to_string())
        }
    }
}
