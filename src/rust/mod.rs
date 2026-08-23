// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

//! Rust language analyzer for the `dupes-core` duplicate detection framework.
//!
//! This crate provides [`RustAnalyzer`], which implements the
//! [`crate::analyzer::LanguageAnalyzer`] trait using `syn` for AST parsing
//! and normalization.

pub mod normalizer;
pub mod parser;

use std::path::Path;

use crate::analyzer::LanguageAnalyzer;
use crate::code_unit::CodeUnit;
use crate::config::AnalysisConfig;
use parser::parse_source;

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
    fn file_extensions(&self) -> &[&str] {
        &["rs"]
    }

    fn parse_file(
        &self,
        path: &Path,
        source: &str,
        config: &AnalysisConfig,
    ) -> Result<Vec<CodeUnit>, Box<dyn std::error::Error + Send + Sync>> {
        parse_source(path, source, config.min_nodes, config.min_lines)
            .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { e.into() })
    }
}
