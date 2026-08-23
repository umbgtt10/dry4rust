// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::config::Config;
use crate::error::Result;
use crate::threshold::Threshold;

/// Optional CLI overrides applied on top of file-based config.
#[derive(Debug, Clone, Default)]
pub struct CliOverrides {
    pub min_nodes: Option<usize>,
    pub min_lines: Option<usize>,
    pub threshold: Option<f64>,
    pub exclude: Vec<String>,
    pub exclude_tests: Option<bool>,
    pub sub_function: Option<bool>,
    pub min_sub_nodes: Option<usize>,
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
            config.min_nodes = min_nodes;
        }
        if let Some(min_lines) = self.min_lines {
            config.min_lines = min_lines;
        }
        if let Some(threshold) = self.threshold {
            config.similarity_threshold = Threshold::fraction("--threshold", threshold)?;
        }
        if !self.exclude.is_empty() {
            config.exclude.extend(self.exclude.iter().cloned());
        }
        if let Some(v) = self.exclude_tests {
            config.exclude_tests = v;
        }
        if let Some(v) = self.sub_function {
            config.sub_function = v;
        }
        if let Some(min_sub_nodes) = self.min_sub_nodes {
            config.min_sub_nodes = min_sub_nodes;
        }
        Ok(config)
    }
}
