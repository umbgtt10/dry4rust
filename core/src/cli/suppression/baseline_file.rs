// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::analysis::Analysis;
use crate::cli::suppression::baseline_constants::DEFAULT_BASELINE_FILE;
use crate::cli::suppression::baseline_constants::FORMAT_VERSION;
use crate::cli::suppression::baseline_entry::BaselineEntry;
use crate::cli::suppression::baseline_kind::BaselineKind;
use crate::duplicate_group::DuplicateGroup;
use crate::error::Error;
use crate::error::Result;
use crate::source_path::SourcePath;
use crate::storage_failure::StorageFailure;
use crate::traits::baseline_store::BaselineStore;

/// The duplication a codebase already had when the baseline was recorded.
#[derive(Debug, Clone)]
pub struct BaselineFile {
    /// The format the entries are written in.
    version: u32,
    /// Every group that was there at the time.
    entries: Vec<BaselineEntry>,
}

impl BaselineFile {
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

    /// Read the baseline at `path` from `store`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Baseline`] when the file is missing, unreadable,
    /// malformed, or written in a format this build does not read. A baseline
    /// that cannot be applied is never treated as an empty one: that would
    /// quietly turn every inherited finding into a new one.
    pub fn load(store: &dyn BaselineStore, path: &SourcePath) -> Result<Self> {
        let file = store.load(path).map_err(|failure| Error::Baseline {
            path: path.clone(),
            problem: match failure {
                StorageFailure::Missing(_) => {
                    String::from("no such file; record one with `cargo dry4rust baseline`")
                }
                StorageFailure::Failed(reason) => reason,
            },
        })?;
        if file.version == FORMAT_VERSION {
            Ok(file)
        } else {
            Err(Error::Baseline {
                path: path.clone(),
                problem: format!(
                    "written in format {}, this build reads format {FORMAT_VERSION}; \
                     re-record it with `cargo dry4rust baseline`",
                    file.version
                ),
            })
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
    pub fn baseline_path(root: &SourcePath, configured: Option<&SourcePath>) -> SourcePath {
        match configured {
            Some(named) if named.is_absolute() => named.clone(),
            Some(named) => root.join(named.as_str()),
            None => root.join(DEFAULT_BASELINE_FILE),
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

    /// Write the baseline to `path` through `store`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Baseline`] if the store cannot write it.
    pub fn save(&self, store: &dyn BaselineStore, path: &SourcePath) -> Result<()> {
        store.save(path, self).map_err(|failure| Error::Baseline {
            path: path.clone(),
            problem: failure.to_string(),
        })
    }

    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }
}
