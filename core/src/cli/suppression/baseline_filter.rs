// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::cli::suppression::baseline_file::BaselineFile;
use crate::cli::suppression::baseline_kind::BaselineKind;
use crate::config::Config;
use crate::duplicate_group::DuplicateGroup;
use crate::error::Result;

/// Keeps the duplication a baseline did not already account for.
///
/// With no baseline configured this is the identity, and `is_in_effect` says
/// so -- which is what lets the summary report a suppressed count only when
/// there is something doing the suppressing.
pub struct BaselineFilter {
    recorded: Option<BaselineFile>,
}

impl BaselineFilter {
    /// A filter that suppresses nothing, for a run recording a baseline rather
    /// than judging against one.
    #[must_use]
    pub const fn none() -> Self {
        Self { recorded: None }
    }

    /// Read the baseline `config` names, if it names one.
    ///
    /// # Errors
    ///
    /// Returns [`crate::error::Error::Baseline`] when a baseline is named and
    /// cannot be read.
    pub fn load(config: &Config) -> Result<Self> {
        let Some(configured) = config.baseline() else {
            return Ok(Self { recorded: None });
        };
        let path = BaselineFile::baseline_path(config.root(), Some(configured));
        Ok(Self {
            recorded: Some(BaselineFile::load(&path)?),
        })
    }

    /// Whether a baseline was loaded at all.
    #[must_use]
    pub const fn is_in_effect(&self) -> bool {
        self.recorded.is_some()
    }

    /// Drop the groups the baseline already accounts for.
    #[must_use]
    pub fn retain_new(
        &self,
        kind: BaselineKind,
        groups: Vec<DuplicateGroup>,
    ) -> Vec<DuplicateGroup> {
        let Some(recorded) = self.recorded.as_ref() else {
            return groups;
        };
        groups
            .into_iter()
            .filter(|group| !recorded.admits(kind, group))
            .collect()
    }
}
