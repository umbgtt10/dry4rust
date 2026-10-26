// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::HashSet;

use crate::cli::suppression::ignore_entry::IgnoreEntry;
use crate::duplicate_group::DuplicateGroup;
use crate::fingerprint::Fingerprint;
use crate::source_path::SourcePath;
use crate::storage_failure::StorageFailure;
use crate::traits::ignore_store::IgnoreStore;

const IGNORE_FILE_NAME: &str = ".dry4rust-ignore.toml";

/// The suppressions a project has decided to keep.
///
/// Every operation that changes the file takes it by value and hands back the
/// changed one, so a caller cannot half-apply a change and cannot be handed a
/// file that something else is still writing to.
#[derive(Debug, Clone, Default)]
pub struct IgnoreFile {
    ignore: Vec<IgnoreEntry>,
}

impl IgnoreFile {
    #[must_use]
    pub const fn new(ignore: Vec<IgnoreEntry>) -> Self {
        Self { ignore }
    }

    /// Read it from `store`, or an empty one if there is nothing readable
    /// there.
    ///
    /// Unlike a baseline, an absent ignore file is the ordinary case: most
    /// projects suppress nothing, and that is indistinguishable from a project
    /// that has not started yet. One the store cannot read or parse is passed
    /// over the same way.
    #[must_use]
    pub fn load(store: &dyn IgnoreStore, root: &SourcePath) -> Self {
        store.load(&Self::path_in(root)).unwrap_or_default()
    }

    /// Whether this fingerprint is suppressed.
    #[must_use]
    pub fn contains(&self, fingerprint: &Fingerprint) -> bool {
        let hex = fingerprint.to_hex();
        self.ignore.iter().any(|entry| entry.fingerprint == hex)
    }

    #[must_use]
    pub fn ignore(&self) -> &[IgnoreEntry] {
        &self.ignore
    }

    /// Where the ignore file lives for a project root.
    #[must_use]
    pub fn path_in(root: &SourcePath) -> SourcePath {
        root.join(IGNORE_FILE_NAME)
    }

    /// Take the ignore file away, and say nothing if it was not there.
    ///
    /// # Errors
    ///
    /// Returns what the store said if the file exists and cannot be removed.
    pub fn remove_from(store: &dyn IgnoreStore, root: &SourcePath) -> Result<(), StorageFailure> {
        match store.remove(&Self::path_in(root)) {
            Err(StorageFailure::Missing(_)) => Ok(()),
            outcome => outcome,
        }
    }

    /// The groups this file does not suppress.
    #[must_use]
    pub fn retain_unsuppressed(&self, groups: Vec<DuplicateGroup>) -> Vec<DuplicateGroup> {
        groups
            .into_iter()
            .filter(|group| !self.contains(&group.fingerprint()))
            .collect()
    }

    /// Write it to the project root, or take the file away when there is
    /// nothing left to record.
    ///
    /// An empty suppression list makes exactly the claim that no suppression
    /// list makes, and `load` cannot tell them apart. `cleanup` pruning the
    /// last entry is the one path that produces one, and a file reading
    /// `ignore = []` is residue rather than a record.
    ///
    /// # Errors
    ///
    /// Returns what the store said if the file cannot be serialized, written
    /// or removed.
    pub fn save(&self, store: &dyn IgnoreStore, root: &SourcePath) -> Result<(), StorageFailure> {
        if self.ignore.is_empty() {
            return Self::remove_from(store, root);
        }
        store.save(&Self::path_in(root), self)
    }

    /// The entries whose fingerprint no longer matches anything live.
    ///
    /// An entry whose fingerprint is not valid hex is always stale: nothing
    /// this tool produces could match it.
    #[must_use]
    pub fn stale(&self, live: &HashSet<Fingerprint>) -> Vec<&IgnoreEntry> {
        self.ignore
            .iter()
            .filter(|entry| !Self::is_live(entry, live))
            .collect()
    }

    /// The same file with `fingerprint` suppressed, or unchanged if it already
    /// was.
    #[must_use]
    pub fn with_ignored(
        self,
        fingerprint: &Fingerprint,
        reason: Option<String>,
        members: Vec<String>,
    ) -> Self {
        let hex = fingerprint.to_hex();
        if self.ignore.iter().any(|entry| entry.fingerprint == hex) {
            return self;
        }
        let mut file = self;
        file.ignore.push(IgnoreEntry {
            fingerprint: hex,
            reason,
            members,
        });
        file
    }

    /// The same file without `fingerprint`, and whether it was there to remove.
    #[must_use]
    pub fn without(self, fingerprint: &str) -> (Self, bool) {
        let mut file = self;
        let before = file.ignore.len();
        file.ignore.retain(|entry| entry.fingerprint != fingerprint);
        let removed = file.ignore.len() < before;
        (file, removed)
    }

    /// The same file with the stale entries taken out, and those entries.
    #[must_use]
    pub fn without_stale(self, live: &HashSet<Fingerprint>) -> (Self, Vec<IgnoreEntry>) {
        let (kept, stale) = self
            .ignore
            .into_iter()
            .partition(|entry| Self::is_live(entry, live));
        (Self { ignore: kept }, stale)
    }

    fn is_live(entry: &IgnoreEntry, live: &HashSet<Fingerprint>) -> bool {
        Fingerprint::from_hex(&entry.fingerprint).is_some_and(|fp| live.contains(&fp))
    }
}
