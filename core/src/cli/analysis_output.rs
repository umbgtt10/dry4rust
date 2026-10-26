// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::analysis::Analysis;
use crate::analyzer::LanguageAnalyzer;
use crate::cli::cli_error::CliError;
use crate::cli::cli_error::CliResult;
use crate::cli::cli_overrides::CliOverrides;
use crate::cli::ports::Ports;
use crate::cli::suppression::baseline_filter::BaselineFilter;
use crate::cli::suppression::ignore_file::IgnoreFile;
use crate::config::Config;
use crate::error::Error;
use crate::scanner::Scanner;
use crate::source_path::SourcePath;

/// Everything one analysis run produces: the configuration it ran under and
/// what it found.
pub struct AnalysisOutput {
    config: Config,
    result: Analysis,
}

impl AnalysisOutput {
    #[must_use]
    pub const fn new(config: Config, result: Analysis) -> Self {
        Self { config, result }
    }

    fn from_config(
        analyzer: &dyn LanguageAnalyzer,
        ports: &Ports<'_>,
        config: Config,
    ) -> CliResult<Self> {
        let scanner = Scanner::new(config.root().clone())
            .with_excludes(config.exclude().to_vec())
            .with_extensions(
                analyzer
                    .file_extensions()
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
            );
        let files: Vec<SourcePath> =
            scanner
                .scan_files(ports.sources)
                .map_err(|failure| Error::Read {
                    path: config.root().clone(),
                    reason: failure.to_string(),
                })?;

        if files.is_empty() {
            return Err(CliError::NoSourceFiles(config.root().clone()));
        }

        let ignore_file = IgnoreFile::load(ports.ignore, config.root());
        let baseline = BaselineFilter::load(ports.baseline, &config)?;
        let result = Analysis::analyze(
            analyzer,
            ports.sources,
            &files,
            &config,
            &ignore_file,
            &baseline,
        );

        Ok(Self { config, result })
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
    /// sources cannot be listed or the baseline cannot be read.
    pub fn produce(
        analyzer: &dyn LanguageAnalyzer,
        ports: &Ports<'_>,
        root: &SourcePath,
        overrides: &CliOverrides,
    ) -> CliResult<Self> {
        Self::from_config(analyzer, ports, Self::configure(ports, root, overrides)?)
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
        ports: &Ports<'_>,
        root: &SourcePath,
        overrides: &CliOverrides,
    ) -> CliResult<Self> {
        let config = Self::configure(ports, root, overrides)?;
        let named = config.baseline().cloned();
        let output = Self::from_config(analyzer, ports, config.with_baseline(None))?;
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
    pub const fn result(&self) -> &Analysis {
        &self.result
    }

    fn configure(
        ports: &Ports<'_>,
        root: &SourcePath,
        overrides: &CliOverrides,
    ) -> CliResult<Config> {
        Config::load(ports.config, root)
            .and_then(|config| overrides.apply_to(config))
            .map_err(|e| CliError::InvalidConfig(e.to_string()))
    }
}
