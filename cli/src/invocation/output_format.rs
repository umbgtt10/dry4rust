// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::output::json_reporter::JsonReporter;
use dry4rust::cli::output::reporter::Reporter;
use dry4rust::cli::output::text_reporter::TextReporter;
use dry4rust::source_path::SourcePath;

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
    pub fn reporter(self, root: Option<&SourcePath>) -> Box<dyn Reporter> {
        match self {
            Self::Text => Box::new(TextReporter::new(root.cloned())),
            Self::Json => Box::new(JsonReporter::new(root.cloned())),
        }
    }
}
