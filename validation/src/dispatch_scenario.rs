// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::cli::command::Command;
use dry4rust::cli::output_format::OutputFormat;
use dry4rust::command_dispatcher::CommandDispatcher;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
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
        let dispatcher =
            CommandDispatcher::new(&analyzer, root, OutputFormat::Text, CliOverrides::default());
        match dispatcher.dispatch(command) {
            Ok(outcome) => (outcome.result, outcome.text),
            Err(error) => (Err(error), String::new()),
        }
    }
}
