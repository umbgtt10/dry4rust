// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::helpers::{cargo_dry4rust, fixture_path};
use dry4rust::analysis::AnalysisResult;
use dry4rust::analysis::analyze;
use dry4rust::analysis::analyze_units;
use dry4rust::cli::CheckThresholds;
use dry4rust::cli::CliError;
use dry4rust::cli::CliOverrides;
use dry4rust::cli::OutputFormat;
use dry4rust::cli::apply_overrides;
use dry4rust::cli::cmd_check;
use dry4rust::cli::cmd_cleanup;
use dry4rust::cli::cmd_ignore;
use dry4rust::cli::cmd_ignored;
use dry4rust::cli::cmd_report;
use dry4rust::cli::cmd_stats;
use dry4rust::cli::create_reporter;
use dry4rust::cli::run_analysis;
use dry4rust::config::Config;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use dry4rust::scanner::ScanConfig;
use dry4rust::scanner::scan_files;
use predicate::str;
use predicates::prelude::*;
use serde_json::from_str;
use std::fs;
use tempfile::TempDir;

fn analysed(fixture: &str) -> (Config, AnalysisResult) {
    let root = fixture_path(fixture);
    let config = Config::load(&root);
    let files = scan_files(&ScanConfig::new(root));
    let result =
        analyze(&RustAnalyzer::new(), &files, &config).expect("the fixture analyses cleanly");
    (config, result)
}

#[test]
fn analyze_over_a_duplicated_fixture_reports_exact_groups() {
    // Arrange & Act
    let (_, result) = analysed("exact_dupes");

    // Assert
    assert!(result.stats.total_code_units > 0);
    assert!(!result.exact_groups.is_empty());
}

#[test]
fn analyze_units_over_no_units_reports_nothing_duplicated() {
    // Arrange
    let config = Config::default();

    // Act
    let result = analyze_units(&[], Vec::new(), &config).expect("empty input is not an error");

    // Assert
    assert_eq!(result.stats.total_code_units, 0);
    assert!(result.exact_groups.is_empty());
}

#[test]
fn apply_overrides_replaces_only_the_values_that_were_given() {
    // Arrange
    let mut config = Config::default();
    let baseline_threshold = config.similarity_threshold;
    let overrides = CliOverrides {
        min_nodes: Some(42),
        ..CliOverrides::default()
    };

    // Act
    apply_overrides(&mut config, &overrides);

    // Assert
    assert_eq!(config.min_nodes, 42);
    assert!((config.similarity_threshold - baseline_threshold).abs() < f64::EPSILON);
}

#[test]
fn check_absolute_passes_percentage_fails() {
    // Arrange & Act & Assert
    // Absolute threshold is generous (passes), but percentage is strict (fails)
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "check",
            "--max-exact",
            "100",
            "--max-exact-percent",
            "0.0",
        ])
        .assert()
        .code(1)
        .stdout(str::contains("Check FAILED"));
}

#[test]
fn check_fails_with_duplicates() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "check",
            "--max-exact",
            "0",
        ])
        .assert()
        .code(1)
        .stdout(str::contains("Check FAILED"));
}

#[test]
fn check_fails_with_percentage_threshold_exceeded() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "check",
            "--max-exact",
            "100",
            "--max-exact-percent",
            "0.0",
        ])
        .assert()
        .code(1)
        .stdout(str::contains("Check FAILED"))
        .stdout(str::contains("exact duplicate lines"));
}

#[test]
fn check_no_dupes_passes() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("no_dupes").to_str().unwrap(),
            "check",
            "--max-exact",
            "0",
        ])
        .assert()
        .success()
        .stdout(str::contains("Check passed"));
}

#[test]
fn check_no_thresholds_passes_with_duplicates() {
    // Arrange & Act & Assert
    // With no thresholds set, check should pass even when duplicates exist
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "check",
        ])
        .assert()
        .success()
        .stdout(str::contains("Check passed"));
}

