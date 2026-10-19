// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::analysis::Analysis;
use dry4rust::analysis::analyze;
use dry4rust::config::Config;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use dry4rust::scanner::Scanner;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

pub struct Fixture;

impl Fixture {
    /// One of the corpus crates in `fixture/`, which sits beside this crate rather
    /// than under it: cargo drops any subdirectory holding a Cargo.toml from a
    /// package, so a corpus kept inside the published crate is a corpus the
    /// packaged tests cannot read.
    #[must_use]
    pub fn path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("fixture")
            .join(name)
    }

    /// Run the analysis pipeline over a fixture crate, as the CLI would.
    #[must_use]
    pub fn analysed(name: &str) -> (Config, Analysis) {
        Self::analysed_under(name, false)
    }

    /// The same, with sub-function analysis on, which no fixture configures for
    /// itself.
    #[must_use]
    pub fn analysed_with_sub_function(name: &str) -> (Config, Analysis) {
        Self::analysed_under(name, true)
    }

    /// A throwaway crate holding a known-duplicated source file, for tests that
    /// need a root they can write an ignore or baseline file into.
    pub fn duplicated_crate_in(tmp: &TempDir) {
        fs::create_dir_all(tmp.path().join("src")).expect("src");
        fs::copy(
            Self::path("exact_dupes").join("src/target.rs"),
            tmp.path().join("src/lib.rs"),
        )
        .expect("copy the fixture");
    }

    fn analysed_under(name: &str, sub_function: bool) -> (Config, Analysis) {
        let root = Self::path(name);
        let config = Config {
            sub_function,
            ..Config::load(&root).expect("the fixture configuration is in range")
        };
        let files = Scanner::new(root).scan_files();
        let result =
            analyze(&RustAnalyzer::new(), &files, &config).expect("the fixture analyses cleanly");
        (config, result)
    }
}
