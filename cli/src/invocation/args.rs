// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::current_dir;
use std::path::PathBuf;

use clap::Parser;
use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::cli::command::Command;

use crate::invocation::cli_command::CliCommand;
use crate::invocation::output_format::OutputFormat;
use crate::system::portable_path::PortablePath;

/// The command line, and what it means.
///
/// It lives in the library rather than in `main` so the mapping from arguments
/// to a run -- which root, which command, which overrides -- is something a
/// test can call. `main` is left with nothing but an exit code to return.
#[derive(Parser)]
// `long_about = None` keeps the doc comment above for a reader of the source
// without clap promoting it into `--help`, where it would stand in for the
// one-line description.
#[command(
    name = "cargo-dry4rust",
    version,
    about = "Detect duplicate code in Rust codebases",
    long_about = None
)]
pub struct Args {
    /// When invoked as `cargo dry4rust`, cargo passes "dry4rust" as the first arg.
    #[arg(hide = true, default_value = "")]
    _cargo_subcommand: String,

    #[command(subcommand)]
    command: Option<CliCommand>,

    /// Path to analyze (defaults to current directory).
    #[arg(short, long, global = true)]
    path: Option<PathBuf>,

    /// Minimum AST node count for analysis.
    #[arg(long, global = true)]
    min_nodes: Option<usize>,

    /// Minimum source line count for analysis.
    #[arg(long, global = true)]
    min_lines: Option<usize>,

    /// Similarity threshold (0.0-1.0).
    #[arg(long, global = true)]
    threshold: Option<f64>,

    /// Output format.
    #[arg(long, global = true, default_value = "text")]
    format: OutputFormat,

    /// Exclude patterns (can be repeated).
    #[arg(long, global = true)]
    exclude: Vec<String>,

    /// Exclude test code (#[test] functions and #[cfg(test)] modules).
    #[arg(long, global = true)]
    exclude_tests: bool,

    /// Enable sub-function duplicate detection (if branches, match arms, loop bodies).
    #[arg(long, short = 's', global = true)]
    sub_function: bool,

    /// Minimum AST node count for sub-function units.
    #[arg(long, global = true)]
    min_sub_nodes: Option<usize>,

    /// Baseline of inherited duplication: groups it records are not reported
    /// and do not fail a check. Record one with the `baseline` subcommand.
    #[arg(long, global = true)]
    baseline: Option<PathBuf>,
}

impl Args {
    /// The command to run. Naming none means `report`.
    #[must_use]
    pub fn command(&self) -> Command {
        self.command
            .as_ref()
            .map_or(Command::Report, CliCommand::command)
    }

    /// The output format asked for.
    #[must_use]
    pub const fn format(&self) -> OutputFormat {
        self.format
    }

    /// What the flags override in the loaded configuration.
    ///
    /// The three booleans become `Some(true)` or `None` rather than
    /// `Some(false)`: a flag that was not passed has no opinion, and a config
    /// file that turned the behaviour on must not be switched off by its
    /// absence. Paths and patterns reach the core with `/` for a separator.
    #[must_use]
    pub fn overrides(&self) -> CliOverrides {
        CliOverrides::default()
            .with_min_nodes(self.min_nodes)
            .with_min_lines(self.min_lines)
            .with_threshold(self.threshold)
            .with_exclude(
                self.exclude
                    .iter()
                    .map(|pattern| PortablePath::pattern(pattern))
                    .collect(),
            )
            .with_exclude_tests(self.exclude_tests.then_some(true))
            .with_sub_function(self.sub_function.then_some(true))
            .with_min_sub_nodes(self.min_sub_nodes)
            .with_baseline(self.baseline.as_deref().map(PortablePath::lossy))
    }

    /// The root to analyse: what `--path` said, or where the shell is.
    #[must_use]
    pub fn root(&self) -> PathBuf {
        self.path
            .clone()
            .unwrap_or_else(|| current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    }
}
