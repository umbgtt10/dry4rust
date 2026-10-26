// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::cli::suppression::ignore_file::IgnoreFile;
use crate::source_path::SourcePath;
use crate::storage_failure::StorageFailure;

// Where the ignore file is kept, and the format it is kept in.
pub trait IgnoreStore {
    fn load(&self, file: &SourcePath) -> Result<IgnoreFile, StorageFailure>;

    fn remove(&self, file: &SourcePath) -> Result<(), StorageFailure>;

    fn save(&self, file: &SourcePath, ignore: &IgnoreFile) -> Result<(), StorageFailure>;
}
