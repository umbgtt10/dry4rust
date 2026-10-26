// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use core::error::Error;
use core::fmt;

// A reporter that could not write its document, and why, in its own words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderFailure {
    pub reason: String,
}

impl fmt::Display for RenderFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.reason)
    }
}

impl Error for RenderFailure {}
