// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io;
use std::io::Write as _;
use std::process::ExitCode;

use clap::Error as ClapError;
use clap::Parser;
use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::command_outcome::CommandOutcome;
use dry4rust::cli::ports::Ports;
use dry4rust::command_dispatcher::CommandDispatcher;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use dry4rust::storage_failure::StorageFailure;

use crate::invocation::args::Args;
use crate::system::json_baseline_store::JsonBaselineStore;
use crate::system::portable_path::PortablePath;
use crate::system::toml_config_source::TomlConfigSource;
use crate::system::toml_ignore_store::TomlIgnoreStore;
use crate::system::walkdir_source_tree::WalkdirSourceTree;

/// The composition root: the command line in, the report out on stdout and
/// the warnings and errors on stderr, an exit code back. The one place the
/// file system, the formats the files are written in and the streams are
/// chosen; everything it composes is the core's, tested without them.
pub struct EntryPoint;

impl EntryPoint {
    /// Parse `args`, run what they ask for, and report the outcome.
    ///
    /// Taking the arguments rather than reading `argv` is what lets a test
    /// call this at all; `main` passes the real ones.
    #[must_use]
    pub fn run(args: Vec<String>) -> ExitCode {
        match Args::try_parse_from(args) {
            Ok(args) => Self::execute(&args),
            Err(error) => Self::parsing_failed(&error),
        }
    }

    fn execute(args: &Args) -> ExitCode {
        let analyzer = RustAnalyzer::new();
        let root = PortablePath::lossy(&args.root());
        let reporter = args.format().reporter(Some(&root));
        let sources = WalkdirSourceTree::new();
        let config = TomlConfigSource::new();
        let ignore = TomlIgnoreStore::new();
        let baseline = JsonBaselineStore::new();
        let ports = Ports {
            sources: &sources,
            config: &config,
            ignore: &ignore,
            baseline: &baseline,
        };
        let dispatcher =
            CommandDispatcher::new(&analyzer, &root, reporter.as_ref(), args.overrides(), ports);
        match dispatcher.dispatch(&args.command()) {
            Ok(outcome) => Self::printed(&outcome),
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

    fn printed(outcome: &CommandOutcome) -> ExitCode {
        for warning in outcome.warnings() {
            eprintln!("Warning: {warning}");
        }
        let stdout = io::stdout();
        let mut writer = stdout.lock();
        match writer
            .write_all(outcome.text().as_bytes())
            .and_then(|()| writer.flush())
        {
            Ok(()) => match outcome.result() {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => Self::reported(e),
            },
            Err(e) => Self::reported(&CliError::Io(StorageFailure::Failed(e.to_string()))),
        }
    }

    fn reported(error: &CliError) -> ExitCode {
        if !matches!(error, CliError::CheckFailed) {
            eprintln!("Error: {error}");
        }
        ExitCode::from(error.exit_code() as u8)
    }
}
