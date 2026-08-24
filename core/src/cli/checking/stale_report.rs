// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::suppression::ignore_entry::IgnoreEntry;

/// What `cleanup` found, and the words it uses to say so.
///
/// The two halves of `cleanup` -- listing stale entries and removing them --
/// print the same three things in the same order and differ only in tense.
/// Holding the tense here leaves one printing path instead of two.
pub struct StaleReport<'a> {
    entries: Vec<&'a IgnoreEntry>,
    removed: bool,
}

impl<'a> StaleReport<'a> {
    /// Entries that would go, had this not been a dry run.
    #[must_use]
    pub const fn dry_run(entries: Vec<&'a IgnoreEntry>) -> Self {
        Self {
            entries,
            removed: false,
        }
    }

    /// Entries that have gone.
    #[must_use]
    pub const fn removed(entries: Vec<&'a IgnoreEntry>) -> Self {
        Self {
            entries,
            removed: true,
        }
    }

    /// Whether the ignore file held nothing stale.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The entries themselves, in the order they were found.
    #[must_use]
    pub fn entries(&self) -> &[&'a IgnoreEntry] {
        &self.entries
    }

    /// The line introducing the list.
    #[must_use]
    pub const fn heading(&self) -> &'static str {
        if self.removed {
            "Removed stale entries:"
        } else {
            "Stale entries (dry run):"
        }
    }

    /// The line closing it, which is where the tense shows.
    #[must_use]
    pub fn summary(&self) -> String {
        let count = self.entries.len();
        if self.removed {
            format!("\nRemoved {count} stale entries.")
        } else {
            format!("\n{count} stale entries would be removed.")
        }
    }
}
