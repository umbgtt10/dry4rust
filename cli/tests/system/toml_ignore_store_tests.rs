// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::result_with;
use crate::common::source_path;
use dry4rust::cli::cleanup_command::CleanupCommand;
use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::ignore_command::IgnoreCommand;
use dry4rust::cli::suppression::ignore_file::IgnoreFile;
use dry4rust::fingerprint::Fingerprint;
use dry4rust::node::literal_kind::LiteralKind;
use dry4rust::node::node_kind::NodeKind;
use dry4rust::node::normalized_node::NormalizedNode;
use dry4rust::storage_failure::StorageFailure;
use dry4rust::traits::ignore_store::IgnoreStore;
use dry4rust_cli::system::toml_ignore_store::TomlIgnoreStore;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn cleaned(dry_run: bool) -> (TempDir, String) {
    let tmp = TempDir::new().expect("temp dir");
    let root = source_path(tmp.path());
    IgnoreCommand::new(&TomlIgnoreStore::new(), &root, "deadbeef12345678", None)
        .run()
        .expect("ignore accepted");
    let result = result_with(vec![], vec![], vec![], vec![]);

    let text = CleanupCommand::new(&TomlIgnoreStore::new(), &root, &result, dry_run)
        .run()
        .expect("cleanup succeeds")
        .text()
        .to_string();

    (tmp, text)
}

fn suppressing(fingerprints: &[(Fingerprint, Option<&str>)]) -> IgnoreFile {
    fingerprints
        .iter()
        .fold(IgnoreFile::default(), |file, (fp, reason)| {
            file.with_ignored(fp, reason.map(ToOwned::to_owned), vec![])
        })
}

fn test_fingerprint() -> Fingerprint {
    Fingerprint::from_node(&NormalizedNode::leaf(NodeKind::Literal(LiteralKind::Int)))
}

#[test]
fn load_of_a_file_that_is_not_there_says_it_is_missing() {
    // Arrange
    let tmp = TempDir::new().unwrap();

    // Act
    let outcome =
        TomlIgnoreStore::new().load(&source_path(&tmp.path().join(".dry4rust-ignore.toml")));

    // Assert
    assert!(
        matches!(outcome, Err(StorageFailure::Missing(_))),
        "{outcome:?}"
    );
}

#[test]
fn load_of_a_file_that_is_not_toml_reports_what_the_parser_said() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join(".dry4rust-ignore.toml"), "ignore = = =\n").unwrap();

    // Act
    let outcome =
        TomlIgnoreStore::new().load(&source_path(&tmp.path().join(".dry4rust-ignore.toml")));

    // Assert
    assert!(
        matches!(outcome, Err(StorageFailure::Failed(_))),
        "{outcome:?}"
    );
}

#[test]
fn remove_of_a_file_that_is_not_there_says_it_is_missing() {
    // Arrange
    let tmp = TempDir::new().unwrap();

    // Act
    let outcome =
        TomlIgnoreStore::new().remove(&source_path(&tmp.path().join(".dry4rust-ignore.toml")));

    // Assert
    assert!(
        matches!(outcome, Err(StorageFailure::Missing(_))),
        "{outcome:?}"
    );
}

#[test]
fn run_in_dry_run_names_the_stale_entry_and_leaves_the_file_alone() {
    // Arrange & Act
    let (tmp, text) = cleaned(true);

    // Assert
    assert!(text.contains("deadbeef12345678"), "{text}");
    assert!(text.contains("would be removed"), "{text}");
    let kept = fs::read_to_string(IgnoreFile::path_in(&source_path(tmp.path())).as_str())
        .expect("read back");
    assert!(kept.contains("deadbeef12345678"), "{kept}");
}

#[test]
fn run_outside_dry_run_prunes_the_entry_that_matches_nothing() {
    // Arrange & Act
    let (tmp, text) = cleaned(false);

    // Assert
    assert!(text.contains("Removed 1 stale entries"), "{text}");
    assert!(
        !Path::new(IgnoreFile::path_in(&source_path(tmp.path())).as_str()).exists(),
        "that was the only entry, so cleanup leaves no file rather than one          reading `ignore = []` -- which is how this repository acquired its own"
    );
}

