// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::helpers::{cargo_dry4rust, fixture_path};
use predicate::str;
use predicates::prelude::*;
use serde_json::from_str;
use std::fs;
use tempfile::TempDir;

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
    let ignore_path = tmp.path().join(".dupes-ignore.toml");
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
    let ignore_path = tmp.path().join(".dupes-ignore.toml");
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
