// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::current_dir;
use std::io;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Error as ClapError;
use clap::Parser;

use crate::cli::cli_error::CliError;
use crate::cli::cli_overrides::CliOverrides;
use crate::cli::command::Command;
use crate::cli::output_format::OutputFormat;
use crate::command_dispatcher::CommandDispatcher;
use crate::rust::rust_analyzer::RustAnalyzer;

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
pub struct EntryPoint {
    /// When invoked as `cargo dry4rust`, cargo passes "dry4rust" as the first arg.
    #[arg(hide = true, default_value = "")]
    _cargo_subcommand: String,

    #[command(subcommand)]
    command: Option<Command>,

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

impl EntryPoint {
    /// Parse `args`, run what they ask for, and report the outcome.
    ///
    /// Taking the arguments rather than reading `argv` is what lets a test
    /// call this at all; `main` passes the real ones.
    #[must_use]
    pub fn run(args: Vec<String>) -> ExitCode {
        match Self::try_parse_from(args) {
            Ok(entry) => entry.execute(),
            Err(error) => Self::parsing_failed(&error),
        }
    }

    fn execute(self) -> ExitCode {
        let analyzer = RustAnalyzer::new();
        let root = self.root();
        let dispatcher = CommandDispatcher::new(&analyzer, &root, self.format, self.overrides());
        let stdout = io::stdout();
        let mut writer = stdout.lock();
        match dispatcher.dispatch(&self.command(), &mut writer) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => Self::reported(&e),
        }
    }

    /// `--help` and `--version` are not failures; clap reports them the same
    /// way it reports a typo, and only `use_stderr` tells the two apart.
    fn parsing_failed(error: &ClapError) -> ExitCode {
        let _ = error.print();
        if error.use_stderr() {
            ExitCode::from(2)
        } else {
            ExitCode::SUCCESS
        }
    }

    /// The root to analyse: what `--path` said, or where the shell is.
    #[must_use]
    pub fn root(&self) -> PathBuf {
        self.path
            .clone()
            .unwrap_or_else(|| current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    }

    /// The command to run. Naming none means `report`.
    #[must_use]
    pub fn command(&self) -> Command {
        self.command.clone().unwrap_or(Command::Report)
    }

    /// What the flags override in the loaded configuration.
    ///
    /// The three booleans become `Some(true)` or `None` rather than
    /// `Some(false)`: a flag that was not passed has no opinion, and a config
    /// file that turned the behaviour on must not be switched off by its
    /// absence.
    #[must_use]
    pub fn overrides(&self) -> CliOverrides {
        CliOverrides {
            min_nodes: self.min_nodes,
            min_lines: self.min_lines,
            threshold: self.threshold,
            exclude: self.exclude.clone(),
            exclude_tests: self.exclude_tests.then_some(true),
            sub_function: self.sub_function.then_some(true),
            min_sub_nodes: self.min_sub_nodes,
            baseline: self.baseline.clone(),
        }
    }

    /// The output format asked for.
    #[must_use]
    pub const fn format(&self) -> OutputFormat {
        self.format
    }

    fn reported(error: &CliError) -> ExitCode {
        if !matches!(error, CliError::CheckFailed) {
            eprintln!("Error: {error}");
        }
        ExitCode::from(error.exit_code() as u8)
    }
}
