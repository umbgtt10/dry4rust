// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;
use std::path::PathBuf;

use crate::config::Config;
use crate::error::Result;
use crate::threshold::Threshold;

/// Optional CLI overrides applied on top of file-based config.
#[derive(Debug, Clone, Default)]
pub struct CliOverrides {
    min_nodes: Option<usize>,
    min_lines: Option<usize>,
    threshold: Option<f64>,
    exclude: Vec<String>,
    exclude_tests: Option<bool>,
    sub_function: Option<bool>,
    min_sub_nodes: Option<usize>,
    baseline: Option<PathBuf>,
}

impl CliOverrides {
    /// Apply these overrides to a loaded `Config`, returning the result.
    ///
    /// An absent override leaves the loaded value alone; `exclude` is the one
    /// exception, and is *appended* to the config-file excludes rather than
    /// replacing them.
    ///
    /// # Errors
    ///
    /// Returns [`crate::error::Error::InvalidConfig`] when `--threshold` is
    /// outside the range a similarity score can occupy.
    pub fn apply_to(&self, config: Config) -> Result<Config> {
        let mut config = config;
        if let Some(min_nodes) = self.min_nodes {
            config = config.with_min_nodes(min_nodes);
        }
        if let Some(min_lines) = self.min_lines {
            config = config.with_min_lines(min_lines);
        }
        if let Some(threshold) = self.threshold {
            config =
                config.with_similarity_threshold(Threshold::fraction("--threshold", threshold)?);
        }
        if !self.exclude.is_empty() {
            let mut exclude = config.exclude().to_vec();
            exclude.extend(self.exclude.iter().cloned());
            config = config.with_exclude(exclude);
        }
        if let Some(v) = self.exclude_tests {
            config = config.with_exclude_tests(v);
        }
        if let Some(v) = self.sub_function {
            config = config.with_sub_function(v);
        }
        if let Some(min_sub_nodes) = self.min_sub_nodes {
            config = config.with_min_sub_nodes(min_sub_nodes);
        }
        if let Some(ref baseline) = self.baseline {
            config = config.with_baseline(Some(baseline.clone()));
        }
        Ok(config)
    }

    #[must_use]
    pub fn baseline(&self) -> Option<&Path> {
        self.baseline.as_deref()
    }

    #[must_use]
    pub fn exclude(&self) -> &[String] {
        &self.exclude
    }

    #[must_use]
    pub const fn exclude_tests(&self) -> Option<bool> {
        self.exclude_tests
    }

    #[must_use]
    pub const fn min_lines(&self) -> Option<usize> {
        self.min_lines
    }

    #[must_use]
    pub const fn min_nodes(&self) -> Option<usize> {
        self.min_nodes
    }

    #[must_use]
    pub const fn min_sub_nodes(&self) -> Option<usize> {
        self.min_sub_nodes
    }

    #[must_use]
    pub const fn sub_function(&self) -> Option<bool> {
        self.sub_function
    }

    #[must_use]
    pub const fn threshold(&self) -> Option<f64> {
        self.threshold
    }

    #[must_use]
    pub fn with_baseline(self, baseline: Option<PathBuf>) -> Self {
        Self { baseline, ..self }
    }

    #[must_use]
    pub fn with_exclude(self, exclude: Vec<String>) -> Self {
        Self { exclude, ..self }
    }

    #[must_use]
    pub fn with_exclude_tests(self, exclude_tests: Option<bool>) -> Self {
        Self {
            exclude_tests,
            ..self
        }
    }

    #[must_use]
    pub fn with_min_lines(self, min_lines: Option<usize>) -> Self {
        Self { min_lines, ..self }
    }

    #[must_use]
    pub fn with_min_nodes(self, min_nodes: Option<usize>) -> Self {
        Self { min_nodes, ..self }
    }

    #[must_use]
    pub fn with_min_sub_nodes(self, min_sub_nodes: Option<usize>) -> Self {
        Self {
            min_sub_nodes,
            ..self
        }
    }

    #[must_use]
    pub fn with_sub_function(self, sub_function: Option<bool>) -> Self {
        Self {
            sub_function,
            ..self
        }
    }

    #[must_use]
    pub fn with_threshold(self, threshold: Option<f64>) -> Self {
        Self { threshold, ..self }
    }
}
