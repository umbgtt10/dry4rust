// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

//! Rust language analyzer for the `dry4rust` duplicate detection framework.
//!
//! This crate provides [`RustAnalyzer`], which implements the
//! [`crate::analyzer::LanguageAnalyzer`] trait using `syn` for AST parsing
//! and normalization.

use super::parser::parse_source;
use crate::analysis_config::AnalysisConfig;
use crate::analyzer::LanguageAnalyzer;
use crate::code_unit::CodeUnit;
use crate::error::Error;
use crate::source_path::SourcePath;

/// Rust language analyzer using syn for AST parsing.
pub struct RustAnalyzer;

impl RustAnalyzer {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for RustAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageAnalyzer for RustAnalyzer {
    fn is_test_code(&self, unit: &CodeUnit) -> bool {
        unit.is_test
    }

    fn file_extensions(&self) -> &[&str] {
        &["rs"]
    }

    fn parse_file(
        &self,
        path: &SourcePath,
        source: &str,
        config: &AnalysisConfig,
    ) -> Result<Vec<CodeUnit>, Error> {
        parse_source(path, source, config.min_nodes, config.min_lines)
    }
}
