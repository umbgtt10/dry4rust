// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io::Write;
use std::path::Path;

use crate::cli::cli_error::CliResult;
use crate::cli::ignore_entry_line::IgnoreEntryLine;
use crate::suppression::ignore_file::IgnoreFile;

/// `ignored`: list what the ignore file holds.
pub struct IgnoredCommand<'a> {
    root: &'a Path,
}

impl<'a> IgnoredCommand<'a> {
    #[must_use]
    pub const fn new(root: &'a Path) -> Self {
        Self { root }
    }

    /// Write every ignore entry, or say there are none.
    ///
    /// # Errors
    ///
    /// Returns [`crate::cli::cli_error::CliError::Io`] if the writer fails.
    pub fn run(&self, writer: &mut impl Write) -> CliResult {
        let ignore_file = IgnoreFile::load(self.root);
        if ignore_file.ignore.is_empty() {
            writeln!(writer, "No ignored fingerprints.")?;
            return Ok(());
        }
        writeln!(writer, "Ignored fingerprints:")?;
        for entry in &ignore_file.ignore {
            writeln!(writer, "{}", IgnoreEntryLine::new(entry))?;
        }
        Ok(())
    }
}
