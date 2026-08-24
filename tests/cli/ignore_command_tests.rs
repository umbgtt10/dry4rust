// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::{cargo_dry4rust, fixture_path};
use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::ignore_command::IgnoreCommand;
use dry4rust::suppression::ignore_file::IgnoreFile;
use predicate::str;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn fingerprint_in(report: &str) -> String {
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

#[test]
fn ignore_add_then_report_suppresses_the_group() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join("src")).unwrap();
    fs::copy(
        fixture_path("exact_dupes").join("src/lib.rs"),
        tmp.path().join("src/lib.rs"),
    )
    .unwrap();

    let output = cargo_dry4rust()
        .args(["--path", tmp.path().to_str().unwrap(), "report"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let fp = fingerprint_in(&String::from_utf8(output).unwrap());

    cargo_dry4rust()
        .args([
            "--path",
            tmp.path().to_str().unwrap(),
            "ignore",
            &fp,
            "--reason",
            "test ignore",
        ])
        .assert()
        .success()
        .stdout(str::contains("Added"));

    cargo_dry4rust()
        .args(["--path", tmp.path().to_str().unwrap(), "ignored"])
        .assert()
        .success()
        .stdout(str::contains(&fp))
        .stdout(str::contains("test ignore"));

    let output_after = cargo_dry4rust()
        .args(["--path", tmp.path().to_str().unwrap(), "stats"])
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
        fixture_path("near_dupes").join("src/lib.rs"),
        tmp.path().join("src/lib.rs"),
    )
    .unwrap();

    let output = cargo_dry4rust()
        .args([
            "--path",
            tmp.path().to_str().unwrap(),
            "--threshold",
            "0.7",
            "report",
        ])
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

    cargo_dry4rust()
        .args([
            "--path",
            tmp.path().to_str().unwrap(),
            "ignore",
            &fp,
            "--reason",
            "near dupe ignore test",
        ])
        .assert()
        .success()
        .stdout(str::contains("Added"));

    let output_after = cargo_dry4rust()
        .args([
            "--path",
            tmp.path().to_str().unwrap(),
            "--threshold",
            "0.7",
            "stats",
        ])
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
    cargo_dry4rust()
        .arg("ignore")
        .arg("cafebabe00000001")
        .arg("--path")
        .arg(tmp.path())
        .assert()
        .success();
}

#[test]
fn run_over_the_same_fingerprint_twice_records_it_once() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let mut out = Vec::new();
    IgnoreCommand::new(tmp.path(), "deadbeef12345678", None)
        .run(&mut out)
        .expect("the first is accepted");

    // Act
    IgnoreCommand::new(tmp.path(), "deadbeef12345678", None)
        .run(&mut out)
        .expect("the second is accepted");

    // Assert
    assert_eq!(IgnoreFile::load(tmp.path()).ignore.len(), 1);
}

#[test]
fn run_records_the_fingerprint_with_the_reason_it_was_given() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let mut out = Vec::new();

    // Act
    IgnoreCommand::new(
        tmp.path(),
        "deadbeef12345678",
        Some("trait impls are meant to look alike"),
    )
    .run(&mut out)
    .expect("a valid fingerprint is accepted");

    // Assert
    let ignore_file = IgnoreFile::load(tmp.path());
    assert_eq!(ignore_file.ignore.len(), 1);
    assert_eq!(ignore_file.ignore[0].fingerprint, "deadbeef12345678");
    assert_eq!(
        ignore_file.ignore[0].reason.as_deref(),
        Some("trait impls are meant to look alike")
    );
}

#[test]
fn run_with_a_malformed_fingerprint_is_rejected() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let mut out = Vec::new();

    // Act
    let outcome = IgnoreCommand::new(tmp.path(), "not-a-fingerprint", None).run(&mut out);

    // Assert
    assert!(matches!(outcome, Err(CliError::InvalidFingerprint(_))));
    assert!(
        !tmp.path().join(".dry4rust-ignore.toml").exists(),
        "a rejected fingerprint leaves no file behind"
    );
}
