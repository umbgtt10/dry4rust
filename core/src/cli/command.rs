// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// What the command line asked for, once parsed.
///
/// The core's own enum: the command line that spells it lives in the cli crate,
/// which builds one of these and hands it to the dispatcher.
#[derive(Debug, Clone)]
pub enum Command {
    /// Show duplication statistics only.
    Stats,
    /// Show full duplication report (default).
    Report,
    /// Check for duplicates and exit with non-zero if thresholds exceeded.
    Check {
        /// Maximum allowed exact duplicate groups.
        max_exact: Option<usize>,
        /// Maximum allowed near duplicate groups.
        max_near: Option<usize>,
        /// Maximum allowed exact duplicate percentage.
        max_exact_percent: Option<f64>,
        /// Maximum allowed near duplicate percentage.
        max_near_percent: Option<f64>,
    },
    /// Add a fingerprint to the ignore list.
    Ignore {
        /// The fingerprint to ignore (hex string).
        fingerprint: String,
        /// Reason for ignoring.
        reason: Option<String>,
    },
    /// List all ignored fingerprints.
    Ignored,
    /// Remove stale entries from the ignore file.
    Cleanup {
        /// Only list stale entries without removing them.
        dry_run: bool,
    },
    /// Record the duplication that is already there.
    Baseline {
        /// Only list what would be recorded, without writing the file.
        dry_run: bool,
    },
}
