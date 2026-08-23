// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;

use crate::code_unit::CodeUnit;
use crate::config::AnalysisConfig;
use std::error::Error;

/// Trait for language-specific code analysis.
///
/// Implementors provide file-extension detection and parsing logic,
/// allowing `dry4rust` to work with any language.
///
/// **Test code handling:** Analyzers should set [`CodeUnit::is_test`] to `true`
/// for test functions, test modules, etc. The [`crate::analysis::analyze`] function will
/// filter them out when `Config::exclude_tests` is enabled, using [`is_test_code`].
pub trait LanguageAnalyzer: Send + Sync {
    /// File extensions this analyzer handles (without the leading dot).
    fn file_extensions(&self) -> &[&str];

    /// Parse a single source file into code units.
    ///
    /// `path` is the file's location (for diagnostics), `source` is the file content,
    /// and `config` carries min-node / min-line thresholds.
    ///
    /// Analyzers should tag test code via [`CodeUnit::is_test`] rather than
    /// filtering it out; the caller handles exclusion.
    fn parse_file(
        &self,
        path: &Path,
        source: &str,
        config: &AnalysisConfig,
    ) -> Result<Vec<CodeUnit>, Box<dyn Error + Send + Sync>>;

    /// Check whether a code unit represents test code.
    ///
    /// Required rather than defaulted: an analyser that cannot tell test code
    /// from production code should have to say so, not inherit an answer.
    fn is_test_code(&self, unit: &CodeUnit) -> bool;
}
