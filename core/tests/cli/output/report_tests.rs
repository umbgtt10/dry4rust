// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use dry4rust::cli::output::report::Report;
use dry4rust::cli::output::text_reporter::TextReporter;
use dry4rust::duplicate_group::DuplicateGroup;
use dry4rust::traits::reporter::Reporter;

fn rendered(reporter: &dyn Reporter, report: &Report<'_>) -> String {
    reporter.report(report).expect("reporting succeeds")
}

#[test]
fn report_in_text_leaves_out_the_sections_with_nothing_in_them() {
    // Arrange
    let stats = DuplicateGroup::compute_stats_with_sub(&[], &[], &[], &[], &[]);
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
    let stats = DuplicateGroup::compute_stats_with_sub(&[], &[], &[], &[], &[]);
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
