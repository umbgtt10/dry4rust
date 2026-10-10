// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use predicate::str;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;
use validation::dry_binary::DryBinary;
use validation::fixture::Fixture;
use validation::ignore_scenario::IgnoreScenario;
use validation::text_report::TextReport;

#[test]
fn ignore_add_then_report_suppresses_the_group() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join("src")).unwrap();
    fs::copy(
        Fixture::path("exact_dupes").join("src/target.rs"),
        tmp.path().join("src/lib.rs"),
    )
    .unwrap();

    let output = IgnoreScenario::command_in(tmp.path())
        .args(["report"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let fp = TextReport::fingerprint_in(&String::from_utf8(output).unwrap());

    IgnoreScenario::command_in(tmp.path())
        .args(["ignore", &fp, "--reason", "test ignore"])
        .assert()
        .success()
        .stdout(str::contains("Added"));

    IgnoreScenario::command_in(tmp.path())
        .args(["ignored"])
        .assert()
        .success()
        .stdout(str::contains(&fp))
        .stdout(str::contains("test ignore"));

    let output_after = IgnoreScenario::command_in(tmp.path())
        .args(["stats"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text_after = String::from_utf8(output_after).unwrap();

    // Assert
    assert!(text_after.contains("Exact duplicates: 0 groups"));
}

#[test]
fn ignore_near_duplicate_workflow() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join("src")).unwrap();
    fs::copy(
        Fixture::path("near_dupes").join("src/target.rs"),
        tmp.path().join("src/lib.rs"),
    )
    .unwrap();

    let output = IgnoreScenario::command_in(tmp.path())
        .args(["--threshold", "0.7", "report"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();

    let fp = text
        .lines()
        .find(|l| l.contains("fingerprint:") && l.contains("similarity:"))
        .and_then(|l| {
            let start = l.find("fingerprint: ")? + 13;
            let end = l[start..].find(',')?;
            Some(l[start..start + end].to_string())
        })
        .expect("Should find a fingerprint in near-duplicate group");

    IgnoreScenario::command_in(tmp.path())
        .args(["ignore", &fp, "--reason", "near dupe ignore test"])
        .assert()
        .success()
        .stdout(str::contains("Added"));

    let output_after = IgnoreScenario::command_in(tmp.path())
        .args(["--threshold", "0.7", "stats"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text_after = String::from_utf8(output_after).unwrap();

    // Assert
    assert!(text_after.contains("Near duplicates:  0 groups"));
}

#[test]
fn main_dispatches_the_ignore_subcommand_and_records_the_fingerprint() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");

    // Act & Assert
    DryBinary::command()
        .arg("ignore")
        .arg("cafebabe00000001")
        .arg("--path")
        .arg(tmp.path())
        .assert()
        .success();
}

#[test]
fn main_dispatches_the_ignored_subcommand_without_an_ignore_file() {
    // Arrange & Act & Assert
    DryBinary::command()
        .arg("ignored")
        .arg("--path")
        .arg(Fixture::path("no_dupes"))
        .assert()
        .success();
}