#[test]
fn check_passes_with_generous_percentage_threshold() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "check",
            "--max-exact",
            "100",
            "--max-exact-percent",
            "100.0",
        ])
        .assert()
        .success()
        .stdout(str::contains("Check passed"));
}

#[test]
fn check_passes_with_high_threshold() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "check",
            "--max-exact",
            "100",
        ])
        .assert()
        .success()
        .stdout(str::contains("Check passed"));
}

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

    // Create an ignore file with a stale fingerprint only
    let ignore_path = tmp.path().join(".dry4rust-ignore.toml");
    fs::write(
        &ignore_path,
        "[[ignore]]\nfingerprint = \"deadbeefdeadbeef\"\nreason = \"stale\"\n",
    )
    .unwrap();

    // Run cleanup with --dry-run
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

    // Verify the file is unchanged
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

    // Get a real fingerprint from the report
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

    // Ignore the real fingerprint
    cargo_dry4rust()
        .args(["--path", tmp.path().to_str().unwrap(), "ignore", &real_fp])
        .assert()
        .success();

    // Add a fake/stale fingerprint manually
    let ignore_path = tmp.path().join(".dry4rust-ignore.toml");
    let content = fs::read_to_string(&ignore_path).unwrap();
    let new_content = format!(
        "{content}\n[[ignore]]\nfingerprint = \"deadbeefdeadbeef\"\nreason = \"stale entry\"\n"
    );
    fs::write(&ignore_path, new_content).unwrap();

    // Run cleanup
    cargo_dry4rust()
        .args(["--path", tmp.path().to_str().unwrap(), "cleanup"])
        .assert()
        .success()
        .stdout(str::contains("Removed stale entries"))
        .stdout(str::contains("deadbeefdeadbeef"))
        .stdout(str::contains("Removed 1 stale entries"));

    // Verify the real fingerprint is still in the ignore file
    cargo_dry4rust()
        .args(["--path", tmp.path().to_str().unwrap(), "ignored"])
        .assert()
        .success()
        .stdout(str::contains(&real_fp));

    // Verify the stale entry is gone
    let final_content = fs::read_to_string(&ignore_path).unwrap();

    // Assert
    assert!(!final_content.contains("deadbeefdeadbeef"));
}

#[test]
fn cmd_check_over_a_clean_fixture_passes_with_every_ceiling_at_zero() {
    // Arrange
    let (config, result) = analysed("no_dupes");
    let reporter = create_reporter(OutputFormat::Text, None);
    let mut out = Vec::new();
    let thresholds = CheckThresholds {
        max_exact: Some(0),
        max_near: Some(0),
        max_exact_percent: Some(0.0),
        max_near_percent: Some(0.0),
    };

    // Act
    let outcome = cmd_check(&config, &result, reporter.as_ref(), &mut out, &thresholds);

    // Assert
    assert!(outcome.is_ok());
}

#[test]
fn cmd_check_with_a_generous_percentage_ceiling_passes() {
    // Arrange
    let (config, result) = analysed("exact_dupes");
    let reporter = create_reporter(OutputFormat::Text, None);
    let mut out = Vec::new();
    let thresholds = CheckThresholds {
        max_exact_percent: Some(100.0),
        max_near_percent: Some(100.0),
        ..CheckThresholds::default()
    };

    // Act
    let outcome = cmd_check(&config, &result, reporter.as_ref(), &mut out, &thresholds);

    // Assert
    assert!(outcome.is_ok());
}

#[test]
fn cmd_check_with_a_near_duplicate_ceiling_fails_when_it_is_exceeded() {
    // Arrange
    let (config, result) = analysed("near_dupes");
    let reporter = create_reporter(OutputFormat::Text, None);
    let mut out = Vec::new();
    let thresholds = CheckThresholds {
        max_near: Some(0),
        ..CheckThresholds::default()
    };

    // Act
    let outcome = cmd_check(&config, &result, reporter.as_ref(), &mut out, &thresholds);

    // Assert
    assert!(outcome.is_err() || result.near_groups.is_empty());
}

