// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use dry4rust::code_unit::CodeUnit;
use dry4rust::code_unit_kind::CodeUnitKind;
use dry4rust::duplicate_group::DuplicateGroup;
use dry4rust::fingerprint::Fingerprint;
use dry4rust::node::NodeKind;
use dry4rust::node::NormalizedNode;
use dry4rust::output::check_breach::CheckBreach;
use dry4rust::output::display_path::DisplayPath;
use dry4rust::output::duplication_stats::DuplicationStats;
use dry4rust::output::reporter::Reporter;
use dry4rust::output::text_reporter::*;
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
        ..Default::default()
    }
}

#[test]
fn relative_path_stripping() {
    // Arrange & Act
    let base = PathBuf::from("/home/user/project");
    let result = DisplayPath::of(
        Some(base.as_path()),
        Path::new("/home/user/project/src/main.rs"),
    );

    // Assert
    assert_eq!(result, "src/main.rs");
}

#[test]
fn text_report_check_puts_each_breach_before_the_groups_behind_it() {
    // Arrange
    let reporter = TextReporter::new(None);
    let stats = DuplicateGroup::compute_stats_with_sub(&[], &[], &[], &[], &[]);
    let groups = vec![group(0x11, &["process", "compute"])];
    let breaches = vec![CheckBreach {
        message: String::from("1 exact duplicate groups (max: 0)"),
        groups: &groups,
        of_exact: true,
    }];

    // Act
    let output = reporter
        .report_check(&stats, &breaches)
        .expect("reporting succeeds");

    // Assert
    let sentence = output.find("Check FAILED").expect("the breach is named");
    let listing = output
        .find("Exact Duplicates")
        .expect("and its groups shown");
    assert!(
        sentence < listing,
        "the sentence comes first and the groups explain it, got: {output}"
    );
    assert!(!output.contains("Check passed."), "{output}");
}

#[test]
fn text_report_check_with_no_breaches_says_it_passed() {
    // Arrange
    let reporter = TextReporter::new(None);
    let stats = DuplicateGroup::compute_stats_with_sub(&[], &[], &[], &[], &[]);

    // Act
    let output = reporter
        .report_check(&stats, &[])
        .expect("reporting succeeds");

    // Assert
    assert!(output.contains("Duplication Statistics"), "{output}");
    assert!(output.ends_with("\nCheck passed.\n"), "{output}");
}

#[test]
fn text_report_exact_empty() {
    // Arrange & Act
    let reporter = TextReporter::new(None);
    let output = reporter.report_exact(&[]).unwrap();

    // Assert
    assert!(output.contains("No exact duplicates"));
}

#[test]
fn text_report_exact_with_groups() {
    // Arrange & Act
    let reporter = TextReporter::new(Some(PathBuf::from("/project")));
    let group = DuplicateGroup::new(
        Fingerprint::from_node(&NormalizedNode::leaf(NodeKind::Opaque)),
        vec![
            make_unit("foo", "/project/src/a.rs", 10, 20),
            make_unit("bar", "/project/src/b.rs", 30, 40),
        ],
        1.0,
    );
    let output = reporter.report_exact(&[group]).unwrap();

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
    let output = reporter.report_near(&[]).unwrap();

    // Assert
    assert!(output.contains("No near duplicates"));
}

#[test]
fn text_report_near_with_groups() {
    // Arrange & Act
    let reporter = TextReporter::new(None);
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
    let stats = DuplicationStats::default()
        .with_total_code_units(100)
        .with_total_lines(1000)
        .with_exact_duplicate_groups(5)
        .with_exact_duplicate_units(12)
        .with_near_duplicate_groups(3)
        .with_near_duplicate_units(8)
        .with_exact_duplicate_lines(60)
        .with_near_duplicate_lines(40);
    let output = reporter.report_stats(&stats).unwrap();

    // Assert
    assert!(output.contains("100"));
    assert!(output.contains("5 groups"));
    assert!(output.contains("3 groups"));
}

#[test]
fn text_report_sub_exact_heads_its_own_section_and_names_the_parent() {
    // Arrange
    let reporter = TextReporter::new(None);
    let member = CodeUnit {
        parent_name: Some(String::from("handle_positive")),
        ..make_unit("if-then branch", "/src/a.rs", 10, 25)
    };
    let group = DuplicateGroup::new(
        Fingerprint::from_node(&NormalizedNode::leaf(NodeKind::Opaque)),
        vec![member],
        1.0,
    );

    // Act
    let output = reporter.report_sub_exact(&[group]).unwrap();

    // Assert
    assert!(output.contains("Sub-function Exact Duplicates"), "{output}");
    assert!(
        output.contains("in handle_positive"),
        "a branch is only locatable through the function it came from, got: {output}"
    );
    assert!(
        !output.contains("similarity:"),
        "an exact section states no similarity, got: {output}"
    );
}

#[test]
fn text_report_sub_near_heads_its_own_section_and_states_the_similarity() {
    // Arrange
    let reporter = TextReporter::new(None);
    let member = CodeUnit {
        parent_name: Some(String::from("total_rising")),
        ..make_unit("for body", "/src/a.rs", 10, 25)
    };
    let group = DuplicateGroup::new(
        Fingerprint::from_node(&NormalizedNode::leaf(NodeKind::Opaque)),
        vec![member],
        0.95,
    );

    // Act
    let output = reporter.report_sub_near(&[group]).unwrap();

    // Assert
    assert!(output.contains("Sub-function Near Duplicates"), "{output}");
    assert!(output.contains("95%"), "{output}");
    assert!(output.contains("in total_rising"), "{output}");
}

#[test]
fn text_report_sub_near_with_nothing_to_report_writes_nothing_at_all() {
    // Arrange
    let reporter = TextReporter::new(None);

    // Act
    let output = reporter.report_sub_near(&[]).unwrap();

    // Assert
    assert!(
        output.is_empty(),
        "the two sub-function sections have no empty message, unlike the exact \
         one, so an absent section is absent rather than announced"
    );
}
