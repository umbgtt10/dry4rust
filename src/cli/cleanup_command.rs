// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io::Write;
use std::path::Path;

use crate::analysis::AnalysisResult;
use crate::cli::checking::stale_report::StaleReport;
use crate::cli::cli_error::CliResult;
use crate::cli::ignore_entry_line::IgnoreEntryLine;
use crate::ignore::find_stale_entries;
use crate::ignore::load_ignore_file;
use crate::ignore::remove_stale_entries;
use crate::ignore::save_ignore_file;

/// `cleanup`: drop ignore entries whose fingerprint no longer matches anything.
///
/// In dry run the file is read and reported on but never written, so the
/// command can be pointed at a repository whose ignore file is under review.
pub struct CleanupCommand<'a> {
    root: &'a Path,
    result: &'a AnalysisResult,
    dry_run: bool,
}

impl<'a> CleanupCommand<'a> {
    #[must_use]
    pub const fn new(root: &'a Path, result: &'a AnalysisResult, dry_run: bool) -> Self {
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
    /// cannot be written or the writer fails.
    pub fn run(&self, writer: &mut impl Write) -> CliResult {
        let mut ignore_file = load_ignore_file(self.root);
        let taken;

        let report = if self.dry_run {
            StaleReport::dry_run(find_stale_entries(
                &ignore_file,
                &self.result.all_fingerprints,
            ))
        } else {
            taken = remove_stale_entries(&mut ignore_file, &self.result.all_fingerprints);
            if !taken.is_empty() {
                save_ignore_file(self.root, &ignore_file)?;
            }
            StaleReport::removed(taken.iter().collect())
        };

        if report.is_empty() {
            writeln!(writer, "No stale entries found.")?;
            return Ok(());
        }

        writeln!(writer, "{}", report.heading())?;
        for entry in report.entries() {
            writeln!(writer, "{}", IgnoreEntryLine::new(entry))?;
        }
        writeln!(writer, "{}", report.summary())?;
        Ok(())
    }
}
