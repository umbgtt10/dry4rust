// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::result_with;
use dry4rust::cli::output_format::OutputFormat;
use dry4rust::cli::stats_command::StatsCommand;
use serde_json::Value;
use serde_json::from_str;

fn summary_in(format: OutputFormat) -> String {
    let result = result_with(vec![], vec![], vec![], vec![]);
    let reporter = format.reporter(None);
    let mut out = Vec::new();

    StatsCommand::new(&result, reporter.as_ref())
        .run(&mut out)
        .expect("reporting succeeds");

    String::from_utf8(out).expect("utf-8")
}

#[test]
fn default_is_text_so_a_run_without_the_flag_is_readable() {
    // Arrange & Act
    let format = OutputFormat::default();

    // Assert
    assert_eq!(format, OutputFormat::Text);
}

#[test]
fn reporter_in_json_mode_writes_parseable_json() {
    // Arrange & Act
    let summary = summary_in(OutputFormat::Json);

    // Assert
    let parsed: Value = from_str(&summary).expect("valid json");
    assert!(parsed.is_object());
}

#[test]
fn reporter_in_text_mode_writes_the_headed_summary() {
    // Arrange & Act
    let summary = summary_in(OutputFormat::Text);

    // Assert
    assert!(summary.starts_with("Duplication Statistics"), "{summary}");
}
