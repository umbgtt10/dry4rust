// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::file_config::FileConfig;
use dry4rust::source_path::SourcePath;
use dry4rust::traits::config_source::ConfigSource;
use serde::Deserialize;
use toml::Value;
use toml::from_str;

use crate::system::disk_file::DiskFile;
use crate::system::portable_path::PortablePath;

// Configuration in TOML, read from disk. A file that cannot be read or parsed
// is passed over -- `None` -- exactly as a file that is not there. A baseline
// path and the exclusion patterns take the core's separator.
#[derive(Debug, Clone, Copy, Default)]
pub struct TomlConfigSource;

// Every field as `dry4rust.toml` and `[package.metadata.dry4rust]` spell it,
// each optional, unknown keys ignored.
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct ConfigDocument {
    pub min_nodes: Option<usize>,
    pub similarity_threshold: Option<f64>,
    pub exclude: Option<Vec<String>>,
    pub max_exact_duplicates: Option<usize>,
    pub max_near_duplicates: Option<usize>,
    pub max_exact_percent: Option<f64>,
    pub max_near_percent: Option<f64>,
    pub min_lines: Option<usize>,
    pub exclude_tests: Option<bool>,
    pub sub_function: Option<bool>,
    pub min_sub_nodes: Option<usize>,
    pub baseline: Option<String>,
}

impl From<ConfigDocument> for FileConfig {
    fn from(document: ConfigDocument) -> Self {
        Self::default()
            .with_min_nodes(document.min_nodes)
            .with_similarity_threshold(document.similarity_threshold)
            .with_exclude(
                document
                    .exclude
                    .map(|patterns| patterns.iter().map(|p| PortablePath::pattern(p)).collect()),
            )
            .with_max_exact_duplicates(document.max_exact_duplicates)
            .with_max_near_duplicates(document.max_near_duplicates)
            .with_max_exact_percent(document.max_exact_percent)
            .with_max_near_percent(document.max_near_percent)
            .with_min_lines(document.min_lines)
            .with_exclude_tests(document.exclude_tests)
            .with_sub_function(document.sub_function)
            .with_min_sub_nodes(document.min_sub_nodes)
            .with_baseline(document.baseline.as_deref().map(PortablePath::of_text))
    }
}

impl TomlConfigSource {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    fn read(file: &SourcePath) -> Option<String> {
        DiskFile::new(file).read().ok()
    }
}

impl ConfigSource for TomlConfigSource {
    fn file_config(&self, file: &SourcePath) -> Option<FileConfig> {
        let content = Self::read(file)?;
        from_str::<ConfigDocument>(&content)
            .ok()
            .map(FileConfig::from)
    }

    fn package_metadata(&self, manifest: &SourcePath) -> Option<FileConfig> {
        let content = Self::read(manifest)?;
        from_str::<Value>(&content)
            .ok()?
            .get("package")?
            .get("metadata")?
            .get("dry4rust")?
            .clone()
            .try_into::<ConfigDocument>()
            .ok()
            .map(FileConfig::from)
    }
}
