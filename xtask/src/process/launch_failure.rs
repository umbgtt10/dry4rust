// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fmt;

// A program the runner could not start, and why -- said the way the run has
// always reported it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchFailure {
    program: String,
    reason: String,
}

impl LaunchFailure {
    #[must_use]
    pub const fn new(program: String, reason: String) -> Self {
        Self { program, reason }
    }

    #[must_use]
    pub fn program(&self) -> &str {
        &self.program
    }

    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl fmt::Display for LaunchFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "failed to launch {}: {}",
            self.program, self.reason
        )
    }
}
