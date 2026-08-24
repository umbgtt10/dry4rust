// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use crate::common::result_with;
use dry4rust::cli::baseline_command::BaselineCommand;
use dry4rust::config::Config;
use dry4rust::suppression::baseline_file::BaselineFile;
use tempfile::TempDir;

fn recorded(dry_run: bool) -> (TempDir, String) {
    let tmp = TempDir::new().expect("temp dir");
    let result = result_with(
        vec![group(0x11, &["process_data", "compute_total"])],
        vec![],
        vec![],
        vec![],
    );
    let config = Config {
        root: tmp.path().to_path_buf(),
        ..Config::default()
    };
    let mut out = Vec::new();

    BaselineCommand::new(tmp.path(), &config, &result, dry_run)
        .run(&mut out)
        .expect("recording succeeds");

    (tmp, String::from_utf8(out).expect("utf-8"))
}

#[test]
fn run_in_dry_run_names_what_it_would_record_without_writing_it() {
    // Arrange & Act
    let (tmp, text) = recorded(true);

    // Assert
    assert!(text.contains("1 groups would be recorded"), "{text}");
    assert!(text.contains("process_data"), "{text}");
    assert!(!tmp.path().join("dry4rust-baseline.json").exists());
}

#[test]
fn run_outside_dry_run_writes_a_baseline_that_loads_back() {
    // Arrange & Act
    let (tmp, text) = recorded(false);

    // Assert
    assert!(text.contains("Recorded 1 groups"), "{text}");
    let written = BaselineFile::load(&tmp.path().join("dry4rust-baseline.json"))
        .expect("what was written is what this build reads");
    assert_eq!(written.len(), 1);
}

#[test]
fn run_over_a_clean_result_records_nothing_and_says_so() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let result = result_with(vec![], vec![], vec![], vec![]);
    let config = Config {
        root: tmp.path().to_path_buf(),
        ..Config::default()
    };
    let mut out = Vec::new();

    // Act
    BaselineCommand::new(tmp.path(), &config, &result, false)
        .run(&mut out)
        .expect("recording succeeds");

    // Assert
    assert!(
        String::from_utf8(out)
            .expect("utf-8")
            .contains("Recorded 0 groups"),
        "a clean codebase has nothing to inherit"
    );
}
