// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fmt;

use crate::ignore::IgnoreEntry;

/// One ignore entry as a single report line.
///
/// `ignored` and `cleanup` both list entries and must list them identically,
/// so the shape lives in one place. `Display` rather than a writer-taking
/// helper: the line is then a value a test can compare against, without a
/// buffer in between.
pub struct IgnoreEntryLine<'a> {
    entry: &'a IgnoreEntry,
}

impl<'a> IgnoreEntryLine<'a> {
    #[must_use]
    pub const fn new(entry: &'a IgnoreEntry) -> Self {
        Self { entry }
    }
}

impl fmt::Display for IgnoreEntryLine<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  {}", self.entry.fingerprint)?;
        if let Some(reason) = &self.entry.reason {
            write!(f, " (reason: {reason})")?;
        }
        if !self.entry.members.is_empty() {
            write!(f, " [{}]", self.entry.members.join(", "))?;
        }
        Ok(())
    }
}