#[test]
fn run_with_a_malformed_fingerprint_is_rejected() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");

    // Act
    let outcome = IgnoreCommand::new(
        &TomlIgnoreStore::new(),
        &source_path(tmp.path()),
        "not-a-fingerprint",
        None,
    )
    .run();

    // Assert
    assert!(matches!(outcome, Err(CliError::InvalidFingerprint(_))));
    assert!(
        !tmp.path().join(".dry4rust-ignore.toml").exists(),
        "a rejected fingerprint leaves no file behind"
    );
}

#[test]
fn run_with_nothing_stale_says_so_and_writes_no_file() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let result = result_with(vec![], vec![], vec![], vec![]);

    // Act
    let text = CleanupCommand::new(
        &TomlIgnoreStore::new(),
        &source_path(tmp.path()),
        &result,
        false,
    )
    .run()
    .expect("cleanup succeeds")
    .text()
    .to_string();

    // Assert
    assert_eq!(text, "No stale entries found.\n");
    assert!(!Path::new(IgnoreFile::path_in(&source_path(tmp.path())).as_str()).exists());
}

#[test]
fn save_into_a_directory_that_does_not_exist_fails_with_what_the_platform_said() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("missing").join(".dry4rust-ignore.toml");

    // Act
    let outcome = TomlIgnoreStore::new().save(
        &source_path(&file),
        &suppressing(&[(test_fingerprint(), None)]),
    );

    // Assert
    let error = CliError::from(outcome.expect_err("there is no directory to write into"));
    assert!(error.to_string().contains("os error"), "{error}");
}

#[test]
fn save_of_an_emptied_file_takes_it_away_rather_than_writing_an_empty_list() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    let fp = test_fingerprint();
    suppressing(&[(fp, None)])
        .save(&TomlIgnoreStore::new(), &source_path(tmp.path()))
        .expect("the first save writes it");
    let (emptied, _) =
        IgnoreFile::load(&TomlIgnoreStore::new(), &source_path(tmp.path())).without(&fp.to_hex());

    // Act
    emptied
        .save(&TomlIgnoreStore::new(), &source_path(tmp.path()))
        .expect("saving nothing succeeds");

    // Assert
    assert!(
        !Path::new(IgnoreFile::path_in(&source_path(tmp.path())).as_str()).exists(),
        "an empty suppression list makes exactly the claim that no file makes,          and load cannot tell them apart"
    );
}

#[test]
fn save_then_load_returns_what_was_recorded() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    let fp = test_fingerprint();
    let ignore = IgnoreFile::default().with_ignored(
        &fp,
        Some("test reason".to_string()),
        vec!["foo".to_string(), "bar".to_string()],
    );

    // Act
    ignore
        .save(&TomlIgnoreStore::new(), &source_path(tmp.path()))
        .unwrap();
    let loaded = IgnoreFile::load(&TomlIgnoreStore::new(), &source_path(tmp.path()));

    // Assert
    assert_eq!(loaded.ignore().len(), 1);
    assert_eq!(loaded.ignore()[0].fingerprint, fp.to_hex());
    assert_eq!(loaded.ignore()[0].reason.as_deref(), Some("test reason"));
    assert_eq!(loaded.ignore()[0].members, vec!["foo", "bar"]);
}

#[test]
fn save_writes_one_table_per_entry_in_the_file_beside_the_root() {
    // Arrange
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join(".dry4rust-ignore.toml");

    // Act
    TomlIgnoreStore::new()
        .save(
            &source_path(&file),
            &suppressing(&[(test_fingerprint(), Some("kept"))]),
        )
        .expect("the file is written");

    // Assert
    let written = fs::read_to_string(&file).expect("read back");
    assert_eq!(
        written,
        format!(
            "[[ignore]]\nfingerprint = \"{}\"\nreason = \"kept\"\n",
            test_fingerprint().to_hex()
        )
    );
}
