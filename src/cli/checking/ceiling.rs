// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::threshold::Threshold;

/// One limit `check` measures a result against, and the sentence it produces
/// when the result exceeds it.
///
/// `check` had four of these written out in full, differing only in what they
/// counted and how the number reads -- a group count as an integer, a
/// duplicated-line share to one decimal place. Four copies of one shape is how
/// a function acquires branches it does not need.
pub struct Ceiling {
    limit: Option<f64>,
    actual: f64,
    subject: &'static str,
    percentage: bool,
}

impl Ceiling {
    /// A ceiling on a number of groups.
    #[must_use]
    pub fn count(limit: Option<usize>, actual: usize, subject: &'static str) -> Self {
        Self {
            limit: limit.map(|value| value as f64),
            actual: actual as f64,
            subject,
            percentage: false,
        }
    }

    /// A ceiling on a share of duplicated lines.
    #[must_use]
    pub fn percent(limit: Option<Threshold>, actual: f64, subject: &'static str) -> Self {
        Self {
            limit: limit.map(Threshold::as_percent),
            actual,
            subject,
            percentage: true,
        }
    }

    /// What to say when the ceiling is exceeded, and `None` when it is not.
    ///
    /// An unset limit is not a ceiling of zero. It means the caller did not
    /// ask, so nothing can breach it.
    #[must_use]
    pub fn breach(&self) -> Option<String> {
        let limit = self.limit?;
        if self.actual <= limit {
            return None;
        }
        Some(if self.percentage {
            format!("{:.1}% {} (max: {:.1}%)", self.actual, self.subject, limit)
        } else {
            format!(
                "{} {} (max: {})",
                self.actual as usize, self.subject, limit as usize
            )
        })
    }
}
