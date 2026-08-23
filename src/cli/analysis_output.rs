// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;
use std::string::ToString;

use crate::analysis::AnalysisResult;
use crate::analysis::analyze;
use crate::analyzer::LanguageAnalyzer;
use crate::cli::cli_error::CliError;
use crate::cli::cli_error::CliResult;
use crate::cli::cli_overrides::CliOverrides;
use crate::cli::output_format::OutputFormat;
use crate::config::Config;
use crate::output::reporter::Reporter;
use crate::scanner::ScanConfig;
use crate::scanner::scan_files;

/// Everything one analysis run produces: the configuration it ran under, what
/// it found, and the reporter that can write it out.
pub struct AnalysisOutput {
    pub config: Config,
    pub result: AnalysisResult,
    pub reporter: Box<dyn Reporter>,
}

impl AnalysisOutput {
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
        let config = Config::load(root)
            .and_then(|config| overrides.apply_to(config))
            .map_err(|e| CliError::InvalidConfig(e.to_string()))?;

        let scan_config = ScanConfig::new(config.root.clone())
            .with_excludes(config.exclude.clone())
            .with_extensions(
                analyzer
                    .file_extensions()
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
            );
        let files = scan_files(&scan_config);

        if files.is_empty() {
            return Err(CliError::NoSourceFiles(config.root));
        }

        let result = analyze(analyzer, &files, &config)?;
        let reporter = format.reporter(Some(root));

        Ok(Self {
            config,
            result,
            reporter,
        })
    }
}
