// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io::Write;
use std::path::Path;

use crate::analysis::AnalysisResult;
use crate::cli::cli_error::CliError;
use crate::cli::cli_error::CliResult;
use crate::config::Config;
use crate::suppression::baseline_file::BaselineFile;
use crate::suppression::baseline_file::baseline_path;

/// `baseline`: record the duplication that is already there.
///
/// The run behind this command judges nothing -- it cannot, or a second
/// recording would inherit the first and shrink to nothing. What it writes is
/// every group found, so a later `check` fails on what is added rather than on
/// what was found.
pub struct BaselineCommand<'a> {
    root: &'a Path,
    config: &'a Config,
    result: &'a AnalysisResult,
    dry_run: bool,
}

impl<'a> BaselineCommand<'a> {
    #[must_use]
    pub const fn new(
        root: &'a Path,
        config: &'a Config,
        result: &'a AnalysisResult,
        dry_run: bool,
    ) -> Self {
        Self {
            root,
            config,
            result,
            dry_run,
        }
    }

    /// Record the current duplication, or say what recording it would hold.
    ///
    /// # Errors
    ///
    /// Returns [`CliError::Analysis`] if the baseline cannot be written, and
    /// [`CliError::Io`] if the writer fails.
    pub fn run(&self, writer: &mut impl Write) -> CliResult {
        let path = baseline_path(self.root, self.config.baseline.as_deref());
        let recorded = BaselineFile::record(self.result);

        for entry in &recorded.entries {
            writeln!(
                writer,
                "  {} {} ({} members: {})",
                entry.kind,
                entry.fingerprint,
                entry.members,
                entry.names.join(", ")
            )?;
        }

        if self.dry_run {
            writeln!(
                writer,
                "{} groups would be recorded in {}.",
                recorded.len(),
                path.display()
            )?;
            return Ok(());
        }

        recorded.save(&path).map_err(CliError::Analysis)?;
        writeln!(
            writer,
            "Recorded {} groups in {}.",
            recorded.len(),
            path.display()
        )?;
        Ok(())
    }
}
