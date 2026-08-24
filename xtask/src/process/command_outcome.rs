// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutcome {
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

impl CommandOutcome {
    #[must_use]
    pub const fn new(exit_code: Option<i32>, stdout: String, stderr: String) -> Self {
        Self {
            exit_code,
            stdout,
            stderr,
        }
    }

    // None means the process was killed by a signal rather than exiting, which
    // is a failure and not an unknown.
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.exit_code == Some(0)
    }
}
