// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use crate::common::result_with;
use dry4rust::cli::output_format::OutputFormat;
use dry4rust::cli::stats_command::StatsCommand;
use dry4rust::grouper::DuplicateGroup;

fn summary_of(exact: Vec<DuplicateGroup>) -> String {
    let result = result_with(exact, vec![], vec![], vec![]);
    let reporter = OutputFormat::Text.reporter(None);
    let mut out = Vec::new();

    StatsCommand::new(&result, reporter.as_ref())
        .run(&mut out)
        .expect("reporting succeeds");

    String::from_utf8(out).expect("utf-8")
}

#[test]
fn run_counts_the_groups_it_was_given() {
    // Arrange & Act
    let text = summary_of(vec![group(0x11, &["a", "b"])]);

    // Assert
    assert!(
        text.contains("Exact duplicates: 1 groups (2 code units)"),
        "{text}"
    );
}

#[test]
fn run_over_a_clean_result_reports_nothing_duplicated() {
    // Arrange & Act
    let text = summary_of(vec![]);

    // Assert
    assert!(text.contains("Exact duplicates: 0 groups"), "{text}");
}

#[test]
fn run_writes_the_summary_and_nothing_else() {
    // Arrange & Act
    let text = summary_of(vec![group(0x11, &["a", "b"])]);

    // Assert
    assert!(text.contains("Duplication Statistics"), "{text}");
    assert!(
        !text.contains("Exact Duplicates"),
        "stats stops short of the per-group listing report gives, got: {text}"
    );
}
