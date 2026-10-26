// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::fixture::Fixture;
use dry4rust::cli::analysis_output::AnalysisOutput;
use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use dry4rust_cli::system::walkdir_source_tree::WalkdirSourceTree;
use std::path::Path;

pub struct PipelineScenario;

impl PipelineScenario {
    #[must_use]
    pub fn produced(root: &Path, overrides: &CliOverrides) -> AnalysisOutput {
        AnalysisOutput::produce(
            &RustAnalyzer::new(),
            &WalkdirSourceTree,
            &Fixture::root_of(root),
            overrides,
        )
        .expect("the fixture analyses cleanly")
    }
}
