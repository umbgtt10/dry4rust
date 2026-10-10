// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use assert_cmd::Command;
use std::env::consts::EXE_SUFFIX;
use std::env::current_exe;
use std::path::Path;
use std::process::Command as StdCommand;

pub struct DryBinary;

impl DryBinary {
    /// The built `cargo-dry4rust` binary.
    ///
    /// Found beside the running test binary rather than through
    /// `CARGO_BIN_EXE_*`, which cargo defines only for tests in the package that
    /// declares the binary -- and the binary is core's, while these tests are
    /// not. `cargo_bin` would resolve it, and is deprecated. Living in this
    /// library rather than in the tests changes nothing: `current_exe` is the
    /// process, and the process is still the test binary that called it.
    #[must_use]
    pub fn command() -> Command {
        let mut path = current_exe().expect("the test binary knows where it is");
        path.pop();
        if path.ends_with("deps") {
            path.pop();
        }
        path.push(format!("cargo-dry4rust{EXE_SUFFIX}"));
        Command::from_std(StdCommand::new(path))
    }

    /// The same, with `--path` already pointing at `root`, which is how most
    /// scenarios start.
    #[must_use]
    pub fn command_in(root: &Path) -> Command {
        let mut command = Self::command();
        command.arg("--path").arg(root);
        command
    }
}
