// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use dry4rust::code_unit::{CodeUnit, CodeUnitKind};
use dry4rust::fingerprint::Fingerprint;
use dry4rust::grouper::compute_stats_with_sub;
use dry4rust::grouper::{DuplicateGroup, DuplicationStats};
use dry4rust::node::{NodeKind, NormalizedNode};
use dry4rust::output::check_breach::CheckBreach;
use dry4rust::output::json::*;
use dry4rust::output::reporter::Reporter;
use serde_json::Value;
use serde_json::from_str;
use std::path::PathBuf;

fn make_unit(name: &str, file: &str, line_start: usize, line_end: usize) -> CodeUnit {
    CodeUnit {
        kind: CodeUnitKind::Function,
        name: name.to_string(),
        file: PathBuf::from(file),
        line_start,
        line_end,
        signature: NormalizedNode::leaf(NodeKind::Opaque),
        body: NormalizedNode::with_children(NodeKind::Block, vec![]),
        fingerprint: Fingerprint::from_node(&NormalizedNode::leaf(NodeKind::Opaque)),
        node_count: 10,
        parent_name: None,
        is_test: false,
    }
}

#[test]
fn json_is_valid() {
    // Arrange & Act
    let reporter = JsonReporter::new(Some(PathBuf::from("/project")));
    let group = DuplicateGroup {
        fingerprint: Fingerprint::from_node(&NormalizedNode::leaf(NodeKind::Opaque)),
        members: vec![make_unit("foo", "/project/src/a.rs", 10, 20)],
        similarity: 1.0,
    };
    let mut buf = Vec::new();
    reporter.report_exact(&[group], &mut buf).unwrap();
    let output = String::from_utf8(buf).unwrap();
    // Should be valid JSON

    // Assert
    assert!(from_str::<Value>(&output).is_ok());
}

#[test]
fn json_relative_paths() {
    // Arrange & Act
    let reporter = JsonReporter::new(Some(PathBuf::from("/home/user/project")));
    let fp = Fingerprint::from_node(&NormalizedNode::with_children(NodeKind::Block, vec![]));
    let group = DuplicateGroup {
        fingerprint: fp,
        members: vec![make_unit("foo", "/home/user/project/src/main.rs", 1, 10)],
        similarity: 0.9,
    };
    let mut buf = Vec::new();
    reporter.report_near(&[group], &mut buf).unwrap();
    let output = String::from_utf8(buf).unwrap();

    // Assert
    assert!(output.contains("src/main.rs"));
    assert!(!output.contains("/home/user/project"));
}

#[test]
fn json_report_check_gathers_the_groups_rather_than_repeating_them() {
    // Arrange -- two ceilings on the same set, both breached
    let reporter = JsonReporter::new(None);
    let stats = compute_stats_with_sub(&[], &[], &[], &[], &[]);
    let groups = vec![group(0x11, &["process", "compute"])];
    let breaches = vec![
        CheckBreach::new(
            String::from("1 exact duplicate groups (max: 0)"),
            &groups,
            true,
        ),
        CheckBreach::new(
            String::from("42.9% exact duplicate lines (max: 0.0%)"),
            &groups,
            true,
        ),
    ];
    let mut buf = Vec::new();

    // Act
    reporter
        .report_check(&stats, &breaches, &mut buf)
        .expect("reporting succeeds");

    // Assert
    let document: Value = from_str(&String::from_utf8(buf).unwrap()).expect("valid json");
    assert_eq!(document["breaches"].as_array().unwrap().len(), 2);
    assert_eq!(
        document["exact"].as_array().unwrap().len(),
        1,
        "text lists the groups under each breach; a document names them once"
    );
    assert_eq!(document["passed"], false);
}

#[test]
fn json_report_exact_empty() {
    // Arrange & Act
    let reporter = JsonReporter::new(None);
    let mut buf = Vec::new();
    reporter.report_exact(&[], &mut buf).unwrap();
    let output = String::from_utf8(buf).unwrap();
    let parsed: Value = from_str(&output).unwrap();

    // Assert
    assert!(parsed.as_array().unwrap().is_empty());
}

