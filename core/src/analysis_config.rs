// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// The subset of configuration a language analyzer needs.
///
/// `LanguageAnalyzer` takes this rather than the whole [`crate::config::Config`]
/// so a backend cannot reach for a threshold, a ceiling or an ignore path that
/// has nothing to do with turning source into code units.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisConfig {
    /// Minimum number of AST nodes for a code unit to be analyzed.
    min_nodes: usize,
    /// Minimum number of source lines for a code unit to be analyzed.
    min_lines: usize,
}

impl AnalysisConfig {
    #[must_use]
    pub const fn new(min_nodes: usize, min_lines: usize) -> Self {
        Self {
            min_nodes,
            min_lines,
        }
    }

    #[must_use]
    pub const fn min_lines(&self) -> usize {
        self.min_lines
    }

    #[must_use]
    pub const fn min_nodes(&self) -> usize {
        self.min_nodes
    }
}
