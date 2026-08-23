// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io::Write;
use std::path::Path;

use crate::analyzer::LanguageAnalyzer;
use crate::cli::AnalysisOutput;
use crate::cli::CheckThresholds;
use crate::cli::CliOverrides;
use crate::cli::CliResult;
use crate::cli::Command;
use crate::cli::OutputFormat;
use crate::cli::cmd_check;
use crate::cli::cmd_cleanup;
use crate::cli::cmd_ignore;
use crate::cli::cmd_ignored;
use crate::cli::cmd_report;
use crate::cli::cmd_stats;
use crate::cli::run_analysis;
use crate::output::reporter::Reporter;

/// Routes a parsed [`Command`] to the function that serves it.
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
    /// failure, or [`crate::cli::CliError::CheckFailed`] when `check` finds
    /// more duplication than its ceilings allow.
    pub fn dispatch(&self, command: &Command, writer: &mut impl Write) -> CliResult {
        match command {
            Command::Ignore {
                fingerprint,
                reason,
            } => cmd_ignore(self.root, fingerprint, reason.clone(), writer),
            Command::Ignored => cmd_ignored(self.root, writer),
            _ => self.dispatch_analysed(command, writer),
        }
    }

    fn dispatch_analysed(&self, command: &Command, writer: &mut impl Write) -> CliResult {
        let output = run_analysis(self.analyzer, self.root, self.format, &self.overrides)?;
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
            Command::Stats => cmd_stats(&output.result, reporter, writer),
            Command::Report => cmd_report(&output.result, reporter, writer),
            Command::Check {
                max_exact,
                max_near,
                max_exact_percent,
                max_near_percent,
            } => cmd_check(
                &output.config,
                &output.result,
                reporter,
                writer,
                &CheckThresholds {
                    max_exact: *max_exact,
                    max_near: *max_near,
                    max_exact_percent: *max_exact_percent,
                    max_near_percent: *max_near_percent,
                },
            ),
            Command::Cleanup { dry_run } => {
                cmd_cleanup(self.root, &output.result, writer, *dry_run)
            }
            Command::Ignore { .. } | Command::Ignored => unreachable!(),
        }
    }
}
