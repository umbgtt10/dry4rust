// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;

use crate::output::json::JsonReporter;
use crate::output::reporter::Reporter;
use crate::output::text::TextReporter;

/// Output format for CLI reports.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, clap::ValueEnum)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}

impl OutputFormat {
    /// Build the reporter that writes this format.
    ///
    /// `root` is the base every reported path is shown relative to; `None`
    /// reports absolute paths.
    #[must_use]
    pub fn reporter(self, root: Option<&Path>) -> Box<dyn Reporter> {
        match self {
            Self::Text => Box::new(TextReporter::new(root.map(Path::to_path_buf))),
            Self::Json => Box::new(JsonReporter::new(root.map(Path::to_path_buf))),
        }
    }
}
