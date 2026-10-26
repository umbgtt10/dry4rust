// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

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
use crate::cli::output::reporter::Reporter;
use crate::cli::report_command::ReportCommand;
use crate::cli::stats_command::StatsCommand;
use crate::source_path::SourcePath;
use crate::traits::source_tree::SourceTree;

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
    sources: &'a dyn SourceTree,
}

impl<'a> CommandDispatcher<'a> {
    #[must_use]
    pub const fn new(
        analyzer: &'a dyn LanguageAnalyzer,
        root: &'a SourcePath,
        reporter: &'a dyn Reporter,
        overrides: CliOverrides,
        sources: &'a dyn SourceTree,
    ) -> Self {
        Self {
            analyzer,
            root,
            reporter,
            overrides,
            sources,
        }
    }

    /// Run `command` and return the text it produced and its verdict.
    ///
    /// # Errors
    ///
    /// Returns whatever the command returns before it can produce text: a
    /// failed analysis, an I/O failure, or an invalid fingerprint. A breached
    /// `check` ceiling is carried in the outcome rather than returned here.
    pub fn dispatch(&self, command: &Command) -> CliResult<CommandOutcome> {
        match command {
            Command::Ignore {
                fingerprint,
                reason,
            } => IgnoreCommand::new(self.root, fingerprint, reason.as_deref()).run(),
            Command::Ignored => IgnoredCommand::new(self.root).run(),
            _ => self.dispatch_analysed(command),
        }
    }

    fn dispatch_analysed(&self, command: &Command) -> CliResult<CommandOutcome> {
        let output = if matches!(command, Command::Baseline { .. }) {
            AnalysisOutput::produce_ignoring_baseline(
                self.analyzer,
                self.sources,
                self.root,
                &self.overrides,
            )?
        } else {
            AnalysisOutput::produce(self.analyzer, self.sources, self.root, &self.overrides)?
        };
        Self::report_warnings(&output);
        self.render(command, &output)
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
                CleanupCommand::new(self.root, output.result(), *dry_run).run()
            }
            Command::Baseline { dry_run } => {
                BaselineCommand::new(self.root, output.config(), output.result(), *dry_run).run()
            }
            Command::Ignore { .. } | Command::Ignored => unreachable!(),
        }
    }

    fn report_warnings(output: &AnalysisOutput) {
        for warning in output.result().warnings() {
            eprintln!("Warning: {warning}");
        }
    }
}
