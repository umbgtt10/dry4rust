// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Configuration for scanning the filesystem for source files.
pub struct ScanConfig {
    /// Root directory to scan.
    pub root: PathBuf,
    /// Glob patterns to exclude (simple substring matching for now).
    pub exclude_patterns: Vec<String>,
    /// File extensions to include (without the leading dot). Defaults to `["rs"]`.
    pub extensions: Vec<String>,
}

impl ScanConfig {
    #[must_use]
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            exclude_patterns: Vec::new(),
            extensions: vec!["rs".to_string()],
        }
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

/// Scan for source files under the given config.
/// Always skips `target/` directories.
#[must_use]
pub fn scan_files(config: &ScanConfig) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in WalkDir::new(&config.root)
        .into_iter()
        .filter_entry(|e| {
            let path = e.path();
            // Only filter directories (not the root itself for hidden check)
            if path.is_dir()
                && let Some(name) = path.file_name().and_then(|n| n.to_str())
            {
                if name == "target" {
                    return false;
                }
                // Skip hidden directories, but not the root
                if name.starts_with('.') && path != config.root.as_path() {
                    return false;
                }
            }
            true
        })
        .flatten()
    {
        let path = entry.path();
        if path.is_file()
            && path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| {
                    config
                        .extensions
                        .iter()
                        .any(|e| e.eq_ignore_ascii_case(ext))
                })
            && !is_excluded(path, &config.exclude_patterns)
        {
            files.push(path.to_path_buf());
        }
    }

    files
}

/// Check if a path should be excluded based on exclusion patterns.
#[must_use]
pub fn is_excluded(path: &Path, patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    patterns
        .iter()
        .any(|pattern| path_str.contains(pattern.as_str()))
}
