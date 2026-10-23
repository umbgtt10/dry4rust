// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;
use std::string::ToString;

use crate::analysis::Analysis;
use crate::analyzer::LanguageAnalyzer;
use crate::cli::cli_error::CliError;
use crate::cli::cli_error::CliResult;
use crate::cli::cli_overrides::CliOverrides;
use crate::cli::output_format::OutputFormat;
use crate::config::Config;
use crate::output::reporter::Reporter;
use crate::scanner::Scanner;

/// Everything one analysis run produces: the configuration it ran under, what
/// it found, and the reporter that can write it out.
pub struct AnalysisOutput {
    config: Config,
    result: Analysis,
    reporter: Box<dyn Reporter>,
}

impl AnalysisOutput {
    #[must_use]
    pub fn new(config: Config, result: Analysis, reporter: Box<dyn Reporter>) -> Self {
        Self {
            config,
            result,
            reporter,
        }
    }

    fn from_config(
        analyzer: &dyn LanguageAnalyzer,
        root: &Path,
        format: OutputFormat,
        config: Config,
    ) -> CliResult<Self> {
        let scanner = Scanner::new(config.root().to_path_buf())
            .with_excludes(config.exclude().to_vec())
            .with_extensions(
                analyzer
                    .file_extensions()
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
            );
        let files = scanner.scan_files();

        if files.is_empty() {
            return Err(CliError::NoSourceFiles(config.root().to_path_buf()));
        }

        let result = Analysis::analyze(analyzer, &files, &config)?;
        let reporter = format.reporter(Some(root));

        Ok(Self {
            config,
            result,
            reporter,
        })
    }

    /// Scan files, run the analysis pipeline, and return the output.
    ///
    /// Warnings are stored in [`AnalysisOutput::result`] but **not** printed;
    /// the caller is responsible for writing them to stderr.
    ///
    /// # Errors
    ///
    /// Returns [`CliError::InvalidConfig`] when the configuration states a
    /// value the tool cannot run under, [`CliError::NoSourceFiles`] when the
    /// scan finds nothing to analyse, and [`CliError::Analysis`] when the
    /// pipeline itself fails.
    pub fn produce(
        analyzer: &dyn LanguageAnalyzer,
        root: &Path,
        format: OutputFormat,
        overrides: &CliOverrides,
    ) -> CliResult<Self> {
        Self::from_config(analyzer, root, format, Self::configure(root, overrides)?)
    }

    /// The same run, with the baseline left out of it.
    ///
    /// What `baseline` records with: a recording that judged against the
    /// previous recording would hold only what had been added since, and the
    /// second run would empty the file.
    ///
    /// # Errors
    ///
    /// The same as [`AnalysisOutput::produce`], less the baseline.
    pub fn produce_ignoring_baseline(
        analyzer: &dyn LanguageAnalyzer,
        root: &Path,
        format: OutputFormat,
        overrides: &CliOverrides,
    ) -> CliResult<Self> {
        let config = Self::configure(root, overrides)?;
        let named = config.baseline().map(Path::to_path_buf);
        let output = Self::from_config(analyzer, root, format, config.with_baseline(None))?;
        Ok(Self {
            config: output.config.with_baseline(named),
            ..output
        })
    }

    #[must_use]
    pub const fn config(&self) -> &Config {
        &self.config
    }

    #[must_use]
    pub fn reporter(&self) -> &dyn Reporter {
        self.reporter.as_ref()
    }

    #[must_use]
    pub const fn result(&self) -> &Analysis {
        &self.result
    }

    fn configure(root: &Path, overrides: &CliOverrides) -> CliResult<Config> {
        Config::load(root)
            .and_then(|config| overrides.apply_to(config))
            .map_err(|e| CliError::InvalidConfig(e.to_string()))
    }
}
