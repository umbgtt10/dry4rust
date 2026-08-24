// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use serde::Deserialize;

use crate::config::Config;
use crate::error::Result;
use crate::threshold::Threshold;

/// Configuration as `dry4rust.toml` and `[package.metadata.dry4rust]` spell it.
///
/// Every field is optional, because a file states what it wants changed and
/// says nothing about the rest. Turning that into a [`Config`] is where the
/// ranges are checked, so a value that cannot be honoured is refused at the
/// point it is read rather than carried further.
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct FileConfig {
    min_nodes: Option<usize>,
    similarity_threshold: Option<f64>,
    exclude: Option<Vec<String>>,
    max_exact_duplicates: Option<usize>,
    max_near_duplicates: Option<usize>,
    max_exact_percent: Option<f64>,
    max_near_percent: Option<f64>,
    min_lines: Option<usize>,
    exclude_tests: Option<bool>,
    sub_function: Option<bool>,
    min_sub_nodes: Option<usize>,
    baseline: Option<PathBuf>,
}

impl FileConfig {
    /// Apply what this file states to `config`, leaving the rest alone.
    ///
    /// # Errors
    ///
    /// Returns [`crate::error::Error::InvalidConfig`] naming the first field
    /// whose value falls outside the range it allows.
    pub fn apply_to(&self, config: Config) -> Result<Config> {
        let mut config = config;
        if let Some(v) = self.min_nodes {
            config.min_nodes = v;
        }
        if let Some(v) = self.similarity_threshold {
            config.similarity_threshold = Threshold::fraction("similarity_threshold", v)?;
        }
        if let Some(ref v) = self.exclude {
            config.exclude.clone_from(v);
        }
        if let Some(v) = self.max_exact_duplicates {
            config.max_exact_duplicates = Some(v);
        }
        if let Some(v) = self.max_near_duplicates {
            config.max_near_duplicates = Some(v);
        }
        if let Some(v) = self.max_exact_percent {
            config.max_exact_percent = Some(Threshold::percent("max_exact_percent", v)?);
        }
        if let Some(v) = self.max_near_percent {
            config.max_near_percent = Some(Threshold::percent("max_near_percent", v)?);
        }
        if let Some(v) = self.min_lines {
            config.min_lines = v;
        }
        if let Some(v) = self.exclude_tests {
            config.exclude_tests = v;
        }
        if let Some(v) = self.sub_function {
            config.sub_function = v;
        }
        if let Some(v) = self.min_sub_nodes {
            config.min_sub_nodes = v;
        }
        if let Some(ref v) = self.baseline {
            config.baseline = Some(v.clone());
        }
        Ok(config)
    }
}
