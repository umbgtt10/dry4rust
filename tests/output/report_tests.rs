// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use dry4rust::grouper::DuplicateGroup;
use dry4rust::grouper::compute_stats_with_sub;
use dry4rust::output::json::JsonReporter;
use dry4rust::output::report::Report;
use dry4rust::output::reporter::Reporter;
use dry4rust::output::text::TextReporter;
use serde_json::Value;
use serde_json::from_str;

fn rendered(reporter: &dyn Reporter, report: &Report<'_>) -> String {
    let mut buf = Vec::new();
    reporter
        .report(report, &mut buf)
        .expect("reporting succeeds");
    String::from_utf8(buf).expect("utf-8")
}

#[test]
fn report_in_json_is_one_document_with_the_sections_named() {
    // Arrange
    let stats = compute_stats_with_sub(&[], &[], &[], &[], &[]);
    let exact = vec![group(0x11, &["a", "b"])];
    let sub_near = vec![group(0x22, &["c", "d"])];
    let report = Report {
        stats: &stats,
        exact: &exact,
        near: &[],
        sub_exact: &[],
        sub_near: &sub_near,
    };

    // Act
    let output = rendered(&JsonReporter::new(None), &report);

    // Assert
    let document: Value =
        from_str(&output).expect("one parse reads the whole thing, which is the point");
    assert!(document["stats"].is_object());
    assert_eq!(document["exact"].as_array().expect("exact").len(), 1);
    assert_eq!(document["near"].as_array().expect("near").len(), 0);
    assert_eq!(document["sub_near"].as_array().expect("sub_near").len(), 1);
    assert!(
        document.get("sub_exact").is_none(),
        "an empty sub-function section is absent rather than an empty array, \
         because sub-function analysis is opt-in and `[]` would read as \
         having looked, got: {output}"
    );
}

#[test]
fn report_in_json_keeps_exact_and_near_even_when_empty() {
    // Arrange
    let stats = compute_stats_with_sub(&[], &[], &[], &[], &[]);
    let report = Report {
        stats: &stats,
        exact: &[],
        near: &[],
        sub_exact: &[],
        sub_near: &[],
    };

    // Act
    let output = rendered(&JsonReporter::new(None), &report);

    // Assert
    let document: Value = from_str(&output).expect("valid json");
    assert!(
        document["exact"].is_array() && document["near"].is_array(),
        "these two are always analysed, so a reader can always index them, \
         got: {output}"
    );
}

#[test]
fn report_in_text_leaves_out_the_sections_with_nothing_in_them() {
    // Arrange
    let stats = compute_stats_with_sub(&[], &[], &[], &[], &[]);
    let exact: Vec<DuplicateGroup> = vec![group(0x11, &["a", "b"])];
    let report = Report {
        stats: &stats,
        exact: &exact,
        near: &[],
        sub_exact: &[],
        sub_near: &[],
    };

    // Act
    let output = rendered(&TextReporter::new(None), &report);

    // Assert
    assert!(output.contains("Duplication Statistics"), "{output}");
    assert!(output.contains("Exact Duplicates"), "{output}");
    assert!(!output.contains("Near Duplicates"), "{output}");
    assert!(!output.contains("Sub-function"), "{output}");
}

#[test]
fn report_in_text_writes_every_section_that_has_something_in_it() {
    // Arrange
    let stats = compute_stats_with_sub(&[], &[], &[], &[], &[]);
    let groups = vec![group(0x11, &["a", "b"])];
    let report = Report {
        stats: &stats,
        exact: &groups,
        near: &groups,
        sub_exact: &groups,
        sub_near: &groups,
    };

    // Act
    let output = rendered(&TextReporter::new(None), &report);

    // Assert
    assert!(output.contains("Exact Duplicates"), "{output}");
    assert!(output.contains("Near Duplicates"), "{output}");
    assert!(output.contains("Sub-function Exact Duplicates"), "{output}");
    assert!(output.contains("Sub-function Near Duplicates"), "{output}");
}
