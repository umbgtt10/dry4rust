// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use core::fmt::Write as _;

use crate::cli::cli_error::CliResult;
use crate::cli::command_outcome::CommandOutcome;
use crate::cli::ignore_entry_line::IgnoreEntryLine;
use crate::cli::suppression::ignore_file::IgnoreFile;
use crate::source_path::SourcePath;
use crate::traits::ignore_store::IgnoreStore;

/// `ignored`: list what the ignore file holds.
pub struct IgnoredCommand<'a> {
    store: &'a dyn IgnoreStore,
    root: &'a SourcePath,
}

impl<'a> IgnoredCommand<'a> {
    #[must_use]
    pub const fn new(store: &'a dyn IgnoreStore, root: &'a SourcePath) -> Self {
        Self { store, root }
    }

    /// Build every ignore entry, or say there are none.
    ///
    /// # Errors
    ///
    /// Never fails; the signature matches its siblings so the dispatcher can
    /// treat every command alike.
    pub fn run(&self) -> CliResult<CommandOutcome> {
        let ignore_file = IgnoreFile::load(self.store, self.root);
        let mut out = String::new();
        if ignore_file.ignore().is_empty() {
            let _ = writeln!(out, "No ignored fingerprints.");
            return Ok(CommandOutcome::succeeded(out));
        }
        let _ = writeln!(out, "Ignored fingerprints:");
        for entry in ignore_file.ignore() {
            let _ = writeln!(out, "{}", IgnoreEntryLine { entry });
        }
        Ok(CommandOutcome::succeeded(out))
    }
}