#[test]
fn json_report_exact_with_groups() {
    // Arrange & Act
    let reporter = JsonReporter::new(Some(PathBuf::from("/project")));
    let group = DuplicateGroup {
        fingerprint: Fingerprint::from_node(&NormalizedNode::leaf(NodeKind::Opaque)),
        members: vec![
            make_unit("foo", "/project/src/a.rs", 10, 20),
            make_unit("bar", "/project/src/b.rs", 30, 40),
        ],
        similarity: 1.0,
    };
    let mut buf = Vec::new();
    reporter.report_exact(&[group], &mut buf).unwrap();
    let output = String::from_utf8(buf).unwrap();
    let parsed: Value = from_str(&output).unwrap();
    let groups = parsed.as_array().unwrap();

    // Assert
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0]["members"].as_array().unwrap().len(), 2);
    assert_eq!(groups[0]["similarity"], 1.0);
    assert!(groups[0]["fingerprint"].is_string());
}

#[test]
fn json_report_near_with_groups() {
    // Arrange & Act
    let reporter = JsonReporter::new(None);
    let fp = Fingerprint::from_node(&NormalizedNode::with_children(NodeKind::Block, vec![]));
    let group = DuplicateGroup {
        fingerprint: fp,
        members: vec![
            make_unit("process", "/src/a.rs", 10, 25),
            make_unit("compute", "/src/b.rs", 30, 45),
        ],
        similarity: 0.85,
    };
    let mut buf = Vec::new();
    reporter.report_near(&[group], &mut buf).unwrap();
    let output = String::from_utf8(buf).unwrap();
    let parsed: Value = from_str(&output).unwrap();
    let groups = parsed.as_array().unwrap();

    // Assert
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0]["fingerprint"].as_str().unwrap(), fp.to_hex());
    assert_eq!(groups[0]["similarity"], 0.85);
}

#[test]
fn json_report_stats() {
    // Arrange & Act
    let reporter = JsonReporter::new(None);
    let stats = DuplicationStats {
        total_code_units: 50,
        total_lines: 500,
        exact_duplicate_groups: 3,
        exact_duplicate_units: 8,
        near_duplicate_groups: 2,
        near_duplicate_units: 5,
        exact_duplicate_lines: 30,
        near_duplicate_lines: 20,
        sub_exact_groups: 0,
        sub_exact_units: 0,
        sub_near_groups: 0,
        sub_near_units: 0,
        baseline_suppressed: None,
    };
    let mut buf = Vec::new();
    reporter.report_stats(&stats, &mut buf).unwrap();
    let output = String::from_utf8(buf).unwrap();
    let parsed: Value = from_str(&output).unwrap();

    // Assert
    assert_eq!(parsed["total_code_units"], 50);
    assert_eq!(parsed["exact_duplicate_groups"], 3);
}

#[test]
fn json_report_sub_exact_with_groups() {
    // Arrange
    let reporter = JsonReporter::new(None);
    let fp = Fingerprint::from_node(&NormalizedNode::leaf(NodeKind::Opaque));
    let group = DuplicateGroup {
        fingerprint: fp,
        members: vec![make_unit("if-then branch", "/src/a.rs", 10, 25)],
        similarity: 1.0,
    };
    let mut buf = Vec::new();

    // Act
    reporter.report_sub_exact(&[group], &mut buf).unwrap();

    // Assert
    let parsed: Value = from_str(&String::from_utf8(buf).unwrap()).unwrap();
    let groups = parsed.as_array().unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0]["fingerprint"].as_str().unwrap(), fp.to_hex());
    assert_eq!(groups[0]["similarity"], 1.0);
}

#[test]
fn json_report_sub_near_with_groups() {
    // Arrange
    let reporter = JsonReporter::new(None);
    let fp = Fingerprint::from_node(&NormalizedNode::leaf(NodeKind::Opaque));
    let group = DuplicateGroup {
        fingerprint: fp,
        members: vec![make_unit("for body", "/src/a.rs", 10, 25)],
        similarity: 0.95,
    };
    let mut buf = Vec::new();

    // Act
    reporter.report_sub_near(&[group], &mut buf).unwrap();

    // Assert
    let parsed: Value = from_str(&String::from_utf8(buf).unwrap()).unwrap();
    let groups = parsed.as_array().unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0]["fingerprint"].as_str().unwrap(), fp.to_hex());
    assert_eq!(groups[0]["similarity"], 0.95);
}
