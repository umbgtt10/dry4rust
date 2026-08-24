// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

/// Which of the four sets of groups a baseline entry was recorded from.
///
/// Recorded alongside the fingerprint because the four sets are counted and
/// gated separately, and a fingerprint alone would let a recorded sub-function
/// duplicate suppress a whole function that happened to hash the same.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BaselineKind {
    Exact,
    Near,
    SubExact,
    SubNear,
}

impl fmt::Display for BaselineKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Exact => "exact",
            Self::Near => "near",
            Self::SubExact => "sub-function exact",
            Self::SubNear => "sub-function near",
        };
        f.write_str(name)
    }
}
