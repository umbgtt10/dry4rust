// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::fixture::Fixture;
use dry4rust::cli::check_command::CheckCommand;
use dry4rust::cli::checking::check_thresholds::CheckThresholds;
use dry4rust::cli::cli_error::CliResult;
use dry4rust::cli::output_format::OutputFormat;
use dry4rust::threshold::Threshold;

pub struct CheckScenario;

impl CheckScenario {
    pub fn checked(fixture: &str, thresholds: &CheckThresholds) -> (CliResult, String) {
        let (config, result) = Fixture::analysed(fixture);
        let reporter = OutputFormat::Text.reporter(None);

        let outcome = CheckCommand::new(&config, &result, reporter.as_ref(), thresholds)
            .run()
            .expect("check produces a report");

        (outcome.result, outcome.text)
    }

    #[must_use]
    pub fn percent(value: f64) -> Threshold {
        Threshold::percent("a ceiling", value).expect("the test states a share of a hundred")
    }
}
