// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::borrow::Cow;
use std::path::Path;

/// How a reporter shows a file: relative to a base when it lies under one.
pub struct DisplayPath;

impl DisplayPath {
    /// Compute a display path relative to an optional base, falling back to the absolute path.
    #[must_use]
    pub fn of<'a>(base: Option<&Path>, path: &'a Path) -> Cow<'a, str> {
        if let Some(base) = base
            && let Ok(rel) = path.strip_prefix(base)
        {
            return rel.to_string_lossy();
        }
        path.to_string_lossy()
    }
}
