// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::fixture::Fixture;
use dry4rust::cli::stats_command::StatsCommand;
use dry4rust_cli::invocation::output_format::OutputFormat;

pub struct FormatScenario;

impl FormatScenario {
    #[must_use]
    pub fn summary_in(format: OutputFormat) -> String {
        let (_, result) = Fixture::analysed("exact_dupes");
        let reporter = format.reporter(None);

        StatsCommand::new(&result, reporter.as_ref())
            .run()
            .expect("reporting succeeds")
            .text()
            .to_string()
    }
}