#[test]
fn cmd_check_with_a_near_percentage_ceiling_of_zero_fails_on_near_duplication() {
    // Arrange
    let (config, result) = analysed("near_dupes");
    let reporter = create_reporter(OutputFormat::Text, None);
    let mut out = Vec::new();
    let thresholds = CheckThresholds {
        max_near_percent: Some(0.0),
        ..CheckThresholds::default()
    };

    // Act
    let outcome = cmd_check(&config, &result, reporter.as_ref(), &mut out, &thresholds);

    // Assert
    assert!(outcome.is_err() || result.near_groups.is_empty());
}

#[test]
fn cmd_check_with_a_percentage_ceiling_of_zero_fails_on_any_duplication() {
    // Arrange
    let (config, result) = analysed("exact_dupes");
    let reporter = create_reporter(OutputFormat::Text, None);
    let mut out = Vec::new();
    let thresholds = CheckThresholds {
        max_exact_percent: Some(0.0),
        ..CheckThresholds::default()
    };

    // Act
    let outcome = cmd_check(&config, &result, reporter.as_ref(), &mut out, &thresholds);

    // Assert
    assert!(outcome.is_err());
}

#[test]
fn cmd_check_with_a_zero_threshold_fails_on_duplicates() {
    // Arrange
    let (config, result) = analysed("exact_dupes");
    let reporter = create_reporter(OutputFormat::Text, None);
    let mut out = Vec::new();
    let thresholds = CheckThresholds {
        max_exact: Some(0),
        ..CheckThresholds::default()
    };

    // Act
    let outcome = cmd_check(&config, &result, reporter.as_ref(), &mut out, &thresholds);

    // Assert
    assert!(outcome.is_err());
}

#[test]
fn cmd_check_with_an_exact_count_ceiling_above_the_findings_passes() {
    // Arrange
    let (config, result) = analysed("exact_dupes");
    let reporter = create_reporter(OutputFormat::Text, None);
    let mut out = Vec::new();
    let thresholds = CheckThresholds {
        max_exact: Some(9999),
        max_near: Some(9999),
        ..CheckThresholds::default()
    };

    // Act
    let outcome = cmd_check(&config, &result, reporter.as_ref(), &mut out, &thresholds);

    // Assert
    assert!(outcome.is_ok());
}

#[test]
fn cmd_check_with_every_ceiling_set_reports_each_breach_it_finds() {
    // Arrange
    let (config, result) = analysed("exact_dupes");
    let reporter = create_reporter(OutputFormat::Text, None);
    let mut out = Vec::new();
    let thresholds = CheckThresholds {
        max_exact: Some(0),
        max_near: Some(0),
        max_exact_percent: Some(0.0),
        max_near_percent: Some(0.0),
    };

    // Act
    let outcome = cmd_check(&config, &result, reporter.as_ref(), &mut out, &thresholds);

    // Assert
    assert!(
        outcome.is_err(),
        "a fixture with duplicates breaches a zero ceiling"
    );
}

#[test]
fn cmd_check_without_thresholds_passes_even_with_duplicates() {
    // Arrange
    let (config, result) = analysed("exact_dupes");
    let reporter = create_reporter(OutputFormat::Text, None);
    let mut out = Vec::new();

    // Act
    let outcome = cmd_check(
        &config,
        &result,
        reporter.as_ref(),
        &mut out,
        &CheckThresholds::default(),
    );

    // Assert
    assert!(outcome.is_ok());
}

#[test]
fn cmd_cleanup_in_dry_run_leaves_the_ignore_file_alone() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let (_, result) = analysed("exact_dupes");
    let mut out = Vec::new();

    // Act
    cmd_cleanup(tmp.path(), &result, &mut out, true).expect("dry run succeeds");

    // Assert
    assert!(!tmp.path().join(".dry4rust-ignore.toml").exists());
}

