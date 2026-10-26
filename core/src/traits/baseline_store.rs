// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::cli::suppression::baseline_file::BaselineFile;
use crate::source_path::SourcePath;
use crate::storage_failure::StorageFailure;

// Where a baseline is kept, and the format it is kept in. Whether what was read
// is a format this build judges against is the core's question, not the store's.
pub trait BaselineStore {
    fn load(&self, file: &SourcePath) -> Result<BaselineFile, StorageFailure>;

    fn save(&self, file: &SourcePath, baseline: &BaselineFile) -> Result<(), StorageFailure>;
}
