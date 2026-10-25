// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fmt::Write as _;
use std::path::Path;

use crate::analysis::Analysis;
use crate::cli::checking::stale_report::StaleReport;
use crate::cli::cli_error::CliResult;
use crate::cli::command_outcome::CommandOutcome;
use crate::cli::ignore_entry_line::IgnoreEntryLine;
use crate::suppression::ignore_file::IgnoreFile;

/// `cleanup`: drop ignore entries whose fingerprint no longer matches anything.
///
/// In dry run the file is read and reported on but never written, so the
/// command can be pointed at a repository whose ignore file is under review.
pub struct CleanupCommand<'a> {
    root: &'a Path,
    result: &'a Analysis,
    dry_run: bool,
}

impl<'a> CleanupCommand<'a> {
    #[must_use]
    pub const fn new(root: &'a Path, result: &'a Analysis, dry_run: bool) -> Self {
        Self {
            root,
            result,
            dry_run,
        }
    }

    /// Report the stale entries, and remove them unless this is a dry run.
    ///
    /// # Errors
    ///
    /// Returns [`crate::cli::cli_error::CliError::Io`] if the ignore file
    /// cannot be written.
    pub fn run(&self) -> CliResult<CommandOutcome> {
        let ignore_file = IgnoreFile::load(self.root);
        let taken;

        let report = if self.dry_run {
            StaleReport::dry_run(ignore_file.stale(self.result.all_fingerprints()))
        } else {
            let pruned;
            (pruned, taken) = ignore_file.without_stale(self.result.all_fingerprints());
            if !taken.is_empty() {
                pruned.save(self.root)?;
            }
            StaleReport::removed(taken.iter().collect())
        };

        let mut out = String::new();
        if report.is_empty() {
            let _ = writeln!(out, "No stale entries found.");
            return Ok(CommandOutcome::succeeded(out));
        }

        let _ = writeln!(out, "{}", report.heading());
        for entry in report.entries() {
            let _ = writeln!(out, "{}", IgnoreEntryLine { entry });
        }
        let _ = writeln!(out, "{}", report.summary());
        Ok(CommandOutcome::succeeded(out))
    }
}
