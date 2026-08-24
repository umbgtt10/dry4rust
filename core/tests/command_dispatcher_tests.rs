// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::cli_error::CliError;
use dry4rust::cli::cli_overrides::CliOverrides;
use dry4rust::cli::command::Command;
use dry4rust::cli::output_format::OutputFormat;
use dry4rust::command_dispatcher::CommandDispatcher;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use std::path::Path;
use tempfile::TempDir;

fn dispatch_in(root: &Path, command: &Command) -> (Result<(), CliError>, String) {
    let analyzer = RustAnalyzer::new();
    let dispatcher =
        CommandDispatcher::new(&analyzer, root, OutputFormat::Text, CliOverrides::default());
    let mut out = Vec::new();
    let outcome = dispatcher.dispatch(command, &mut out);
    (outcome, String::from_utf8(out).expect("utf-8"))
}

#[test]
fn dispatch_with_ignore_records_the_fingerprint_without_analysing() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let command = Command::Ignore {
        fingerprint: "cafebabe00000001".to_string(),
        reason: Some("a reason".to_string()),
    };

    // Act
    let (outcome, _) = dispatch_in(tmp.path(), &command);

    // Assert
    assert!(
        outcome.is_ok(),
        "an empty directory has no sources, yet ignore does not analyse"
    );
}

#[test]
fn dispatch_with_ignored_lists_what_ignore_recorded() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let recorded = Command::Ignore {
        fingerprint: "cafebabe00000002".to_string(),
        reason: None,
    };
    dispatch_in(tmp.path(), &recorded)
        .0
        .expect("ignore accepted");

    // Act
    let (outcome, text) = dispatch_in(tmp.path(), &Command::Ignored);

    // Assert
    assert!(outcome.is_ok());
    assert!(
        text.contains("cafebabe00000002"),
        "the listing names the fingerprint just recorded, got: {text}"
    );
}
