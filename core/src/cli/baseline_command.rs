// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fmt::Write as _;
use std::path::Path;

use crate::analysis::Analysis;
use crate::cli::cli_error::CliError;
use crate::cli::cli_error::CliResult;
use crate::cli::command_outcome::CommandOutcome;
use crate::config::Config;
use crate::suppression::baseline_file::BaselineFile;

/// `baseline`: record the duplication that is already there.
///
/// The run behind this command judges nothing -- it cannot, or a second
/// recording would inherit the first and shrink to nothing. What it writes is
/// every group found, so a later `check` fails on what is added rather than on
/// what was found.
pub struct BaselineCommand<'a> {
    root: &'a Path,
    config: &'a Config,
    result: &'a Analysis,
    dry_run: bool,
}

impl<'a> BaselineCommand<'a> {
    #[must_use]
    pub const fn new(
        root: &'a Path,
        config: &'a Config,
        result: &'a Analysis,
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
    /// Never returns an error itself. A baseline that cannot be written is
    /// carried in the outcome's result as [`CliError::Analysis`], beside the
    /// entries already listed, so they still reach stdout ahead of the error.
    pub fn run(&self) -> CliResult<CommandOutcome> {
        let path = BaselineFile::baseline_path(self.root, self.config.baseline());
        let recorded = BaselineFile::record(self.result);

        let mut out = String::new();
        for entry in recorded.entries() {
            let _ = writeln!(
                out,
                "  {} {} ({} members: {})",
                entry.kind(),
                entry.fingerprint(),
                entry.members(),
                entry.names().join(", ")
            );
        }

        if self.dry_run {
            let _ = writeln!(
                out,
                "{} groups would be recorded in {}.",
                recorded.len(),
                path.display()
            );
            return Ok(CommandOutcome::succeeded(out));
        }

        if let Err(error) = recorded.save(&path) {
            return Ok(CommandOutcome::new(out, Err(CliError::Analysis(error))));
        }
        let _ = writeln!(
            out,
            "Recorded {} groups in {}.",
            recorded.len(),
            path.display()
        );
        Ok(CommandOutcome::succeeded(out))
    }
}
