// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::{Path, PathBuf};

use serde::Deserialize;
use std::fs;
use toml::from_str;

/// The subset of configuration relevant to language-specific parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisConfig {
    /// Minimum number of AST nodes for a code unit to be analyzed.
    pub min_nodes: usize,
    /// Minimum number of source lines for a code unit to be analyzed.
    pub min_lines: usize,
}

/// Configuration for cargo-dupes analysis.
#[derive(Debug, Clone)]
pub struct Config {
    /// Minimum number of AST nodes for a code unit to be analyzed.
    pub min_nodes: usize,
    /// Similarity threshold for near-duplicates (0.0 to 1.0).
    pub similarity_threshold: f64,
    /// Path patterns to exclude from scanning.
    pub exclude: Vec<String>,
    /// Exit code threshold: fail if exact duplicate count exceeds this.
    pub max_exact_duplicates: Option<usize>,
    /// Exit code threshold: fail if near duplicate count exceeds this.
    pub max_near_duplicates: Option<usize>,
    /// Exit code threshold: fail if exact duplicate percentage exceeds this.
    pub max_exact_percent: Option<f64>,
    /// Exit code threshold: fail if near duplicate percentage exceeds this.
    pub max_near_percent: Option<f64>,
    /// Minimum number of source lines for a code unit to be analyzed.
    pub min_lines: usize,
    /// Exclude test code (#[test] functions and #[cfg(test)] modules).
    pub exclude_tests: bool,
    /// Enable sub-function duplicate detection.
    pub sub_function: bool,
    /// Minimum number of AST nodes for a sub-function unit to be analyzed.
    pub min_sub_nodes: usize,
    /// Root path to analyze.
    pub root: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            min_nodes: 10,
            similarity_threshold: 0.9,
            exclude: Vec::new(),
            max_exact_duplicates: None,
            max_near_duplicates: None,
            max_exact_percent: None,
            max_near_percent: None,
            min_lines: 0,
            exclude_tests: false,
            sub_function: false,
            min_sub_nodes: 5,
            root: PathBuf::from("."),
        }
    }
}

/// Config as stored in dupes.toml or Cargo.toml metadata.
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
    dupes: Option<FileConfig>,
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
    /// 2. dupes.toml in the project root
    /// 3. `[package.metadata.dupes]` in Cargo.toml
    /// 4. Defaults
    #[must_use]
    pub fn load(root: &Path) -> Self {
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
            && let Some(dupes) = meta.dupes
        {
            config.apply_file_config(&dupes);
        }

        // Try dupes.toml (higher priority)
        let dupes_toml = root.join("dupes.toml");
        if dupes_toml.exists()
            && let Ok(content) = fs::read_to_string(&dupes_toml)
            && let Ok(file_config) = from_str::<FileConfig>(&content)
        {
            config.apply_file_config(&file_config);
        }

        config
    }

    fn apply_file_config(&mut self, fc: &FileConfig) {
        if let Some(v) = fc.min_nodes {
            self.min_nodes = v;
        }
        if let Some(v) = fc.similarity_threshold {
            self.similarity_threshold = v;
        }
        if let Some(ref v) = fc.exclude {
            self.exclude.clone_from(v);
        }
        if let Some(v) = fc.max_exact_duplicates {
            self.max_exact_duplicates = Some(v);
        }
        if let Some(v) = fc.max_near_duplicates {
            self.max_near_duplicates = Some(v);
        }
        if let Some(v) = fc.max_exact_percent {
            self.max_exact_percent = Some(v);
        }
        if let Some(v) = fc.max_near_percent {
            self.max_near_percent = Some(v);
        }
        if let Some(v) = fc.min_lines {
            self.min_lines = v;
        }
        if let Some(v) = fc.exclude_tests {
            self.exclude_tests = v;
        }
        if let Some(v) = fc.sub_function {
            self.sub_function = v;
        }
        if let Some(v) = fc.min_sub_nodes {
            self.min_sub_nodes = v;
        }
    }
}
