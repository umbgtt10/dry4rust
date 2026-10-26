// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use clap::Subcommand;
use dry4rust::cli::command::Command;

/// CLI subcommands for `cargo-dry4rust`.
#[derive(Debug, Clone, Subcommand)]
pub enum CliCommand {
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
    /// Record the duplication that is already there, so `check` fails on what
    /// is added rather than on what was inherited.
    Baseline {
        /// Only list what would be recorded, without writing the file.
        #[arg(long)]
        dry_run: bool,
    },
}

impl CliCommand {
    /// The core's command for what was typed.
    #[must_use]
    pub fn command(&self) -> Command {
        match self.clone() {
            Self::Stats => Command::Stats,
            Self::Report => Command::Report,
            Self::Check {
                max_exact,
                max_near,
                max_exact_percent,
                max_near_percent,
            } => Command::Check {
                max_exact,
                max_near,
                max_exact_percent,
                max_near_percent,
            },
            Self::Ignore {
                fingerprint,
                reason,
            } => Command::Ignore {
                fingerprint,
                reason,
            },
            Self::Ignored => Command::Ignored,
            Self::Cleanup { dry_run } => Command::Cleanup { dry_run },
            Self::Baseline { dry_run } => Command::Baseline { dry_run },
        }
    }
}
