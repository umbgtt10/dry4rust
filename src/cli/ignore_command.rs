// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io::Write;
use std::path::Path;

use crate::cli::cli_error::CliError;
use crate::cli::cli_error::CliResult;
use crate::fingerprint::Fingerprint;
use crate::ignore::add_ignore;
use crate::ignore::load_ignore_file;
use crate::ignore::save_ignore_file;

/// `ignore`: record one fingerprint as a duplicate that is meant to be there.
pub struct IgnoreCommand<'a> {
    root: &'a Path,
    fingerprint: &'a str,
    reason: Option<&'a str>,
}

impl<'a> IgnoreCommand<'a> {
    #[must_use]
    pub const fn new(root: &'a Path, fingerprint: &'a str, reason: Option<&'a str>) -> Self {
        Self {
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
    pub fn run(&self, writer: &mut impl Write) -> CliResult {
        let fingerprint = Fingerprint::from_hex(self.fingerprint)
            .ok_or_else(|| CliError::InvalidFingerprint(self.fingerprint.to_owned()))?;
        let mut ignore_file = load_ignore_file(self.root);
        add_ignore(
            &mut ignore_file,
            &fingerprint,
            self.reason.map(ToOwned::to_owned),
            vec![],
        );
        save_ignore_file(self.root, &ignore_file)?;
        writeln!(writer, "Added {} to ignore list.", self.fingerprint)?;
        Ok(())
    }
}
