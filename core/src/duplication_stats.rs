// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// Statistics about duplication in the analyzed codebase.
#[derive(Debug, Clone, Default)]
pub struct DuplicationStats {
    total_code_units: usize,
    total_lines: usize,
    exact_duplicate_groups: usize,
    exact_duplicate_units: usize,
    near_duplicate_groups: usize,
    near_duplicate_units: usize,
    exact_duplicate_lines: usize,
    near_duplicate_lines: usize,
    // Sub-function stats
    sub_exact_groups: usize,
    sub_exact_units: usize,
    sub_near_groups: usize,
    sub_near_units: usize,
    /// Groups a baseline accounted for, or `None` when no baseline was in
    /// effect. Zero and "there was nothing to suppress with" are different
    /// facts, and a reader of the summary needs to tell them apart.
    baseline_suppressed: Option<usize>,
}

impl DuplicationStats {
    #[must_use]
    pub const fn baseline_suppressed(&self) -> Option<usize> {
        self.baseline_suppressed
    }

    #[must_use]
    pub const fn exact_duplicate_groups(&self) -> usize {
        self.exact_duplicate_groups
    }

    #[must_use]
    pub const fn exact_duplicate_lines(&self) -> usize {
        self.exact_duplicate_lines
    }

    /// Percentage of total lines that are exact duplicates.
    #[must_use]
    pub fn exact_duplicate_percent(&self) -> f64 {
        self.percent_of_total(self.exact_duplicate_lines)
    }

    #[must_use]
    pub const fn exact_duplicate_units(&self) -> usize {
        self.exact_duplicate_units
    }

    #[must_use]
    pub const fn near_duplicate_groups(&self) -> usize {
        self.near_duplicate_groups
    }

    #[must_use]
    pub const fn near_duplicate_lines(&self) -> usize {
        self.near_duplicate_lines
    }

    /// Percentage of total lines that are near duplicates.
    #[must_use]
    pub fn near_duplicate_percent(&self) -> f64 {
        self.percent_of_total(self.near_duplicate_lines)
    }

    #[must_use]
    pub const fn near_duplicate_units(&self) -> usize {
        self.near_duplicate_units
    }

    #[must_use]
    pub const fn sub_exact_groups(&self) -> usize {
        self.sub_exact_groups
    }

    #[must_use]
    pub const fn sub_exact_units(&self) -> usize {
        self.sub_exact_units
    }

    #[must_use]
    pub const fn sub_near_groups(&self) -> usize {
        self.sub_near_groups
    }

    #[must_use]
    pub const fn sub_near_units(&self) -> usize {
        self.sub_near_units
    }

    #[must_use]
    pub const fn total_code_units(&self) -> usize {
        self.total_code_units
    }

    #[must_use]
    pub const fn total_lines(&self) -> usize {
        self.total_lines
    }

    /// The same statistics, with what the baseline took out of them.
    #[must_use]
    pub const fn with_baseline_suppressed(self, suppressed: Option<usize>) -> Self {
        Self {
            baseline_suppressed: suppressed,
            ..self
        }
    }

    #[must_use]
    pub const fn with_exact_duplicate_groups(self, exact_duplicate_groups: usize) -> Self {
        Self {
            exact_duplicate_groups,
            ..self
        }
    }

    #[must_use]
    pub const fn with_exact_duplicate_lines(self, exact_duplicate_lines: usize) -> Self {
        Self {
            exact_duplicate_lines,
            ..self
        }
    }

    #[must_use]
    pub const fn with_exact_duplicate_units(self, exact_duplicate_units: usize) -> Self {
        Self {
            exact_duplicate_units,
            ..self
        }
    }

    #[must_use]
    pub const fn with_near_duplicate_groups(self, near_duplicate_groups: usize) -> Self {
        Self {
            near_duplicate_groups,
            ..self
        }
    }

    #[must_use]
    pub const fn with_near_duplicate_lines(self, near_duplicate_lines: usize) -> Self {
        Self {
            near_duplicate_lines,
            ..self
        }
    }

    #[must_use]
    pub const fn with_near_duplicate_units(self, near_duplicate_units: usize) -> Self {
        Self {
            near_duplicate_units,
            ..self
        }
    }

    #[must_use]
    pub const fn with_sub_exact_groups(self, sub_exact_groups: usize) -> Self {
        Self {
            sub_exact_groups,
            ..self
        }
    }

    #[must_use]
    pub const fn with_sub_exact_units(self, sub_exact_units: usize) -> Self {
        Self {
            sub_exact_units,
            ..self
        }
    }

    #[must_use]
    pub const fn with_sub_near_groups(self, sub_near_groups: usize) -> Self {
        Self {
            sub_near_groups,
            ..self
        }
    }

    #[must_use]
    pub const fn with_sub_near_units(self, sub_near_units: usize) -> Self {
        Self {
            sub_near_units,
            ..self
        }
    }

    #[must_use]
    pub const fn with_total_code_units(self, total_code_units: usize) -> Self {
        Self {
            total_code_units,
            ..self
        }
    }

    #[must_use]
    pub const fn with_total_lines(self, total_lines: usize) -> Self {
        Self {
            total_lines,
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
}
