// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::fixture::Fixture;
use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::cli::command::Command;
use dry4rust::command_dispatcher::CommandDispatcher;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use dry4rust_cli::invocation::output_format::OutputFormat;
use dry4rust_cli::system::walkdir_source_tree::WalkdirSourceTree;
use std::path::Path;

pub struct DispatchScenario;

impl DispatchScenario {
    #[must_use]
    pub fn breaching_check() -> Command {
        Command::Check {
            max_exact: Some(0),
            max_near: None,
            max_exact_percent: None,
            max_near_percent: None,
        }
    }

    pub fn dispatch_in(root: &Path, command: &Command) -> (Result<(), CliError>, String) {
        let analyzer = RustAnalyzer::new();
        let root = Fixture::root_of(root);
        let reporter = OutputFormat::Text.reporter(Some(&root));
        let dispatcher = CommandDispatcher::new(
            &analyzer,
            &root,
            reporter.as_ref(),
            CliOverrides::default(),
            &WalkdirSourceTree,
        );
        match dispatcher.dispatch(command) {
            Ok(outcome) => outcome.into_parts(),
            Err(error) => (Err(error), String::new()),
        }
    }
}
