// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Deserialize;

const CLEAN_VERDICT: &str = "Clean";

#[derive(Debug, Clone, Deserialize)]
pub struct CrapFunction {
    name: String,
    relative_file: String,
    line: u32,
    complexity: u32,
    coverage: f64,
    crap_score: f64,
    verdict: String,
}

impl CrapFunction {
    #[must_use]
    pub const fn complexity(&self) -> u32 {
        self.complexity
    }

    #[must_use]
    pub const fn coverage(&self) -> f64 {
        self.coverage
    }

    #[must_use]
    pub const fn crap_score(&self) -> f64 {
        self.crap_score
    }

    #[must_use]
    pub fn describe(&self) -> String {
        format!(
            "{}:{} {} (complexity {}, coverage {:.0}%, crap {:.1}) [{}]",
            self.relative_file,
            self.line,
            self.name,
            self.complexity,
            self.coverage * 100.0,
            self.crap_score,
            self.verdict,
        )
    }

    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.verdict == CLEAN_VERDICT
    }

    #[must_use]
    pub const fn line(&self) -> u32 {
        self.line
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn relative_file(&self) -> &str {
        &self.relative_file
    }

    #[must_use]
    pub fn verdict(&self) -> &str {
        &self.verdict
    }
}
