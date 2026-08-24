// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use clap::CommandFactory;
use clap::Parser;
use dry4rust::cli::command::Command;

/// The smallest thing that can parse a `Command`, so these tests are about the
/// subcommand definitions and not about `main`'s surrounding flags.
#[derive(Parser)]
struct Harness {
    #[command(subcommand)]
    command: Command,
}

fn parsed(args: &[&str]) -> Command {
    Harness::try_parse_from(args)
        .expect("the arguments parse")
        .command
}

#[test]
fn augment_subcommands_declares_every_command_the_tool_serves() {
    // Arrange
    let command = Harness::command();

    // Act
    let names: Vec<&str> = command
        .get_subcommands()
        .map(clap::Command::get_name)
        .collect();

    // Assert
    assert_eq!(
        names,
        vec![
            "stats", "report", "check", "ignore", "ignored", "cleanup", "baseline"
        ],
        "the dispatcher matches on all seven; a new one added here and nowhere \
         else would be unreachable"
    );
}

#[test]
fn augment_subcommands_rejects_a_subcommand_that_does_not_exist() {
    // Arrange & Act
    let outcome = Harness::try_parse_from(["dry4rust", "polish"]);

    // Assert
    assert!(
        outcome.is_err(),
        "a name close to nothing is a typo, not a default"
    );
}

#[test]
fn from_arg_matches_leaves_every_ceiling_unset_when_check_is_given_none() {
    // Arrange & Act
    let command = parsed(&["dry4rust", "check"]);

    // Assert
    assert!(matches!(
        command,
        Command::Check {
            max_exact: None,
            max_near: None,
            max_exact_percent: None,
            max_near_percent: None,
        }
    ));
}

#[test]
fn from_arg_matches_reads_baseline_with_its_dry_run_flag() {
    // Arrange & Act
    let command = parsed(&["dry4rust", "baseline", "--dry-run"]);

    // Assert
    assert!(matches!(command, Command::Baseline { dry_run: true }));
}

#[test]
fn from_arg_matches_reads_check_with_all_four_ceilings() {
    // Arrange & Act
    let command = parsed(&[
        "dry4rust",
        "check",
        "--max-exact",
        "0",
        "--max-near",
        "3",
        "--max-exact-percent",
        "5",
        "--max-near-percent",
        "10.5",
    ]);

    // Assert
    let Command::Check {
        max_exact,
        max_near,
        max_exact_percent,
        max_near_percent,
    } = command
    else {
        panic!("check parses to Check");
    };
    assert_eq!(max_exact, Some(0));
    assert_eq!(max_near, Some(3));
    assert_eq!(max_exact_percent, Some(5.0));
    assert_eq!(max_near_percent, Some(10.5));
}

#[test]
fn from_arg_matches_reads_ignore_with_its_fingerprint_and_reason() {
    // Arrange & Act
    let command = parsed(&[
        "dry4rust",
        "ignore",
        "deadbeef12345678",
        "--reason",
        "trait impls are meant to look alike",
    ]);

    // Assert
    let Command::Ignore {
        fingerprint,
        reason,
    } = command
    else {
        panic!("ignore parses to Ignore");
    };
    assert_eq!(fingerprint, "deadbeef12345678");
    assert_eq!(
        reason.as_deref(),
        Some("trait impls are meant to look alike")
    );
}
