// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::cli_error::CliError;
use dry4rust::error::Error as AnalysisError;
use std::error::Error;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::path::PathBuf;

#[test]
fn exit_code_distinguishes_a_failed_check_from_an_internal_error() {
    // Arrange & Act & Assert
    assert_eq!(CliError::CheckFailed.exit_code(), 1);
    assert_eq!(CliError::NoRecognizedFiles.exit_code(), 2);
}

#[test]
fn exit_code_of_invalid_config_is_an_internal_error_rather_than_a_breach() {
    // Arrange
    let error = CliError::InvalidConfig(String::from("similarity_threshold must be in 0.0..=1.0"));

    // Act
    let code = error.exit_code();

    // Assert
    assert_eq!(
        code, 2,
        "a config the tool cannot run under is an error, not duplication over a ceiling"
    );
}

#[test]
fn fmt_of_ambiguous_language_lists_every_candidate() {
    // Arrange
    let error = CliError::AmbiguousLanguage(vec![String::from("rust"), String::from("python")]);

    // Act
    let message = error.to_string();

    // Assert
    assert!(message.contains("rust, python"), "{message}");
}

#[test]
fn fmt_of_invalid_config_is_the_message_it_was_given_and_nothing_more() {
    // Arrange
    let error = CliError::InvalidConfig(String::from("min_nodes must be at least 1"));

    // Act
    let message = error.to_string();

    // Assert
    assert_eq!(message, "min_nodes must be at least 1");
}

#[test]
fn fmt_of_invalid_fingerprint_quotes_what_was_offered() {
    // Arrange
    let error = CliError::InvalidFingerprint(String::from("not-a-fingerprint"));

    // Act
    let message = error.to_string();

    // Assert
    assert_eq!(message, "Invalid fingerprint: not-a-fingerprint");
}

#[test]
fn fmt_of_no_recognized_files_says_which_flag_would_resolve_it() {
    // Arrange
    let error = CliError::NoRecognizedFiles;

    // Act
    let message = error.to_string();

    // Assert
    assert!(message.contains("--language"), "{message}");
}

#[test]
fn fmt_of_no_source_files_names_the_path_it_looked_in() {
    // Arrange
    let error = CliError::NoSourceFiles(PathBuf::from("/some/root"));

    // Act
    let message = error.to_string();

    // Assert
    assert!(
        message.contains("/some/root"),
        "a reader needs the path to know where to look, got: {message}"
    );
}

#[test]
fn from_an_analysis_error_keeps_it_as_the_source() {
    // Arrange
    let analysis = AnalysisError::Other(String::from("the pipeline gave up"));

    // Act
    let error = CliError::from(analysis);

    // Assert
    assert!(matches!(error, CliError::Analysis(_)));
    assert!(
        error.source().is_some(),
        "the cause has to survive the conversion or the report loses it"
    );
    assert_eq!(error.to_string(), "the pipeline gave up");
}

#[test]
fn from_an_io_error_keeps_it_as_the_source() {
    // Arrange
    let io_error = IoError::new(ErrorKind::PermissionDenied, "no");

    // Act
    let error = CliError::from(io_error);

    // Assert
    assert!(matches!(error, CliError::Io(_)));
    assert!(error.source().is_some());
}

#[test]
fn source_of_a_failed_check_is_absent_because_nothing_caused_it() {
    // Arrange
    let error = CliError::CheckFailed;

    // Act
    let cause = error.source();

    // Assert
    assert!(
        cause.is_none(),
        "a breached ceiling is a verdict, not a wrapped failure"
    );
}
