// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::source_path::SourcePath;
use crate::storage_failure::StorageFailure;
use crate::traits::source_tree::SourceTree;

/// Decides which source files under a root are analysed.
///
/// The walk itself belongs to a [`SourceTree`]; what it enters and what it
/// keeps is decided here: never a `target/` directory, never a hidden one below
/// the root, and only files with one of the extensions that no exclusion
/// pattern matches.
pub struct Scanner {
    /// Root directory to scan.
    root: SourcePath,
    /// Glob patterns to exclude (simple substring matching for now).
    exclude_patterns: Vec<String>,
    /// File extensions to include (without the leading dot). Defaults to `["rs"]`.
    extensions: Vec<String>,
}

impl Scanner {
    #[must_use]
    pub fn new(root: SourcePath) -> Self {
        Self {
            root,
            exclude_patterns: Vec::new(),
            extensions: vec!["rs".to_string()],
        }
    }

    /// Whether a file is analysed: one of the extensions, and no exclusion
    /// pattern in its path.
    #[must_use]
    pub fn admits(&self, file: &SourcePath) -> bool {
        file.extension()
            .is_some_and(|ext| self.extensions.iter().any(|e| e.eq_ignore_ascii_case(ext)))
            && !Self::is_excluded(file, &self.exclude_patterns)
    }

    /// Whether the walk enters a directory. `target/` never; a hidden
    /// directory only when it is the root itself.
    #[must_use]
    pub fn descends_into(&self, directory: &SourcePath) -> bool {
        let Some(name) = directory.file_name() else {
            return true;
        };
        if name == "target" {
            return false;
        }
        !(name.starts_with('.') && *directory != self.root)
    }

    #[must_use]
    pub fn extensions(&self) -> &[String] {
        &self.extensions
    }

    /// Check if a path should be excluded based on exclusion patterns.
    #[must_use]
    pub fn is_excluded(path: &SourcePath, patterns: &[String]) -> bool {
        let path_str = path.as_str();
        patterns
            .iter()
            .any(|pattern| path_str.contains(pattern.as_str()))
    }

    #[must_use]
    pub const fn root(&self) -> &SourcePath {
        &self.root
    }

    /// The source files `tree` holds under the root that this scanner admits.
    pub fn scan_files(&self, tree: &dyn SourceTree) -> Result<Vec<SourcePath>, StorageFailure> {
        tree.source_files(self)
    }

    #[must_use]
    pub fn with_excludes(mut self, patterns: Vec<String>) -> Self {
        self.exclude_patterns = patterns;
        self
    }

    #[must_use]
    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = extensions;
        self
    }
}
