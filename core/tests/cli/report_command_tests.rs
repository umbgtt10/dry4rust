// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use crate::common::result_with;
use dry4rust::cli::output::text_reporter::TextReporter;
use dry4rust::cli::report_command::ReportCommand;
use dry4rust::duplicate_group::DuplicateGroup;

fn report_of(exact: Vec<DuplicateGroup>, near: Vec<DuplicateGroup>) -> String {
    let result = result_with(exact, near, vec![], vec![]);
    let reporter = TextReporter::new(None);

    ReportCommand::new(&result, &reporter)
        .run()
        .expect("reporting succeeds")
        .text()
        .to_string()
}

#[test]
fn run_over_a_clean_result_says_so_rather_than_printing_an_empty_section() {
    // Arrange & Act
    let text = report_of(vec![], vec![]);

    // Assert
    assert!(text.contains("No exact duplicates found."), "{text}");
    assert!(
        !text.contains("Near Duplicates"),
        "an empty near section is left out entirely, got: {text}"
    );
}

#[test]
fn run_writes_both_the_stats_and_the_groups() {
    // Arrange & Act
    let text = report_of(vec![group(0x11, &["a", "b"])], vec![]);

    // Assert
    assert!(text.contains("Duplication Statistics"), "{text}");
    assert!(text.contains("Exact Duplicates"), "{text}");
}

#[test]
fn run_writes_the_near_section_when_there_is_one() {
    // Arrange & Act
    let text = report_of(vec![], vec![group(0x22, &["c", "d"])]);

    // Assert
    assert!(text.contains("Near Duplicates"), "{text}");
}
