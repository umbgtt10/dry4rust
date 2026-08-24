// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use assert_cmd::Command;
use dry4rust::analysis::AnalysisResult;
use dry4rust::analysis::analyze;
use dry4rust::code_unit::CodeUnit;
use dry4rust::code_unit::CodeUnitKind;
use dry4rust::config::Config;
use dry4rust::fingerprint::Fingerprint;
use dry4rust::grouper::DuplicateGroup;
use dry4rust::node::NodeKind;
use dry4rust::node::NormalizedNode;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use dry4rust::scanner::ScanConfig;
use dry4rust::scanner::scan_files;
use std::env::consts::EXE_SUFFIX;
use std::env::current_exe;
use std::fs;
use std::path::PathBuf;
use std::process::Command as StdCommand;
use tempfile::TempDir;

/// Run the analysis pipeline over a fixture crate, as the CLI would.
pub fn analysed(fixture: &str) -> (Config, AnalysisResult) {
    analysed_under(fixture, false)
}

fn analysed_under(fixture: &str, sub_function: bool) -> (Config, AnalysisResult) {
    let root = fixture_path(fixture);
    let config = Config {
        sub_function,
        ..Config::load(&root).expect("the fixture configuration is in range")
    };
    let files = scan_files(&ScanConfig::new(root));
    let result =
        analyze(&RustAnalyzer::new(), &files, &config).expect("the fixture analyses cleanly");
    (config, result)
}

/// The same, with sub-function analysis on, which no fixture configures for
/// itself.
pub fn analysed_with_sub_function(fixture: &str) -> (Config, AnalysisResult) {
    analysed_under(fixture, true)
}

/// The built `cargo-dry4rust` binary.
///
/// Found beside this test binary rather than through `CARGO_BIN_EXE_*`, which
/// cargo defines only for tests in the package that declares the binary --
/// and the binary is core's, while these tests are not. `cargo_bin` would
/// resolve it, and is deprecated.
pub fn cargo_dry4rust() -> Command {
    let mut path = current_exe().expect("the test binary knows where it is");
    path.pop();
    if path.ends_with("deps") {
        path.pop();
    }
    path.push(format!("cargo-dry4rust{EXE_SUFFIX}"));
    Command::from_std(StdCommand::new(path))
}

/// A throwaway crate holding a known-duplicated source file, for tests that
/// need a root they can write an ignore or baseline file into.
pub fn duplicated_crate_in(tmp: &TempDir) {
    fs::create_dir_all(tmp.path().join("src")).expect("src");
    fs::copy(
        fixture_path("exact_dupes").join("src/lib.rs"),
        tmp.path().join("src/lib.rs"),
    )
    .expect("copy the fixture");
}

/// The first fingerprint a text report names, which is how these tests learn a
/// value only the tool can produce.
pub fn fingerprint_in(report: &str) -> String {
    report
        .lines()
        .find(|line| line.contains("fingerprint:"))
        .and_then(|line| {
            let start = line.find("fingerprint: ")? + 13;
            let end = line[start..].find(',')?;
            Some(line[start..start + end].to_owned())
        })
        .expect("the report names a fingerprint")
}

/// One of the corpus crates in `fixture/`, which sits beside this crate rather
/// than under it: cargo drops any subdirectory holding a Cargo.toml from a
/// package, so a corpus kept inside the published crate is a corpus the
/// packaged tests cannot read.
pub fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("fixture")
        .join(name)
}

/// A duplicate group with a stated fingerprint and members, for tests that
/// care about grouping rather than about the code behind it.
pub fn group(fingerprint: u64, names: &[&str]) -> DuplicateGroup {
    let fingerprint = Fingerprint::new(fingerprint);
    DuplicateGroup {
        fingerprint,
        members: names
            .iter()
            .map(|name| CodeUnit {
                kind: CodeUnitKind::Function,
                name: (*name).to_owned(),
                file: PathBuf::from("src/lib.rs"),
                line_start: 1,
                line_end: 9,
                signature: NormalizedNode::leaf(NodeKind::Opaque),
                body: NormalizedNode::leaf(NodeKind::Opaque),
                fingerprint,
                node_count: 9,
                parent_name: None,
                is_test: false,
            })
            .collect(),
        similarity: 1.0,
    }
}
