// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;
use std::env::args;
use std::ffi::OsString;
use std::process::ExitCode;

// The command line, such as it is.
//
// `dry4rust` is a reserved placeholder: it takes arguments, ignores them and
// says so. The shape is here so the crate name is claimed and the binary
// answers, and so that whatever eventually reads those arguments has a file to
// be written in rather than a `lib.rs` to grow into.
pub struct Cli;

impl Cli {
    pub fn run() -> Result<ExitCode> {
        Self::run_from_args(args())
    }

    // Arguments are taken and dropped rather than left unread, so the signature
    // the real implementation needs is already the one every caller uses.
    pub fn run_from_args<I, T>(args: I) -> Result<ExitCode>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let _ = args;
        println!(
            "dry4rust: not yet implemented — reserved placeholder for Rust code-duplication (DRY) analysis."
        );
        Ok(ExitCode::SUCCESS)
    }
}
