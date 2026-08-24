// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::grouper::{DuplicateGroup, DuplicationStats};

/// Everything a full report shows, handed to a reporter in one piece.
///
/// It exists because the shape of the document is the format's business and
/// not the command's. Text can be written a section at a time and read
/// correctly; JSON cannot, and a caller that emitted one section per call
/// produced several top-level documents rather than one.
pub struct Report<'a> {
    pub stats: &'a DuplicationStats,
    pub exact: &'a [DuplicateGroup],
    pub near: &'a [DuplicateGroup],
    pub sub_exact: &'a [DuplicateGroup],
    pub sub_near: &'a [DuplicateGroup],
}
