// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::baseline::baseline_file::BaselineFile;
use crate::baseline::baseline_file::baseline_path;
use crate::baseline::baseline_kind::BaselineKind;
use crate::config::Config;
use crate::error::Result;
use crate::grouper::DuplicateGroup;

/// Keeps the duplication a baseline did not already account for.
///
/// With no baseline configured this is the identity, and `is_in_effect` says
/// so -- which is what lets the summary report a suppressed count only when
/// there is something doing the suppressing.
pub struct BaselineFilter {
    recorded: Option<BaselineFile>,
}

impl BaselineFilter {
    /// Read the baseline `config` names, if it names one.
    ///
    /// # Errors
    ///
    /// Returns [`crate::error::Error::Baseline`] when a baseline is named and
    /// cannot be read.
    pub fn load(config: &Config) -> Result<Self> {
        let Some(configured) = config.baseline.as_deref() else {
            return Ok(Self { recorded: None });
        };
        let path = baseline_path(&config.root, Some(configured));
        Ok(Self {
            recorded: Some(BaselineFile::load(&path)?),
        })
    }

    /// A filter that suppresses nothing, for a run recording a baseline rather
    /// than judging against one.
    #[must_use]
    pub const fn none() -> Self {
        Self { recorded: None }
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
