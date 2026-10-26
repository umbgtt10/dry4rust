// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use core::error::Error;
use core::fmt;

// What a port's storage said when it could not do what was asked, in the
// platform's words. `Missing` is the one case the core tells apart: a file that
// was never there, which some callers treat as an answer rather than a fault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageFailure {
    Missing(String),
    Failed(String),
}

impl fmt::Display for StorageFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(reason) | Self::Failed(reason) => formatter.write_str(reason),
        }
    }
}

impl Error for StorageFailure {}
