// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env;
use std::io;
use std::path::PathBuf;
use std::process;

use clap::Parser;

use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::cli::command::Command;
use dry4rust::cli::output_format::OutputFormat;
use dry4rust::command_dispatcher::CommandDispatcher;
use dry4rust::rust::rust_analyzer::RustAnalyzer;

#[derive(Parser)]
#[command(
    name = "cargo-dry4rust",
    version,
    about = "Detect duplicate code in Rust codebases"
)]
struct Cli {
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
    let overrides = CliOverrides {
        min_nodes,
        min_lines,
        threshold,
        exclude,
        exclude_tests: if exclude_tests { Some(true) } else { None },
        sub_function: if sub_function { Some(true) } else { None },
        min_sub_nodes,
    };

    let analyzer = RustAnalyzer::new();
    let dispatcher = CommandDispatcher::new(&analyzer, &root, format, overrides);
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    if let Err(e) = dispatcher.dispatch(&command, &mut writer) {
        if matches!(e, CliError::CheckFailed) {
            process::exit(1);
        }
        eprintln!("Error: {e}");
        process::exit(e.exit_code());
    }
}
