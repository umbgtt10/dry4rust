// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::{MAIN_SEPARATOR, Path};

use dry4rust::source_path::SourcePath;

// Where a platform path becomes the core's: `/` is the core's only separator, so
// on Windows each `\` turns into one; on Unix `\` is an ordinary byte of a name
// and stays. `of` is `None` for a path that is not UTF-8, which the core cannot
// hold; `lossy` holds it anyway, replacing what is not UTF-8.
pub struct PortablePath;

impl PortablePath {
    #[must_use]
    pub fn lossy(path: &Path) -> SourcePath {
        Self::of_text(&path.to_string_lossy())
    }

    #[must_use]
    pub fn of(path: &Path) -> Option<SourcePath> {
        path.to_str().map(Self::of_text)
    }

    #[must_use]
    pub fn of_text(text: &str) -> SourcePath {
        SourcePath::new(Self::pattern(text))
    }

    // An exclusion pattern is matched against the core's paths as text, so it
    // takes the same separator they do.
    #[must_use]
    pub fn pattern(text: &str) -> String {
        if MAIN_SEPARATOR == '\\' {
            text.replace('\\', "/")
        } else {
            String::from(text)
        }
    }
}
