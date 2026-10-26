// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::borrow::ToOwned;
use alloc::format;
use alloc::vec;

use crate::cli::cli_error::CliError;
use crate::cli::cli_error::CliResult;
use crate::cli::command_outcome::CommandOutcome;
use crate::cli::suppression::ignore_file::IgnoreFile;
use crate::fingerprint::Fingerprint;
use crate::source_path::SourcePath;
use crate::traits::ignore_store::IgnoreStore;

/// `ignore`: record one fingerprint as a duplicate that is meant to be there.
pub struct IgnoreCommand<'a> {
    store: &'a dyn IgnoreStore,
    root: &'a SourcePath,
    fingerprint: &'a str,
    reason: Option<&'a str>,
}

impl<'a> IgnoreCommand<'a> {
    #[must_use]
    pub const fn new(
        store: &'a dyn IgnoreStore,
        root: &'a SourcePath,
        fingerprint: &'a str,
        reason: Option<&'a str>,
    ) -> Self {
        Self {
            store,
            root,
            fingerprint,
            reason,
        }
    }

    /// Add the fingerprint to the ignore file.
    ///
    /// # Errors
    ///
    /// Returns [`CliError::InvalidFingerprint`] if the string is not a
    /// fingerprint this tool could have produced, and [`CliError::Io`] if the
    /// ignore file cannot be written.
    pub fn run(&self) -> CliResult<CommandOutcome> {
        let fingerprint = Fingerprint::from_hex(self.fingerprint)
            .ok_or_else(|| CliError::InvalidFingerprint(self.fingerprint.to_owned()))?;
        IgnoreFile::load(self.store, self.root)
            .with_ignored(&fingerprint, self.reason.map(ToOwned::to_owned), vec![])
            .save(self.store, self.root)?;
        Ok(CommandOutcome::succeeded(format!(
            "Added {} to ignore list.\n",
            self.fingerprint
        )))
    }
}
