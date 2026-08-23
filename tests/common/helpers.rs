// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use assert_cmd::Command;
use assert_cmd::cargo::cargo_bin_cmd;
use dry4rust::analysis::AnalysisResult;
use dry4rust::analysis::analyze;
use dry4rust::config::Config;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use dry4rust::scanner::ScanConfig;
use dry4rust::scanner::scan_files;
use std::path::PathBuf;

/// Run the analysis pipeline over a fixture crate, as the CLI would.
pub fn analysed(fixture: &str) -> (Config, AnalysisResult) {
    let root = fixture_path(fixture);
    let config = Config::load(&root);
    let files = scan_files(&ScanConfig::new(root));
    let result =
        analyze(&RustAnalyzer::new(), &files, &config).expect("the fixture analyses cleanly");
    (config, result)
}

pub fn cargo_dry4rust() -> Command {
    cargo_bin_cmd!("cargo-dry4rust")
}

pub fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}
