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
    pub min_nodes: usize,
    /// Minimum number of source lines for a code unit to be analyzed.
    pub min_lines: usize,
}
