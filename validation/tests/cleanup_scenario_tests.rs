// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::cleanup_command::CleanupCommand;
use dry4rust::cli::ignore_command::IgnoreCommand;
use predicate::str;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;
use validation::cleanup_scenario::CleanupScenario;
use validation::dry_binary::DryBinary;
use validation::fixture::Fixture;

#[test]
fn cleanup_dry_run() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join("src")).unwrap();
    fs::copy(
        Fixture::path("exact_dupes").join("src/target.rs"),
        tmp.path().join("src/lib.rs"),
    )
    .unwrap();

    let ignore_path = tmp.path().join(".dry4rust-ignore.toml");
    fs::write(
        &ignore_path,
        "[[ignore]]\nfingerprint = \"deadbeefdeadbeef\"\nreason = \"stale\"\n",
    )
    .unwrap();

    CleanupScenario::command_in(tmp.path())
        .args(["cleanup", "--dry-run"])
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
        Fixture::path("exact_dupes").join("src/target.rs"),
        tmp.path().join("src/lib.rs"),
    )
    .unwrap();

    let output = CleanupScenario::command_in(tmp.path())
        .args(["report"])
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

    CleanupScenario::command_in(tmp.path())
        .args(["ignore", &real_fp])
        .assert()
        .success();

    let ignore_path = tmp.path().join(".dry4rust-ignore.toml");
    let content = fs::read_to_string(&ignore_path).unwrap();
    let new_content = format!(
        "{content}\n[[ignore]]\nfingerprint = \"deadbeefdeadbeef\"\nreason = \"stale entry\"\n"
    );
    fs::write(&ignore_path, new_content).unwrap();

    CleanupScenario::command_in(tmp.path())
        .args(["cleanup"])
        .assert()
        .success()
        .stdout(str::contains("Removed stale entries"))
        .stdout(str::contains("deadbeefdeadbeef"))
        .stdout(str::contains("Removed 1 stale entries"));

    CleanupScenario::command_in(tmp.path())
        .args(["ignored"])
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
    DryBinary::command()
        .arg("cleanup")
        .arg("--dry-run")
        .arg("--path")
        .arg(Fixture::path("exact_dupes"))
        .assert()
        .success();
}

#[test]
fn run_in_dry_run_leaves_the_ignore_file_alone() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let (_, result) = Fixture::analysed("exact_dupes");

    // Act
    CleanupCommand::new(
        Fixture::ports().ignore,
        &Fixture::root_of(tmp.path()),
        &result,
        true,
    )
    .run()
    .expect("dry run succeeds");

    // Assert
    assert!(!tmp.path().join(".dry4rust-ignore.toml").exists());
}

#[test]
fn run_outside_dry_run_takes_away_a_file_it_emptied() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let (_, result) = Fixture::analysed("exact_dupes");
    IgnoreCommand::new(
        Fixture::ports().ignore,
        &Fixture::root_of(tmp.path()),
        "deadbeef12345678",
        None,
    )
    .run()
    .expect("ignore accepted");

    // Act
    let text = CleanupCommand::new(
        Fixture::ports().ignore,
        &Fixture::root_of(tmp.path()),
        &result,
        false,
    )
    .run()
    .expect("cleanup succeeds")
    .text()
    .to_string();

    // Assert
    assert!(
        text.contains("stale"),
        "cleanup reports on stale entries by name, got: {text}"
    );
    assert!(
        !tmp.path().join(".dry4rust-ignore.toml").exists(),
        "that entry was the only one, so pruning it leaves no file rather than          one reading `ignore = []`"
    );
}

#[test]
fn run_with_nothing_stale_says_so_and_writes_no_file() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let (_, result) = Fixture::analysed("exact_dupes");

    // Act
    let text = CleanupCommand::new(
        Fixture::ports().ignore,
        &Fixture::root_of(tmp.path()),
        &result,
        false,
    )
    .run()
    .expect("cleanup succeeds")
    .text()
    .to_string();

    // Assert
    assert_eq!(text, "No stale entries found.\n");
    assert!(!tmp.path().join(".dry4rust-ignore.toml").exists());
}
