// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;

use dry4rust::source_path::SourcePath;
use dry4rust::storage_failure::StorageFailure;

// One file on disk, read with what the platform says turned into the core's
// storage failure: a file that is not there is `Missing`, anything else
// `Failed`, each in the platform's words.
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

    pub fn read(&self) -> Result<String, StorageFailure> {
        fs::read_to_string(self.path).map_err(|error| Self::failure(&error))
    }

    fn failure(error: &Error) -> StorageFailure {
        if error.kind() == ErrorKind::NotFound {
            StorageFailure::Missing(error.to_string())
        } else {
            StorageFailure::Failed(error.to_string())
        }
    }
}
