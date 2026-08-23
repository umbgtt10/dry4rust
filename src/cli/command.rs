// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// CLI subcommands for `cargo-dry4rust`.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum Command {
    /// Show duplication statistics only.
    Stats,
    /// Show full duplication report (default).
    Report,
    /// Check for duplicates and exit with non-zero if thresholds exceeded.
    Check {
        /// Maximum allowed exact duplicate groups (exit 1 if exceeded).
        #[arg(long)]
        max_exact: Option<usize>,
        /// Maximum allowed near duplicate groups (exit 1 if exceeded).
        #[arg(long)]
        max_near: Option<usize>,
        /// Maximum allowed exact duplicate percentage (exit 1 if exceeded).
        #[arg(long)]
        max_exact_percent: Option<f64>,
        /// Maximum allowed near duplicate percentage (exit 1 if exceeded).
        #[arg(long)]
        max_near_percent: Option<f64>,
    },
    /// Add a fingerprint to the ignore list.
    Ignore {
        /// The fingerprint to ignore (hex string).
        fingerprint: String,
        /// Reason for ignoring.
        #[arg(long)]
        reason: Option<String>,
    },
    /// List all ignored fingerprints.
    Ignored,
    /// Remove stale entries from the ignore file.
    Cleanup {
        /// Only list stale entries without removing them.
        #[arg(long)]
        dry_run: bool,
    },
}
