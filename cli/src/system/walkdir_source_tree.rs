// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::ffi::OsStr;
use std::path::Path;

use dry4rust::scanner::Scanner;
use dry4rust::source_path::SourcePath;
use dry4rust::storage_failure::StorageFailure;
use dry4rust::traits::source_tree::SourceTree;
use walkdir::WalkDir;

use crate::system::disk_file::DiskFile;
use crate::system::portable_path::PortablePath;

// The source tree on disk, walked in the order the file system lists it. The
// scanner decides which directories are entered and which files are kept; a
// directory whose name is not UTF-8 is entered, as no rule names it. A file the
// scanner keeps whose path is not UTF-8 cannot become a `SourcePath`, so it
// fails the listing by name rather than being skipped unanalysed.
#[derive(Debug, Clone, Copy, Default)]
pub struct WalkdirSourceTree;

impl WalkdirSourceTree {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    fn enters(scanner: &Scanner, path: &Path) -> bool {
        if !path.is_dir() || path.file_name().and_then(OsStr::to_str).is_none() {
            return true;
        }
        scanner.descends_into(&PortablePath::lossy(path))
    }
}

impl SourceTree for WalkdirSourceTree {
    fn read_source(&self, file: &SourcePath) -> Result<String, StorageFailure> {
        DiskFile::new(file).read()
    }

    fn source_files(&self, scanner: &Scanner) -> Result<Vec<SourcePath>, StorageFailure> {
        let mut files = Vec::new();
        for entry in WalkDir::new(scanner.root().as_str())
            .into_iter()
            .filter_entry(|entry| Self::enters(scanner, entry.path()))
            .flatten()
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let file = PortablePath::of(path);
            let named = file.clone().unwrap_or_else(|| PortablePath::lossy(path));
            if !scanner.admits(&named) {
                continue;
            }
            match file {
                Some(file) => files.push(file),
                None => {
                    return Err(StorageFailure::Failed(format!(
                        "{named}: the path is not valid UTF-8"
                    )));
                }
            }
        }
        Ok(files)
    }
}
