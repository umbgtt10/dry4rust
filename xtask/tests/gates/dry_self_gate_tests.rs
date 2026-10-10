// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::process::command_runner_tests::FakeCommandRunner;
use xtask::gates::dry_self_gate::DrySelfGate;
use xtask::gates::gate::Gate;

fn gate(runner: &FakeCommandRunner) -> DrySelfGate<'_> {
    DrySelfGate::new(
        runner,
        String::from("cargo-dry4rust"),
        String::from("/repo/core/src"),
        String::from("/repo/dry4rust-baseline.json"),
        String::from("25"),
    )
}

// Everything before the `--` is cargo's; everything after it is the tool's.
fn split_at_separator(call: &[String]) -> (Vec<String>, Vec<String>) {
    let separator = call
        .iter()
        .position(|argument| argument == "--")
        .expect("the call separates cargo's arguments from the tool's");
    (call[..separator].to_vec(), call[separator + 1..].to_vec())
}

#[test]
fn label_names_the_duplication_gate() {
    // Arrange
    let runner = FakeCommandRunner::new();

    // Act
    let label = gate(&runner).label();

    // Assert
    assert_eq!(label, "Duplication");
}

// The gate judges this checkout with this checkout's dry4rust. An installed
// copy would be whatever was last published, and a check it passes says
// nothing about the one this tree is about to ship.
#[test]
fn run_builds_the_tool_from_source_rather_than_an_install() {
    // Arrange
    let runner = FakeCommandRunner::new();

    // Act
    let _ = gate(&runner).run();

    // Assert
    let (for_cargo, _) = split_at_separator(&runner.calls()[0]);
    assert_eq!(
        for_cargo,
        vec![
            String::from("run"),
            String::from("--quiet"),
            String::from("--bin"),
            String::from("cargo-dry4rust"),
        ]
    );
}

// The baseline is what turns a zero ceiling into a gate on what a change adds
// rather than a wall made of what was already there, and it matches only at
// the floor it was recorded at. The leading `dry4rust` is the subcommand name
// cargo would insert for `cargo dry4rust`, which the binary expects.
#[test]
fn run_checks_the_path_above_the_floor_against_the_baseline_with_zero_ceilings() {
    // Arrange
    let runner = FakeCommandRunner::new();

    // Act
    let _ = gate(&runner).run();

    // Assert
    let (_, for_tool) = split_at_separator(&runner.calls()[0]);
    assert_eq!(
        for_tool,
        vec![
            String::from("dry4rust"),
            String::from("--path"),
            String::from("/repo/core/src"),
            String::from("--baseline"),
            String::from("/repo/dry4rust-baseline.json"),
            String::from("--min-nodes"),
            String::from("25"),
            String::from("check"),
            String::from("--max-exact"),
            String::from("0"),
            String::from("--max-near"),
            String::from("0"),
        ]
    );
}

#[test]
fn run_with_a_zero_exit_code_returns_ok() {
    // Arrange
    let runner = FakeCommandRunner::new().with_streaming_code(Some(0));

    // Act
    let result = gate(&runner).run();

    // Assert
    assert!(result.is_ok());
}

// 1 is dry4rust's own "a ceiling was exceeded", which has to read as
// duplication added rather than as the tool failing to run.
#[test]
fn run_with_exit_code_one_reports_duplication_beyond_the_baseline() {
    // Arrange
    let runner = FakeCommandRunner::new().with_streaming_code(Some(1));

    // Act
    let result = gate(&runner).run();

    // Assert
    assert_eq!(
        result,
        Err(String::from(
            "duplication was added beyond /repo/dry4rust-baseline.json"
        ))
    );
}

// 2 is a configuration dry4rust refused, or a baseline it could not read --
// the tool did not get as far as measuring anything.
#[test]
fn run_with_exit_code_two_reports_the_exit_code() {
    // Arrange
    let runner = FakeCommandRunner::new().with_streaming_code(Some(2));

    // Act
    let result = gate(&runner).run();

    // Assert
    assert_eq!(result, Err(String::from("exit code Some(2)")));
}

// None means the process was signalled rather than exiting: no verdict at all,
// and it must not be mistaken for one.
#[test]
fn run_with_no_exit_code_reports_the_exit_code() {
    // Arrange
    let runner = FakeCommandRunner::new().with_streaming_code(None);

    // Act
    let result = gate(&runner).run();

    // Assert
    assert_eq!(result, Err(String::from("exit code None")));
}

// Nothing is installed for this gate to find, so it never asks. A missing
// install must not fail a gate that builds the tool itself.
#[test]
fn run_with_the_tool_not_installed_still_builds_it_from_source() {
    // Arrange
    let runner = FakeCommandRunner::new().with_available(false);

    // Act
    let result = gate(&runner).run();

    // Assert
    assert!(result.is_ok());
    assert_eq!(runner.calls()[0][0], "run");
}
