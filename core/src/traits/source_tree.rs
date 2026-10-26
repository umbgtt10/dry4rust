// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::scanner::Scanner;
use crate::source_path::SourcePath;
use crate::storage_failure::StorageFailure;

// The source files under a root, and what each of them holds. The walk asks the
// scanner which directories to enter and which files to keep, so every rule
// about what is analysed lives in the core and only the reading lives outside.
pub trait SourceTree {
    fn read_source(&self, file: &SourcePath) -> Result<String, StorageFailure>;

    fn source_files(&self, scanner: &Scanner) -> Result<Vec<SourcePath>, StorageFailure>;
}
