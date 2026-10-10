// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use predicate::str;
use predicates::prelude::*;
use serde_json::Value;
use serde_json::from_str;
use validation::dry_binary::DryBinary;
use validation::options_scenario::OptionsScenario;

#[test]
fn default_command_is_report() {
    // Arrange & Act & Assert
    OptionsScenario::command_on("exact_dupes")
        .assert()
        .success()
        .stdout(str::contains("Duplication Statistics"))
        .stdout(str::contains("Exact Duplicates"));
}

#[test]
fn error_on_nonexistent_path() {
    // Arrange & Act & Assert
    DryBinary::command()
        .args(["--path", "/nonexistent/path/that/does/not/exist", "stats"])
        .assert()
        .code(2)
        .stderr(str::contains("No source files"));
}

#[test]
fn exclude_option_drops_the_named_paths_from_the_report() {
    // Arrange & Act & Assert -- `src` rather than `lib.rs`, because a fixture's
    // code lives in target.rs and lib.rs is only the registry that names it;
    // excluding one file no longer leaves the scan with nothing
    OptionsScenario::command_on("exact_dupes")
        .args(["--exclude", "src", "stats"])
        .assert()
        .code(2)
        .stderr(str::contains("No source files"));
}

#[test]
fn exclude_option_leaves_the_files_it_does_not_name() {
    // Arrange & Act & Assert -- target.rs holds every unit, so dropping it
    // leaves a registry that parses to nothing rather than nothing to parse
    OptionsScenario::command_on("exact_dupes")
        .args(["--exclude", "target.rs", "stats"])
        .assert()
        .success()
        .stdout(str::contains("Total code units analyzed: 0"));
}

#[test]
fn exclude_tests_flag_reduces_duplicates() {
    // Arrange & Act
    let output_all = OptionsScenario::command_on("test_code")
        .args(["--format", "json", "stats"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let all: Value = from_str(&String::from_utf8(output_all).unwrap()).unwrap();

    // Assert
    assert_eq!(all["exact_duplicate_units"].as_u64().unwrap(), 3);

    let output_excl = OptionsScenario::command_on("test_code")
        .args(["--exclude-tests", "--format", "json", "stats"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let excl: Value = from_str(&String::from_utf8(output_excl).unwrap()).unwrap();
    assert_eq!(excl["exact_duplicate_units"].as_u64().unwrap(), 2);
    assert_eq!(excl["total_code_units"].as_u64().unwrap(), 2);
}

#[test]
fn exclude_tests_text_report() {
    // Arrange & Act & Assert
    OptionsScenario::command_on("test_code")
        .args(["--exclude-tests", "report"])
        .assert()
        .success()
        .stdout(str::contains("Exact Duplicates"))
        .stdout(str::contains("Group 1"));
}

#[test]
fn help_flag_lists_every_subcommand_the_enum_declares() {
    // Arrange & Act & Assert
    DryBinary::command()
        .arg("--help")
        .assert()
        .success()
        .stdout(str::contains("stats"))
        .stdout(str::contains("report"))
        .stdout(str::contains("check"))
        .stdout(str::contains("ignore"))
        .stdout(str::contains("ignored"))
        .stdout(str::contains("cleanup"));
}

#[test]
fn help_flag_prints_usage_and_exits_success() {
    // Arrange & Act & Assert
    DryBinary::command()
        .arg("--help")
        .assert()
        .success()
        .stdout(str::contains("Detect duplicate code"));
}

#[test]
fn main_over_a_path_that_does_not_exist_reports_the_error_and_exits_non_zero() {
    // Arrange & Act & Assert
    OptionsScenario::command_on("no_such_fixture_anywhere")
        .assert()
        .failure()
        .stderr(str::contains("Error"));
}

#[test]
fn min_lines_option() {
    // Arrange & Act & Assert
    OptionsScenario::command_on("exact_dupes")
        .args(["--min-lines", "1000", "stats"])
        .assert()
        .success()
        .stdout(str::contains("Exact duplicates: 0 groups"));
}

#[test]
fn min_nodes_option() {
    // Arrange & Act & Assert
    OptionsScenario::command_on("exact_dupes")
        .args(["--min-nodes", "1000", "stats"])
        .assert()
        .success()
        .stdout(str::contains("Exact duplicates: 0 groups"));
}

#[test]
fn threshold_above_one_is_rejected_rather_than_silently_finding_nothing() {
    // Arrange & Act & Assert
    OptionsScenario::command_on("near_dupes")
        .args(["--threshold", "5", "report"])
        .assert()
        .code(2)
        .stderr(str::contains(
            "--threshold must be a fraction between 0.0 and 1.0, got 5",
        ));
}
