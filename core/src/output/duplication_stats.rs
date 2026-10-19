// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// Statistics about duplication in the analyzed codebase.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DuplicationStats {
    pub total_code_units: usize,
    pub total_lines: usize,
    pub exact_duplicate_groups: usize,
    pub exact_duplicate_units: usize,
    pub near_duplicate_groups: usize,
    pub near_duplicate_units: usize,
    pub exact_duplicate_lines: usize,
    pub near_duplicate_lines: usize,
    // Sub-function stats
    pub sub_exact_groups: usize,
    pub sub_exact_units: usize,
    pub sub_near_groups: usize,
    pub sub_near_units: usize,
    /// Groups a baseline accounted for, or `None` when no baseline was in
    /// effect. Zero and "there was nothing to suppress with" are different
    /// facts, and a reader of the summary needs to tell them apart.
    pub baseline_suppressed: Option<usize>,
}

impl DuplicationStats {
    /// The same statistics, with what the baseline took out of them.
    #[must_use]
    pub const fn with_baseline_suppressed(self, suppressed: Option<usize>) -> Self {
        Self {
            baseline_suppressed: suppressed,
            ..self
        }
    }

    fn percent_of_total(&self, lines: usize) -> f64 {
        if self.total_lines == 0 {
            0.0
        } else {
            lines as f64 / self.total_lines as f64 * 100.0
        }
    }

    /// Percentage of total lines that are exact duplicates.
    #[must_use]
    pub fn exact_duplicate_percent(&self) -> f64 {
        self.percent_of_total(self.exact_duplicate_lines)
    }

    /// Percentage of total lines that are near duplicates.
    #[must_use]
    pub fn near_duplicate_percent(&self) -> f64 {
        self.percent_of_total(self.near_duplicate_lines)
    }
}
