// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::analysed;
use crate::common::cargo_dry4rust;
use crate::common::fixture_path;
use dry4rust::cli::cleanup_command::CleanupCommand;
use dry4rust::cli::ignore_command::IgnoreCommand;
use predicate::str;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn cleanup_dry_run() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join("src")).unwrap();
    fs::copy(
        fixture_path("exact_dupes").join("src/lib.rs"),
        tmp.path().join("src/lib.rs"),
    )
    .unwrap();

    let ignore_path = tmp.path().join(".dry4rust-ignore.toml");
    fs::write(
        &ignore_path,
        "[[ignore]]\nfingerprint = \"deadbeefdeadbeef\"\nreason = \"stale\"\n",
    )
    .unwrap();

    cargo_dry4rust()
        .args([
            "--path",
            tmp.path().to_str().unwrap(),
            "cleanup",
            "--dry-run",
        ])
        .assert()
        .success()
        .stdout(str::contains("Stale entries (dry run)"))
        .stdout(str::contains("deadbeefdeadbeef"))
        .stdout(str::contains("would be removed"));

    let content = fs::read_to_string(&ignore_path).unwrap();

    // Assert
    assert!(content.contains("deadbeefdeadbeef"));
}

#[test]
fn cleanup_removes_stale_entries() {
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
    let text = String::from_utf8(output).unwrap();

    let real_fp = text
        .lines()
        .find(|l| l.contains("fingerprint:"))
        .and_then(|l| {
            let start = l.find("fingerprint: ")? + 13;
            let end = l[start..].find(',')?;
            Some(l[start..start + end].to_string())
        })
        .expect("Should find a fingerprint");

    cargo_dry4rust()
        .args(["--path", tmp.path().to_str().unwrap(), "ignore", &real_fp])
        .assert()
        .success();

    let ignore_path = tmp.path().join(".dry4rust-ignore.toml");
    let content = fs::read_to_string(&ignore_path).unwrap();
    let new_content = format!(
        "{content}\n[[ignore]]\nfingerprint = \"deadbeefdeadbeef\"\nreason = \"stale entry\"\n"
    );
    fs::write(&ignore_path, new_content).unwrap();

    cargo_dry4rust()
        .args(["--path", tmp.path().to_str().unwrap(), "cleanup"])
        .assert()
        .success()
        .stdout(str::contains("Removed stale entries"))
        .stdout(str::contains("deadbeefdeadbeef"))
        .stdout(str::contains("Removed 1 stale entries"));

    cargo_dry4rust()
        .args(["--path", tmp.path().to_str().unwrap(), "ignored"])
        .assert()
        .success()
        .stdout(str::contains(&real_fp));

    let final_content = fs::read_to_string(&ignore_path).unwrap();

    // Assert
    assert!(!final_content.contains("deadbeefdeadbeef"));
}

#[test]
fn main_dispatches_the_cleanup_subcommand_in_dry_run() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .arg("cleanup")
        .arg("--dry-run")
        .arg("--path")
        .arg(fixture_path("exact_dupes"))
        .assert()
        .success();
}

#[test]
fn run_in_dry_run_leaves_the_ignore_file_alone() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let (_, result) = analysed("exact_dupes");
    let mut out = Vec::new();

    // Act
    CleanupCommand::new(tmp.path(), &result, true)
        .run(&mut out)
        .expect("dry run succeeds");

    // Assert
    assert!(!tmp.path().join(".dry4rust-ignore.toml").exists());
}

#[test]
fn run_outside_dry_run_writes_the_pruned_ignore_file() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let (_, result) = analysed("exact_dupes");
    let mut added = Vec::new();
    IgnoreCommand::new(tmp.path(), "deadbeef12345678", None)
        .run(&mut added)
        .expect("ignore accepted");
    let mut out = Vec::new();

    // Act
    CleanupCommand::new(tmp.path(), &result, false)
        .run(&mut out)
        .expect("cleanup succeeds");

    // Assert
    let text = String::from_utf8(out).expect("utf-8");
    assert!(
        text.contains("stale"),
        "cleanup reports on stale entries by name, got: {text}"
    );
    let pruned = fs::read_to_string(tmp.path().join(".dry4rust-ignore.toml")).expect("read back");
    assert!(
        !pruned.contains("deadbeef12345678"),
        "the entry matching nothing is gone from the file, got: {pruned}"
    );
}

#[test]
fn run_with_nothing_stale_says_so_and_writes_no_file() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let (_, result) = analysed("exact_dupes");
    let mut out = Vec::new();

    // Act
    CleanupCommand::new(tmp.path(), &result, false)
        .run(&mut out)
        .expect("cleanup succeeds");

    // Assert
    assert_eq!(
        String::from_utf8(out).expect("utf-8"),
        "No stale entries found.\n"
    );
    assert!(!tmp.path().join(".dry4rust-ignore.toml").exists());
}
