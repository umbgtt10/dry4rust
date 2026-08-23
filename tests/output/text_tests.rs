// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::code_unit::{CodeUnit, CodeUnitKind};
use dry4rust::fingerprint::Fingerprint;
use dry4rust::grouper::{DuplicateGroup, DuplicationStats};
use dry4rust::node::{NodeKind, NormalizedNode};
use dry4rust::output::text::*;
use dry4rust::output::{Reporter, display_path};
use std::path::Path;
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
fn relative_path_stripping() {
    // Arrange & Act
    let base = PathBuf::from("/home/user/project");
    let result = display_path(
        Some(base.as_path()),
        Path::new("/home/user/project/src/main.rs"),
    );

    // Assert
    assert_eq!(result, "src/main.rs");
}

#[test]
fn text_report_exact_empty() {
    // Arrange & Act
    let reporter = TextReporter::new(None);
    let mut buf = Vec::new();
    reporter.report_exact(&[], &mut buf).unwrap();
    let output = String::from_utf8(buf).unwrap();

    // Assert
    assert!(output.contains("No exact duplicates"));
}

#[test]
fn text_report_exact_with_groups() {
    // Arrange & Act
    let reporter = TextReporter::new(Some(PathBuf::from("/project")));
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

    // Assert
    assert!(output.contains("Group 1"));
    assert!(output.contains("foo"));
    assert!(output.contains("bar"));
    assert!(output.contains("src/a.rs"));
    assert!(output.contains("src/b.rs"));
}

#[test]
fn text_report_near_empty() {
    // Arrange & Act
    let reporter = TextReporter::new(None);
    let mut buf = Vec::new();
    reporter.report_near(&[], &mut buf).unwrap();
    let output = String::from_utf8(buf).unwrap();

    // Assert
    assert!(output.contains("No near duplicates"));
}

#[test]
fn text_report_near_with_groups() {
    // Arrange & Act
    let reporter = TextReporter::new(None);
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

    // Assert
    assert!(output.contains("fingerprint:"));
    assert!(output.contains(&fp.to_hex()));
    assert!(output.contains("85%"));
    assert!(output.contains("process"));
    assert!(output.contains("compute"));
}

#[test]
fn text_report_stats() {
    // Arrange & Act
    let reporter = TextReporter::new(None);
    let stats = DuplicationStats {
        total_code_units: 100,
        total_lines: 1000,
        exact_duplicate_groups: 5,
        exact_duplicate_units: 12,
        near_duplicate_groups: 3,
        near_duplicate_units: 8,
        exact_duplicate_lines: 60,
        near_duplicate_lines: 40,
        sub_exact_groups: 0,
        sub_exact_units: 0,
        sub_near_groups: 0,
        sub_near_units: 0,
    };
    let mut buf = Vec::new();
    reporter.report_stats(&stats, &mut buf).unwrap();
    let output = String::from_utf8(buf).unwrap();

    // Assert
    assert!(output.contains("100"));
    assert!(output.contains("5 groups"));
    assert!(output.contains("3 groups"));
}
