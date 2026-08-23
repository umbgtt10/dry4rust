// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::{Path, PathBuf};

use serde::Deserialize;
use std::fs;
use toml::from_str;

use crate::error::Result;
use crate::threshold::Threshold;

/// The subset of configuration relevant to language-specific parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisConfig {
    /// Minimum number of AST nodes for a code unit to be analyzed.
    pub min_nodes: usize,
    /// Minimum number of source lines for a code unit to be analyzed.
    pub min_lines: usize,
}

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
    pub min_nodes: usize,
    /// Similarity threshold for near-duplicates.
    pub similarity_threshold: Threshold,
    /// Path patterns to exclude from scanning.
    pub exclude: Vec<String>,
    /// Exit code threshold: fail if exact duplicate count exceeds this.
    pub max_exact_duplicates: Option<usize>,
    /// Exit code threshold: fail if near duplicate count exceeds this.
    pub max_near_duplicates: Option<usize>,
    /// Exit code threshold: fail if exact duplicate percentage exceeds this.
    pub max_exact_percent: Option<Threshold>,
    /// Exit code threshold: fail if near duplicate percentage exceeds this.
    pub max_near_percent: Option<Threshold>,
    /// Minimum number of source lines for a code unit to be analyzed.
    pub min_lines: usize,
    /// Exclude test code (#[test] functions and #[cfg(test)] modules).
    pub exclude_tests: bool,
    /// Enable sub-function duplicate detection.
    pub sub_function: bool,
    /// Minimum number of AST nodes for a sub-function unit to be analyzed.
    pub min_sub_nodes: usize,
    /// Baseline of inherited duplication to judge against, if any.
    pub baseline: Option<PathBuf>,
    /// Root path to analyze.
    pub root: PathBuf,
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

/// Config as stored in dry4rust.toml or Cargo.toml metadata.
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct FileConfig {
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

/// Cargo.toml metadata section.
#[derive(Debug, Deserialize)]
struct CargoMetadata {
    #[serde(default)]
    package: Option<CargoPackage>,
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    #[serde(default)]
    metadata: Option<CargoPackageMetadata>,
}

#[derive(Debug, Deserialize)]
struct CargoPackageMetadata {
    #[serde(default)]
    dry4rust: Option<FileConfig>,
}

impl Config {
    /// Extract the parsing-relevant subset of the configuration.
    #[must_use]
    pub const fn analysis_config(&self) -> AnalysisConfig {
        AnalysisConfig {
            min_nodes: self.min_nodes,
            min_lines: self.min_lines,
        }
    }

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

        // Try Cargo.toml metadata first (lowest priority file config)
        let cargo_toml = root.join("Cargo.toml");
        if cargo_toml.exists()
            && let Ok(content) = fs::read_to_string(&cargo_toml)
            && let Ok(cargo) = from_str::<CargoMetadata>(&content)
            && let Some(pkg) = cargo.package
            && let Some(meta) = pkg.metadata
            && let Some(file_config) = meta.dry4rust
        {
            config = config.with_file_config(&file_config)?;
        }

        // Try dry4rust.toml (higher priority)
        let dry4rust_toml = root.join("dry4rust.toml");
        if dry4rust_toml.exists()
            && let Ok(content) = fs::read_to_string(&dry4rust_toml)
            && let Ok(file_config) = from_str::<FileConfig>(&content)
        {
            config = config.with_file_config(&file_config)?;
        }

        Ok(config)
    }

    fn with_file_config(self, fc: &FileConfig) -> Result<Self> {
        let mut config = self;
        if let Some(v) = fc.min_nodes {
            config.min_nodes = v;
        }
        if let Some(v) = fc.similarity_threshold {
            config.similarity_threshold = Threshold::fraction("similarity_threshold", v)?;
        }
        if let Some(ref v) = fc.exclude {
            config.exclude.clone_from(v);
        }
        if let Some(v) = fc.max_exact_duplicates {
            config.max_exact_duplicates = Some(v);
        }
        if let Some(v) = fc.max_near_duplicates {
            config.max_near_duplicates = Some(v);
        }
        if let Some(v) = fc.max_exact_percent {
            config.max_exact_percent = Some(Threshold::percent("max_exact_percent", v)?);
        }
        if let Some(v) = fc.max_near_percent {
            config.max_near_percent = Some(Threshold::percent("max_near_percent", v)?);
        }
        if let Some(v) = fc.min_lines {
            config.min_lines = v;
        }
        if let Some(v) = fc.exclude_tests {
            config.exclude_tests = v;
        }
        if let Some(v) = fc.sub_function {
            config.sub_function = v;
        }
        if let Some(v) = fc.min_sub_nodes {
            config.min_sub_nodes = v;
        }
        if let Some(ref v) = fc.baseline {
            config.baseline = Some(v.clone());
        }
        Ok(config)
    }
}
