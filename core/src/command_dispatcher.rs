// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::string::ToString;

use crate::analyzer::LanguageAnalyzer;
use crate::cli::analysis_output::AnalysisOutput;
use crate::cli::baseline_command::BaselineCommand;
use crate::cli::check_command::CheckCommand;
use crate::cli::checking::check_thresholds::CheckThresholds;
use crate::cli::cleanup_command::CleanupCommand;
use crate::cli::cli_error::CliError;
use crate::cli::cli_error::CliResult;
use crate::cli::cli_overrides::CliOverrides;
use crate::cli::command::Command;
use crate::cli::command_outcome::CommandOutcome;
use crate::cli::ignore_command::IgnoreCommand;
use crate::cli::ignored_command::IgnoredCommand;
use crate::cli::ports::Ports;
use crate::cli::report_command::ReportCommand;
use crate::cli::stats_command::StatsCommand;
use crate::source_path::SourcePath;
use crate::traits::reporter::Reporter;

/// Routes a parsed [`Command`] to the type that serves it.
///
/// Two commands read and write the ignore file without analysing anything;
/// the other four need an analysis first. Splitting on that distinction is
/// the whole of the routing decision, and keeping it here rather than in
/// `main` puts it where a test can reach it.
pub struct CommandDispatcher<'a> {
    analyzer: &'a dyn LanguageAnalyzer,
    root: &'a SourcePath,
    reporter: &'a dyn Reporter,
    overrides: CliOverrides,
    ports: Ports<'a>,
}

impl<'a> CommandDispatcher<'a> {
    #[must_use]
    pub const fn new(
        analyzer: &'a dyn LanguageAnalyzer,
        root: &'a SourcePath,
        reporter: &'a dyn Reporter,
        overrides: CliOverrides,
        ports: Ports<'a>,
    ) -> Self {
        Self {
            analyzer,
            root,
            reporter,
            overrides,
            ports,
        }
    }

    /// Run `command` and return the text it produced, its verdict and the
    /// warnings the analysis raised.
    ///
    /// # Errors
    ///
    /// Returns whatever stops a command before an analysis has run: a
    /// configuration out of range, no sources, a baseline that cannot be read,
    /// an ignore file that cannot be written, or an invalid fingerprint. Once
    /// an analysis has run, its warnings and any failure -- a breached `check`
    /// ceiling included -- are carried in the outcome rather than returned
    /// here, so the warnings still reach stderr ahead of the error.
    pub fn dispatch(&self, command: &Command) -> CliResult<CommandOutcome> {
        match command {
            Command::Ignore {
                fingerprint,
                reason,
            } => IgnoreCommand::new(self.ports.ignore, self.root, fingerprint, reason.as_deref())
                .run(),
            Command::Ignored => IgnoredCommand::new(self.ports.ignore, self.root).run(),
            _ => self.dispatch_analysed(command),
        }
    }

    fn dispatch_analysed(&self, command: &Command) -> CliResult<CommandOutcome> {
        let output = if matches!(command, Command::Baseline { .. }) {
            AnalysisOutput::produce_ignoring_baseline(
                self.analyzer,
                &self.ports,
                self.root,
                &self.overrides,
            )?
        } else {
            AnalysisOutput::produce(self.analyzer, &self.ports, self.root, &self.overrides)?
        };
        let outcome = self
            .render(command, &output)
            .unwrap_or_else(|error| CommandOutcome::new(String::new(), Err(error)));
        Ok(outcome.with_warnings(output.result().warnings().to_vec()))
    }

    fn render(&self, command: &Command, output: &AnalysisOutput) -> CliResult<CommandOutcome> {
        let reporter = self.reporter;
        match command {
            Command::Stats => StatsCommand::new(output.result(), reporter).run(),
            Command::Report => ReportCommand::new(output.result(), reporter).run(),
            Command::Check {
                max_exact,
                max_near,
                max_exact_percent,
                max_near_percent,
            } => {
                let thresholds = CheckThresholds::new(
                    *max_exact,
                    *max_near,
                    *max_exact_percent,
                    *max_near_percent,
                )
                .map_err(|e| CliError::InvalidConfig(e.to_string()))?;
                CheckCommand::new(output.config(), output.result(), reporter, &thresholds).run()
            }
            Command::Cleanup { dry_run } => {
                CleanupCommand::new(self.ports.ignore, self.root, output.result(), *dry_run).run()
            }
            Command::Baseline { dry_run } => BaselineCommand::new(
                self.ports.baseline,
                self.root,
                output.config(),
                output.result(),
                *dry_run,
            )
            .run(),
            Command::Ignore { .. } | Command::Ignored => unreachable!(),
        }
    }
}
