// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use dry4rust::cli::output::check_breach::CheckBreach;
use dry4rust::cli::output::json_reporter::*;
use dry4rust::cli::output::reporter::Reporter;
use dry4rust::code_unit::CodeUnit;
use dry4rust::code_unit_kind::CodeUnitKind;
use dry4rust::duplicate_group::DuplicateGroup;
use dry4rust::duplication_stats::DuplicationStats;
use dry4rust::fingerprint::Fingerprint;
use dry4rust::node::node_kind::NodeKind;
use dry4rust::node::normalized_node::NormalizedNode;
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
        ..Default::default()
    }
}

#[test]
fn json_is_valid() {
    // Arrange & Act
    let reporter = JsonReporter::new(Some(PathBuf::from("/project")));
    let group = DuplicateGroup::new(
        Fingerprint::from_node(&NormalizedNode::leaf(NodeKind::Opaque)),
        vec![make_unit("foo", "/project/src/a.rs", 10, 20)],
        1.0,
    );
    let output = reporter.report_exact(&[group]).unwrap();
    // Should be valid JSON

    // Assert
    assert!(from_str::<Value>(&output).is_ok());
}

#[test]
fn json_relative_paths() {
    // Arrange & Act
    let reporter = JsonReporter::new(Some(PathBuf::from("/home/user/project")));
    let fp = Fingerprint::from_node(&NormalizedNode::with_children(NodeKind::Block, vec![]));
    let group = DuplicateGroup::new(
        fp,
        vec![make_unit("foo", "/home/user/project/src/main.rs", 1, 10)],
        0.9,
    );
    let output = reporter.report_near(&[group]).unwrap();

    // Assert
    assert!(output.contains("src/main.rs"));
    assert!(!output.contains("/home/user/project"));
}

#[test]
fn json_report_check_gathers_the_groups_rather_than_repeating_them() {
    // Arrange -- two ceilings on the same set, both breached
    let reporter = JsonReporter::new(None);
    let stats = DuplicateGroup::compute_stats_with_sub(&[], &[], &[], &[], &[]);
    let groups = vec![group(0x11, &["process", "compute"])];
    let breaches = vec![
        CheckBreach {
            message: String::from("1 exact duplicate groups (max: 0)"),
            groups: &groups,
            of_exact: true,
        },
        CheckBreach {
            message: String::from("42.9% exact duplicate lines (max: 0.0%)"),
            groups: &groups,
            of_exact: true,
        },
    ];

    // Act
    let output = reporter
        .report_check(&stats, &breaches)
        .expect("reporting succeeds");

    // Assert
    let document: Value = from_str(&output).expect("valid json");
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
    let output = reporter.report_exact(&[]).unwrap();
    let parsed: Value = from_str(&output).unwrap();

    // Assert
    assert!(parsed.as_array().unwrap().is_empty());
}

#[test]
fn json_report_exact_with_groups() {
    // Arrange & Act
    let reporter = JsonReporter::new(Some(PathBuf::from("/project")));
    let group = DuplicateGroup::new(
        Fingerprint::from_node(&NormalizedNode::leaf(NodeKind::Opaque)),
        vec![
            make_unit("foo", "/project/src/a.rs", 10, 20),
            make_unit("bar", "/project/src/b.rs", 30, 40),
        ],
        1.0,
    );
    let output = reporter.report_exact(&[group]).unwrap();
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
    let group = DuplicateGroup::new(
        fp,
        vec![
            make_unit("process", "/src/a.rs", 10, 25),
            make_unit("compute", "/src/b.rs", 30, 45),
        ],
        0.85,
    );
    let output = reporter.report_near(&[group]).unwrap();
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
    let stats = DuplicationStats::default()
        .with_total_code_units(50)
        .with_total_lines(500)
        .with_exact_duplicate_groups(3)
        .with_exact_duplicate_units(8)
        .with_near_duplicate_groups(2)
        .with_near_duplicate_units(5)
        .with_exact_duplicate_lines(30)
        .with_near_duplicate_lines(20);
    let output = reporter.report_stats(&stats).unwrap();
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
    let group = DuplicateGroup::new(
        fp,
        vec![make_unit("if-then branch", "/src/a.rs", 10, 25)],
        1.0,
    );

    // Act
    let output = reporter.report_sub_exact(&[group]).unwrap();

    // Assert
    let parsed: Value = from_str(&output).unwrap();
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
    let group = DuplicateGroup::new(fp, vec![make_unit("for body", "/src/a.rs", 10, 25)], 0.95);

    // Act
    let output = reporter.report_sub_near(&[group]).unwrap();

    // Assert
    let parsed: Value = from_str(&output).unwrap();
    let groups = parsed.as_array().unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0]["fingerprint"].as_str().unwrap(), fp.to_hex());
    assert_eq!(groups[0]["similarity"], 0.95);
}
