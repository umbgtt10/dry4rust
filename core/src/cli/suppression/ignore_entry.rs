// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// One duplicate somebody decided should stay.
///
/// The counterpart of [`crate::cli::suppression::baseline_entry::BaselineEntry`],
/// and the difference is who writes it. This one is written by a person and
/// carries a reason in their words; a baseline entry is written by the tool
/// and read back by it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IgnoreEntry {
    /// The fingerprint of the duplicated code.
    pub fingerprint: String,
    /// Optional reason for ignoring.
    pub reason: Option<String>,
    /// Names of the code units in the group (for documentation).
    pub members: Vec<String>,
}
