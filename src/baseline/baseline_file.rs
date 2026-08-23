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

use crate::analysis::AnalysisResult;
use crate::baseline::baseline_entry::BaselineEntry;
use crate::baseline::baseline_kind::BaselineKind;
use crate::error::Error;
use crate::error::Result;
use crate::grouper::DuplicateGroup;

/// What `baseline` writes when no path says otherwise.
///
/// JSON rather than the TOML the ignore file uses, and named for the tool the
/// way every other file in this family is. The two suppression files differ in
/// who writes them: an ignore entry is written by a person and carries a reason
/// in their words, and a baseline is written by the tool and read back by it.
pub const DEFAULT_BASELINE_FILE: &str = "dry4rust-baseline.json";

/// The format this build writes, and the only one it reads.
pub const FORMAT_VERSION: u32 = 1;

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

/// The duplication a codebase already had when the baseline was recorded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineFile {
    /// The format the entries are written in.
    pub version: u32,
    /// Every group that was there at the time.
    pub entries: Vec<BaselineEntry>,
}

impl BaselineFile {
    /// Record everything `result` found.
    ///
    /// Entries are ordered by kind and then by fingerprint, so re-recording an
    /// unchanged codebase produces a byte-identical file and a diff shows only
    /// what actually moved.
    #[must_use]
    pub fn record(result: &AnalysisResult) -> Self {
        let mut entries: Vec<BaselineEntry> = [
            (BaselineKind::Exact, &result.exact_groups),
            (BaselineKind::Near, &result.near_groups),
            (BaselineKind::SubExact, &result.sub_exact_groups),
            (BaselineKind::SubNear, &result.sub_near_groups),
        ]
        .into_iter()
        .flat_map(|(kind, groups)| groups.iter().map(move |g| BaselineEntry::of(kind, g)))
        .collect();
        entries.sort_by(|a, b| {
            a.kind
                .cmp(&b.kind)
                .then_with(|| a.fingerprint.cmp(&b.fingerprint))
        });
        Self {
            version: FORMAT_VERSION,
            entries,
        }
    }

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

    /// How many groups this baseline records.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether it records nothing at all.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Whether some entry accounts for `group` in full.
    #[must_use]
    pub fn admits(&self, kind: BaselineKind, group: &DuplicateGroup) -> bool {
        self.entries.iter().any(|entry| entry.admits(kind, group))
    }
}
