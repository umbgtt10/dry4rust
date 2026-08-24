// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::{analysed, cargo_dry4rust, fixture_path};
use dry4rust::cli::baseline_command::BaselineCommand;
use dry4rust::config::Config;
use dry4rust::suppression::baseline_file::BaselineFile;
use predicate::str;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn duplicated_crate_in(tmp: &TempDir) {
    fs::create_dir_all(tmp.path().join("src")).expect("src");
    fs::copy(
        fixture_path("exact_dupes").join("src/lib.rs"),
        tmp.path().join("src/lib.rs"),
    )
    .expect("copy the fixture");
}

#[test]
fn baseline_reads_its_path_from_dry4rust_toml() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    duplicated_crate_in(&tmp);
    fs::write(
        tmp.path().join("dry4rust.toml"),
        "baseline = \"ci/recorded.json\"\n",
    )
    .expect("write the config");
    let root = tmp.path().to_str().expect("utf-8 path");

    // Act
    cargo_dry4rust()
        .args(["--path", root, "baseline"])
        .assert()
        .success();

    // Assert
    assert!(tmp.path().join("ci/recorded.json").exists());
    cargo_dry4rust()
        .args(["--path", root, "check", "--max-exact", "0"])
        .assert()
        .success()
        .stdout(str::contains("Baseline: 1 groups suppressed"));
}

#[test]
fn baseline_then_check_passes_on_what_was_inherited_and_fails_on_what_is_added() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    duplicated_crate_in(&tmp);
    let root = tmp.path().to_str().expect("utf-8 path");

    cargo_dry4rust()
        .args(["--path", root, "check", "--max-exact", "0"])
        .assert()
        .code(1);

    cargo_dry4rust()
        .args(["--path", root, "baseline"])
        .assert()
        .success();

    // Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            root,
            "--baseline",
            "dry4rust-baseline.json",
            "check",
            "--max-exact",
            "0",
        ])
        .assert()
        .success()
        .stdout(str::contains("Baseline: 1 groups suppressed"))
        .stdout(str::contains("Check passed"));

    let mut grown = fs::read_to_string(tmp.path().join("src/lib.rs")).expect("read");
    grown.push_str(
        "\npub fn summarise(numbers: Vec<i32>) -> i32 {\n    let mut acc = 0;\n    \
         for n in numbers.iter() {\n        if *n > 0 {\n            acc += *n;\n        \
         }\n    }\n    acc\n}\n",
    );
    fs::write(tmp.path().join("src/lib.rs"), grown).expect("write");

    cargo_dry4rust()
        .args([
            "--path",
            root,
            "--baseline",
            "dry4rust-baseline.json",
            "check",
            "--max-exact",
            "0",
        ])
        .assert()
        .code(1)
        .stdout(str::contains("Check FAILED"));
}

#[test]
fn main_dispatches_the_baseline_subcommand_in_dry_run() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .arg("baseline")
        .arg("--dry-run")
        .arg("--path")
        .arg(fixture_path("exact_dupes"))
        .assert()
        .success()
        .stdout(str::contains("would be recorded"));
}

#[test]
fn run_in_dry_run_names_what_it_would_record_without_writing_it() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let (config, result) = analysed("exact_dupes");
    let mut out = Vec::new();

    // Act
    BaselineCommand::new(tmp.path(), &config, &result, true)
        .run(&mut out)
        .expect("a dry run succeeds");

    // Assert
    let text = String::from_utf8(out).expect("utf-8");
    assert!(text.contains("1 groups would be recorded"), "{text}");
    assert!(text.contains("process_data"), "{text}");
    assert!(!tmp.path().join("dry4rust-baseline.json").exists());
}

#[test]
fn run_outside_dry_run_writes_a_baseline_that_loads_back() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let (config, result) = analysed("exact_dupes");
    let mut out = Vec::new();

    // Act
    BaselineCommand::new(tmp.path(), &config, &result, false)
        .run(&mut out)
        .expect("recording succeeds");

    // Assert
    let text = String::from_utf8(out).expect("utf-8");
    assert!(text.contains("Recorded 1 groups"), "{text}");
    let written = BaselineFile::load(&tmp.path().join("dry4rust-baseline.json"))
        .expect("what was written is what this build reads");
    assert_eq!(written.len(), 1);
}

#[test]
fn run_over_a_clean_codebase_records_nothing_and_says_so() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let (_, result) = analysed("no_dupes");
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
    assert_eq!(
        String::from_utf8(out).expect("utf-8"),
        format!(
            "Recorded 0 groups in {}.\n",
            tmp.path().join("dry4rust-baseline.json").display()
        )
    );
}

#[test]
fn run_twice_records_the_same_groups_rather_than_emptying_the_file() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    duplicated_crate_in(&tmp);
    let root = tmp.path().to_str().expect("utf-8 path");
    cargo_dry4rust()
        .args(["--path", root, "baseline"])
        .assert()
        .success();
    let first = fs::read_to_string(tmp.path().join("dry4rust-baseline.json")).expect("read");

    // Act
    cargo_dry4rust()
        .args([
            "--path",
            root,
            "--baseline",
            "dry4rust-baseline.json",
            "baseline",
        ])
        .assert()
        .success();

    // Assert
    let second = fs::read_to_string(tmp.path().join("dry4rust-baseline.json")).expect("read");
    assert_eq!(
        first, second,
        "recording judges nothing; a recording that judged against the previous \
         one would find nothing left to record and empty the file"
    );
}