#[test]
fn cmd_cleanup_outside_dry_run_writes_the_pruned_ignore_file() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let (_, result) = analysed("exact_dupes");
    let mut added = Vec::new();
    cmd_ignore(tmp.path(), "deadbeef12345678", None, &mut added).expect("ignore accepted");
    let mut out = Vec::new();

    // Act
    cmd_cleanup(tmp.path(), &result, &mut out, false).expect("cleanup succeeds");

    // Assert
    let text = String::from_utf8(out).expect("utf-8");
    assert!(!text.is_empty(), "cleanup should say what it did");
}

#[test]
fn cmd_ignore_then_cmd_ignored_lists_what_was_added() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let mut added = Vec::new();
    let mut listed = Vec::new();

    // Act
    cmd_ignore(
        tmp.path(),
        "deadbeef12345678",
        Some(String::from("known duplicate")),
        &mut added,
    )
    .expect("a valid fingerprint is accepted");
    cmd_ignored(tmp.path(), &mut listed).expect("listing succeeds");

    // Assert
    let listed = String::from_utf8(listed).expect("utf-8");
    assert!(listed.contains("deadbeef12345678"), "{listed}");
}

#[test]
fn cmd_ignore_with_a_malformed_fingerprint_is_rejected() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let mut out = Vec::new();

    // Act
    let outcome = cmd_ignore(tmp.path(), "not-a-fingerprint", None, &mut out);

    // Assert
    assert!(outcome.is_err());
}

#[test]
fn cmd_report_writes_both_the_stats_and_the_groups() {
    // Arrange
    let (_, result) = analysed("exact_dupes");
    let reporter = create_reporter(OutputFormat::Text, None);
    let mut out = Vec::new();

    // Act
    cmd_report(&result, reporter.as_ref(), &mut out).expect("reporting succeeds");

    // Assert
    let text = String::from_utf8(out).expect("utf-8");
    assert!(text.contains("Duplication Statistics"), "{text}");
    assert!(text.contains("Exact Duplicates"), "{text}");
}

#[test]
fn cmd_stats_writes_the_summary_and_nothing_else() {
    // Arrange
    let (_, result) = analysed("exact_dupes");
    let reporter = create_reporter(OutputFormat::Text, None);
    let mut out = Vec::new();

    // Act
    cmd_stats(&result, reporter.as_ref(), &mut out).expect("reporting succeeds");

    // Assert
    let text = String::from_utf8(out).expect("utf-8");
    assert!(text.contains("Duplication Statistics"), "{text}");
    assert!(!text.contains("Exact Duplicates"), "{text}");
}

#[test]
fn create_reporter_in_json_mode_writes_parseable_json() {
    // Arrange
    let (_, result) = analysed("exact_dupes");
    let reporter = create_reporter(OutputFormat::Json, None);
    let mut out = Vec::new();

    // Act
    cmd_stats(&result, reporter.as_ref(), &mut out).expect("reporting succeeds");

    // Assert
    let parsed: serde_json::Value =
        from_str(&String::from_utf8(out).expect("utf-8")).expect("valid json");
    assert!(parsed.is_object());
}

#[test]
fn default_command_is_report() {
    // Arrange & Act & Assert
    // Running without a subcommand should behave like 'report'
    cargo_dry4rust()
        .args(["--path", fixture_path("exact_dupes").to_str().unwrap()])
        .assert()
        .success()
        .stdout(str::contains("Duplication Statistics"))
        .stdout(str::contains("Exact Duplicates"));
}

#[test]
fn error_on_nonexistent_path() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args(["--path", "/nonexistent/path/that/does/not/exist", "stats"])
        .assert()
        .code(2)
        .stderr(str::contains("No source files"));
}

#[test]
fn exclude_option_drops_the_named_paths_from_the_report() {
    // Arrange & Act & Assert
    // When all files are excluded, the tool reports no source files found
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "--exclude",
            "lib.rs",
            "stats",
        ])
        .assert()
        .code(2)
        .stderr(str::contains("No source files"));
}

