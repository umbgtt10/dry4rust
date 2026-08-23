// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;
use std::process;

use clap::Parser;

use cli::cmd_check;
use cli::cmd_cleanup;
use cli::cmd_ignore;
use cli::cmd_ignored;
use cli::cmd_report;
use cli::cmd_stats;
use cli::run_analysis;
use dry4rust::cli::{self, CliOverrides, Command, OutputFormat};
use dry4rust::rust::RustAnalyzer;
use std::env;
use std::io;

#[derive(Parser)]
#[command(
    name = "cargo-dupes",
    version,
    about = "Detect duplicate code in Rust codebases"
)]
struct Cli {
    /// When invoked as `cargo dupes`, cargo passes "dupes" as the first arg.
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
}

fn main() {
    let Cli {
        command,
        path,
        min_nodes,
        min_lines,
        threshold,
        format,
        exclude,
        exclude_tests,
        sub_function,
        min_sub_nodes,
        ..
    } = Cli::parse();

    let root = path.unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let command = command.unwrap_or(Command::Report);
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    let result = match &command {
        Command::Ignore {
            fingerprint,
            reason,
        } => cmd_ignore(&root, fingerprint, reason.clone(), &mut writer),
        Command::Ignored => cmd_ignored(&root, &mut writer),
        _ => {
            let analyzer = RustAnalyzer::new();
            let overrides = CliOverrides {
                min_nodes,
                min_lines,
                threshold,
                exclude,
                exclude_tests: if exclude_tests { Some(true) } else { None },
                sub_function: if sub_function { Some(true) } else { None },
                min_sub_nodes,
            };
            let output = match run_analysis(&analyzer, &root, format, &overrides) {
                Ok(o) => o,
                Err(e) => {
                    eprintln!("Error: {e}");
                    process::exit(e.exit_code());
                }
            };

            for warning in &output.result.warnings {
                eprintln!("Warning: {warning}");
            }

            let reporter: &dyn dry4rust::output::Reporter = &*output.reporter;

            match &command {
                Command::Stats => cmd_stats(&output.result, reporter, &mut writer),
                Command::Report => cmd_report(&output.result, reporter, &mut writer),
                Command::Check {
                    max_exact,
                    max_near,
                    max_exact_percent,
                    max_near_percent,
                } => cmd_check(
                    &output.config,
                    &output.result,
                    reporter,
                    &mut writer,
                    &cli::CheckThresholds {
                        max_exact: *max_exact,
                        max_near: *max_near,
                        max_exact_percent: *max_exact_percent,
                        max_near_percent: *max_near_percent,
                    },
                ),
                Command::Cleanup { dry_run } => {
                    cmd_cleanup(&root, &output.result, &mut writer, *dry_run)
                }
                Command::Ignore { .. } | Command::Ignored => unreachable!(),
            }
        }
    };

    if let Err(e) = result {
        if matches!(e, cli::CliError::CheckFailed) {
            process::exit(1);
        } else {
            eprintln!("Error: {e}");
            process::exit(e.exit_code());
        }
    }
}
