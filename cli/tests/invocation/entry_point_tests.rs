// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust_cli::invocation::entry_point::EntryPoint;
use std::fs;
use std::process::ExitCode;
use tempfile::TempDir;

const DUPLICATED: &str = r"pub fn process_data(input: Vec<i32>) -> i32 {
    let mut sum = 0;
    for item in input.iter() {
        if *item > 0 {
            sum += *item;
        }
    }
    sum
}

pub fn compute_total(values: Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in values.iter() {
        if *value > 0 {
            sum += *value;
        }
    }
    sum
}
";

fn argv(args: &[&str]) -> Vec<String> {
    args.iter().map(|a| (*a).to_owned()).collect()
}

fn duplicated_crate() -> TempDir {
    let tmp = TempDir::new().expect("temp dir");
    fs::create_dir_all(tmp.path().join("src")).expect("src");
    fs::write(tmp.path().join("src/lib.rs"), DUPLICATED).expect("write");
    tmp
}

#[test]
fn run_over_a_crate_past_a_ceiling_exits_one() {
    // Arrange
    let tmp = duplicated_crate();
    let root = tmp.path().to_str().expect("utf-8").to_owned();

    // Act
    let code = EntryPoint::run(argv(&[
        "cargo-dry4rust",
        "dry4rust",
        "--path",
        &root,
        "check",
        "--max-exact",
        "0",
    ]));

    // Assert
    assert_eq!(
        code,
        ExitCode::from(1),
        "1 is duplication over a ceiling, as against 2 for an error"
    );
}

#[test]
fn run_over_a_crate_within_its_ceilings_exits_zero() {
    // Arrange
    let tmp = duplicated_crate();
    let root = tmp.path().to_str().expect("utf-8").to_owned();

    // Act
    let code = EntryPoint::run(argv(&[
        "cargo-dry4rust",
        "dry4rust",
        "--path",
        &root,
        "check",
        "--max-exact",
        "1",
    ]));

    // Assert
    assert_eq!(code, ExitCode::SUCCESS);
}

#[test]
fn run_over_a_request_for_help_exits_zero() {
    // Arrange
    let args = ["cargo-dry4rust", "dry4rust", "--help"];

    // Act
    let code = EntryPoint::run(args.iter().map(|a| (*a).to_owned()).collect());

    // Assert
    assert_eq!(
        code,
        ExitCode::SUCCESS,
        "clap reports --help the same way it reports a typo; only use_stderr          tells them apart"
    );
}

#[test]
fn run_over_a_root_with_no_sources_reports_the_error_and_exits_two() {
    // Arrange
    let args = [
        "cargo-dry4rust",
        "dry4rust",
        "--path",
        "/no/such/place",
        "stats",
    ];

    // Act
    let code = EntryPoint::run(args.iter().map(|a| (*a).to_owned()).collect());

    // Assert
    assert_eq!(
        code,
        ExitCode::from(2),
        "2 is the code for an error the tool could not run through, as against          1 for duplication over a ceiling"
    );
}

#[test]
fn run_over_arguments_it_cannot_parse_exits_two() {
    // Arrange
    let args = ["cargo-dry4rust", "dry4rust", "--no-such-flag"];

    // Act
    let code = EntryPoint::run(args.iter().map(|a| (*a).to_owned()).collect());

    // Assert
    assert_eq!(code, ExitCode::from(2));
}