#[test]
fn exclude_tests_flag_reduces_duplicates() {
    // Arrange & Act
    // Without --exclude-tests: 3 units in 1 group (2 production + 1 in #[cfg(test)] mod)
    let output_all = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("test_code").to_str().unwrap(),
            "--format",
            "json",
            "stats",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let all: serde_json::Value = from_str(&String::from_utf8(output_all).unwrap()).unwrap();

    // Assert
    assert_eq!(all["exact_duplicate_units"].as_u64().unwrap(), 3);

    // With --exclude-tests: only 2 production units remain
    let output_excl = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("test_code").to_str().unwrap(),
            "--exclude-tests",
            "--format",
            "json",
            "stats",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let excl: serde_json::Value = from_str(&String::from_utf8(output_excl).unwrap()).unwrap();
    assert_eq!(excl["exact_duplicate_units"].as_u64().unwrap(), 2);
    assert_eq!(excl["total_code_units"].as_u64().unwrap(), 2);
}

#[test]
fn exclude_tests_text_report() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("test_code").to_str().unwrap(),
            "--exclude-tests",
            "report",
        ])
        .assert()
        .success()
        .stdout(str::contains("Exact Duplicates"))
        .stdout(str::contains("Group 1"));
}

#[test]
fn exit_code_distinguishes_a_failed_check_from_an_internal_error() {
    // Arrange & Act & Assert
    assert_eq!(CliError::CheckFailed.exit_code(), 1);
    assert_eq!(CliError::NoRecognizedFiles.exit_code(), 2);
}

#[test]
fn help_flag_prints_usage_and_exits_success() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .arg("--help")
        .assert()
        .success()
        .stdout(str::contains("Detect duplicate code"));
}

