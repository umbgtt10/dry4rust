// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::file_config::FileConfig;
use dry4rust::source_path::SourcePath;
use dry4rust::traits::config_source::ConfigSource;
use std::collections::BTreeMap;

// Configuration held in memory: what each `dry4rust.toml` and each
// `Cargo.toml`'s metadata table would state, by the file's path.
#[derive(Debug, Clone, Default)]
pub struct InMemoryConfigSource {
    files: BTreeMap<SourcePath, FileConfig>,
    manifests: BTreeMap<SourcePath, FileConfig>,
}

impl InMemoryConfigSource {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_file_config(self, path: &str, config: FileConfig) -> Self {
        let mut files = self.files;
        files.insert(SourcePath::new(String::from(path)), config);
        Self { files, ..self }
    }

    #[must_use]
    pub fn with_package_metadata(self, path: &str, config: FileConfig) -> Self {
        let mut manifests = self.manifests;
        manifests.insert(SourcePath::new(String::from(path)), config);
        Self { manifests, ..self }
    }
}

impl ConfigSource for InMemoryConfigSource {
    fn file_config(&self, file: &SourcePath) -> Option<FileConfig> {
        self.files.get(file).cloned()
    }

    fn package_metadata(&self, manifest: &SourcePath) -> Option<FileConfig> {
        self.manifests.get(manifest).cloned()
    }
}
