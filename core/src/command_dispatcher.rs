// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io::Write;
use std::path::Path;

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
use crate::cli::ignore_command::IgnoreCommand;
use crate::cli::ignored_command::IgnoredCommand;
use crate::cli::output_format::OutputFormat;
use crate::cli::report_command::ReportCommand;
use crate::cli::stats_command::StatsCommand;
use crate::output::reporter::Reporter;

/// Routes a parsed [`Command`] to the type that serves it.
///
/// Two commands read and write the ignore file without analysing anything;
/// the other four need an analysis first. Splitting on that distinction is
/// the whole of the routing decision, and keeping it here rather than in
/// `main` puts it where a test can reach it.
pub struct CommandDispatcher<'a> {
    analyzer: &'a dyn LanguageAnalyzer,
    root: &'a Path,
    format: OutputFormat,
    overrides: CliOverrides,
}

impl<'a> CommandDispatcher<'a> {
    #[must_use]
    pub const fn new(
        analyzer: &'a dyn LanguageAnalyzer,
        root: &'a Path,
        format: OutputFormat,
        overrides: CliOverrides,
    ) -> Self {
        Self {
            analyzer,
            root,
            format,
            overrides,
        }
    }

    /// Run `command`, writing whatever it produces to `writer`.
    ///
    /// # Errors
    ///
    /// Returns whatever the command returns: a failed analysis, an I/O
    /// failure, or [`crate::cli::cli_error::CliError::CheckFailed`] when
    /// `check` finds more duplication than its ceilings allow.
    pub fn dispatch(&self, command: &Command, writer: &mut impl Write) -> CliResult {
        match command {
            Command::Ignore {
                fingerprint,
                reason,
            } => IgnoreCommand::new(self.root, fingerprint, reason.as_deref()).run(writer),
            Command::Ignored => IgnoredCommand::new(self.root).run(writer),
            _ => self.dispatch_analysed(command, writer),
        }
    }

    fn dispatch_analysed(&self, command: &Command, writer: &mut impl Write) -> CliResult {
        let output = if matches!(command, Command::Baseline { .. }) {
            AnalysisOutput::produce_ignoring_baseline(
                self.analyzer,
                self.root,
                self.format,
                &self.overrides,
            )?
        } else {
            AnalysisOutput::produce(self.analyzer, self.root, self.format, &self.overrides)?
        };
        Self::report_warnings(&output);
        self.render(command, &output, writer)
    }

    fn report_warnings(output: &AnalysisOutput) {
        for warning in &output.result.warnings {
            eprintln!("Warning: {warning}");
        }
    }

    fn render(
        &self,
        command: &Command,
        output: &AnalysisOutput,
        writer: &mut impl Write,
    ) -> CliResult {
        let reporter: &dyn Reporter = &*output.reporter;
        match command {
            Command::Stats => StatsCommand::new(&output.result, reporter).run(writer),
            Command::Report => ReportCommand::new(&output.result, reporter).run(writer),
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
                CheckCommand::new(&output.config, &output.result, reporter, &thresholds).run(writer)
            }
            Command::Cleanup { dry_run } => {
                CleanupCommand::new(self.root, &output.result, *dry_run).run(writer)
            }
            Command::Baseline { dry_run } => {
                BaselineCommand::new(self.root, &output.config, &output.result, *dry_run)
                    .run(writer)
            }
            Command::Ignore { .. } | Command::Ignored => unreachable!(),
        }
    }
}
