// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_json::to_string_pretty;

use crate::analysis::Analysis;
use crate::cli::suppression::baseline_constants::DEFAULT_BASELINE_FILE;
use crate::cli::suppression::baseline_constants::FORMAT_VERSION;
use crate::cli::suppression::baseline_entry::BaselineEntry;
use crate::cli::suppression::baseline_kind::BaselineKind;
use crate::duplicate_group::DuplicateGroup;
use crate::error::Error;
use crate::error::Result;

/// The duplication a codebase already had when the baseline was recorded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineFile {
    /// The format the entries are written in.
    version: u32,
    /// Every group that was there at the time.
    entries: Vec<BaselineEntry>,
}

impl BaselineFile {
    /// Read the baseline at `path`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Baseline`] when the file is missing, unreadable,
    /// malformed, or written in a format this build does not read. A baseline
    /// that cannot be applied is never treated as an empty one: that would
    /// quietly turn every inherited finding into a new one.
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path).map_err(|e| Error::Baseline {
            path: path.to_path_buf(),
            problem: if e.kind() == ErrorKind::NotFound {
                String::from("no such file; record one with `cargo dry4rust baseline`")
            } else {
                e.to_string()
            },
        })?;
        let file: Self = from_str(&content).map_err(|e| Error::Baseline {
            path: path.to_path_buf(),
            problem: e.to_string(),
        })?;
        if file.version == FORMAT_VERSION {
            Ok(file)
        } else {
            Err(Error::Baseline {
                path: path.to_path_buf(),
                problem: format!(
                    "written in format {}, this build reads format {FORMAT_VERSION}; \
                     re-record it with `cargo dry4rust baseline`",
                    file.version
                ),
            })
        }
    }

    /// Record everything `result` found.
    ///
    /// Entries are ordered by kind and then by fingerprint, so re-recording an
    /// unchanged codebase produces a byte-identical file and a diff shows only
    /// what actually moved.
    #[must_use]
    pub fn record(result: &Analysis) -> Self {
        let mut entries: Vec<BaselineEntry> = [
            (BaselineKind::Exact, &result.exact_groups()),
            (BaselineKind::Near, &result.near_groups()),
            (BaselineKind::SubExact, &result.sub_exact_groups()),
            (BaselineKind::SubNear, &result.sub_near_groups()),
        ]
        .into_iter()
        .flat_map(|(kind, groups)| groups.iter().map(move |g| BaselineEntry::of(kind, g)))
        .collect();
        entries.sort_by(|a, b| {
            a.kind()
                .cmp(&b.kind())
                .then_with(|| a.fingerprint().cmp(b.fingerprint()))
        });
        Self {
            version: FORMAT_VERSION,
            entries,
        }
    }

    #[must_use]
    pub const fn new(version: u32, entries: Vec<BaselineEntry>) -> Self {
        Self { version, entries }
    }

    /// Whether some entry accounts for `group` in full.
    #[must_use]
    pub fn admits(&self, kind: BaselineKind, group: &DuplicateGroup) -> bool {
        self.entries.iter().any(|entry| entry.admits(kind, group))
    }

    /// Where the baseline lives for a given root.
    ///
    /// A relative path is taken as relative to the analysed root, so a
    /// `dry4rust.toml` naming one means the same thing wherever it is run from.
    #[must_use]
    pub fn baseline_path(root: &Path, configured: Option<&Path>) -> PathBuf {
        let named = configured.unwrap_or_else(|| Path::new(DEFAULT_BASELINE_FILE));
        if named.is_absolute() {
            named.to_path_buf()
        } else {
            root.join(named)
        }
    }

    #[must_use]
    pub fn entries(&self) -> &[BaselineEntry] {
        &self.entries
    }

    /// Whether it records nothing at all.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// How many groups this baseline records.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.entries.len()
    }

    /// Write the baseline to `path`, creating the directory above it.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Baseline`] if the file cannot be written.
    pub fn save(&self, path: &Path) -> Result<()> {
        let problem = |e: IoError| Error::Baseline {
            path: path.to_path_buf(),
            problem: e.to_string(),
        };
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).map_err(problem)?;
        }
        let json = to_string_pretty(self).map_err(|e| Error::Baseline {
            path: path.to_path_buf(),
            problem: e.to_string(),
        })?;
        fs::write(path, format!("{json}\n")).map_err(problem)
    }

    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }
}