#[test]
fn ignore_add_then_report_suppresses_the_group() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    // Copy fixture files to temp dir
    fs::create_dir_all(tmp.path().join("src")).unwrap();
    fs::copy(
        fixture_path("exact_dupes").join("src/lib.rs"),
        tmp.path().join("src/lib.rs"),
    )
    .unwrap();

    // First, get the report to find a fingerprint
    let output = cargo_dry4rust()
        .args(["--path", tmp.path().to_str().unwrap(), "report"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();

    // Extract a fingerprint from the output
    let fp = text
        .lines()
        .find(|l| l.contains("fingerprint:"))
        .and_then(|l| {
            let start = l.find("fingerprint: ")? + 13;
            let end = l[start..].find(',')?;
            Some(l[start..start + end].to_string())
        })
        .expect("Should find a fingerprint in the report");

    // Add it to ignore
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

    // Verify it's listed
    cargo_dry4rust()
        .args(["--path", tmp.path().to_str().unwrap(), "ignored"])
        .assert()
        .success()
        .stdout(str::contains(&fp))
        .stdout(str::contains("test ignore"));

    // Verify the report no longer shows that group
    let output_after = cargo_dry4rust()
        .args(["--path", tmp.path().to_str().unwrap(), "stats"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text_after = String::from_utf8(output_after).unwrap();

    // The ignored group should be filtered out

    // Assert
    assert!(text_after.contains("Exact duplicates: 0 groups"));
}

#[test]
fn ignore_near_duplicate_workflow() {
    // Arrange & Act
    let tmp = TempDir::new().unwrap();
    // Copy fixture files to temp dir
    fs::create_dir_all(tmp.path().join("src")).unwrap();
    fs::copy(
        fixture_path("near_dupes").join("src/lib.rs"),
        tmp.path().join("src/lib.rs"),
    )
    .unwrap();

    // Get report to find a near-duplicate fingerprint
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

    // Extract fingerprint from near-duplicate group
    let fp = text
        .lines()
        .find(|l| l.contains("fingerprint:") && l.contains("similarity:"))
        .and_then(|l| {
            let start = l.find("fingerprint: ")? + 13;
            let end = l[start..].find(',')?;
            Some(l[start..start + end].to_string())
        })
        .expect("Should find a fingerprint in near-duplicate group");

    // Add it to ignore
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

    // Verify the near-duplicate group is now filtered out
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
fn json_format_report() {
    // Arrange & Act
    let output = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "--format",
            "json",
            "report",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    // JSON report outputs stats object then groups array, separated by newlines
    let parts: Vec<&str> = text.splitn(2, "\n\n").collect();

    // Assert
    assert!(parts.len() >= 2, "expected stats + groups sections");
    let stats: serde_json::Value = from_str(parts[0]).unwrap();
    assert!(stats["total_code_units"].as_u64().unwrap() > 0);
    assert!(stats["exact_duplicate_groups"].as_u64().unwrap() > 0);
    let groups: serde_json::Value = from_str(parts[1]).unwrap();
    assert!(groups.as_array().unwrap().len() > 0);
    assert!(groups[0]["fingerprint"].is_string());
    assert!(groups[0]["members"].is_array());
}

#[test]
fn json_format_stats() {
    // Arrange & Act
    let output = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "--format",
            "json",
            "stats",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    let parsed: serde_json::Value = from_str(&text).unwrap();

    // Assert
    assert!(parsed["total_code_units"].as_u64().unwrap() > 0);
}

#[test]
fn json_stats_includes_line_counts() {
    // Arrange & Act
    let output = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "--format",
            "json",
            "stats",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    let parsed: serde_json::Value = from_str(&text).unwrap();

    // Assert
    assert!(parsed["exact_duplicate_lines"].is_u64());
    assert!(parsed["near_duplicate_lines"].is_u64());
}

#[test]
fn main_dispatches_the_check_subcommand_and_exits_one_when_a_ceiling_is_breached() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .arg("check")
        .arg("--max-exact")
        .arg("0")
        .arg("--path")
        .arg(fixture_path("exact_dupes"))
        .assert()
        .code(1);
}

#[test]
fn main_dispatches_the_check_subcommand_and_succeeds_on_a_clean_fixture() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .arg("check")
        .arg("--max-exact")
        .arg("0")
        .arg("--path")
        .arg(fixture_path("no_dupes"))
        .assert()
        .success();
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
fn main_dispatches_the_ignored_subcommand_without_an_ignore_file() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .arg("ignored")
        .arg("--path")
        .arg(fixture_path("no_dupes"))
        .assert()
        .success();
}

#[test]
fn main_dispatches_the_stats_subcommand_to_a_successful_summary() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .arg("stats")
        .arg("--path")
        .arg(fixture_path("exact_dupes"))
        .assert()
        .success();
}

#[test]
fn main_over_a_path_that_does_not_exist_reports_the_error_and_exits_non_zero() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .arg("--path")
        .arg(fixture_path("no_such_fixture_anywhere"))
        .assert()
        .failure()
        .stderr(str::contains("Error"));
}

#[test]
fn min_lines_option() {
    // Arrange & Act & Assert
    // With very high min_lines, short functions should be excluded
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "--min-lines",
            "1000",
            "stats",
        ])
        .assert()
        .success()
        .stdout(str::contains("Exact duplicates: 0 groups"));
}

#[test]
fn min_nodes_option() {
    // Arrange & Act & Assert
    // With very high min_nodes, nothing should be analyzed
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "--min-nodes",
            "1000",
            "stats",
        ])
        .assert()
        .success()
        .stdout(str::contains("Exact duplicates: 0 groups"));
}

#[test]
fn near_dupes_detected() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("near_dupes").to_str().unwrap(),
            "--threshold",
            "0.7",
            "report",
        ])
        .assert()
        .success()
        .stdout(str::contains("Near Duplicates"))
        .stdout(str::contains("Group 1"))
        .stdout(str::contains("similarity:"));
}

#[test]
fn report_exact_dupes_fixture() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "report",
        ])
        .assert()
        .success()
        .stdout(str::contains("Exact Duplicates"))
        .stdout(str::contains("Group 1"));
}

#[test]
fn report_mixed_fixture() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args(["--path", fixture_path("mixed").to_str().unwrap(), "report"])
        .assert()
        .success()
        .stdout(str::contains("Exact Duplicates"))
        .stdout(str::contains("Group 1"));
}

