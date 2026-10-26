// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec::Vec;

use crate::config::Config;
use crate::error::Result;
use crate::source_path::SourcePath;
use crate::threshold::Threshold;

/// Configuration as `dry4rust.toml` and `[package.metadata.dry4rust]` spell it.
///
/// Every field is optional, because a file states what it wants changed and
/// says nothing about the rest. Turning that into a [`Config`] is where the
/// ranges are checked, so a value that cannot be honoured is refused at the
/// point it is read rather than carried further. Reading the file is a
/// [`crate::traits::config_source::ConfigSource`]'s; it builds one of these
/// with a step per field it found.
#[derive(Debug, Clone, Default)]
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
    baseline: Option<SourcePath>,
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
            config = config.with_min_nodes(v);
        }
        if let Some(v) = self.similarity_threshold {
            config =
                config.with_similarity_threshold(Threshold::fraction("similarity_threshold", v)?);
        }
        if let Some(ref v) = self.exclude {
            config = config.with_exclude(v.clone());
        }
        if let Some(v) = self.max_exact_duplicates {
            config = config.with_max_exact_duplicates(Some(v));
        }
        if let Some(v) = self.max_near_duplicates {
            config = config.with_max_near_duplicates(Some(v));
        }
        if let Some(v) = self.max_exact_percent {
            config =
                config.with_max_exact_percent(Some(Threshold::percent("max_exact_percent", v)?));
        }
        if let Some(v) = self.max_near_percent {
            config = config.with_max_near_percent(Some(Threshold::percent("max_near_percent", v)?));
        }
        if let Some(v) = self.min_lines {
            config = config.with_min_lines(v);
        }
        if let Some(v) = self.exclude_tests {
            config = config.with_exclude_tests(v);
        }
        if let Some(v) = self.sub_function {
            config = config.with_sub_function(v);
        }
        if let Some(v) = self.min_sub_nodes {
            config = config.with_min_sub_nodes(v);
        }
        if let Some(ref v) = self.baseline {
            config = config.with_baseline(Some(v.clone()));
        }
        Ok(config)
    }

    #[must_use]
    pub fn with_baseline(self, baseline: Option<SourcePath>) -> Self {
        Self { baseline, ..self }
    }

    #[must_use]
    pub fn with_exclude(self, exclude: Option<Vec<String>>) -> Self {
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
    pub fn with_max_exact_duplicates(self, max_exact_duplicates: Option<usize>) -> Self {
        Self {
            max_exact_duplicates,
            ..self
        }
    }

    #[must_use]
    pub fn with_max_exact_percent(self, max_exact_percent: Option<f64>) -> Self {
        Self {
            max_exact_percent,
            ..self
        }
    }

    #[must_use]
    pub fn with_max_near_duplicates(self, max_near_duplicates: Option<usize>) -> Self {
        Self {
            max_near_duplicates,
            ..self
        }
    }

    #[must_use]
    pub fn with_max_near_percent(self, max_near_percent: Option<f64>) -> Self {
        Self {
            max_near_percent,
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
    pub fn with_similarity_threshold(self, similarity_threshold: Option<f64>) -> Self {
        Self {
            similarity_threshold,
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
}
