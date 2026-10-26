// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::file_config::FileConfig;
use crate::source_path::SourcePath;

// Configuration as the two files that can state it spell it: a `dry4rust.toml`,
// and the `[package.metadata.dry4rust]` table of a `Cargo.toml`. `None` when the
// file is absent or cannot be read as configuration -- a project with none is
// the ordinary case, and looks the same.
pub trait ConfigSource {
    fn file_config(&self, file: &SourcePath) -> Option<FileConfig>;

    fn package_metadata(&self, manifest: &SourcePath) -> Option<FileConfig>;
}