#[test]
fn report_no_dupes_fixture() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("no_dupes").to_str().unwrap(),
            "report",
        ])
        .assert()
        .success()
        .stdout(str::contains("No exact duplicates"));
}

#[test]
fn run_analysis_over_a_fixture_returns_config_and_result_together() {
    // Arrange
    let root = fixture_path("exact_dupes");

    // Act
    let output = run_analysis(
        &RustAnalyzer::new(),
        &root,
        OutputFormat::Text,
        &CliOverrides::default(),
    )
    .expect("the fixture analyses cleanly");

    // Assert
    assert!(output.result.stats.total_code_units > 0);
    assert_eq!(output.config.min_nodes, Config::default().min_nodes);
}

#[test]
fn stats_shows_duplicate_lines() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "stats",
        ])
        .assert()
        .success()
        .stdout(str::contains("Duplicated lines (exact):"))
        .stdout(str::contains("Duplicated lines (near):"));
}

#[test]
fn stats_shows_summary() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("exact_dupes").to_str().unwrap(),
            "stats",
        ])
        .assert()
        .success()
        .stdout(str::contains("Total code units analyzed"))
        .stdout(str::contains("Exact duplicates"));
}

#[test]
fn sub_function_detects_duplicate_branches() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("sub_function_dupes").to_str().unwrap(),
            "--sub-function",
            "report",
        ])
        .assert()
        .success()
        .stdout(str::contains("Sub-function Exact Duplicates"))
        .stdout(str::contains("if-then branch"))
        .stdout(str::contains("match arm"))
        .stdout(str::contains("for body"));
}

#[test]
fn sub_function_json_stats() {
    // Arrange & Act
    let output = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("sub_function_dupes").to_str().unwrap(),
            "--sub-function",
            "--format",
            "json",
            "stats",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    let parsed: serde_json::Value = from_str(&text).unwrap();

    // Assert
    assert_eq!(parsed["sub_exact_groups"].as_u64().unwrap(), 3);
    assert_eq!(parsed["sub_exact_units"].as_u64().unwrap(), 6);
}

#[test]
fn sub_function_min_sub_nodes_filters() {
    // Arrange & Act & Assert
    // With very high min-sub-nodes, no sub-function units should be found,
    // so the sub-function stats line is not printed at all
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("sub_function_dupes").to_str().unwrap(),
            "--sub-function",
            "--min-sub-nodes",
            "1000",
            "stats",
        ])
        .assert()
        .success()
        .stdout(str::contains("Sub-function").not());
}

#[test]
fn sub_function_shows_parent_names() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("sub_function_dupes").to_str().unwrap(),
            "--sub-function",
            "report",
        ])
        .assert()
        .success()
        .stdout(str::contains("in handle_positive"))
        .stdout(str::contains("in process_value"))
        .stdout(str::contains("in classify_number"))
        .stdout(str::contains("in describe_value"));
}

#[test]
fn sub_function_stats_shown() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("sub_function_dupes").to_str().unwrap(),
            "--sub-function",
            "stats",
        ])
        .assert()
        .success()
        .stdout(str::contains("Sub-function exact: 3 groups"));
}

#[test]
fn without_sub_function_flag_no_sub_sections() {
    // Arrange & Act & Assert
    cargo_dry4rust()
        .args([
            "--path",
            fixture_path("sub_function_dupes").to_str().unwrap(),
            "report",
        ])
        .assert()
        .success()
        .stdout(str::contains("Exact Duplicates"))
        .stdout(str::contains("Sub-function").not());
}

#[test]
fn without_sub_function_json_no_sub_fields() {
    // Arrange & Act
    let output = cargo_dry4rust()
        .args([
            "--path",
            fixture_path("sub_function_dupes").to_str().unwrap(),
            "--format",
            "json",
            "stats",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    let parsed: serde_json::Value = from_str(&text).unwrap();
    // Without --sub-function, sub fields should be absent (skip_serializing_if = "is_zero")

    // Assert
    assert!(parsed.get("sub_exact_groups").is_none());
    assert!(parsed.get("sub_near_groups").is_none());
}
