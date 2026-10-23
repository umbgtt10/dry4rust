// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;
use std::path::{Path, PathBuf};

use toml::Value;
use toml::from_str;

use crate::analysis_config::AnalysisConfig;
use crate::error::Result;
use crate::file_config::FileConfig;
use crate::threshold::Threshold;

/// Configuration for cargo-dry4rust analysis.
///
/// The fields are public and settable, and the ones with a range carry it in
/// their type: a [`Threshold`] cannot be built out of range, so a `Config`
/// cannot hold a similarity threshold of five however it was assembled. The
/// counts are `usize` and have no upper bound to state -- a floor of zero
/// nodes admits everything, which is a choice rather than a mistake.
#[derive(Debug, Clone)]
pub struct Config {
    /// Minimum number of AST nodes for a code unit to be analyzed.
    min_nodes: usize,
    /// Similarity threshold for near-duplicates.
    similarity_threshold: Threshold,
    /// Path patterns to exclude from scanning.
    exclude: Vec<String>,
    /// Exit code threshold: fail if exact duplicate count exceeds this.
    max_exact_duplicates: Option<usize>,
    /// Exit code threshold: fail if near duplicate count exceeds this.
    max_near_duplicates: Option<usize>,
    /// Exit code threshold: fail if exact duplicate percentage exceeds this.
    max_exact_percent: Option<Threshold>,
    /// Exit code threshold: fail if near duplicate percentage exceeds this.
    max_near_percent: Option<Threshold>,
    /// Minimum number of source lines for a code unit to be analyzed.
    min_lines: usize,
    /// Exclude test code (#[test] functions and #[cfg(test)] modules).
    exclude_tests: bool,
    /// Enable sub-function duplicate detection.
    sub_function: bool,
    /// Minimum number of AST nodes for a sub-function unit to be analyzed.
    min_sub_nodes: usize,
    /// Baseline of inherited duplication to judge against, if any.
    baseline: Option<PathBuf>,
    /// Root path to analyze.
    root: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            min_nodes: 10,
            similarity_threshold: Threshold::DEFAULT_SIMILARITY,
            exclude: Vec::new(),
            max_exact_duplicates: None,
            max_near_duplicates: None,
            max_exact_percent: None,
            max_near_percent: None,
            min_lines: 0,
            exclude_tests: false,
            sub_function: false,
            min_sub_nodes: 5,
            baseline: None,
            root: PathBuf::from("."),
        }
    }
}

impl Config {
    /// Load config with the following precedence:
    /// 1. CLI overrides (applied by the caller after this method)
    /// 2. dry4rust.toml in the project root
    /// 3. `[package.metadata.dry4rust]` in Cargo.toml
    /// 4. Defaults
    ///
    /// A file that cannot be read or parsed is passed over, because a project
    /// with no configuration is the ordinary case and looks the same. A file
    /// that parses and then states an impossible value is not passed over:
    /// the caller asked for something the tool cannot do, and saying so is
    /// the only way they find out.
    ///
    /// # Errors
    ///
    /// Returns [`crate::error::Error::InvalidConfig`] naming the first field
    /// whose value falls outside the range it allows.
    pub fn load(root: &Path) -> Result<Self> {
        let mut config = Self {
            root: root.to_path_buf(),
            ..Default::default()
        };

        // Cargo.toml metadata first: the lowest-priority file config
        if let Some(file_config) = Self::from_cargo_metadata(root) {
            config = file_config.apply_to(config)?;
        }

        // dry4rust.toml wins over it
        if let Some(file_config) = Self::from_named_file(root) {
            config = file_config.apply_to(config)?;
        }

        Ok(config)
    }

    /// Extract the parsing-relevant subset of the configuration.
    #[must_use]
    pub const fn analysis_config(&self) -> AnalysisConfig {
        AnalysisConfig::new(self.min_nodes, self.min_lines)
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
    pub const fn exclude_tests(&self) -> bool {
        self.exclude_tests
    }

    #[must_use]
    pub const fn max_exact_duplicates(&self) -> Option<usize> {
        self.max_exact_duplicates
    }

    #[must_use]
    pub const fn max_exact_percent(&self) -> Option<Threshold> {
        self.max_exact_percent
    }

    #[must_use]
    pub const fn max_near_duplicates(&self) -> Option<usize> {
        self.max_near_duplicates
    }

    #[must_use]
    pub const fn max_near_percent(&self) -> Option<Threshold> {
        self.max_near_percent
    }

    #[must_use]
    pub const fn min_lines(&self) -> usize {
        self.min_lines
    }

    #[must_use]
    pub const fn min_nodes(&self) -> usize {
        self.min_nodes
    }

    #[must_use]
    pub const fn min_sub_nodes(&self) -> usize {
        self.min_sub_nodes
    }

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    #[must_use]
    pub const fn similarity_threshold(&self) -> Threshold {
        self.similarity_threshold
    }

    #[must_use]
    pub const fn sub_function(&self) -> bool {
        self.sub_function
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
    pub fn with_exclude_tests(self, exclude_tests: bool) -> Self {
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
    pub fn with_max_exact_percent(self, max_exact_percent: Option<Threshold>) -> Self {
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
    pub fn with_max_near_percent(self, max_near_percent: Option<Threshold>) -> Self {
        Self {
            max_near_percent,
            ..self
        }
    }

    #[must_use]
    pub fn with_min_lines(self, min_lines: usize) -> Self {
        Self { min_lines, ..self }
    }

    #[must_use]
    pub fn with_min_nodes(self, min_nodes: usize) -> Self {
        Self { min_nodes, ..self }
    }

    #[must_use]
    pub fn with_min_sub_nodes(self, min_sub_nodes: usize) -> Self {
        Self {
            min_sub_nodes,
            ..self
        }
    }

    #[must_use]
    pub fn with_root(self, root: PathBuf) -> Self {
        Self { root, ..self }
    }

    #[must_use]
    pub fn with_similarity_threshold(self, similarity_threshold: Threshold) -> Self {
        Self {
            similarity_threshold,
            ..self
        }
    }

    #[must_use]
    pub fn with_sub_function(self, sub_function: bool) -> Self {
        Self {
            sub_function,
            ..self
        }
    }

    fn from_cargo_metadata(root: &Path) -> Option<FileConfig> {
        let content = fs::read_to_string(root.join("Cargo.toml")).ok()?;
        from_str::<Value>(&content)
            .ok()?
            .get("package")?
            .get("metadata")?
            .get("dry4rust")?
            .clone()
            .try_into()
            .ok()
    }

    fn from_named_file(root: &Path) -> Option<FileConfig> {
        let content = fs::read_to_string(root.join("dry4rust.toml")).ok()?;
        from_str(&content).ok()
    }
}
